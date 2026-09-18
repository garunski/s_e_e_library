---
name: stories
description: >-
  Expert story author for the S.E.E. stories store. Atomic, testable,
  AI-implementable stories written through the story MCP tools or project
  HTTP API. Use when the user says /stories, /story, "create a
  story", "add a story", or works with stories, milestones, or labels in
  the S.E.E. project.
---

# S.E.E. Story Authoring Agent

**Role**: Expert story author for the S.E.E. stories store. Create atomic, testable, AI-implementable stories.

Story and milestone writes go through the story MCP tools or the documented project HTTP API. Direct Markdown editing is unsupported. If neither store interface is reachable, stop and report that authoring is blocked. Never fall back to `apply_patch`, shell writes, a guessed id, or a guessed filename.

## Store Layout

```
.s_e_e/
├── stories/
│   ├── config.json              # statuses, labels, prefixes (read-only for agents)
│   ├── next-id                 # next numeric suffix to allocate (decimal); reserved by the store
│   ├── stories/
│   │   └── story-<n> - <Plain-Title>.md
│   ├── milestones/
│   │   └── m-<n> - <Plain-Title>.md
│   └── archive/stories/        # archived stories (do not write here)
└── knowledge/
    ├── decisions/decision-<n> - <Kebab-Title>.md     # see doc skill
    └── docs/<category>/<area>/doc-<n> - <Title>.md   # see doc skill
```

- `<n>` is allocated by `story_create`, `milestone_create`, or the equivalent project HTTP create endpoint. Never infer it from directory contents or the counter.
- `<Plain-Title>` contains only ASCII letters, numbers, and spaces. Do not use punctuation in story or milestone titles. The store writes the filename; agents never construct it.
- Stories live in `.s_e_e/planning/stories/`; milestones in `.s_e_e/planning/milestones/`. Decisions and docs live under `.s_e_e/knowledge/` (use the `doc` skill).

## Story File - Required Shape

```markdown
---
id: story-<n>
title: <Sentence-case title>
status: New
assignee: []
created_date: 'YYYY-MM-DD HH:mm'
updated_date: 'YYYY-MM-DD HH:mm'
labels:
  - <area>
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal and scope. Why this story exists. No implementation details.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Outcome-based, verifiable criterion
- [ ] #2 Another testable outcome
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Bullet steps the implementer will follow
- Reference exact files / modules they will touch
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
<!-- SECTION:FINAL_SUMMARY:END -->
```

**Critical**: every section uses HTML comment markers. The store parser uses them to round-trip the file. Do not rename, nest, or duplicate them.

## Frontmatter Rules

- `id` - `story-<n>`. Must match the filename's `<n>`.
- `title` - sentence case. Send it to the store and let the store compute or rename the filename.
- `status` - one of the `stories.statuses` values in `.s_e_e/config.json` (`New`, `Ready for Dev`, `In Progress`, `Done`). Start at `New`.
- `assignee` - empty list on creation; populated when work starts.
- `created_date` / `updated_date` - `'YYYY-MM-DD HH:mm'` quoted strings.
- `labels` - list of strings; pick from existing labels first (look across `.s_e_e/planning/stories/`); only invent a new label when no existing one fits. Common labels reference architecture areas (e.g. `engine`, `gui`, `core`, `types`) or doc ids (e.g. `doc-13`, `doc-18`).
- `dependencies` - list of `story-<m>` ids that must be Done before this story can start. Only reference lower ids.
- `priority` - `low`, `medium`, or `high`.

## Story Quality Rules

A good story is:

- **Atomic** - one PR-sized scope; one or two source files plus tests.
- **Independent** - does not assume future stories.
- **Testable** - every AC is verifiable by running tests, checking files, or invoking an HTTP endpoint.
- **AI-implementable** - another agent reading just this story can do the work.
- **Dependency-safe** - only references lower-numbered stories.
- **Schema-correct** - frontmatter fields match `config.json`; HTML markers present and balanced.

A story MUST NOT:

