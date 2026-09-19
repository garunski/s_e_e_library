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

export function normalizeToolIds(toolIds) {
  if (!Array.isArray(toolIds)) {
    return null;
  }
  const out = toolIds
    .map((id) => (typeof id === "string" ? id.trim() : ""))
    .filter(Boolean);
  if (out.length !== toolIds.length) {
    return null;
  }
  return [...new Set(out)].sort();
}

export function normalizeReleases(releases) {
  if (!Array.isArray(releases)) {
    return null;
  }
  const out = [];
  for (const release of releases) {
    if (!release || typeof release !== "object") {
      return null;
    }
    const version =
      typeof release.version === "string" ? release.version.trim() : "";
    const date = typeof release.date === "string" ? release.date.trim() : "";
    const note = typeof release.note === "string" ? release.note.trim() : "";
    if (!version || !date || !note) {
      return null;
    }
    out.push({ version, date, note });
  }
  return out;
}

export function catalogWithoutUpdated(catalog) {
  const { updated: _updated, ...rest } = catalog;
  return rest;
}

export function stableStringify(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

export function marketplaceRefErrors({
  tools = [],
  stacks = [],
  featuredStackId,
  packageIds = [],
}) {
  const errors = [];
  const toolIds = new Set();
  for (const tool of tools) {
    const id = typeof tool?.id === "string" ? tool.id.trim() : "";
    const name = typeof tool?.name === "string" ? tool.name.trim() : "";
    if (!id || !name) {
      errors.push("tool requires id and name");
      continue;
    }
    if (toolIds.has(id)) {
      errors.push(`duplicate tool id: ${id}`);
    }
    toolIds.add(id);
  }
  const packages = new Set(packageIds);
  const stackIds = new Set();
  for (const stack of stacks) {
    const id = typeof stack?.id === "string" ? stack.id.trim() : "";
    const name = typeof stack?.name === "string" ? stack.name.trim() : "";
    if (!id || !name) {
      errors.push("stack requires id and name");
      continue;
    }
    if (stackIds.has(id)) {
      errors.push(`duplicate stack id: ${id}`);
    }
    stackIds.add(id);
    const shared = Array.isArray(stack.sharedPackageIds)
      ? stack.sharedPackageIds
      : null;
    if (!shared) {
      errors.push(`${id}: sharedPackageIds must be an array`);
      continue;
    }
    for (const packageId of shared) {
      if (typeof packageId !== "string" || !packageId.trim()) {
        errors.push(`${id}: invalid shared package id`);
        continue;
      }
      if (!packages.has(packageId)) {
        errors.push(`${id}: unknown stack package id: ${packageId}`);
      }
    }
    const variants = stack.variants ?? [];
    if (!Array.isArray(variants)) {
      errors.push(`${id}: variants must be an array`);
      continue;
    }
    for (const variant of variants) {
      const toolId =
        typeof variant?.toolId === "string" ? variant.toolId.trim() : "";
      if (!toolId) {
        errors.push(`${id}: variant requires toolId`);
        continue;
      }
      if (!toolIds.has(toolId)) {
        errors.push(`${id}: undeclared tool in stack variant: ${toolId}`);
      }
      const variantPackages = Array.isArray(variant.packageIds)
        ? variant.packageIds
        : null;
      if (!variantPackages) {
        errors.push(`${id}: variant packageIds must be an array`);
        continue;
      }
      for (const packageId of variantPackages) {
        if (typeof packageId !== "string" || !packageId.trim()) {
          errors.push(`${id}: invalid variant package id`);
          continue;
        }
        if (!packages.has(packageId)) {
          errors.push(`${id}: unknown stack package id: ${packageId}`);
        }
      }
    }
  }
  if (featuredStackId !== undefined && featuredStackId !== null) {
    if (typeof featuredStackId !== "string" || !featuredStackId.trim()) {
      errors.push("featuredStackId must be a non-empty string");
    } else if (!stackIds.has(featuredStackId)) {
      errors.push(`unknown featured stack id: ${featuredStackId}`);
    }
  }
  return errors;
}

export function loadMarketplaceSource(root) {
  const path = join(root, "scripts", "stacks.json");
  if (!existsSync(path)) {
    return { error: "missing scripts/stacks.json" };
  }
  let source;
  try {
    source = JSON.parse(readFileSync(path, "utf8"));
  } catch (e) {
    return { error: `invalid scripts/stacks.json: ${e.message}` };
  }
  if (!Array.isArray(source.tools)) {
    return { error: "scripts/stacks.json tools must be an array" };
  }
  if (!Array.isArray(source.stacks)) {
    return { error: "scripts/stacks.json stacks must be an array" };
  }
  return {
    source: {
      tools: source.tools,
      stacks: source.stacks,
      featuredStackId: source.featuredStackId,
    },
  };
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
