use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaRoutingRules() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Routing rules schema",
            meta_description: "LLM task_profiles and cost_caps. Stored as llm.routing in hub config.json. Validated on save.",
            rail_number: "11",
            rail_title: "Validators run on save.",
            rail_description: "llm.routing. Lives in hub config.json under llm.routing.",
            p { class: "{PAGE_LEDE}",
                "LLM routing rules. Stored as "
                code { class: "{INLINE_CODE}", "llm.routing" }
                " in "
                Link { to: Route::SchemaHubConfig {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "hub config" }
                "."
            }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "task_profiles" }, " - required object. Each value is "
                    code { class: "{INLINE_CODE}", "{{ provider, model, requires_tool_calling? }}" }, ". "
                    code { class: "{INLINE_CODE}", "provider" }, " and "
                    code { class: "{INLINE_CODE}", "model" }, " are required."
                }
                li {
                    code { class: "{INLINE_CODE}", "cost_caps" }, " - object. Each value is "
                    code { class: "{INLINE_CODE}", "{{ max_usd }}" }, "."
                }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "At least one task profile is required at save time." }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "routing-rules.schema.json" }
        }
    }
}
