use super::{config_schema_paths, config_schema_slugs_match_route_table, CONFIG_SCHEMA_PAGES};
use crate::pages::schema_assets::schema_download;
use crate::route_table::schema_path;

#[test]
fn eight_config_pages_align_with_route_table_suffix() {
    assert!(config_schema_slugs_match_route_table());
    assert_eq!(CONFIG_SCHEMA_PAGES.len(), 8);
}

#[test]
fn config_schema_paths_are_under_schema() {
    for path in config_schema_paths() {
        assert!(path.starts_with("/schema/"));
        assert!(path.ends_with('/'));
    }
}

#[test]
fn each_config_page_has_embedded_schema_file() {
    for page in CONFIG_SCHEMA_PAGES {
        let entry = schema_download(page.schema_filename)
            .unwrap_or_else(|| panic!("missing embed for {}", page.schema_filename));
        assert_eq!(entry.filename, page.schema_filename);
        assert!(!entry.json.is_empty());
    }
}

#[test]
fn hub_config_route_matches_slug() {
    let page = CONFIG_SCHEMA_PAGES
        .iter()
        .find(|p| p.slug == "hub-config")
        .expect("hub-config page");
    assert_eq!(schema_path(page.slug), "/schema/hub-config/");
    assert!(!page.field_markers.is_empty());
}

#[test]
fn experimental_features_docs_public_schema_matches_application_contract() {
    let public: serde_json::Value =
        serde_json::from_str(schema_download("hub-config.schema.json").unwrap().json).unwrap();
    let source_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../s_e_e/core/schema/hub-config.schema.json");
    let mut source: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(source_path).unwrap()).unwrap();
    source["$id"] = public["$id"].clone();
    assert_eq!(public, source);
    let flags = &public["properties"]["project"]["properties"]["experimental_features"];
    assert_eq!(flags["additionalProperties"], false);
    for key in [
        "orchestrator",
        "orchestrator_cycle",
        "knowledge_cycle",
        "planning_cycle",
        "knowledge_research",
        "global_chat",
    ] {
        assert_eq!(flags["properties"][key]["type"], "boolean");
        assert_eq!(flags["properties"][key]["default"], false);
    }
    assert!(CONFIG_SCHEMA_PAGES[0]
        .field_markers
        .contains(&"project.experimental_features"));
    let llms = include_str!("../../../public/llms.txt");
    let section = llms
        .split("Source: /schema/hub-config/")
        .nth(1)
        .unwrap()
        .split("\nSource: ")
        .next()
        .unwrap();
    let json = section
        .split("```json\n")
        .nth(1)
        .unwrap()
        .split("```")
        .next()
        .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(json).unwrap(),
        public
    );
}
