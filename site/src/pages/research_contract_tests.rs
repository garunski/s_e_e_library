use dioxus::prelude::*;
use s_e_e_shapes::{tool_contract::research_candidate_contracts, wire::*};
use serde_json::{json, Value};

use super::authoring::research::{EXAMPLES, HELP, OPERATIONS};
use super::{LibraryAuthoringResearch, LibrarySchemaResearch};

const SCHEMA: &str = include_str!("../../../public/schema/knowledge-research.schema.json");
const DOC5: &str = include_str!("../../content/authoring/research_doc5_contract.md");
const LLMS: &str = include_str!("../../../public/llms.txt");

fn render(component: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(component);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

fn root_schema(mut schema: Value) -> Value {
    schema.as_object_mut().unwrap().remove("$schema");
    schema.as_object_mut().unwrap().remove("$defs");
    schema
}

fn mapping_valid(rows: &Value) -> bool {
    let rows = rows.as_array().unwrap();
    let expected: Vec<_> = DOC5
        .lines()
        .filter(|line| line.starts_with("| ") && line.contains(" | /"))
        .map(|line| line.split('|').map(str::trim).collect::<Vec<_>>())
        .collect();
    rows.len() == 28
        && expected.len() == 28
        && expected.iter().all(|cells| {
            let name = format!("knowledge_research_{}", cells[1]);
            rows.iter().filter(|row| row["name"] == name).count() == 1
                && rows.iter().any(|row| {
                    row["name"] == name
                        && row["method"] == cells[2]
                        && row["path"]
                            == format!("/api/projects/{{project}}/knowledge/research{}", cells[3])
                        && row["scope"] == cells[4]
                })
        })
}

#[test]
fn research_scoped_857_examples_validate_four_kinds_and_reject_malformed_scopes() {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let sequences: Value = serde_json::from_str(EXAMPLES).unwrap();
    let mut kinds = std::collections::BTreeSet::new();
    for sequence in sequences.as_array().unwrap() {
        kinds.insert(sequence["kind"].as_str().unwrap());
        let calls = sequence["calls"].as_array().unwrap();
        for call in calls {
            let errors: Vec<_> = validator.iter_errors(call).map(|e| e.to_string()).collect();
            assert!(errors.is_empty(), "{}: {errors:?}", call["tool"]);
            let name = call["tool"].as_str().unwrap();
            let mut args = call["arguments"].clone();
            args.as_object_mut().unwrap().remove("filters");
            (s_e_e_shapes::tool_contract::research_candidate_contract(name)
                .unwrap()
                .decode_request)(args)
            .unwrap();
            let mut missing_project = call.clone();
            missing_project["arguments"]
                .as_object_mut()
                .unwrap()
                .remove("project");
            assert!(!validator.is_valid(&missing_project));
        }
        for (suffix, token, read) in [
            ("page_create", "document_token", "get"),
            ("section_create", "page_token", "page_get"),
            ("section_put", "section_token", "section_get"),
            ("source_put", "source_token", "source_get"),
        ] {
            let index = calls
                .iter()
                .position(|c| c["tool"] == format!("knowledge_research_{suffix}"))
                .unwrap();
            assert!(calls[..index]
                .iter()
                .any(|c| c["tool"] == format!("knowledge_research_{read}")));
            let mut wrong = calls[index].clone();
            let value = wrong["arguments"]
                .as_object_mut()
                .unwrap()
                .remove(token)
                .unwrap();
            wrong["arguments"]["conflict_token"] = value;
            assert!(!validator.is_valid(&wrong));
        }
        let create = calls
            .iter()
            .find(|c| c["tool"] == "knowledge_research_create")
            .unwrap();
        assert_eq!(create["arguments"]["document"]["kind"], sequence["kind"]);
        let mut nested = create.clone();
        nested["arguments"]["document"]["pages"] = json!([]);
        assert!(!validator.is_valid(&nested));
        let list = calls
            .iter()
            .find(|c| c["tool"] == "knowledge_research_list")
            .unwrap();
        let filters: FilterRequest = serde_json::from_value(list["arguments"].clone()).unwrap();
        assert_eq!(filters.filters.unwrap().len(), 1);
        assert_eq!(list["arguments"]["state"], "draft");
    }
    assert_eq!(
        kinds,
        ["research", "report", "analysis", "guide"]
            .into_iter()
            .collect()
    );
}

#[test]
fn research_scoped_857_operations_schemas_and_http_mapping_match_producers_and_doc5() {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    let rows: Value = serde_json::from_str(OPERATIONS).unwrap();
    assert!(mapping_valid(&rows));
    let contracts = research_candidate_contracts();
    assert_eq!(contracts.len(), 28);
    assert_eq!(schema["x-operations"].as_object().unwrap().len(), 28);
    for contract in contracts {
        let op = contract.operation;
        let row = rows
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["name"] == op.id)
            .unwrap();
        assert_eq!(row["method"], op.http.method);
        assert_eq!(row["path"], op.http.path);
        let public = &schema["x-operations"][op.id];
        assert_eq!(public["method"], row["method"]);
        assert_eq!(public["path"], row["path"]);
        let mut input = (contract.request_schema)();
        if op.id == "knowledge_research_list" {
            let filters = serde_json::to_value(schemars::schema_for!(FilterRequest)).unwrap();
            input["properties"]["filters"] = filters["properties"]["filters"].clone();
        }
        for (field, expected) in [
            ("request_schema", input),
            ("response_schema", (contract.response_schema)()),
        ] {
            let reference = public[field].as_str().unwrap().trim_start_matches('#');
            assert_eq!(
                *schema.pointer(reference).unwrap(),
                root_schema(expected.clone()),
                "{} {field}",
                op.id
            );
            if let Some(defs) = expected["$defs"].as_object() {
                for (key, value) in defs {
                    assert_eq!(&schema["$defs"][key], value, "{} {key}", op.id);
                }
            }
        }
    }
    let mut missing = rows.clone();
    missing.as_array_mut().unwrap().pop();
    assert!(!mapping_valid(&missing));
    let mut wrong = rows.clone();
    wrong[0]["method"] = json!("POST");
    assert!(!mapping_valid(&wrong));
    wrong = rows.clone();
    wrong[0]["path"] = json!("/wrong");
    assert!(!mapping_valid(&wrong));
}

