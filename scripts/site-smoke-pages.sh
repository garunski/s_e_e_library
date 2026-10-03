#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
staging="${SEE_LIBRARY_PAGES_STAGING:-${root}/target/pages-artifact}"
port="${SITE_SMOKE_PORT:-19928}"

if [[ ! -d "${staging}" ]]; then
  echo "site-smoke-pages: missing staging directory ${staging}" >&2
  echo "run scripts/site-package-pages.sh first" >&2
  exit 1
fi

busybox httpd -f -p "${port}" -h "${staging}" &
httpd_pid=$!
trap 'kill "${httpd_pid}" 2>/dev/null || true' EXIT

ready=0
for _ in $(seq 1 30); do
  if curl -sf -o /dev/null "http://127.0.0.1:${port}/"; then
    ready=1
    break
  fi
  sleep 0.1
done
if [[ "${ready}" != "1" ]]; then
  echo "site-smoke-pages: static server did not become ready on port ${port}" >&2
  exit 1
fi

cargo run -q --manifest-path "${root}/Cargo.toml" -p see_library_catalog_tool -- \
  --root "${root}" smoke-pages --staging "${staging}" --base-url "http://127.0.0.1:${port}"

echo "site-smoke-pages: ok"
