## Workspace layout

This checkout is a **library spoke**. It hosts the public catalog documentation site (Next.js + Nextra); it has **no hub data root**.

Sibling checkouts:

- Hub (admin spoke): `../s_e_e_project/` (hub data under `../s_e_e_project/.s_e_e/`)
- Code monorepo: `../s_e_e/`

**Path anchoring:** In agent-facing text, filesystem paths must be absolute or prefixed with an explicit root token (`<hub>`, `<project>`, `<spoke:{name}>`); never write bare relative paths whose meaning depends on cwd. Full contract: **`doc-34`** (`<hub>/.s_e_e/knowledge/docs/overview/projects/doc-34 - Workspace Layout and Path Resolution.md`).
