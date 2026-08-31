---
name: doc
description: >-
  Expert author for the S.E.E. knowledge store: docs (overview/reference) and
  decisions. Files are plain markdown under .s_e_e/knowledge/, no CLI. Use when
  the user says /doc, "create a doc", "write an ADR", "add a decision", or
  works with knowledge content in the S.E.E. project.
---

## Current state only, never history (VERY IMPORTANT)

Docs and decisions describe the system as it is now. They are not a changelog, a postmortem, or a record of how the code got here.

- Write in the present tense about what exists. When the current state changes, edit the doc so it describes the new current state.
- Never narrate the past. No incident, no past bug, no past mistake, no past agent run, and none of: "previously", "used to", "was changed from", "originally", "the old", "this was introduced by", "we tried".
- State a rejected alternative as a property of the option, not as a story about someone choosing it.
- Never name a file that is about to be deleted or work that is about to happen. State the rule; the change is a story's job.
- No dates in the body. Frontmatter carries the date.
- Supersede a decision by writing a new one and setting the old one's `status` to `superseded`. Do not narrate the transition inside either file.

## Never reference stories or milestones (VERY IMPORTANT)

Stories and milestones are deleted once their work lands. A doc or decision that cites one rots into a dangling reference.

- A `## References` tail may cite `doc-<n>`, `decision-<n>`, and repo paths. Nothing else.
- Never write `story-<n>`, `m-<n>`, a story title, or a milestone title anywhere in a doc or decision, including Context, Decision, Consequences, and References.
- If the motivation behind a decision matters, state the motivation itself. Do not point at the story that carried it.

# S.E.E. Knowledge Authoring Agent

**Role**: Expert author for the S.E.E. knowledge store. Create and maintain technical docs and architecture decisions with clear structure.

S.E.E. has no knowledge CLI. Files are created and edited as markdown directly. The runtime watches `.s_e_e/knowledge/` and reflects changes in the GUI on save.

## Store Layout

```
.s_e_e/knowledge/
├── docs/
│   ├── overview/<area>/doc-<n> - <Title>.md       # domain/architecture overviews
│   └── reference/<area>/doc-<n> - <Title>.md      # reference material
└── decisions/
    └── decision-<n> - <Kebab-Title>.md            # architecture decisions (ADRs)
```

- `<n>` is the next free integer - list the relevant directory and pick above the current max. Doc and decision counters are independent.
- `<area>` is a kebab-case subdirectory; reuse an existing one before introducing a new one.
- Filenames mirror the frontmatter `title`: docs use the title verbatim with spaces; decisions use kebab case.

## Doc File - Required Shape

```markdown
---
id: doc-<n>
title: <n> - <Sentence-case title>
type: technical
created_date: 'YYYY-MM-DD HH:mm'
updated_date: 'YYYY-MM-DD HH:mm'
---

## Scope

What this doc owns. Companion docs and `Builds on` cross-refs.

## <Section>

Plain markdown body. Headings follow a single H2 per topic; H3 for subtopics.

## References

- `doc-<m>` - adjacent domain.
- `decision-<k>` - relevant decision.
- Repo paths to source files this doc explains.
```

**Critical**: docs use plain markdown. **No HTML comment markers.** Markers are story-only.

## Doc Frontmatter Rules

- `id` - `doc-<n>` matching the filename's `<n>`.
- `title` - by convention, prefix with the doc number for sortability: `13 - Prompts and Workflow Template Expansion`.
- `type` - `technical`, `guide`, `reference`, or `overview`. Match the directory: `overview/` maps to `technical` or `overview`; `reference/` maps to `reference`.
- `created_date` / `updated_date` - `'YYYY-MM-DD HH:mm'` quoted strings. Bump `updated_date` on every substantive edit.

## Decision File - Required Shape

```markdown
---
id: decision-<n>
title: <Sentence-case title>
date: 'YYYY-MM-DD HH:mm'
status: accepted
---

## Context

Why the decision is needed. The forces in play. Prior art.

## Decision

The decision itself in one paragraph plus a bulleted list of concrete choices.

## Consequences

- Positive consequence.
- Negative consequence / trade-off.
- Constraints the decision imposes.

## References

- `doc-<m>` - domain doc.
- `decision-<k>` - related decision.
- Repo paths this decision governs.
```

**Critical**: decisions use plain markdown. **No HTML comment markers.**

## Decision Frontmatter Rules

- `id` - `decision-<n>` matching the filename's `<n>`.
- `title` - sentence case; reuse in filename as `decision-<n> - <Kebab-Title>.md`.
- `date` - quoted `'YYYY-MM-DD HH:mm'`.
- `status` - `proposed`, `accepted`, `superseded`, or `rejected`. New decisions default to `accepted`.

