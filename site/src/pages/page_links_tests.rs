use super::{
    authoring_guide_paths_match_route_table, authoring_index_link_hrefs, home_link_hrefs,
    resolve_page_link, AUTHORING_GUIDE_LINKS, AUTHORING_PUBLISH_LINK, HOME_PACKAGE_TYPES,
    HOME_UTILITY_LINKS, SCHEMA_PACKAGE_TYPE_LINKS, PageLink,
};
use crate::pages::schema_package_contract::package_schema_paths;

#[test]
fn home_utility_links_include_catalog_authoring_schema_and_llms() {
    let labels: Vec<_> = HOME_UTILITY_LINKS.iter().map(|link| link.label).collect();
    assert!(labels.iter().any(|label| label.contains("catalog")));
    assert!(labels.contains(&"Authoring"));
    assert!(labels.contains(&"Schema"));
    assert!(labels.contains(&"llms.txt"));
}

#[test]
fn home_package_summaries_cover_five_types() {
    assert_eq!(HOME_PACKAGE_TYPES.len(), 5);
    assert_eq!(HOME_PACKAGE_TYPES[0].title, "Workflows");
    assert_eq!(HOME_PACKAGE_TYPES[4].title, "Bundles");
}

#[test]
fn authoring_index_lists_six_guides_and_publish() {
    assert_eq!(AUTHORING_GUIDE_LINKS.len(), 6);
    assert!(AUTHORING_GUIDE_LINKS.iter().any(|link| link.label == "Cycles"));
    assert_eq!(AUTHORING_PUBLISH_LINK.path, "/authoring/publish/");
}

#[test]
fn resolved_links_use_pages_base_prefix() {
    let base = "/s_e_e_library";
    assert_eq!(
        resolve_page_link(
            base,
            PageLink {
                label: "Schema",
                path: "/schema/",
                external: false,
            }
        ),
        "/s_e_e_library/schema/"
    );
    assert_eq!(
        resolve_page_link(
            base,
            PageLink {
                label: "catalog.json",
                path: "/catalog.json",
                external: false,
            }
        ),
        "/s_e_e_library/catalog.json"
    );
}

#[test]
fn home_and_authoring_href_sets_stay_under_pages_base() {
    let base = "/s_e_e_library";
    for href in home_link_hrefs(base) {
        assert!(
            href.starts_with(base) || href.starts_with("http"),
            "unexpected href: {href}"
        );
    }
    for href in authoring_index_link_hrefs(base) {
        assert!(
            href.starts_with(base) || href.starts_with("http"),
            "unexpected href: {href}"
        );
    }
}

#[test]
fn authoring_guide_paths_align_with_route_table() {
    assert!(authoring_guide_paths_match_route_table());
}

#[test]
fn catalog_index_package_schema_links_match_route_table() {
    let paths = package_schema_paths();
    assert_eq!(SCHEMA_PACKAGE_TYPE_LINKS.len(), paths.len());
    for (link, path) in SCHEMA_PACKAGE_TYPE_LINKS.iter().zip(paths.iter()) {
        assert_eq!(link.path, *path);
    }
    let base = "/s_e_e_library";
    for link in SCHEMA_PACKAGE_TYPE_LINKS {
        assert_eq!(resolve_page_link(base, link), format!("{base}{}", link.path));
    }
}
