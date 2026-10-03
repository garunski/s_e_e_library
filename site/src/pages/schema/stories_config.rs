use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaStoriesConfig() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Stories config schema",
            meta_description: "Stories section of hub config.json. project_name is required.",
            rail_number: "12",
            rail_title: "Lives in hub config.json.",
            rail_description: "stories. project_name is required and must be non-empty.",
            p { class: "{PAGE_LEDE}",
                "Stories section of "
                Link { to: Route::SchemaHubConfig {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "hub config" }
                "."
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "project_name" }, " - required, non-empty." }
                li { code { class: "{INLINE_CODE}", "statuses" }, " - string array." }
                li { code { class: "{INLINE_CODE}", "labels" }, " - string array." }
                li { code { class: "{INLINE_CODE}", "story_prefix" }, " - non-empty string when present." }
                li { code { class: "{INLINE_CODE}", "date_format" }, " - non-empty string when present." }
                li { code { class: "{INLINE_CODE}", "default_create_status" }, " - string." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "stories-config.schema.json" }
        }
    }
}
