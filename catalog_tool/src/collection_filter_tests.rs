use reqwest::Url;
use s_e_e_shapes::wire::{
    collection_filter_descriptor, decode_http_filters, CollectionFilterDescriptor,
    CollectionFilterError, CompiledCollectionFilter, FilterFieldValue as V, FilterRequest,
    FilterScope, FilteredRequest, ListPage, ListQuery, StoryListRequest, MAX_FILTER_JSON_BYTES,
    MAX_FILTER_MEMBERS, MAX_FILTER_PREDICATES, MAX_FILTER_STRING_CHARS,
};
use serde_json::{json, Value};

const STORIES: &str = include_str!("../../site/content/authoring/collection_filter_stories.json");
const MILESTONES: &str =
    include_str!("../../site/content/authoring/collection_filter_milestones.json");
const DOCS: &str = include_str!("../../site/content/authoring/collection_filter_docs.json");
const CORRECTIONS: &str =
    include_str!("../../site/content/authoring/collection_filter_corrections.json");
const SCHEMA: &str = include_str!("../../public/schema/collection-filter.schema.json");

fn descriptor(collection: &str) -> CollectionFilterDescriptor {
    collection_filter_descriptor(
        collection,
        FilterScope::Project,
        &["New".into(), "In Progress".into(), "Done".into()],
    )
    .expect("published project collection")
}

fn compile(
    collection: &str,
    request: &Value,
) -> Result<CompiledCollectionFilter, CollectionFilterError> {
    let request: FilterRequest = serde_json::from_value(request.clone()).expect("typed request");
    CompiledCollectionFilter::compile(&descriptor(collection), request.filters.as_deref())
}

#[test]
fn collection_filter_published_examples_decode_encode_and_compile() {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    for (collection, data) in [
        ("stories", STORIES),
        ("milestones", MILESTONES),
        ("knowledge_docs", DOCS),
    ] {
        let example: Value = serde_json::from_str(data).unwrap();
        assert_eq!(example["project"], "s-e-e");
        assert!(validator.is_valid(&example));
        let request: FilterRequest = serde_json::from_str(data).unwrap();
        let mut url = Url::parse("http://127.0.0.1/api/projects/s-e-e/stories").unwrap();
        url.query_pairs_mut()
            .append_pair("filters", &example["filters"].to_string());
        assert!(url.as_str().contains("filters=%5B"));
        let pairs: Vec<_> = url.query_pairs().collect();
        let decoded =
            decode_http_filters(pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))).unwrap();
        assert_eq!(decoded, request.filters);
        let descriptor = descriptor(collection);
        assert_eq!(descriptor.scope, FilterScope::Project);
        assert!(
            !CompiledCollectionFilter::compile(&descriptor, decoded.as_deref())
                .unwrap()
                .is_empty()
        );
        assert!(collection_filter_descriptor(collection, FilterScope::Application, &[]).is_none());
    }
}

#[test]
fn collection_filter_story_example_composes_legacy_selection_before_pages() {
    let example: Value = serde_json::from_str(STORIES).unwrap();
    let mut request = example.clone();
    request["milestone"] = request["milestone_id"].clone();
    let mut request: FilteredRequest<StoryListRequest> = serde_json::from_value(request).unwrap();
    request.request.page = ListQuery::page(1, 1);
    let filter = compile("stories", &example).unwrap();
    let rows = [
        ("story-1", "m-45", "Done"),
        ("story-2", "m-45", "New"),
        ("story-3", "m-44", "New"),
        ("story-4", "m-45", "In Progress"),
    ];
    let selected: Vec<_> = rows
        .iter()
        .filter(|(_, milestone, status)| {
            request.request.milestone.as_deref() == Some(*milestone)
                && filter.matches(|_| V::String(status))
        })
        .map(|(id, _, _)| *id)
        .collect();
    let first = ListPage::from_items(selected.clone(), request.request.page);
    let second = ListPage::from_items(selected, ListQuery::page(2, 1));
    assert_eq!(
        (first.items, first.total, first.has_more),
        (vec!["story-2"], 2, true)
    );
    assert_eq!(
        (second.items, second.total, second.has_more),
        (vec!["story-4"], 2, false)
    );
    request.request.status = Some("Done".into());
    assert!(!rows.iter().any(|(_, milestone, status)| {
        request.request.milestone.as_deref() == Some(*milestone)
            && request.request.status.as_deref() == Some(*status)
            && filter.matches(|_| V::String(status))
    }));
}

