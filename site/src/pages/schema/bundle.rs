use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaBundle() -> Element {
    let install_paths_href = crate::library_href("/schema/#install-paths");
    rsx! {
        SchemaGuideFrame {
            document_title: "Bundle schema",
            meta_description: "Category bundle. No payload of its own. catalog files[] installs member payloads together.",
            rail_number: "06",
            rail_title: "Metadata lives in the sidecar.",
            rail_description: "s_e_e_package.json carries id, slug, category, name, and dependencies.",
            p { class: "{PAGE_LEDE}",
                "Category "
                code { class: "{INLINE_CODE}", "bundle" }
                ". No payload of its own."
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "A bundle's catalog "
                code { class: "{INLINE_CODE}", "files[]" }
                " array installs the payloads of several other packages together; the build infers each file's kind from its "
                code { class: "{INLINE_CODE}", "to" }
                " prefix (see "
                a {
                    class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300",
                    href: "{install_paths_href}",
                    "Install paths"
                }
                " on the catalog schema page)."
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "Bundle metadata lives in its "
                code { class: "{INLINE_CODE}", "s_e_e_package.json" }
                ":"
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "id" }, ", ", code { class: "{INLINE_CODE}", "slug" }, ", ", code { class: "{INLINE_CODE}", "category" }, " (bundle), ", code { class: "{INLINE_CODE}", "name" }, ", ", code { class: "{INLINE_CODE}", "description" }, ", ", code { class: "{INLINE_CODE}", "labels" }, ", ", code { class: "{INLINE_CODE}", "dependencies" }, "." }
            }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "Schema for a bundle's "
                code { class: "{INLINE_CODE}", "s_e_e_package.json" }
                "."
            }
            SchemaBlockByName { filename: "bundle-package.schema.json" }
            p { class: "mt-6 text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                Link { to: Route::AuthoringBundles {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "bundle authoring guide" }
                " and the "
                Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                "."
            }
        }
    }
}
