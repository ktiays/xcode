mod common;

use anyhow::Result;
use serde_json::json;

use common::{canonicalize_json, normalize_newlines, read_fixture, run_ts_oracle};

#[test]
fn scheme_root_attributes_match_ts_and_round_trip_is_stable() -> Result<()> {
    let fixtures = [
        "src/scheme/__tests__/fixtures/iOS.xcscheme",
        "src/scheme/__tests__/fixtures/WatchApp.xcscheme",
        "src/scheme/__tests__/fixtures/MinimalInformation.xcscheme",
    ];

    for fixture in fixtures {
        let xml = read_fixture(fixture)?;

        let ts = run_ts_oracle(&json!({
            "module": "scheme",
            "operation": "parse",
            "input": xml,
        }))?;

        let rs = xcode::scheme::parse(&xml)?;

        assert_eq!(
            rs.version.as_deref(),
            ts.get("version").and_then(|v| v.as_str()),
            "scheme version mismatch for fixture: {fixture}"
        );
        assert_eq!(
            rs.last_upgrade_version.as_deref(),
            ts.get("lastUpgradeVersion").and_then(|v| v.as_str()),
            "scheme LastUpgradeVersion mismatch for fixture: {fixture}"
        );

        let rs_built = xcode::scheme::build(&rs);
        assert_eq!(
            normalize_newlines(&xml),
            normalize_newlines(&rs_built),
            "scheme round-trip output must preserve bytes for fixture: {fixture}"
        );

        let ts_from_rs = run_ts_oracle(&json!({
            "module": "scheme",
            "operation": "parse",
            "input": rs_built,
        }))?;

        assert_eq!(
            ts.get("version").and_then(|v| v.as_str()),
            ts_from_rs.get("version").and_then(|v| v.as_str())
        );
        assert_eq!(
            ts.get("lastUpgradeVersion").and_then(|v| v.as_str()),
            ts_from_rs
                .get("lastUpgradeVersion")
                .and_then(|v| v.as_str())
        );
    }

    Ok(())
}

#[test]
fn scheme_management_parse_and_build_match_ts_semantics() -> Result<()> {
    let plist = read_fixture("src/scheme/__tests__/fixtures/xcschememanagement.plist")?;

    let ts = run_ts_oracle(&json!({
        "module": "scheme",
        "operation": "parseManagement",
        "input": plist,
    }))?;

    let rs = xcode::scheme::parse_management(&plist)?;
    let rs_json = serde_json::to_value(&rs)?;

    assert_eq!(canonicalize_json(&ts), canonicalize_json(&rs_json));

    let ts_built = run_ts_oracle(&json!({
        "module": "scheme",
        "operation": "buildManagement",
        "value": ts,
    }))?
    .as_str()
    .expect("scheme buildManagement should return plist string")
    .to_string();

    let rs_built = xcode::scheme::build_management(&rs)?;

    let ts_from_ts_built = run_ts_oracle(&json!({
        "module": "scheme",
        "operation": "parseManagement",
        "input": ts_built,
    }))?;

    let ts_from_rs_built = run_ts_oracle(&json!({
        "module": "scheme",
        "operation": "parseManagement",
        "input": rs_built,
    }))?;

    assert_eq!(
        canonicalize_json(&ts_from_ts_built),
        canonicalize_json(&ts_from_rs_built)
    );

    Ok(())
}
