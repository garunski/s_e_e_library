# Publish to the catalog

After authoring a package payload, add it to this repository and validate before
opening a pull request.

## Payload layout

Place files under:

```
packages/&#123;slug&#125;/&#123;version&#125;/
```

Examples:

- `packages/cursor-agent/1.0.0/command.json`
- `packages/system-implement-story/1.0.0/prompt.json`
- `packages/work/1.0.0/SKILL.md`
- `packages/implement-story/1.0.0/system-implement-story/definition.json`

Add `packages/&#123;slug&#125;/s_e_e_package.json` metadata for every catalog entry.

Bundles copy member payloads under the bundle slug folder, for example
`packages/implement-story/1.0.0/work/SKILL.md`.

## `catalog.json` entry

Append a package object to `catalog.json` `packages[]`:

| Field | Purpose |
| --- | --- |
| `id` | Globally unique id (e.g. `cmd-cursor-agent`, `bundle-implement-story`) |
| `slug` | Url-safe slug matching the `packages/` folder |
| `category` | `workflow`, `prompt`, `skill`, `command`, or `bundle` |
| `version` | Semver string (e.g. `1.0.0`) |
| `author` | Publisher id |
| `verified` | `true` when signed off by a maintainer |
| `license` | SPDX id (e.g. `AGPL-3.0-only`) |
| `requires` | External tools required at install time (empty array when none) |
| `dependencies` | Other package `id`s installed alongside this one |
| `files[]` | Install map with `to` and `from` paths |

Optional presentation fields: `name`, `description`, `labels`.

Example command entry:

```json
{
  "id": "cmd-cursor-agent",
  "slug": "cursor-agent",
  "category": "command",
  "version": "1.0.0",
  "author": "garunski",
  "verified": true,
  "license": "AGPL-3.0-only",
  "requires": [],
  "dependencies": [],
  "files": [
    {
      "to": ".s_e_e/commands/cursor-agent.json",
      "from": "packages/cursor-agent/1.0.0/command.json"
    }
  ],
  "name": "Cursor Agent",
  "description": "Cursor CLI agent run against a spoke workspace."
}
```

## Validate and build

```bash
npm run validate
npm run build
```

- `npm run validate` runs `node scripts/build-catalog.mjs --check` (read-only;
  fails when payloads or metadata are invalid or out of sync).
- `npm run build` validates payloads, refreshes `catalog.json` `updated`, and
  writes any derived fields.

Commit both the payload under `packages/` and the updated `catalog.json`.

## Docs site

```bash
npm run docs:build
```

Builds the VitePress site to `.site/` and copies `catalog.json` plus `packages/`
so GitHub Pages URLs stay stable.

## Schema reference

- Catalog manifest: [`see.library/v1`](/catalog.json)
- [Repository README](https://github.com/garunski/s_e_e_library/blob/main/README.md)
- [Library Catalog Schema (doc-23)](https://github.com/garunski/s_e_e/blob/main/.s_e_e/knowledge/docs/overview/library/doc-23%20-%20Library%20Catalog%20Schema.md)
