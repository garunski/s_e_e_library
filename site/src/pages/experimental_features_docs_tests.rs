use crate::routes::Route;
use dioxus::history::{provide_history_context, MemoryHistory};
use dioxus::prelude::*;

fn render(path: &'static str) -> String {
    fn test_app(path: &'static str) -> Element {
        use_hook(move || {
            provide_history_context(std::rc::Rc::new(MemoryHistory::with_initial_path(path)))
        });
        rsx! { Router::<Route> {} }
    }
    let mut dom = VirtualDom::new_with_props(test_app, path);
    dom.rebuild_in_place();
    dioxus::ssr::render(&dom)
}

#[test]
fn experimental_features_docs_public_rendered_pages_and_llms_sections() {
    let llms = include_str!("../../../public/llms.txt");
    for (path, facts) in [
        (
            "/schema/hub-config/",
            vec![
                "project.experimental_features",
                "orchestrator_cycle",
                "knowledge_cycle",
                "planning_cycle",
                "knowledge_research",
                "global_chat",
                "configured",
                "effective",
                "conflict_token",
                "PATCH",
                "changes",
                "HTTP 409",
                "union",
                "bound project",
                "mixed-project",
                "target project afresh",
                "EXPERIMENTAL_FEATURE_DISABLED",
                "stop controls",
                "/v1/chat/completions",
                "workflow-space defaults",
            ],
        ),
        (
            "/schema/knowledge-research/",
            vec![
                "knowledge_research",
                "Experimental",
                "immutable editions",
                "EXPERIMENTAL_FEATURE_DISABLED",
                "knowledge_research_profile_get",
                "knowledge_research_list",
                "explicit target project",
                "no separate Research edition or search HTTP endpoint",
            ],
        ),
        (
            "/authoring/cycles/",
            vec![
                "Experimental",
                "orchestrator_cycle",
                "knowledge_cycle",
                "planning_cycle",
                "default to false",
                "saved run controls",
                "does not cancel admitted work",
                "stop controls",
                "EXPERIMENTAL_FEATURE_DISABLED",
                "Manual story and milestone planning",
                "no Planning cycle-control HTTP endpoint or manual-stage HTTP route",
                "Library package storage",
            ],
        ),
        (
            "/authoring/",
            vec![
                "Experimental",
                "default to off",
                "reusable workflows remain available",
                "target project",
                "Experimental configuration and API contract",
                "Research profiles, availability and MCP examples",
            ],
        ),
    ] {
        let html = render(path);
        let marker = format!("Source: {path}\n");
        let section = llms
            .split(&marker)
            .nth(1)
            .unwrap()
            .split("\nSource: ")
            .next()
            .unwrap();
        for fact in facts {
            assert!(html.contains(fact), "render {path} missing {fact}");
            assert!(section.contains(fact), "llms {path} missing {fact}");
        }
    }
}

#[test]
fn experimental_features_docs_public_core_guides_and_reference_links() {
    for (path, fact) in [
        ("/schema/routing-rules/", "task_profiles"),
        ("/schema/stories-config/", "statuses"),
        ("/authoring/workflows/", "definition.json"),
    ] {
        assert!(render(path).contains(fact), "core guide {path}");
    }
    let index = render("/authoring/");
    assert!(index.contains("/schema/hub-config/"));
    assert!(index.contains("/schema/knowledge-research/"));
    for path in [
        "/schema/hub-config/",
        "/schema/knowledge-research/",
        "/authoring/cycles/",
        "/authoring/",
    ] {
        assert!(crate::documentation_route_paths().contains(&path));
    }
}
