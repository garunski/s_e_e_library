import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

export function normalizeLabels(labels) {
  if (!Array.isArray(labels)) {
    return null;
  }
  const out = labels.map((l) => (typeof l === "string" ? l.trim() : "")).filter(Boolean);
  if (out.length !== labels.length) {
    return null;
  }
  return [...new Set(out)].sort();
}

export function normalizeDeps(deps) {
  if (!Array.isArray(deps)) {
    return null;
  }
  const out = deps.map((d) => (typeof d === "string" ? d.trim() : "")).filter(Boolean);
  if (out.length !== deps.length) {
    return null;
  }
  return [...new Set(out)].sort();
}

export function slugCounts(entries) {
  const counts = new Map();
  for (const entry of entries) {
    counts.set(entry.slug, (counts.get(entry.slug) ?? 0) + 1);
  }
  return counts;
}

export function metaFileName(entry, counts) {
  if ((counts.get(entry.slug) ?? 0) <= 1) {
    return "s_e_e_package.json";
  }
  return `s_e_e_package.${entry.id}.json`;
}

export function metaPath(root, entry, counts) {
  return join(root, "packages", entry.slug, metaFileName(entry, counts));
}

export function readExistingMeta(root, entry, counts) {
  const path = metaPath(root, entry, counts);
  if (!existsSync(path)) {
    return null;
  }
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch {
    return null;
  }
}
