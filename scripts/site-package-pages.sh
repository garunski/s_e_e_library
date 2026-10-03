#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dx_public="${1:-${root}/target/dx/wasm32-unknown-unknown/release/web/public}"
staging="${SEE_LIBRARY_PAGES_STAGING:-${root}/target/pages-artifact}"

cargo run -q --manifest-path "${root}/Cargo.toml" -p see_library_catalog_tool -- \
  --root "${root}" package-pages --dx-public "${dx_public}" --staging "${staging}"

echo "site-package-pages: staged artifact at ${staging}"
