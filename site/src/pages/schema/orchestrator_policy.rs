use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaOrchestratorPolicy() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Orchestrator policy schema",
            meta_description: "auto_mode, policy.levels, and budgets. Stored as orchestrator in hub config.json. Validated on save.",
            rail_number: "13",
            rail_title: "Validators run on save.",
            rail_description: "orchestrator. policy and budgets are required. Lives in hub config.json.",
            p { class: "{PAGE_LEDE}",
                "Orchestrator policy. Stored as "
                code { class: "{INLINE_CODE}", "orchestrator" }
                " in "
                Link { to: Route::SchemaHubConfig {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "hub config" }
                "."
            }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "auto_mode" }, " - "
                    code { class: "{INLINE_CODE}", "off" }, ", "
                    code { class: "{INLINE_CODE}", "assisted" }, ", or "
                    code { class: "{INLINE_CODE}", "auto" }, "."
                }
                li {
                    code { class: "{INLINE_CODE}", "policy" }, " - required. "
                    code { class: "{INLINE_CODE}", "levels" }, " maps action classes to "
                    code { class: "{INLINE_CODE}", "auto" }, ", "
                    code { class: "{INLINE_CODE}", "propose" }, ", or "
                    code { class: "{INLINE_CODE}", "forbid" }, "."
                }
                li { code { class: "{INLINE_CODE}", "budgets" }, " - required object (run caps, spend caps, window)." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "orchestrator-policy.schema.json" }
        }
    }
}
