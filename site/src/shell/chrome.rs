use chrono::Datelike;
use dioxus::prelude::*;
use dioxus_router::use_route;
use s_e_e_gui_kit::{
    brand::APP_DISPLAY_NAME, components::garunski_credit::GarunskiCredit, icons::Icon,
};
use s_e_e_shapes::Theme;

use super::AppLink;
use crate::library_href;
use crate::nav::{is_primary_current, shell_layout_classes, HEADER_CATALOG, PRIMARY_NAV};
use crate::pages::page_links::{AUTHORING_GUIDES, AUTHORING_PUBLISH_LINK};
use crate::routes::Route;
use crate::theme::{
    apply_theme_to_html, next_theme, read_cached_theme, theme_storage_label, write_cached_theme,
};

const NAV_LINK: &str = "library-nav-link";
const THEME_TRIGGER: &str = "library-theme-trigger";
const LIBRARY_COPYRIGHT_NAME: &str = "Garunski LLC";
const MARKETING_SITE: &str = "https://garunski.github.io/s_e_e_site";
const LIBRARY_CATALOG: &str = "https://garunski.github.io/s_e_e_library/";
const BETA_URL: &str = "https://garunski.github.io/s_e_e_site/join-beta/";

#[derive(Clone, Copy, PartialEq, Eq)]
struct FooterShellLink {
    label: &'static str,
    href: &'static str,
}

const FOOTER_SHELL_LINKS: &[FooterShellLink] = &[
    FooterShellLink {
        label: "Home",
        href: "https://garunski.github.io/s_e_e_site/",
    },
    FooterShellLink {
        label: "Learn",
        href: "https://garunski.github.io/s_e_e_site/learn/",
    },
    FooterShellLink {
        label: "Official Library",
        href: LIBRARY_CATALOG,
    },
    FooterShellLink {
        label: "Join the beta",
        href: BETA_URL,
    },
];

#[must_use]
pub fn route_current_path(route: &Route) -> &'static str {
    match route {
        Route::Home {} => "/",
        Route::AuthoringIndex {} => "/authoring/",
        Route::AuthoringWorkflows {} => "/authoring/workflows/",
        Route::AuthoringPrompts {} => "/authoring/prompts/",
        Route::AuthoringSkills {} => "/authoring/skills/",
        Route::AuthoringCommands {} => "/authoring/commands/",
        Route::AuthoringBundles {} => "/authoring/bundles/",
        Route::AuthoringCycles {} => "/authoring/cycles/",
        Route::AuthoringPublish {} => "/authoring/publish/",
        Route::SchemaIndex {} => "/schema/",
        Route::SchemaCatalog {} => "/schema/catalog/",
        Route::SchemaWorkflow {} => "/schema/workflow/",
        Route::SchemaPrompt {} => "/schema/prompt/",
        Route::SchemaSkill {} => "/schema/skill/",
        Route::SchemaCommand {} => "/schema/command/",
        Route::SchemaRuleTemplate {} => "/schema/rule-template/",
        Route::SchemaBundle {} => "/schema/bundle/",
        Route::SchemaCycle {} => "/schema/cycle/",
        Route::SchemaHubConfig {} => "/schema/hub-config/",
        Route::SchemaGlobalConfig {} => "/schema/global-config/",
        Route::SchemaAppSettings {} => "/schema/app-settings/",
        Route::SchemaRoutingRules {} => "/schema/routing-rules/",
        Route::SchemaStoriesConfig {} => "/schema/stories-config/",
        Route::SchemaOrchestratorPolicy {} => "/schema/orchestrator-policy/",
        Route::SchemaSchedule {} => "/schema/schedule/",
        Route::SchemaScheduleRuleSet {} => "/schema/schedule-rule-set/",
        Route::SchemaResearch {} => "/schema/knowledge-research/",
        Route::NotFound { .. } => "/404",
    }
}

#[component]
fn LibraryThemeControl() -> Element {
    let mut theme = use_signal(|| read_cached_theme().unwrap_or(Theme::System));
    let icon = match theme() {
        Theme::System => "computer-desktop",
        Theme::Light => "sun",
        Theme::Dark => "moon",
    };
    let label = format!("Theme: {}", theme_storage_label(&theme()));
    let accessible_label = format!(
        "{label}; change to {}",
        theme_storage_label(&next_theme(&theme()))
    );

    use_effect(move || {
        apply_theme_to_html(&theme());
    });

    rsx! {
        button {
                r#type: "button",
                class: THEME_TRIGGER,
                aria_label: accessible_label,
                title: label,
                onclick: move |_| {
                    let next = next_theme(&theme());
                    theme.set(next.clone());
                    write_cached_theme(&next);
                    apply_theme_to_html(&next);
                },
                Icon {
                    name: icon.to_string(),
                    class: Some("shrink-0".to_string()),
                    variant: Some("outline".to_string()),
                }
        }
    }
}

