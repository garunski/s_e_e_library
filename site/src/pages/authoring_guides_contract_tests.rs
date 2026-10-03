use super::{guide_contract, PACKAGE_AUTHORING_GUIDES};
use crate::route_table::authoring_path;

#[test]
fn six_package_guides_match_route_table_paths() {
    assert_eq!(PACKAGE_AUTHORING_GUIDES.len(), 6);
    for guide in PACKAGE_AUTHORING_GUIDES {
        let path = authoring_path(guide.slug);
        assert!(path.starts_with("/authoring/"));
        assert!(path.ends_with('/'));
        assert!(!guide.document_title.is_empty());
        assert!(!guide.schema_path.is_empty());
    }
}

#[test]
fn workflow_guide_names_payload_and_schema() {
    let guide = guide_contract("workflows").expect("workflows guide");
    assert_eq!(guide.payload_file, "definition.json");
    assert_eq!(guide.schema_path, "/schema/workflow/");
}

#[test]
fn bundle_guide_uses_v2_catalog_label() {
    let guide = guide_contract("bundles").expect("bundles guide");
    assert_eq!(guide.catalog_schema_label, "see.library/v2");
}
