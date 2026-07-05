# S.E.E. Official Library

Public catalog source for the [S.E.E.](https://github.com/garunski/s_e_e) Library hub: versioned packages of workflows, prompts, skills, and commands installable into hub or spoke projects.

## Catalog URL

After GitHub Pages deploy:

- Manifest: `https://garunski.github.io/s_e_e_library/catalog.json`
- Payloads: `https://garunski.github.io/s_e_e_library/packages/{slug}/{version}/{file}`

Schema: `see.library/v1` (see S.E.E. knowledge doc-23).

## Layout

- `catalog.json` — manifest (`packages[]` entries with `files[]` install map)
- `packages/{slug}/{version}/` — package payloads
- `scripts/build-catalog.mjs` — validate payloads and refresh `catalog.json` `updated` timestamp
- `scripts/harvest.mjs` — copy assets from a local S.E.E. hub checkout into `packages/` and regenerate `catalog.json`

## Local development

```bash
node scripts/harvest.mjs
npm run build
npm run validate
```

`npm run validate` runs `build-catalog.mjs --check` (no writes; fails if catalog is invalid or out of sync with `packages/`).

## Adding a package

1. Add payload under `packages/{slug}/{version}/`.
2. Append a manifest entry to `catalog.json` (`id`, `slug`, `category`, `version`, `author`, `verified`, `license`, `requires`, `dependencies`, `files[]`).
3. Run `npm run build` and commit both the payload and updated `catalog.json`.

Install `to` paths must start with `.s_e_e/`, `.agents/`, `.cursor/`, or `templates/`.

## GitHub Pages

1. Repo **Settings → Pages → Build and deployment → Source: GitHub Actions**.
2. Push to `main`; `.github/workflows/pages.yml` validates the catalog and deploys the repo root.

## License

AGPL-3.0 — see [LICENSE](LICENSE).
