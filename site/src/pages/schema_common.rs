use dioxus::prelude::*;
use dioxus_router::Link;
use s_e_e_gui_kit::components::button::{button_classes, ButtonSize, ButtonVariant};

use crate::library_href;
use crate::pages::authoring_common::{AuthoringGuideFrame, PAGE_LEDE, PRE};
use crate::pages::schema_assets::SchemaDownload;

const DOWNLOAD_ROW: &str = "mt-3 flex flex-wrap gap-2";

#[component]
pub fn SchemaGuideFrame(
    document_title: &'static str,
    meta_description: &'static str,
    rail_title: &'static str,
    rail_description: &'static str,
    children: Element,
) -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title,
            meta_description,
            div { class: "library-schema-details",
                p { class: "{PAGE_LEDE} mb-6 border-l-2 border-zinc-200 pl-4 [overflow-wrap:anywhere] dark:border-zinc-700",
                    strong { class: "font-semibold text-zinc-900 dark:text-zinc-50", "{rail_title}" }
                    if !rail_description.is_empty() {
                        span { class: "block", "{rail_description}" }
                    }
                }
                {children}
            }
        }
    }
}

#[component]
pub fn SchemaBlock(entry: &'static SchemaDownload) -> Element {
    let href = library_href(&format!("/schema/{}", entry.filename));
    rsx! {
        div { class: "{DOWNLOAD_ROW}",
            a {
                class: "{button_classes(ButtonVariant::Primary, ButtonSize::Small, false, None)}",
                href: "{href}",
                "Download {entry.filename}"
            }
        }
        pre { class: "{PRE}", "{entry.json}" }
    }
}

#[component]
pub fn SchemaBlockByName(filename: &'static str) -> Element {
    let entry = crate::pages::schema_assets::schema_download(filename)
        .expect("schema file registered in schema_assets");
    rsx! {
        SchemaBlock { entry }
    }
}

#[component]
pub fn NamedSchemaLink(
    route: crate::routes::Route,
    label: &'static str,
    description: &'static str,
) -> Element {
    rsx! {
        li {
            strong {
                Link { to: route, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300 dark:hover:text-sky-200", "{label}" }
            }
            span { class: "ml-1 text-zinc-600 dark:text-zinc-400", "{description}" }
        }
    }
}
