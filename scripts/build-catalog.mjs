import {
  readFileSync,
  writeFileSync,
  existsSync,
} from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { presentationForEntry } from "./payload-meta.mjs";
import {
  metaPath as packageMetaPath,
  normalizeDeps,
  normalizeLabels,
  slugCounts,
} from "./package-meta.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const CATALOG_PATH = join(ROOT, "catalog.json");
const SCHEMA = "see.library/v1";
const CATALOG_NAME = "S.E.E. Official Library";
const ALLOWED_TO_PREFIXES = [".s_e_e/", ".agents/", ".cursor/", "templates/"];
const CATEGORIES = new Set([
  "workflow",
  "prompt",
  "skill",
  "rule",
  "template",
  "command",
  "bundle",
]);
const checkOnly = process.argv.includes("--check");

function fail(message) {
  console.error(`error: ${message}`);
  process.exit(1);
}

function readCatalog() {
  if (!existsSync(CATALOG_PATH)) {
    fail("catalog.json not found");
  }
  let catalog;
  try {
    catalog = JSON.parse(readFileSync(CATALOG_PATH, "utf8"));
  } catch (e) {
    fail(`invalid catalog.json: ${e.message}`);
  }
  if (catalog.schema !== SCHEMA) {
    fail(`unsupported schema: expected ${SCHEMA}, got ${catalog.schema ?? "(missing)"}`);
  }
  if (!Array.isArray(catalog.packages)) {
    fail("catalog.packages must be an array");
  }
  return catalog;
}

function splitSkillFrontmatter(content) {
  const text = content.trimStart().replace(/^\uFEFF/, "");
  if (!text.startsWith("---")) {
    return null;
  }
  const afterOpen = text.slice(3).replace(/^\r?\n/, "");
  const close = afterOpen.indexOf("\n---");
  if (close < 0) {
    return null;
  }
  return afterOpen.slice(0, close).trim();
}

function parseSkillFrontmatter(yaml) {
  const name = yaml.match(/^name:\s*(.+)$/m);
  const description = yaml.match(/^description:\s*>?-?\s*([\s\S]*?)(?:\n[a-z_]+:|$)/m);
  if (!name?.[1]?.trim()) {
    return "skill frontmatter requires name";
  }
  if (!description?.[1]?.trim()) {
    return "skill frontmatter requires description";
  }
  return null;
}

function looksLikeWorkflow(value) {
  if (value?.tasks) {
    return true;
  }
  const inner = value?.content;
  if (typeof inner === "string") {
    try {
      const parsed = JSON.parse(inner);
      return Boolean(parsed?.tasks);
    } catch {
      return false;
    }
  }
  if (inner && typeof inner === "object" && inner.tasks) {
    return true;
  }
  return false;
}

function readPackageMeta(entry, label, counts) {
  const path = packageMetaPath(ROOT, entry, counts);
  if (!existsSync(path)) {
    const file = path.slice(ROOT.length + 1);
    return { error: `${label}: missing ${file}` };
  }
  let meta;
  try {
    meta = JSON.parse(readFileSync(path, "utf8"));
  } catch (e) {
    return { error: `${label}: invalid s_e_e_package.json: ${e.message}` };
  }
  if (meta.id !== entry.id) {
    return { error: `${label}: s_e_e_package.json id mismatch: expected ${entry.id}, got ${meta.id}` };
  }
  if (meta.slug !== entry.slug) {
    return { error: `${label}: s_e_e_package.json slug mismatch: expected ${entry.slug}, got ${meta.slug}` };
  }
  if (meta.category !== entry.category) {
    return { error: `${label}: s_e_e_package.json category mismatch: expected ${entry.category}, got ${meta.category}` };
  }
  if (!meta.name?.trim()) {
    return { error: `${label}: s_e_e_package.json requires a non-empty name` };
  }
  if (meta.description !== undefined && typeof meta.description !== "string") {
    return { error: `${label}: s_e_e_package.json description must be a string` };
  }
  const labels = normalizeLabels(meta.labels ?? []);
  if (labels === null) {
    return { error: `${label}: s_e_e_package.json labels must be an array of non-empty strings` };
  }
  const dependencies = normalizeDeps(meta.dependencies ?? []);
  if (dependencies === null) {
    return { error: `${label}: s_e_e_package.json dependencies must be an array of non-empty strings` };
  }
  return {
    meta: {
      name: meta.name.trim(),
      description: meta.description?.trim() ?? "",
      labels,
      dependencies,
    },
  };
}

