# S.E.E. Official Library

The Official Library distributes versioned workflows, prompts, skills, commands,
bundles, and cycles for [S.E.E.](https://workwithsee.com/).

- [Catalog manifest](https://workwithsee.github.io/library/catalog.json) uses `see.library/v2`.
- Packages are published at `https://workwithsee.github.io/library/packages/{slug}/{version}/{file}`.
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

The Pages workflow publishes `LICENSE`, `catalog.json`, `packages/`, and machine-readable
schemas from `target/pages-artifact`.

## License

Proprietary. Copyright (c) 2026 Garunski LLC. All rights reserved. See [LICENSE](LICENSE).

Catalog entries use `LicenseRef-SEE-Proprietary` to identify these terms.
Use, modification, or redistribution requires prior written authorization.

Users are responsible for reviewing AI output and for commands, automation,
permissions, security, backups, data sharing, and provider charges. See [LICENSE](LICENSE)
for the user-responsibility, warranty, and liability terms covering Garunski LLC
and its owners, staff, contributors, and agents, subject to applicable law.

The contractual terms require express acceptance in a separate written or
electronic agreement identifying the applicable license version. Publication
of this notice alone does not establish a user's agreement to those terms.
