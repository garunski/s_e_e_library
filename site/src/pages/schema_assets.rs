//! Build-time JSON Schema bytes embedded from `public/schema/`.

pub const CATALOG_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/catalog.schema.json");
pub const PACKAGE_META_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/package-meta.schema.json");
pub const WORKFLOW_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/workflow.schema.json");
pub const WORKFLOW_DEFINITION_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/workflow-definition.schema.json");
pub const WORKFLOW_ENGINE_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/workflow-engine.schema.json");
pub const PROMPT_SCHEMA_JSON: &str = include_str!("../../../public/schema/prompt.schema.json");
pub const SKILL_FRONTMATTER_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/skill-frontmatter.schema.json");
pub const COMMAND_SCHEMA_JSON: &str = include_str!("../../../public/schema/command.schema.json");
pub const BUNDLE_PACKAGE_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/bundle-package.schema.json");
pub const CYCLE_DOCUMENT_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/cycle-document.schema.json");
pub const HUB_CONFIG_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/hub-config.schema.json");
pub const GLOBAL_CONFIG_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/global-config.schema.json");
pub const APP_SETTINGS_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/app-settings.schema.json");
pub const ROUTING_RULES_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/routing-rules.schema.json");
pub const STORIES_CONFIG_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/stories-config.schema.json");
pub const ORCHESTRATOR_POLICY_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/orchestrator-policy.schema.json");
pub const SCHEDULE_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/schedule.schema.json");
pub const SCHEDULE_RULE_SET_SCHEMA_JSON: &str =
    include_str!("../../../public/schema/schedule-rule-set.schema.json");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaDownload {
    pub filename: &'static str,
    pub json: &'static str,
}

/// Schemas embedded on package schema pages (download link + pre block).
pub const PACKAGE_PAGE_SCHEMAS: [SchemaDownload; 10] = [
    SchemaDownload {
        filename: "catalog.schema.json",
        json: CATALOG_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "package-meta.schema.json",
        json: PACKAGE_META_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "workflow.schema.json",
        json: WORKFLOW_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "workflow-definition.schema.json",
        json: WORKFLOW_DEFINITION_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "workflow-engine.schema.json",
        json: WORKFLOW_ENGINE_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "prompt.schema.json",
        json: PROMPT_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "skill-frontmatter.schema.json",
        json: SKILL_FRONTMATTER_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "command.schema.json",
        json: COMMAND_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "bundle-package.schema.json",
        json: BUNDLE_PACKAGE_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "cycle-document.schema.json",
        json: CYCLE_DOCUMENT_SCHEMA_JSON,
    },
];

/// Hub and scheduling schemas on configuration reference pages.
pub const CONFIG_PAGE_SCHEMAS: [SchemaDownload; 8] = [
    SchemaDownload {
        filename: "hub-config.schema.json",
        json: HUB_CONFIG_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "global-config.schema.json",
        json: GLOBAL_CONFIG_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "app-settings.schema.json",
        json: APP_SETTINGS_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "routing-rules.schema.json",
        json: ROUTING_RULES_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "stories-config.schema.json",
        json: STORIES_CONFIG_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "orchestrator-policy.schema.json",
        json: ORCHESTRATOR_POLICY_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "schedule.schema.json",
        json: SCHEDULE_SCHEMA_JSON,
    },
    SchemaDownload {
        filename: "schedule-rule-set.schema.json",
        json: SCHEDULE_RULE_SET_SCHEMA_JSON,
    },
];

#[must_use]
pub fn all_embedded_schemas() -> impl Iterator<Item = &'static SchemaDownload> {
    PACKAGE_PAGE_SCHEMAS
        .iter()
        .chain(CONFIG_PAGE_SCHEMAS.iter())
}

#[must_use]
pub fn schema_download(filename: &str) -> Option<&'static SchemaDownload> {
    all_embedded_schemas().find(|entry| entry.filename == filename)
}

#[cfg(test)]
#[path = "schema_assets_tests.rs"]
mod schema_assets_tests;
