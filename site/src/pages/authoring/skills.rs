use dioxus::prelude::*;

use crate::pages::authoring_common::{
    AuthoringGuideFrame, AuthoringReference, CodeExample, BODY_TEXT, INLINE_CODE, LIST,
};
use crate::routes::Route;

const EXAMPLE: &str = include_str!("../../../content/authoring/skills_example.md");
const EDIT_REQUEST: &str =
    r#"{"content":"Updated skill body","expected_content":"Previously loaded skill body"}"#;
const INSTALL: &str = include_str!("../../../content/authoring/skills_install.json");

#[component]
pub fn LibraryAuthoringSkills() -> Element {
    rsx! {
        AuthoringGuideFrame {
            document_title: "Skills",
            meta_description: "A skill package teaches an agent a repeatable capability with YAML frontmatter and a markdown body.",
            section {
                h2 { "Skill format" }
                p { class: BODY_TEXT,
                    "A skill package teaches an agent a repeatable capability. Payload file: "
                    code { class: "{INLINE_CODE}", "SKILL.md" }
                    ". Installs to "
                    code { class: "{INLINE_CODE}", ".agents/skills/{{slug}}/SKILL.md" }
                    "."
                }
                p { class: "{BODY_TEXT}", "YAML frontmatter plus markdown body:" }
                ul { class: "{LIST}",
                    li {
                        code { class: "{INLINE_CODE}", "name" }
                        ": short skill id (matches the install folder name)"
                    }
                    li {
                        code { class: "{INLINE_CODE}", "description" }
                        ": activation text; when the user says these phrases, the agent should load this skill"
                    }
                }
                p { class: "{BODY_TEXT}",
                    "The body carries task-specific procedure and constraints that change what the agent does. Keep activation text short and use authoritative references for details needed only in particular situations."
                }
            }
            section {
                h2 { "Editing installed skills" }
                p { class: BODY_TEXT,
                    "GET /api/projects/:project/spokes/:spoke/agent-skills/:id returns a content string. PUT accepts content and an optional expected_content string containing the exact previously loaded text. A conditional update requires a nonempty body and an existing matching file; a stale or deleted file returns 409 without writing. Success remains 204. Omit expected_content to retain unconditional create or overwrite behavior. Updates change only SKILL.md; other files in the skill directory are preserved."
                }
                CodeExample { text: EDIT_REQUEST }
            }
            section {
                h2 { "Where guidance belongs" }
                p { class: "{BODY_TEXT}",
                    "Repository "
                    code { class: "{INLINE_CODE}", "AGENTS.md" }
                    " holds durable local operating rules. A skill holds reusable procedure for one capability. A workflow selects the tasks and inputs for a run; its prompt supplies instructions needed for that task. A story names one implementation outcome and its acceptance criteria. Agents read the applicable repository rules and load only the skills and references relevant to the work. Copying every layer into the first prompt makes repeated and conflicting guidance harder to maintain."
                }
                p { class: "{BODY_TEXT}",
                    "The current "
                    code { class: "{INLINE_CODE}", "doc" }
                    ", "
                    code { class: "{INLINE_CODE}", "stories" }
                    ", "
                    code { class: "{INLINE_CODE}", "workflow" }
                    ", and "
                    code { class: "{INLINE_CODE}", "work" }
                    " skills use the project store interfaces for installed records. They do not direct agents to edit store Markdown or workflow JSON on disk. Library package source files remain editable in this repository before publication. Publish a changed skill as a new version and refresh any bundle that embeds its payload. Existing installations need an upgrade to receive that version."
                }
            }
            section {
                h2 { "Write skills for any project" }
                p { class: "{BODY_TEXT}",
                    "S.E.E. installs "
                    code { class: "{INLINE_CODE}", "SKILL.md" }
                    " exactly as written. Do not use "
                    code { class: "{INLINE_CODE}", "<hub>" }
                    " or "
                    code { class: "{INLINE_CODE}", "<spoke:NAME>" }
                    " path tokens, and do not copy another project's paths, ids, languages, or commands into a skill; catalog validation rejects path tokens. Point agents at the repository "
                    code { class: "{INLINE_CODE}", "AGENTS.md" }
                    " for local rules and commands. Project setup writes a managed S.E.E. section into each project folder's "
                    code { class: "{INLINE_CODE}", "AGENTS.md" }
                    " that names the project id, the "
                    code { class: "{INLINE_CODE}", "see" }
                    " MCP server, and where the S.E.E. skills are installed."
                }
            }
            section {
                h2 { "Example" }
                p { class: "{BODY_TEXT}",
                    "From "
                    code { class: "{INLINE_CODE}", "packages/work/1.0.1/SKILL.md" }
                    ":"
                }
                CodeExample { text: EXAMPLE }
                p { class: "{BODY_TEXT}",
                    "Write the "
                    code { class: "{INLINE_CODE}", "description" }
                    " as a single activation sentence agents can match against user intent. Keep procedures in the markdown body, not in frontmatter."
                }
            }
            section {
                h2 { "Installation" }
                CodeExample { text: INSTALL }
            }
            section {
                h2 { "Story authoring packages" }
                p { class: "{BODY_TEXT}",
                    "The bundled "
                    code { class: "{INLINE_CODE}", "stories" }
                    " skill and "
                    code { class: "{INLINE_CODE}", "system-story-authoring" }
                    " prompt carry the story specificity review. Broad stories hide affected surfaces and can make partial work look complete. Apply the specificity review whenever a story is created or materially revised, before it is handed off, scheduled, or moved into active work."
                }
                p { class: "{BODY_TEXT}",
                    "Verify the current gap and count affected screens, stored objects, application code paths, APIs, runtime components, Library definitions, and existing installations. Name each affected path and exact behavior in the implementation plan. Split independently deliverable work into separate stories. Give each affected surface an observable acceptance criterion with evidence beyond the authored change. Complete and read back any store-created shell in the same authoring session."
                }
                p { class: "{BODY_TEXT}",
                    "When publishing either authoring package, keep this rule in its payload so installed agents receive the same guidance. S.E.E. Help, under "
                    strong { "Writing implementation ready stories" }
                    ", explains the review for people writing or reviewing stories."
                }
            }
            section {
                h2 { "Milestone authoring packages" }
                p { class: "{BODY_TEXT}",
                    "The "
                    code { class: "{INLINE_CODE}", "stories" }
                    " skill and "
                    code { class: "{INLINE_CODE}", "system-milestone-authoring" }
                    " prompt carry the milestone cut and completion review. Apply it when creating or revising a milestone and before declaring it Done. A milestone cut names one observable outcome, includes the stories necessary to deliver it, and states why other work is deferred."
                }
                p { class: "{BODY_TEXT}",
                    "Assign stories through their milestone field and read membership back. Add a "
                    code { class: "{INLINE_CODE}", "milestone_progress" }
                    " criterion targeting the milestone at 100 percent, then add separate criteria for the combined outcome and compatibility behavior. Story acceptance criteria verify individual changes; milestone criteria prove the release outcome. Check manual criteria only after reviewing their named evidence. If scope changes, revise the cut, membership, and criteria before claiming completion."
                }
                p { class: "{BODY_TEXT}",
                    "The Done transition requires at least one member story, self progress at 100 percent, a separate outcome criterion, and every criterion met. The detailed "
                    code { class: "{INLINE_CODE}", "milestone_get" }
                    " result shows each criterion's evaluation. The milestone store cannot observe project execution, issue, or spoke quality facts, so a "
                    code { class: "{INLINE_CODE}", "quality_gate" }
                    " criterion is unevaluable for milestone Done."
                }
                p { class: "{BODY_TEXT}",
                    "Keep this guidance in the published skill and prompt payloads. S.E.E. Help, under "
                    strong { "Authoring milestones and making the cut" }
                    ", explains the membership and completion review."
                }
            }
            AuthoringReference {
                schema_route: Route::SchemaSkill {},
                schema_link_label: "Skill schema",
                extra_schema_links: None,
            }
        }
    }
}
