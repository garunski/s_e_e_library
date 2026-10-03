use dioxus::prelude::*;
use s_e_e_gui_kit::components::{
    layout::page_header::PageHeader,
    page_structure::{ContentRail, EditorialFrame},
};

#[component]
pub fn LibraryDocFrame(
    title: String,
    description: String,
    rail_number: &'static str,
    rail_title: &'static str,
    rail_description: &'static str,
    children: Element,
) -> Element {
    rsx! {
        PageHeader {
            title: title.clone(),
            description: description.clone(),
            actions: None,
        }
        EditorialFrame {
            ContentRail {
                number: rail_number,
                title: rail_title,
                description: rail_description,
                {children}
            }
        }
    }
}
