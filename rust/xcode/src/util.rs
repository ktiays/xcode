use serde_json::{Map, Number, Value};

pub const JS_MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;

pub fn buffer_to_value(bytes: &[u8]) -> Value {
    let mut obj = Map::new();
    obj.insert("type".to_string(), Value::String("Buffer".to_string()));
    obj.insert(
        "data".to_string(),
        Value::Array(
            bytes
                .iter()
                .map(|b| Value::Number(Number::from(*b)))
                .collect(),
        ),
    );
    Value::Object(obj)
}

pub fn value_to_buffer(value: &Value) -> Option<Vec<u8>> {
    let obj = value.as_object()?;
    if obj.get("type")?.as_str()? != "Buffer" {
        return None;
    }
    let data = obj.get("data")?.as_array()?;
    let mut out = Vec::with_capacity(data.len());
    for v in data {
        let n = v.as_u64()?;
        if n > u8::MAX as u64 {
            return None;
        }
        out.push(n as u8);
    }
    Some(out)
}

pub fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
