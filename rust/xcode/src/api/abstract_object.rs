use serde_json::Value;

#[derive(Debug, Clone)]
pub struct AbstractObject {
    pub uuid: String,
    pub isa: String,
    pub props: Value,
}

impl AbstractObject {
    pub fn new(uuid: impl Into<String>, isa: impl Into<String>, props: Value) -> Self {
        Self {
            uuid: uuid.into(),
            isa: isa.into(),
            props,
        }
    }

    pub fn to_json(&self) -> Value {
        self.props.clone()
    }

    pub fn is_referencing(&self, uuid: &str) -> bool {
        references_uuid(&self.props, uuid)
    }
}

fn references_uuid(value: &Value, uuid: &str) -> bool {
    match value {
        Value::String(v) => v == uuid,
        Value::Array(arr) => arr.iter().any(|entry| references_uuid(entry, uuid)),
        Value::Object(map) => map.values().any(|entry| references_uuid(entry, uuid)),
        _ => false,
    }
}
