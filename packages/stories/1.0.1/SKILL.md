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

**No tool fallback to files.** If the story MCP tools are unavailable, use the documented project HTTP API. If neither interface is reachable, stop and report that story authoring is blocked. Never create or repair a story or milestone with direct Markdown editing, `apply_patch`, shell writes, a guessed id, or a guessed filename.

**Keep filenames plain.** Use story and milestone titles containing only ASCII letters, numbers, and spaces. Do not use punctuation in titles. The store then writes `<id> - <title>.md`. After every create or title change, read the story with `story_get`, or read the milestone with `milestone_get`, and treat a successful store read as the required naming check. Do not list every milestone to read one.

## MCP tools

Read `see:/projects` for valid ids. Pass that `project` on every MCP call.

**Stories** (tool input schemas are the contract; do not hand-author frontmatter, filenames, id allocation, or section markers):

- `story_create` - allocate id and create shell (`title` required).
- `story_get` - read full story including description, AC, plan, `conflict_token`.
- `story_update` - write `description`, `implementation_plan`, `implementation_notes`, `final_summary` (pass `conflict_token` from the prior read).
- `story_set_status`, `story_set_labels`, `story_set_assignee`, `story_set_priority`, `story_set_milestone`, `story_set_dependencies` - metadata (pass `conflict_token` from the prior read).
- `story_add_criterion`, `story_edit_criterion`, `story_check_criterion`, `story_remove_criterion`, `story_reorder_criteria` - acceptance criteria.

**Milestones** (a milestone is a record, not a label: status, priority, labels, dates, ordinal, typed criteria, plan, notes, summary; no assignee):

- `milestone_list` - browse ids and progress.
- `milestone_get` - full record including criteria, body sections, and `conflict_token`.
- `milestone_create` - allocate id from title and description.
- `milestone_update` - write description, implementation_plan, implementation_notes, and final_summary.
- `milestone_set_status`, `milestone_set_priority`, `milestone_set_labels`, `milestone_set_ordinal`, `milestone_set_title`, `milestone_set_description`.
- `milestone_add_members`, `milestone_remove_members`.
- `milestone_add_criterion`, `milestone_edit_criterion`, `milestone_remove_criterion`, `milestone_reorder_criteria`, `milestone_check_criterion` (Manual only).

## MCP prompts

- `story-authoring` - full new-story procedure.
- `milestone-authoring` - milestone creation procedure.

## Store contract

**Milestone membership** - `milestone` is a frontmatter field whose value is a milestone id (`m-<n>`). Set it with `story_set_milestone`. Board milestone filters and `story_list` with `arguments.milestone_id` read this field only; do not add an `m-*` label to imply membership.

**Milestone criteria** - Typed `ObjectiveCriterion` rows in frontmatter with stable `criterion_id` (`c-N`). Kinds: `entity_status`, `milestone_progress`, `subject_rules`, `quality_gate`, `manual`. Derived kinds evaluate on read and have no checked flag. Only `manual` has `checked` and can be toggled. Address by `criterion_id`, never by numeric index. Story acceptance criteria are a numbered markdown checklist in the story body; they are a different model.

**Milestone completion** - Declared status is the completeness claim. A transition to Done requires at least one member, a self-targeting `milestone_progress` criterion at 100 percent, a separate outcome criterion, and all criteria met. Detailed `milestone_get` includes the evaluated result for each criterion. The milestone store cannot observe project execution, issue, and spoke facts, so `quality_gate` is unevaluable for milestone Done. Derived member-story counts are display and still feed objective `milestone_progress`. When status is empty, completeness follows criteria if any exist, otherwise derived story counts. Disagreement is shown, not silently resolved.

## Milestone cut and completion contract

Accepted `decision-25` applies whenever a milestone is created, revised, or declared Done. Write the observable outcome and its boundary first. Inventory every necessary screen, stored object, API, runtime path, Library definition, and existing-installation behavior. Map each necessary behavior to an implementation-ready story under `decision-27`. Include a story only when it is necessary for that outcome or an explicit milestone criterion. Keep optional enhancements, speculative work, and unrelated cleanup outside. Do not defer a necessary prerequisite while retaining the same outcome claim. State what is outside the cut and why it does not block the outcome.

Assign members through `milestone_add_members` or `story_set_milestone`; then read the milestone back and compare actual members with the scope inventory. Labels do not assign membership. Set story dependencies and implementation order where sequence matters. Every authored milestone has at least one member story.

Add a `milestone_progress` criterion targeting the new milestone id with `min_completion_percent: 100`. Member counts alone do not gate a declared Done status. Add separate typed criteria for observable cross-story outcomes, including compatibility and existing-installation behavior when relevant. Use `entity_status` for a specifically required story state. Use `manual` only for evidence the evaluator cannot derive; name the evidence and check it after review. Do not replace the progress criterion with a broad manual checkbox. Do not use a derived kind unless its evaluator sees the data being asserted. Address criteria by stable `criterion_id`, not their position.

