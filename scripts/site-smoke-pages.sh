#!/usr/bin/env bash
set -euo pipefail
umask 077

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
staging="${SEE_LIBRARY_PAGES_STAGING:-${root}/target/pages-artifact}"
port="${SITE_SMOKE_PORT:-19928}"

if [[ ! -d "${staging}" ]]; then
  echo "site-smoke-pages: missing staging directory ${staging}" >&2
  echo "run scripts/site-package-pages.sh first" >&2
  exit 1
fi

staging="$(cd "${staging}" && pwd)"
if [[ ! -f "${staging}/catalog.json" || ! -d "${staging}/packages" || ! -d "${staging}/schema" || -e "${staging}/library" ]]; then
  echo 'site-smoke-pages: expected a flat catalog/packages/schema artifact' >&2
  exit 1
fi
mount=$(mktemp -d "${TMPDIR:-/tmp}/see-library-prefix.XXXXXX")
httpd_pid=
cleanup() {
  if [[ -n "$httpd_pid" ]]; then
    kill "$httpd_pid" 2>/dev/null || true
    wait "$httpd_pid" 2>/dev/null || true
  fi
  rm -rf "$mount"
}
trap cleanup EXIT
ln -s "$staging" "$mount/library"
busybox httpd -f -p "127.0.0.1:${port}" -h "$mount" &
httpd_pid=$!
base="http://127.0.0.1:${port}/library"

ready=0
for _ in $(seq 1 30); do
  kill -0 "$httpd_pid" 2>/dev/null || { echo 'site-smoke-pages: owned static server stopped' >&2; exit 1; }
  if curl --connect-timeout 1 --max-time 2 -sf -o /dev/null "${base}/catalog.json"; then
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
  --root "${root}" smoke-pages --staging "${staging}" --base-url "$base"

status=$(curl --connect-timeout 1 --max-time 2 -s -o /dev/null -w '%{http_code}' "$base/library/catalog.json")
[[ "$status" == 404 ]] || { echo 'site-smoke-pages: doubled Library prefix must be absent' >&2; exit 1; }

echo "site-smoke-pages: /library/ prefix and exact bytes verified; doubled prefix absent"
