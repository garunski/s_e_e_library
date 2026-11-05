use dioxus::prelude::*;

use crate::library_href;
use crate::pages::authoring_common::{CodeExample, INLINE_CODE, LIST, SECTION_TITLE};
use crate::pages::schema::index_markers::{
    CATEGORIES_HEADING, CLASSIFICATION_HEADING, DOCUMENT_TITLE, INSTALL_PATHS_HEADING,
};
use crate::pages::schema_common::SchemaBlockByName;

const STACKS_EXAMPLE: &str = include_str!("../../../content/schema/stacks_example.json");
const META_DESCRIPTION: &str = "The catalog is a single JSON manifest plus versioned payload files under packages/. Schema tags see.library/v1 and see.library/v2.";

#[component]
pub fn LibrarySchemaCatalog() -> Element {
    rsx! {
        document::Title { "{DOCUMENT_TITLE} - S.E.E. Library" }
        document::Meta { name: "description", content: META_DESCRIPTION }
        div { class: "library-home",
            div { class: "library-doc-page",
                h1 { "{DOCUMENT_TITLE}" }
                div { class: "library-doc-sections",
                    section {
                        h2 { id: "catalog-format", class: "scroll-mt-24", "Catalog format" }
                        h3 { class: SECTION_TITLE, "Manifest" }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            code { class: "{INLINE_CODE}", "catalog.json" }
                            ":"
                        }
                        ul { class: "{LIST}",
                            li {
                                code { class: "{INLINE_CODE}", "schema" }
                                " - "
                                code { class: "{INLINE_CODE}", "see.library/v1" }
                                " or "
                                code { class: "{INLINE_CODE}", "see.library/v2" }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "name" }
                                " - catalog display name."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "updated" }
                                " - ISO-8601 timestamp (refreshed by "
                                code { class: "{INLINE_CODE}", "mise run catalog" }
                                ")."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "packages[]" }
                                " - array of package entries."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "tools[]" }
                                " - optional CLI tool registry."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "stacks[]" }
                                " - optional source-owned stacks with "
                                code { class: "{INLINE_CODE}", "sharedPackageIds" }
                                " and "
                                code { class: "{INLINE_CODE}", "variants" }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "featuredStackId" }
                                " - optional id of one declared stack."
                            }
                        }
                        h3 { class: "mt-6 {SECTION_TITLE}", "Package entry" }
                        ul { class: "{LIST}",
                            li {
                                code { class: "{INLINE_CODE}", "id" }
                                ", "
                                code { class: "{INLINE_CODE}", "slug" }
                                ", "
                                code { class: "{INLINE_CODE}", "category" }
                                " (workflow, prompt, skill, rule, template, command, bundle, cycle), "
                                code { class: "{INLINE_CODE}", "version" }
                                ", "
                                code { class: "{INLINE_CODE}", "author" }
                                ", "
                                code { class: "{INLINE_CODE}", "verified" }
                                ", "
                                code { class: "{INLINE_CODE}", "license" }
                                ", "
                                code { class: "{INLINE_CODE}", "requires" }
                                ", "
                                code { class: "{INLINE_CODE}", "dependencies" }
                                ", "
                                code { class: "{INLINE_CODE}", "files[]" }
                                " (install map)."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "toolIds" }
                                ", "
                                code { class: "{INLINE_CODE}", "usageContexts" }
                                ", "
                                code { class: "{INLINE_CODE}", "cliScope" }
                                ", "
                                code { class: "{INLINE_CODE}", "releases" }
                                "."
                            }
                        }
                        h3 { class: "mt-6 {SECTION_TITLE}", "{CLASSIFICATION_HEADING}" }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            code { class: "{INLINE_CODE}", "category" }
                            " is the package kind. "
                            code { class: "{INLINE_CODE}", "labels" }
                            " are topic tags. Classification fields describe use context and compatibility. New packages must author "
                            code { class: "{INLINE_CODE}", "usageContexts" }
                            " and "
                            code { class: "{INLINE_CODE}", "cliScope" }
                            " on the sidecar (copied into "
                            code { class: "{INLINE_CODE}", "catalog.json" }
                            " by "
                            code { class: "{INLINE_CODE}", "mise run catalog" }
                            ")."
                        }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "Legacy v2 entries without classification still load. The hub infers effective values without rewriting catalog files:"
                        }
                        ul { class: "{LIST}",
                            li {
                                code { class: "{INLINE_CODE}", "usageContexts" }
                                ": "
                                code { class: "{INLINE_CODE}", "cycle" }
                                " when "
                                code { class: "{INLINE_CODE}", "category" }
                                " is "
                                code { class: "{INLINE_CODE}", "cycle" }
                                ", otherwise "
                                code { class: "{INLINE_CODE}", "standalone" }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "cliScope" }
                                ": "
                                code { class: "{INLINE_CODE}", "any" }
                                " when "
                                code { class: "{INLINE_CODE}", "toolIds" }
                                " is empty or omitted; "
                                code { class: "{INLINE_CODE}", "specific" }
                                " with those ids when "
                                code { class: "{INLINE_CODE}", "toolIds" }
                                " is non-empty."
                            }
                        }
                        h3 { class: "mt-6 {SECTION_TITLE}", "Package metadata sidecar" }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "Every package carries a "
                            code { class: "{INLINE_CODE}", "packages/{{slug}}/s_e_e_package.json" }
                            "; the build cross-checks it against the catalog entry (id, slug, category, name, description, labels, dependencies, toolIds, classification fields, releases)."
                        }
                    }
                    section {
                        h2 { "Installation" }
                        h3 { class: SECTION_TITLE, "{CATEGORIES_HEADING}" }
                        ul { class: "{LIST}",
                            li {
                                code { class: "{INLINE_CODE}", "workflow" }
                                " - "
                                code { class: "{INLINE_CODE}",
                                    ".s_e_e/workflows/definitions/{{slug}}.json"
                                }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "prompt" }
                                " - "
                                code { class: "{INLINE_CODE}", ".s_e_e/prompts/{{file}}.json" }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "skill" }
                                " - "
                                code { class: "{INLINE_CODE}", ".agents/skills/{{slug}}/SKILL.md" }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "rule" }
                                " - "
                                code { class: "{INLINE_CODE}", ".cursor/rules/{{slug}}.mdc" }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "template" }
                                " - under "
                                code { class: "{INLINE_CODE}", "templates/" }
                                "."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "command" }
                                " - "
                                code { class: "{INLINE_CODE}", ".s_e_e/commands/{{id}}.json" }
                                " (to path must match)."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "bundle" }
                                " - no payload; "
                                code { class: "{INLINE_CODE}", "files[]" }
                                " installs members."
                            }
                            li {
                                code { class: "{INLINE_CODE}", "cycle" }
                                " - "
                                code { class: "{INLINE_CODE}", ".s_e_e/cycles/{{id}}.json" }
                                "."
                            }
                        }
                        h3 {
                            id: "install-paths",
                            class: "mt-6 scroll-mt-24 {SECTION_TITLE}",
                            "{INSTALL_PATHS_HEADING}"
                        }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "Every "
                            code { class: "{INLINE_CODE}", "files[]" }
                            " "
                            code { class: "{INLINE_CODE}", "to" }
                            " path must start with "
                            code { class: "{INLINE_CODE}", ".s_e_e/" }
                            ", "
                            code { class: "{INLINE_CODE}", ".agents/" }
                            ", "
                            code { class: "{INLINE_CODE}", ".cursor/" }
                            ", or "
                            code { class: "{INLINE_CODE}", "templates/" }
                            "; use forward slashes; be unique within the package. Bundle "
                            code { class: "{INLINE_CODE}", "to" }
                            " prefixes determine payload validation kind (workflow, prompt, command, skill, rule, template, schedule, cycle, knowledge markdown)."
                        }
                    }
                    section {
                        h2 { "Stacks" }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "Author stacks in "
                            code { class: "{INLINE_CODE}", "scripts/stacks.json" }
                            ". "
                            code { class: "{INLINE_CODE}", "sharedPackageIds" }
                            " lists packages every variant installs. Each variant names a declared "
                            code { class: "{INLINE_CODE}", "toolId" }
                            " and additional "
                            code { class: "{INLINE_CODE}", "packageIds" }
                            ". "
                            code { class: "{INLINE_CODE}", "featuredStackId" }
                            " must be one declared stack id."
                        }
                        CodeExample { text: STACKS_EXAMPLE }
                    }
                    section {
                        h2 { "Validation" }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "The Rust catalog validator enforces cross-references between scopes and registries. Run "
                            code { class: "{INLINE_CODE}", "mise run catalog" }
                            " to validate payloads and refresh the manifest, or "
                            code { class: "{INLINE_CODE}", "mise run validate" }
                            " (no writes) in CI."
                        }
                    }
                    section {
                        h2 { id: "json-schema", class: "scroll-mt-24", "JSON Schema" }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "Live catalog: "
                            a {
                                class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300",
                                href: library_href("/catalog.json"),
                                code { class: "{INLINE_CODE}", "catalog.json" }
                            }
                            "."
                        }
                        SchemaBlockByName { filename: "catalog.schema.json" }
                        h3 { class: "mt-6 {SECTION_TITLE}", "Package metadata sidecar schema" }
                        SchemaBlockByName { filename: "package-meta.schema.json" }
                    }
                }
            }
        }
    }
}
