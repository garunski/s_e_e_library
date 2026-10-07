use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, CodeExample, SchemaReferenceFooter, BODY_TEXT, DD, DL, DT, INLINE_CODE,
    LIST, PAGE_LEDE, SECTION_TITLE, SUBSECTION_TITLE,
};
use crate::routes::Route;

const UNBOUND_TASK: &str = include_str!("../../../content/authoring/workflows_unbound_task.json");
const FIXED_TASK: &str = include_str!("../../../content/authoring/workflows_fixed_task.json");
const REFERENCE_EXAMPLE: &str =
    include_str!("../../../content/authoring/workflows_reference_example.json");
const INSTALL: &str = include_str!("../../../content/authoring/workflows_install.json");

#[component]
pub fn LibraryAuthoringWorkflows() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Workflows",
            meta_description: "A workflow package defines tasks the S.E.E. engine runs against a hub or spoke.",
            rail_number: "01",
            rail_title: "Installs under workflow definitions.",
            rail_description: "Destination is .s_e_e/workflows/definitions/{{slug}}.json.",
            p { class: "{PAGE_LEDE}",
                "A workflow package defines tasks the S.E.E. engine runs against a hub or spoke. Payload file: "
                code { class: "{INLINE_CODE}", "definition.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/workflows/definitions/{{slug}}.json" }
                "."
            }
            p { class: "{BODY_TEXT}",
                "Authoring rules live in the bundled "
                strong { "workflow" }
                " skill ("
                code { class: "{INLINE_CODE}", "packages/workflow/1.0.0/SKILL.md" }
                "). This page covers the library payload shape only."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Envelope (definition.json)" }
            dl { class: "{DL}",
                dt { class: "{DT}", code { "id" } }
                dd { class: "{DD}",
                    "Outer id, usually "
                    code { class: "{INLINE_CODE}", "system-&lt;name&gt;" }
                    "."
                }
                dt { class: "{DT}", code { "name" }, ", ", code { "description" } }
                dd { class: "{DD}", "Hub display metadata." }
                dt { class: "{DT}", code { "content" } }
                dd { class: "{DD}", "Engine workflow object." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Inner content" }
            dl { class: "{DL}",
                dt { class: "{DT}", code { "runtime_inputs" } }
                dd { class: "{DD}",
                    "Run-start values; each object with "
                    code { class: "{INLINE_CODE}", "key" }
                    ", "
                    code { class: "{INLINE_CODE}", "input_type" }
                    ", "
                    code { class: "{INLINE_CODE}", "required" }
                    ", "
                    code { class: "{INLINE_CODE}", "label" }
                    "; keys must not contain dots."
                }
                dt { class: "{DT}", code { "default_command_id" } }
                dd { class: "{DD}",
                    "Optional installed command id for "
                    code { class: "{INLINE_CODE}", "command" }
                    " tasks that omit "
                    code { class: "{INLINE_CODE}", "command_id" }
                    ". Empty strings are invalid."
                }
                dt { class: "{DT}", code { "tasks" } }
                dd { class: "{DD}", "Ordered graph of engine tasks." }
                dt { class: "{DT}", code { "spoke_root" } }
                dd { class: "{DD}",
                    "Required when command tasks use spoke cwd or git actions omit per-task spoke. Set it to an existing absolute path or an exact "
                    code { class: "{INLINE_CODE}", "{{runtime.KEY}}" }
                    " reference to a string runtime input. The run resolves and validates the path before executing tasks and again when resuming after user input."
                }
            }
            h3 { class: "mt-6 {SUBSECTION_TITLE}", "Task types" }
            p { class: "{BODY_TEXT}",
                "Each task has exactly one handler under "
                code { class: "{INLINE_CODE}", "function" }
                ":"
            }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "cli_command" }
                    ": shell command and args"
                }
                li {
                    code { class: "{INLINE_CODE}", "command" }
                    ": run an installed command definition. Omit "
                    code { class: "{INLINE_CODE}", "command_id" }
                    " to leave the task unbound, set "
                    code { class: "{INLINE_CODE}", "default_command_id" }
                    " on the workflow, or pin a non-empty "
                    code { class: "{INLINE_CODE}", "command_id" }
                    " to fix the task to one CLI. Optional "
                    code { class: "{INLINE_CODE}", "prompt" }
                    " when the definition uses prompt delivery."
                }
                li {
                    code { class: "{INLINE_CODE}", "user_input" }
                    ": pause the run to collect an operator answer when this task is reached"
                }
                li {
                    code { class: "{INLINE_CODE}", "git_action" }
                    ", "
                    code { class: "{INLINE_CODE}", "s_e_e_action" }
                    ", "
                    code { class: "{INLINE_CODE}", "foreach" }
                    ", "
                    code { class: "{INLINE_CODE}", "custom" }
                }
            }
            p { class: "{BODY_TEXT}",
                code { class: "{INLINE_CODE}", "s_e_e_action" }
                " calls one MCP tool. "
                code { class: "{INLINE_CODE}", "input.tool" }
                " is the tool name. "
                code { class: "{INLINE_CODE}", "input.arguments" }
                " is that tool's argument object. Omit "
                code { class: "{INLINE_CODE}", "project" }
                "; the run injects the execution project. Tool names are in the Tools reference."
            }
            p { class: "{BODY_TEXT}",
                code { class: "{INLINE_CODE}", "foreach" }
                " has exactly one "
                code { class: "{INLINE_CODE}", "next_tasks" }
                " entry, and that body must be one chain with a single leaf. A later branch is rejected. Nested "
                code { class: "{INLINE_CODE}", "foreach" }
                " is allowed when each body is still one chain. "
                code { class: "{INLINE_CODE}", "repair_loop" }
                " is the other handler with a chain rule: one leaf body, then one verifier. Other tasks may branch."
            }
            h3 { class: "mt-6 {SUBSECTION_TITLE}", "Command selection" }
            p { class: "{BODY_TEXT}",
                "Generic library workflows should not hardcode "
                code { class: "{INLINE_CODE}", "cursor-agent" }
                " on every agent task. Use an "
                strong { "unbound" }
                " "
                code { class: "{INLINE_CODE}", "command" }
                " task (no "
                code { class: "{INLINE_CODE}", "command_id" }
                " in "
                code { class: "{INLINE_CODE}", "function.input" }
                ") and let the hub resolve a command at launch. A "
                strong { "fixed" }
                " task sets a non-empty "
                code { class: "{INLINE_CODE}", "command_id" }
                " and always runs that command, even when the operator picks another command for the run."
            }
            p { class: "{BODY_TEXT}",
                "For an unbound task, resolution order is: launch selection, then workflow "
                code { class: "{INLINE_CODE}", "default_command_id" }
                ", then the project "
                code { class: "{INLINE_CODE}", "default_command_id" }
                " in hub "
                code { class: "{INLINE_CODE}", "config.json" }
                ". Launch selection comes from the execute request "
                code { class: "{INLINE_CODE}", "command_id" }
                ", an optional "
                code { class: "{INLINE_CODE}", "command_id" }
                " on a workflow schedule, or an optional "
                code { class: "{INLINE_CODE}", "command_id" }
                " on a cycle stage entry. Explicit task "
                code { class: "{INLINE_CODE}", "command_id" }
                " values are not overridden by launch selection."
            }
            h4 { class: "mt-4 text-xs font-semibold text-zinc-800 dark:text-zinc-100",
                "Unbound task (inherits at launch)"
            }
            CodeExample { text: UNBOUND_TASK }
            h4 { class: "mt-4 text-xs font-semibold text-zinc-800 dark:text-zinc-100",
                "Fixed task (always this command)"
            }
            CodeExample { text: FIXED_TASK }
            h3 { class: "mt-6 {SUBSECTION_TITLE}", "Template references" }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "{{prompt.<id>}}" }
                    ": inject stored prompt text"
                }
                li {
                    code { class: "{INLINE_CODE}", "{{runtime.<key>}}" }
                    ": value from "
                    code { class: "{INLINE_CODE}", "runtime_inputs" }
                }
                li {
                    code { class: "{INLINE_CODE}", "{{userinput.<task_id>.value}}" }
                    ": earlier "
                    code { class: "{INLINE_CODE}", "user_input" }
                    " task"
                }
                li {
                    code { class: "{INLINE_CODE}", "{{task.<task_id>.output...}}" }
                    ": captured output from an earlier task"
                }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Repository guidance workflow" }
            p { class: "{BODY_TEXT}",
                "The packaged "
                code { class: "{INLINE_CODE}", "system-update-repository-agents" }
                " workflow accepts an absolute "
                code { class: "{INLINE_CODE}", "repository_root" }
                ", inspects the repository without editing it, and pauses for answers to questions based on that inspection. After the answer, it rechecks the source files and updates only the root "
                code { class: "{INLINE_CODE}", "AGENTS.md" }
                ". It declares a "
                code { class: "{INLINE_CODE}", "command_id" }
                " runtime input so the operator or project default selects the inspecting agent at launch."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Reference example" }
            p { class: "{BODY_TEXT}",
                "From "
                code { class: "{INLINE_CODE}", "packages/system-implement-story/1.0.0/definition.json" }
                " (inner "
                code { class: "{INLINE_CODE}", "content" }
                " pretty-printed; agent task is unbound):"
            }
            CodeExample { text: REFERENCE_EXAMPLE }
            p { class: "{BODY_TEXT}",
                "On disk "
                code { class: "{INLINE_CODE}", "content" }
                " is a nested JSON object, not a string."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Install path" }
            CodeExample { text: INSTALL }
            SchemaReferenceFooter {
                catalog_schema_label: "see.library/v1",
                schema_route: Route::SchemaWorkflow {},
                schema_link_label: "Workflow schema",
                extra_schema_links: None,
            }
        }
    }
}
