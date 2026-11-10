use dioxus::prelude::*;

use crate::library_href;
use crate::pages::authoring_common::{AuthoringGuideFrame, CodeExample};

pub(crate) const HELP: &str = include_str!("../../../content/authoring/research.md");
pub(crate) const EXAMPLES: &str = include_str!("../../../content/authoring/research_examples.json");
pub(crate) const OPERATIONS: &str =
    include_str!("../../../content/authoring/research_operations.json");

#[component]
pub fn LibraryAuthoringResearch() -> Element {
    let operations: serde_json::Value =
        serde_json::from_str(OPERATIONS).expect("Research operations");
    rsx! {
        AuthoringGuideFrame {
            document_title: "Research authoring",
            meta_description: "Scoped Research, report, analysis and guide authoring in S.E.E.",
            s_e_e_gui_kit::components::MarkdownHtml { src: HELP.to_string() }
            h2 { "Operations" }
            p { "All operations require project. HTTP binds project and record identities from the path. The scoped request schemas use the corresponding project, research_id, page_key, section_key, source_id and edition_id fields." }
            div { class: "overflow-x-auto",
                table {
                    thead { tr { th { "MCP / native operation" } th { "HTTP" } th { "Path" } th { "Scope" } th { "Applicable scoped errors" } } }
                    tbody {
                        for op in operations.as_array().unwrap() {
                            tr {
                                td { code { {op["name"].as_str().unwrap()} } }
                                td { {op["method"].as_str().unwrap()} }
                                td { code { {op["path"].as_str().unwrap()} } }
                                td { {op["scope"].as_str().unwrap()} }
                                td { {op["errors"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>().join(", ")} }
                            }
                        }
                    }
                }
            }
            h2 { "Four-kind request sequences" }
            p { "These are illustrative Draft calls. Replace IDs and FROM_* markers with saved identities and the latest tokens. Each sequence creates one context section; cover every profile role before publication." }
            CodeExample { text: EXAMPLES }
            a { href: library_href("/schema/knowledge-research/"), "Research wire schema" }
        }
    }
}
