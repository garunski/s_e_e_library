use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaCommand() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Command schema",
            meta_description: "Category command. Payload file command.json. Installs to .s_e_e/commands/{{id}}.json.",
            rail_number: "04",
            rail_title: "The to path must match the id.",
            rail_description: "Destination is .s_e_e/commands/{{id}}.json, exactly.",
            p { class: "{PAGE_LEDE}",
                "Category "
                code { class: "{INLINE_CODE}", "command" }
                ". Payload file "
                code { class: "{INLINE_CODE}", "command.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/commands/{{id}}.json" }
                " (the "
                code { class: "{INLINE_CODE}", "to" }
                " path must equal this exactly)."
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "id" }, " - unique command id; also the install filename. Required, non-empty." }
                li { code { class: "{INLINE_CODE}", "name" }, ", ", code { class: "{INLINE_CODE}", "icon" }, " - display metadata." }
                li { code { class: "{INLINE_CODE}", "binary" }, " - executable to run. Required, non-empty." }
                li { code { class: "{INLINE_CODE}", "base_args" }, ", ", code { class: "{INLINE_CODE}", "forced_args" }, ", ", code { class: "{INLINE_CODE}", "workspace_arg" }, ", ", code { class: "{INLINE_CODE}", "prompt" }, ", ", code { class: "{INLINE_CODE}", "params[]" }, ", ", code { class: "{INLINE_CODE}", "env" }, ", ", code { class: "{INLINE_CODE}", "result" }, ", ", code { class: "{INLINE_CODE}", "execution" }, ", ", code { class: "{INLINE_CODE}", "mcp" }, " - see the command authoring guide for shapes." }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "A payload with ", code { class: "{INLINE_CODE}", "content" }, " but no ", code { class: "{INLINE_CODE}", "binary" }, " is treated as a prompt."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "command.schema.json" }
            p { class: "mt-6 text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                Link { to: Route::AuthoringCommands {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "command authoring guide" }
                " and the "
                Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                "."
            }
        }
    }
}
