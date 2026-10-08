use crate::routes::Route;
use crate::shell::chrome::{library_copyright_line, route_current_path};

#[test]
fn route_current_path_maps_documentation_routes() {
    assert_eq!(route_current_path(&Route::Home {}), "/");
    assert_eq!(
        route_current_path(&Route::AuthoringWorkflows {}),
        "/authoring/workflows/"
    );
    assert_eq!(
        route_current_path(&Route::SchemaWorkflow {}),
        "/schema/workflow/"
    );
}

#[test]
fn library_copyright_names_garunski_llc() {
    let line = library_copyright_line(2026);
    assert_eq!(line, "© 2026 Garunski LLC");
    assert!(!line.contains("S.E.E."));
}

#[test]
fn footer_shell_points_at_the_new_marketing_pages() {
    let marketing = super::FOOTER_SHELL_GROUPS
        .iter()
        .find(|group| group.label == "S.E.E.")
        .expect("marketing group");
    let how_it_works = marketing
        .links
        .iter()
        .find(|link| link.label == "How it works")
        .expect("how it works");
    assert_eq!(
        how_it_works.href,
        "https://garunski.github.io/s_e_e_site/how-it-works/"
    );
}
