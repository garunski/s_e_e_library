use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring::publish_wire::{
    CATALOG_IDENTITY, CLI_SCOPE_ANY, CLI_SCOPE_SPECIFIC, PACKAGE_SIDECAR_PATH, PAYLOAD_LAYOUT,
    RELEASE_HISTORY_SHAPE,
};
use crate::pages::authoring_common::{
    AuthoringGuideFrame, AuthoringReference, CodeExample, BODY_TEXT, DD, DL, DT, INLINE_CODE, LIST,
};
use crate::routes::Route;

const STACKS: &str = include_str!("../../../content/authoring/publish_stacks.json");
const CLASSIFIED_SIDECAR: &str =
    include_str!("../../../content/authoring/publish_classified_sidecar.json");
const COMMAND_ENTRY: &str = include_str!("../../../content/authoring/publish_command_entry.json");

#[component]
pub fn LibraryAuthoringPublish() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Publish to the catalog",
            meta_description: "Add a package payload to this repository, update catalog.json, and validate before opening a pull request.",
            section {
                h2 { "Payload layout" }
                p { class: "{BODY_TEXT}", "Place files under:" }
                CodeExample { text: PAYLOAD_LAYOUT }
                p { class: "{BODY_TEXT}", "Examples:" }
                ul { class: "{LIST}",
                    li {
                        code { class: "{INLINE_CODE}", "packages/cursor-agent/1.0.0/command.json" }
                    }
                    li {
                        code { class: "{INLINE_CODE}",
                            "packages/system-implement-story/1.0.0/prompt.json"
                        }
                    }
                    li {
                        code { class: "{INLINE_CODE}", "packages/work/1.0.0/SKILL.md" }
                    }
                    li {
                        code { class: "{INLINE_CODE}",
                            "packages/implement-story/1.0.0/system-implement-story/definition.json"
                        }
                    }
                }
                p { class: "{BODY_TEXT}",
                    "Add "
                    code { class: "{INLINE_CODE}", "{PACKAGE_SIDECAR_PATH}" }
                    " metadata for every catalog entry."
                }
                p { class: "{BODY_TEXT}",
                    "Bundles copy member payloads under the bundle slug folder, for example "
                    code { class: "{INLINE_CODE}", "packages/implement-story/1.0.0/work/SKILL.md" }
                    "."
                }
            }
            section {
                h2 { "Catalog entry" }
                p { class: "{BODY_TEXT}",
                    "Append a package object to "
                    code { class: "{INLINE_CODE}", "catalog.json" }
                    " "
                    code { class: "{INLINE_CODE}", "packages[]" }
                    ":"
                }
                dl { class: "{DL}",
                    div {
                        dt { class: "{DT}",
                            code { "id" }
                        }
                        dd { class: "{DD}",
                            "Globally unique id (for example "
                            code { class: "{INLINE_CODE}", "cmd-cursor-agent" }
                            ", "
                            code { class: "{INLINE_CODE}", "bundle-implement-story" }
                            ")."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "slug" }
                        }
                        dd { class: "{DD}",
                            "URL-safe slug matching the "
                            code { class: "{INLINE_CODE}", "packages/" }
                            " folder."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "category" }
                        }
                        dd { class: "{DD}",
                            code { class: "{INLINE_CODE}", "workflow" }
                            ", "
                            code { class: "{INLINE_CODE}", "prompt" }
                            ", "
                            code { class: "{INLINE_CODE}", "skill" }
                            ", "
                            code { class: "{INLINE_CODE}", "command" }
                            ", or "
                            code { class: "{INLINE_CODE}", "bundle" }
                            "."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "version" }
                        }
                        dd { class: "{DD}",
                            "Semver string (for example "
                            code { class: "{INLINE_CODE}", "1.0.0" }
                            ")."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "author" }
                        }
                        dd { class: "{DD}", "Publisher id." }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "verified" }
                        }
                        dd { class: "{DD}",
                            code { class: "{INLINE_CODE}", "true" }
                            " when signed off by a maintainer."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "license" }
                        }
                        dd { class: "{DD}",
                            "SPDX id (for example "
                            code { class: "{INLINE_CODE}", "AGPL-3.0-only" }
                            ")."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "requires" }
                        }
                        dd { class: "{DD}",
                            "External tools required at install time (empty array when none)."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "dependencies" }
                        }
                        dd { class: "{DD}",
                            "Other package "
                            code { class: "{INLINE_CODE}", "id" }
                            "s installed alongside this one."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "files[]" }
                        }
                        dd { class: "{DD}",
                            "Install map with "
                            code { class: "{INLINE_CODE}", "to" }
                            " and "
                            code { class: "{INLINE_CODE}", "from" }
                            " paths."
                        }
                    }
                }
                p { class: "{BODY_TEXT}",
                    "Optional presentation fields: "
                    code { class: "{INLINE_CODE}", "name" }
                    ", "
                    code { class: "{INLINE_CODE}", "description" }
                    ", "
                    code { class: "{INLINE_CODE}", "labels" }
                    "."
                }
                p { class: "{BODY_TEXT}", "{CATALOG_IDENTITY}" }
                p { class: "{BODY_TEXT}",
                    "Marketplace and classification fields live on the sidecar and are copied into the catalog. Do not hand-edit them on "
                    code { class: "{INLINE_CODE}", "catalog.json" }
                    "."
                }
            }
            section {
                h2 { "Package classification" }
                p { class: "{BODY_TEXT}",
                    "Every new package must declare usage contexts and CLI compatibility in "
                    code { class: "{INLINE_CODE}", "s_e_e_package.json" }
                    ". Specific CLI IDs must be declared in the catalog tools registry."
                }
                dl { class: "{DL}",
                    div {
                        dt { class: "{DT}",
                            code { "usageContexts" }
                        }
                        dd { class: "{DD}",
                            "Non-empty array of "
                            code { class: "{INLINE_CODE}", "cycle" }
                            " or "
                            code { class: "{INLINE_CODE}", "standalone" }
                            ". Use "
                            code { class: "{INLINE_CODE}", "cycle" }
                            " for cycle host documents; otherwise include "
                            code { class: "{INLINE_CODE}", "standalone" }
                            "."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "cliScope" }
                        }
                        dd { class: "{DD}",
                            code { class: "{INLINE_CODE}", "{CLI_SCOPE_ANY}" }
                            " or "
                            code { class: "{INLINE_CODE}", "{CLI_SCOPE_SPECIFIC}" }
                            ". Each "
                            code { class: "{INLINE_CODE}", "cliIds" }
                            " entry must exist in "
                            code { class: "{INLINE_CODE}", "tools[]" }
                            "."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "toolIds" }
                        }
                        dd { class: "{DD}",
                            "Declared CLI tool ids. Use "
                            code { class: "{INLINE_CODE}", "[]" }
                            " with "
                            code { class: "{INLINE_CODE}", "cliScope.scope: any" }
                            ". When scope is specific, "
                            code { class: "{INLINE_CODE}", "toolIds" }
                            " should match "
                            code { class: "{INLINE_CODE}", "cliIds" }
                            ". Never guess from the package name."
                        }
                    }
                    div {
                        dt { class: "{DT}",
                            code { "releases" }
                        }
                        dd { class: "{DD}",
                            code { class: "{INLINE_CODE}", "{RELEASE_HISTORY_SHAPE}" }
                            " history for the package."
                        }
                    }
                }
                p { class: "{BODY_TEXT}",
                    "Older catalog entries without these fields still install. The hub infers usage and scopes from "
                    code { class: "{INLINE_CODE}", "category" }
                    " and legacy "
                    code { class: "{INLINE_CODE}", "toolIds" }
                    " until you republish with explicit classification."
                }
                p { class: "{BODY_TEXT}",
                    "Stacks, the tool registry, and featured selection live in "
                    code { class: "{INLINE_CODE}", "scripts/stacks.json" }
                    ":"
                }
                CodeExample { text: STACKS }
                p { class: "{BODY_TEXT}",
                    "Example classified workflow sidecar fields (see also catalog schema):"
                }
                CodeExample { text: CLASSIFIED_SIDECAR }
                p { class: "{BODY_TEXT}", "Example command catalog entry (presentation fields only):" }
                CodeExample { text: COMMAND_ENTRY }
            }
            AuthoringReference {
                schema_route: Route::SchemaCatalog {},
                schema_link_label: "Catalog schema",
                extra_schema_links: None,
            }
        }
    }
}

#[cfg(test)]
#[path = "publish_wire_tests.rs"]
mod publish_wire_tests;
