use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::util::value_to_buffer;

use super::comments::{create_reference_list, is_pbx_build_file, is_pbx_file_reference};

#[derive(Clone, Debug)]
pub struct WriterOptions {
    pub tab: String,
    pub shebang: String,
    pub skip_nullish_values: bool,
}

impl Default for WriterOptions {
    fn default() -> Self {
        Self {
            tab: "\t".to_string(),
            shebang: "!$*UTF8*$!".to_string(),
            skip_nullish_values: false,
        }
    }
}

pub struct JsonWriter<'a> {
    project: &'a Value,
    options: WriterOptions,
    indent: usize,
    contents: String,
    comments: HashMap<String, String>,
}

impl<'a> JsonWriter<'a> {
    pub fn new(project: &'a Value, options: WriterOptions) -> Self {
        let comments = create_reference_list(project);
        let mut writer = Self {
            project,
            options,
            indent: 0,
            contents: String::new(),
            comments,
        };
        writer.write_shebang();
        writer.write_project();
        writer
    }

    pub fn into_string(self) -> String {
        self.contents
    }

    fn eol() -> &'static str {
        if cfg!(windows) {
            "\r\n"
        } else {
            "\n"
        }
    }

    fn pad(&self) -> String {
        self.options.tab.repeat(self.indent)
    }

    fn println(&mut self, text: &str) {
        self.contents.push_str(&self.pad());
        self.contents.push_str(text);
        self.contents.push_str(Self::eol());
    }

    fn print(&mut self, text: &str) {
        self.contents.push_str(&self.pad());
        self.contents.push_str(text);
    }

    fn flush(&mut self, text: &str) {
        let old = self.indent;
        self.indent = 0;
        self.print(text);
        self.indent = old;
    }

    fn print_assign_ln(&mut self, key: &str, value: &str) {
        self.println(&format!("{} = {};", key, value));
    }

    fn write_shebang(&mut self) {
        self.println(&format!("// {}", self.options.shebang));
    }

    fn write_project(&mut self) {
        self.println("{");
        if let Some(obj) = self.project.as_object() {
            self.indent += 1;
            self.write_object(obj, true);
            self.indent -= 1;
        }
        self.println("}");
    }

    fn key_has_float_value(&self, key: &str) -> bool {
        key == key.to_uppercase()
            && (key.ends_with("SWIFT_VERSION")
                || key.ends_with("MARKETING_VERSION")
                || key.ends_with("_DEPLOYMENT_TARGET"))
    }

    fn write_object(&mut self, object: &Map<String, Value>, is_base: bool) {
        let mut keys: Vec<&String> = object.keys().collect();
        keys.sort_by(|a, b| sort_isa_first(a, b));

        for key in keys {
            let value = &object[key];

            if self.options.skip_nullish_values && value.is_null() {
                continue;
            }

            if let Some(buffer) = value_to_buffer(value) {
                self.print_assign_ln(&ensure_quotes(key), &format_data(&buffer));
                continue;
            }

            match value {
                Value::Array(array) => self.write_array(key, array),
                Value::Object(nested) => {
                    if !is_base && nested.is_empty() {
                        self.println(&format!("{} = {{}};", ensure_quotes(key)));
                        continue;
                    }

                    self.println(&format!("{} = {{", ensure_quotes(key)));
                    self.indent += 1;
                    if is_base && key.as_str() == "objects" {
                        self.write_pbx_objects(nested);
                    } else {
                        self.write_object(nested, false);
                    }
                    self.indent -= 1;
                    self.println("};");
                }
                Value::Number(num) => {
                    let as_string = if self.key_has_float_value(key)
                        && num.is_i64()
                        && num.as_i64().is_some()
                    {
                        format!("{}.0", num)
                    } else {
                        num.to_string()
                    };
                    self.print_assign_ln(&ensure_quotes(key), &ensure_quotes(&as_string));
                }
                _ => {
                    let scalar = scalar_to_string(value);
                    let formatted = if key.as_str() == "remoteGlobalIDString"
                        || key.as_str() == "TestTargetID"
                    {
                        ensure_quotes(&scalar)
                    } else {
                        self.format_id(&scalar)
                    };
                    self.print_assign_ln(&ensure_quotes(key), &formatted);
                }
            }
        }
    }

    fn format_id(&self, id: &str) -> String {
        if let Some(comment) = self.comments.get(id)
            && !comment.is_empty() {
                return format!("{} /* {} */", id, comment);
            }
        ensure_quotes(id)
    }

    fn write_pbx_objects(&mut self, project_objects: &Map<String, Value>) {
        let mut grouped: HashMap<String, Vec<(String, Value)>> = HashMap::new();

        for (id, object) in project_objects {
            let isa = object
                .get("isa")
                .and_then(Value::as_str)
                .unwrap_or("[unknown]")
                .to_string();
            grouped
                .entry(isa)
                .or_default()
                .push((id.clone(), object.clone()));
        }

        let mut isas: Vec<String> = grouped.keys().cloned().collect();
        isas.sort();

        for isa in isas {
            self.flush(Self::eol());
            self.flush(&format!("/* Begin {} section */{}", isa, Self::eol()));
            let mut objects = grouped.remove(&isa).unwrap_or_default();
            objects.sort_by(|a, b| a.0.cmp(&b.0));
            for (id, obj) in objects {
                self.write_object_inclusive(&id, &obj);
            }
            self.flush(&format!("/* End {} section */{}", isa, Self::eol()));
        }
    }

    fn write_array(&mut self, key: &str, value: &[Value]) {
        self.println(&format!("{} = (", ensure_quotes(key)));
        self.indent += 1;

        for item in value {
            if let Some(buffer) = value_to_buffer(item) {
                self.println(&format!("{},", format_data(&buffer)));
            } else if item.is_null() {
                continue;
            } else if let Some(object) = item.as_object() {
                self.println("{");
                self.indent += 1;
                self.write_object(object, false);
                self.indent -= 1;
                self.println("},");
            } else {
                self.println(&format!("{},", self.format_id(&scalar_to_string(item))));
            }
        }

        self.indent -= 1;
        self.println(");");
    }

    fn write_object_inclusive(&mut self, key: &str, value: &Value) {
        if is_pbx_build_file(value) || is_pbx_file_reference(value) {
            self.write_object_without_indent(key, value);
            return;
        }

        let Some(object) = value.as_object() else {
            return;
        };

        self.println(&format!("{} = {{", self.format_id(key)));
        self.indent += 1;
        self.write_object(object, false);
        self.indent -= 1;
        self.println("};");
    }

    fn write_object_without_indent(&mut self, key: &str, value: &Value) {
        let mut line = String::new();
        self.build_inline(&mut line, key, value);
        self.println(line.trim());
    }

    fn build_inline(&self, line: &mut String, key: &str, value: &Value) {
        use std::fmt::Write;
        let _ = write!(line, "{} = {{", self.format_id(key));

        let Some(object) = value.as_object() else {
            line.push_str("}; ");
            return;
        };

        let mut keys: Vec<&String> = object.keys().collect();
        keys.sort_by(|a, b| sort_isa_first(a, b));

        for key in keys {
            let obj = &object[key];
            if self.options.skip_nullish_values && obj.is_null() {
                continue;
            }

            if let Some(buffer) = value_to_buffer(obj) {
                let _ = write!(line, "{} = {}; ", ensure_quotes(key), format_data(&buffer));
            } else if let Some(array) = obj.as_array() {
                let _ = write!(line, "{} = (", ensure_quotes(key));
                for item in array {
                    let _ = write!(line, "{}, ", ensure_quotes(&scalar_to_string(item)));
                }
                line.push_str("); ");
            } else if obj.is_object() {
                self.build_inline(line, key, obj);
            } else {
                let _ = write!(
                    line,
                    "{} = {}; ",
                    ensure_quotes(key),
                    self.format_id(&scalar_to_string(obj))
                );
            }
        }

        line.push_str("}; ");
    }
}

