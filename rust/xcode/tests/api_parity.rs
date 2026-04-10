mod common;

use anyhow::Result;
use serde_json::{json, Value};

use common::{canonicalize_json, run_ts_oracle};

fn minimal_project_json() -> Value {
    json!({
        "archiveVersion": 1,
        "objectVersion": 56,
        "rootObject": "ROOT",
        "classes": {},
        "objects": {
            "ROOT": {
                "isa": "PBXProject"
            }
        }
    })
}

#[test]
fn api_project_summary_matches_ts() -> Result<()> {
    let project = minimal_project_json();

    let ts = run_ts_oracle(&json!({
        "module": "api",
        "operation": "projectSummary",
        "project": project,
    }))?;

    let rs = xcode::api::XcodeProject::from_json("/tmp/project.pbxproj", minimal_project_json())?;

    let rs_summary = json!({
        "archiveVersion": rs.archive_version,
        "objectVersion": rs.object_version,
        "rootObjectUuid": rs.root_object,
        "rootObjectIsa": rs.root_object()?.isa,
        "objectCount": rs.objects.len(),
    });

    assert_eq!(canonicalize_json(&ts), canonicalize_json(&rs_summary));

    Ok(())
}

#[test]
fn api_create_model_and_delete_matches_ts() -> Result<()> {
    let model = json!({
        "isa": "PBXGroup",
        "name": "MyGroup",
        "children": [],
        "sourceTree": "<group>"
    });

    let ts = run_ts_oracle(&json!({
        "module": "api",
        "operation": "createModelAndDelete",
        "project": minimal_project_json(),
        "model": model,
    }))?;

    let mut rs =
        xcode::api::XcodeProject::from_json("/tmp/project.pbxproj", minimal_project_json())?;
    let created = rs.create_model(model)?;

    let mut referrers_before: Vec<String> = rs
        .get_referrers(&created.uuid)
        .iter()
        .map(|obj| obj.uuid.clone())
        .collect();
    referrers_before.sort();

    let deleted = rs.delete(&created.uuid);

    let rs_result = json!({
        "createdUuid": created.uuid,
        "deleted": deleted,
        "existsAfterDelete": rs.has(&created.uuid),
        "referrersBefore": referrers_before,
        "objectCountAfter": rs.objects.len(),
    });

    assert_eq!(canonicalize_json(&ts), canonicalize_json(&rs_result));

    Ok(())
}

#[test]
fn api_get_referrers_matches_ts() -> Result<()> {
    let project = json!({
        "archiveVersion": 1,
        "objectVersion": 56,
        "rootObject": "ROOT",
        "classes": {},
        "objects": {
            "ROOT": {
                "isa": "PBXProject",
                "mainGroup": "GROUP"
            },
            "GROUP": {
                "isa": "PBXGroup",
                "children": [],
                "sourceTree": "<group>",
                "name": "Main"
            }
        }
    });

    let ts = run_ts_oracle(&json!({
        "module": "api",
        "operation": "getReferrers",
        "project": project,
        "uuid": "GROUP",
    }))?;

    let rs = xcode::api::XcodeProject::from_json("/tmp/project.pbxproj", project)?;
    let mut refs: Vec<Value> = rs
        .get_referrers("GROUP")
        .into_iter()
        .map(|obj| json!({"uuid": obj.uuid, "isa": obj.isa}))
        .collect();
    refs.sort_by(|a, b| {
        a.get("uuid")
            .and_then(Value::as_str)
            .unwrap_or("")
            .cmp(b.get("uuid").and_then(Value::as_str).unwrap_or(""))
    });

    assert_eq!(
        canonicalize_json(&ts),
        canonicalize_json(&Value::Array(refs))
    );

    Ok(())
}
