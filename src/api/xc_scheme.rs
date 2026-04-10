use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::scheme;

#[derive(Debug, Clone)]
pub struct XCSchemeFile {
    pub file_path: Option<PathBuf>,
    pub scheme: scheme::XCScheme,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildableReference {
    pub buildable_identifier: String,
    pub blueprint_identifier: String,
    pub buildable_name: String,
    pub blueprint_name: String,
    pub referenced_container: String,
}

pub fn create_buildable_reference(
    blueprint_identifier: impl Into<String>,
    buildable_name: impl Into<String>,
    blueprint_name: impl Into<String>,
    referenced_container: impl Into<String>,
) -> BuildableReference {
    BuildableReference {
        buildable_identifier: "primary".to_string(),
        blueprint_identifier: blueprint_identifier.into(),
        buildable_name: buildable_name.into(),
        blueprint_name: blueprint_name.into(),
        referenced_container: referenced_container.into(),
    }
}

impl XCSchemeFile {
    pub fn open(file_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file_path = file_path.as_ref().to_path_buf();
        let xml = fs::read_to_string(&file_path)?;
        let scheme = scheme::parse(&xml)?;

        Ok(Self {
            file_path: Some(file_path),
            scheme,
        })
    }

    pub fn save(&self, file_path: Option<impl AsRef<Path>>) -> anyhow::Result<()> {
        let target_path = if let Some(path) = file_path {
            path.as_ref().to_path_buf()
        } else {
            self.file_path
                .clone()
                .ok_or_else(|| anyhow::anyhow!("No file path specified"))?
        };

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let xml = scheme::build(&self.scheme);
        fs::write(target_path, xml)?;
        Ok(())
    }
}
