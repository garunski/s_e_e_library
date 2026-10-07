use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, CodeExample, SchemaReferenceFooter, BODY_TEXT, DD, DL, DT, INLINE_CODE,
    LIST, PAGE_LEDE, SECTION_TITLE, SUBSECTION_TITLE,
};
use crate::routes::Route;

const ORCHESTRATOR_EXAMPLE: &str =
    include_str!("../../../content/authoring/cycles_orchestrator_example.json");
const KNOWLEDGE_EXAMPLE: &str =
    include_str!("../../../content/authoring/cycles_knowledge_example.json");
const INSTALL_ORCHESTRATOR: &str =
    include_str!("../../../content/authoring/cycles_install_orchestrator.json");
const INSTALL_KNOWLEDGE: &str =
    include_str!("../../../content/authoring/cycles_install_knowledge.json");

const ORCHESTRATOR_STORES: [(&str, &str); 7] = [
    (
        "world",
        "Live stories, workflows, and spoke heads (source).",
    ),
    (
        "objectives",
        "Authored objectives with measurable criteria.",
    ),
    ("deliberations", "Shaping steps per objective."),
    ("ranking", "Candidate order and weights for the next tick."),
    ("proposals", "Gated proposals; at most one per tick."),
    ("executions", "Runs with frozen workflow snapshots."),
    ("learning", "Judged outcomes per finished run."),
];

const KNOWLEDGE_STORES: [(&str, &str); 8] = [
    ("spokes", "Current git on each spoke (source)."),
    ("changes", "Recorded git ranges."),
    ("components", "Notes for named pieces."),
    ("impacts", "Consumer compatibility (gated)."),
    ("audits", "Notes vs live code (gated)."),
    ("docs", "Published topic pages."),
    ("decisions", "ADRs (authored)."),
    ("links", "Broken citations (gated)."),
];

const BLOCKING_CHECKS: [(&str, &str); 6] = [
    (
        "unknown_store",
        "A store key is not declared for this host. Install rejects unknown keys; activation checks repeat the rule.",
    ),
    (
        "store_has_two_writers",
        "Two stages write the same store, so provenance is ambiguous.",
    ),
    (
        "stage_has_no_reads",
        "A stage lists no reads, so it cannot be replayed or judged.",
    ),
    (
        "workflow_not_installed",
        "A stage names a workflow definition id with no file on the hub.",
    ),
    ("gate_missing", "The orchestrator gate stage was removed."),
    (
        "gate_rewired",
        "The orchestrator gate stage no longer matches the compiled reads, writes, workflow, or trigger.",
    ),
];

