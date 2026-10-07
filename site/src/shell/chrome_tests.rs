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
