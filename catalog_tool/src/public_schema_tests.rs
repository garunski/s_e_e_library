use serde_json::{json, Value};

fn public(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../public/schema")
        .join(format!("{name}.schema.json"));
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

#[test]
fn settings_metadata_matches_application_with_public_identity() {
    for (name, mut current) in [
        (
            "app-settings",
            s_e_e_core::validation::schema::load_app_settings_schema(),
        ),
        (
            "hub-config",
            s_e_e_core::validation::schema::load_hub_config_schema(),
        ),
    ] {
        let published = public(name);
        assert_eq!(
            published["$id"],
            format!("https://garunski.github.io/s_e_e_library/schema/{name}.schema.json")
        );
        current["$id"] = published["$id"].clone();
        assert_eq!(published, current, "{name}");
    }
    let schema = public("app-settings");
    let validator = jsonschema::validator_for(&schema).unwrap();
    assert!(validator.is_valid(&json!({"editor_auto_save":true,"mcp_auth_enabled":true,"mcp_allowed_browser_origins":"http://localhost","default_workflow":"legacy","mcp_enabled":true})));
    assert!(!validator.is_valid(&json!({"editor_auto_save":"true"})));
}

#[test]
fn research_operations_match_registered_markdown_bindings() {
    assert_eq!(
        public("knowledge-research"),
        s_e_e_shapes::tool_contract::research_markdown_public_schema()
    );
}

#[test]
fn research_markdown_example_and_obsolete_fields() {
    let schema = public("knowledge-research");
    let validator = jsonschema::validator_for(&schema).unwrap();
    let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../public/examples/knowledge-research.json");
    let example: Value = serde_json::from_slice(&std::fs::read(example_path).unwrap()).unwrap();
    assert!(validator.is_valid(&example));
    let mut old_role = example.clone();
    old_role["pages"][0]["sections"][0]["metadata"]["role"] = json!("method");
    assert!(!validator.is_valid(&old_role));
    let mut global_sources = example.clone();
    global_sources["root"]["metadata"]["sources"] =
        example["pages"][0]["page"]["metadata"]["sources"].clone();
    assert!(!validator.is_valid(&global_sources));
    let mut global_sources = example;
    global_sources["sources"] = json!([]);
    assert!(!validator.is_valid(&global_sources));
    let operations = schema["x-operations"].as_object().unwrap();
    assert!(operations.contains_key("knowledge_research_sources_get"));
    assert!(operations.contains_key("knowledge_research_sources_put"));
    assert!(!operations.contains_key("knowledge_research_profile_get"));
    assert!(!operations.contains_key("knowledge_research_source_put"));
}
