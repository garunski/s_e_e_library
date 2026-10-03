use crate::nav::{is_primary_current, shell_layout_classes, PRIMARY_NAV};

#[test]
fn primary_nav_covers_authoring_schema_and_llms() {
    let labels: Vec<_> = PRIMARY_NAV.iter().map(|link| link.label).collect();
    assert!(labels.contains(&"Authoring"));
    assert!(labels.contains(&"Schema"));
    assert!(labels.contains(&"llms.txt"));
}

#[test]
fn is_primary_current_matches_section_roots() {
    assert!(is_primary_current("/authoring/", "/authoring/workflows/"));
    assert!(is_primary_current("/schema/", "/schema/workflow/"));
    assert!(!is_primary_current("/authoring/", "/schema/"));
}

#[test]
fn shell_layout_prevents_horizontal_overflow() {
    let classes = shell_layout_classes();
    assert!(classes.root.contains("min-w-0"));
    assert!(classes.root.contains("overflow-x-hidden"));
    assert!(classes.content_inner.contains("min-w-0"));
}
