# S.E.E. Library

Read `/home/fang/code/s_e_e_project/s_e_e_project/AGENTS.md` for shared rules. For Rust tests, follow the application spoke's rules.

- Keep `packages/` and `catalog.json` in sync. Packaged authoring skills follow the stored procedures and store-assigned IDs.
- For contract changes, sync the public pages, examples, schemas, matching `public/llms.txt` sections, and overlapping in-app help. Record an explicit follow-up for any deferred update.
- Use `mise run validate` for catalog checks and `mise run build` for Pages staging. Start the dev site only on request.

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
