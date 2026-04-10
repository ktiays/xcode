use serde_json::Value;

use super::AbstractObject;

#[derive(Debug, Clone)]
pub struct PBXProject {
    pub inner: AbstractObject,
}

impl PBXProject {
    pub fn new(inner: AbstractObject) -> Self {
        Self { inner }
    }

    pub fn targets(&self) -> Vec<String> {
        self.inner
            .props
            .get("targets")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn main_group(&self) -> Option<String> {
        self.inner
            .props
            .get("mainGroup")
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    pub fn get_display_name(&self) -> String {
        self.inner
            .props
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("Project")
            .to_string()
    }
}
