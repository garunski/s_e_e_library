use crate::library_href;
use crate::links::library_href_with_base;

#[test]
fn library_href_without_pages_base_keeps_root_relative_paths() {
    assert_eq!(
        library_href_with_base("", "/authoring/workflows/"),
        "/authoring/workflows/"
    );
    assert_eq!(library_href("/"), "/");
}

#[test]
fn library_href_with_pages_base_prefixes_paths() {
    assert_eq!(
        library_href_with_base("/s_e_e_library", "/schema/workflow/"),
        "/s_e_e_library/schema/workflow/"
    );
}
