use crate::nav::{is_primary_current, shell_layout_classes, PRIMARY_NAV};

#[test]
fn primary_nav_links_to_library_home_and_sections() {
    assert_eq!(PRIMARY_NAV[0].label, "Home");
    assert_eq!(PRIMARY_NAV[0].path, "/");
    assert!(!PRIMARY_NAV[0].external);
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
    assert!(is_primary_current("/", "/"));
    assert!(!is_primary_current("/", "/schema/"));
}

#[test]
fn shell_layout_prevents_horizontal_overflow() {
    let classes = shell_layout_classes();
    assert!(classes.root.contains("min-w-0"));
    assert!(classes.root.contains("overflow-x-hidden"));
    assert!(classes.content_inner.contains("min-w-0"));
}
