use dioxus::prelude::*;

use crate::pages::authoring_common::{INLINE_CODE, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};

#[component]
pub fn LibrarySchemaGlobalConfig() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Global config schema",
            meta_description: "User-level ~/.s_e_e/config.json. A JSON object. Validated on save.",
            rail_number: "09",
            rail_title: "Validators run on save.",
            rail_description: "~/.s_e_e/config.json. The value must be a JSON object, not an array or scalar.",
            p { class: "{PAGE_LEDE}",
                "User-level "
                code { class: "{INLINE_CODE}", "~/.s_e_e/config.json" }
                ". Must be a JSON object. Keys are free-form."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "global-config.schema.json" }
        }
    }
}
