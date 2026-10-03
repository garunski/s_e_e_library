use crate::routes::Route;
use crate::shell::chrome::route_current_path;

#[test]
fn route_current_path_maps_documentation_routes() {
    assert_eq!(route_current_path(&Route::Home {}), "/");
    assert_eq!(
        route_current_path(&Route::AuthoringWorkflows {}),
        "/authoring/workflows/"
    );
    assert_eq!(route_current_path(&Route::SchemaWorkflow {}), "/schema/workflow/");
}
