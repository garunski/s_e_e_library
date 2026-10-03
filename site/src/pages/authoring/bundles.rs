use dioxus::prelude::*;
use dioxus_router::Link;

use crate::library_href;
use crate::pages::authoring_common::{
    AuthoringGuideFrame, BODY_TEXT, CodeExample, INLINE_CODE, LINK, LIST, PAGE_LEDE,
    SECTION_TITLE, DL, DD, DT,
};
use crate::routes::Route;

const METADATA: &str = include_str!("../../../content/authoring/bundles_metadata.json");
const INSTALL_MAP: &str = include_str!("../../../content/authoring/bundles_install_map.json");

#[component]
pub fn LibraryAuthoringBundles() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Bundles",
            meta_description: "A bundle installs several library packages together using a shared catalog files map.",
            rail_number: "05",
            rail_title: "The install path decides the kind.",
            rail_description: "The build decides payload kind from the destination prefix.",
            p { class: "{PAGE_LEDE}",
                "A bundle installs several library packages together. It has no payload file of its own; the catalog entry "
                code { class: "{INLINE_CODE}", "files[]" }
                " lists every member install map."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Metadata (s_e_e_package.json)" }
            p { class: "{BODY_TEXT}",
                "Each bundle carries a sidecar at "
                code { class: "{INLINE_CODE}", "packages/{{slug}}/s_e_e_package.json" }
                ":"
            }
            CodeExample { text: METADATA }
            dl { class: "{DL}",
                dt { class: "{DT}", code { "id" } }
                dd { class: "{DD}",
                    "Globally unique package id ("
                    code { class: "{INLINE_CODE}", "bundle-{{slug}}" }
                    ")."
                }
                dt { class: "{DT}", code { "slug" } }
                dd { class: "{DD}",
                    "URL-safe folder name under "
                    code { class: "{INLINE_CODE}", "packages/" }
                    "."
                }
                dt { class: "{DT}", code { "category" } }
                dd { class: "{DD}",
                    "Must be "
                    code { class: "{INLINE_CODE}", "bundle" }
                    "."
                }
                dt { class: "{DT}", code { "name" }, ", ", code { "description" } }
                dd { class: "{DD}", "Catalog presentation." }
                dt { class: "{DT}", code { "labels" } }
                dd { class: "{DD}", "Discovery tags." }
                dt { class: "{DT}", code { "dependencies" } }
                dd { class: "{DD}", "Other package ids installed alongside this bundle." }
                dt { class: "{DT}", code { "toolIds" } }
                dd { class: "{DD}",
                    "Declared tool ids, or "
                    code { class: "{INLINE_CODE}", "[]" }
                    " when uncategorized."
                }
                dt { class: "{DT}", code { "releases" } }
                dd { class: "{DD}",
                    code { class: "{INLINE_CODE}", "{{ version, date, note }}" }
                    " history."
                }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Members" }
            p { class: "{BODY_TEXT}",
                "When harvesting from a local hub, "
                code { class: "{INLINE_CODE}", "scripts/bundles.json" }
                " lists member package ids per bundle slug. The harvest script copies each member payload into "
                code { class: "{INLINE_CODE}", "packages/{{bundle-slug}}/{{version}}/" }
                " and builds the catalog "
                code { class: "{INLINE_CODE}", "files[]" }
                " array. Hand-authored bundles list every "
                code { class: "{INLINE_CODE}", "to" }
                "/"
                code { class: "{INLINE_CODE}", "from" }
                " pair directly in "
                code { class: "{INLINE_CODE}", "catalog.json" }
                "."
            }
            p { class: "{BODY_TEXT}",
                "Marketplace stacks are a different source. "
                code { class: "{INLINE_CODE}", "scripts/stacks.json" }
                " declares "
                code { class: "{INLINE_CODE}", "sharedPackageIds" }
                ", per-tool "
                code { class: "{INLINE_CODE}", "variants" }
                ", and "
                code { class: "{INLINE_CODE}", "featuredStackId" }
                ". A bundle is an install map; a stack is a source-owned grouping that can install one tool path."
            }
            p { class: "{BODY_TEXT}",
                "Example members for "
                code { class: "{INLINE_CODE}", "implement-story" }
                " (from "
                code { class: "{INLINE_CODE}", "scripts/bundles.json" }
                "):"
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "cmd-cursor-agent" } }
                li { code { class: "{INLINE_CODE}", "prompt-system-implement-story" } }
                li { code { class: "{INLINE_CODE}", "skill-work" } }
                li { code { class: "{INLINE_CODE}", "wf-system-implement-story" } }
                li { code { class: "{INLINE_CODE}", "wf-system-implement-stories-bulk" } }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Install map" }
            p { class: "{BODY_TEXT}", "Each entry maps a repo path to an install target:" }
            CodeExample { text: INSTALL_MAP }
            p { class: "{BODY_TEXT}",
                "The build infers payload kind from the "
                code { class: "{INLINE_CODE}", "to" }
                " prefix:"
            }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", ".s_e_e/workflows/definitions/" }
                    " is a workflow;"
                }
                li {
                    code { class: "{INLINE_CODE}", ".s_e_e/workflows/schedules/" }
                    " is a schedule;"
                }
                li {
                    code { class: "{INLINE_CODE}", ".s_e_e/workflows/schedule_rule_sets/" }
                    " is a schedule rule set;"
                }
                li {
                    code { class: "{INLINE_CODE}", ".s_e_e/prompts/" }
                    " is a prompt;"
                }
                li {
                    code { class: "{INLINE_CODE}", ".s_e_e/commands/" }
                    " is a command;"
                }
                li {
                    code { class: "{INLINE_CODE}", ".s_e_e/knowledge/" }
                    " is a knowledge markdown payload (for example the component inventory template);"
                }
                li {
                    code { class: "{INLINE_CODE}", ".agents/skills/" }
                    " is a skill;"
                }
                li {
                    code { class: "{INLINE_CODE}", ".cursor/" }
                    " is a rule;"
                }
                li {
                    code { class: "{INLINE_CODE}", "templates/" }
                    " is a template."
                }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Install path rules" }
            p { class: "{BODY_TEXT}",
                "Every "
                code { class: "{INLINE_CODE}", "files[]" }
                " "
                code { class: "{INLINE_CODE}", "to" }
                " path must start with one of:"
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", ".s_e_e/" } }
                li { code { class: "{INLINE_CODE}", ".agents/" } }
                li { code { class: "{INLINE_CODE}", ".cursor/" } }
                li { code { class: "{INLINE_CODE}", "templates/" } }
            }
            p { class: "{BODY_TEXT}", "Paths use forward slashes and must be unique within the package." }
            h2 { class: "mt-8 {SECTION_TITLE}", "Schema reference" }
            ul { class: "mt-3 list-disc space-y-2 pl-5 text-sm text-zinc-600 dark:text-zinc-400",
                li {
                    "Catalog manifest: "
                    a {
                        class: "{LINK}",
                        href: library_href("/catalog.json"),
                        code { class: "{INLINE_CODE}", "see.library/v2" }
                    }
                }
                li {
                    Link { to: Route::SchemaBundle {}, class: "{LINK}", "Bundle schema" }
                }
                li {
                    Link { to: Route::SchemaSchedule {}, class: "{LINK}", "Schedule schema" }
                }
                li {
                    Link { to: Route::SchemaScheduleRuleSet {}, class: "{LINK}", "Schedule rule set schema" }
                }
            }
        }
    }
}
