use chrono::Datelike;
use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use dioxus_router::{use_route, Link};
use s_e_e_gui_kit::{
    brand::APP_DISPLAY_NAME,
    components::{
        button::{button_classes, ButtonSize, ButtonVariant},
        garunski_credit::GarunskiCredit,
        popover::{PopoverContent, PopoverRoot, PopoverTrigger},
    },
    icons::Icon,
};
use s_e_e_shapes::Theme;

use super::AppLink;
use crate::library_href;
use crate::nav::{is_primary_current, shell_layout_classes, HEADER_CATALOG, PRIMARY_NAV};
use crate::routes::Route;
use crate::theme::{
    apply_theme_to_html, detect_system_dark_mode, read_cached_theme, theme_appears_dark,
    write_cached_theme,
};

const MASTHEAD_LOGO_SIZE: &str = "h-7 w-7";
const SHELL_HEADER: &str =
    "sticky top-0 z-20 border-b border-zinc-200/80 bg-white dark:border-white/10 dark:bg-zinc-950";
const MASTHEAD_GRID: &str =
    "grid h-11 grid-cols-[minmax(0,1fr)_auto] items-center gap-x-3 max-md:gap-x-1.5";
const BRAND_LINK: &str =
    "flex min-w-0 items-center gap-2 rounded-lg py-0.5 outline-none transition hover:bg-zinc-50 focus-visible:ring-2 focus-visible:ring-sky-100 dark:hover:bg-zinc-900/40 dark:focus-visible:ring-sky-400/20";
const BRAND_LIBRARY: &str =
    "truncate text-xs font-medium text-zinc-500 dark:text-zinc-400 max-md:hidden";
const NAV_ROW: &str =
    "flex min-w-0 flex-wrap items-center gap-1 border-t border-zinc-200/80 py-2 dark:border-white/10 sm:flex-nowrap sm:overflow-x-auto sm:[scrollbar-width:none]";
const NAV_LINK: &str =
    "inline-flex shrink-0 items-center whitespace-nowrap rounded-md px-2.5 py-1.5 text-[13px] font-medium text-zinc-700 transition hover:text-zinc-900 focus-visible:ring-2 focus-visible:ring-sky-100 focus-visible:outline-none dark:text-zinc-300 dark:hover:text-white dark:focus-visible:ring-sky-400/20";
const NAV_ON: &str =
    "inline-flex shrink-0 items-center whitespace-nowrap rounded-md bg-sky-50 px-2.5 py-1.5 text-[13px] font-semibold text-sky-700 ring-1 ring-inset ring-sky-200/60 dark:bg-sky-950/40 dark:text-sky-300 dark:ring-sky-400/25";
const ICON_BTN: &str =
    "inline-flex h-8 w-8 shrink-0 items-center justify-center rounded-md border border-zinc-200/80 bg-white text-zinc-500 shadow-sm transition hover:border-sky-300 hover:text-sky-600 focus-visible:ring-2 focus-visible:ring-sky-100 focus-visible:outline-none dark:border-white/10 dark:bg-zinc-900 dark:hover:border-sky-400/40 dark:hover:text-sky-400 dark:focus-visible:ring-sky-400/20";
const THEME_MENU: &str =
    "flex min-w-[12rem] flex-col gap-1 rounded-lg border border-zinc-200/80 bg-white p-2 shadow-lg dark:border-white/10 dark:bg-zinc-900";
const THEME_OPTION: &str =
    "rounded-md px-3 py-2 text-left text-xs font-medium text-zinc-700 transition hover:bg-zinc-50 focus-visible:ring-2 focus-visible:ring-sky-100 focus-visible:outline-none dark:text-zinc-200 dark:hover:bg-zinc-800 dark:focus-visible:ring-sky-400/20";
const THEME_OPTION_ON: &str =
    "rounded-md bg-sky-50 px-3 py-2 text-left text-xs font-semibold text-sky-700 ring-1 ring-inset ring-sky-200/60 dark:bg-sky-950/40 dark:text-sky-300 dark:ring-sky-400/25";
const LIBRARY_COPYRIGHT_NAME: &str = "Garunski LLC";
const MARKETING_SITE: &str = "https://garunski.github.io/s_e_e_site";
const LIBRARY_CATALOG: &str = "https://garunski.github.io/s_e_e_library/";
const BETA_URL: &str = "https://see.fang.garunski.com/";

#[derive(Clone, Copy, PartialEq, Eq)]
struct FooterShellLink {
    label: &'static str,
    href: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct FooterShellGroup {
    aria_label: &'static str,
    label: &'static str,
    links: &'static [FooterShellLink],
}

const FOOTER_SHELL_GROUPS: &[FooterShellGroup] = &[
    FooterShellGroup {
        aria_label: "S.E.E. site links",
        label: "S.E.E.",
        links: &[
            FooterShellLink {
                label: "Home",
                href: "https://garunski.github.io/s_e_e_site/",
            },
            FooterShellLink {
                label: "How it works",
                href: "https://garunski.github.io/s_e_e_site/how-it-works/",
            },
            FooterShellLink {
                label: "Evidence",
                href: "https://garunski.github.io/s_e_e_site/evidence/",
            },
            FooterShellLink {
                label: "Access",
                href: "https://garunski.github.io/s_e_e_site/access/",
            },
        ],
    },
    FooterShellGroup {
        aria_label: "Resource links",
        label: "Resources",
        links: &[
            FooterShellLink {
                label: "Official Library",
                href: LIBRARY_CATALOG,
            },
            FooterShellLink {
                label: "Join the beta",
                href: BETA_URL,
            },
        ],
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
        Route::NotFound { .. } => "/404",
    }
}

fn theme_option_class(active: bool) -> &'static str {
    if active {
        THEME_OPTION_ON
    } else {
        THEME_OPTION
    }
}

