use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pages::authoring_common::{INLINE_CODE, LIST, PAGE_LEDE, SECTION_TITLE};
use crate::pages::schema_common::{SchemaBlockByName, SchemaGuideFrame};
use crate::routes::Route;

#[component]
pub fn LibrarySchemaSkill() -> Element {
    rsx! {
        SchemaGuideFrame {
            document_title: "Skill schema",
            meta_description: "Category skill. Payload file SKILL.md. Installs to .agents/skills/{{slug}}/SKILL.md.",
            rail_number: "03",
            rail_title: "Installs under .agents/skills.",
            rail_description: "Destination is .agents/skills/{{slug}}/SKILL.md.",
            p { class: "{PAGE_LEDE}",
                "Category "
                code { class: "{INLINE_CODE}", "skill" }
                ". Payload file "
                code { class: "{INLINE_CODE}", "SKILL.md" }
                ". Installs to "
                code { class: "{INLINE_CODE}", ".agents/skills/{{slug}}/SKILL.md" }
                "."
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                code { class: "{INLINE_CODE}", "SKILL.md" }
                " is YAML frontmatter plus a markdown body. Frontmatter:"
            }
            ul { class: "{LIST}",
                li { code { class: "{INLINE_CODE}", "name" }, " - non-empty skill name." }
                li { code { class: "{INLINE_CODE}", "description" }, " - non-empty activation description an agent matches against." }
            }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400", "The markdown after the frontmatter is the skill instructions." }
            h2 { class: "mt-8 {SECTION_TITLE}", "JSON Schema" }
            p { class: "text-sm text-zinc-600 dark:text-zinc-400",
                "The payload is a markdown file; this schema describes its YAML frontmatter block."
            }
            SchemaBlockByName { filename: "skill-frontmatter.schema.json" }
            p { class: "mt-6 text-sm text-zinc-600 dark:text-zinc-400",
                "See the "
                Link { to: Route::AuthoringSkills {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "skill authoring guide" }
                " and the "
                Link { to: Route::SchemaIndex {}, class: "font-semibold text-sky-700 hover:text-sky-600 dark:text-sky-300", "catalog schema" }
                "."
            }
        }
    }
}