#[component]
pub fn LibraryAuthoringCycles() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Cycles",
            meta_description: "A cycle package describes stages, stores, and triggers for an orchestrator or knowledge host.",
            rail_number: "06",
            rail_title: "Installs under .s_e_e/cycles.",
            rail_description: "Payload file cycle.json; to path must match the document id.",
            p { class: "{PAGE_LEDE}",
                "A "
                strong { "cycle" }
                " package describes how workflows connect stores on a host. Payload file: "
                code { class: "{INLINE_CODE}", "cycle.json" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".s_e_e/cycles/{{id}}.json" }
                " where "
                code { class: "{INLINE_CODE}", "id" }
                " matches the document "
                code { class: "{INLINE_CODE}", "id" }
                " field."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Stages and stores" }
            p { class: "{BODY_TEXT}",
                "Each "
                strong { "stage" }
                " runs one workflow definition. It "
                code { class: "{INLINE_CODE}", "reads" }
                " zero or more store keys and "
                code { class: "{INLINE_CODE}", "writes" }
                " exactly one store key. The "
                code { class: "{INLINE_CODE}", "stores" }
                " array lists every key the cycle touches; each key must already exist for that host in the app."
            }
            p { class: "{BODY_TEXT}",
                strong { "One writer per store:" }
                " at most one stage may write a given store key (orchestrator host). Two writers make it impossible to say which stage filled the store."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Triggers" }
            p { class: "{BODY_TEXT}", "Each stage has exactly one trigger:" }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "tick" }
                    " - runs on the host tick (orchestrator default loop)."
                }
                li {
                    code { class: "{INLINE_CODE}", "schedule" }
                    " - runs when a schedule fires (knowledge audit and publish stages)."
                }
                li {
                    code { class: "{INLINE_CODE}", "event" }
                    " - runs when the host raises an event (knowledge refresh after a recorded change)."
                }
                li {
                    code { class: "{INLINE_CODE}", "manual" }
                    " - runs only when an operator starts it; nothing will schedule it automatically."
                }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Store registry" }
            p { class: "{BODY_TEXT}",
                "Store keys are "
                strong { "not" }
                " free-form. The app ships a fixed registry per host. A cycle document may only name keys from that list; a key outside the list is rejected on install and reported again at activation. If the app later adds a store, this guide and the registry in the app must be updated together."
            }
            h3 { class: "mt-4 {SUBSECTION_TITLE}", "Orchestrator host" }
            dl { class: "{DL}",
                for (key, detail) in ORCHESTRATOR_STORES {
                    dt { class: "{DT}", code { "{key}" } }
                    dd { class: "{DD}", "{detail}" }
                }
            }
            h3 { class: "mt-6 {SUBSECTION_TITLE}", "Knowledge host" }
            dl { class: "{DL}",
                for (key, detail) in KNOWLEDGE_STORES {
                    dt { class: "{DT}", code { "{key}" } }
                    dd { class: "{DD}", "{detail}" }
                }
            }
            p { class: "{BODY_TEXT}",
                "The JSON Schema allows "
                code { class: "{INLINE_CODE}", "stories" }
                " and "
                code { class: "{INLINE_CODE}", "events" }
                " as host values, but the app does not install or activate cycles for those hosts yet."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "The orchestrator gate" }
            p { class: "{BODY_TEXT}",
                "On the orchestrator host, the "
                code { class: "{INLINE_CODE}", "propose" }
                " stage is the "
                strong { "gate" }
                ". It is compiled into the app: you cannot declare a different gate stage, remove it, or change its reads, writes, workflow, or trigger. Without that fixed gate, work could run without an approved proposal."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Activation checks" }
            p { class: "{BODY_TEXT}",
                "After install, the app runs cycle document checks before activation."
                strong { " Errors" }
                " block activation; warnings do not. Fix errors before publishing a cycle you expect operators to turn on:"
            }
            dl { class: "{DL}",
                for (check_id, detail) in BLOCKING_CHECKS {
                    dt { class: "{DT}", code { "{check_id}" } }
                    dd { class: "{DD}", "{detail}" }
                }
            }
            p { class: "{BODY_TEXT}",
                "Warnings (for example a store nothing reads, or a manual stage you never run) still appear in the UI but do not block activation."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Reference example" }
            p { class: "{BODY_TEXT}",
                "From "
                code { class: "{INLINE_CODE}", "packages/cycle-orchestrator-default/1.0.0/cycle.json" }
                " (five orchestrator stages, all "
                code { class: "{INLINE_CODE}", "tick" }
                "; the "
                code { class: "{INLINE_CODE}", "propose" }
                " stage uses the compiled "
                code { class: "{INLINE_CODE}", "create_proposal" }
                " gate):"
            }
            CodeExample { text: ORCHESTRATOR_EXAMPLE }
            p { class: "{BODY_TEXT}",
                "From "
                code { class: "{INLINE_CODE}", "packages/cycle-knowledge-default/1.0.0/cycle.json" }
                " (seven knowledge stages, mixed triggers):"
            }
            CodeExample { text: KNOWLEDGE_EXAMPLE }
            p { class: "{BODY_TEXT}",
                "The full files in the catalog list every stage. Declare the workflow packages those stages need as "
                code { class: "{INLINE_CODE}", "dependencies" }
                ". The compiled gate has no catalog package. Install those workflows (or a bundle that installs them) before you expect activation to succeed."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Install path" }
            CodeExample { text: INSTALL_ORCHESTRATOR }
            CodeExample { text: INSTALL_KNOWLEDGE }
            SchemaReferenceFooter {
                catalog_schema_label: "see.library/v1",
                schema_route: Route::SchemaCycle {},
                schema_link_label: "Cycle schema",
                extra_schema_links: None,
            }
        }
    }
}
