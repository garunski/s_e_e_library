use dioxus::prelude::*;

use crate::library_href;
use crate::pages::page_links::{AUTHORING_GUIDE_LINKS, AUTHORING_PUBLISH_LINK};
use crate::shell::LibraryDocFrame;

const PAGE_LEDE: &str = "text-sm leading-relaxed text-zinc-600 dark:text-zinc-400";
const SECTION_TITLE: &str = "text-sm font-semibold text-zinc-900 dark:text-zinc-50";
const LIST_LINK: &str =
    "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200";
const LIST_ITEM: &str =
    "space-y-1 border-t border-zinc-200/80 py-3 first:border-t-0 dark:border-zinc-700";
const INLINE_CODE: &str = "rounded bg-zinc-100 px-1 py-0.5 font-mono text-[11px] text-zinc-800 dark:bg-zinc-800 dark:text-zinc-200";

#[component]
pub fn LibraryAuthoringIndex() -> Element {
    rsx! {
        document::Title { "Authoring" }
        document::Meta {
            name: "description",
            content: "How to author each S.E.E. library package type: workflows, prompts, skills, commands, bundles, cycles, and publishing to the catalog.",
        }
        LibraryDocFrame {
            title: "Authoring".to_string(),
            description: "How to author each S.E.E. library package type: workflows, prompts, skills, commands, bundles, cycles, and publishing to the catalog.".to_string(),
            rail_number: "01",
            rail_title: "Six package types",
            rail_description: "The guide names the file, the fields, and the install destination.",
            p { class: "{PAGE_LEDE}",
                "Every package in the Official Library is one of six types. Pick the type, write its payload, then publish it to the catalog."
            }
            h2 { class: "mt-6 {SECTION_TITLE}", "Package types" }
            ul { class: "mt-3 divide-y divide-zinc-200/80 dark:divide-zinc-700",
                li { class: "{LIST_ITEM}",
                    strong {
                        a { class: "{LIST_LINK}", href: library_href(AUTHORING_GUIDE_LINKS[0].path), "Workflows" }
                    }
                    span { class: "block text-sm text-zinc-600 dark:text-zinc-400",
                        "Task graphs the engine runs ("
                        code { class: "{INLINE_CODE}", "definition.json" }
                        ")."
                    }
                }
                li { class: "{LIST_ITEM}",
                    strong {
                        a { class: "{LIST_LINK}", href: library_href(AUTHORING_GUIDE_LINKS[1].path), "Prompts" }
                    }
                    span { class: "block text-sm text-zinc-600 dark:text-zinc-400",
                        "Named instruction text ("
                        code { class: "{INLINE_CODE}", "prompt.json" }
                        ")."
                    }
                }
                li { class: "{LIST_ITEM}",
                    strong {
                        a { class: "{LIST_LINK}", href: library_href(AUTHORING_GUIDE_LINKS[2].path), "Skills" }
                    }
                    span { class: "block text-sm text-zinc-600 dark:text-zinc-400",
                        "Activatable capability guides ("
                        code { class: "{INLINE_CODE}", "SKILL.md" }
                        ")."
                    }
                }
                li { class: "{LIST_ITEM}",
                    strong {
                        a { class: "{LIST_LINK}", href: library_href(AUTHORING_GUIDE_LINKS[3].path), "Commands" }
                    }
                    span { class: "block text-sm text-zinc-600 dark:text-zinc-400",
                        "Worker definitions for an agent CLI ("
                        code { class: "{INLINE_CODE}", "command.json" }
                        ")."
                    }
                }
                li { class: "{LIST_ITEM}",
                    strong {
                        a { class: "{LIST_LINK}", href: library_href(AUTHORING_GUIDE_LINKS[4].path), "Bundles" }
                    }
                    span { class: "block text-sm text-zinc-600 dark:text-zinc-400",
                        "Install maps that deliver several packages together."
                    }
                }
                li { class: "{LIST_ITEM}",
                    strong {
                        a { class: "{LIST_LINK}", href: library_href(AUTHORING_GUIDE_LINKS[5].path), "Cycles" }
                    }
                    span { class: "block text-sm text-zinc-600 dark:text-zinc-400",
                        "Host loops wiring workflows to stores ("
                        code { class: "{INLINE_CODE}", "cycle.json" }
                        ")."
                    }
                }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Publishing" }
            p { class: "mt-2 text-sm text-zinc-600 dark:text-zinc-400",
                a {
                    class: "{LIST_LINK}",
                    href: library_href(AUTHORING_PUBLISH_LINK.path),
                    "Publish to the catalog"
                }
                " covers payload layout, the "
                code { class: "{INLINE_CODE}", "catalog.json" }
                " entry, and validation."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "The contract" }
            ul { class: "mt-3 list-disc space-y-2 pl-5 text-sm text-zinc-600 dark:text-zinc-400",
                li {
                    a {
                        class: "{LIST_LINK}",
                        href: library_href("/schema/"),
                        "Catalog schema"
                    }
                    ", per-type payload schemas, and hub document schemas"
                }
                li {
                    "Catalog manifest: "
                    a {
                        class: "{LIST_LINK}",
                        href: library_href("/catalog.json"),
                        code { class: "{INLINE_CODE}", "see.library/v1" }
                    }
                }
                li {
                    "Same guidance as one plain-text file for agents: "
                    a {
                        class: "{LIST_LINK}",
                        href: library_href("/llms.txt"),
                        code { class: "{INLINE_CODE}", "llms.txt" }
                    }
                }
            }
        }
    }
}
