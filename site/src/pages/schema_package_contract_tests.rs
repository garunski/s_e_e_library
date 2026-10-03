use super::{
    package_schema_paths, package_schema_slugs_match_route_table, CATALOG_INDEX_MARKERS,
    PACKAGE_SCHEMA_PAGES,
};
use crate::route_table::schema_path;

#[test]
fn seven_package_pages_align_with_route_table_prefix() {
    assert!(package_schema_slugs_match_route_table());
    assert_eq!(PACKAGE_SCHEMA_PAGES.len(), 7);
}

#[test]
fn package_schema_paths_are_under_schema() {
    for path in package_schema_paths() {
        assert!(path.starts_with("/schema/"));
        assert!(path.ends_with('/'));
    }
}

#[test]
fn catalog_index_markers_cover_classification_contract() {
    let joined = CATALOG_INDEX_MARKERS.join(" ");
    assert!(joined.contains("see.library/v1"));
    assert!(joined.contains("Package classification"));
    assert!(joined.contains("Install paths"));
}

#[test]
fn workflow_schema_route_matches_slug() {
    let page = PACKAGE_SCHEMA_PAGES
        .iter()
        .find(|p| p.slug == "workflow")
        .expect("workflow page");
    assert_eq!(schema_path(page.slug), "/schema/workflow/");
}