function applyPackageMeta(entry, label, counts) {
  const result = readPackageMeta(entry, label, counts);
  if (result.error) {
    return result.error;
  }
  const { name, description, labels, dependencies } = result.meta;
  if (entry.name !== undefined && entry.name !== name) {
    return `${label}: catalog name does not match s_e_e_package.json`;
  }
  const entryDescription = entry.description?.trim() ?? "";
  if (entry.description !== undefined && entryDescription !== description) {
    return `${label}: catalog description does not match s_e_e_package.json`;
  }
  const entryLabels = normalizeLabels(entry.labels ?? []);
  if (entryLabels === null) {
    return `${label}: labels must be an array of non-empty strings`;
  }
  if (JSON.stringify(entryLabels) !== JSON.stringify(labels)) {
    return `${label}: catalog labels do not match s_e_e_package.json`;
  }
  const entryDeps = normalizeDeps(entry.dependencies ?? []);
  if (entryDeps === null) {
    return `${label}: dependencies must be an array of non-empty strings`;
  }
  if (JSON.stringify(entryDeps) !== JSON.stringify(dependencies)) {
    return `${label}: catalog dependencies do not match s_e_e_package.json`;
  }
  entry.name = name;
  entry.description = description || undefined;
  entry.labels = labels;
  entry.dependencies = dependencies;
  return null;
}

function validateDependencyIds(entry, label) {
  for (const dep of entry.dependencies) {
    if (typeof dep !== "string" || !dep.trim()) {
      return `${label}: invalid dependency id`;
    }
    if (dep === entry.id) {
      return `${label}: package cannot depend on itself`;
    }
  }
  return null;
}

function validatePayload(category, toPath, content) {
  switch (category) {
    case "workflow": {
      let value;
      try {
        value = JSON.parse(content);
      } catch (e) {
        return `invalid workflow JSON: ${e.message}`;
      }
      if (!looksLikeWorkflow(value)) {
        return "payload does not contain workflow tasks";
      }
      return null;
    }
    case "prompt": {
      let value;
      try {
        value = JSON.parse(content);
      } catch (e) {
        return `invalid prompt JSON: ${e.message}`;
      }
      if (looksLikeWorkflow(value)) {
        return "payload looks like a workflow, not a prompt";
      }
      if (!value?.id?.trim()) {
        return "prompt requires a non-empty id";
      }
      if (!value?.name?.trim()) {
        return "prompt requires a non-empty name";
      }
      return null;
    }
    case "command": {
      if (!toPath.startsWith(".s_e_e/commands/")) {
        return "command install path must be under .s_e_e/commands/";
      }
      let value;
      try {
        value = JSON.parse(content);
      } catch (e) {
        return `invalid command JSON: ${e.message}`;
      }
      if (looksLikeWorkflow(value)) {
        return "payload looks like a workflow, not a command";
      }
      if (value?.content && !value?.binary) {
        return "payload looks like a prompt, not a command";
      }
      if (!value?.id?.trim()) {
        return "command requires a non-empty id";
      }
      if (!value?.binary?.trim()) {
        return "command requires a non-empty binary";
      }
      const expected = `.s_e_e/commands/${value.id}.json`;
      if (toPath !== expected) {
        return `command install path must be ${expected}`;
      }
      return null;
    }
    case "skill": {
      if (!toPath.endsWith("SKILL.md")) {
        return "skill payload must be a SKILL.md file";
      }
      const yaml = splitSkillFrontmatter(content);
      if (!yaml) {
        return "skill must start with YAML frontmatter";
      }
      return parseSkillFrontmatter(yaml);
    }
    case "rule": {
      if (!toPath.endsWith(".mdc")) {
        return "rule payload must be a .mdc file";
      }
      if (!content.trim()) {
        return "rule file cannot be empty";
      }
      return null;
    }
    case "template": {
      if (!toPath.startsWith("templates/")) {
        return "template install path must be under templates/";
      }
      if (!content.trim()) {
        return "template file cannot be empty";
      }
      return null;
    }
    default:
      return `unknown category: ${category}`;
  }
}

function inferKindFromTo(toPath) {
  if (toPath.startsWith(".s_e_e/workflows/definitions/")) {
    return "workflow";
  }
  if (toPath.startsWith(".s_e_e/prompts/")) {
    return "prompt";
  }
  if (toPath.startsWith(".s_e_e/commands/")) {
    return "command";
  }
  if (toPath.startsWith(".agents/skills/") && toPath.endsWith("SKILL.md")) {
    return "skill";
  }
  if (toPath.endsWith(".mdc")) {
    return "rule";
  }
  if (toPath.startsWith("templates/")) {
    return "template";
  }
  return null;
}

