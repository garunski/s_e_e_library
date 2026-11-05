use super::{CATALOG_IDENTITY, PAYLOAD_LAYOUT};

#[test]
fn catalog_identity_mentions_id_version_and_install_map() {
    let lower = CATALOG_IDENTITY.to_ascii_lowercase();
    assert!(lower.contains("id"));
    assert!(lower.contains("version"));
    assert!(lower.contains("files[]"));
}

#[test]
fn payload_layout_matches_packages_convention() {
    assert!(PAYLOAD_LAYOUT.contains("packages/"));
}
