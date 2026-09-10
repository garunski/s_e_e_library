---
name: stories
description: >-
  Expert story author for the S.E.E. stories store. Atomic, testable,
  AI-implementable stories via MCP tools and the project HTTP API. Use when
  the user says /stories, /story, "create a story", "add a story", or works
  with stories, milestones, or labels in the S.E.E. project.
---
## Roots

Hub store paths use `<hub>/.s_e_e/...`. Code spoke paths use `<spoke:s_e_e>/...`. Root tokens: **`doc-1`** (`<hub>/.s_e_e/knowledge/docs/overview/projects/doc-1 - Workspace Layout and Path Resolution.md`).

## Languages: Rust and shell only (VERY IMPORTANT)

Hard ban, no exceptions. Full contract: **`decision-15`** (`<hub>/.s_e_e/knowledge/decisions/decision-15 - Keep the workspace to Rust and shell.md`).

- Application code is Rust. Infrastructure code is bash under `<spoke:s_e_e>/.mise/scripts/*.sh`, using only bash and the pinned text tools (`awk`, `sed`, `grep`, `rg`, `sort`, `cut`, `jq`, `cargo`).
- TypeScript only in `<spoke:s_e_e_library>`, the documentation site, and only the minimum that site needs.
- NO PYTHON, anywhere, for anything. No `.py` files, no `python` or `python3` invocation, no `python -c`, no Python heredoc inside a shell script, no `__pycache__`. This includes throwaway work inside a single turn: never reach for Python to check a number, parse a file, or verify your own output.
- No other language anywhere: no Go, Ruby, Perl, Lua, or Node scripts.
- A shell script whose body launches another interpreter is a violation of this rule, not a way to satisfy it.
- Infrastructure has no tests. mise tasks and `.mise/scripts/*` ship without test files; they prove themselves by running. Never write an acceptance criterion that requires a test for a mise task, and never quote a metric as a value a test must assert.
- TOML, JSON, YAML, and JSON Schema are data, not languages, and are unaffected.

A story must never ask for another language, must never ask for tests over infrastructure, and must never prove itself by launching a workflow execution. If a story seems to need any of those, stop and ask the user first.

# S.E.E. Story Authoring Agent

**Role**: Expert story author for the S.E.E. stories store. Create atomic, testable, AI-implementable stories.

**Direct markdown file editing is unsupported.** Use the MCP tools and HTTP endpoints below. Every MCP call requires an explicit `project` id. Read `see:/projects/{project}/stories/config` and `see:/projects/{project}/workspace-layout` for statuses, labels, and path rules; do not hardcode them.

## MCP tools

Read `see:/projects` for valid ids. Pass that `project` on every MCP call.

**Stories** (tool input schemas are the contract; do not hand-author frontmatter, filenames, id allocation, or section markers):

- `story_create` - allocate id and create shell (`title` required).
- `story_get` - read full story including description, AC, plan, `conflict_token`.
- `story_update` - write `description`, `implementation_plan`, `implementation_notes`, `final_summary` (pass `conflict_token` from the prior read).
- `story_set_status`, `story_set_labels`, `story_set_assignee`, `story_set_priority`, `story_set_milestone`, `story_set_dependencies` - metadata (pass `conflict_token` from the prior read).
- `story_add_criterion`, `story_edit_criterion`, `story_check_criterion`, `story_remove_criterion`, `story_reorder_criteria` - acceptance criteria.

**Milestones**:

- `milestone_list`, `milestone_create`, `milestone_set_title`, `milestone_set_description`, `milestone_add_members`, `milestone_remove_members`.

## MCP prompts

- `story-authoring` - full new-story procedure.
- `milestone-authoring` - milestone creation procedure.

## Store contract

**Milestone membership** - `milestone` is a frontmatter field whose value is a milestone id (`m-<n>`). Set it with `story_set_milestone`. Board milestone filters and `list_stories_by_milestone` read this field only; do not add an `m-*` label to imply membership.

**Statuses** - Valid values live in `see:/projects/{project}/stories/config` (backed by the `stories` section of `<hub>/.s_e_e/config.json`). Read that resource before `story_set_status`; do not restate or invent status names in authored content.

**Declared cross-references** - `related_docs` and `related_decisions` are optional frontmatter list fields. Each entry is a `doc-<n>` or `decision-<n>` id the story declares in the link graph (distinct from `labels` such as `doc-<n>` used for board filtering). The serializer writes them; `story_get` returns them on the wire. No dedicated MCP setter ships yet.

**Frontmatter shape (reference)** - The store owns frontmatter. Tool calls allocate ids and filenames; do not hand-author YAML or section markers. On disk, fields appear in this order (`<spoke:s_e_e>/stories/src/serializer/story.rs`):

```yaml
id: story-<n>
title: Short title
status: New
assignee: []
created_date: 'YYYY-MM-DD HH:mm'
updated_date: 'YYYY-MM-DD HH:mm'
labels: []
milestone: m-<n>
dependencies: []
references: []
documentation: []
parent_story_id: story-<n>
sub_stories: []
priority: medium
ordinal: 1
implementation_order: 1
spoke: s_e_e
related_docs:
- doc-<n>
related_decisions:
- decision-<n>
```

