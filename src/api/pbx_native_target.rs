use serde_json::Value;

use super::AbstractObject;

#[derive(Debug, Clone)]
pub struct PBXNativeTarget {
    pub inner: AbstractObject,
}

impl PBXNativeTarget {
    pub fn new(inner: AbstractObject) -> Self {
        Self { inner }
    }

    pub fn name(&self) -> Option<String> {
        self.inner
            .props
            .get("name")
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    pub fn product_type(&self) -> Option<String> {
        self.inner
            .props
            .get("productType")
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    pub fn is_watch_os_target(&self) -> bool {
        matches!(
            self.product_type().as_deref(),
            Some("com.apple.product-type.application.watchapp")
                | Some("com.apple.product-type.application.watchapp2")
                | Some("com.apple.product-type.application.watchapp2-container")
                | Some("com.apple.product-type.watchkit-extension")
                | Some("com.apple.product-type.watchkit2-extension")
        )
    }

    pub fn dependencies(&self) -> Vec<String> {
        self.inner
            .props
            .get("dependencies")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    }
}
