use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaSchedule() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Schedule schema",
            meta_description: "Workflow schedule JSON. id, name, workflow_id, kind, and enabled are required. Validated on save.",
            rail_number: "14",
            rail_title: "Validators run on save.",
            rail_description: ".s_e_e/workflows/schedules. workflow_id must name an existing workflow definition.",
            p { class: "{PAGE_LEDE}",
                "Workflow schedule. Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/workflows/schedules/{{id}}.json" }
                ". Bundles may include schedule files; kind is inferred from the "
                code { class: "{INLINE_CODE}", "to" }
                " prefix."
            }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "id" }, ", "
                    code { class: "{INLINE_CODE}", "name" }, ", "
                    code { class: "{INLINE_CODE}", "workflow_id" }, " - required, non-empty."
                }
                li {
                    code { class: "{INLINE_CODE}", "kind" }, " - required object with a "
                    code { class: "{INLINE_CODE}", "kind" }, " tag ("
                    code { class: "{INLINE_CODE}", "cron" }, ", "
                    code { class: "{INLINE_CODE}", "interval" }, ", or "
                    code { class: "{INLINE_CODE}", "entity_event" }, ")."
                }
                li { code { class: "{INLINE_CODE}", "enabled" }, " - required boolean." }
                li {
                    code { class: "{INLINE_CODE}", "concurrency" }, ", "
                    code { class: "{INLINE_CODE}", "runtime_inputs" }, ", "
                    code { class: "{INLINE_CODE}", "story_rules" }, ", "
                    code { class: "{INLINE_CODE}", "catchup_policy" }, ", "
                    code { class: "{INLINE_CODE}", "notifications" }, " - objects."
                }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "Subject filters can point at a "
                Link { to: Route::SchemaScheduleRuleSet {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "rule set" }
                " by id."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "schedule.schema.json" }
        }
    }
}
