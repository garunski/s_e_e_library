---
name: work
description: >-
  Implements an S.E.E. story with tests until acceptance criteria pass: load
  the story file directly, research, create a plan, and implement in
  tight loops, mark Done by editing the story file. Use when the user says
  work, "start work", "implement story-N", or works through story IDs with
  acceptance criteria.
---

# Work

Implement an S.E.E. story with tests until all acceptance criteria pass. The story is a markdown file under `.s_e_e/stories/stories/`; there is no CLI. All metadata and content updates are direct file edits.

For **build**, **test**, and **quality** commands: infer from the repo (`mise.toml`, `Cargo.toml`, `Makefile`, `package.json` scripts, CI config). The S.E.E. workspace conventionally uses:

- `cargo check --workspace` and `cargo test -p <crate>` for Rust crates.
- `mise run quality` for the inner-loop and completion gate (formatters, lint probes, kiss, GUI guards, asset builds, unit tests via the nextest `fast` profile). CI additionally runs `mise run quality-full` (everything in `quality` plus clippy, wasm build, full nextest, npm audit).

Do not assume a specific runner without checking the repo.

## Step 1: Load the Story

Read `.s_e_e/stories/stories/story-<id> - <title>.md`. Capture:

- Description.
- Acceptance Criteria (each `- [ ] #N ...`).
- Implementation Plan.
- Frontmatter `status`, `dependencies`, `labels`.

If `status` is not `New` or `Ready for Dev`, stop and confirm with the user before proceeding.

If `dependencies` lists any story not yet `Done`, stop and surface that to the user.

## Step 2: Set Status to In Progress

Edit the story frontmatter:

- `status: In Progress`
- `updated_date: '<now>'`

Do not touch any HTML section markers in this step.

## Step 3: Research & Plan

### Research

- Read every file the Implementation Plan references.
- Search the codebase for similar implementations.
- Note any imports, traits, or patterns to reuse.
- Skim the relevant doc(s) referenced by the story labels (e.g. `doc-13` for template expansion, `doc-18` for workflow authoring).

### Output Plan

```
## Implementation Plan

### Files to Modify
- path/to/file.rs - what changes and why
- path/to/file_tests.rs - what tests to add

### Steps
1. Specific change with rationale
2. Specific change with rationale

### Tests Required
- Unit test: what it covers
- Integration test (if any): what it covers
- Edge cases: explicit scenarios

### AC Verification
- AC #1: how this plan satisfies it
- AC #2: how this plan satisfies it
```

Then proceed immediately to Step 4 — do not wait for user approval.

## Step 4: Implement Loop

Repeat until all AC pass:

1. Make one focused change.
2. Add or update tests for that change.
3. Run the project **build** (e.g. `cargo check --workspace`). On failure, fix and repeat from step 3.
4. Run the relevant **tests** (e.g. `cargo test -p <crate>`). On failure, fix and repeat from step 3.
5. Note which AC the change satisfies.

After each iteration, state:

- What you just did.
- Which AC(s) now pass.
- What is next, or: **All AC pass, ready to complete.**

Run **`mise run quality`** after substantive changes during the implement loop and again before declaring done — the completion gate. Do not declare done if `quality` fails.

### Per-AC Tracking

As each AC passes, edit the story file and flip its checkbox: `- [ ] #N ...` → `- [x] #N ...`. Do not flip checkboxes speculatively.

## Step 5: Fill In Notes and Final Summary

Before marking Done, edit the story file:

- `## Implementation Notes` — short bullet list of what actually changed: modified files, trade-offs, deviations from the plan, key tests added.
- `## Final Summary` — one-paragraph summary of the outcome and verification (which crates' tests passed, whether `mise run quality` passed, any caveats).

Both sections must stay between their existing HTML comment markers (`<!-- SECTION:NOTES:BEGIN -->` / `END`, `<!-- SECTION:FINAL_SUMMARY:BEGIN -->` / `END`). Do not duplicate or rename them.

## Step 6: Mark Done

Edit the story frontmatter:

- `status: Done`
- `updated_date: '<now>'`

Save. The runtime watcher updates the GUI.

## Rules

- Do proper research before coding.
- Create a plan (Step 3), then implement (Step 4) without waiting for user approval.
- Every code change needs a test.
- Never proceed past a failing build or test.
- State progress after each iteration.
- Do not mark Done until all AC pass and `mise run quality` is green.
- Do not flip AC checkboxes you did not verify.
- Keep code and comments minimal and terse.
- Be terse in assistant responses unless the user asks for detail.
- Do not edit other stories or knowledge files unless the AC explicitly require it.

## Definition of Done

- [ ] All AC checked `[x]`.
- [ ] Implementation Notes filled in.
- [ ] Final Summary filled in.
- [ ] `status: Done`, `updated_date` bumped.
- [ ] Workspace builds; targeted tests pass.
- [ ] `mise run quality` passes (or any failing probe is unrelated and explicitly called out in Final Summary).

## Related

- Story authoring conventions: [../stories/SKILL.md](../stories/SKILL.md)
- Knowledge docs and decisions: [../doc/SKILL.md](../doc/SKILL.md)
- Stories domain doc: `.s_e_e/knowledge/docs/overview/stories/doc-2 - 2-Stories-and-Knowledge.md`
- Workflow authoring (relevant for engine/handler stories): `.s_e_e/knowledge/docs/overview/workflow-engine/doc-18 - 18-Workflow-Authoring-Guide.md`
