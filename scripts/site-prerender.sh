#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
site_root="${root}/site"
public_dir="${1:-${root}/target/dx/wasm32-unknown-unknown/release/web/public}"
server_bin="${2:-${root}/target/dx/wasm32-unknown-unknown/release/web/server}"
pages_base="${SEE_LIBRARY_PAGES_BASE:-/s_e_e_library}"
pages_base="${pages_base%/}"
port="${SITE_PRERENDER_PORT:-19927}"

if [[ ! -x "${server_bin}" ]]; then
  echo "site-prerender: missing server binary at ${server_bin}" >&2
  exit 1
fi

routes_json="$(cargo run -q --manifest-path "${site_root}/Cargo.toml" --bin see_library_site_routes)"
route_count="$(echo "${routes_json}" | jq 'length')"
if [[ "${route_count}" != "25" ]]; then
  echo "site-prerender: expected 25 routes, got ${route_count}" >&2
  exit 1
fi

IP=127.0.0.1 PORT="${port}" DIOXUS_CLI_ENABLED=1 "${server_bin}" &
server_pid=$!
trap 'kill "${server_pid}" 2>/dev/null || true' EXIT

ready=0
for _ in $(seq 1 30); do
  if curl -sf -o /dev/null "http://127.0.0.1:${port}${pages_base}/"; then
    ready=1
    break
  fi
  sleep 0.2
done
if [[ "${ready}" != "1" ]]; then
  echo "site-prerender: server did not become ready on port ${port}" >&2
  exit 1
fi

while IFS= read -r route; do
  route_dir="${public_dir}${pages_base}${route}"
  mkdir -p "${route_dir}"
  curl -sf "http://127.0.0.1:${port}${pages_base}${route}" -o "${route_dir}/index.html"
  if ! grep -q '<title>' "${route_dir}/index.html"; then
    echo "site-prerender: missing title in ${route_dir}/index.html" >&2
    exit 1
  fi
done < <(echo "${routes_json}" | jq -r '.[]')

unknown_dir="${public_dir}${pages_base}/__no-such-library-page__"
mkdir -p "${unknown_dir}"
curl -sf "http://127.0.0.1:${port}${pages_base}/__no-such-library-page__/" -o "${unknown_dir}/index.html"
if ! grep -qi 'not found' "${unknown_dir}/index.html"; then
  echo "site-prerender: not-found page missing expected copy" >&2
  exit 1
fi
cp "${unknown_dir}/index.html" "${public_dir}${pages_base}/404.html"

echo "site-prerender: wrote ${route_count} documentation pages under ${public_dir}${pages_base}"
