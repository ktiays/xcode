mod common;

use anyhow::Result;
use serde_json::json;

use common::{canonicalize_json, fixture_path, normalize_newlines, read_fixture, run_ts_oracle};

#[test]
fn xcconfig_parse_build_and_flatten_match_ts() -> Result<()> {
    let fixtures = [
        "src/xcconfig/__tests__/fixtures/simple.xcconfig",
        "src/xcconfig/__tests__/fixtures/conditional.xcconfig",
    ];

    for fixture in fixtures {
        let content = read_fixture(fixture)?;

        let ts = run_ts_oracle(&json!({
            "module": "xcconfig",
            "operation": "parse",
            "input": content,
        }))?;

        let rs = xcode::xcconfig::parse(&content);
        let rs_json = serde_json::to_value(&rs)?;

        assert_eq!(
            canonicalize_json(&ts),
            canonicalize_json(&rs_json),
            "xcconfig parse mismatch for fixture: {fixture}"
        );

        let ts_built = run_ts_oracle(&json!({
            "module": "xcconfig",
            "operation": "build",
            "value": ts,
        }))?
        .as_str()
        .expect("xcconfig build should return text")
        .to_string();

        let rs_built = xcode::xcconfig::build(&rs);

        assert_eq!(normalize_newlines(&ts_built), normalize_newlines(&rs_built));
    }

    let parent_fixture = fixture_path("src/xcconfig/__tests__/fixtures/Parent.xcconfig");
    let parent_fixture_str = parent_fixture
        .to_str()
        .expect("fixture path should be valid UTF-8");

    let ts_parse_file = run_ts_oracle(&json!({
        "module": "xcconfig",
        "operation": "parseFile",
        "filePath": parent_fixture_str,
    }))?;

    let rs_parse_file = xcode::xcconfig::parse_file(parent_fixture_str)?;
    let rs_parse_file_json = serde_json::to_value(&rs_parse_file)?;

    assert_eq!(
        canonicalize_json(&ts_parse_file),
        canonicalize_json(&rs_parse_file_json)
    );

    let options = xcode::xcconfig::XCConfigFlattenOptions {
        sdk: Some("iphoneos".to_string()),
        arch: Some("arm64".to_string()),
        config: Some("Debug".to_string()),
    };

    let rs_flattened = xcode::xcconfig::flatten_build_settings(&rs_parse_file, options.clone());
    let rs_flattened_json = serde_json::to_value(rs_flattened)?;

    let ts_flattened = run_ts_oracle(&json!({
        "module": "xcconfig",
        "operation": "flattenBuildSettings",
        "value": ts_parse_file,
        "options": {
            "sdk": options.sdk,
            "arch": options.arch,
            "config": options.config,
        }
    }))?;

    assert_eq!(
        canonicalize_json(&ts_flattened),
        canonicalize_json(&rs_flattened_json)
    );

    Ok(())
}
