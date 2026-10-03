#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${root}/site"
DIOXUS_ASSET_ROOT=/s_e_e_library SEE_LIBRARY_PAGES_BASE=/s_e_e_library \
  dx serve --web --fullstack --package see_library_site \
  --client-target wasm32-unknown-unknown \
  --addr 127.0.0.1 --port 5173 \
  @client --no-default-features --features web \
  @server --no-default-features --features server
