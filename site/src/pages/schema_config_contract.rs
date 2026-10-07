//! Machine-checkable routes and schema filenames for hub configuration pages (story-499).

use crate::route_table::{schema_path, SCHEMA_SLUGS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConfigSchemaPageContract {
    pub slug: &'static str,
    pub document_title: &'static str,
    pub schema_filename: &'static str,
    pub field_markers: &'static [&'static str],
}

pub const CONFIG_SCHEMA_PAGES: [ConfigSchemaPageContract; 8] = [
    ConfigSchemaPageContract {
        slug: "hub-config",
        document_title: "Hub config schema",
        schema_filename: "hub-config.schema.json",
        field_markers: &[
            "project",
            "overrides",
            "library",
            "stories",
            "llm.routing",
            "orchestrator",
        ],
    },
    ConfigSchemaPageContract {
        slug: "global-config",
        document_title: "Global config schema",
        schema_filename: "global-config.schema.json",
        field_markers: &["~/.s_e_e/config.json"],
    },
    ConfigSchemaPageContract {
        slug: "app-settings",
        document_title: "App settings schema",
        schema_filename: "app-settings.schema.json",
        field_markers: &["theme", "command_binary_allowlist", "command_secrets"],
    },
    ConfigSchemaPageContract {
        slug: "routing-rules",
        document_title: "Routing rules schema",
        schema_filename: "routing-rules.schema.json",
        field_markers: &["task_profiles", "cost_caps"],
    },
    ConfigSchemaPageContract {
        slug: "stories-config",
        document_title: "Stories config schema",
        schema_filename: "stories-config.schema.json",
        field_markers: &["project_name", "story_prefix", "default_create_status"],
    },
    ConfigSchemaPageContract {
        slug: "orchestrator-policy",
        document_title: "Orchestrator policy schema",
        schema_filename: "orchestrator-policy.schema.json",
        field_markers: &["auto_mode", "policy", "budgets"],
    },
    ConfigSchemaPageContract {
        slug: "schedule",
        document_title: "Schedule schema",
        schema_filename: "schedule.schema.json",
        field_markers: &["workflow_id", "entity_event", "story_rules"],
    },
    ConfigSchemaPageContract {
        slug: "schedule-rule-set",
        document_title: "Schedule rule set schema",
        schema_filename: "schedule-rule-set.schema.json",
        field_markers: &["schedule_rule_sets", "story_rules"],
    },
];

#[must_use]
pub fn config_schema_slugs_match_route_table() -> bool {
    let hub_slugs = &SCHEMA_SLUGS[7..];
    hub_slugs.len() == CONFIG_SCHEMA_PAGES.len()
        && CONFIG_SCHEMA_PAGES
            .iter()
            .zip(hub_slugs.iter())
            .all(|(page, slug)| page.slug == *slug)
}

#[must_use]
pub fn config_schema_paths() -> Vec<&'static str> {
    CONFIG_SCHEMA_PAGES
        .iter()
        .map(|page| schema_path(page.slug))
        .collect()
}

#[cfg(test)]
#[path = "schema_config_contract_tests.rs"]
mod schema_config_contract_tests;
