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
fn footer_shell_overview_points_at_the_marketing_site() {
    let product = super::FOOTER_SHELL_GROUPS
        .iter()
        .find(|group| group.label == "Product")
        .expect("product group");
    let overview = product
        .links
        .iter()
        .find(|link| link.label == "Overview")
        .expect("overview");
    assert_eq!(
        overview.href,
        "https://garunski.github.io/s_e_e_site/product/overview/"
    );
}