#[component]
fn LibraryThemeMenu() -> Element {
    let mut theme = use_signal(|| read_cached_theme().unwrap_or(Theme::System));
    let mut open = use_signal(|| false);
    let system_dark = detect_system_dark_mode();
    let dark = theme_appears_dark(&theme(), system_dark);
    let icon = if dark { "sun" } else { "moon" };

    use_effect(move || {
        apply_theme_to_html(&theme());
    });

    rsx! {
        PopoverRoot {
            open: open(),
            on_open_change: EventHandler::new(move |value: bool| open.set(value)),
            PopoverTrigger {
                class: ICON_BTN,
                aria_label: "Theme",
                title: "Theme",
                Icon {
                    name: icon.to_string(),
                    class: Some("shrink-0".to_string()),
                    size: Some("h-[18px] w-[18px]".to_string()),
                    variant: Some("outline".to_string()),
                }
            }
            PopoverContent {
                side: ContentSide::Bottom,
                align: ContentAlign::End,
                div {
                    class: THEME_MENU,
                    role: "group",
                    aria_label: "Theme",
                    for (value, label) in [
                        (Theme::Light, "Light"),
                        (Theme::Dark, "Dark"),
                        (Theme::System, "System"),
                    ] {
                        button {
                            r#type: "button",
                            class: theme_option_class(theme() == value),
                            onclick: move |_| {
                                theme.set(value.clone());
                                write_cached_theme(&value);
                                apply_theme_to_html(&value);
                                open.set(false);
                            },
                            "{label}"
                        }
                    }
                }
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
            class: SHELL_HEADER,
            div {
                class: "{shell_layout_classes().content_inner} {MASTHEAD_GRID}",
                Link {
                    to: Route::Home {},
                    class: BRAND_LINK,
                    Icon {
                        name: "logo".to_string(),
                        class: Some("shrink-0".to_string()),
                        size: Some(MASTHEAD_LOGO_SIZE.to_string()),
                        variant: None,
                    }
                    span { class: "truncate text-sm font-semibold tracking-tight text-zinc-900 dark:text-zinc-50 max-md:hidden",
                        {APP_DISPLAY_NAME}
                    }
                    span { class: BRAND_LIBRARY, "Library" }
                }
                div { class: "flex shrink-0 items-center gap-2",
                    LibraryThemeMenu {}
                    a {
                        class: "{button_classes(ButtonVariant::Secondary, ButtonSize::Medium, false, None)} hidden sm:inline-flex",
                        href: library_href(HEADER_CATALOG.path),
                        "{HEADER_CATALOG.label}"
                    }
                }
            }
            nav {
                class: "{shell_layout_classes().content_inner} {NAV_ROW}",
                aria_label: "Primary navigation",
                for link in PRIMARY_NAV {
                    if link.file || link.external {
                        a {
                            class: if is_primary_current(link.path, current) { NAV_ON } else { NAV_LINK },
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
                    class: "{button_classes(ButtonVariant::Secondary, ButtonSize::Small, false, None)} sm:hidden",
                    href: library_href(HEADER_CATALOG.path),
                    "Catalog"
                }
            }
        }
    }
}

#[component]
fn LibraryNavRouterLink(path: &'static str, label: &'static str, current: String) -> Element {
    let active = is_primary_current(path, current.as_str());
    let class = if active { NAV_ON } else { NAV_LINK };
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

#[must_use]
pub fn library_copyright_line(year: i32) -> String {
    format!("© {year} {LIBRARY_COPYRIGHT_NAME}")
}

#[component]
fn FooterShellGroupNav(group: FooterShellGroup) -> Element {
    rsx! {
        nav {
            "aria-label": group.aria_label,
            class: "flex flex-col items-start gap-2.5",
            p {
                class: "mb-1 text-[0.76rem] font-semibold tracking-[0.14em] text-amber-400 uppercase",
                "{group.label}"
            }
            for link in group.links {
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
                    class: "grid gap-[clamp(3rem,7vw,7rem)] min-[801px]:grid-cols-[minmax(15rem,1.2fr)_minmax(30rem,2fr)]",
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
                            "Goal-driven agent execution that keeps direction, work, and evidence connected."
                        }
                    }
                    div { class: "grid grid-cols-1 gap-8 sm:grid-cols-3",
                        for group in FOOTER_SHELL_GROUPS {
                            FooterShellGroupNav { key: "{group.label}", group: *group }
                        }
                    }
                }
                div {
                    class: "mt-12 flex flex-col items-center gap-1 border-t border-zinc-700 pt-5 text-center text-sm tracking-wide text-zinc-500",
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
    let current_path = route_current_path(&route).to_string();
    let layout = shell_layout_classes();

    rsx! {
        div {
            id: "top",
            class: layout.root,
            a {
                class: "sr-only focus:not-sr-only focus:absolute focus:left-4 focus:top-4 focus:z-50 focus:rounded-md focus:bg-white focus:px-3 focus:py-2 focus:text-sm focus:shadow dark:focus:bg-zinc-900",
                href: "#library-page",
                "Skip to content"
            }
            LibraryMasthead { current_path }
            main {
                id: "library-page",
                "data-testid": "library-shell-main",
                class: layout.main,
                div {
                    class: layout.content_inner,
                    Outlet::<Route> {}
                }
            }
            LibraryFooter {}
        }
    }
}

#[cfg(test)]
#[path = "chrome_tests.rs"]
mod chrome_tests;
