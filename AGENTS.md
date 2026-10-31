# S E E Library spoke

Read `/home/fang/code/s_e_e_project/s_e_e_project/AGENTS.md` for shared workspace rules and root tokens. This checkout owns the public catalog and Dioxus documentation site; it has no hub store. The site uses `<spoke:s_e_e>` as a path dependency. Rust tests follow `<spoke:s_e_e>/AGENTS.md`.

## Publication

- Catalog payloads live in `<spoke:s_e_e_library>/packages/` and `<spoke:s_e_e_library>/catalog.json`. A build stages them under `<spoke:s_e_e_library>/target/pages-artifact/`; staged package skills must match their source.
- Published story-authoring skills must preserve store-backed writes, store-assigned names/ids, and the readiness/completion contracts in the canonical story and milestone authoring prompts. Do not embed a competing procedure.

## Documentation synchronization

When catalog wire contracts, category/install/path rules, workflow schemas, template expansion or overlapping in-app Library guidance change, update the corresponding public documentation in the same change or record an explicit follow-up.

The public site must stand alone. Maintain its authoring/schema pages under `<spoke:s_e_e_library>/site/src/pages/`, examples under `<spoke:s_e_e_library>/site/content/`, formal schemas under `<spoke:s_e_e_library>/public/schema/`, and matching `Source: /path/` sections in `<spoke:s_e_e_library>/public/llms.txt`. Keep overlapping help under `<spoke:s_e_e>/gui_kit/src/pages/help_content/articles/` consistent.

## Checks

Run from `<spoke:s_e_e_library>`: `mise run validate` for read-only catalog checks, `mise run build` for Pages staging, and `mise run quality` for changed published pages or schemas (Rust tests, build and smoke checks). Use `mise run dev` only when requested.

<!-- s-e-e:begin -->
## S.E.E. project

S.E.E. maintains this section and rewrites it when S.E.E. starts or the project changes. Keep repository rules outside it.

This repository belongs to the S.E.E. project `S-E-E` (project id `s-e-e`). Its stories, milestones, knowledge docs, decisions, and workflows live in the S.E.E. store.

- Use the `see` MCP server configured for this repository. Pass `project` `s-e-e` on every tool call.
- Read and change S.E.E. records only through `see` tools: `item_fetch` and `item_call`, or named tools such as `story_get`, `story_update`, `story_check_criterion`, `story_set_status`, and `knowledge_doc_get`. Never edit files under `../s_e_e_project/.s_e_e/` directly.
- Pass the `conflict_token` from your latest read on every write. After a conflict, read again and retry.
- Read valid story statuses from the `see:/projects/s-e-e/stories/config` resource instead of guessing them.
- If the `see` server is unavailable, stop and report it. Do not fall back to editing store files.
- S.E.E. task skills are installed under `../s_e_e_project/.agents/skills/`: `work` implements a story, `stories` writes stories and milestones, `doc` writes knowledge docs and decisions, and `workflow` writes S.E.E. workflows. Read the matching `SKILL.md` before that kind of task.
<!-- s-e-e:end -->
