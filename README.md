# S.E.E. Official Library

Public catalog source for the [S.E.E.](https://github.com/garunski/s_e_e) Official Library: versioned packages of workflows, prompts, skills, commands, and bundles that install into hub or spoke projects.

## Catalog URL

After GitHub Pages deploy:

- Manifest: `https://garunski.github.io/s_e_e_library/catalog.json`
- Payloads: `https://garunski.github.io/s_e_e_library/packages/{slug}/{version}/{file}`
- Every page as one text file: `https://garunski.github.io/s_e_e_library/llms.txt`

Schema: `see.library/v1`; see the `/schema/` pages.

## Layout

- `catalog.json`: manifest (`packages[]` entries with `files[]` install map)
- `packages/{slug}/{version}/`: package payloads (source of truth at the repo root)
- `public/schema/`: formal JSON Schema files
- `public/llms.txt`: every authoring guide and schema page as one text file for agents
- `site/`: Dioxus 0.7.10 documentation site (`/authoring/` guides, `/schema/` contracts)
- `catalog_tool/`: Rust CLI for validate, catalog, harvest, discover-bundles, and Pages staging checks
- `scripts/stacks.json` and `scripts/bundles.json`: marketplace and bundle discovery data

## Local development

Requires [mise](https://mise.jdx.dev/) (`mise install` provisions Rust 1.96.0, `wasm32-unknown-unknown`, and Dioxus CLI 0.7.10). The site crate path-depends on a sibling checkout of `s_e_e` at `../s_e_e` (pinned in CI via `.github/see-code-revision`).

From the library repository root:

```bash
mise run harvest
mise run catalog
mise run validate
mise run discover-bundles
mise run dev          # http://127.0.0.1:5173/s_e_e_library/
mise run build        # validate, dx bundle, prerender, stage target/pages-artifact
mise run quality      # unit tests, build, HTTP smoke on the staged artifact
```

`harvest` copies payloads from the sibling hub checkout (`../s_e_e_project` by default). Pass `--hub /path/to/hub` to the catalog tool for another hub root.

`discover-bundles` prints bundle ids and member edges to stdout. Pass `--write` to refresh `scripts/bundles.json`.

`validate` is read-only and fails when `catalog.json` is stale relative to `packages/`.

## Adding a package

1. Add payload under `packages/{slug}/{version}/`.
2. Append a manifest entry to `catalog.json` (`id`, `slug`, `category`, `version`, `author`, `verified`, `license`, `requires`, `dependencies`, `files[]`).
3. Run `mise run catalog` and commit both the payload and updated `catalog.json`.

Install `to` paths must start with `.s_e_e/`, `.agents/`, `.cursor/`, or `templates/`.

## Docs site

The public site is a Dioxus fullstack crate under `site/`. Authoring copy lives in `site/content/authoring/`; schema page copy in `site/content/schema/`. After you change a page, update the matching section in `public/llms.txt`. `mise run quality` runs Rust tests that keep `llms.txt` aligned with the routed pages.

## GitHub Pages

1. Repo **Settings, Pages, Build and deployment, Source: GitHub Actions**.
2. Push to `main`; `.github/workflows/pages.yml` validates the catalog, builds and prerenders the Dioxus site, stages `target/pages-artifact`, and deploys that tree (docs, `catalog.json`, `packages/`, schemas, `llms.txt`).

## License

AGPL-3.0-only; see [LICENSE](LICENSE).
