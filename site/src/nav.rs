#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavLink {
    pub path: &'static str,
    pub label: &'static str,
    pub file: bool,
    pub external: bool,
}

pub const PRIMARY_NAV: &[NavLink] = &[
    NavLink {
        path: "/authoring/",
        label: "Authoring",
        file: false,
        external: false,
    },
    NavLink {
        path: "/schema/",
        label: "Schema",
        file: false,
        external: false,
    },
    NavLink {
        path: "/llms.txt",
        label: "llms.txt",
        file: true,
        external: false,
    },
];

pub const HEADER_CATALOG: NavLink = NavLink {
    path: "/catalog.json",
    label: "Open catalog.json",
    file: true,
    external: false,
};

fn section_root(href: &str) -> &'static str {
    match href.split('/').nth(1) {
        Some("authoring") => "/authoring/",
        Some("schema") => "/schema/",
        _ => "/",
    }
}

#[must_use]
pub fn is_primary_current(href: &str, current: &str) -> bool {
    if href.ends_with(".json") || href.ends_with(".txt") {
        return href == current;
    }
    let section = section_root(href);
    current == section || current.starts_with(section)
}

#[must_use]
pub fn shell_layout_classes() -> ShellLayoutClasses {
    ShellLayoutClasses {
        root:
            "flex min-h-screen w-full min-w-0 flex-col overflow-x-hidden bg-white dark:bg-zinc-950",
        content_inner: "app-content-width mx-auto w-full min-w-0 px-4 sm:px-6",
        main: "flex-1 min-w-0 py-6 sm:py-8",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShellLayoutClasses {
    pub root: &'static str,
    pub content_inner: &'static str,
    pub main: &'static str,
}

#[cfg(test)]
#[path = "nav_tests.rs"]
mod nav_tests;