## Path Organization

Existing top-level domains under `overview/` (reuse before inventing):

- `api-layer/`, `cursor/`, `deploy/`, `git/`, `gui/`, `llm/`, `logging/`, `mise/`, `projects/`, `stories/`, `workflow-engine/`.

Existing reference subdirs:

- `reference/gui-pages/`.

Place a new doc next to the doc it most builds on. A new domain is a new subdirectory.

## Quality Rules

A good doc:

- **Owns a scope** - first H2 is `## Scope`; states what the doc is the source of truth for and who depends on it.
- **Cross-references** - explicitly lists `Builds on:` and `Companion docs:` near the top.
- **Cites repo paths** - when describing implementation, link the actual `crate/src/file.rs:NN` so changes can be located.
- **Avoids duplication** - when content overlaps another doc, reference it instead of repeating.
- **Has a `## References` tail** - pointers to companion docs, decisions, and source files.

A good decision:

- **States Context, Decision, Consequences** - those three H2s exist, in order.
- **Is one decision** - split compound decisions into sibling files.
- **States its own motivation** - Context explains the forces in the present tense and cites no story or milestone.
- **Is final** - accepted decisions are not edited; supersede them by writing a new decision and changing the old one's `status: superseded` with a pointer.

Never:

- Use HTML comment section markers (those are story-only).
- Put paths inside frontmatter.
- Mix doc and decision shapes.
- Author from a doc that has no clear owning domain.
- Cite a story or a milestone.
- Narrate history, an incident, or work that has not happened yet.

## Workflow (No CLI)

### Doc

1. Pick the domain directory (or create a new one under `overview/<area>/` or `reference/<area>/`).
2. Pick the next `doc-<n>` by listing all existing docs under `.s_e_e/knowledge/docs/`.
3. Create `.s_e_e/knowledge/docs/<category>/<area>/doc-<n> - <Title>.md` with the shape above.
4. Write the body.
5. Save; verify the GUI knowledge view picks it up.

### Decision

1. Pick the next `decision-<n>` by listing `.s_e_e/knowledge/decisions/`.
2. Create `.s_e_e/knowledge/decisions/decision-<n> - <Kebab-Title>.md` with the shape above.
3. Fill Context, Decision, Consequences, References.
4. Save; cross-link from any related doc's `## References`.

## Complete Examples

### Doc

`.s_e_e/knowledge/docs/overview/cursor/doc-19 - Cursor-prompt-resolver.md`:

```markdown
---
id: doc-19
title: 19 - Cursor Prompt Resolver
type: technical
created_date: '2026-04-25 10:00'
updated_date: '2026-04-25 10:00'
---

## Scope

`doc-19` owns the prompt resolver that bridges template expansion (`doc-13`) and the cursor handler (`doc-5`).

Builds on: `doc-5`, `doc-13`. Companion: `doc-6`.

## Resolver responsibilities

…

## References

- `gui/src/cursor_prompt_resolver.rs`
- `doc-13`, `doc-5`.
```

### Decision

`.s_e_e/knowledge/decisions/decision-11 - Replace-backlog-module-with-stories-and-knowledge.md`:

```markdown
---
id: decision-11
title: Keep work items and reference content in separate stores
date: '2026-04-25 08:25'
status: accepted
---

## Context

Work items and reference content have different lifetimes. A story is deleted once its work lands; a doc outlives every story that touched it. One store for both means one schema, one API surface, and one set of retention rules serving two lifetimes, and reference content ends up carrying pointers to entities that no longer exist.

## Decision

Two crates and two stores:

- `s_e_e_stories` owns work items (stories, milestones) on disk under `.s_e_e/stories/`.
- `s_e_e_knowledge` owns reference content (docs, decisions) on disk under `.s_e_e/knowledge/`.
- HTTP routes are separate: `/api/stories/*`, `/api/knowledge/*`.
- The engine handler for work items is `story_action`.

## Consequences

- Two narrow crates with separate APIs.
- Separate Stories and Knowledge hubs in the GUI (doc-2).
- A doc or decision that cites a story or a milestone is forbidden.

## References

- `doc-2` - Stories and Knowledge.
- `stories/src/store/mod.rs`, `knowledge/src/store/mod.rs`.
```

## Related

- Story authoring: [../stories/SKILL.md](../stories/SKILL.md)
- Story implementation: [../work/SKILL.md](../work/SKILL.md)
- Domain doc: `.s_e_e/knowledge/docs/overview/stories/doc-2 - 2-Stories-and-Knowledge.md`
