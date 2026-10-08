use crate::route_table::documentation_route_paths;
use crate::routes::{route_for_path, Route};

#[test]
fn route_for_path_covers_every_documentation_route() {
    for path in documentation_route_paths() {
        assert!(route_for_path(path).is_some(), "{path}");
    }
    assert_eq!(route_for_path("/"), Some(Route::Home {}));
    assert_eq!(route_for_path("/catalog.json"), None);
    assert_eq!(route_for_path("/llms.txt"), None);
    assert_eq!(
        route_for_path("https://garunski.github.io/s_e_e_site/"),
        None
    );
}
