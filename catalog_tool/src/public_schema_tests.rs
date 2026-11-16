use s_e_e_shapes::tool_contract::research_candidate_contracts;
use s_e_e_shapes::wire::{
    collection_filter_descriptor, decode_http_filters, CompiledCollectionFilter, FilterScope,
    FilteredRequest, ResearchScopedListRequest,
};
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
fn research_operations_match_registered_typed_bindings() {
    let published = public("knowledge-research");
    let operations = published["x-operations"].as_object().unwrap();
    let contracts = research_candidate_contracts();
    assert_eq!(operations.len(), contracts.len());
    assert_eq!(
        published["oneOf"].as_array().unwrap().len(),
        contracts.len()
    );
    for contract in contracts {
        let name = contract.operation.id;
        let entry = &operations[name];
        assert_eq!(entry["method"], contract.operation.http.method, "{name}");
        assert_eq!(entry["path"], contract.operation.http.path, "{name}");
        for (field, suffix, generated) in [
            ("request_schema", "Request", (contract.request_schema)()),
            ("response_schema", "Response", (contract.response_schema)()),
        ] {
            assert_eq!(entry[field], format!("#/$defs/{name}{suffix}"));
            let actual = published
                .pointer(entry[field].as_str().unwrap().trim_start_matches('#'))
                .unwrap();
            let mut expected = generated;
            if let Some(definitions) = expected["$defs"].as_object() {
                for (key, value) in definitions {
                    assert_eq!(published["$defs"][key], *value, "{name} definition {key}");
                }
            }
            expected.as_object_mut().unwrap().remove("$schema");
            expected.as_object_mut().unwrap().remove("$defs");
            if name == "knowledge_research_list" && field == "request_schema" {
                expected["properties"]["filters"] = json!({
                    "items":{"$ref":"#/$defs/FilterPredicate"},
                    "type":["array","null"]
                });
            }
            assert_eq!(*actual, expected, "{name} {field}");
        }
        assert!(published["oneOf"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["properties"]["tool"]["const"] == name
                && entry["properties"]["arguments"]["$ref"] == format!("#/$defs/{name}Request")));
    }
}

#[test]
fn research_predicates_preserve_mcp_and_http_types() {
    let filters = public("collection-filter");
    let research = public("knowledge-research");
    let registry = jsonschema::Registry::new()
        .add(filters["$id"].as_str().unwrap(), &filters)
        .unwrap()
        .add(research["$id"].as_str().unwrap(), &research)
        .unwrap()
        .prepare()
        .unwrap();
    let root = research["$id"].as_str().unwrap();
    let build = |name: &str| {
        jsonschema::options()
            .with_registry(&registry)
            .build(&json!({"$ref":format!("{root}#/$defs/{name}")}))
            .unwrap()
    };
    let mcp = build("McpListArgs");
    let http = build("ListQuery");
    let predicates = json!([{"field":"page_count","op":"gte","value":1}]);
    let example = json!({"project":"s-e-e","limit":20,"filters":predicates});
    assert!(mcp.is_valid(&example));
    let request: FilteredRequest<ResearchScopedListRequest> =
        serde_json::from_value(example).unwrap();
    let mut url =
        reqwest::Url::parse("http://localhost/api/projects/s-e-e/knowledge/research").unwrap();
    url.query_pairs_mut()
        .append_pair("filters", &predicates.to_string());
    assert!(url.as_str().contains("filters=%5B"));
    let pairs: Vec<_> = url.query_pairs().collect();
    let decoded = decode_http_filters(pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))).unwrap();
    assert_eq!(decoded, request.filters);
    let descriptor =
        collection_filter_descriptor("knowledge_research", FilterScope::Project, &[]).unwrap();
    assert!(
        !CompiledCollectionFilter::compile(&descriptor, decoded.as_deref())
            .unwrap()
            .is_empty()
    );
    assert!(http.is_valid(&json!({"limit":20,"filters":predicates.to_string()})));
    assert!(!http.is_valid(&json!({"filters":predicates})));
    assert!(!mcp.is_valid(
        &json!({"project":"s-e-e","filters":[{"field":"page_count","op":"gte","value":null}]})
    ));
    assert!(decode_http_filters([("filters", "null")]).is_err());
}