#[component]
pub fn LibraryMasthead(current_path: String) -> Element {
    let current = current_path.as_str();
    rsx! {
        header {
            "data-testid": "library-shell-header",
            class: "library-header",
            div {
                class: "library-masthead",
                a {
                    href: library_href("/"),
                    class: "library-brand",
                    aria_label: "S.E.E. Library home",
                    img {
                        src: asset!("/assets/branding/logo.svg"),
                        width: "44",
                        height: "44",
                        alt: "",
                    }
                    span { class: "library-brand-label",
                        span { {APP_DISPLAY_NAME} }
                        small { "Library" }
                    }
                }
            nav {
                class: "library-primary-nav",
                aria_label: "Library navigation",
                for link in PRIMARY_NAV {
                    if link.file || link.external {
                        a {
                            class: NAV_LINK,
                            href: library_href(link.path),
                            aria_current: if is_primary_current(link.path, current) { "page" } else { "false" },
                            "{link.label}"
                        }
                    } else {
                        LibraryNavRouterLink {
                            path: link.path,
                            label: link.label,
                            current: current_path.clone(),
                        }
                    }
                }
                a {
                    class: NAV_LINK,
                    href: library_href(HEADER_CATALOG.path),
                    "catalog.json"
                }
            }
            div { class: "library-header-theme",
                LibraryThemeControl {}
            }
            }
        }
    }
}

#[component]
fn LibraryNavRouterLink(path: &'static str, label: &'static str, current: String) -> Element {
    let active = is_primary_current(path, current.as_str());
    let class = NAV_LINK;
    let aria_current = if active { "page" } else { "false" };
    rsx! {
        AppLink {
            path,
            class,
            aria_current,
            {label}
        }
    }
}

#[component]
fn AuthoringNav(current: &'static str) -> Element {
    let links = AUTHORING_GUIDES
        .iter()
        .map(|guide| (guide.path, guide.label))
        .chain(std::iter::once((
            AUTHORING_PUBLISH_LINK.path,
            AUTHORING_PUBLISH_LINK.label,
        )))
        .map(|(path, label)| {
            let aria_current = if current == path { "page" } else { "false" };
            (path, label, aria_current)
        });

    rsx! {
        nav { class: "library-section-nav", aria_label: "Authoring sections",
            for (path, label, aria_current) in links {
                AppLink {
                    key: "{path}",
                    path,
                    aria_current,
                    "{label}"
                }
            }
        }
    }
}

#[must_use]
pub fn library_copyright_line(year: i32) -> String {
    format!("© {year} {LIBRARY_COPYRIGHT_NAME}")
}

#[component]
pub fn LibraryFooter() -> Element {
    let copyright = library_copyright_line(chrono::Local::now().year());
    let inner = shell_layout_classes().content_inner;
    rsx! {
        footer {
            "data-testid": "library-shell-footer",
            class: "dark mt-auto w-full border-t border-zinc-700 bg-zinc-900 text-zinc-400",
            div {
                class: "{inner} pt-12 pb-6 md:pt-20",
                div {
                    class: "grid gap-[clamp(3rem,7vw,7rem)] min-[801px]:grid-cols-[minmax(0,1.2fr)_minmax(0,2fr)]",
                    div {
                        a {
                            class: "inline-flex items-center gap-2.5 text-[1.15rem] font-semibold tracking-[0.12em] text-white no-underline",
                            href: MARKETING_SITE,
                            rel: "noreferrer",
                            aria_label: "S.E.E. home",
                            img {
                                src: asset!("/assets/branding/logo.svg"),
                                width: "44",
                                height: "44",
                                alt: "",
                            }
                            span { {APP_DISPLAY_NAME} }
                        }
                        p {
                            class: "mt-5 max-w-sm text-[0.95rem] leading-relaxed text-zinc-400",
                            "A local workspace for planning, agent workflows, and project knowledge."
                        }
                    }
                    nav {
                        aria_label: "Footer navigation",
                        class: "flex flex-wrap content-start items-start gap-x-6 gap-y-3",
                        for link in FOOTER_SHELL_LINKS {
                            a {
                                key: "{link.href}",
                                class: "text-[0.95rem] text-zinc-300 no-underline hover:text-sky-400",
                                href: link.href,
                                rel: "noreferrer",
                                "{link.label}"
                            }
                        }
                    }
                }
                div {
                    class: "mt-12 flex flex-col items-center gap-1 border-t border-zinc-700 pt-5 text-center text-sm tracking-wide text-zinc-400",
                    p { "{copyright}" }
                    GarunskiCredit {}
                }
            }
        }
    }
}

#[component]
pub fn LibraryShell() -> Element {
    let route = use_route::<Route>();
    let current_path = route_current_path(&route);
    let layout = shell_layout_classes();
    let home = matches!(route, Route::Home {});
    let display_font = asset!("/assets/fonts/autour-one-latin-400.woff2");

    rsx! {
        document::Style {
            "@font-face {{ font-family: 'Autour One'; font-style: normal; font-weight: 400; font-display: swap; src: url('{display_font}') format('woff2'); }}"
        }
        div {
            id: "top",
            class: layout.root,
            a {
                class: "sr-only focus:not-sr-only focus:absolute focus:left-4 focus:top-4 focus:z-50 focus:rounded-md focus:bg-white focus:px-3 focus:py-2 focus:text-sm focus:shadow dark:focus:bg-zinc-900",
                href: "#library-page",
                "Skip to content"
            }
            LibraryMasthead { current_path: current_path.to_string() }
            if current_path.starts_with("/authoring/") {
                AuthoringNav { current: current_path }
            }
            main {
                id: "library-page",
                tabindex: "-1",
                "data-testid": "library-shell-main",
                class: layout.main,
                if home {
                    Outlet::<Route> {}
                } else {
                    div {
                        class: layout.content_inner,
                        Outlet::<Route> {}
                    }
                }
            }
            LibraryFooter {}
        }
    }
}

#[cfg(test)]
#[path = "chrome_tests.rs"]
mod chrome_tests;
