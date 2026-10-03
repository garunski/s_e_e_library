use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{CodeExample, INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

const UNBOUND: &str = r#"// Unbound (inherits)
"function": { "name": "command", "input": { "prompt": "{{prompt.system-implement-story}}" } }

// Fixed (always audit-system-agent)
"function": {
  "name": "command",
  "input": { "command_id": "audit-system-agent", "prompt": "{{prompt.system-audit}}" }
}"#;

const TEMPLATE_EXPANSION: &str = r#"{{prompt.ID}}              injects a stored prompt; the id must exist in the hub
{{runtime.KEY}}            run-start value; declared in runtime_inputs, no dots
{{userinput.TASK_ID.value}} value from an earlier user_input task
{{task.TASK_ID.output}}    captured output from an earlier task with capture_output: true
                           (cli_command output is { exit_code, stdout, stderr })"#;

#[component]
pub fn LibrarySchemaWorkflow() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Workflow schema",
            meta_description: "Category workflow. Payload file definition.json. Installs to .s_e_e/workflows/definitions/{{slug}}.json.",
            rail_number: "01",
            rail_title: "Installs under workflow definitions.",
            rail_description: "Destination is .s_e_e/workflows/definitions/{{slug}}.json.",
            p { class: "{PAGE_LEDE}",
                "Category "
                code { class: "{INLINE_CODE}", "workflow" }
                ". Payload file "
                code { class: "{INLINE_CODE}", "definition.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/workflows/definitions/{{slug}}.json" }
                "."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Envelope" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "An outer envelope wraps the engine ", code { class: "{INLINE_CODE}", "content" }, ":" }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "id" }, " - ", code { class: "{INLINE_CODE}", "system-<name>" }, " (defaults) or ", code { class: "{INLINE_CODE}", "user-<name>" }, ". Required." }
                li { code { class: "{INLINE_CODE}", "name" }, ", ", code { class: "{INLINE_CODE}", "description" }, ", ", code { class: "{INLINE_CODE}", "version" }, " - strings." }
                li {
                    code { class: "{INLINE_CODE}", "content" }, " - the engine workflow object:"
                    ul { class: "{LIST}",
                        li { code { class: "{INLINE_CODE}", "id" }, " - required, non-empty." }
                        li { code { class: "{INLINE_CODE}", "name" }, " - required." }
                        li { code { class: "{INLINE_CODE}", "tasks[]" }, " - required, non-empty." }
                        li { code { class: "{INLINE_CODE}", "runtime_inputs[]" }, " - optional; each ", code { class: "{INLINE_CODE}", "{{ key, input_type, required, label }}" }, "; ", code { class: "{INLINE_CODE}", "key" }, " must not contain dots." }
                        li { code { class: "{INLINE_CODE}", "default_command_id" }, " - optional installed command id for ", code { class: "{INLINE_CODE}", "command" }, " tasks that omit ", code { class: "{INLINE_CODE}", "function.input.command_id" }, "." }
                        li { code { class: "{INLINE_CODE}", "spoke_root" }, ", ", code { class: "{INLINE_CODE}", "hub_root" }, ", ", code { class: "{INLINE_CODE}", "stories_root" }, ", ", code { class: "{INLINE_CODE}", "knowledge_root" }, " - optional root paths." }
                    }
                }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Tasks" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Each task has ", code { class: "{INLINE_CODE}", "id" }, ", ", code { class: "{INLINE_CODE}", "name" }, ", ", code { class: "{INLINE_CODE}", "function" }, " (one handler), and optional ", code { class: "{INLINE_CODE}", "next_tasks[]" }, ", ", code { class: "{INLINE_CODE}", "capture_output" }, ", ", code { class: "{INLINE_CODE}", "wait_for" }, ", ", code { class: "{INLINE_CODE}", "continue_on_failure" }, ", ", code { class: "{INLINE_CODE}", "run_when" }, "." }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "A payload containing ", code { class: "{INLINE_CODE}", "tasks" }, " is treated as a workflow (not a prompt)." }
            h2 { class: "mt-8 {SECTION_TITLE}", "Command tasks" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Handler ", code { class: "{INLINE_CODE}", "command" }, " runs an installed definition from ", code { class: "{INLINE_CODE}", ".s_e_e/commands/" }, ". Omit ", code { class: "{INLINE_CODE}", "command_id" }, " for an unbound task; set a non-empty ", code { class: "{INLINE_CODE}", "command_id" }, " to fix the task to one command." }
            CodeExample { text: UNBOUND }
            h2 { class: "mt-8 {SECTION_TITLE}", "Template expansion" }
            CodeExample { text: TEMPLATE_EXPANSION }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Library package envelope (id, name, content):" }
            SchemaBlockByName { filename: "workflow.schema.json" }
            h2 { class: "mt-8 {SECTION_TITLE}", "Stored definition envelope" }
            SchemaBlockByName { filename: "workflow-definition.schema.json" }
            h2 { class: "mt-8 {SECTION_TITLE}", "Engine content schema" }
            SchemaBlockByName { filename: "workflow-engine.schema.json" }
            p { class: "mt-6 text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                Link { to: Route::AuthoringWorkflows {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "workflow authoring guide" }
                " and the "
                Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                "."
            }
        }
    }
}