fn error_status(code: &str) -> u16 {
    match code {
        "not_found" | "view_not_found" => 404,
        "stale_token" | "already_exists" | "referenced_source" | "nonempty_page"
        | "tree_changed" => 409,
        "invalid_content" | "invalid_publication" => 422,
        _ => 400,
    }
}

fn errors_valid(rows: &Value, schema: &Value) -> bool {
    rows.as_array().unwrap().iter().all(|row| {
        let name = row["name"].as_str().unwrap();
        let required = if name.ends_with("page_delete") {
            "nonempty_page"
        } else if name.ends_with("source_delete") {
            "referenced_source"
        } else if name.ends_with("publish") {
            "invalid_publication"
        } else if name.ends_with("reorder") {
            "invalid_order"
        } else {
            "invalid_key"
        };
        row["errors"].as_array().unwrap().contains(&json!(required))
            && (![
                "knowledge_research_page_delete",
                "knowledge_research_section_delete",
                "knowledge_research_source_delete",
            ]
            .contains(&name)
                || row["errors"]
                    .as_array()
                    .unwrap()
                    .contains(&json!("invalid_content")))
            && schema["x-operations"][name]["errors"] == row["errors"]
    })
}

#[test]
fn research_scoped_857_error_envelopes_and_applicable_contracts_match_typed_errors() {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    let rows: Value = serde_json::from_str(OPERATIONS).unwrap();
    assert!(errors_valid(&rows, &schema));
    let mut error_schema = schema.clone();
    error_schema.as_object_mut().unwrap().remove("oneOf");
    error_schema["$ref"] = json!("#/$defs/ResearchScopedError");
    let validator = jsonschema::validator_for(&error_schema).unwrap();
    for row in rows.as_array().unwrap() {
        for code in row["errors"].as_array().unwrap() {
            let error = json!({"code":code,"affected_ids":{"research_id":"research-1"},"field_path":"token","remedy":"Reread the affected scope","current_token":"CURRENT"});
            assert!(validator.is_valid(&error));
            let typed: ResearchScopedError = serde_json::from_value(error).unwrap();
            assert_eq!(typed.http_status(), error_status(code.as_str().unwrap()));
        }
    }
    let mut bad = rows.clone();
    bad[0]["errors"] = json!(["invented_error"]);
    assert!(!errors_valid(&bad, &schema));
    bad = rows.clone();
    bad[10]["errors"] = json!([]);
    assert!(!errors_valid(&bad, &schema));
    bad = rows.clone();
    bad[10]["errors"]
        .as_array_mut()
        .unwrap()
        .retain(|code| code != "invalid_content");
    let mut missing_confirmation = schema.clone();
    missing_confirmation["x-operations"]["knowledge_research_page_delete"]["errors"] =
        bad[10]["errors"].clone();
    assert!(!errors_valid(&bad, &missing_confirmation));
}

