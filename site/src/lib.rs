mod links;
mod nav;
mod pages;
pub mod route_table;
mod routes;
mod shell;
mod shell_contract;
mod theme;

#[cfg(test)]
mod links_tests;
#[cfg(test)]
mod route_table_tests;
#[cfg(test)]
mod visual_tokens_tests;

pub use links::{library_href, pages_base};
pub use route_table::{
    authoring_path, documentation_route_paths, schema_path, AUTHORING_SLUGS, SCHEMA_SLUGS,
};
pub use routes::Route;

use dioxus::prelude::*;
use theme::THEME_BOOT_SCRIPT;

/// Route list consumed by `dx bundle --ssg` (`GET /api/static_routes`).
#[server]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(documentation_route_paths()
        .into_iter()
        .map(str::to_string)
        .collect())
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Style { "{s_e_e_gui_kit::brand::commissioner_font_face()}" }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/dx-components-theme.css") }
        document::Link { rel: "icon", href: asset!("/assets/branding/logo-32.png") }
        document::Script { {THEME_BOOT_SCRIPT} }
        Router::<Route> {}
    }
}
