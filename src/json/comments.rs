use std::collections::HashMap;

use serde_json::Value;

pub fn create_reference_list(project: &Value) -> HashMap<String, String> {
    let mut cache = HashMap::new();

    let Some(objects) = project.get("objects").and_then(Value::as_object) else {
        return cache;
    };

    let mut state = CommentState {
        objects,
        cache: &mut cache,
    };

    for (id, object) in objects {
        let _ = state.get_comment_for_object(id, object);
    }

    cache
}

struct CommentState<'a> {
    objects: &'a serde_json::Map<String, Value>,
    cache: &'a mut HashMap<String, String>,
}

impl<'a> CommentState<'a> {
    fn get_comment_for_object(&mut self, id: &str, object: &Value) -> Option<String> {
        if self.cache.contains_key(id) {
            return self.cache.get(id).cloned();
        }

        let isa = object.get("isa").and_then(Value::as_str)?;

        let resolved = if isa == "PBXBuildFile" {
            self.get_pbx_build_file_comment(id, object)
        } else if isa == "XCConfigurationList" {
            self.get_xcconfiguration_list_comment(id)
        } else if isa == "XCRemoteSwiftPackageReference" {
            let repo = object
                .get("repositoryURL")
                .and_then(Value::as_str)
                .map(repo_name_from_url)
                .unwrap_or_else(|| "XCRemoteSwiftPackageReference".to_string());
            if repo == "XCRemoteSwiftPackageReference" {
                repo
            } else {
                format!("XCRemoteSwiftPackageReference \"{}\"", repo)
            }
        } else if isa == "XCLocalSwiftPackageReference" {
            if let Some(path) = object.get("relativePath").and_then(Value::as_str) {
                format!("XCLocalSwiftPackageReference \"{}\"", path)
            } else {
                "XCLocalSwiftPackageReference".to_string()
            }
        } else if isa == "PBXProject" {
            "Project object".to_string()
        } else if isa.ends_with("BuildPhase") {
            self.get_build_phase_name(object).unwrap_or_default()
        } else if isa == "PBXGroup" && object.get("name").is_none() && object.get("path").is_none()
        {
            String::new()
        } else {
            first_string(object, &["name", "productName", "path", "isa"]).unwrap_or_default()
        };

        self.cache.insert(id.to_string(), resolved.clone());
        Some(resolved)
    }

    fn get_xcconfiguration_list_comment(&self, id: &str) -> String {
        for (inner_id, obj) in self.objects {
            if obj.get("buildConfigurationList").and_then(Value::as_str) == Some(id) {
                let mut name = first_string(obj, &["name", "path", "productName"]);
                if name.is_none()
                    && let Some(targets) = obj.get("targets").and_then(Value::as_array)
                        && let Some(first_target_id) = targets.first().and_then(Value::as_str) {
                            name = self
                                .objects
                                .get(first_target_id)
                                .and_then(|target| first_string(target, &["productName", "name"]));
                        }

                if name.is_none() {
                    let proxy = self.objects.values().find(|candidate| {
                        candidate.get("isa").and_then(Value::as_str)
                            == Some("PBXContainerItemProxy")
                            && candidate.get("containerPortal").and_then(Value::as_str)
                                == Some(inner_id)
                    });
                    if let Some(proxy) = proxy {
                        name = proxy
                            .get("remoteInfo")
                            .and_then(Value::as_str)
                            .map(str::to_string);
                    }
                }

                return format!(
                    "Build configuration list for {} \"{}\"",
                    obj.get("isa")
                        .and_then(Value::as_str)
                        .unwrap_or("[unknown]"),
                    name.unwrap_or_else(|| "[unknown]".to_string())
                );
            }
        }
        "Build configuration list for [unknown]".to_string()
    }

    fn get_pbx_build_file_comment(&mut self, id: &str, build_file: &Value) -> String {
        let build_phase_name = self
            .get_build_phase_name_containing_file(id)
            .unwrap_or_else(|| "[missing build phase]".to_string());

        let ref_id = build_file
            .get("fileRef")
            .and_then(Value::as_str)
            .or_else(|| build_file.get("productRef").and_then(Value::as_str));

        let Some(ref_id) = ref_id else {
            return format!("[unknown] in {}", build_phase_name);
        };

        let comment = self
            .objects
            .get(ref_id)
            .and_then(|obj| self.get_comment_for_object(ref_id, obj))
            .unwrap_or_else(|| "[unknown]".to_string());

        format!("{} in {}", comment, build_phase_name)
    }

    fn get_build_phase_name_containing_file(&self, build_file_id: &str) -> Option<String> {
        for obj in self.objects.values() {
            if let Some(files) = obj.get("files").and_then(Value::as_array)
                && files
                    .iter()
                    .any(|entry| entry.as_str() == Some(build_file_id))
                {
                    return self.get_build_phase_name(obj);
                }
        }
        None
    }

    fn get_build_phase_name(&self, build_phase: &Value) -> Option<String> {
        if let Some(name) = build_phase.get("name").and_then(Value::as_str) {
            return Some(name.to_string());
        }

        let isa = build_phase.get("isa").and_then(Value::as_str)?;
        default_build_phase_name(isa)
    }
}

fn default_build_phase_name(isa: &str) -> Option<String> {
    isa.strip_prefix("PBX")
        .and_then(|s| s.strip_suffix("BuildPhase"))
        .map(str::to_string)
}

fn first_string(value: &Value, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(v) = value.get(*key).and_then(Value::as_str) {
            return Some(v.to_string());
        }
    }
    None
}

fn repo_name_from_url(url: &str) -> String {
    if let Some(trimmed) = url.strip_prefix("https://github.com/") {
        return trimmed.split('/').next_back().unwrap_or(url).to_string();
    }
    url.to_string()
}

pub fn is_pbx_build_file(value: &Value) -> bool {
    value.get("isa").and_then(Value::as_str) == Some("PBXBuildFile")
}

pub fn is_pbx_file_reference(value: &Value) -> bool {
    value.get("isa").and_then(Value::as_str) == Some("PBXFileReference")
}