- Mix unrelated changes (e.g. "add field X and rewrite logging").
- Put implementation details in `Description` (those go in `Implementation Plan`).
- Put steps in `Acceptance Criteria` (those describe the outcome, not the path).
- Reference higher-numbered stories in `dependencies`.
- Be marked `Done` without all AC checked and `Final Summary` filled.

## AC Conventions

- Number each AC: `- [ ] #1 ...`. The store renders these in the GUI.
- Each AC is one outcome. Split compound criteria.
- Use the past/present-tense outcome form: *"`parse_workflow_from_value` rejects entries with empty `key`"*, not *"add validation for empty key"*.
- Test coverage shows up as its own AC: *"#7 Unit tests cover absent field, empty array, duplicate key rejection"*.

## Workflow

1. **Read project contracts** - resolve the project, then read its stories config and workspace layout.
2. **Create through the store** - call `story_create` or `POST /api/projects/{pid}/stories` with the title and supported initial fields. Do not reserve an id and create a file manually.
3. **Author through the store** - use story update, criterion, and metadata tools or their documented HTTP endpoints. Pass the latest conflict token or client mtime where required.
4. **Verify canonical identity** - call `story_get` after creation and after every title change. A successful read confirms that id, title, serialization, and canonical filename agree.
5. **Hand off** - use the `work` skill after the final store read succeeds.

When editing an existing story, use the dedicated mutation tool or HTTP endpoint. A title update must go through the store so it renames the file atomically. Never rename the Markdown file yourself.

## Milestones

Create and update milestones through `milestone_create`, `milestone_set_title`, `milestone_set_description`, and membership tools or their documented HTTP equivalents. The store owns the id, serialization, and canonical filename.

```markdown
---
id: m-<n>
title: <title>
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Outcome the milestone represents.
<!-- SECTION:DESCRIPTION:END -->
```

Set membership with `story_set_milestone` or the equivalent HTTP endpoint. The `milestone` frontmatter field is authoritative; an `m-*` label is not membership.

## Decisions and Docs

These are knowledge artifacts and live under `.s_e_e/knowledge/`. Use the `doc` skill ([../doc/SKILL.md](../doc/SKILL.md)) - different shape (no AC/plan/notes, different frontmatter). Do not author decisions or docs from this skill.

## Definition of Done

- All AC marked `[x]`.
- `Implementation Notes` summarizes what actually changed (modified files, trade-offs, deviations from the plan).
- `Final Summary` summarizes the outcome and verification (which tests/quality runs passed).
- `status: Done` and `updated_date` bumped.

## Complete Example

`story_get` for `story-241`:

```markdown
---
id: story-241
title: Render runtime input defaults in pre-run modal
status: New
assignee: []
created_date: '2026-04-25 09:30'
updated_date: '2026-04-25 09:30'
labels:
  - gui
  - doc-13
dependencies:
  - story-209
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The pre-run runtime-inputs modal shows declared keys but ignores the `default` field on each declaration. Operators have to retype values that the workflow author already supplied. The modal should pre-populate every input from its declaration's default and let the operator edit before submit.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `RuntimeInputsForm` initializes each field's draft value from the declaration's `default` when present
- [ ] #2 Boolean defaults render as the corresponding checkbox state
- [ ] #3 Number defaults render as their numeric form (no quotes)
- [ ] #4 String defaults render verbatim, including empty string
- [ ] #5 Submit posts the operator's final values, not the defaults, even when unchanged
- [ ] #6 Component test covers all four input types with a default and one without
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
- Update `gui/src/components/forms/runtime_inputs_form.rs` `initial_value_for` to read `decl.default` per type
- Adjust the form state seed in the parent hook to use the new helper
- Add a component test in the sibling `_tests.rs` file
- Verify `mise run quality` passes
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
<!-- SECTION:FINAL_SUMMARY:END -->
```

## Related

- Story implementation flow: [../work/SKILL.md](../work/SKILL.md)
- Knowledge docs and decisions: [../doc/SKILL.md](../doc/SKILL.md)
- Domain: `.s_e_e/knowledge/docs/overview/stories/doc-2 - 2-Stories-and-Knowledge.md`
- Store config: `.s_e_e/config.json` (`stories` section)
