use dioxus::prelude::*;
use s_e_e_gui_kit::components::{
    layout::page_header::PageHeader,
    page_structure::{system_rail_body_class, EditorialFrame},
};

#[component]
pub fn LibraryDocFrame(
    title: String,
    rail_title: &'static str,
    rail_description: &'static str,
    children: Element,
) -> Element {
    rsx! {
        document::Title { "{title} - S.E.E. Library" }
        div { class: "py-6 sm:py-8",
            PageHeader {
                title,
                description: String::new(),
                actions: None,
            }
            EditorialFrame {
                section { class: "ds-system-rail",
                    div { class: "min-w-0 bg-zinc-50 px-5 py-5 [overflow-wrap:anywhere] dark:bg-zinc-950",
                        h2 { class: "text-sm font-semibold text-zinc-950 dark:text-white", "{rail_title}" }
                        if !rail_description.is_empty() {
                            p { class: "mt-1 text-xs leading-relaxed text-zinc-500 dark:text-zinc-400", "{rail_description}" }
                        }
                    }
                    div { class: system_rail_body_class(false),
                        {children}
                    }
                }
            }
        }
    }
}
