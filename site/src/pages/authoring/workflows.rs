use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, AuthoringReference, CodeExample, BODY_TEXT, DD, DL, DT, INLINE_CODE, LIST,
    SUBSECTION_TITLE,
};
use crate::routes::Route;

const UNBOUND_TASK: &str = include_str!("../../../content/authoring/workflows_unbound_task.json");
const FIXED_TASK: &str = include_str!("../../../content/authoring/workflows_fixed_task.json");
const REFERENCE_EXAMPLE: &str =
    include_str!("../../../content/authoring/workflows_reference_example.json");
const INSTALL: &str = include_str!("../../../content/authoring/workflows_install.json");
const PROJECT_PLACEHOLDER: &str = "{{project.id}}";
const RUNTIME_HELP: &str = include_str!("../../../content/authoring/workflow_runtime.md");
const RUNNING_EXAMPLE: &str =
    include_str!("../../../content/authoring/workflow_execution_running.json");
const WORKER_FAILURE_EXAMPLE: &str =
    include_str!("../../../content/authoring/workflow_execution_worker_failed.json");
const LIMIT_ERROR_EXAMPLE: &str =
    include_str!("../../../content/authoring/workflow_execution_limit_error.json");
const FILTER_HELP: &str = include_str!("../../../content/authoring/collection_filters.md");
const FILTER_STORIES: &str =
    include_str!("../../../content/authoring/collection_filter_stories.json");
const FILTER_MILESTONES: &str =
    include_str!("../../../content/authoring/collection_filter_milestones.json");
const FILTER_DOCS: &str = include_str!("../../../content/authoring/collection_filter_docs.json");

#[component]
pub fn LibraryAuthoringWorkflows() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Workflows",
            meta_description: "A workflow package defines tasks the S.E.E. engine runs against a hub or spoke.",
            section {
                h2 { "Native collection selection" }
                s_e_e_gui_kit::components::MarkdownHtml { src: FILTER_HELP.to_string() }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Unfinished milestone stories" }
                p { class: "{BODY_TEXT}", "Arguments for story_list. Substitute your project and milestone IDs and use its configured status choices." }
                CodeExample { text: FILTER_STORIES }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Milestone progress" }
                p { class: "{BODY_TEXT}", "Arguments for milestone_list. Derived progress describes all member stories independently of list selection and declared milestone status." }
                CodeExample { text: FILTER_MILESTONES }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Document type and tags" }
                p { class: "{BODY_TEXT}", "Arguments for knowledge_doc_list. Discover knowledge_docs for supported document type choices." }
                CodeExample { text: FILTER_DOCS }
            }
            section {
                h2 { "Workflow definition" }
                p { class: "{BODY_TEXT}",
                    "Use the bundled workflow skill at "
                    code { class: "{INLINE_CODE}", "packages/workflow/1.0.0/SKILL.md" }
                    " when writing a workflow."
                }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Envelope (definition.json)" }
                dl { class: "{DL}",
                    div {
                        dt { class: "{DT}",
                            code { "id" }
                        }
                        dd { class: "{DD}",
                            "Outer id, usually "
                            code { class: "{INLINE_CODE}", "system-<name>" }
                            "."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "name" }
                            ", "
                            code { "description" }
                        }
                        dd { class: "{DD}", "Hub display metadata." }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "content" }
                        }
                        dd { class: "{DD}", "Engine workflow object." }
                    }
                }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Workflow content" }
                dl { class: "{DL}",
                    div {
                        dt { class: "{DT}",
                            code { "runtime_inputs" }
                        }
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
                    }
                    div {
                        dt { class: "{DT}",
                            code { "default_command_id" }
                        }
                        dd { class: "{DD}",
                            "Optional installed command id for "
                            code { class: "{INLINE_CODE}", "command" }
                            " tasks that omit "
                            code { class: "{INLINE_CODE}", "command_id" }
                            ". Empty strings are invalid."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "tasks" }
                        }
                        dd { class: "{DD}", "Ordered graph of engine tasks." }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "spoke_root" }
                        }
                        dd { class: "{DD}",
                            "Optional. Omit it in Library packages: the run then uses the project's code folder for story implementation and code knowledge workflows, and the project home folder for every other workflow. To let the user pick the folder, set it to an exact "
                            code { class: "{INLINE_CODE}", "{{runtime.KEY}}" }
                            " reference to a string runtime input; the run resolves and validates the absolute path before executing tasks and again when resuming after user input. Catalog validation rejects a package that sets a literal path."
                        }
                    }
                }
            }
            section {
                h2 { "Tasks" }
                h3 { class: SUBSECTION_TITLE, "Task types" }
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
                    " is the precise named tool id. "
                    code { class: "{INLINE_CODE}", "item_call" }
                    " remains for a call that has no named tool. "
                    code { class: "{INLINE_CODE}", "input.arguments" }
                    " is that tool's argument object and includes an explicit "
                    code { class: "{INLINE_CODE}", "project" }
                    " such as "
                    code { class: "{INLINE_CODE}", "{PROJECT_PLACEHOLDER}" }
                    ". Tool names are in the Tools reference."
                }
                p { class: "{BODY_TEXT}",
                    code { class: "{INLINE_CODE}", "foreach" }
                    " has exactly one "
                    code { class: "{INLINE_CODE}", "next_tasks" }
                    " entry, and that body must be one chain with a single leaf. A later branch is rejected. Nested "
                    code { class: "{INLINE_CODE}", "foreach" }
                    " is allowed when each body is still one chain. "
                    code { class: "{INLINE_CODE}", "repair_loop" }
                    " has one leaf body, then one verifier, and can be a foreach body. Each item keeps its own runtime values, attempt outputs and verifier feedback. The next item waits for verification to pass. Its retry budget starts at the first body task, not at workflow launch, and the start time survives checkpoint resume. Task timeouts remain separate. Other tasks may branch."
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
                h4 { class: "mt-4 {SUBSECTION_TITLE}",
                    "Unbound task (inherits at launch)"
                }
                CodeExample { text: UNBOUND_TASK }
                h4 { class: "mt-4 {SUBSECTION_TITLE}",
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
            }
            section {
                h2 { "Workflow examples" }
                h3 { class: SUBSECTION_TITLE, "Repository guidance workflow" }
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
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Story implementation workflow" }
                p { class: "{BODY_TEXT}",
                    "From "
                    code { class: "{INLINE_CODE}",
                        "packages/system-implement-story/1.0.0/definition.json"
                    }
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
            }
            section {
                h2 { "Installation" }
                p { class: BODY_TEXT,
                    "The workflow payload is "
                    code { class: INLINE_CODE, "definition.json" }
                    ". The catalog installs it at "
                    code { class: INLINE_CODE, ".s_e_e/workflows/definitions/{{slug}}.json" }
                    "."
                }
                CodeExample { text: INSTALL }
            }
            section {
                h2 {
                    id: "execution-runtime-and-controls",
                    class: "scroll-mt-24",
                    "Execution"
                }
                s_e_e_gui_kit::components::MarkdownHtml { src: RUNTIME_HELP.to_string() }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Running execution example" }
                CodeExample { text: RUNNING_EXAMPLE }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Worker failure example" }
                CodeExample { text: WORKER_FAILURE_EXAMPLE }
                h3 { class: "mt-6 {SUBSECTION_TITLE}", "Resource rejection example" }
                CodeExample { text: LIMIT_ERROR_EXAMPLE }
            }
            AuthoringReference {
                schema_route: Route::SchemaWorkflow {},
                schema_link_label: "Workflow schema",
                extra_schema_links: None,
            }
        }
    }
}
