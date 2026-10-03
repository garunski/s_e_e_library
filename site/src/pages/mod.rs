mod authoring;
mod authoring_common;
mod authoring_guides_contract;
mod authoring_index;
mod home;
mod page_links;
mod schema;
mod schema_assets;
mod schema_common;
mod schema_config_contract;
mod schema_package_contract;

pub use authoring::{
    LibraryAuthoringBundles, LibraryAuthoringCommands, LibraryAuthoringCycles,
    LibraryAuthoringPrompts, LibraryAuthoringPublish, LibraryAuthoringSkills,
    LibraryAuthoringWorkflows,
};
pub use authoring_index::LibraryAuthoringIndex;
pub use home::LibraryHome;
pub use schema::{
    LibrarySchemaAppSettings, LibrarySchemaBundle, LibrarySchemaCommand, LibrarySchemaCycle,
    LibrarySchemaGlobalConfig, LibrarySchemaHubConfig, LibrarySchemaIndex,
    LibrarySchemaOrchestratorPolicy, LibrarySchemaPrompt, LibrarySchemaRoutingRules,
    LibrarySchemaRuleTemplate, LibrarySchemaSchedule, LibrarySchemaScheduleRuleSet,
    LibrarySchemaSkill, LibrarySchemaStoriesConfig, LibrarySchemaWorkflow,
};

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::routes::Route;
use crate::shell::LibraryDocFrame;

pub fn doc_placeholder(
    title: &'static str,
    description: &'static str,
    rail_title: &'static str,
    rail_description: &'static str,
) -> Element {
    rsx! {
        DocPlaceholder {
            title,
            description,
            rail_title,
            rail_description,
        }
    }
}

#[component]
pub fn DocPlaceholder(
    title: &'static str,
    description: &'static str,
    rail_title: &'static str,
    rail_description: &'static str,
) -> Element {
    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "description", content: "{description}" }
        LibraryDocFrame {
            title: title.to_string(),
            description: description.to_string(),
            rail_number: "01",
            rail_title,
            rail_description,
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "Placeholder page. Content will be ported from the legacy site."
            }
            Link {
                to: Route::Home {},
                class: "mt-4 inline-flex text-xs font-medium text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200",
                "Library home"
            }
        }
    }
}

#[component]
pub fn LibraryNotFound(segments: Vec<String>) -> Element {
    let path = if segments.is_empty() {
        String::new()
    } else {
        format!("/{}", segments.join("/"))
    };
    rsx! {
        document::Title { "Page not found" }
        document::Meta {
            name: "description",
            content: "The requested library documentation page does not exist.",
        }
        LibraryDocFrame {
            title: "Page not found".to_string(),
            description: "The requested library documentation page does not exist.".to_string(),
            rail_number: "404",
            rail_title: "Not found",
            rail_description: "This URL is not part of the published documentation set.",
            if !path.is_empty() {
                p { class: "text-sm text-zinc-600 dark:text-zinc-400", "No page at {path}." }
            }
            Link {
                to: Route::Home {},
                class: "mt-4 inline-flex text-xs font-medium text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200",
                "Back to library home"
            }
        }
    }
}
