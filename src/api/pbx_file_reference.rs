use serde_json::Value;

use super::AbstractObject;

#[derive(Debug, Clone)]
pub struct PBXFileReference {
    pub inner: AbstractObject,
}

impl PBXFileReference {
    pub fn new(inner: AbstractObject) -> Self {
        Self { inner }
    }

    pub fn path(&self) -> Option<String> {
        self.inner
            .props
            .get("path")
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    pub fn source_tree(&self) -> Option<String> {
        self.inner
            .props
            .get("sourceTree")
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    pub fn display_name(&self) -> String {
        self.inner
            .props
            .get("name")
            .and_then(Value::as_str)
            .or_else(|| self.inner.props.get("path").and_then(Value::as_str))
            .unwrap_or("PBXFileReference")
            .to_string()
    }
}
