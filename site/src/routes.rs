use dioxus::prelude::*;

use crate::pages::{
    LibraryAuthoringBundles, LibraryAuthoringCommands, LibraryAuthoringCycles,
    LibraryAuthoringIndex, LibraryAuthoringPrompts, LibraryAuthoringPublish,
    LibraryAuthoringSkills, LibraryAuthoringWorkflows, LibraryHome, LibraryNotFound,
    LibrarySchemaAppSettings, LibrarySchemaBundle, LibrarySchemaCommand, LibrarySchemaCycle,
    LibrarySchemaGlobalConfig, LibrarySchemaHubConfig, LibrarySchemaIndex,
    LibrarySchemaOrchestratorPolicy, LibrarySchemaPrompt, LibrarySchemaRoutingRules,
    LibrarySchemaRuleTemplate, LibrarySchemaSchedule, LibrarySchemaScheduleRuleSet,
    LibrarySchemaSkill, LibrarySchemaStoriesConfig, LibrarySchemaWorkflow,
};
use crate::shell::LibraryShell;

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LibraryShell)]
        #[route("/")]
        Home {},
        #[route("/authoring/")]
        AuthoringIndex {},
        #[route("/authoring/workflows/")]
        AuthoringWorkflows {},
        #[route("/authoring/prompts/")]
        AuthoringPrompts {},
        #[route("/authoring/skills/")]
        AuthoringSkills {},
        #[route("/authoring/commands/")]
        AuthoringCommands {},
        #[route("/authoring/bundles/")]
        AuthoringBundles {},
        #[route("/authoring/cycles/")]
        AuthoringCycles {},
        #[route("/authoring/publish/")]
        AuthoringPublish {},
        #[route("/schema/")]
        SchemaIndex {},
        #[route("/schema/workflow/")]
        SchemaWorkflow {},
        #[route("/schema/prompt/")]
        SchemaPrompt {},
        #[route("/schema/skill/")]
        SchemaSkill {},
        #[route("/schema/command/")]
        SchemaCommand {},
        #[route("/schema/rule-template/")]
        SchemaRuleTemplate {},
        #[route("/schema/bundle/")]
        SchemaBundle {},
        #[route("/schema/cycle/")]
        SchemaCycle {},
        #[route("/schema/hub-config/")]
        SchemaHubConfig {},
        #[route("/schema/global-config/")]
        SchemaGlobalConfig {},
        #[route("/schema/app-settings/")]
        SchemaAppSettings {},
        #[route("/schema/routing-rules/")]
        SchemaRoutingRules {},
        #[route("/schema/stories-config/")]
        SchemaStoriesConfig {},
        #[route("/schema/orchestrator-policy/")]
        SchemaOrchestratorPolicy {},
        #[route("/schema/schedule/")]
        SchemaSchedule {},
        #[route("/schema/schedule-rule-set/")]
        SchemaScheduleRuleSet {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

#[must_use]
pub fn route_for_path(href: &str) -> Option<Route> {
    if href.starts_with('#') || href.contains("://") {
        return None;
    }
    let path = href.split(['#', '?']).next().unwrap_or(href);
    Some(match path {
        "/" => Route::Home {},
        "/authoring/" => Route::AuthoringIndex {},
        "/authoring/workflows/" => Route::AuthoringWorkflows {},
        "/authoring/prompts/" => Route::AuthoringPrompts {},
        "/authoring/skills/" => Route::AuthoringSkills {},
        "/authoring/commands/" => Route::AuthoringCommands {},
        "/authoring/bundles/" => Route::AuthoringBundles {},
        "/authoring/cycles/" => Route::AuthoringCycles {},
        "/authoring/publish/" => Route::AuthoringPublish {},
        "/schema/" => Route::SchemaIndex {},
        "/schema/workflow/" => Route::SchemaWorkflow {},
        "/schema/prompt/" => Route::SchemaPrompt {},
        "/schema/skill/" => Route::SchemaSkill {},
        "/schema/command/" => Route::SchemaCommand {},
        "/schema/rule-template/" => Route::SchemaRuleTemplate {},
        "/schema/bundle/" => Route::SchemaBundle {},
        "/schema/cycle/" => Route::SchemaCycle {},
        "/schema/hub-config/" => Route::SchemaHubConfig {},
        "/schema/global-config/" => Route::SchemaGlobalConfig {},
        "/schema/app-settings/" => Route::SchemaAppSettings {},
        "/schema/routing-rules/" => Route::SchemaRoutingRules {},
        "/schema/stories-config/" => Route::SchemaStoriesConfig {},
        "/schema/orchestrator-policy/" => Route::SchemaOrchestratorPolicy {},
        "/schema/schedule/" => Route::SchemaSchedule {},
        "/schema/schedule-rule-set/" => Route::SchemaScheduleRuleSet {},
        _ => return None,
    })
}

#[component]
fn Home() -> Element {
    rsx! {
        LibraryHome {}
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        LibraryNotFound { segments }
    }
}

#[component]
fn AuthoringIndex() -> Element {
    rsx! {
        LibraryAuthoringIndex {}
    }
}

#[component]
fn AuthoringWorkflows() -> Element {
    rsx! {
        LibraryAuthoringWorkflows {}
    }
}

#[component]
fn AuthoringPrompts() -> Element {
    rsx! {
        LibraryAuthoringPrompts {}
    }
}

#[component]
fn AuthoringSkills() -> Element {
    rsx! {
        LibraryAuthoringSkills {}
    }
}

#[component]
fn AuthoringCommands() -> Element {
    rsx! {
        LibraryAuthoringCommands {}
    }
}

#[component]
fn AuthoringBundles() -> Element {
    rsx! {
        LibraryAuthoringBundles {}
    }
}

#[component]
fn AuthoringCycles() -> Element {
    rsx! {
        LibraryAuthoringCycles {}
    }
}

#[component]
fn AuthoringPublish() -> Element {
    rsx! {
        LibraryAuthoringPublish {}
    }
}

#[component]
fn SchemaIndex() -> Element {
    rsx! {
        LibrarySchemaIndex {}
    }
}

#[component]
fn SchemaWorkflow() -> Element {
    rsx! {
        LibrarySchemaWorkflow {}
    }
}

#[component]
fn SchemaPrompt() -> Element {
    rsx! {
        LibrarySchemaPrompt {}
    }
}

#[component]
fn SchemaSkill() -> Element {
    rsx! {
        LibrarySchemaSkill {}
    }
}

#[component]
fn SchemaCommand() -> Element {
    rsx! {
        LibrarySchemaCommand {}
    }
}

#[component]
fn SchemaRuleTemplate() -> Element {
    rsx! {
        LibrarySchemaRuleTemplate {}
    }
}

#[component]
fn SchemaBundle() -> Element {
    rsx! {
        LibrarySchemaBundle {}
    }
}

#[component]
fn SchemaCycle() -> Element {
    rsx! {
        LibrarySchemaCycle {}
    }
}

#[component]
fn SchemaHubConfig() -> Element {
    rsx! {
        LibrarySchemaHubConfig {}
    }
}

#[component]
fn SchemaGlobalConfig() -> Element {
    rsx! {
        LibrarySchemaGlobalConfig {}
    }
}

#[component]
fn SchemaAppSettings() -> Element {
    rsx! {
        LibrarySchemaAppSettings {}
    }
}

#[component]
fn SchemaRoutingRules() -> Element {
    rsx! {
        LibrarySchemaRoutingRules {}
    }
}

#[component]
fn SchemaStoriesConfig() -> Element {
    rsx! {
        LibrarySchemaStoriesConfig {}
    }
}

#[component]
fn SchemaOrchestratorPolicy() -> Element {
    rsx! {
        LibrarySchemaOrchestratorPolicy {}
    }
}

#[component]
fn SchemaSchedule() -> Element {
    rsx! {
        LibrarySchemaSchedule {}
    }
}

#[component]
fn SchemaScheduleRuleSet() -> Element {
    rsx! {
        LibrarySchemaScheduleRuleSet {}
    }
}

#[cfg(test)]
#[path = "routes_tests.rs"]
mod routes_tests;
