# S.E.E. Official Library

The Official Library distributes versioned workflows, prompts, skills, commands,
bundles, and cycles for [S.E.E.](https://github.com/garunski/s_e_e).

- [Catalog manifest](https://garunski.github.io/s_e_e_library/catalog.json) uses `see.library/v2`.
- Packages are published at `https://garunski.github.io/s_e_e_library/packages/{slug}/{version}/{file}`.
- Open **Help** in S.E.E. for authoring guides and product reference.

## Repository layout

- `catalog.json` contains the package entries and their install maps.
- `packages/` holds versioned payloads and package metadata.
- `public/schema/` holds the JSON schemas used by published packages and schema references.
- `catalog_tool/` validates and generates the catalog, harvests hub payloads,
  and checks the Pages artifact.
- `scripts/stacks.json` and `scripts/bundles.json` hold marketplace and bundle data.

## Local development

Use [mise](https://mise.jdx.dev/) to install Rust 1.96.0. The catalog tool
uses application crates from a sibling `../s_e_e` checkout.

```sh
mise run validate          # Read-only catalog and payload validation
mise run catalog           # Regenerate catalog.json
mise run harvest           # Import payloads from ../s_e_e_project
mise run discover-bundles  # Print bundle IDs and member edges
mise run build             # Validate and stage the Pages artifact
mise run site-smoke-pages  # Check staged catalog and package URLs
```

To add a package, place its payload and metadata in `packages/` and run `mise run catalog`.
Commit the payload, metadata, and generated catalog together.

## GitHub Pages

The Pages workflow publishes `catalog.json`, `packages/`, and machine-readable
schemas from `target/pages-artifact`.

## License

AGPL-3.0-only. See [LICENSE](LICENSE).
