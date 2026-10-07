use dioxus::prelude::Routable;

use crate::documentation_route_paths;
use crate::route_table::{AUTHORING_SLUGS, SCHEMA_SLUGS};
use crate::Route;

#[test]
fn documentation_route_count_matches_legacy_site() {
    assert_eq!(documentation_route_paths().len(), 25);
}

#[test]
fn authoring_and_schema_slug_counts_match_authoring_pages_script() {
    assert_eq!(AUTHORING_SLUGS.len(), 7);
    assert_eq!(SCHEMA_SLUGS.len(), 15);
}

#[test]
fn documentation_paths_use_trailing_slashes_except_home() {
    for path in documentation_route_paths() {
        if path == "/" {
            continue;
        }
        assert!(path.ends_with('/'), "expected trailing slash on {path}");
        assert!(path.starts_with('/'));
    }
}

#[test]
fn static_routes_match_documentation_route_table() {
    assert_eq!(
        Route::static_routes().len(),
        documentation_route_paths().len()
    );
}
