use crate::links::library_href_with_base;
use crate::route_table::{authoring_path, AUTHORING_SLUGS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageLink {
    pub label: &'static str,
    pub path: &'static str,
    pub external: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackageSummary {
    pub number: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub path: &'static str,
}

pub const HOME_PACKAGE_TYPES: [PackageSummary; 5] = [
    PackageSummary {
        number: "01",
        title: "Workflows",
        description: "Multi-step task definitions the engine runs against a project.",
        path: "/authoring/workflows/",
    },
    PackageSummary {
        number: "02",
        title: "Prompts",
        description: "Named system instruction referenced by workflows and commands.",
        path: "/authoring/prompts/",
    },
    PackageSummary {
        number: "03",
        title: "Skills",
        description: "Activatable guides that teach a worker a repeatable capability.",
        path: "/authoring/skills/",
    },
    PackageSummary {
        number: "04",
        title: "Commands",
        description: "Worker definitions that map a task to an external agent CLI.",
        path: "/authoring/commands/",
    },
    PackageSummary {
        number: "05",
        title: "Bundles",
        description: "Curated install maps that deliver several linked packages together.",
        path: "/authoring/bundles/",
    },
];

pub const HOME_UTILITY_LINKS: [PageLink; 4] = [
    PageLink {
        label: "catalog.json",
        path: "/catalog.json",
        external: false,
    },
    PageLink {
        label: "Authoring",
        path: "/authoring/",
        external: false,
    },
    PageLink {
        label: "Schema",
        path: "/schema/",
        external: false,
    },
    PageLink {
        label: "llms.txt",
        path: "/llms.txt",
        external: false,
    },
];

pub const AUTHORING_GUIDE_LINKS: [PageLink; 6] = [
    PageLink {
        label: "Workflows",
        path: "/authoring/workflows/",
        external: false,
    },
    PageLink {
        label: "Prompts",
        path: "/authoring/prompts/",
        external: false,
    },
    PageLink {
        label: "Skills",
        path: "/authoring/skills/",
        external: false,
    },
    PageLink {
        label: "Commands",
        path: "/authoring/commands/",
        external: false,
    },
    PageLink {
        label: "Bundles",
        path: "/authoring/bundles/",
        external: false,
    },
    PageLink {
        label: "Cycles",
        path: "/authoring/cycles/",
        external: false,
    },
];

/// Package schema routes linked from the catalog index (story-498).
pub const SCHEMA_PACKAGE_TYPE_LINKS: [PageLink; 7] = [
    PageLink {
        label: "Workflow",
        path: "/schema/workflow/",
        external: false,
    },
    PageLink {
        label: "Prompt",
        path: "/schema/prompt/",
        external: false,
    },
    PageLink {
        label: "Skill",
        path: "/schema/skill/",
        external: false,
    },
    PageLink {
        label: "Command",
        path: "/schema/command/",
        external: false,
    },
    PageLink {
        label: "Rule and template",
        path: "/schema/rule-template/",
        external: false,
    },
    PageLink {
        label: "Bundle",
        path: "/schema/bundle/",
        external: false,
    },
    PageLink {
        label: "Cycle",
        path: "/schema/cycle/",
        external: false,
    },
];

pub const AUTHORING_PUBLISH_LINK: PageLink = PageLink {
    label: "Publish",
    path: "/authoring/publish/",
    external: false,
};

#[must_use]
#[cfg_attr(not(test), allow(dead_code))]
pub fn resolve_page_link(base: &str, link: PageLink) -> String {
    if link.external {
        link.path.to_string()
    } else {
        library_href_with_base(base, link.path)
    }
}

#[must_use]
#[cfg_attr(not(test), allow(dead_code))]
pub fn home_link_hrefs(base: &str) -> Vec<String> {
    let mut hrefs = HOME_UTILITY_LINKS
        .iter()
        .map(|link| resolve_page_link(base, *link))
        .collect::<Vec<_>>();
    for summary in HOME_PACKAGE_TYPES {
        hrefs.push(library_href_with_base(base, summary.path));
    }
    hrefs.push(library_href_with_base(base, AUTHORING_PUBLISH_LINK.path));
    hrefs
}

#[must_use]
#[cfg_attr(not(test), allow(dead_code))]
pub fn authoring_index_link_hrefs(base: &str) -> Vec<String> {
    let mut hrefs = AUTHORING_GUIDE_LINKS
        .iter()
        .map(|link| resolve_page_link(base, *link))
        .collect::<Vec<_>>();
    hrefs.push(resolve_page_link(base, AUTHORING_PUBLISH_LINK));
    hrefs.push(library_href_with_base(base, "/schema/"));
    hrefs.push(library_href_with_base(base, "/catalog.json"));
    hrefs.push(library_href_with_base(base, "/llms.txt"));
    hrefs
}

#[must_use]
#[cfg_attr(not(test), allow(dead_code))]
pub fn authoring_guide_paths_match_route_table() -> bool {
    let slugs = [
        "workflows",
        "prompts",
        "skills",
        "commands",
        "bundles",
        "cycles",
    ];
    slugs.len() == AUTHORING_GUIDE_LINKS.len()
        && slugs
            .iter()
            .zip(AUTHORING_GUIDE_LINKS.iter())
            .all(|(slug, link)| link.path == authoring_path(slug))
        && AUTHORING_SLUGS.contains(&"publish")
}

#[cfg(test)]
#[path = "page_links_tests.rs"]
mod page_links_tests;
