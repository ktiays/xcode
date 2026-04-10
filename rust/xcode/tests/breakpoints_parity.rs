mod common;

use anyhow::Result;
use serde_json::json;

use common::{canonicalize_json, normalize_newlines, read_fixture, run_ts_oracle};

#[test]
fn breakpoints_parse_and_build_match_ts() -> Result<()> {
    let xml = read_fixture("src/breakpoints/__tests__/fixtures/Breakpoints_v2.xcbkptlist")?;

    let ts = run_ts_oracle(&json!({
        "module": "breakpoints",
        "operation": "parse",
        "input": xml,
    }))?;

    let rs = xcode::breakpoints::parse(&xml)?;
    let rs_json = serde_json::to_value(&rs)?;

    assert_eq!(canonicalize_json(&ts), canonicalize_json(&rs_json));

    let ts_built = run_ts_oracle(&json!({
        "module": "breakpoints",
        "operation": "build",
        "value": ts,
    }))?
    .as_str()
    .expect("breakpoints build should return xml string")
    .to_string();

    let rs_built = xcode::breakpoints::build(&rs);

    assert_eq!(normalize_newlines(&ts_built), normalize_newlines(&rs_built));

    Ok(())
}
