---
name: doc
description: Create or revise S.E.E. knowledge docs and architecture decisions. Use for requests to document current behavior, write a knowledge page, or record an architectural decision.
---

# S.E.E. knowledge authoring

Use this skill for durable knowledge records. A repository `AGENTS.md` gives agents local operating rules; a knowledge doc explains the current system; a decision records a durable choice and its consequences. Keep task procedure in the relevant skill and implementation scope in the story. Do not copy those layers into a knowledge record.

## Access and evidence

- Resolve the project id and read `see:/projects/{project}/workspace-layout` before choosing a category or citing repository paths.
- Use `knowledge_doc_list` or `knowledge_decision_list` to find related records and avoid duplicates. Read the record with `knowledge_doc_get` or `knowledge_decision_get` before changing it.
- Use the knowledge MCP create and put tools for writes. If MCP is unavailable, use the documented project knowledge HTTP API when available. If neither store interface works, stop and report the missing capability. Never allocate an id, choose a store filename, or edit knowledge Markdown directly.
- A put needs the `conflict_token` from the latest get. If it is stale, reread, reconcile, and retry. Preserve fields outside the requested change.
- Inspect the relevant source code and applicable repository `AGENTS.md` when the document makes claims about implementation. Cite stable doc or decision ids and unambiguous repository paths. Verify each claim against current evidence.

## Docs

Use `knowledge_doc_create` with `title`, `type`, `category`, `body`, and relevant `tags`; the store allocates `doc-N`. For an update, get the record, revise its body and metadata as needed, and use `knowledge_doc_put` with the current token. Read it back after the write.

- Describe the system as it works now, in present tense. A doc is neither a changelog nor a plan for pending work.
- Start the body with `## Scope`, naming what this doc owns. Use focused sections and a `## References` section for related docs, decisions, and source paths.
- Link to another owning document instead of repeating a large explanation. Use `Builds on` or `Companion docs` only when the relationship helps a reader find the source of truth.
- Do not cite story or milestone ids or titles. Those work records have a shorter lifetime than knowledge docs.

## Decisions

Use `knowledge_decision_create` with the required `title`, `date`, `context`, `decision`, and `consequences` fields. Use `knowledge_decision_get` and `knowledge_decision_put` for an existing record, passing the latest token. Read the result back.

- Record one choice per decision. Explain the current forces in `context`, the selected rule in `decision`, and its costs and effects in `consequences`. State rejected alternatives as properties of those options, not as a history of agent attempts.
- Keep accepted decisions stable. When the rule changes, create a successor and mark the old decision superseded through the store. Link the records using stable decision ids.
- Do not cite stories or milestones in the decision body. State the underlying motivation and cite durable sources instead.

## Completion

Report the store id, title, owning scope or decision, and the sources checked. If evidence is missing, distinguish the open question from a verified fact and ask before publishing an unsupported claim.