#[test]
fn research_scoped_857_rendered_pages_help_and_llms_match_scoped_workflow() {
    let author = render(LibraryAuthoringResearch);
    let wire = render(LibrarySchemaResearch);
    let help = s_e_e_gui_kit::pages::help::help_article("research-authoring").unwrap();
    for (path, text) in [
        ("/authoring/research/", HELP),
        ("/schema/knowledge-research/", super::schema::research::HELP),
    ] {
        let section = LLMS
            .split(&format!("Source: {path}\n"))
            .nth(1)
            .unwrap()
            .split("\nSource: ")
            .next()
            .unwrap();
        assert!(section.contains(text.trim()));
        if path == "/authoring/research/" {
            assert!(section.contains(EXAMPLES.trim()));
            for row in serde_json::from_str::<Value>(OPERATIONS)
                .unwrap()
                .as_array()
                .unwrap()
            {
                for field in ["name", "method", "path", "scope"] {
                    assert!(section.contains(row[field].as_str().unwrap()));
                }
                for code in row["errors"].as_array().unwrap() {
                    assert!(section.contains(code.as_str().unwrap()));
                }
            }
        }
    }
    for fact in [
        "manifest.json",
        "sections/{section_key}.json",
        "sources/{source_id}.json",
        "document_token",
        "page_token",
        "section_token",
        "source_token",
        "tree_token",
        "state=all|draft|published",
        "3-8 sections",
        "1500 words",
        "advisory",
        "do not truncate",
        "explicit target project",
        "EXPERIMENTAL_FEATURE_DISABLED",
        "AND before pagination",
        "invalid_cursor",
        "stale_token",
        "Publish only when requested",
    ] {
        assert!(author.contains(fact), "author: {fact}");
        assert!(help.html.contains(fact), "help: {fact}");
    }
    assert!(wire.contains("knowledge-research.schema.json"));
    assert!(wire.contains("section_token"));
    assert!(wire.contains("filters compose with kind, tag, query and state"));
    for row in serde_json::from_str::<Value>(OPERATIONS)
        .unwrap()
        .as_array()
        .unwrap()
    {
        for field in ["name", "method", "path", "scope"] {
            assert!(
                author.contains(row[field].as_str().unwrap()),
                "rendered {field}"
            );
            assert!(LLMS.contains(row[field].as_str().unwrap()));
        }
        for code in row["errors"].as_array().unwrap() {
            assert!(author.contains(code.as_str().unwrap()));
        }
    }
    assert!(!help.body.contains("latest aggregate"));
}
