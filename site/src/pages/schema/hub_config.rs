use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaHubConfig() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Hub config schema",
            meta_description: "Hub data-root config.json. A JSON object with at least one property. Validated on save.",
            rail_title: "Validators run on save.",
            rail_description: ".s_e_e/config.json. additionalProperties is allowed so project-specific keys persist.",
            p { class: "{PAGE_LEDE}",
                "Hub data-root "
                code { class: "{INLINE_CODE}", "config.json" }
                ". Must be a non-empty JSON object."
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "project" }, " - object; typically includes ", code { class: "{INLINE_CODE}", "id" }, "." }
                li { code { class: "{INLINE_CODE}", "overrides" }, " - object." }
                li {
                    code { class: "{INLINE_CODE}", "library" }, " - catalog sources and collections. See "
                    Link { to: Route::SchemaCatalog {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                    "."
                }
                li {
                    code { class: "{INLINE_CODE}", "stories" }, " - see "
                    Link { to: Route::SchemaStoriesConfig {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "stories config" }
                    "."
                }
                li {
                    code { class: "{INLINE_CODE}", "llm.routing" }, " - see "
                    Link { to: Route::SchemaRoutingRules {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "routing rules" }
                    "."
                }
                li {
                    code { class: "{INLINE_CODE}", "orchestrator" }, " - see "
                    Link { to: Route::SchemaOrchestratorPolicy {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "orchestrator policy" }
                    "."
                }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Unknown top-level keys are kept." }
            h2 { class: "mt-8 {SECTION_TITLE}", "Experimental settings" }
            p { class: "text-sm", { "Project settings > Experimental manages project.experimental_features. The six boolean keys are orchestrator, orchestrator_cycle, knowledge_cycle, planning_cycle, knowledge_research and global_chat. Existing and new projects resolve missing objects or keys to false without an eager migration write. Unknown experiment keys and non-boolean values are rejected. Only orchestrator_cycle requires both its configured value and orchestrator. Other switches are independent, including global_chat." } }
            p { class: "text-sm", { "GET /api/projects/{project}/experimental-features returns configured, effective and conflict_token. PATCH on the same path accepts {\"changes\":{\"global_chat\":true},\"conflict_token\":\"latest-read-token\"}. Send only changed boolean keys. Stale tokens return HTTP 409 without mutation. A successful response is the canonical snapshot for the next edit. The stored object contains configured booleans only. Effective values and the token are response fields." } }
            p { class: "text-sm", { "Disabling preserves data, schedules and saved run controls and does not cancel admitted work. Fresh admission stops. Subsequent feature-owned HTTP and MCP calls, including calls from existing runs, return EXPERIMENTAL_FEATURE_DISABLED. Generic execution, process, log and stop controls remain available. Enabling may resume scheduling on the next normal tick. Manual stories, milestones, shared objectives, knowledge docs and decisions, reusable workflows and Library package storage remain available. Providers, models, routing, spend, /v1/chat/completions and workflow-space defaults remain available." } }
            p { class: "text-sm", { "MCP tool discovery is the union across authorized projects, restricted to the bound project when present. A mixed-project client can see a tool enabled elsewhere. Invocation checks the target project afresh. Exact-project resources and qualified Research prompts follow target availability. Clients supporting list-change notifications refresh discovery after changes. Cached tool lists do not authorize calls." } }
            h2 { class: "mt-8 {SECTION_TITLE}", "Conditional project settings writes" }
            p { class: "text-sm", { "Project settings writes accept an optional expected_settings object containing the last-read values for the edited section. The expected_settings fields on PUT /api/projects/{project} are name, hub_path, and command_timeout_secs (null means no override). On PUT /api/projects/{project}/command-defaults they are default_command_id (null means none) and default_workflow_by_command. The patch still uses clear_command_timeout:true or clear_default_command:true to remove the respective override. Comparison and patch run together under the server write coordinator; stale values return HTTP 409 without mutation. Omission preserves existing unconditional callers. For example, send {\"name\":\"New name\",\"expected_settings\":{\"name\":\"Old name\"}}. Use current values from the latest project read or successful write; hub_path corresponds to the project response path field. Timeout updates persist in hub config and preserve other sections." } }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "hub-config.schema.json" }
        }
    }
}
