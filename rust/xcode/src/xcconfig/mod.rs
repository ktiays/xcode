use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

static INCLUDE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^#include(\?)?\s+\"(.+)\"$"#).unwrap());
static SETTING_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*(?:\[[^\]]+\])*)\s*=\s*(.*)$").unwrap());
static CONDITION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([a-zA-Z]+)=([^\]]+)\]").unwrap());
static COMMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//.*").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XCConfig {
    pub includes: Vec<XCConfigIncludeResolved>,
    pub build_settings: Vec<XCConfigSetting>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XCConfigInclude {
    pub path: String,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XCConfigIncludeResolved {
    pub include: XCConfigInclude,
    pub resolved_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<XCConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XCConfigSetting {
    pub key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<XCConfigCondition>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct XCConfigCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct XCConfigFlattenOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

pub fn parse(content: &str) -> XCConfig {
    let mut includes = Vec::new();
    let mut build_settings = Vec::new();

    for raw_line in content.lines() {
        let line = COMMENT_RE.replace(raw_line, "").trim().to_string();
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = INCLUDE_RE.captures(&line) {
            let optional = caps.get(1).is_some();
            let include_path = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            includes.push(XCConfigIncludeResolved {
                include: XCConfigInclude {
                    path: include_path.to_string(),
                    optional,
                },
                resolved_path: include_path.to_string(),
                config: None,
            });
            continue;
        }

        if let Some(caps) = SETTING_RE.captures(&line) {
            let key_with_conditions = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
            let value = caps
                .get(2)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            let conditions = parse_conditions(key_with_conditions);
            let key = CONDITION_RE
                .replace_all(key_with_conditions, "")
                .to_string();

            build_settings.push(XCConfigSetting {
                key,
                value,
                conditions: if conditions.is_empty() {
                    None
                } else {
                    Some(conditions)
                },
            });
        }
    }

    XCConfig {
        includes,
        build_settings,
    }
}

fn parse_conditions(key_with_conditions: &str) -> Vec<XCConfigCondition> {
    let mut conditions = Vec::new();

    for caps in CONDITION_RE.captures_iter(key_with_conditions) {
        let condition_type = caps
            .get(1)
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();
        let condition_value = caps.get(2).map(|m| m.as_str().to_string());

        let mut condition = XCConfigCondition::default();
        match condition_type.as_str() {
            "sdk" => condition.sdk = condition_value,
            "arch" => condition.arch = condition_value,
            "config" => condition.config = condition_value,
            _ => {}
        }

        if condition.sdk.is_some() || condition.arch.is_some() || condition.config.is_some() {
            conditions.push(condition);
        }
    }

    conditions
}

pub fn parse_file(file_path: impl AsRef<Path>) -> anyhow::Result<XCConfig> {
    let mut visited = HashSet::new();
    parse_file_inner(file_path.as_ref(), &mut visited)
}

fn parse_file_inner(file_path: &Path, visited: &mut HashSet<PathBuf>) -> anyhow::Result<XCConfig> {
    let absolute_path = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        std::env::current_dir()?.join(file_path)
    };

    if visited.contains(&absolute_path) {
        anyhow::bail!("Circular include detected: {}", absolute_path.display());
    }

    let content = fs::read_to_string(&absolute_path)?;
    let mut config = parse(&content);
    let file_dir = absolute_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    visited.insert(absolute_path.clone());

    let mut resolved = Vec::with_capacity(config.includes.len());
    for include in config.includes {
        let resolved_path = file_dir.join(&include.include.path);

        if !resolved_path.exists() {
            if include.include.optional {
                resolved.push(XCConfigIncludeResolved {
                    include: include.include,
                    resolved_path: resolved_path.display().to_string(),
                    config: None,
                });
                continue;
            }
            anyhow::bail!("Include file not found: {}", resolved_path.display());
        }

        let included_config = parse_file_inner(&resolved_path, visited)?;
        resolved.push(XCConfigIncludeResolved {
            include: include.include,
            resolved_path: resolved_path.display().to_string(),
            config: Some(included_config),
        });
    }

    visited.remove(&absolute_path);
    config.includes = resolved;
    Ok(config)
}

pub fn build(config: &XCConfig) -> String {
    let mut out = String::new();

    for include in &config.includes {
        if include.include.optional {
            out.push_str(&format!("#include? \"{}\"\n", include.include.path));
        } else {
            out.push_str(&format!("#include \"{}\"\n", include.include.path));
        }
    }

    for setting in &config.build_settings {
        out.push_str(&setting.key);
        if let Some(conditions) = &setting.conditions {
            for condition in conditions {
                if let Some(sdk) = &condition.sdk {
                    out.push_str(&format!("[sdk={}]", sdk));
                }
                if let Some(arch) = &condition.arch {
                    out.push_str(&format!("[arch={}]", arch));
                }
                if let Some(config) = &condition.config {
                    out.push_str(&format!("[config={}]", config));
                }
            }
        }
        out.push_str(&format!(" = {}\n", setting.value));
    }

    out
}

pub fn flatten_build_settings(
    config: &XCConfig,
    options: XCConfigFlattenOptions,
) -> std::collections::HashMap<String, String> {
    let mut settings = std::collections::HashMap::new();

    for include in &config.includes {
        if let Some(included) = &include.config {
            let nested = flatten_build_settings(included, options.clone());
            for (k, v) in nested {
                settings.insert(k, v);
            }
        }
    }

    for setting in &config.build_settings {
        if matches_conditions(setting.conditions.as_deref(), &options) {
            let value = resolve_inherited(&setting.value, &setting.key, &settings);
            settings.insert(setting.key.clone(), value);
        }
    }

    settings
}

fn matches_conditions(
    conditions: Option<&[XCConfigCondition]>,
    options: &XCConfigFlattenOptions,
) -> bool {
    let Some(conditions) = conditions else {
        return true;
    };

    conditions.iter().all(|condition| {
        if let Some(sdk) = &condition.sdk {
            let Some(option_sdk) = options.sdk.as_deref() else {
                return false;
            };
            if !match_wildcard(option_sdk, sdk) {
                return false;
            }
        }

        if let Some(arch) = &condition.arch {
            let Some(option_arch) = options.arch.as_deref() else {
                return false;
            };
            if !match_wildcard(option_arch, arch) {
                return false;
            }
        }

        if let Some(config) = &condition.config {
            let Some(option_config) = options.config.as_deref() else {
                return false;
            };
            if !match_wildcard(option_config, config) {
                return false;
            }
        }

        true
    })
}

fn match_wildcard(value: &str, pattern: &str) -> bool {
    let regex_pattern = regex::escape(pattern).replace("\\*", ".*");
    let regex = Regex::new(&format!("(?i)^{}$", regex_pattern)).expect("valid wildcard regex");
    regex.is_match(value)
}

fn resolve_inherited(
    value: &str,
    key: &str,
    existing_settings: &std::collections::HashMap<String, String>,
) -> String {
    if !value.contains("$(inherited)") {
        return value.to_string();
    }

    let existing = existing_settings.get(key).map(|s| s.as_str()).unwrap_or_default();
    value.replace("$(inherited)", existing)
}

#[cfg(test)]
mod tests {
    use super::{build, parse};

    #[test]
    fn parse_simple() {
        let config = parse("PRODUCT_NAME = MyApp\n");
        assert_eq!(config.build_settings[0].key, "PRODUCT_NAME");
        assert_eq!(config.build_settings[0].value, "MyApp");
    }

    #[test]
    fn build_simple() {
        let config = parse("PRODUCT_NAME = MyApp\n");
        let out = build(&config);
        assert_eq!(out, "PRODUCT_NAME = MyApp\n");
    }
}