#[test]
fn collection_filter_milestone_and_doc_examples_select_typed_values() {
    let milestone: Value = serde_json::from_str(MILESTONES).unwrap();
    let filter = compile("milestones", &milestone).unwrap();
    let count = serde_json::Number::from(4);
    let progress = serde_json::Number::from(75);
    assert!(filter.matches(|field| match field {
        "linked_story_count" => V::Number(&count),
        "completion_percent" => V::Number(&progress),
        _ => V::Missing,
    }));
    let complete = serde_json::Number::from(100);
    assert!(!filter.matches(|_| V::Number(&complete)));
    let example: Value = serde_json::from_str(DOCS).unwrap();
    let filter = compile("knowledge_docs", &example).unwrap();
    let tags = vec!["api".into(), "filters".into(), "mcp".into()];
    assert!(filter.matches(|field| match field {
        "type" => V::String("specification"),
        "tags" => V::Strings(&tags),
        _ => V::Missing,
    }));
    assert!(!filter.matches(|field| match field {
        "type" => V::String("guide"),
        "tags" => V::Strings(&tags),
        _ => V::Missing,
    }));
}

#[test]
fn collection_filter_published_corrections_resolve_indexed_diagnostics() {
    let examples: Vec<Value> = serde_json::from_str(CORRECTIONS).unwrap();
    for example in examples {
        let collection = example["collection"].as_str().unwrap();
        let error = compile(collection, &json!({"filters": example["invalid"]})).unwrap_err();
        let CollectionFilterError::Predicate(diagnostic) = error else {
            panic!("predicate error")
        };
        assert_eq!(diagnostic.predicate_index, 0);
        assert_eq!(diagnostic.field, example["invalid"][0]["field"]);
        assert_eq!(diagnostic.code, example["code"]);
        assert!(!diagnostic.remedy.is_empty());
        compile(collection, &json!({"filters": example["corrected"]})).unwrap();
    }
}

#[test]
fn collection_filter_absent_empty_null_and_presence_are_distinct() {
    for example in [json!({}), json!({"filters": []}), json!({"filters": null})] {
        assert!(compile("stories", &example).unwrap().is_empty());
    }
    assert!(decode_http_filters([]).unwrap().is_none());
    assert!(decode_http_filters([("filters", "[]")])
        .unwrap()
        .unwrap()
        .is_empty());
    assert!(
        matches!(decode_http_filters([("filters", "null")]), Err(CollectionFilterError::Request { code, .. }) if code == "invalid_array")
    );
    let ordinary = compile(
        "stories",
        &json!({"filters":[{"field":"milestone_id","op":"ne","value":"m-45"}]}),
    )
    .unwrap();
    assert!(!ordinary.matches(|_| V::Null));
    assert!(!ordinary.matches(|_| V::Missing));
    let presence = compile(
        "stories",
        &json!({"filters":[{"field":"milestone_id","op":"is_unset"}]}),
    )
    .unwrap();
    assert!(presence.matches(|_| V::Null));
    assert!(presence.matches(|_| V::Missing));
    assert!(!presence.matches(|_| V::String("")));
    let error = compile(
        "stories",
        &json!({"filters":[{"field":"milestone_id","op":"eq","value":null}]}),
    )
    .unwrap_err();
    assert!(matches!(error, CollectionFilterError::Predicate(d) if d.code == "missing_value"));
}

