use std::fs;
use std::path::{Path, PathBuf};

use crate::workspace;

#[derive(Debug, Clone)]
pub struct IDEWorkspaceChecksFile {
    pub file_path: Option<PathBuf>,
    pub checks: workspace::IDEWorkspaceChecks,
}

impl IDEWorkspaceChecksFile {
    pub fn open(workspace_path: impl AsRef<Path>) -> anyhow::Result<Option<Self>> {
        let checks_path = workspace_path
            .as_ref()
            .join("xcshareddata")
            .join("IDEWorkspaceChecks.plist");

        if !checks_path.exists() {
            return Ok(None);
        }

        let plist = fs::read_to_string(&checks_path)?;
        let checks = workspace::parse_checks(&plist)?;

        Ok(Some(Self {
            file_path: Some(checks_path),
            checks,
        }))
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

        let plist = workspace::build_checks(&self.checks)?;
        fs::write(target_path, plist)?;
        Ok(())
    }
}
