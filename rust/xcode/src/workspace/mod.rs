use std::collections::{BTreeMap, HashMap};

use plist::Value as PlistValue;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XCWorkspace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_refs: Option<Vec<FileRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileRef {
    pub location: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_refs: Option<Vec<FileRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Workspace")]
struct WorkspaceXml {
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "FileRef", default)]
    file_refs: Vec<FileRefXml>,
    #[serde(rename = "Group", default)]
    groups: Vec<GroupXml>,
}

#[derive(Debug, Deserialize)]
struct FileRefXml {
    #[serde(rename = "@location")]
    location: String,
}

#[derive(Debug, Deserialize)]
struct GroupXml {
    #[serde(rename = "@location")]
    location: Option<String>,
    #[serde(rename = "@name")]
    name: Option<String>,
    #[serde(rename = "FileRef", default)]
    file_refs: Vec<FileRefXml>,
    #[serde(rename = "Group", default)]
    groups: Vec<GroupXml>,
}

pub fn parse(xml: &str) -> anyhow::Result<XCWorkspace> {
    let parsed: WorkspaceXml = from_str(xml)?;

    Ok(XCWorkspace {
        version: parsed.version,
        file_refs: if parsed.file_refs.is_empty() {
            None
        } else {
            Some(
                parsed
                    .file_refs
                    .into_iter()
                    .map(|f| FileRef {
                        location: f.location,
                    })
                    .collect(),
            )
        },
        groups: if parsed.groups.is_empty() {
            None
        } else {
            Some(parsed.groups.into_iter().map(group_from_xml).collect())
        },
    })
}

fn group_from_xml(group: GroupXml) -> Group {
    Group {
        location: group.location,
        name: group.name,
        file_refs: if group.file_refs.is_empty() {
            None
        } else {
            Some(
                group
                    .file_refs
                    .into_iter()
                    .map(|f| FileRef {
                        location: f.location,
                    })
                    .collect(),
            )
        },
        groups: if group.groups.is_empty() {
            None
        } else {
            Some(group.groups.into_iter().map(group_from_xml).collect())
        },
    }
}

pub fn build(workspace: &XCWorkspace) -> String {
    let mut lines = Vec::<String>::new();

    lines.push("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string());

    let mut attrs = Vec::<String>::new();
    if let Some(version) = &workspace.version {
        attrs.push(format!("version = \"{}\"", version));
    }

    if attrs.is_empty() {
        lines.push("<Workspace>".to_string());
    } else {
        lines.push("<Workspace".to_string());
        for attr in attrs {
            lines.push(format!("   {}", attr));
        }
        if let Some(last) = lines.last_mut() {
            last.push('>');
        }
    }

    if let Some(file_refs) = &workspace.file_refs {
        for file_ref in file_refs {
            lines.extend(build_file_ref(file_ref, 1));
        }
    }

    if let Some(groups) = &workspace.groups {
        for group in groups {
            lines.extend(build_group(group, 1));
        }
    }

    lines.push("</Workspace>".to_string());
    lines.join("\n") + "\n"
}

fn build_file_ref(file_ref: &FileRef, depth: usize) -> Vec<String> {
    let indent = "   ".repeat(depth);
    vec![
        format!("{}<FileRef", indent),
        format!(
            "{}   location = \"{}\">",
            indent,
            escape_xml(&file_ref.location)
        ),
        format!("{}</FileRef>", indent),
    ]
}

fn build_group(group: &Group, depth: usize) -> Vec<String> {
    let mut lines = Vec::<String>::new();
    let indent = "   ".repeat(depth);

    let mut attrs = Vec::<String>::new();
    if let Some(location) = &group.location {
        attrs.push(format!("location = \"{}\"", escape_xml(location)));
    }
    if let Some(name) = &group.name {
        attrs.push(format!("name = \"{}\"", escape_xml(name)));
    }

    lines.push(format!("{}<Group", indent));
    for attr in attrs {
        lines.push(format!("{}   {}", indent, attr));
    }
    if let Some(last) = lines.last_mut() {
        last.push('>');
    }

    if let Some(file_refs) = &group.file_refs {
        for file_ref in file_refs {
            lines.extend(build_file_ref(file_ref, depth + 1));
        }
    }

    if let Some(groups) = &group.groups {
        for nested in groups {
            lines.extend(build_group(nested, depth + 1));
        }
    }

    lines.push(format!("{}</Group>", indent));
    lines
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub type IDEWorkspaceChecks = HashMap<String, bool>;

pub fn parse_checks(plist_string: &str) -> anyhow::Result<IDEWorkspaceChecks> {
    let value = plist::Value::from_reader_xml(plist_string.as_bytes())?;
    let dict = value
        .into_dictionary()
        .ok_or_else(|| anyhow::anyhow!("Expected plist dictionary"))?;

    let mut result = HashMap::new();
    for (key, value) in dict {
        if let Some(flag) = value.as_boolean() {
            result.insert(key, flag);
        }
    }

    Ok(result)
}

pub fn build_checks(checks: &IDEWorkspaceChecks) -> anyhow::Result<String> {
    let mut dict = BTreeMap::<String, PlistValue>::new();
    for (key, value) in checks {
        dict.insert(key.clone(), PlistValue::Boolean(*value));
    }

    let mut bytes = Vec::new();
    PlistValue::Dictionary(dict.into_iter().collect()).to_writer_xml(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::{build, parse, XCWorkspace};

    #[test]
    fn build_workspace() {
        let ws = XCWorkspace {
            version: Some("1.0".to_string()),
            file_refs: Some(vec![super::FileRef {
                location: "group:App.xcodeproj".to_string(),
            }]),
            groups: None,
        };

        let out = build(&ws);
        assert!(out.contains("<Workspace"));
        assert!(out.contains("group:App.xcodeproj"));

        let parsed = parse(&out).unwrap();
        assert_eq!(parsed.version.as_deref(), Some("1.0"));
    }
}
