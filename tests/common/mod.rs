use std::path::PathBuf;

use anyhow::Context;
use serde_json::{Map, Value};

pub fn fixture_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

pub fn read_fixture(relative: &str) -> anyhow::Result<String> {
    let path = fixture_path(relative);
    std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read fixture: {}", path.display()))
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
