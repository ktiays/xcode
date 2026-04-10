use std::fs;
use std::path::{Path, PathBuf};

use crate::{breakpoints, scheme, settings};

use super::XCSchemeFile;

#[derive(Debug, Clone)]
pub struct XCSharedData {
    pub file_path: Option<PathBuf>,
}

impl XCSharedData {
    pub fn open(shared_data_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let shared_data_path = shared_data_path.as_ref().to_path_buf();
        if !shared_data_path.exists() {
            anyhow::bail!(
                "Shared data path does not exist: {}",
                shared_data_path.display()
            );
        }

        Ok(Self {
            file_path: Some(shared_data_path),
        })
    }

    pub fn list_schemes(&self) -> anyhow::Result<Vec<XCSchemeFile>> {
        let dir = self
            .file_path
            .as_ref()
            .map(|p| p.join("xcschemes"))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut schemes = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("xcscheme") {
                if let Ok(scheme) = XCSchemeFile::open(&path) {
                    schemes.push(scheme);
                }
            }
        }

        schemes.sort_by(|a, b| a.file_path.cmp(&b.file_path));
        Ok(schemes)
    }

    pub fn get_scheme(&self, name: &str) -> anyhow::Result<Option<XCSchemeFile>> {
        let path = self
            .file_path
            .as_ref()
            .map(|p| p.join("xcschemes").join(format!("{}.xcscheme", name)))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;

        if !path.exists() {
            return Ok(None);
        }

        Ok(Some(XCSchemeFile::open(path)?))
    }

    pub fn save_scheme(&self, name: &str, scheme_file: &XCSchemeFile) -> anyhow::Result<()> {
        let dir = self
            .file_path
            .as_ref()
            .map(|p| p.join("xcschemes"))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;
        fs::create_dir_all(&dir)?;
        scheme_file.save(Some(dir.join(format!("{}.xcscheme", name))))
    }

    pub fn load_scheme_management(&self) -> anyhow::Result<Option<scheme::XCSchemeManagement>> {
        let path = self
            .file_path
            .as_ref()
            .map(|p| p.join("xcschemes").join("xcschememanagement.plist"))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;

        if !path.exists() {
            return Ok(None);
        }

        let plist = fs::read_to_string(path)?;
        Ok(Some(scheme::parse_management(&plist)?))
    }

    pub fn save_scheme_management(
        &self,
        management: &scheme::XCSchemeManagement,
    ) -> anyhow::Result<()> {
        let dir = self
            .file_path
            .as_ref()
            .map(|p| p.join("xcschemes"))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;
        fs::create_dir_all(&dir)?;

        let plist = scheme::build_management(management)?;
        fs::write(dir.join("xcschememanagement.plist"), plist)?;
        Ok(())
    }

    pub fn load_breakpoints(&self) -> anyhow::Result<Option<breakpoints::XCBreakpointList>> {
        let path = self
            .file_path
            .as_ref()
            .map(|p| p.join("xcdebugger").join("Breakpoints_v2.xcbkptlist"))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;

        if !path.exists() {
            return Ok(None);
        }

        let xml = fs::read_to_string(path)?;
        Ok(Some(breakpoints::parse(&xml)?))
    }

    pub fn save_breakpoints(&self, list: &breakpoints::XCBreakpointList) -> anyhow::Result<()> {
        let dir = self
            .file_path
            .as_ref()
            .map(|p| p.join("xcdebugger"))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;

        fs::create_dir_all(&dir)?;
        fs::write(
            dir.join("Breakpoints_v2.xcbkptlist"),
            breakpoints::build(list),
        )?;
        Ok(())
    }

    pub fn load_workspace_settings(&self) -> anyhow::Result<Option<settings::WorkspaceSettings>> {
        let path = self
            .file_path
            .as_ref()
            .map(|p| p.join("WorkspaceSettings.xcsettings"))
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;

        if !path.exists() {
            return Ok(None);
        }

        let plist = fs::read_to_string(path)?;
        Ok(Some(settings::parse(&plist)?))
    }

    pub fn save_workspace_settings(&self, ws: &settings::WorkspaceSettings) -> anyhow::Result<()> {
        let base = self
            .file_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No file path set for XCSharedData"))?;

        fs::create_dir_all(base)?;
        fs::write(
            base.join("WorkspaceSettings.xcsettings"),
            settings::build(ws)?,
        )?;
        Ok(())
    }
}
