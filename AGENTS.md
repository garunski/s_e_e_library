## Workspace layout

This checkout is a **library spoke**. It hosts the public catalog and a Dioxus documentation site; it has **no hub data root**.

Sibling checkouts:

- Hub (admin spoke): `../s_e_e_project/` (hub data under `../s_e_e_project/.s_e_e/`)
- Code monorepo: `../s_e_e/` (path dependency for `site/`; CI pins a revision in `.github/see-code-revision`)

**Path anchoring:** In agent-facing text, filesystem paths must be absolute or prefixed with an explicit root token (`<hub>`, `<project>`, `<spoke:{name}>`); never write bare relative paths whose meaning depends on cwd. Full contract: **`doc-1`** (`<hub>/.s_e_e/knowledge/docs/overview/projects/doc-1 - Workspace Layout and Path Resolution.md`). Index: **`doc-11`** (`<hub>/.s_e_e/knowledge/docs/overview/doc-11 - S-E-E- knowledge index.md`).

## Languages: Rust and shell only (VERY IMPORTANT)

Hard ban, no exceptions. Full contract: **`decision-15`** (`<hub>/.s_e_e/knowledge/decisions/decision-15 - Keep the workspace to Rust and shell.md`).

- Application and site code is Rust under `catalog_tool/` and `site/`.
- Infrastructure is bash under `scripts/*.sh`, using only bash and the pinned text tools (`awk`, `sed`, `grep`, `rg`, `sort`, `cut`, `jq`, `cargo`).
- NO PYTHON. No `.py` files, no `python` or `python3` invocation.
- No TypeScript, Node, npm, Next.js, or React in this checkout.
- No other language anywhere for this spoke.
- Infrastructure shell scripts have no test files; they prove themselves by running. Rust tests live in `*_tests.rs` siblings per hub `AGENTS.md`.
- TOML, JSON, YAML, and JSON Schema are data, not languages, and are unaffected.

If you believe a task genuinely needs another language, stop and ask the user first.

## Git: work on main, never detach HEAD (VERY IMPORTANT)

Every checkout in this workspace commits directly to `main`. There is no branching model and no feature branches.

- Never create a branch: no `git branch <name>`, no `git checkout -b`, no `git switch -c`, no pushing a branch to origin.
- Never check out a bare commit SHA or tag. `git checkout <sha>` detaches HEAD.
- Before committing, confirm `git symbolic-ref --short HEAD` prints `main`.
- To read history, use `git show <sha>`, `git log`, or `git diff <sha>`. None of those move HEAD.
- To discard local work, use `git restore` or `git reset` while attached to `main`.

If review surfaces a branch or a detached commit you created, reattach to `main` and fast-forward it before continuing.

## Story-authoring packages

Every story-authoring skill published from this checkout must require writes through the story MCP tools or documented project HTTP API. It must never instruct an agent to create, rename, or update store Markdown directly. If neither store interface is reachable, the skill must stop rather than fall back to a guessed id, filename, frontmatter shape, `apply_patch`, or shell write. Store HTTP writes accept `conflict_token` in the query `?conflict_token=`, the JSON body field `conflict_token`, or the header `x-s-e-e-conflict-token`; PUT bodies may omit a redundant entity `id` and take it from the URL path.

Story and milestone titles must use only ASCII letters, numbers, and spaces so filenames stay plain as `<id> - <title>.md`. Every create or title change must end with a store read of the entity. Keep each changed package skill identical in `packages/` and the staged Pages artifact under `target/pages-artifact/packages/` when you run `mise run build`.

Every story-authoring skill published from here must also state that a story contains no choices. A story is a fully planned piece of work; the implementing agent executes it, it does not design it. The skill must forbid "decide whether", "choose between", "either X or Y", "consider whether", "prefer X", and "record the decision" in a description, plan, or acceptance criterion, require that one approach is picked and named with a single sentence on why the other loses, require thresholds and limits to be written as numbers, and require the author to stop and ask rather than publish a story with the choice embedded.

## Site and catalog tasks

From the library root:

- `mise run validate`, `mise run catalog`, `mise run harvest`, `mise run discover-bundles`
- `mise run dev` (Dioxus dev server)
- `mise run build` (production Pages staging tree)
- `mise run quality` (Rust tests, build, smoke checks)

When you change catalog wire schema or documentation routes, keep `public/schema/*.schema.json`, `site/content/**`, `public/llms.txt`, and the in-app library help articles in `<spoke:s_e_e>` in sync per `<spoke:s_e_e>/AGENTS.md`.
