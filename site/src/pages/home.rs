use dioxus::prelude::*;

use crate::pages::page_links::HOME_PACKAGE_TYPES;
use crate::shell::AppLink;

#[component]
pub fn LibraryHome() -> Element {
    rsx! {
        document::Title { "S.E.E. Library" }
        document::Meta {
            name: "description",
            content: "Install versioned workflows, prompts, skills, and commands, or create your own S.E.E. packages.",
        }
        div { class: "library-home",
            section { class: "library-hero", aria_labelledby: "library-home-title",
                div { class: "library-hero-human",
                    h1 { id: "library-home-title", "Reuse agent workflows." }
                    p { "Install versioned workflows, prompts, skills, and commands in your S.E.E. project." }
                }
                div { class: "library-hero-machine",
                    p { class: "library-kicker", "Official Library" }
                    h2 { "Create your own." }
                    p { "Follow the authoring guides and schemas to build packages you can version and share." }
                }
            }
            section { class: "library-home-white", aria_labelledby: "packages-heading",
                div { class: "library-content-width library-home-section",
                    p { class: "library-kicker", "Packages" }
                    h2 { id: "packages-heading", "Choose what the task needs." }
                    ol { class: "library-package-list",
                        for (index, item) in HOME_PACKAGE_TYPES.iter().enumerate() {
                            li {
                                span { class: "library-package-number", "{index + 1:02}" }
                                div {
                                    h3 { AppLink { path: item.path, "{item.title}" } }
                                    p { "{item.description}" }
                                }
                            }
                        }
                    }
                }
            }
            section { class: "library-home-black", aria_labelledby: "install-heading",
                div { class: "library-content-width library-home-section",
                    p { class: "library-kicker", "Installation" }
                    h2 { id: "install-heading", "Add packages to your project." }
                    p { class: "library-home-lede", "Use the in-app Library to install packages from this catalog, with a named version and declared install paths." }
                }
            }
            section { class: "library-home-white", aria_labelledby: "author-heading",
                div { class: "library-content-width library-home-section",
                    p { class: "library-kicker", "Authoring" }
                    h2 { id: "author-heading", "Package work you want to reuse." }
                    p { class: "library-home-lede", "Write the definition, check its schema, and publish a version with an install map." }
                    AppLink { class: "library-home-link", path: "/authoring/publish/", "Publishing guide" }
                }
            }
        }
    }
}
