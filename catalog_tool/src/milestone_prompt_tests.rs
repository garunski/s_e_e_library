use std::path::Path;

use crate::milestone_prompt::milestone_authoring_contract_errors;

const PROMPT_FROM: &str = "packages/system-milestone-authoring/1.0.0/prompt.json";

#[test]
fn stale_minimal_milestone_copy_fails_verification() {
    let stale = "MCP milestone tools: milestone_list, milestone_create.\nMinimal - milestones carry no acceptance criteria and no implementation plan.";
    let errors = milestone_authoring_contract_errors(stale);
    assert!(errors.iter().any(|e| e.contains("forbidden obsolete copy")));
    assert!(errors.iter().any(|e| e.contains("item_fetch")));
    assert!(errors.iter().any(|e| e.contains("milestone_add_criterion")));
}

#[test]
fn installable_milestone_authoring_prompt_matches_contract() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let payload = std::fs::read_to_string(root.join(PROMPT_FROM)).expect("prompt.json");
    let value: serde_json::Value = serde_json::from_str(&payload).expect("json");
    let content = value.get("content").and_then(|v| v.as_str()).expect("content");
    assert!(milestone_authoring_contract_errors(content).is_empty());
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
    let file = entry
        .get("files")
        .and_then(|v| v.as_array())
        .and_then(|files| {
            files
                .iter()
                .find(|row| row.get("from").and_then(|v| v.as_str()) == Some(PROMPT_FROM))
        })
        .expect("prompt file row");
    let from = file.get("from").and_then(|v| v.as_str()).expect("from");
    let payload = std::fs::read_to_string(root.join(from)).expect("payload");
    let value: serde_json::Value = serde_json::from_str(&payload).expect("json");
    let content = value.get("content").and_then(|v| v.as_str()).expect("content");
    assert!(milestone_authoring_contract_errors(content).is_empty());
}
