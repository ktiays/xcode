use std::collections::BTreeMap;

use plist::Value as PlistValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    #[serde(rename = "BuildSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_system_type: Option<String>,
    #[serde(rename = "DerivedDataLocationStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_data_location_style: Option<String>,
    #[serde(rename = "DerivedDataCustomLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_data_custom_location: Option<String>,
    #[serde(rename = "IDEWorkspaceSharedSettings_AutocreateContextsIfNeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ide_workspace_shared_settings_autocreate_contexts_if_needed: Option<bool>,
    #[serde(rename = "PreviewsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews_enabled: Option<bool>,
    #[serde(rename = "BuildLocationStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_location_style: Option<String>,
    #[serde(rename = "LiveSourceIssuesEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_source_issues_enabled: Option<bool>,
    #[serde(rename = "GatherCoverageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gather_coverage_data: Option<bool>,
    #[serde(rename = "IDEIndexEnableInWorkspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ide_index_enable_in_workspace: Option<bool>,
}

pub fn parse(plist_string: &str) -> anyhow::Result<WorkspaceSettings> {
    let value = plist::Value::from_reader_xml(plist_string.as_bytes())?;
    let dict = value
        .into_dictionary()
        .ok_or_else(|| anyhow::anyhow!("Expected plist dictionary"))?;

    Ok(WorkspaceSettings {
        build_system_type: get_string(&dict, "BuildSystemType"),
        derived_data_location_style: get_string(&dict, "DerivedDataLocationStyle"),
        derived_data_custom_location: get_string(&dict, "DerivedDataCustomLocation"),
        ide_workspace_shared_settings_autocreate_contexts_if_needed: get_bool(
            &dict,
            "IDEWorkspaceSharedSettings_AutocreateContextsIfNeeded",
        ),
        previews_enabled: get_bool(&dict, "PreviewsEnabled"),
        build_location_style: get_string(&dict, "BuildLocationStyle"),
        live_source_issues_enabled: get_bool(&dict, "LiveSourceIssuesEnabled"),
        gather_coverage_data: get_bool(&dict, "GatherCoverageData"),
        ide_index_enable_in_workspace: get_bool(&dict, "IDEIndexEnableInWorkspace"),
    })
}

pub fn build(settings: &WorkspaceSettings) -> anyhow::Result<String> {
    let mut dict = BTreeMap::<String, PlistValue>::new();

    if let Some(v) = &settings.build_system_type {
        dict.insert("BuildSystemType".to_string(), PlistValue::String(v.clone()));
    }
    if let Some(v) = &settings.derived_data_location_style {
        dict.insert(
            "DerivedDataLocationStyle".to_string(),
            PlistValue::String(v.clone()),
        );
    }
    if let Some(v) = &settings.derived_data_custom_location {
        dict.insert(
            "DerivedDataCustomLocation".to_string(),
            PlistValue::String(v.clone()),
        );
    }
    if let Some(v) = settings.ide_workspace_shared_settings_autocreate_contexts_if_needed {
        dict.insert(
            "IDEWorkspaceSharedSettings_AutocreateContextsIfNeeded".to_string(),
            PlistValue::Boolean(v),
        );
    }
    if let Some(v) = settings.previews_enabled {
        dict.insert("PreviewsEnabled".to_string(), PlistValue::Boolean(v));
    }
    if let Some(v) = &settings.build_location_style {
        dict.insert(
            "BuildLocationStyle".to_string(),
            PlistValue::String(v.clone()),
        );
    }
    if let Some(v) = settings.live_source_issues_enabled {
        dict.insert(
            "LiveSourceIssuesEnabled".to_string(),
            PlistValue::Boolean(v),
        );
    }
    if let Some(v) = settings.gather_coverage_data {
        dict.insert("GatherCoverageData".to_string(), PlistValue::Boolean(v));
    }
    if let Some(v) = settings.ide_index_enable_in_workspace {
        dict.insert(
            "IDEIndexEnableInWorkspace".to_string(),
            PlistValue::Boolean(v),
        );
    }

    let mut bytes = Vec::new();
    PlistValue::Dictionary(dict.into_iter().collect()).to_writer_xml(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}

fn get_string(dict: &plist::Dictionary, key: &str) -> Option<String> {
    dict.get(key)
        .and_then(PlistValue::as_string)
        .map(str::to_string)
}

fn get_bool(dict: &plist::Dictionary, key: &str) -> Option<bool> {
    dict.get(key).and_then(PlistValue::as_boolean)
}

#[cfg(test)]
mod tests {
    use super::{build, parse};

    #[test]
    fn round_trip() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict><key>BuildSystemType</key><string>Original</string></dict></plist>"#;

        let parsed = parse(input).unwrap();
        assert_eq!(parsed.build_system_type.as_deref(), Some("Original"));

        let output = build(&parsed).unwrap();
        assert!(output.contains("BuildSystemType"));
    }
}
