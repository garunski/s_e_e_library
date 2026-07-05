---
name: stories
description: >-
  Expert story author for the S.E.E. stories store. Atomic, testable,
  AI-implementable stories written directly as markdown under
  .s_e_e/stories/stories/. Use when the user says /stories, /story, "create a
  story", "add a story", or works with stories, milestones, or labels in
  the S.E.E. project.
---

# S.E.E. Story Authoring Agent

**Role**: Expert story author for the S.E.E. stories store. Create atomic, testable, AI-implementable stories.

S.E.E. has no story CLI. Files are created and edited as markdown directly. The runtime watches `.s_e_e/stories/` and reflects changes in the GUI on save. Filenames are part of the contract.

## Store Layout

```
.s_e_e/
├── stories/
│   ├── config.yml              # statuses, labels, prefixes (read-only for agents)
│   ├── next-id                 # next numeric suffix to allocate (decimal); reserved by the store
│   ├── stories/
│   │   └── story-<n> - <Kebab-Title>.md
│   ├── milestones/
│   │   └── m-<n> - <Kebab-Title>.md
│   └── archive/stories/        # archived stories (do not write here)
└── knowledge/
    ├── decisions/decision-<n> - <Kebab-Title>.md     # see doc skill
    └── docs/<category>/<area>/doc-<n> - <Title>.md   # see doc skill
```

- `<n>` is allocated by the store — never guess from globs. Reserve via `POST /api/projects/{pid}/stories/reserve-id` (returns `{"id":"story-<n>","n":<n>}`) or read `.s_e_e/stories/next-id` only after a successful reserve/create. The on-disk counter lives at `.s_e_e/stories/next-id`.
- `<Kebab-Title>` matches the frontmatter `title` with spaces replaced by `-`. Keep punctuation minimal.
- Stories live in `.s_e_e/stories/stories/`; milestones in `.s_e_e/stories/milestones/`. Decisions and docs live under `.s_e_e/knowledge/` (use the `doc` skill).

## Story File — Required Shape

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

- `id` — `story-<n>`. Must match the filename's `<n>`.
- `title` — sentence case; reuse in the filename as `story-<n> - <Kebab-Title>.md`.
- `status` — one of the values in `.s_e_e/stories/config.yml` (`New`, `Ready for Dev`, `In Progress`, `Done`). Start at `New`.
- `assignee` — empty list on creation; populated when work starts.
- `created_date` / `updated_date` — `'YYYY-MM-DD HH:mm'` quoted strings.
- `labels` — list of strings; pick from existing labels first (look across `.s_e_e/stories/stories/`); only invent a new label when no existing one fits. Common labels reference architecture areas (e.g. `engine`, `gui`, `core`, `types`) or doc ids (e.g. `doc-13`, `doc-18`).
- `dependencies` — list of `story-<m>` ids that must be Done before this story can start. Only reference lower ids.
- `priority` — `low`, `medium`, or `high`.

## Story Quality Rules

A good story is:

- **Atomic** — one PR-sized scope; one or two source files plus tests.
- **Independent** — does not assume future stories.
- **Testable** — every AC is verifiable by running tests, checking files, or invoking an HTTP endpoint.
- **AI-implementable** — another agent reading just this story can do the work.
- **Dependency-safe** — only references lower-numbered stories.
- **Schema-correct** — frontmatter fields match `config.yml`; HTML markers present and balanced.

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

## Workflow (No CLI)

1. **Reserve the next id** — `POST /api/projects/{pid}/stories/reserve-id` (HTTP 201, body `{"id":"story-<n>","n":<n>}`). Use that id exactly; do not glob `story-*.md` to pick `<n>`. After reserve, `.s_e_e/stories/next-id` holds the next suffix (read-only confirmation).
2. **Create the file** — `.s_e_e/stories/stories/story-<n> - <Kebab-Title>.md` with the frontmatter and section markers above.
3. **Author content** — Description, AC, Plan. Leave Notes and Final Summary empty.
4. **Save** — the file watcher reflects the new story in the GUI; verify the executions/stories list shows it.
5. **Hand off** — when an agent works the story, they update `status`, check off AC, and fill Notes / Final Summary using the `work` skill.

When *editing* an existing story to change metadata (status, labels, assignee), edit the frontmatter directly and bump `updated_date`. Do not rename the file unless `title` itself changes — and if it does, both filename and frontmatter `title` change together.

## Milestones (No CLI)

Milestones live at `.s_e_e/stories/milestones/m-<n> - <Kebab-Title>.md`. Minimal frontmatter:

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

Stories may reference a milestone via a `milestone` label (e.g. `m-3`); this is convention, not required.

## Decisions and Docs

These are knowledge artifacts and live under `.s_e_e/knowledge/`. Use the `doc` skill ([../doc/SKILL.md](../doc/SKILL.md)) — different shape (no AC/plan/notes, different frontmatter). Do not author decisions or docs from this skill.

## Definition of Done

- All AC marked `[x]`.
- `Implementation Notes` summarizes what actually changed (modified files, trade-offs, deviations from the plan).
- `Final Summary` summarizes the outcome and verification (which tests/quality runs passed).
- `status: Done` and `updated_date` bumped.

## Complete Example

`.s_e_e/stories/stories/story-241 - Render-runtime-input-defaults-in-pre-run-modal.md`:

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
- Store config: `.s_e_e/stories/config.yml`
