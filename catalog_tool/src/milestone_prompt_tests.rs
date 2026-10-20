use std::path::Path;

use crate::milestone_prompt::milestone_authoring_contract_errors;

fn published_milestone_prompt(root: &Path) -> (String, String) {
    let catalog: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("catalog.json")).expect("catalog"))
            .expect("catalog json");
    let entry = catalog
        .get("packages")
        .and_then(|v| v.as_array())
        .and_then(|packages| {
            packages.iter().find(|pkg| {
                pkg.get("slug").and_then(|v| v.as_str()) == Some("system-milestone-authoring")
            })
        })
        .expect("system-milestone-authoring");
    let from = entry
        .get("files")
        .and_then(|v| v.as_array())
        .and_then(|files| {
            files.iter().find(|row| {
                row.get("to").and_then(|v| v.as_str())
                    == Some(".s_e_e/prompts/system-milestone-authoring.json")
            })
        })
        .and_then(|row| row.get("from").and_then(|v| v.as_str()))
        .expect("prompt file row")
        .to_string();
    let payload = std::fs::read_to_string(root.join(&from)).expect("payload");
    let value: serde_json::Value = serde_json::from_str(&payload).expect("json");
    let content = value
        .get("content")
        .and_then(|v| v.as_str())
        .expect("content")
        .to_string();
    (from, content)
}

#[test]
fn stale_minimal_milestone_copy_fails_verification() {
    let stale = "MCP milestone tools: milestone_list, milestone_create.\nMinimal - milestones carry no acceptance criteria and no implementation plan.";
    let errors = milestone_authoring_contract_errors(stale);
    assert!(errors.iter().any(|e| e.contains("forbidden obsolete copy")));
    assert!(errors.iter().any(|e| e.contains("milestone_get")));
    assert!(errors.iter().any(|e| e.contains("milestone_add_criterion")));
}

#[test]
fn installable_milestone_authoring_prompt_matches_contract() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let (from, content) = published_milestone_prompt(&root);
    assert!(from.contains("system-milestone-authoring"));
    assert!(from.ends_with("/prompt.json"));
    assert!(milestone_authoring_contract_errors(&content).is_empty());
}

#[test]
fn root_catalog_points_at_corrected_prompt_payload() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let catalog: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("catalog.json")).expect("catalog"))
            .expect("catalog json");
    let entry = catalog
        .get("packages")
        .and_then(|v| v.as_array())
        .and_then(|packages| {
            packages
                .iter()
                .find(|pkg| pkg.get("slug").and_then(|v| v.as_str()) == Some("system-milestone-authoring"))
        })
        .expect("system-milestone-authoring");
    let (from, content) = published_milestone_prompt(&root);
    let version = entry
        .get("version")
        .and_then(|v| v.as_str())
        .expect("version");
    assert!(from.ends_with(&format!("/{version}/prompt.json")), "{from} is not version {version}");
    assert!(milestone_authoring_contract_errors(&content).is_empty());
}