Before setting Done, reconcile expected scope against actual membership; verify each member story's own acceptance criteria and Done status; inspect all milestone criterion results and manual evidence; and fill the final summary. The store rejects Done when a criterion is unmet, but it cannot detect an omitted member or vague outcome. When a member is added or removed, a new surface is discovered, or the target changes, revise the cut, criteria, and membership together before making the completion claim.

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

- **Atomic** - one independently deliverable behavior. A cross-layer change may touch many files when those layers must ship together.
- **Independent** - does not assume future stories.
- **Testable** - every AC is verifiable by running tests, checking files, or invoking an HTTP endpoint.
- **AI-implementable** - another agent reading just this story can do the work.
- **Dependency-safe** - only references lower-numbered stories.

## Required specificity review

The accepted decision `decision-27` makes a story an implementation contract. Apply this review to every new story and every material scope revision, including stories written as part of a milestone. A `story_create` shell is temporary: complete its description, plan, metadata, and criteria in the same authoring session and read the result back before handoff. Do not schedule, hand off, or move an incomplete shell into active work.

Before writing, verify the gap still exists. Inventory the affected screens, stored objects and fields, API or MCP operations, runtime components, Library and published definitions, and existing-installation paths. For each affected category, name the actual path or interface and required behavior in the implementation plan. State "not applicable" when an omitted category could look overlooked. Include exact defaults, numeric thresholds, error behavior, migration behavior, and dependencies where relevant.

Split independently deliverable effects into separate stories with explicit dependencies. If multiple layers must ship together, keep them in one story but enumerate each layer. No umbrella criterion such as "all views are updated" can stand in for individually verifiable outcomes. Map each affected surface to a plan step and an acceptance criterion with evidence outside the author's own diff. For a changed external integration, name the external result; for a published or seeded definition, include existing installations. Count screens, objects, and application code paths before declaring the story ready.

If the required approach cannot be chosen from the available evidence, ask for the missing decision before creating the story. Repeat the review when implementation reveals a new affected surface or dependency.

A story MUST NOT:

- Mix unrelated changes (e.g. "add field X and rewrite logging").
- Put implementation details in Description (those go in Implementation Plan).
- Put steps in Acceptance Criteria (those describe the outcome, not the path).
- Reference higher-numbered stories in dependencies.
- Be marked Done without all AC checked and Final Summary filled.
- Require launching a workflow execution, a probe workflow, or a live `execution_get` as the proof.

## Evidence first

Follow decision-27. Record an observed fact and where it was checked before a technical approach. An open product choice pauses authoring. An implementation hypothesis is not a fact. Do not prescribe a path, limit, or fixture until its system fact is checked. If the fact is missing, record the clarification question and pause. Keep a bounded outcome and acceptance evidence an outsider can check. Use item_fetch to read and item_call to write. Pass the conflict_token from the latest read. Read the record back with item_fetch before handoff. When the story belongs to a milestone, carry that milestone id, title, outcome, cut, existing members, and dependency context into the story.

## A story contains no choices (VERY IMPORTANT)

A story is a fully planned piece of work. The implementing agent executes it; it does not design it. Every decision is the author's, made at authoring time, written into the story as the thing to do.

Never leave a decision open. These phrasings are banned in the description, the plan, and the criteria:

- "Decide whether ...", "Choose between ...", "Determine whether ..."
- "Either X or Y", "X or Y, prefer X", "Options in rough order of invasiveness", "The candidates are ..."
- "Consider whether ...", "Weigh X against Y"
- "... and record the decision", "... is decided deliberately", "the reasoning is recorded in a decision record"

When two approaches exist, pick one, name it, and give one sentence on why the other loses. That sentence is the entire value of having considered the alternative; the alternative does not belong in the plan as a live option.

An acceptance criterion is never satisfied by making a decision. "The status vocabulary is decided and recorded" is not a criterion. "Status values come from the configured story status set" is. If a criterion contains "decided", "whether", "either", or "recorded", rewrite it as the behavior that results from the decision you already made.

Research the story genuinely needs is a plan step with an expected answer and a stated fallback, not an open question. "Confirm the pinned crate version exposes a max file count; if it does not, sweep on startup instead" is a plan step. "Investigate how to bound the files" is not.

Numbers are decisions too. Thresholds, intervals, retention windows, and limits are chosen by the author and written down. "Prune by count or age" is a choice; "prune records older than 90 days" is a story.

If authoring surfaces a decision you cannot make, stop and ask the user before creating the story. Never create the story with the choice embedded.

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
7. Read the created or renamed entity back through the store and confirm its id and title.
8. Apply the required specificity review and read back the completed body, criteria, and metadata.
9. Hand off implementation with the `work` skill.

For milestones: `milestone_create` and related MCP tools, or the `milestone-authoring` prompt. After create or a title change, read back with `milestone_get`. Do not list every milestone to read one. Link stories with `story_set_milestone`, not labels.

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
