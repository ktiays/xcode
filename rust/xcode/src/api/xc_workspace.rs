use std::fs;
use std::path::{Path, PathBuf};

use crate::workspace;

#[derive(Debug, Clone)]
pub struct XCWorkspaceFile {
    pub file_path: Option<PathBuf>,
    pub workspace: workspace::XCWorkspace,
}

impl XCWorkspaceFile {
    pub fn open(workspace_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let workspace_path = workspace_path.as_ref();
        let contents_path = workspace_path.join("contents.xcworkspacedata");
        if !contents_path.exists() {
            anyhow::bail!("Invalid workspace: missing contents.xcworkspacedata");
        }

        let xml = fs::read_to_string(&contents_path)?;
        let workspace = workspace::parse(&xml)?;

        Ok(Self {
            file_path: Some(workspace_path.to_path_buf()),
            workspace,
        })
    }

    pub fn save(&self, workspace_path: Option<impl AsRef<Path>>) -> anyhow::Result<()> {
        let target_path = if let Some(path) = workspace_path {
            path.as_ref().to_path_buf()
        } else {
            self.file_path
                .clone()
                .ok_or_else(|| anyhow::anyhow!("No file path specified"))?
        };

        fs::create_dir_all(&target_path)?;
        let xml = workspace::build(&self.workspace);
        fs::write(target_path.join("contents.xcworkspacedata"), xml)?;
        Ok(())
    }

    pub fn add_project(&mut self, project_location: impl Into<String>) {
        let mut refs = self.workspace.file_refs.take().unwrap_or_default();
        refs.push(workspace::FileRef {
            location: project_location.into(),
        });
        self.workspace.file_refs = Some(refs);
    }

    pub fn has_project(&self, project_name_or_location: &str) -> bool {
        self.workspace
            .file_refs
            .as_ref()
            .map(|refs| {
                refs.iter().any(|file_ref| {
                    file_ref.location == project_name_or_location
                        || file_ref.location.ends_with(project_name_or_location)
                })
            })
            .unwrap_or(false)
    }
}
