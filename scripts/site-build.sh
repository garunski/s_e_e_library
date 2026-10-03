#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${root}/site"
DIOXUS_ASSET_ROOT=/s_e_e_library SEE_LIBRARY_PAGES_BASE=/s_e_e_library \
  dx bundle --release --web --fullstack --package see_library_site \
  --client-target wasm32-unknown-unknown \
  --base-path s_e_e_library \
  @client --no-default-features --features web \
  @server --no-default-features --features server --target x86_64-unknown-linux-gnu
SEE_LIBRARY_PAGES_BASE=/s_e_e_library bash "${root}/scripts/site-prerender.sh"
