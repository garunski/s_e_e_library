use dioxus::prelude::*;

use crate::pages::authoring_common::INDEX_LINK_LIST;
use crate::pages::schema_common::NamedSchemaLink;
use crate::routes::Route;

#[component]
pub fn LibrarySchemaIndex() -> Element {
    rsx! {
        document::Title { "Schemas - S.E.E. Library" }
        div { class: "library-home",
            div { class: "library-doc-page",
                h1 { "S.E.E. schemas" }
                div { class: "library-doc-sections",
                    section {
                        h2 { id: "catalog-format", class: "scroll-mt-24", "Library" }
                        ul { class: INDEX_LINK_LIST,
                            NamedSchemaLink {
                                route: Route::SchemaCatalog {},
                                label: "Catalog schema",
                                description: "Manifest, package metadata, and install paths.",
                            }
                        }
                    }
                    section {
                        h2 { id: "package-schemas", class: "scroll-mt-24", "Package schemas" }
                        ul { class: INDEX_LINK_LIST,
                            NamedSchemaLink {
                                route: Route::SchemaWorkflow {},
                                label: "Workflow",
                                description: "definition.json envelope and engine content.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaPrompt {},
                                label: "Prompt",
                                description: "id, name, and content.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaSkill {},
                                label: "Skill",
                                description: "SKILL.md frontmatter.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaCommand {},
                                label: "Command",
                                description: "binary, args, params, result, execution.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaRuleTemplate {},
                                label: "Rule and template",
                                description: "Lightweight file-only categories.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaBundle {},
                                label: "Bundle",
                                description: "Install map with no payload of its own.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaCycle {},
                                label: "Cycle",
                                description: "Stages, stores, and triggers for a host loop.",
                            }
                        }
                    }
                    section {
                        h2 { id: "configuration", class: "scroll-mt-24", "Configuration" }
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "S.E.E. validates these documents on save."
                        }
                        ul { class: INDEX_LINK_LIST,
                            NamedSchemaLink {
                                route: Route::SchemaHubConfig {},
                                label: "Hub config",
                                description: ".s_e_e/config.json at the hub data root.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaGlobalConfig {},
                                label: "Global config",
                                description: "~/.s_e_e/config.json.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaAppSettings {},
                                label: "App settings",
                                description: "~/.s_e_e/settings.json.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaRoutingRules {},
                                label: "Routing rules",
                                description: "llm.routing in hub config.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaStoriesConfig {},
                                label: "Stories config",
                                description: "stories in hub config.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaOrchestratorPolicy {},
                                label: "Orchestrator policy",
                                description: "orchestrator in hub config.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaSchedule {},
                                label: "Schedule",
                                description: ".s_e_e/workflows/schedules/{{id}}.json.",
                            }
                            NamedSchemaLink {
                                route: Route::SchemaScheduleRuleSet {},
                                label: "Schedule rule set",
                                description: ".s_e_e/workflows/schedule_rule_sets/{{id}}.json.",
                            }
                        }
                    }
                    section {
                        h2 { id: "knowledge", class: "scroll-mt-24", "Knowledge" }
                        p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                            "Component updates at " code { "/api/projects/{{project}}/knowledge/components/{{id}}" }
                            " require the current conflict token. PUT accepts body, verified_against, and conflict_token. A stale token or superseded note returns 409 WORKSPACE_CONFLICT with the current note and token without creating a revision. Success creates a new current note: use its returned id and token for subsequent updates."
                        }
                        ul { class: INDEX_LINK_LIST,
                            NamedSchemaLink {
                                route: Route::SchemaResearch {},
                                label: "Knowledge Research",
                                description: "Eleven HTTP/MCP operations, wire models, errors, and profile version contract.",
                            }
                        }
                    }
                }
            }
        }
    }
}
