use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, AuthoringReference, CodeExample, BODY_TEXT, DD, DL, DT, INLINE_CODE,
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
            section {
                h2 { "Prompt fields" }
                p { class: BODY_TEXT,
                    "A prompt package stores reusable system prompt text referenced by workflows and commands via the "
                    code { class: "{INLINE_CODE}", "{{prompt.<id>}}" }
                    " template form. Payload file: "
                    code { class: "{INLINE_CODE}", "prompt.json" }
                    ". Installs to "
                    code { class: "{INLINE_CODE}", ".s_e_e/prompts/{{id}}.json" }
                    "."
                }
                dl { class: "{DL}",
                    div {
                        dt { class: "{DT}",
                            code { "id" }
                        }
                        dd { class: "{DD}",
                            "Stable prompt id (for example "
                            code { class: "{INLINE_CODE}", "system-implement-story" }
                            ")."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "name" }
                        }
                        dd { class: "{DD}", "Display name in the hub." }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "content" }
                        }
                        dd { class: "{DD}",
                            "Prompt body; may include "
                            code { class: "{INLINE_CODE}", "{{runtime.*}}" }
                            " placeholders expanded at run time."
                        }
                    }
                }
            }
            section {
                h2 { "Hub MCP assignment" }
                p { class: "{BODY_TEXT}",
                    "Installed "
                    code { class: "{INLINE_CODE}", "prompt.json" }
                    " files carry only "
                    code { class: "{INLINE_CODE}", "id" }
                    ", "
                    code { class: "{INLINE_CODE}", "name" }
                    ", and "
                    code { class: "{INLINE_CODE}", "content" }
                    ". In the hub Prompts editor you may assign an MCP prompt name and choose Legacy authoring arguments, Project only, or up to 32 custom MCP argument rows. Those hub fields are not part of portable library packages."
                }
            }
            section {
                h2 { "Example" }
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
            }
            section {
                h2 { "Installation" }
                CodeExample { text: INSTALL }
            }
            AuthoringReference {
                schema_route: Route::SchemaPrompt {},
                schema_link_label: "Prompt schema",
                extra_schema_links: None,
            }
        }
    }
}
