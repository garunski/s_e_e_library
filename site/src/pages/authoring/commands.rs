use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, BODY_TEXT, CodeExample, INLINE_CODE, PAGE_LEDE, SchemaReferenceFooter,
    SECTION_TITLE, DL, DD, DT,
};
use crate::routes::Route;

const EXAMPLE: &str = include_str!("../../../content/authoring/commands_example.json");
const INSTALL: &str = include_str!("../../../content/authoring/commands_install.json");

#[component]
pub fn LibraryAuthoringCommands() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Commands",
            meta_description: "A command package defines how the S.E.E. engine runs an external agent CLI against a spoke workspace.",
            rail_number: "04",
            rail_title: "Install path is the command id.",
            rail_description: "The catalog to path must equal .s_e_e/commands/{{id}}.json.",
            p { class: "{PAGE_LEDE}",
                "A command package defines how the S.E.E. engine runs an external agent CLI against a spoke workspace. Payload file: "
                code { class: "{INLINE_CODE}", "command.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/commands/{{id}}.json" }
                "."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Fields" }
            dl { class: "{DL}",
                dt { class: "{DT}", code { "binary" } }
                dd { class: "{DD}",
                    "Executable name on "
                    code { class: "{INLINE_CODE}", "PATH" }
                    " (for example "
                    code { class: "{INLINE_CODE}", "cursor" }
                    ", "
                    code { class: "{INLINE_CODE}", "claude" }
                    ")."
                }
                dt { class: "{DT}", code { "base_args" } }
                dd { class: "{DD}", "Arguments always passed before operator overrides." }
                dt { class: "{DT}", code { "forced_args" } }
                dd { class: "{DD}", "Arguments the engine appends; not overridable." }
                dt { class: "{DT}", code { "params" } }
                dd { class: "{DD}",
                    "Typed run-time knobs ("
                    code { class: "{INLINE_CODE}", "string" }
                    ", "
                    code { class: "{INLINE_CODE}", "number" }
                    ", "
                    code { class: "{INLINE_CODE}", "boolean" }
                    ", "
                    code { class: "{INLINE_CODE}", "enum" }
                    ", "
                    code { class: "{INLINE_CODE}", "string_array" }
                    ") mapped to env, timeout, extra args, lock policy, or stream toggle."
                }
                dt { class: "{DT}", code { "result" } }
                dd { class: "{DD}",
                    "Success rule ("
                    code { class: "{INLINE_CODE}", "success_when" }
                    "), captured fields, optional "
                    code { class: "{INLINE_CODE}", "applied_paths" }
                    " and "
                    code { class: "{INLINE_CODE}", "run_id" }
                    " strategies."
                }
                dt { class: "{DT}", code { "execution" } }
                dd { class: "{DD}",
                    "Working directory ("
                    code { class: "{INLINE_CODE}", "cwd" }
                    ": "
                    code { class: "{INLINE_CODE}", "spoke" }
                    " or "
                    code { class: "{INLINE_CODE}", "hub" }
                    "), lock scope, stream format, "
                    code { class: "{INLINE_CODE}", "non_interactive" }
                    "."
                }
            }
            p { class: "{BODY_TEXT}",
                "Additional fields: "
                code { class: "{INLINE_CODE}", "id" }
                ", "
                code { class: "{INLINE_CODE}", "name" }
                ", "
                code { class: "{INLINE_CODE}", "icon" }
                ", "
                code { class: "{INLINE_CODE}", "workspace_arg" }
                ", "
                code { class: "{INLINE_CODE}", "prompt" }
                " delivery, "
                code { class: "{INLINE_CODE}", "env" }
                ", and optional "
                code { class: "{INLINE_CODE}", "mcp" }
                " provisioning."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Reference example" }
            p { class: "{BODY_TEXT}",
                "From "
                code { class: "{INLINE_CODE}", "packages/cursor-agent/1.0.0/command.json" }
                ":"
            }
            CodeExample { text: EXAMPLE }
            h2 { class: "mt-8 {SECTION_TITLE}", "Install path" }
            p { class: "{BODY_TEXT}", "Catalog " code { class: "{INLINE_CODE}", "files[]" } " entry:" }
            CodeExample { text: INSTALL }
            p { class: "{BODY_TEXT}",
                "The "
                code { class: "{INLINE_CODE}", "to" }
                " path must equal "
                code { class: "{INLINE_CODE}", ".s_e_e/commands/{{id}}.json" }
                " where "
                code { class: "{INLINE_CODE}", "id" }
                " matches "
                code { class: "{INLINE_CODE}", "command.json" }
                " "
                code { class: "{INLINE_CODE}", "id" }
                "."
            }
            SchemaReferenceFooter {
                catalog_schema_label: "see.library/v1",
                schema_route: Route::SchemaCommand {},
                schema_link_label: "Command schema",
                extra_schema_links: None,
            }
        }
    }
}
