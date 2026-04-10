use std::fs;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use md5::{Digest, Md5};
use serde_json::{Map, Number, Value};

use crate::json;

use super::AbstractObject;

#[derive(Debug, Clone)]
pub struct XcodeProject {
    pub file_path: PathBuf,
    pub archive_version: i64,
    pub object_version: i64,
    pub root_object: String,
    pub classes: Value,
    pub objects: IndexMap<String, AbstractObject>,
}

impl XcodeProject {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file_path = path.as_ref().to_path_buf();
        let contents = fs::read_to_string(&file_path)?;
        let parsed = json::parse(&contents)?;
        Self::from_json(file_path, parsed)
    }

    pub fn from_json(file_path: impl AsRef<Path>, value: Value) -> anyhow::Result<Self> {
        let file_path = file_path.as_ref().to_path_buf();
        let root = value
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Project root must be an object"))?;

        let archive_version = root
            .get("archiveVersion")
            .and_then(Value::as_i64)
            .unwrap_or(1);
        let object_version = root
            .get("objectVersion")
            .and_then(Value::as_i64)
            .unwrap_or(56);
        let root_object = root
            .get("rootObject")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Missing rootObject"))?
            .to_string();

        let classes = root
            .get("classes")
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new()));

        let mut objects = IndexMap::<String, AbstractObject>::new();
        let json_objects = root
            .get("objects")
            .and_then(Value::as_object)
            .ok_or_else(|| anyhow::anyhow!("Missing objects dictionary"))?;

        for (uuid, obj) in json_objects {
            let isa = obj
                .get("isa")
                .and_then(Value::as_str)
                .unwrap_or("[unknown]")
                .to_string();
            objects.insert(
                uuid.clone(),
                AbstractObject::new(uuid.clone(), isa, obj.clone()),
            );
        }

        Ok(Self {
            file_path,
            archive_version,
            object_version,
            root_object,
            classes,
            objects,
        })
    }

    pub fn to_json(&self) -> Value {
        let mut root = Map::new();
        root.insert(
            "archiveVersion".to_string(),
            Value::Number(Number::from(self.archive_version)),
        );
        root.insert(
            "objectVersion".to_string(),
            Value::Number(Number::from(self.object_version)),
        );
        root.insert(
            "rootObject".to_string(),
            Value::String(self.root_object.clone()),
        );
        root.insert("classes".to_string(), self.classes.clone());

        let mut objects = Map::new();
        for (uuid, object) in &self.objects {
            objects.insert(uuid.clone(), object.to_json());
        }
        root.insert("objects".to_string(), Value::Object(objects));

        Value::Object(root)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let text = json::build(&self.to_json());
        fs::write(&self.file_path, text)?;
        Ok(())
    }

    pub fn has(&self, uuid: &str) -> bool {
        self.objects.contains_key(uuid)
    }

    pub fn get_object(&self, uuid: &str) -> anyhow::Result<&AbstractObject> {
        self.objects
            .get(uuid)
            .ok_or_else(|| anyhow::anyhow!("object with uuid '{}' not found.", uuid))
    }

    pub fn get_object_mut(&mut self, uuid: &str) -> anyhow::Result<&mut AbstractObject> {
        self.objects
            .get_mut(uuid)
            .ok_or_else(|| anyhow::anyhow!("object with uuid '{}' not found.", uuid))
    }

    pub fn root_object(&self) -> anyhow::Result<&AbstractObject> {
        self.get_object(&self.root_object)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &AbstractObject)> {
        self.objects.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &AbstractObject> {
        self.objects.values()
    }

    pub fn create_model(&mut self, mut props: Value) -> anyhow::Result<AbstractObject> {
        let isa = props
            .get("isa")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Model must include isa"))?
            .to_string();

        let canonical = canonicalize(&props).to_string();
        let mut counter = 0usize;
        let uuid = loop {
            let seed = if counter == 0 {
                canonical.clone()
            } else {
                format!("{}#{}", canonical, counter)
            };
            let id = uuid_for_seed(&seed);
            if !self.objects.contains_key(&id) {
                break id;
            }
            counter += 1;
        };

        normalize_references(&mut props);
        let object = AbstractObject::new(uuid.clone(), isa, props);
        self.objects.insert(uuid, object.clone());
        Ok(object)
    }

    pub fn get_referrers(&self, uuid: &str) -> Vec<&AbstractObject> {
        self.objects
            .values()
            .filter(|obj| obj.is_referencing(uuid))
            .collect()
    }

    pub fn delete(&mut self, uuid: &str) -> bool {
        let removed = self.objects.swap_remove(uuid).is_some();
        if removed {
            for object in self.objects.values_mut() {
                remove_reference(&mut object.props, uuid);
            }
        }
        removed
    }
}

fn uuid_for_seed(seed: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(seed.as_bytes());
    let digest = hasher.finalize();
    let hex = format!("{:X}", digest);
    format!("XX{}XX", &hex[..20])
}

fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.iter().map(canonicalize).collect()),
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();

            let mut out = Map::new();
            for key in keys {
                out.insert(key.clone(), canonicalize(&map[key]));
            }
            Value::Object(out)
        }
        _ => value.clone(),
    }
}

fn normalize_references(_value: &mut Value) {
    // Reserved extension point for future strict UUID/reference normalization parity.
}

fn remove_reference(value: &mut Value, uuid: &str) {
    match value {
        Value::Array(array) => {
            array.retain(|entry| entry.as_str() != Some(uuid));
            for entry in array.iter_mut() {
                remove_reference(entry, uuid);
            }
        }
        Value::Object(map) => {
            let keys_to_remove: Vec<String> = map
                .iter()
                .filter_map(|(key, entry)| {
                    if entry.as_str() == Some(uuid) {
                        Some(key.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for key in keys_to_remove {
                map.remove(&key);
            }
            for entry in map.values_mut() {
                remove_reference(entry, uuid);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::XcodeProject;

    #[test]
    fn create_model_generates_deterministic_uuid_shape() {
        let root = json!({
            "archiveVersion": 1,
            "objectVersion": 56,
            "rootObject": "ROOT",
            "classes": {},
            "objects": {
                "ROOT": {"isa": "PBXProject"}
            }
        });

        let mut project = XcodeProject::from_json("/tmp/test.pbxproj", root).unwrap();
        let model = project
            .create_model(json!({
                "isa": "PBXGroup",
                "children": [],
                "sourceTree": "<group>",
                "name": "MyGroup"
            }))
            .unwrap();

        assert_eq!(model.uuid.len(), 24);
        assert!(model.uuid.starts_with("XX"));
        assert!(model.uuid.ends_with("XX"));
    }
}
