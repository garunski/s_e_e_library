use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaPrompt() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Prompt schema",
            meta_description: "Category prompt. Payload file prompt.json. Installs to .s_e_e/prompts/{{file}}.json.",
            rail_number: "02",
            rail_title: "Installs under .s_e_e/prompts.",
            rail_description: "Destination is .s_e_e/prompts/{{file}}.json.",
            p { class: "{PAGE_LEDE}",
                "Category "
                code { class: "{INLINE_CODE}", "prompt" }
                ". Payload file "
                code { class: "{INLINE_CODE}", "prompt.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/prompts/{{file}}.json" }
                "."
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "id" }, " - prompt id used by ", code { class: "{INLINE_CODE}", "{{prompt.ID}}" }, ". Required, non-empty; may carry a namespace prefix." }
                li { code { class: "{INLINE_CODE}", "name" }, " - display name. Required, non-empty." }
                li { code { class: "{INLINE_CODE}", "content" }, " - prompt body; a single string (use ", code { class: "{INLINE_CODE}", "\\n" }, " for line breaks)." }
                li { code { class: "{INLINE_CODE}", "created_at" }, " - ISO-8601 timestamp. Optional." }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "A payload that contains ", code { class: "{INLINE_CODE}", "tasks" }, " is treated as a workflow, not a prompt."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "prompt.schema.json" }
            p { class: "mt-6 text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                Link { to: Route::AuthoringPrompts {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "prompt authoring guide" }
                " and the "
                Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                "."
            }
        }
    }
}
