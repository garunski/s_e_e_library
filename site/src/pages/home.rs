use dioxus::prelude::*;
use dioxus_router::Link;
use s_e_e_gui_kit::components::{
    button::{button_classes, ButtonSize, ButtonVariant},
    layout::page_header::PageHeader,
    page_structure::{EditorialFrame, SystemOverview, SystemOverviewGroup, SystemOverviewKind},
};

use crate::library_href;
use crate::pages::page_links::{self, HOME_PACKAGE_TYPES, HOME_UTILITY_LINKS};
use crate::routes::Route;
use crate::shell::AppLink;
const PACKAGE_CARD: &str = "block rounded-lg border border-zinc-200/80 bg-white p-4 transition hover:border-sky-300 hover:bg-zinc-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-100 dark:border-zinc-700 dark:bg-zinc-900 dark:hover:border-sky-400/40 dark:hover:bg-zinc-800/60 dark:focus-visible:ring-sky-400/20";
const SECTION_LEDE: &str = "text-sm leading-relaxed text-zinc-600 dark:text-zinc-400";
const SECTION_KICKER: &str =
    "text-[11px] font-medium uppercase tracking-wide text-zinc-400 dark:text-zinc-500";
const SECTION_TITLE: &str =
    "text-base font-semibold tracking-tight text-zinc-900 dark:text-zinc-50";
const NOTE_LINK: &str =
    "text-sm font-medium text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200";

#[component]
pub fn LibraryHome() -> Element {
    rsx! {
        document::Title { "S.E.E. Official Library" }
        document::Meta {
            name: "description",
            content: "Versioned workflows, prompts, skills, commands, and bundles for hub and spoke projects.",
        }
        PageHeader {
            title: "S.E.E. Official Library".to_string(),
            description: "Versioned workflows, prompts, skills, commands, and bundles for hub and spoke projects.".to_string(),
            actions: Some(rsx! {
                a {
                    class: "{button_classes(ButtonVariant::Primary, ButtonSize::Medium, false, None)}",
                    href: "{library_href(HOME_UTILITY_LINKS[0].path)}",
                    "Open catalog.json"
                }
                Link {
                    to: Route::AuthoringIndex {},
                    class: "{button_classes(ButtonVariant::Secondary, ButtonSize::Medium, false, None)}",
                    "Author a package"
                }
            }),
        }
        div { class: "space-y-8 max-w-prose",
            p { class: "{SECTION_LEDE}",
                "Workflows, prompts, skills, and commands only travel between projects once they are named, versioned, and installable."
            }
            p { class: "{SECTION_LEDE}",
                "The in-app Library reads catalog.json as the install map. Payload files live under packages, and the schemas decide what is valid."
            }
        }
        div { class: "mt-8 space-y-8",
            SystemOverview {
                kind: SystemOverviewKind::OrderedJourney,
                kicker: "Package types".to_string(),
                title: "Five package types carry the machinery.".to_string(),
                SystemOverviewGroup {
                    kind: SystemOverviewKind::OrderedJourney,
                    number: "01",
                    title: "Package types",
                    description: "Each type has a payload shape, an install destination, and an authoring guide. Bundles install several of them together.",
                    ol { class: "space-y-3",
                        for item in HOME_PACKAGE_TYPES {
                            li {
                                AppLink {
                                    class: PACKAGE_CARD,
                                    path: item.path,
                                    span { class: "font-mono text-[11px] font-semibold text-sky-700 dark:text-sky-400", "{item.number}" }
                                    h3 { class: "mt-2 text-sm font-semibold text-zinc-900 dark:text-zinc-50", "{item.title}" }
                                    p { class: "mt-1 text-xs leading-relaxed text-zinc-500 dark:text-zinc-400", "{item.description}" }
                                }
                            }
                        }
                    }
                    div { class: "mt-6 space-y-2 border-t border-zinc-200/80 pt-4 dark:border-zinc-700",
                        p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                            "After the payload exists, Publish is the catalog entry, the install map, and the validation step."
                        }
                        AppLink {
                            class: NOTE_LINK,
                            path: page_links::AUTHORING_PUBLISH_LINK.path,
                            "Publish a package"
                        }
                    }
                }
            }
            EditorialFrame {
                section { class: "space-y-4 px-5 py-6 sm:px-6",
                    aria_labelledby: "contract-heading",
                    p { class: "{SECTION_KICKER}", "The contract" }
                    h2 {
                        id: "contract-heading",
                        class: "{SECTION_TITLE}",
                        "Schema pages own the payload shape."
                    }
                    p { class: "{SECTION_LEDE}",
                        "Every schema page carries its JSON Schema file, downloadable under /schema/. That includes package payloads and hub documents the runtime validates on save. Authoring guides explain how to write a package that satisfies one."
                    }
                    div { class: "flex flex-wrap items-center gap-4 pt-2",
                        Link {
                            to: Route::SchemaIndex {},
                            class: "{button_classes(ButtonVariant::Primary, ButtonSize::Medium, false, None)}",
                            "Catalog schema"
                        }
                        AppLink {
                            class: "text-sm font-medium text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200",
                            path: HOME_UTILITY_LINKS[3].path,
                            "Everything as one text file"
                        }
                    }
                }
            }
        }
    }
}