## Story quality (judgment)

A good story is:

- **Atomic** - one PR-sized scope; one or two source files plus tests.
- **Independent** - does not assume future stories.
- **Testable** - every AC is verifiable by running tests, checking files, or invoking an HTTP endpoint.
- **AI-implementable** - another agent reading just this story can do the work.
- **Dependency-safe** - only references lower-numbered stories.

A story MUST NOT:

- Mix unrelated changes (e.g. "add field X and rewrite logging").
- Put implementation details in Description (those go in Implementation Plan).
- Put steps in Acceptance Criteria (those describe the outcome, not the path).
- Reference higher-numbered stories in dependencies.
- Be marked Done without all AC checked and Final Summary filled.
- Require launching a workflow execution, a probe workflow, or a live `execution_get` as the proof.

## AC conventions

- Number each AC: `- [ ] #1 ...`. Each AC is one outcome; split compound criteria.
- Use the past/present-tense outcome form: *"`parse_workflow_from_value` rejects entries with empty `key`"*, not *"add validation for empty key"*.
- Test coverage shows up as its own AC when appropriate.
- Never write an AC proved by `workflow_execute` or a live `execution_get`.

## Authoring workflow

1. Read `see:/projects`.
2. Read the stories config resource for labels and allowed statuses.
3. `story_create` with title, or run the `story-authoring` prompt.
4. Set description and implementation plan with `story_update` (pass `conflict_token` from `story_get`) or prompt follow-through.
5. Add numbered AC with `story_add_criterion`.
6. Set labels, priority, milestone, and dependencies via MCP tools.
7. Hand off implementation with the `work` skill.

For milestones: `milestone_create` and related MCP tools, or the `milestone-authoring` prompt. Link stories with `story_set_milestone`, not labels.

For knowledge docs and decisions, use the `doc` skill.

## Definition of Done (authoring)

Description, AC, and plan are filled; metadata is set; the story is ready for the `work` skill.

## Complete example

Use `story_get` on `story-100` after resolving `project`. It exemplifies atomic scope, numbered AC, and a concrete implementation plan referencing real modules.

## Writing acceptance criteria that cannot be faked

These eight rules come from reading agent runs against real stories. Each one exists because a criterion passed while the feature did not work.

1. **State a behavior, not an artifact.** If the subject of the criterion is a file, type, field, or config entry rather than an observable outcome, rewrite it. Banned openings include "X exists", "X is declared", "the envelope declares", "the seeded definition is updated", "the field is added", "X is configured". `story-139` shipped a dead flag because criterion #1 read "the envelope declares an optional `subject_required` flag"; the working version reads "a workflow with `subject_required` set refuses to launch until a subject of a declared kind is selected".

2. **Name evidence the implementer does not author.** A criterion satisfiable by inspecting one's own diff is not a criterion. Where the behavior crosses a process, machine, or third-party boundary, name the artifact the far side produces: a process log line, an HTTP response body, a rendered surface, a stored record. `story-217` #2 read "the child process is launched with the MCP server configured"; the implementer wrote the injection and then checked the injection, and the child could not use it.

3. **Integrating with a third-party binary requires a mechanism criterion.** State how that binary actually discovers or receives the thing, and require the plan to record where that was confirmed. Setting an environment variable is not the same as the tool reading it. `story-217` assumed `MCP_CONFIG` in the child environment; Cursor CLI loads MCP from its project `mcp.json` under the workspace `.cursor` directory and ignored it, which cost two full runs to discover.

4. **Changing seeded or default data requires an existing-installation criterion.** State the outcome on an installation that already holds the entity, not only on a fresh one. Seeding usually runs once, so editing a seed file changes nothing for anyone already running.

5. **A red completion gate means the story is not Done.** If the gate fails, the story stays In Progress unless a criterion explicitly permits the failure, the failing tests are named, and the evidence that they fail identically at the base commit is recorded. "Unrelated, already dirty" without that evidence is an assertion, not a finding.

6. **Partial completion is a valid outcome.** Leaving a story In Progress with unchecked criteria and named blockers is correct, and better than ticking a criterion that was not verified. Where a criterion is expected to be hard to verify, say so in the story so the implementer knows stopping is allowed.

7. **Verify the gap before writing the story.** Confirm the defect still exists at authoring time and record in the description the path, command, or line that shows it. A story written against stale state spends a full agent run rediscovering that the work is already done.

8. **Never prove a story by launching a workflow execution.** Never write a criterion that requires `workflow_execute`, a probe or throwaway workflow, or a live `execution_get`. Engine crate tests that exercise the runtime in-process remain allowed. For prompt-delivery stories, the stored prompt fragments and command definitions are the evidence.

## Related

- Story implementation flow: [../work/SKILL.md](../work/SKILL.md)
- Knowledge docs and decisions: [../doc/SKILL.md](../doc/SKILL.md)
- Stories pillar: `<hub>/.s_e_e/knowledge/decisions/decision-16 - Stories-as-a-first-class-pillar.md`
- Routes and board: `<hub>/.s_e_e/knowledge/docs/reference/gui-pages/doc-16 - GUI Pages and Routes.md`
