use dioxus::prelude::*;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};

#[component]
pub fn LibrarySchemaAppSettings() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "App settings schema",
            meta_description: "User-level ~/.s_e_e/settings.json. Theme, auth, retention, and command allowlist. Validated on save.",
            rail_number: "10",
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
                    code { class: "{INLINE_CODE}", "auto_save" }, ", "
                    code { class: "{INLINE_CODE}", "notifications" }, ", "
                    code { class: "{INLINE_CODE}", "auth_enabled" }, ", "
                    code { class: "{INLINE_CODE}", "mcp_enabled" }, ", "
                    code { class: "{INLINE_CODE}", "llm_content_capture_enabled" }, " - booleans."
                }
                li { code { class: "{INLINE_CODE}", "default_workflow" }, " - string or null." }
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
                li { code { class: "{INLINE_CODE}", "command_binary_allowlist" }, " - string array." }
                li { code { class: "{INLINE_CODE}", "command_secrets" }, " - string map." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            SchemaBlockByName { filename: "app-settings.schema.json" }
        }
    }
}
