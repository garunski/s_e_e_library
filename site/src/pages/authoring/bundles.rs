use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, AuthoringReference, CodeExample, BODY_TEXT, DD, DL, DT, INLINE_CODE, LINK,
    LIST,
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
            section {
                h2 { "Package metadata" }
                p { class: BODY_TEXT,
                    "A bundle installs several library packages together. It has no payload file of its own; the catalog entry "
                    code { class: "{INLINE_CODE}", "files[]" }
                    " lists every member install map."
                }
                p { class: "{BODY_TEXT}",
                    "Each bundle carries a sidecar at "
                    code { class: "{INLINE_CODE}", "packages/{{slug}}/s_e_e_package.json" }
                    ":"
                }
                CodeExample { text: METADATA }
                dl { class: "{DL}",
                    div {
                        dt { class: "{DT}",
                            code { "id" }
                        }
                        dd { class: "{DD}",
                            "Globally unique package id ("
                            code { class: "{INLINE_CODE}", "bundle-{{slug}}" }
                            ")."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "slug" }
                        }
                        dd { class: "{DD}",
                            "URL-safe folder name under "
                            code { class: "{INLINE_CODE}", "packages/" }
                            "."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "category" }
                        }
                        dd { class: "{DD}",
                            "Must be "
                            code { class: "{INLINE_CODE}", "bundle" }
                            "."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "name" }
                            ", "
                            code { "description" }
                        }
                        dd { class: "{DD}", "Catalog presentation." }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "labels" }
                        }
                        dd { class: "{DD}", "Discovery tags." }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "dependencies" }
                        }
                        dd { class: "{DD}", "Other package ids installed alongside this bundle." }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "toolIds" }
                        }
                        dd { class: "{DD}",
                            "Declared tool ids, or "
                            code { class: "{INLINE_CODE}", "[]" }
                            " when uncategorized."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "releases" }
                        }
                        dd { class: "{DD}",
                            code { class: "{INLINE_CODE}", "{{ version, date, note }}" }
                            " history."
                        }
                    }
                }
            }
            section {
                h2 { "Members" }
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
                    li {
                        code { class: "{INLINE_CODE}", "cmd-cursor-agent" }
                    }
                    li {
                        code { class: "{INLINE_CODE}", "prompt-system-implement-story" }
                    }
                    li {
                        code { class: "{INLINE_CODE}", "skill-work" }
                    }
                    li {
                        code { class: "{INLINE_CODE}", "wf-system-implement-story" }
                    }
                    li {
                        code { class: "{INLINE_CODE}", "wf-system-implement-stories-bulk" }
                    }
                }
            }
            section {
                h2 { "Install map" }
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
            }
            section {
                h2 { "Install path rules" }
                p { class: "{BODY_TEXT}",
                    "Every "
                    code { class: "{INLINE_CODE}", "files[]" }
                    " "
                    code { class: "{INLINE_CODE}", "to" }
                    " path must start with one of:"
                }
                ul { class: "{LIST}",
                    li {
                        code { class: "{INLINE_CODE}", ".s_e_e/" }
                    }
                    li {
                        code { class: "{INLINE_CODE}", ".agents/" }
                    }
                    li {
                        code { class: "{INLINE_CODE}", ".cursor/" }
                    }
                    li {
                        code { class: "{INLINE_CODE}", "templates/" }
                    }
                }
                p { class: "{BODY_TEXT}",
                    "Paths use forward slashes and must be unique within the package."
                }
            }
            AuthoringReference {
                schema_route: Route::SchemaBundle {},
                schema_link_label: "Bundle schema",
                extra_schema_links: Some(rsx! {
                    li {
                        Link { to: Route::SchemaSchedule {}, class: LINK, "Schedule schema" }
                    }
                    li {
                        Link { to: Route::SchemaScheduleRuleSet {}, class: LINK, "Schedule rule set schema" }
                    }
                }),
            }
        }
    }
}
