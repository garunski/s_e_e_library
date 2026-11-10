use dioxus::prelude::*;

use crate::library_href;
use crate::pages::authoring_common::AuthoringGuideFrame;

pub(crate) const HELP: &str = include_str!("../../../content/authoring/research_schema.md");

#[component]
pub fn LibrarySchemaResearch() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Knowledge Research schema",
            meta_description: "Scoped Research requests, responses and errors for S.E.E.",
            s_e_e_gui_kit::components::MarkdownHtml { src: HELP.to_string() }
            a { href: library_href("/authoring/research/"), "Research authoring and all 28 HTTP/MCP operations" }
            a { href: library_href("/schema/collection-filter.schema.json"), "Shared collection-filter schema" }
            a { href: library_href("/schema/knowledge-research.schema.json"), "knowledge-research.schema.json" }
        }
    }
}
