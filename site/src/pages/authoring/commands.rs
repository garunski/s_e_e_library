use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, CodeExample, SchemaReferenceFooter, BODY_TEXT, DD, DL, DT, INLINE_CODE,
    PAGE_LEDE, SECTION_TITLE,
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
            h2 { class: "mt-8 {SECTION_TITLE}", "Codex JSON output" }
            p { class: "{BODY_TEXT}",
                "The success_when rule supports one comparison of exit_code with a signed 32-bit integer using ==, !=, <, <=, > or >=, with optional whitespace. Unsupported expressions fail before the child starts. Generic commands may accept a configured nonzero code; Codex JSON commands always also require actual exit code zero."
            }
            p { class: "{BODY_TEXT}",
                "Codex capture is enabled only when the resolved command has "
                code { class: "{INLINE_CODE}", "cliId: codex" }
                " and its effective argv contains "
                code { class: "{INLINE_CODE}", "--json" }
                ". The "
                code { class: "{INLINE_CODE}", "execution.stream.format" }
                " field remains unused metadata and does not select a parser. At actual child exit, output may include "
                code { class: "{INLINE_CODE}", "session_id" }
                ", "
                code { class: "{INLINE_CODE}", "final_message" }
                ", "
                code { class: "{INLINE_CODE}", "usage" }
                ", and "
                code { class: "{INLINE_CODE}", "codex_terminal" }
                "; capture and terminal failures add "
                code { class: "{INLINE_CODE}", "codex_error" }
                ". These fields preserve SEE's "
                code { class: "{INLINE_CODE}", "run_id" }
                ", actual process "
                code { class: "{INLINE_CODE}", "exit_code" }
                ", duration, and output tails."
            }
            p { class: "{BODY_TEXT}",
                "Codex task success requires the actual child exit code to be zero, the configured "
                code { class: "{INLINE_CODE}", "success_when" }
                " rule to pass, and the latest turn to end with "
                code { class: "{INLINE_CODE}", "turn.completed" }
                ". A failed or missing terminal turn, nonzero exit, or frame/final-message size over 4,194,304 bytes fails the task. A later "
                code { class: "{INLINE_CODE}", "turn.started" }
                " makes the previous terminal event incomplete. Recovered standalone errors do not fail a completed turn. Explicit "
                code { class: "{INLINE_CODE}", "result.extract[]" }
                " behavior continues to use its configured stdout/stderr source and path."
            }
            p { class: "{BODY_TEXT}",
                "Codex transport failures keep the existing four-attempt retry bound when the child exits nonzero, stderr contains RetriableError, and no terminal event or capture-size failure was received. Each attempt starts with fresh capture metadata. Failed terminal turns, zero-exit incomplete streams, capture-size failures and success-rule mismatches are not retried. Cancellation and timeout keep their existing cleanup behavior. Command success does not verify story criteria or mark a story Done."
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
