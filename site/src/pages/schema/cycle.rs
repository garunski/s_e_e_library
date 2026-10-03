use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaCycle() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Cycle schema",
            meta_description: "Category cycle. Payload file cycle.json. Installs to .s_e_e/cycles/{{id}}.json.",
            rail_number: "07",
            rail_title: "Installs under .s_e_e/cycles.",
            rail_description: "Destination is .s_e_e/cycles/{{id}}.json where id matches the document id.",
            p { class: "{PAGE_LEDE}",
                "Category "
                code { class: "{INLINE_CODE}", "cycle" }
                ". Payload file "
                code { class: "{INLINE_CODE}", "cycle.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/cycles/{{id}}.json" }
                " (the "
                code { class: "{INLINE_CODE}", "to" }
                " path must equal "
                code { class: "{INLINE_CODE}", ".s_e_e/cycles/" }
                " plus the document "
                code { class: "{INLINE_CODE}", "id" }
                " plus "
                code { class: "{INLINE_CODE}", ".json" }
                ")."
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "id" }, " - stable cycle id; must match the install file name." }
                li { code { class: "{INLINE_CODE}", "name" }, " - display name." }
                li { code { class: "{INLINE_CODE}", "host" }, " - ", code { class: "{INLINE_CODE}", "orchestrator" }, " or ", code { class: "{INLINE_CODE}", "knowledge" }, "." }
                li { code { class: "{INLINE_CODE}", "schema_version" }, " - integer, currently ", code { class: "{INLINE_CODE}", "1" }, "." }
                li { code { class: "{INLINE_CODE}", "stores" }, " - store keys this cycle uses." }
                li { code { class: "{INLINE_CODE}", "stages[]" }, " - ordered stages with ", code { class: "{INLINE_CODE}", "key" }, ", ", code { class: "{INLINE_CODE}", "verb" }, ", ", code { class: "{INLINE_CODE}", "workflow_definition_id" }, ", ", code { class: "{INLINE_CODE}", "trigger" }, ", ", code { class: "{INLINE_CODE}", "reads" }, ", ", code { class: "{INLINE_CODE}", "writes" }, "." }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                Link { to: Route::AuthoringCycles {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "cycle authoring guide" }
                " for store registry keys and activation checks."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "cycle-document.schema.json" }
            p { class: "mt-6 text-sm text-zinc-600 dark:text-zinc-400",
                Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "Catalog schema" }
            }
        }
    }
}
