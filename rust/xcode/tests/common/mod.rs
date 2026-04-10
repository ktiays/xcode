use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{anyhow, bail, Context};
use serde_json::{Map, Value};

pub fn repo_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .expect("crate must be nested under rust/xcode")
}

pub fn fixture_path(relative: &str) -> PathBuf {
    repo_root().join(relative)
}

pub fn read_fixture(relative: &str) -> anyhow::Result<String> {
    let path = fixture_path(relative);
    std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read fixture: {}", path.display()))
}

pub fn run_ts_oracle(request: &Value) -> anyhow::Result<Value> {
    let oracle_path = repo_root().join("rust/tools/ts_oracle.cjs");
    if !oracle_path.exists() {
        bail!("TS oracle script not found at {}", oracle_path.display());
    }

    let payload = serde_json::to_vec(request)?;

    let mut child = Command::new("node")
        .arg(&oracle_path)
        .current_dir(repo_root())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| "failed to spawn node for TS oracle")?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("failed to open stdin for TS oracle"))?;
        stdin.write_all(&payload)?;
    }

    let output = child.wait_with_output()?;

    if output.stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "TS oracle returned empty stdout (status: {}). stderr: {}",
            output.status,
            stderr.trim()
        );
    }

    let response: Value = serde_json::from_slice(&output.stdout).with_context(|| {
        format!(
            "invalid TS oracle response: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    })?;

    if response.get("ok").and_then(Value::as_bool).unwrap_or(false) {
        return Ok(response.get("result").cloned().unwrap_or(Value::Null));
    }

    let err = response.get("error").cloned().unwrap_or(Value::Null);
    bail!("TS oracle error: {}", err);
}

pub fn canonicalize_json(value: &Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.iter().map(canonicalize_json).collect()),
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();

            let mut out = Map::new();
            for key in keys {
                out.insert(key.clone(), canonicalize_json(&map[key]));
            }

            Value::Object(out)
        }
        _ => value.clone(),
    }
}

pub fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}
