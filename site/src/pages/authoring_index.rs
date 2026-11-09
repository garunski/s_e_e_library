use dioxus::prelude::*;

use crate::library_href;
use crate::pages::authoring_common::{INDEX_LINK_LIST, INLINE_CODE, LINK};
use crate::pages::page_links::{AUTHORING_GUIDES, AUTHORING_PUBLISH_LINK};
use crate::shell::AppLink;

#[component]
pub fn LibraryAuthoringIndex() -> Element {
    rsx! {
        document::Title { "Authoring - S.E.E. Library" }
        document::Meta {
            name: "description",
            content: "How to author each S.E.E. Library package type and publish it to the catalog.",
        }
        div { class: "library-home",
            div { class: "library-doc-page",
                h1 { "Authoring" }
                div { class: "library-doc-sections",
                    section {
                        h2 { "Package guides" }
                        ul { class: INDEX_LINK_LIST,
                            for guide in AUTHORING_GUIDES {
                                li {
                                    AppLink { class: LINK, path: guide.path, "{guide.label}" }
                                    span { class: "ml-1",
                                        "{guide.description}"
                                        if let Some(payload) = guide.payload {
                                            " ("
                                            code { class: INLINE_CODE, "{payload}" }
                                            ")"
                                        }
                                        "."
                                    }
                                }
                            }
                        }
                    }
                    section {
                        h2 { "Publishing" }
                        ul { class: INDEX_LINK_LIST,
                            li {
                                AppLink {
                                    class: LINK,
                                    path: AUTHORING_PUBLISH_LINK.path,
                                    "Publish to the catalog"
                                }
                                " covers payload layout, the "
                                code { class: INLINE_CODE, "catalog.json" }
                                " entry, and validation."
                            }
                        }
                    }
                    section {
                        h2 { "Reference" }
                        p { "Project settings > Experimental controls project-local runtime availability. All six switches default to off. Package storage and reusable workflows remain available. MCP discovery across authorized projects can expose a tool enabled elsewhere, while every invocation checks its target project." }
                        ul { class: INDEX_LINK_LIST,
                            li { AppLink { class: LINK, path: "/schema/hub-config/", "Experimental configuration and API contract" } }
                            li { AppLink { class: LINK, path: "/schema/knowledge-research/", "Research profiles, availability and MCP examples" } }
                            li {
                                AppLink { class: LINK, path: "/schema/catalog/", "Catalog schema" }
                                ", per-type payload schemas, and hub document schemas"
                            }
                            li {
                                a {
                                    class: LINK,
                                    href: library_href("/catalog.json"),
                                    "catalog.json"
                                }
                                " Catalog manifest."
                            }
                            li {
                                a {
                                    class: LINK,
                                    href: library_href("/llms.txt"),
                                    "llms.txt"
                                }
                                " Plain-text guidance for agents."
                            }
                        }
                    }
                }
            }
        }
    }
}
