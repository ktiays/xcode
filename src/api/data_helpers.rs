use std::fs;
use std::path::{Path, PathBuf};

use crate::{breakpoints, scheme};

use super::XCSchemeFile;

pub(super) fn require_path<'a>(
    file_path: &'a Option<PathBuf>,
    context: &str,
) -> anyhow::Result<&'a Path> {
    file_path
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("No file path set for {}", context))
}

pub(super) fn list_schemes(base: &Path) -> anyhow::Result<Vec<XCSchemeFile>> {
    let dir = base.join("xcschemes");
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut schemes = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("xcscheme")
            && let Ok(scheme) = XCSchemeFile::open(&path) {
                schemes.push(scheme);
            }
    }

    schemes.sort_by(|a, b| a.file_path.cmp(&b.file_path));
    Ok(schemes)
}

pub(super) fn save_scheme(
    base: &Path,
    name: &str,
    scheme_file: &XCSchemeFile,
) -> anyhow::Result<()> {
    let dir = base.join("xcschemes");
    fs::create_dir_all(&dir)?;
    scheme_file.save(Some(dir.join(format!("{}.xcscheme", name))))
}

pub(super) fn load_scheme_management(
    base: &Path,
) -> anyhow::Result<Option<scheme::XCSchemeManagement>> {
    let path = base.join("xcschemes").join("xcschememanagement.plist");
    if !path.exists() {
        return Ok(None);
    }
    let plist = fs::read_to_string(path)?;
    Ok(Some(scheme::parse_management(&plist)?))
}

pub(super) fn save_scheme_management(
    base: &Path,
    management: &scheme::XCSchemeManagement,
) -> anyhow::Result<()> {
    let dir = base.join("xcschemes");
    fs::create_dir_all(&dir)?;
    fs::write(
        dir.join("xcschememanagement.plist"),
        scheme::build_management(management)?,
    )?;
    Ok(())
}

pub(super) fn load_breakpoints(
    base: &Path,
) -> anyhow::Result<Option<breakpoints::XCBreakpointList>> {
    let path = base.join("xcdebugger").join("Breakpoints_v2.xcbkptlist");
    if !path.exists() {
        return Ok(None);
    }
    let xml = fs::read_to_string(path)?;
    Ok(Some(breakpoints::parse(&xml)?))
}

pub(super) fn save_breakpoints(
    base: &Path,
    list: &breakpoints::XCBreakpointList,
) -> anyhow::Result<()> {
    let dir = base.join("xcdebugger");
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("Breakpoints_v2.xcbkptlist"), breakpoints::build(list))?;
    Ok(())
}
