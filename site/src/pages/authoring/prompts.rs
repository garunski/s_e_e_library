use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, BODY_TEXT, CodeExample, INLINE_CODE, PAGE_LEDE, SchemaReferenceFooter,
    SECTION_TITLE, DL, DD, DT,
};
use crate::routes::Route;

const EXAMPLE: &str = include_str!("../../../content/authoring/prompts_example.json");
const INSTALL: &str = include_str!("../../../content/authoring/prompts_install.json");

#[component]
pub fn LibraryAuthoringPrompts() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Prompts",
            meta_description: "A prompt package stores reusable system prompt text referenced by workflows and commands.",
            rail_number: "02",
            rail_title: "Installs under .s_e_e/prompts.",
            rail_description: "Payload file prompt.json carries id, name, and content.",
            p { class: "{PAGE_LEDE}",
                "A prompt package stores reusable system prompt text referenced by workflows and commands via the "
                code { class: "{INLINE_CODE}", "{{prompt.<id>}}" }
                " template form. Payload file: "
                code { class: "{INLINE_CODE}", "prompt.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/prompts/{{id}}.json" }
                "."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Fields" }
            dl { class: "{DL}",
                dt { class: "{DT}", code { "id" } }
                dd { class: "{DD}",
                    "Stable prompt id (for example "
                    code { class: "{INLINE_CODE}", "system-implement-story" }
                    ")."
                }
                dt { class: "{DT}", code { "name" } }
                dd { class: "{DD}", "Display name in the hub." }
                dt { class: "{DT}", code { "content" } }
                dd { class: "{DD}",
                    "Prompt body; may include "
                    code { class: "{INLINE_CODE}", "{{runtime.*}}" }
                    " placeholders expanded at run time."
                }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Reference example" }
            p { class: "{BODY_TEXT}",
                "From "
                code { class: "{INLINE_CODE}", "packages/system-implement-story/1.0.0/prompt.json" }
                ":"
            }
            CodeExample { text: EXAMPLE }
            p { class: "{BODY_TEXT}",
                "Use a "
                code { class: "{INLINE_CODE}", "system-*" }
                " id for bundled defaults. Keep template placeholders aligned with the workflow "
                code { class: "{INLINE_CODE}", "runtime_inputs" }
                " that declare each "
                code { class: "{INLINE_CODE}", "{{runtime.<key>}}" }
                " placeholder."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Install path" }
            CodeExample { text: INSTALL }
            SchemaReferenceFooter {
                catalog_schema_label: "see.library/v1",
                schema_route: Route::SchemaPrompt {},
                schema_link_label: "Prompt schema",
                extra_schema_links: None,
            }
        }
    }
}
