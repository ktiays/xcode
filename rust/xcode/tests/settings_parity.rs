mod common;

use anyhow::Result;
use serde_json::json;

use common::{canonicalize_json, read_fixture, run_ts_oracle};

#[test]
fn settings_parse_matches_ts_and_build_matches_semantics() -> Result<()> {
    let plist = read_fixture("src/settings/__tests__/fixtures/WorkspaceSettings.xcsettings")?;

    let ts = run_ts_oracle(&json!({
        "module": "settings",
        "operation": "parse",
        "input": plist,
    }))?;

    let rs = xcode::settings::parse(&plist)?;
    let rs_json = serde_json::to_value(&rs)?;

    assert_eq!(canonicalize_json(&ts), canonicalize_json(&rs_json));

    let ts_built = run_ts_oracle(&json!({
        "module": "settings",
        "operation": "build",
        "value": ts,
    }))?
    .as_str()
    .expect("settings build should return plist string")
    .to_string();

    let rs_built = xcode::settings::build(&rs)?;

    let ts_from_ts_built = run_ts_oracle(&json!({
        "module": "settings",
        "operation": "parse",
        "input": ts_built,
    }))?;

    let ts_from_rs_built = run_ts_oracle(&json!({
        "module": "settings",
        "operation": "parse",
        "input": rs_built,
    }))?;

    assert_eq!(
        canonicalize_json(&ts_from_ts_built),
        canonicalize_json(&ts_from_rs_built)
    );

    Ok(())
}
