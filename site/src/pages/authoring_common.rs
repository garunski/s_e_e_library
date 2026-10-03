use dioxus::prelude::*;
use dioxus_router::Link;

use crate::library_href;
use crate::routes::Route;
use crate::shell::LibraryDocFrame;

pub const PAGE_LEDE: &str = "text-sm leading-relaxed text-zinc-600 dark:text-zinc-400";
pub const SECTION_TITLE: &str = "text-sm font-semibold text-zinc-900 dark:text-zinc-50";
pub const SUBSECTION_TITLE: &str = "text-sm font-medium text-zinc-800 dark:text-zinc-100";
pub const BODY_TEXT: &str = "mt-2 text-sm leading-relaxed text-zinc-600 dark:text-zinc-400";
pub const LIST: &str = "mt-2 list-disc space-y-1 pl-5 text-sm text-zinc-600 dark:text-zinc-400";
pub const INLINE_CODE: &str =
    "rounded bg-zinc-100 px-1 py-0.5 font-mono text-[11px] text-zinc-800 dark:bg-zinc-800 dark:text-zinc-200";
pub const LINK: &str =
    "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200";
pub const DL: &str = "mt-3 space-y-3 border-t border-zinc-200/80 pt-3 text-sm dark:border-zinc-700";
pub const DT: &str = "font-mono text-[11px] font-semibold text-zinc-800 dark:text-zinc-200";
pub const DD: &str = "mt-0.5 text-zinc-600 dark:text-zinc-400";
pub const PRE: &str =
    "mt-2 overflow-x-auto rounded-md border border-zinc-200/80 bg-zinc-50 p-3 font-mono text-[11px] leading-relaxed text-zinc-800 dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-200";

#[component]
pub fn AuthoringGuideFrame(
    document_title: &'static str,
    meta_description: &'static str,
    rail_number: &'static str,
    rail_title: &'static str,
    rail_description: &'static str,
    children: Element,
) -> Element {
    rsx! {
        document::Title { "{document_title}" }
        document::Meta { name: "description", content: "{meta_description}" }
        LibraryDocFrame {
            title: document_title.to_string(),
            description: meta_description.to_string(),
            rail_number,
            rail_title,
            rail_description,
            {children}
        }
    }
}

#[component]
pub fn SchemaReferenceFooter(
    catalog_schema_label: &'static str,
    schema_route: Route,
    schema_link_label: &'static str,
    extra_schema_links: Option<Element>,
) -> Element {
    rsx! {
        h2 { class: "mt-8 {SECTION_TITLE}", "Schema reference" }
        ul { class: "mt-3 list-disc space-y-2 pl-5 text-sm text-zinc-600 dark:text-zinc-400",
            li {
                "Catalog manifest: "
                a {
                    class: "{LINK}",
                    href: library_href("/catalog.json"),
                    code { class: "{INLINE_CODE}", "{catalog_schema_label}" }
                }
            }
            li {
                Link { to: schema_route, class: "{LINK}", "{schema_link_label}" }
            }
            if let Some(extra) = extra_schema_links {
                {extra}
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
