use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaHubConfig() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Hub config schema",
            meta_description: "Hub data-root config.json. A JSON object with at least one property. Validated on save.",
            rail_number: "08",
            rail_title: "Validators run on save.",
            rail_description: ".s_e_e/config.json. additionalProperties is allowed so project-specific keys persist.",
            p { class: "{PAGE_LEDE}",
                "Hub data-root "
                code { class: "{INLINE_CODE}", "config.json" }
                ". Must be a non-empty JSON object."
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "project" }, " - object; typically includes ", code { class: "{INLINE_CODE}", "id" }, "." }
                li { code { class: "{INLINE_CODE}", "overrides" }, " - object." }
                li {
                    code { class: "{INLINE_CODE}", "library" }, " - catalog sources and collections. See "
                    Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                    "."
                }
                li {
                    code { class: "{INLINE_CODE}", "stories" }, " - see "
                    Link { to: Route::SchemaStoriesConfig {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "stories config" }
                    "."
                }
                li {
                    code { class: "{INLINE_CODE}", "llm.routing" }, " - see "
                    Link { to: Route::SchemaRoutingRules {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "routing rules" }
                    "."
                }
                li {
                    code { class: "{INLINE_CODE}", "orchestrator" }, " - see "
                    Link { to: Route::SchemaOrchestratorPolicy {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "orchestrator policy" }
                    "."
                }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Unknown top-level keys are kept." }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "hub-config.schema.json" }
        }
    }
}
