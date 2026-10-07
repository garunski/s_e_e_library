use dioxus::prelude::*;

use crate::library_href;
use crate::pages::authoring_common::{CodeExample, INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema::index_markers::{
    CATEGORIES_HEADING, CLASSIFICATION_HEADING, DOCUMENT_TITLE, INSTALL_PATHS_HEADING,
};
use crate::pages::schema_common::{NamedSchemaLink, SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

const STACKS_EXAMPLE: &str = include_str!("../../../content/schema/stacks_example.json");

#[component]
pub fn LibrarySchemaIndex() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: DOCUMENT_TITLE,
            meta_description: "The catalog is a single JSON manifest plus versioned payload files under packages/. Schema tags see.library/v1 and see.library/v2.",
            rail_number: "00",
            rail_title: "The build enforces the schema.",
            rail_description: "see.library/v1 and see.library/v2 catalog manifests.",
            p { class: "{PAGE_LEDE}",
                "The catalog is a single JSON manifest ("
                code { class: "{INLINE_CODE}", "catalog.json" }
                ") plus versioned payload files under "
                code { class: "{INLINE_CODE}", "packages/" }
                ". The manifest schema tag is "
                code { class: "{INLINE_CODE}", "see.library/v1" }
                " or "
                code { class: "{INLINE_CODE}", "see.library/v2" }
                ". This section is the authoritative contract for this repository; "
                code { class: "{INLINE_CODE}", "mise run catalog" }
                " enforces it."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Per-type payload schemas" }
            ul { class: "mt-3 list-none space-y-2 text-sm text-zinc-600 dark:text-zinc-400",
                NamedSchemaLink { route: Route::SchemaWorkflow {}, label: "Workflow", description: "definition.json envelope and engine content." }
                NamedSchemaLink { route: Route::SchemaPrompt {}, label: "Prompt", description: "id, name, and content." }
                NamedSchemaLink { route: Route::SchemaSkill {}, label: "Skill", description: "SKILL.md frontmatter." }
                NamedSchemaLink { route: Route::SchemaCommand {}, label: "Command", description: "binary, args, params, result, execution." }
                NamedSchemaLink { route: Route::SchemaRuleTemplate {}, label: "Rule and template", description: "Lightweight file-only categories." }
                NamedSchemaLink { route: Route::SchemaBundle {}, label: "Bundle", description: "Install map with no payload of its own." }
                NamedSchemaLink { route: Route::SchemaCycle {}, label: "Cycle", description: "Stages, stores, and triggers for a host loop." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Hub document schemas" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "JSON documents the hub stores besides catalog payloads. Each has a schema file and is validated on save."
            }
            ul { class: "mt-3 list-none space-y-2 text-sm text-zinc-600 dark:text-zinc-400",
                NamedSchemaLink { route: Route::SchemaHubConfig {}, label: "Hub config", description: ".s_e_e/config.json at the hub data root." }
                NamedSchemaLink { route: Route::SchemaGlobalConfig {}, label: "Global config", description: "~/.s_e_e/config.json." }
                NamedSchemaLink { route: Route::SchemaAppSettings {}, label: "App settings", description: "~/.s_e_e/settings.json." }
                NamedSchemaLink { route: Route::SchemaRoutingRules {}, label: "Routing rules", description: "llm.routing in hub config." }
                NamedSchemaLink { route: Route::SchemaStoriesConfig {}, label: "Stories config", description: "stories in hub config." }
                NamedSchemaLink { route: Route::SchemaOrchestratorPolicy {}, label: "Orchestrator policy", description: "orchestrator in hub config." }
                NamedSchemaLink { route: Route::SchemaSchedule {}, label: "Schedule", description: ".s_e_e/workflows/schedules/{{id}}.json." }
                NamedSchemaLink { route: Route::SchemaScheduleRuleSet {}, label: "Schedule rule set", description: ".s_e_e/workflows/schedule_rule_sets/{{id}}.json." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Manifest" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", code { class: "{INLINE_CODE}", "catalog.json" }, ":" }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "schema" }, " - ", code { class: "{INLINE_CODE}", "see.library/v1" }, " or ", code { class: "{INLINE_CODE}", "see.library/v2" }, "." }
                li { code { class: "{INLINE_CODE}", "name" }, " - catalog display name." }
                li { code { class: "{INLINE_CODE}", "updated" }, " - ISO-8601 timestamp (refreshed by ", code { class: "{INLINE_CODE}", "mise run catalog" }, ")." }
                li { code { class: "{INLINE_CODE}", "packages[]" }, " - array of package entries." }
                li { code { class: "{INLINE_CODE}", "tools[]" }, " - optional CLI tool registry." }
                li { code { class: "{INLINE_CODE}", "technologies[]" }, " - optional technology registry." }
                li { code { class: "{INLINE_CODE}", "stacks[]" }, " - optional source-owned stacks with ", code { class: "{INLINE_CODE}", "sharedPackageIds" }, " and ", code { class: "{INLINE_CODE}", "variants" }, "." }
                li { code { class: "{INLINE_CODE}", "featuredStackId" }, " - optional id of one declared stack." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Package entry" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Each entry in ", code { class: "{INLINE_CODE}", "packages[]" }, ":" }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "id" }, ", ", code { class: "{INLINE_CODE}", "slug" }, ", ", code { class: "{INLINE_CODE}", "category" }, " (workflow, prompt, skill, rule, template, command, bundle, cycle), ", code { class: "{INLINE_CODE}", "version" }, ", ", code { class: "{INLINE_CODE}", "author" }, ", ", code { class: "{INLINE_CODE}", "verified" }, ", ", code { class: "{INLINE_CODE}", "license" }, ", ", code { class: "{INLINE_CODE}", "requires" }, ", ", code { class: "{INLINE_CODE}", "dependencies" }, ", ", code { class: "{INLINE_CODE}", "files[]" }, " (install map)." }
                li { code { class: "{INLINE_CODE}", "toolIds" }, ", ", code { class: "{INLINE_CODE}", "usageContexts" }, ", ", code { class: "{INLINE_CODE}", "cliScope" }, ", ", code { class: "{INLINE_CODE}", "technologyScope" }, ", ", code { class: "{INLINE_CODE}", "technologyIds" }, ", ", code { class: "{INLINE_CODE}", "releases" }, "." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "{CLASSIFICATION_HEADING}" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                code { class: "{INLINE_CODE}", "category" }
                " is the package kind. "
                code { class: "{INLINE_CODE}", "labels" }
                " are topic tags. Classification fields describe use context and compatibility. New packages must author "
                code { class: "{INLINE_CODE}", "usageContexts" }
                ", "
                code { class: "{INLINE_CODE}", "cliScope" }
                ", and "
                code { class: "{INLINE_CODE}", "technologyScope" }
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
                li { code { class: "{INLINE_CODE}", "usageContexts" }, ": ", code { class: "{INLINE_CODE}", "cycle" }, " when ", code { class: "{INLINE_CODE}", "category" }, " is ", code { class: "{INLINE_CODE}", "cycle" }, ", otherwise ", code { class: "{INLINE_CODE}", "standalone" }, "." }
                li { code { class: "{INLINE_CODE}", "cliScope" }, ": ", code { class: "{INLINE_CODE}", "any" }, " when ", code { class: "{INLINE_CODE}", "toolIds" }, " is empty or omitted; ", code { class: "{INLINE_CODE}", "specific" }, " with those ids when ", code { class: "{INLINE_CODE}", "toolIds" }, " is non-empty." }
                li { code { class: "{INLINE_CODE}", "technologyScope" }, ": ", code { class: "{INLINE_CODE}", "general" }, " when omitted." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Package metadata sidecar" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "Every package carries a "
                code { class: "{INLINE_CODE}", "packages/{{slug}}/s_e_e_package.json" }
                "; the build cross-checks it against the catalog entry (id, slug, category, name, description, labels, dependencies, toolIds, classification fields, releases)."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "{CATEGORIES_HEADING}" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "Each category has a fixed payload shape and install destination:" }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "workflow" }, " - ", code { class: "{INLINE_CODE}", ".s_e_e/workflows/definitions/{{slug}}.json" }, "." }
                li { code { class: "{INLINE_CODE}", "prompt" }, " - ", code { class: "{INLINE_CODE}", ".s_e_e/prompts/{{file}}.json" }, "." }
                li { code { class: "{INLINE_CODE}", "skill" }, " - ", code { class: "{INLINE_CODE}", ".agents/skills/{{slug}}/SKILL.md" }, "." }
                li { code { class: "{INLINE_CODE}", "rule" }, " - ", code { class: "{INLINE_CODE}", ".cursor/rules/{{slug}}.mdc" }, "." }
                li { code { class: "{INLINE_CODE}", "template" }, " - under ", code { class: "{INLINE_CODE}", "templates/" }, "." }
                li { code { class: "{INLINE_CODE}", "command" }, " - ", code { class: "{INLINE_CODE}", ".s_e_e/commands/{{id}}.json" }, " (to path must match)." }
                li { code { class: "{INLINE_CODE}", "bundle" }, " - no payload; ", code { class: "{INLINE_CODE}", "files[]" }, " installs members." }
                li { code { class: "{INLINE_CODE}", "cycle" }, " - ", code { class: "{INLINE_CODE}", ".s_e_e/cycles/{{id}}.json" }, "." }
            }
            h2 { id: "install-paths", class: "mt-8 {SECTION_TITLE}", "{INSTALL_PATHS_HEADING}" }
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
            h2 { class: "mt-8 {SECTION_TITLE}", "Stacks" }
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
            h2 { class: "mt-8 {SECTION_TITLE}", "Validation" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "The Rust catalog validator enforces cross-references between scopes and registries. Run "
                code { class: "{INLINE_CODE}", "mise run catalog" }
                " to validate payloads and refresh the manifest, or "
                code { class: "{INLINE_CODE}", "mise run validate" }
                " (no writes) in CI."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
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
            h2 { class: "mt-8 {SECTION_TITLE}", "Package metadata sidecar schema" }
            SchemaBlockByName { filename: "package-meta.schema.json" }
        }
    }
}
