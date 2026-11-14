#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
staging="${SEE_LIBRARY_PAGES_STAGING:-${root}/target/pages-artifact}"

cargo run -q --manifest-path "${root}/Cargo.toml" -p see_library_catalog_tool -- \
  --root "${root}" package-pages --staging "${staging}"

cp -R "${root}/public/schema" "${staging}/schema"

echo "site-package-pages: staged artifact at ${staging}"
