use dioxus::prelude::*;
use dioxus_router::Link;

use crate::library_href;
use crate::routes::Route;

pub const PAGE_LEDE: &str = "text-zinc-600 dark:text-zinc-400";
pub const SECTION_TITLE: &str = "text-zinc-900 dark:text-zinc-50";
pub const SUBSECTION_TITLE: &str = "text-zinc-800 dark:text-zinc-100";
pub const BODY_TEXT: &str = "mt-2 text-zinc-600 dark:text-zinc-400";
pub const LIST: &str = "mt-2 list-disc space-y-1 pl-5 text-zinc-600 dark:text-zinc-400";
pub const INDEX_LINK_LIST: &str =
    "mt-3 list-none space-y-2 text-zinc-600 dark:text-zinc-400";
pub const INLINE_CODE: &str =
    "rounded bg-zinc-100 px-1 py-0.5 font-mono text-zinc-800 dark:bg-zinc-800 dark:text-zinc-200";
pub const LINK: &str =
    "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200";
pub const DL: &str = "mt-3 space-y-3 border-t border-zinc-200/80 pt-3 dark:border-zinc-700";
pub const DT: &str = "font-mono font-semibold text-zinc-800 dark:text-zinc-200";
pub const DD: &str = "mt-0.5 text-zinc-600 dark:text-zinc-400";
pub const PRE: &str =
    "mt-2 rounded-md border border-zinc-200/80 bg-zinc-50 p-4 font-mono text-zinc-800 dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-200";

#[component]
pub fn AuthoringGuideFrame(
    document_title: &'static str,
    meta_description: &'static str,
    children: Element,
) -> Element {
    rsx! {
        document::Title { "{document_title} - S.E.E. Library" }
        document::Meta { name: "description", content: "{meta_description}" }
        div { class: "library-home library-doc-page",
            h1 { "{document_title}" }
            div { class: "library-doc-sections", {children} }
        }
    }
}

#[component]
pub fn AuthoringReference(
    schema_route: Route,
    schema_link_label: &'static str,
    extra_schema_links: Option<Element>,
) -> Element {
    rsx! {
        section {
            h2 { "Reference" }
            ul { class: INDEX_LINK_LIST,
                li { Link { to: schema_route, class: LINK, "{schema_link_label}" } }
                if let Some(extra) = extra_schema_links {
                    {extra}
                }
                li { a { class: LINK, href: library_href("/catalog.json"), "catalog.json" } }
            }
        }
    }
}

#[component]
pub fn CodeExample(text: &'static str) -> Element {
    rsx! {
        pre { class: "{PRE}", "{text}" }
    }
}
