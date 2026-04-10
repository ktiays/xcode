mod common;

use anyhow::Result;
use serde_json::json;

use common::{canonicalize_json, normalize_newlines, read_fixture, run_ts_oracle};

#[test]
fn json_parse_matches_ts_on_key_fixtures() -> Result<()> {
    let fixtures = [
        "src/json/__tests__/fixtures/project.pbxproj",
        "src/json/__tests__/fixtures/006-spm.pbxproj",
        "src/json/__tests__/fixtures/007-xcode16.pbxproj",
    ];

    for fixture in fixtures {
        let input = read_fixture(fixture)?;

        let ts = run_ts_oracle(&json!({
            "module": "json",
            "operation": "parse",
            "input": input,
        }))?;

        let rs = xcode::json::parse(&input)?;

        assert_eq!(
            canonicalize_json(&ts),
            canonicalize_json(&rs),
            "json parse mismatch for fixture: {fixture}"
        );
    }

    Ok(())
}

#[test]
fn json_build_is_byte_identical_to_ts_for_representative_fixture() -> Result<()> {
    let input = read_fixture("src/json/__tests__/fixtures/project.pbxproj")?;
    let parsed = xcode::json::parse(&input)?;

    let ts_built = run_ts_oracle(&json!({
        "module": "json",
        "operation": "build",
        "value": parsed,
    }))?
    .as_str()
    .expect("TS oracle json.build must return a string")
    .to_string();

    let rs_built = xcode::json::build(&xcode::json::parse(&input)?);

    assert_eq!(normalize_newlines(&ts_built), normalize_newlines(&rs_built));

    Ok(())
}
