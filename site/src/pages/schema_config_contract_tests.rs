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
