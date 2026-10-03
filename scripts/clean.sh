#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fmt_bytes() {
  numfmt --to=iec -- "$1"
}

dir_bytes() {
  local p="$1"
  if [ -e "$p" ] || [ -L "$p" ]; then
    du -sb -- "$p" 2>/dev/null | awk '{print $1+0}'
  else
    printf '0\n'
  fi
}

remove_tree() {
  local path="$1"
  if [ ! -e "$path" ] && [ ! -L "$path" ]; then
    return 0
  fi
  local resolved
  resolved="$(readlink -f -- "$path")"
  case "$resolved" in
    / | /home | "$HOME")
      printf 'skip: refused to remove %s\n' "$resolved" >&2
      return 1
      ;;
  esac
  printf 'removed: %s\n' "$resolved"
  rm -rf -- "$resolved"
}

target="$root/target"
before="$(dir_bytes "$target")"
printf 'library before: %s\n' "$(fmt_bytes "$before")"
printf 'processing: cargo clean --manifest-path %s\n' "$root/Cargo.toml"
cargo clean --manifest-path "$root/Cargo.toml"
remove_tree "$target"
after="$(dir_bytes "$root/target")"
cleaned=$((before - after))
if [ "$cleaned" -lt 0 ]; then
  cleaned=0
fi
printf 'library after: %s\n' "$(fmt_bytes "$after")"
printf 'library cleaned: %s\n' "$(fmt_bytes "$cleaned")"
