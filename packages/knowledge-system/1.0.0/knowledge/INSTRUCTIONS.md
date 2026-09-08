# Knowledge brief

Audience: coding agents working in this workspace. Humans may read the same pages.

## Rules

- Present tense only. Describe what exists.
- Cite implementation as `crate/src/file.rs` (line range when a claim is local).
- Do not cite story ids, milestone ids, story titles, or milestone titles.
- Do not write audits, backlogs, changelogs, or one-shot measurement notes.
- Do not invent files, APIs, or behavior. If evidence is missing, omit the claim.
- One topic page per component. Cross-link instead of repeating.
- `INSTRUCTIONS.md` is operator control metadata. Init and refresh read it. They do not rewrite it.
- Published docs live under `knowledge/docs/` via the project HTTP API. Component notes live under `knowledge/components/`.
- Decisions under `knowledge/decisions/` are ADRs. Do not regenerate them from this inventory.

## Inventory

Each item is one component note and one published topic page. The first token is the component name. Init reads these names.

- workspace-layout: hub vs spoke, path tokens. Seed: `types` `HubDataLayout` / `Project`, `core` registry and discovery.
- hub-files: files as control plane, marker, stores under `<hub>/.s_e_e/`. Seed: `hub`, `decision-2`, `decision-3`.
- http-surface: GUI talks HTTP only. Seed: `server`, `server_write`, `decision-4`.
- workflow-engine: definitions, task handlers, templates. Seed: `engine`, `workflow_runtime`, `core/schema/workflow.schema.json`.
- orchestrator: proposer, not actor; chat does not act. Seed: `orchestrator`, `decision-10`, `decision-11`.
- stories-store: board, statuses, milestones. Seed: `stories`.
- knowledge-store: docs, decisions, changes, components. Seed: `knowledge`.
- gui: crate split, design-system pointer. Seed: `gui`, `gui_kit`, `<hub>/design-system/llms.txt`. Do not duplicate the design-system file.
- quality: Rust and shell, kiss, `mise run quality`, sibling tests. Seed: `decision-15`, `.mise/scripts/quality.sh`, `.kissconfig`.
- library: catalog vs live machinery. Seed: `library_ops`, `decision-12`.

## Index

Publish a root index in `llms.txt` shape: H1, one-line summary, grouped links with one-line descriptions. Point `AGENTS.md` at that index and the workspace-layout page.
