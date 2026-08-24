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
- `src/`: Next.js docs site (`/authoring/` guides, `/schema/` contracts)
- `scripts/build-catalog.mjs`: validate payloads and refresh `catalog.json` `updated` timestamp
- `scripts/copy-public-assets.mjs`: copy root `catalog.json` and `packages/` into `public/` for the Next.js site
- `scripts/harvest.mjs`: copy assets from a local S.E.E. hub checkout into `packages/` and regenerate `catalog.json`

## Local development

```bash
node scripts/harvest.mjs
npm run catalog
npm run validate
npm run copy
npm run dev
```

`npm run validate` runs `build-catalog.mjs --check` (no writes; fails if catalog is invalid or out of sync with `packages/`).

`npm run copy` copies root `catalog.json` and `packages/` into `public/` so the Next.js app can serve them. `npm run dev` and `npm run build` run that copy first.

## Adding a package

1. Add payload under `packages/{slug}/{version}/`.
2. Append a manifest entry to `catalog.json` (`id`, `slug`, `category`, `version`, `author`, `verified`, `license`, `requires`, `dependencies`, `files[]`).
3. Run `npm run catalog` and commit both the payload and updated `catalog.json`.

Install `to` paths must start with `.s_e_e/`, `.agents/`, `.cursor/`, or `templates/`.

## Docs site

A Next.js static site (same stack as `s_e_e_site`) lives under `src/`. Authoring guides are served at `/authoring/`, payload contracts at `/schema/`:

```bash
npm install
npm run dev      # local preview
npm run build    # copy catalog assets, then static export to out/
```

`npm test` builds the site and checks that every `/authoring/` and `/schema/` page has a matching section in `public/llms.txt`, so a new package type cannot ship with the text file out of date.

## GitHub Pages

1. Repo **Settings, Pages, Build and deployment, Source: GitHub Actions**.
2. Push to `main`; `.github/workflows/pages.yml` validates the catalog, builds the Next.js export, and deploys `out/` (docs + `catalog.json` + `packages/` + `llms.txt`).

## License

AGPL-3.0-only; see [LICENSE](LICENSE).
