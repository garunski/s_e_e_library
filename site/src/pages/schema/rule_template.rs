use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::SchemaGuideFrame;
use crate::routes::Route;

#[component]
pub fn LibrarySchemaRuleTemplate() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Rule and template schema",
            meta_description: "Two lightweight, file-only categories: Cursor rule .mdc files and scaffold templates.",
            rail_number: "05",
            rail_title: "Install prefixes are fixed.",
            rail_description: "Rules go to .cursor/rules. Templates go under templates/.",
            p { class: "{PAGE_LEDE}", "Two lightweight, file-only categories." }
            h2 { class: "mt-6 {SECTION_TITLE}", "Rule" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "Category "
                code { class: "{INLINE_CODE}", "rule" }
                ". Payload is a Cursor rule "
                code { class: "{INLINE_CODE}", ".mdc" }
                " file (markdown with rule frontmatter). Installs to "
                code { class: "{INLINE_CODE}", ".cursor/rules/{{slug}}.mdc" }
                "."
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "Template" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "Category "
                code { class: "{INLINE_CODE}", "template" }
                ". Payload is an arbitrary scaffold file. Installs under "
                code { class: "{INLINE_CODE}", "templates/" }
                "."
            }
            p { class: "mt-6 text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                " for the manifest and install-path rules."
            }
        }
    }
}