#[test]
fn collection_filter_documented_bounds_and_request_errors_are_enforced() {
    let predicate = json!({"field":"status","op":"ne","value":"Done"});
    let at_limit = json!({"filters":vec![predicate.clone(); MAX_FILTER_PREDICATES]});
    compile("stories", &at_limit).unwrap();
    let over_limit = json!({"filters":vec![predicate; MAX_FILTER_PREDICATES + 1]});
    assert!(
        matches!(compile("stories", &over_limit), Err(CollectionFilterError::Request { code, .. }) if code == "predicate_count")
    );
    for (count, expected) in [(MAX_FILTER_MEMBERS, true), (MAX_FILTER_MEMBERS + 1, false)] {
        assert_eq!(
            compile(
                "stories",
                &json!({"filters":[{"field":"id","op":"in","value":vec!["s";count]}]})
            )
            .is_ok(),
            expected
        );
    }
    for (count, expected) in [
        (MAX_FILTER_STRING_CHARS, true),
        (MAX_FILTER_STRING_CHARS + 1, false),
    ] {
        assert_eq!(
            compile(
                "stories",
                &json!({"filters":[{"field":"title","op":"contains","value":"é".repeat(count)}]})
            )
            .is_ok(),
            expected
        );
    }
    let padded = format!("[]{}", " ".repeat(MAX_FILTER_JSON_BYTES - 2));
    assert!(decode_http_filters([("filters", padded.as_str())]).is_ok());
    let oversized = format!("{padded} ");
    for (input, expected) in [
        ("{", "invalid_json"),
        ("{}", "invalid_array"),
        (oversized.as_str(), "json_size"),
    ] {
        assert!(
            matches!(decode_http_filters([("filters", input)]), Err(CollectionFilterError::Request { code, remedy }) if code == expected && !remedy.is_empty())
        );
    }
    assert!(
        matches!(decode_http_filters([("filters", "[]"), ("filters", "[]")]), Err(CollectionFilterError::Request { code, .. }) if code == "duplicate_filters")
    );
}

#[test]
fn collection_filter_schema_rejects_malformed_operands_and_accepts_corrections() {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    for filters in [
        json!([{"field":"status","op":"eq","value":null}]),
        json!([{"field":"status","op":"in","value":[]}]),
        json!([{"field":"milestone_id","op":"is_unset","value":null}]),
        json!([{"field":"status","op":"regex","value":".*"}]),
    ] {
        assert!(!validator.is_valid(&json!({"filters":filters})));
    }
    let examples: Vec<Value> = serde_json::from_str(CORRECTIONS).unwrap();
    for example in examples {
        assert!(validator.is_valid(&json!({"filters":example["corrected"]})));
    }
}

#[test]
fn collection_filter_research_schema_preserves_typed_and_http_query_contracts() {
    let filters: Value = serde_json::from_str(SCHEMA).unwrap();
    let research: Value = serde_json::from_str(include_str!(
        "../../public/schema/knowledge-research.schema.json"
    ))
    .unwrap();
    let registry = jsonschema::Registry::new()
        .add(filters["$id"].as_str().unwrap(), &filters)
        .unwrap()
        .add(research["$id"].as_str().unwrap(), &research)
        .unwrap()
        .prepare()
        .unwrap();
    let root = research["$id"].as_str().unwrap();
    let mcp_schema = json!({"$ref":format!("{root}#/$defs/McpListArgs")});
    let http_schema = json!({"$ref":format!("{root}#/$defs/ListQuery")});
    let mcp = jsonschema::options()
        .with_registry(&registry)
        .build(&mcp_schema)
        .unwrap();
    let http = jsonschema::options()
        .with_registry(&registry)
        .build(&http_schema)
        .unwrap();
    let predicates = json!([{"field":"is_published","op":"eq","value":true}]);
    let example = json!({"project":"s-e-e","state":"published","limit":20,"filters":predicates});
    assert!(mcp.is_valid(&example));
    let request: FilteredRequest<s_e_e_shapes::wire::ResearchScopedListRequest> =
        serde_json::from_value(example).unwrap();
    CompiledCollectionFilter::compile(
        &descriptor("knowledge_research"),
        request.filters.as_deref(),
    )
    .unwrap();
    assert!(http.is_valid(&json!({"state":"published","limit":20,"filters":predicates.to_string()})));
    assert!(!http.is_valid(&json!({"filters":predicates})));
    assert!(!mcp.is_valid(
        &json!({"project":"s-e-e","filters":[{"field":"is_published","op":"eq","value":null}]})
    ));
}
