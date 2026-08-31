## Workspace layout

This checkout is a **library spoke**. It hosts the public catalog and a Next.js documentation site; it has **no hub data root**.

Sibling checkouts:

- Hub (admin spoke): `../s_e_e_project/` (hub data under `../s_e_e_project/.s_e_e/`)
- Code monorepo: `../s_e_e/`

**Path anchoring:** In agent-facing text, filesystem paths must be absolute or prefixed with an explicit root token (`<hub>`, `<project>`, `<spoke:{name}>`); never write bare relative paths whose meaning depends on cwd. Full contract: **`doc-34`** (`<hub>/.s_e_e/knowledge/docs/overview/projects/doc-34 - Workspace Layout and Path Resolution.md`).

## Languages: Rust and shell only (VERY IMPORTANT)

Hard ban, no exceptions. Full contract: **`decision-15`** (`<hub>/.s_e_e/knowledge/decisions/decision-15 - Keep the workspace to Rust and shell.md`).

This checkout is the one place TypeScript is allowed, and only the minimum the Next.js documentation site needs. Everywhere else the workspace is Rust for application code and bash for infrastructure.

- NO PYTHON. No `.py` files, no `python` or `python3` invocation, no `python -c`, no Python heredoc inside a shell script, no `__pycache__`. This includes throwaway work inside a single turn: never reach for Python to check a number, parse a file, or verify your own output. `scripts/*.mjs` here are the site's existing Node build steps; do not add more, and do not introduce a new language beside them.
- No other language anywhere, in any checkout, for any purpose: no Go, Ruby, Perl, Lua, or Python.
- A shell script whose body launches another interpreter is a violation of this rule, not a way to satisfy it.
- Infrastructure has no tests. Build and copy steps ship without test files; they prove themselves by running. `npm test` covers site content, not the build glue.
- TOML, JSON, YAML, and JSON Schema are data, not languages, and are unaffected.

If you believe a task genuinely needs another language, stop and ask the user first. Do not write it speculatively. If review surfaces a language violation you introduced, delete the file without argument.
