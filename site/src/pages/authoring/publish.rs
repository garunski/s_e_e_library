use dioxus::prelude::*;
use dioxus_router::Link;

use crate::library_href;
use crate::pages::authoring::publish_wire::{
    CATALOG_IDENTITY, CLI_SCOPE_ANY, CLI_SCOPE_SPECIFIC, DOCS_CHECK_COMMANDS, PACKAGE_SIDECAR_PATH,
    PAYLOAD_LAYOUT, RELEASE_HISTORY_SHAPE, TECH_SCOPE_GENERAL, TECH_SCOPE_SPECIFIC,
    VALIDATE_AND_CATALOG_COMMANDS,
};
use crate::pages::authoring_common::{
    AuthoringGuideFrame, CodeExample, BODY_TEXT, DD, DL, DT, INLINE_CODE, LINK, LIST, PAGE_LEDE,
    SECTION_TITLE,
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
            rail_number: "07",
            rail_title: "Publishing is a catalog entry.",
            rail_description: "The payload lives under packages/; catalog.json is the install map. Validation gates the entry.",
            p { class: "{PAGE_LEDE}",
                "After authoring a package payload, add it to this repository and validate before opening a pull request."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Payload layout" }
            p { class: "{BODY_TEXT}", "Place files under:" }
            CodeExample { text: PAYLOAD_LAYOUT }
            p { class: "{BODY_TEXT}", "Examples:" }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "packages/cursor-agent/1.0.0/command.json" } }
                li { code { class: "{INLINE_CODE}", "packages/system-implement-story/1.0.0/prompt.json" } }
                li { code { class: "{INLINE_CODE}", "packages/work/1.0.0/SKILL.md" } }
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
            h2 { class: "mt-8 {SECTION_TITLE}", "catalog.json entry" }
            p { class: "{BODY_TEXT}",
                "Append a package object to "
                code { class: "{INLINE_CODE}", "catalog.json" }
                " "
                code { class: "{INLINE_CODE}", "packages[]" }
                ":"
            }
            dl { class: "{DL}",
                dt { class: "{DT}", code { "id" } }
                dd { class: "{DD}",
                    "Globally unique id (for example "
                    code { class: "{INLINE_CODE}", "cmd-cursor-agent" }
                    ", "
                    code { class: "{INLINE_CODE}", "bundle-implement-story" }
                    ")."
                }
                dt { class: "{DT}", code { "slug" } }
                dd { class: "{DD}",
                    "URL-safe slug matching the "
                    code { class: "{INLINE_CODE}", "packages/" }
                    " folder."
                }
                dt { class: "{DT}", code { "category" } }
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
                dt { class: "{DT}", code { "version" } }
                dd { class: "{DD}",
                    "Semver string (for example "
                    code { class: "{INLINE_CODE}", "1.0.0" }
                    ")."
                }
                dt { class: "{DT}", code { "author" } }
                dd { class: "{DD}", "Publisher id." }
                dt { class: "{DT}", code { "verified" } }
                dd { class: "{DD}",
                    code { class: "{INLINE_CODE}", "true" }
                    " when signed off by a maintainer."
                }
                dt { class: "{DT}", code { "license" } }
                dd { class: "{DD}",
                    "SPDX id (for example "
                    code { class: "{INLINE_CODE}", "AGPL-3.0-only" }
                    ")."
                }
                dt { class: "{DT}", code { "requires" } }
                dd { class: "{DD}", "External tools required at install time (empty array when none)." }
                dt { class: "{DT}", code { "dependencies" } }
                dd { class: "{DD}",
                    "Other package "
                    code { class: "{INLINE_CODE}", "id" }
                    "s installed alongside this one."
                }
                dt { class: "{DT}", code { "files[]" } }
                dd { class: "{DD}",
                    "Install map with "
                    code { class: "{INLINE_CODE}", "to" }
                    " and "
                    code { class: "{INLINE_CODE}", "from" }
                    " paths."
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
            h2 { class: "mt-8 {SECTION_TITLE}", "Classification (required for new packages)" }
            p { class: "{BODY_TEXT}",
                "Every new package must declare how it is used and what it is compatible with. Add these fields to "
                code { class: "{INLINE_CODE}", "s_e_e_package.json" }
                " (and declare matching registry rows in "
                code { class: "{INLINE_CODE}", "scripts/stacks.json" }
                " or catalog "
                code { class: "{INLINE_CODE}", "technologies[]" }
                " when using specific scopes):"
            }
            dl { class: "{DL}",
                dt { class: "{DT}", code { "usageContexts" } }
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
                dt { class: "{DT}", code { "cliScope" } }
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
                dt { class: "{DT}", code { "technologyScope" } }
                dd { class: "{DD}",
                    code { class: "{INLINE_CODE}", "{TECH_SCOPE_GENERAL}" }
                    " or "
                    code { class: "{INLINE_CODE}", "{TECH_SCOPE_SPECIFIC}" }
                    ". When specific, list "
                    code { class: "{INLINE_CODE}", "technologyIds" }
                    " on the sidecar (or nested on the scope) and declare each id under catalog "
                    code { class: "{INLINE_CODE}", "technologies[]" }
                    "."
                }
                dt { class: "{DT}", code { "toolIds" } }
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
                dt { class: "{DT}", code { "releases" } }
                dd { class: "{DD}",
                    code { class: "{INLINE_CODE}", "{RELEASE_HISTORY_SHAPE}" }
                    " history for the package."
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
            p { class: "{BODY_TEXT}", "Example classified workflow sidecar fields (see also catalog schema):" }
            CodeExample { text: CLASSIFIED_SIDECAR }
            p { class: "{BODY_TEXT}", "Example command catalog entry (presentation fields only):" }
            CodeExample { text: COMMAND_ENTRY }
            h2 { class: "mt-8 {SECTION_TITLE}", "Validate and build" }
            CodeExample { text: VALIDATE_AND_CATALOG_COMMANDS }
            ul { class: "{LIST}",
                li {
                    code { class: "{INLINE_CODE}", "mise run validate" }
                    " runs "
                    code { class: "{INLINE_CODE}", "see_library_catalog_tool validate" }
                    " (read-only; fails when payloads or metadata are invalid or out of sync)."
                }
                li {
                    code { class: "{INLINE_CODE}", "mise run catalog" }
                    " validates payloads, refreshes "
                    code { class: "{INLINE_CODE}", "catalog.json" }
                    " "
                    code { class: "{INLINE_CODE}", "updated" }
                    ", and writes any derived fields."
                }
                li {
                    code { class: "{INLINE_CODE}", "mise run harvest" }
                    " copies hub payloads into "
                    code { class: "{INLINE_CODE}", "packages/" }
                    " and regenerates "
                    code { class: "{INLINE_CODE}", "catalog.json" }
                    " when publishing from a local hub checkout."
                }
            }
            p { class: "{BODY_TEXT}",
                "Commit both the payload under "
                code { class: "{INLINE_CODE}", "packages/" }
                " and the updated "
                code { class: "{INLINE_CODE}", "catalog.json" }
                "."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Docs site" }
            CodeExample { text: DOCS_CHECK_COMMANDS }
            p { class: "{BODY_TEXT}",
                code { class: "{INLINE_CODE}", "mise run quality" }
                " runs catalog validation ("
                code { class: "{INLINE_CODE}", "see_library_catalog_tool validate" }
                ") and site library tests. "
                code { class: "{INLINE_CODE}", "mise run site-build" }
                " exports the Dioxus static site and copies root "
                code { class: "{INLINE_CODE}", "catalog.json" }
                " and "
                code { class: "{INLINE_CODE}", "packages/" }
                " into the publish output so GitHub Pages serves them at the same public URLs."
            }
            p { class: "{BODY_TEXT}",
                "A new package type also needs a section in "
                code { class: "{INLINE_CODE}", "public/llms.txt" }
                ", which carries the same guidance as one plain-text file for agents. Keep the page and "
                code { class: "{INLINE_CODE}", "Source: /authoring/publish/" }
                " section in sync."
            }
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
                    Link { to: Route::SchemaIndex {}, class: "{LINK}", "Catalog schema" }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "publish_wire_tests.rs"]
mod publish_wire_tests;
