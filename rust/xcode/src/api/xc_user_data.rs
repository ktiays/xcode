use std::path::{Path, PathBuf};

use crate::{breakpoints, scheme};

use super::data_helpers::{self, require_path};
use super::XCSchemeFile;

#[derive(Debug, Clone)]
pub struct XCUserData {
    pub file_path: Option<PathBuf>,
}

impl XCUserData {
    pub fn open(user_data_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let user_data_path = user_data_path.as_ref().to_path_buf();
        if !user_data_path.exists() {
            anyhow::bail!(
                "User data path does not exist: {}",
                user_data_path.display()
            );
        }

        Ok(Self {
            file_path: Some(user_data_path),
        })
    }

    fn base(&self) -> anyhow::Result<&Path> {
        require_path(&self.file_path, "XCUserData")
    }

    pub fn list_schemes(&self) -> anyhow::Result<Vec<XCSchemeFile>> {
        data_helpers::list_schemes(self.base()?)
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
}
