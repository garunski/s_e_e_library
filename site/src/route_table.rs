//! Documentation route paths aligned with `scripts/authoring-pages.mjs`.

pub const AUTHORING_SLUGS: &[&str] = &[
    "workflows",
    "prompts",
    "skills",
    "commands",
    "bundles",
    "cycles",
    "publish",
];

pub const SCHEMA_SLUGS: &[&str] = &[
    "workflow",
    "prompt",
    "skill",
    "command",
    "rule-template",
    "bundle",
    "cycle",
    "hub-config",
    "global-config",
    "app-settings",
    "routing-rules",
    "stories-config",
    "orchestrator-policy",
    "schedule",
    "schedule-rule-set",
];

/// Every public documentation URL path (leading slash, trailing slash except home is `/`).
pub fn documentation_route_paths() -> Vec<&'static str> {
    let mut paths = Vec::with_capacity(25);
    paths.push("/");
    paths.push("/authoring/");
    for slug in AUTHORING_SLUGS {
        paths.push(authoring_path(slug));
    }
    paths.push("/schema/");
    for slug in SCHEMA_SLUGS {
        paths.push(schema_path(slug));
    }
    paths
}

pub fn authoring_path(slug: &str) -> &'static str {
    match slug {
        "workflows" => "/authoring/workflows/",
        "prompts" => "/authoring/prompts/",
        "skills" => "/authoring/skills/",
        "commands" => "/authoring/commands/",
        "bundles" => "/authoring/bundles/",
        "cycles" => "/authoring/cycles/",
        "publish" => "/authoring/publish/",
        _ => "/authoring/",
    }
}

pub fn schema_path(slug: &str) -> &'static str {
    match slug {
        "workflow" => "/schema/workflow/",
        "prompt" => "/schema/prompt/",
        "skill" => "/schema/skill/",
        "command" => "/schema/command/",
        "rule-template" => "/schema/rule-template/",
        "bundle" => "/schema/bundle/",
        "cycle" => "/schema/cycle/",
        "hub-config" => "/schema/hub-config/",
        "global-config" => "/schema/global-config/",
        "app-settings" => "/schema/app-settings/",
        "routing-rules" => "/schema/routing-rules/",
        "stories-config" => "/schema/stories-config/",
        "orchestrator-policy" => "/schema/orchestrator-policy/",
        "schedule" => "/schema/schedule/",
        "schedule-rule-set" => "/schema/schedule-rule-set/",
        _ => "/schema/",
    }
}
