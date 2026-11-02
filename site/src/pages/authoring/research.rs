use dioxus::prelude::*;

use crate::pages::authoring_common::{AuthoringGuideFrame, CodeExample, INLINE_CODE, PAGE_LEDE, SECTION_TITLE};

const EXAMPLES: &str = include_str!("../../../content/authoring/research_examples.json");

#[component]
pub fn LibraryAuthoringResearch() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Knowledge Research",
            meta_description: "Author structured S.E.E. Knowledge Research documents, validate drafts, and publish immutable editions.",
            rail_number: "07",
            rail_title: "One profile source.",
            rail_description: "Read the kind and version descriptor before authoring.",
            p { class: "{PAGE_LEDE}",
                "Research is the Knowledge area for research, reports, analysis, and guides. Ordinary Docs remain for general project documentation. The Rust profile descriptor returned by "
                code { class: "{INLINE_CODE}", "knowledge_research_profile_get" }
                " is the runtime authority for field and role requirements. The four descriptors currently use profile version "
                code { class: "{INLINE_CODE}", "1" }
                ". This page and its examples follow those descriptors; do not treat copied role lists as an independent profile registry."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Authoring sequence" }
            ol { class: "mt-3 list-decimal space-y-2 pl-5 text-sm text-zinc-600 dark:text-zinc-400",
                li { "Resolve the project grant, discover the Research tools/resources, and list existing records before creating a duplicate." }
                li { "Choose one kind, read its current profile, and establish audience, purpose, scope, key message, evidence basis, and report period or as-of date where applicable." }
                li { "Plan ordered pages and stable lowercase keys. Put each required machine role in a structured section; headings may be written for readers and roles may span pages." }
                li { "Record bibliography sources with honest access status, inspected scope, known date/version, and limitations. Attach source ids only to the sections they support." }
                li { "Create or update a Draft with structured pages and sections. Read back the manifest/page after each write and use the latest aggregate token." }
                li { "Validate, repair the returned field paths, and review evidence strength and meaning separately from structural checks." }
                li { "Publish only when the task authorizes publication. Read the returned edition and selected pages by edition id; otherwise leave the record Draft." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Eleven operations" }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                "HTTP base: " code { class: "{INLINE_CODE}", "/api/projects/{{project}}/knowledge/research" }
                ". MCP tools use the same operation names and project grant. Reads default to the latest published edition, falling back to a Draft only when no edition exists; explicitly request "
                code { class: "{INLINE_CODE}", "view=draft" }
                " or an edition id to select another view."
            }
            ul { class: "mt-3 list-disc space-y-1 pl-5 text-sm text-zinc-600 dark:text-zinc-400",
                li { code { "knowledge_research_profile_get" }, " - read the versioned kind descriptor." }
                li { code { "knowledge_research_list" }, " - bounded summary list with kind/tag/query/view filters and cursor." }
                li { code { "knowledge_research_get" }, " - manifest and selected Draft/edition metadata; page bodies are read separately." }
                li { code { "knowledge_research_create" }, " and ", code { "knowledge_research_put" }, " - create a Draft or update its brief and bibliography. An optional complete pages array replaces the structure atomically; page_order must list its keys exactly once. Without pages, page_order lists existing page keys." }
                li { code { "knowledge_research_delete" }, " - exact-id deletion requires current aggregate token and confirm=true." }
                li { code { "knowledge_research_page_get" }, " and ", code { "knowledge_research_page_put" }, " - read or replace a structured page." }
                li { code { "knowledge_research_page_delete" }, " - remove a page with the current aggregate token." }
                li { code { "knowledge_research_validate" }, " - inspect full Draft diagnostics without mutation." }
                li { code { "knowledge_research_publish" }, " - revalidate and atomically create an immutable edition." }
            }
            p { class: "mt-3 text-sm text-zinc-600 dark:text-zinc-400",
                "Every write uses the latest aggregate conflict token. Kind changes require "
                code { class: "{INLINE_CODE}", "confirm_kind_change=true" }
                " and complete validation against the new profile. Structural and validation errors carry codes and field paths; repair the data, reread, and validate again. Publication creates a local edition, not an external website deployment or human approval."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Errors and repair" }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                "A validation issue reports " code { class: "{INLINE_CODE}", "code" }, ", "
                code { class: "{INLINE_CODE}", "severity" }, ", "
                code { class: "{INLINE_CODE}", "field_path" }, ", optional page/section keys, and a concrete "
                code { class: "{INLINE_CODE}", "remedy" }
                ". Typical codes include " code { class: "{INLINE_CODE}", "research.required_role_missing" }, ", "
                code { class: "{INLINE_CODE}", "research.source_reference_unresolved" }, ", "
                code { class: "{INLINE_CODE}", "research.source_backed_without_sources" }, ", "
                code { class: "{INLINE_CODE}", "research.action_owner_invalid" }, ", and "
                code { class: "{INLINE_CODE}", "research.field_missing" }
                ". A stale token requires a fresh read and reconciliation; do not replay a write against changed content."
            }
            h2 { id: "examples", class: "mt-8 {SECTION_TITLE}", "Complete examples and repaired failures" }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                "The JSON fixture contains a complete, profile-version-1 Draft for each kind, a rejected/corrected field example for each kind, and Draft-to-edition readback values. All records and source locators are explicitly illustrative; replace them with actual work and evidence."
            }
            CodeExample { text: EXAMPLES }
            h2 { class: "mt-8 {SECTION_TITLE}", "Producer instructions and related help" }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                "For assistant-led writing, discover "
                code { class: "{INLINE_CODE}", "mcp-research-authoring" }
                " (MCP name " code { class: "{INLINE_CODE}", "research-authoring" }
                ") through " code { class: "{INLINE_CODE}", "prompts/list" }, ", "
                code { class: "{INLINE_CODE}", "prompts/get" }, " or SEE "
                code { class: "{INLINE_CODE}", "prompt_get" }
                ". The prompt routes producers through profile lookup, source honesty, Draft readback, diagnostic repair, task-scoped publishing, and edition readback."
            }
            p { class: "mt-3 text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                crate::shell::AppLink { class: "font-semibold text-sky-700 dark:text-sky-300", path: "/schema/knowledge-research/", "Research wire schema" }
                ". In-app Research help covers the same profile-led workflow and points assistant authors to the same producer prompt."
            }
            p { class: "mt-4 text-xs text-zinc-500 dark:text-zinc-400",
                "Local repository validation and Pages staging prove only local source/artifact state. They do not prove remote hosting, installation into every project, or human content review."
            }
        }
    }
}