fn sort_isa_first(a: &str, b: &str) -> std::cmp::Ordering {
    match (a == "isa", b == "isa") {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.cmp(b),
    }
}

pub fn build(project: &Value) -> String {
    JsonWriter::new(project, WriterOptions::default()).into_string()
}

fn format_data(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(2 + data.len() * 2);
    out.push('<');
    for b in data {
        let _ = write!(out, "{:02X}", b);
    }
    out.push('>');
    out
}

fn scalar_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(v) => v.to_string(),
        Value::Null => "null".to_string(),
        _ => value.to_string(),
    }
}

fn ensure_quotes(value: &str) -> String {
    let escaped = add_quotes(value);
    if is_unquoted_identifier(&escaped) {
        escaped
    } else {
        format!("\"{}\"", escaped)
    }
}

fn is_unquoted_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '$' | '/' | ':' | '.'))
}

fn add_quotes(string: &str) -> String {
    let mut out = String::with_capacity(string.len());
    for ch in string.chars() {
        match ch {
            '\x07' => out.push_str("\\a"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x0b' => out.push_str("\\v"),
            '\n' => out.push_str("\\n"),
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\U{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::build;

    #[test]
    fn writes_basic_pbxproj() {
        let v = json!({
            "archiveVersion": 1,
            "classes": {},
            "objectVersion": 56,
            "rootObject": "ABC",
            "objects": {
                "ABC": {"isa": "PBXProject", "targets": []}
            }
        });

        let out = build(&v);
        assert!(out.starts_with("// !$*UTF8*$!"));
        assert!(out.contains("archiveVersion = 1;"));
    }
}
