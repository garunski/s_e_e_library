# Bundles

A **bundle** installs several library packages together. It has no payload file
of its own; the catalog entry `files[]` lists every member install map.

## Metadata (`s_e_e_package.json`)

Each bundle carries a sidecar at `packages/&#123;slug&#125;/s_e_e_package.json`:

```json
{
  "id": "bundle-implement-story",
  "slug": "implement-story",
  "category": "bundle",
  "name": "Implement Story workflows",
  "description": "Packages linked by template references: workflows, prompts, skills, commands.",
  "labels": ["bulk", "story"],
  "dependencies": []
}
```

| Field | Purpose |
| --- | --- |
| `id` | Globally unique package id (`bundle-<slug>`) |
| `slug` | Url-safe folder name under `packages/` |
| `category` | Must be `bundle` |
| `name`, `description` | Catalog presentation |
| `labels` | Discovery tags |
| `dependencies` | Other package ids installed alongside this bundle |

## Members

When harvesting from a local hub, `scripts/bundles.json` lists member package
ids per bundle slug. The harvest script copies each member payload into
`packages/&#123;bundle-slug&#125;/&#123;version&#125;/` and builds the catalog `files[]` array.
Hand-authored bundles list every `to`/`from` pair directly in `catalog.json`.

Example members for `implement-story` (from `scripts/bundles.json`):

- `cmd-cursor-agent`
- `prompt-system-implement-story`
- `skill-work`
- `wf-system-implement-story`
- `wf-system-implement-stories-bulk`

## `files[]` install map

Each entry maps a repo path to an install target:

```json
{
  "to": ".s_e_e/prompts/system-implement-story.json",
  "from": "packages/implement-story/1.0.0/system-implement-story/prompt.json"
}
```

The build infers payload kind from the `to` prefix:

- `.s_e_e/workflows/definitions/` → workflow
- `.s_e_e/prompts/` → prompt
- `.s_e_e/commands/` → command
- `.agents/skills/` → skill
- `.cursor/` → rule
- `templates/` → template

## `to` path rule

Every `files[]` `to` path must start with one of:

- `.s_e_e/`
- `.agents/`
- `.cursor/`
- `templates/`

Paths use forward slashes and must be unique within the package.

## Schema reference

- Catalog manifest: [`see.library/v1`](/catalog.json)
- [Repository README](https://github.com/garunski/s_e_e_library/blob/main/README.md)
- [Library Catalog Schema (doc-23)](https://github.com/garunski/s_e_e/blob/main/.s_e_e/knowledge/docs/overview/library/doc-23%20-%20Library%20Catalog%20Schema.md)
