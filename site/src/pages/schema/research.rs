use dioxus::prelude::*;

use crate::library_href;
use crate::pages::authoring_common::{INLINE_CODE, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};

#[component]
pub fn LibrarySchemaResearch() -> Element {
    rsx! {
        document::Meta {
            name: "description",
            content: "Wire schemas for S.E.E. structured Knowledge Research HTTP and MCP operations.",
        }
        SchemaGuideFrame {
            document_title: "Knowledge Research schema",
            meta_description: "Wire schemas for S.E.E. structured Knowledge Research HTTP and MCP operations.",
            rail_title: "Research wire contract.",
            rail_description: "Four profile descriptors, eleven shared operations, versioned editions.",
            p { class: "{PAGE_LEDE}",
                "This contract describes the implemented Research wire models. The same domain service is exposed through HTTP and eleven named MCP operations. The formal JSON document is available at "
                a { href: library_href("/schema/knowledge-research.schema.json"), class: "font-semibold text-sky-700 dark:text-sky-300", "knowledge-research.schema.json" }
                "."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Profile authority and versions" }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                "Call " code { class: "{INLINE_CODE}", "knowledge_research_profile_get" }
                " for a kind and optional version. Current Research, Report, Analysis, and Guide profiles are version 1. The returned descriptor supplies required fields, required and optional section roles, examples, and validation rules. This runtime descriptor is the authority for profile membership and requirements."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Wire models" }
            ul { class: "mt-2 list-disc space-y-1 pl-5 text-sm text-zinc-600 dark:text-zinc-400",
                li { code { "Document" }, ", ", code { "Page" }, ", ", code { "Section" }, ", bibliography ", code { "Source" }, ", typed action items, report context, validation issues, working copies, immutable editions, manifests, and page reads." }
                li { "Create accepts a complete Document-shaped value; document identity and timestamps are part of that wire model." }
                li { "Document put changes the brief, sources, and exact existing page-key order. Page put replaces one structured page. All writes use the current aggregate conflict token." }
                li { "Read view is default, Draft, or a selected edition. Default selects the latest edition and falls back to Draft only when no edition exists." }
                li { "Delete requires exact-id confirmation. Kind changes require explicit confirmation and validation against the selected profile." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Errors and publication" }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                "Collection selection uses list_filter_get for knowledge_research. Combine typed filters with existing kind, tag, query and view selectors using AND. The response retains documents, next_cursor and limit. A continuation cursor is bound to the normalized selection and view. Restart without the cursor after changing either. MCP entity-resource query templates preserve this cursor independently of protocol catalog pagination. Shared predicate shapes are available in "
                a { href: library_href("/schema/collection-filter.schema.json"), class: "font-semibold text-sky-700 dark:text-sky-300", "collection-filter.schema.json" }
                "."
            }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                "HTTP errors contain an error message and stable code, with optional field path, remedy, and validation issues. Validation issues include code, severity, field path, optional page and section keys, and remedy. Stale tokens return a conflict and require rereading and reconciling the current aggregate. Publish revalidates within the mutation and returns an immutable edition plus the latest aggregate token."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Schema JSON" }
            SchemaBlockByName { filename: "knowledge-research.schema.json" }
        }
    }
}
