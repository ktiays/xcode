mod common;

use anyhow::Result;
use serde_json::json;

use common::{canonicalize_json, normalize_newlines, read_fixture, run_ts_oracle};

#[test]
fn workspace_parse_and_build_match_ts() -> Result<()> {
    let fixtures = [
        "src/workspace/__tests__/fixtures/simple.xcworkspacedata",
        "src/workspace/__tests__/fixtures/with-groups.xcworkspacedata",
        "src/workspace/__tests__/fixtures/special-characters.xcworkspacedata",
    ];

    for fixture in fixtures {
        let xml = read_fixture(fixture)?;

        let ts = run_ts_oracle(&json!({
            "module": "workspace",
            "operation": "parse",
            "input": xml,
        }))?;

        let rs = xcode::workspace::parse(&xml)?;
        let rs_json = serde_json::to_value(&rs)?;

        assert_eq!(
            canonicalize_json(&ts),
            canonicalize_json(&rs_json),
            "workspace parse mismatch for fixture: {fixture}"
        );

        let ts_built = run_ts_oracle(&json!({
            "module": "workspace",
            "operation": "build",
            "value": ts,
        }))?
        .as_str()
        .expect("workspace build should return xml string")
        .to_string();

        let rs_built = xcode::workspace::build(&rs);

        assert_eq!(
            normalize_newlines(&ts_built),
            normalize_newlines(&rs_built),
            "workspace build mismatch for fixture: {fixture}"
        );
    }

    Ok(())
}

#[test]
fn workspace_checks_parse_and_build_match_ts_semantics() -> Result<()> {
    let plist = read_fixture("src/workspace/__tests__/fixtures/IDEWorkspaceChecks.plist")?;

    let ts = run_ts_oracle(&json!({
        "module": "workspace",
        "operation": "parseChecks",
        "input": plist,
    }))?;

    let rs = xcode::workspace::parse_checks(&plist)?;
    let rs_json = serde_json::to_value(&rs)?;

    assert_eq!(canonicalize_json(&ts), canonicalize_json(&rs_json));

    let ts_built = run_ts_oracle(&json!({
        "module": "workspace",
        "operation": "buildChecks",
        "value": ts,
    }))?
    .as_str()
    .expect("workspace buildChecks should return plist string")
    .to_string();

    let rs_built = xcode::workspace::build_checks(&rs)?;

    let ts_from_ts_built = run_ts_oracle(&json!({
        "module": "workspace",
        "operation": "parseChecks",
        "input": ts_built,
    }))?;

    let ts_from_rs_built = run_ts_oracle(&json!({
        "module": "workspace",
        "operation": "parseChecks",
        "input": rs_built,
    }))?;

    assert_eq!(
        canonicalize_json(&ts_from_ts_built),
        canonicalize_json(&ts_from_rs_built)
    );

    Ok(())
}
