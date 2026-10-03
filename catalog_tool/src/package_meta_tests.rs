use std::collections::HashSet;

use serde_json::json;

use crate::package_meta::{
    catalog_without_updated, marketplace_ref_errors, normalize_releases, normalize_tool_ids,
    stable_stringify,
};

#[test]
fn normalize_tool_ids_sorts_unique_ids_and_rejects_blanks() {
    assert_eq!(
        normalize_tool_ids(&json!(["claude", "cursor", "claude"])),
        Some(vec!["claude".to_string(), "cursor".to_string()])
    );
    assert_eq!(normalize_tool_ids(&json!(["cursor", ""])), None);
    assert_eq!(normalize_tool_ids(&json!("cursor")), None);
}

#[test]
fn normalize_tool_ids_does_not_infer_ids_from_package_name() {
    let name = "Cursor Claude Skill";
    let tool_ids = normalize_tool_ids(&json!([])).expect("empty list");
    assert!(tool_ids.is_empty());
    assert!(!tool_ids.iter().any(|id| name.to_lowercase().contains(id)));
}

#[test]
fn normalize_releases_keeps_authored_history_and_rejects_incomplete_rows() {
    assert_eq!(
        normalize_releases(&json!([{
            "version": "1.0.0",
            "date": "2026-09-01",
            "note": "first"
        }])),
        Some(vec![json!({
            "version": "1.0.0",
            "date": "2026-09-01",
            "note": "first"
        })])
    );
    assert_eq!(
        normalize_releases(&json!([{ "version": "1.0.0", "date": "2026-09-01" }])),
        None
    );
    assert_eq!(normalize_releases(&json!("1.0.0")), None);
}

#[test]
fn marketplace_ref_errors_reject_unknown_ids_and_undeclared_featured_stack() {
    let package_ids = HashSet::from([("wf-1".to_string())]);
    let errors = marketplace_ref_errors(
        &[
            json!({"id": "cursor", "name": "Cursor"}),
            json!({"id": "cursor", "name": "Cursor again"}),
        ],
        &[json!({
            "id": "stack-a",
            "name": "A",
            "sharedPackageIds": ["missing"],
            "variants": [{ "toolId": "ghost", "packageIds": ["wf-1"] }]
        })],
        Some("stack-missing"),
        &package_ids,
    );
    assert!(errors.iter().any(|e| e.contains("duplicate tool id: cursor")));
    assert!(errors.iter().any(|e| e.contains("unknown stack package id: missing")));
    assert!(errors
        .iter()
        .any(|e| e.contains("undeclared tool in stack variant: ghost")));
    assert!(errors
        .iter()
        .any(|e| e.contains("unknown featured stack id: stack-missing")));
}

#[test]
fn marketplace_ref_errors_accepts_valid_featured_stack() {
    let package_ids = HashSet::from([
        "wf-1".to_string(),
        "cmd-cursor-agent".to_string(),
    ]);
    let errors = marketplace_ref_errors(
        &[json!({"id": "cursor", "name": "Cursor"})],
        &[json!({
            "id": "stack-a",
            "name": "A",
            "sharedPackageIds": ["wf-1"],
            "variants": [{ "toolId": "cursor", "packageIds": ["cmd-cursor-agent"] }]
        })],
        Some("stack-a"),
        &package_ids,
    );
    assert!(errors.is_empty());
}

#[test]
fn catalog_without_updated_ignores_timestamp_for_equality() {
    let stale = json!({
        "schema": "see.library/v2",
        "name": "Official",
        "updated": "2026-09-01T00:00:00.000Z",
        "packages": [{ "id": "wf-1", "toolIds": [], "releases": [] }]
    });
    let next = json!({
        "schema": "see.library/v2",
        "name": "Official",
        "updated": "2026-09-02T00:00:00.000Z",
        "packages": [{ "id": "wf-1", "toolIds": ["cursor"], "releases": [{
            "version": "1.0.0",
            "date": "2026-09-01",
            "note": "first"
        }] }]
    });
    assert_ne!(
        stable_stringify(&catalog_without_updated(&stale)),
        stable_stringify(&catalog_without_updated(&next))
    );
    assert_eq!(
        stable_stringify(&catalog_without_updated(&next)),
        stable_stringify(&catalog_without_updated(&next))
    );
}
