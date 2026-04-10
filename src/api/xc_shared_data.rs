use std::fs;
use std::path::{Path, PathBuf};

use crate::{breakpoints, scheme, settings};

use super::data_helpers::{self, require_path};
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

    fn base(&self) -> anyhow::Result<&Path> {
        require_path(&self.file_path, "XCSharedData")
    }

    pub fn list_schemes(&self) -> anyhow::Result<Vec<XCSchemeFile>> {
        data_helpers::list_schemes(self.base()?)
    }

    pub fn get_scheme(&self, name: &str) -> anyhow::Result<Option<XCSchemeFile>> {
        let path = self.base()?.join("xcschemes").join(format!("{}.xcscheme", name));
        if !path.exists() {
            return Ok(None);
        }
        Ok(Some(XCSchemeFile::open(path)?))
    }

    pub fn save_scheme(&self, name: &str, scheme_file: &XCSchemeFile) -> anyhow::Result<()> {
        data_helpers::save_scheme(self.base()?, name, scheme_file)
    }

    pub fn load_scheme_management(&self) -> anyhow::Result<Option<scheme::XCSchemeManagement>> {
        data_helpers::load_scheme_management(self.base()?)
    }

    pub fn save_scheme_management(
        &self,
        management: &scheme::XCSchemeManagement,
    ) -> anyhow::Result<()> {
        data_helpers::save_scheme_management(self.base()?, management)
    }

    pub fn load_breakpoints(&self) -> anyhow::Result<Option<breakpoints::XCBreakpointList>> {
        data_helpers::load_breakpoints(self.base()?)
    }

    pub fn save_breakpoints(&self, list: &breakpoints::XCBreakpointList) -> anyhow::Result<()> {
        data_helpers::save_breakpoints(self.base()?, list)
    }

    pub fn load_workspace_settings(&self) -> anyhow::Result<Option<settings::WorkspaceSettings>> {
        let path = self.base()?.join("WorkspaceSettings.xcsettings");
        if !path.exists() {
            return Ok(None);
        }
        let plist = fs::read_to_string(path)?;
        Ok(Some(settings::parse(&plist)?))
    }

    pub fn save_workspace_settings(&self, ws: &settings::WorkspaceSettings) -> anyhow::Result<()> {
        let base = self.base()?;
        fs::create_dir_all(base)?;
        fs::write(
            base.join("WorkspaceSettings.xcsettings"),
            settings::build(ws)?,
        )?;
        Ok(())
    }
}
