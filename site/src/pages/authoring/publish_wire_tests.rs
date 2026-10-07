use super::{CATALOG_IDENTITY, DOCS_CHECK_COMMANDS, PAYLOAD_LAYOUT, VALIDATE_AND_CATALOG_COMMANDS};

#[test]
fn publish_commands_use_mise_catalog_tool_tasks_not_node() {
    for block in [VALIDATE_AND_CATALOG_COMMANDS, DOCS_CHECK_COMMANDS] {
        let lower = block.to_ascii_lowercase();
        assert!(!lower.contains("npm"), "unexpected npm in {block}");
        assert!(!lower.contains("node "), "unexpected node in {block}");
        assert!(
            lower.contains("mise run validate")
                || lower.contains("mise run catalog")
                || lower.contains("mise run harvest")
                || lower.contains("mise run quality"),
            "expected catalog_tool mise task in {block}"
        );
    }
}

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
