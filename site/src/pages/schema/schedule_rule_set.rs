use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaScheduleRuleSet() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Schedule rule set schema",
            meta_description: "Named subject-filter rules for schedules. id and name are required. Validated on save.",
            rail_number: "15",
            rail_title: "Validators run on save.",
            rail_description: ".s_e_e/workflows/schedule_rule_sets. id must be a safe JSON filename.",
            p { class: "{PAGE_LEDE}",
                "Schedule rule set. Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/workflows/schedule_rule_sets/{{id}}.json" }
                "."
            }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "id" }, ", "
                    code { class: "{INLINE_CODE}", "name" }, " - required, non-empty."
                }
                li { code { class: "{INLINE_CODE}", "description" }, " - string." }
                li {
                    code { class: "{INLINE_CODE}", "rules" }, " - object (statuses, labels, acceptance-criteria flags)."
                }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "See "
                Link { to: Route::SchemaSchedule {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "schedule" }
                " for "
                code { class: "{INLINE_CODE}", "story_rules" }
                " that point at a rule set id."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "schedule-rule-set.schema.json" }
        }
    }
}
