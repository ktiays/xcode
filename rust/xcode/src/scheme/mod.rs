use std::collections::BTreeMap;

use plist::Value as PlistValue;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XCScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_upgrade_version: Option<String>,
    #[serde(skip)]
    pub raw_xml: Option<String>,
}

pub fn parse(xml: &str) -> anyhow::Result<XCScheme> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::<u8>::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => {
                if start.name().as_ref() != b"Scheme" {
                    anyhow::bail!("Invalid xcscheme file: root element must be <Scheme>");
                }

                let mut scheme = XCScheme::default();
                for attr in start.attributes() {
                    let attr = attr?;
                    let key = std::str::from_utf8(attr.key.as_ref())?;
                    let value = attr
                        .decode_and_unescape_value(reader.decoder())?
                        .to_string();

                    match key {
                        "version" => scheme.version = Some(value),
                        "LastUpgradeVersion" => scheme.last_upgrade_version = Some(value),
                        _ => {}
                    }
                }

                scheme.raw_xml = Some(xml.to_string());
                return Ok(scheme);
            }
            Ok(Event::Decl(_) | Event::Comment(_) | Event::DocType(_) | Event::Text(_)) => {}
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(err) => return Err(err.into()),
        }

        buf.clear();
    }

    anyhow::bail!("Invalid xcscheme file: root element must be <Scheme>")
}

pub fn build(scheme: &XCScheme) -> String {
    if let Some(raw) = &scheme.raw_xml {
        return raw.clone();
    }

    let mut lines = Vec::<String>::new();
    lines.push("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string());
    lines.push("<Scheme".to_string());

    if let Some(last_upgrade_version) = &scheme.last_upgrade_version {
        lines.push(format!(
            "   LastUpgradeVersion = \"{}\"",
            last_upgrade_version
        ));
    }
    if let Some(version) = &scheme.version {
        lines.push(format!("   version = \"{}\"", version));
    }

    if let Some(last) = lines.last_mut() {
        last.push('>');
    }

    lines.push("</Scheme>".to_string());
    lines.join("\n") + "\n"
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemeUserState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_hint: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppressBuildableAutocreation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct XCSchemeManagement {
    #[serde(rename = "SchemeUserState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_user_state: Option<BTreeMap<String, SchemeUserState>>,
    #[serde(rename = "SuppressBuildableAutocreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_buildable_autocreation: Option<BTreeMap<String, SuppressBuildableAutocreation>>,
}

pub fn parse_management(plist_string: &str) -> anyhow::Result<XCSchemeManagement> {
    let value = plist::Value::from_reader_xml(plist_string.as_bytes())?;
    let dict = value
        .into_dictionary()
        .ok_or_else(|| anyhow::anyhow!("Expected plist dictionary"))?;

    let mut result = XCSchemeManagement::default();

    if let Some(user_state) = dict
        .get("SchemeUserState")
        .and_then(PlistValue::as_dictionary)
    {
        let mut map = BTreeMap::new();
        for (key, value) in user_state {
            if let Some(entry) = value.as_dictionary() {
                map.insert(
                    key.clone(),
                    SchemeUserState {
                        is_shown: entry.get("isShown").and_then(PlistValue::as_boolean),
                        order_hint: entry
                            .get("orderHint")
                            .and_then(PlistValue::as_signed_integer),
                    },
                );
            }
        }
        result.scheme_user_state = Some(map);
    }

    if let Some(suppress) = dict
        .get("SuppressBuildableAutocreation")
        .and_then(PlistValue::as_dictionary)
    {
        let mut map = BTreeMap::new();
        for (key, value) in suppress {
            if let Some(entry) = value.as_dictionary() {
                map.insert(
                    key.clone(),
                    SuppressBuildableAutocreation {
                        primary: entry.get("primary").and_then(PlistValue::as_boolean),
                    },
                );
            }
        }
        result.suppress_buildable_autocreation = Some(map);
    }

    Ok(result)
}

pub fn build_management(management: &XCSchemeManagement) -> anyhow::Result<String> {
    let mut root = plist::Dictionary::new();

    if let Some(user_state) = &management.scheme_user_state {
        let mut map = plist::Dictionary::new();
        for (key, value) in user_state {
            let mut entry = plist::Dictionary::new();
            if let Some(is_shown) = value.is_shown {
                entry.insert("isShown".to_string(), PlistValue::Boolean(is_shown));
            }
            if let Some(order_hint) = value.order_hint {
                entry.insert(
                    "orderHint".to_string(),
                    PlistValue::Integer(order_hint.into()),
                );
            }
            map.insert(key.clone(), PlistValue::Dictionary(entry));
        }
        root.insert("SchemeUserState".to_string(), PlistValue::Dictionary(map));
    }

    if let Some(suppress) = &management.suppress_buildable_autocreation {
        let mut map = plist::Dictionary::new();
        for (key, value) in suppress {
            let mut entry = plist::Dictionary::new();
            if let Some(primary) = value.primary {
                entry.insert("primary".to_string(), PlistValue::Boolean(primary));
            }
            map.insert(key.clone(), PlistValue::Dictionary(entry));
        }
        root.insert(
            "SuppressBuildableAutocreation".to_string(),
            PlistValue::Dictionary(map),
        );
    }

    let mut bytes = Vec::new();
    PlistValue::Dictionary(root).to_writer_xml(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::{build, parse};

    #[test]
    fn parse_scheme_root() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Scheme LastUpgradeVersion="1600" version="1.7"></Scheme>
"#;

        let scheme = parse(xml).unwrap();
        assert_eq!(scheme.version.as_deref(), Some("1.7"));
        assert_eq!(scheme.last_upgrade_version.as_deref(), Some("1600"));

        let out = build(&scheme);
        assert!(out.contains("<Scheme"));
    }
}
