use dioxus::prelude::*;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};

#[component]
pub fn LibrarySchemaAppSettings() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "App settings schema",
            meta_description: "User-level ~/.s_e_e/settings.json. Theme, auth, retention, and command allowlist. Validated on save.",
            rail_title: "Validators run on save.",
            rail_description: "~/.s_e_e/settings.json. theme is light, dark, or system. Extra keys are kept.",
            p { class: "{PAGE_LEDE}",
                "User-level "
                code { class: "{INLINE_CODE}", "~/.s_e_e/settings.json" }
                "."
            }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "theme" }, " - "
                    code { class: "{INLINE_CODE}", "light" }, ", "
                    code { class: "{INLINE_CODE}", "dark" }, ", or "
                    code { class: "{INLINE_CODE}", "system" }, "."
                }
                li {
                    code { class: "{INLINE_CODE}", "notifications" }, ", "
                    code { class: "{INLINE_CODE}", "auth_enabled" }, ", "
                    code { class: "{INLINE_CODE}", "mcp_enabled" }, ", "
                    code { class: "{INLINE_CODE}", "llm_content_capture_enabled" }, " - booleans."
                }
                li { code { class: "{INLINE_CODE}", "default_workflow" }, " - string or null." }
                li { code { class: "{INLINE_CODE}", "editor_auto_save" }, " - boolean, default false. Reserved for the editor autosave rollout; the switch is not exposed yet. Legacy auto_save is ignored. Settings PUTs omitting editor_auto_save preserve its saved value." }
                li {
                    code { class: "{INLINE_CODE}", "server_url" }, ", "
                    code { class: "{INLINE_CODE}", "api_port" }, ", "
                    code { class: "{INLINE_CODE}", "cors_origins" }, " - strings."
                }
                li {
                    code { class: "{INLINE_CODE}", "audit_retention_hours" }, ", "
                    code { class: "{INLINE_CODE}", "log_retention_hours" }, ", "
                    code { class: "{INLINE_CODE}", "session_ttl_hours" }, ", "
                    code { class: "{INLINE_CODE}", "command_timeout_secs" }, " - integer or null."
                }
                li { code { class: "{INLINE_CODE}", "schedule_retention" }, " - object." }
                li { code { class: "{INLINE_CODE}", "log_stream_retention" }, " - per-stream retention days. Null keeps a stream indefinitely." }
                li { code { class: "{INLINE_CODE}", "diagnostic_logging" }, " - diagnostic JSON capture policy, defaulting to Info with no category overrides." }
                li { code { class: "{INLINE_CODE}", "command_binary_allowlist" }, " - string array." }
                li { code { class: "{INLINE_CODE}", "command_secrets" }, " - string map." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Diagnostic logging" }
            p {
                "Settings > Retention > Logging edits "
                code { class: "{INLINE_CODE}", "diagnostic_logging" }
                ". Stored default_level and overrides values are trace, debug, info, warn, error or off. Overrides accept server, trace, timing and frontend. Each threshold captures its level and higher. Off suppresses every diagnostic level in that category. Inherit removes the category key."
            }
            pre { code { {r#"{
  "diagnostic_logging": {
    "default_level": "info",
    "overrides": {"server": "debug", "trace": "off"}
  }
}"#} } }
            p {
                "This fragment captures server Debug diagnostics, suppresses trace diagnostics and leaves timing/frontend at Info. Missing policy defaults to Info with no overrides. An API update omitting the policy preserves its previous value. Supplying it replaces the whole policy. An empty object resets it to defaults, and empty overrides restores category inheritance."
            }
            p {
                "Saved policy applies immediately to subsequent server writes. Workers keep their launch snapshot, and newly launched workers receive updates. RUST_LOG independently controls ordinary tracing. Viewer filters select stored entries. Verbosity changes leave historical files intact, while retention determines file expiry."
            }
            p {
                "Audit, process lifecycle and raw output, workflow history and LLM accounting remain captured at diagnostic Off under their independent capture and retention policies. Raw stdout and stderr use Trace with severity_origin=raw_capture and separate source labels. Successful polling/search diagnostics are Debug, meaningful changes are Info, recoverable failures are Warn and terminal failures are Error."
            }
            p {
                "Stream retention defaults in days are audit_days=365, llm_days=365, server_days=30, trace_days=30 (also timing), workflow_days=30, frontend_days=30, process_days=14 and entity_change_meta_days=90. A null value retains that stream indefinitely. Stream values take precedence over log_retention_hours, whose fallback is 48 hours. The legacy audit_retention_hours field does not control stream expiry."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            p { "Machine definition storage under ~/.s_e_e/definitions/ is separate from settings and secrets here. Client grants default machine definition read/edit off; edit implies read. Project access is separate. Back up machine definitions separately from project Git/backups; runs, logs, schedules, cycles and history remain per project." }
            SchemaBlockByName { filename: "app-settings.schema.json" }
        }
    }
}