function validatePackageFiles(entry, label) {
  if (entry.files.length === 0) {
    return `${label}: files must be a non-empty array`;
  }
  const seenTo = new Set();
  for (const file of entry.files) {
    if (!file?.to || !file?.from) {
      return `${label}: each files[] entry needs to and from`;
    }
    if (file.to.includes("\\")) {
      return `${label}: install path must use forward slashes: ${file.to}`;
    }
    if (!ALLOWED_TO_PREFIXES.some((prefix) => file.to.startsWith(prefix))) {
      return `${label}: install path must start with one of ${ALLOWED_TO_PREFIXES.join(", ")}: ${file.to}`;
    }
    if (seenTo.has(file.to)) {
      return `${label}: duplicate install path: ${file.to}`;
    }
    seenTo.add(file.to);
    const fromPath = join(ROOT, file.from);
    if (!existsSync(fromPath)) {
      return `${label}: missing payload file ${file.from}`;
    }
    const content = readFileSync(fromPath, "utf8");
    const kind =
      entry.category === "bundle"
        ? inferKindFromTo(file.to)
        : entry.category;
    if (entry.category === "bundle" && !kind) {
      return `${label}: cannot infer payload kind for install path: ${file.to}`;
    }
    const payloadError = validatePayload(kind, file.to, content);
    if (payloadError) {
      return `${label}: ${file.from}: ${payloadError}`;
    }
  }
  return null;
}

function validateEntry(entry, ids, counts) {
  const label = entry.id ?? "(missing id)";
  const required = [
    "id",
    "slug",
    "category",
    "version",
    "author",
    "verified",
    "license",
    "requires",
    "dependencies",
    "files",
  ];
  for (const key of required) {
    if (entry[key] === undefined) {
      return `${label}: missing field ${key}`;
    }
  }
  if (!CATEGORIES.has(entry.category)) {
    return `${label}: invalid category ${entry.category}`;
  }
  if (!Array.isArray(entry.requires)) {
    return `${label}: requires must be an array`;
  }
  if (!Array.isArray(entry.dependencies)) {
    return `${label}: dependencies must be an array`;
  }
  if (!Array.isArray(entry.files)) {
    return `${label}: files must be an array`;
  }
  const depError = validateDependencyIds(entry, label);
  if (depError) {
    return depError;
  }
  if (entry.category === "bundle") {
    const fileError = validatePackageFiles(entry, label);
    if (fileError) {
      return fileError;
    }
    return applyPackageMeta(entry, label, counts);
  }
  if (entry.files.length === 0) {
    return `${label}: files must be a non-empty array`;
  }
  const fileError = validatePackageFiles(entry, label);
  if (fileError) {
    return fileError;
  }
  const { name, description } = presentationForEntry(entry, ROOT);
  if (!name.trim()) {
    return `${label}: name is required`;
  }
  if (entry.name !== undefined && typeof entry.name !== "string") {
    return `${label}: name must be a string`;
  }
  if (entry.description !== undefined && typeof entry.description !== "string") {
    return `${label}: description must be a string`;
  }
  entry.name = name;
  entry.description = description || undefined;
  return applyPackageMeta(entry, label, counts);
}

function validateCatalog(catalog) {
  const ids = new Set();
  const counts = slugCounts(catalog.packages);
  for (const entry of catalog.packages) {
    if (ids.has(entry.id)) {
      fail(`duplicate package id: ${entry.id}`);
    }
    ids.add(entry.id);
    const error = validateEntry(entry, ids, counts);
    if (error) {
      fail(error);
    }
  }
  for (const entry of catalog.packages) {
    for (const dep of entry.dependencies) {
      if (!ids.has(dep)) {
        fail(`${entry.id}: dependency not found in catalog: ${dep}`);
      }
    }
  }
}

function stableStringify(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

function catalogWithoutUpdated(catalog) {
  const { updated: _updated, ...rest } = catalog;
  return rest;
}

const catalog = readCatalog();
validateCatalog(catalog);

const next = {
  ...catalog,
  schema: SCHEMA,
  name: catalog.name || CATALOG_NAME,
  updated: new Date().toISOString(),
};

if (checkOnly) {
  const currentRaw = readFileSync(CATALOG_PATH, "utf8");
  const current = JSON.parse(currentRaw);
  validateCatalog(current);
  const currentBody = stableStringify(catalogWithoutUpdated(current));
  const nextBody = stableStringify(catalogWithoutUpdated(next));
  if (currentBody !== nextBody) {
    fail("catalog.json is out of date; run npm run build");
  }
  console.log(
    `ok: ${current.packages.length} packages validated (${current.updated})`
  );
} else {
  writeFileSync(CATALOG_PATH, stableStringify(next));
  console.log(`wrote catalog.json (${next.packages.length} packages)`);
}
