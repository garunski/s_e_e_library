import { cpSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { discoverBundles } from "./discover-bundles.mjs";
import { extractPayloadMeta, presentationForEntry } from "./payload-meta.mjs";
import {
  metaPath as packageMetaPath,
  normalizeDeps,
  normalizeLabels,
  readExistingMeta,
  slugCounts,
} from "./package-meta.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const HUB = join(ROOT, "..", "s_e_e_project");
const VERSION = "1.0.0";
const AUTHOR = "garunski";
const LICENSE = "AGPL-3.0-only";

const PROMPTS = [
  "system-implement-story",
  "system-quality-full-fix",
  "system-stories-from-doc-authoring",
  "system-story-authoring",
  "system-work-story",
  "system-workflow-authoring",
];

const WORKFLOWS = [
  "system-author-stories-from-doc",
  "system-author-story",
  "system-author-workflow",
  "system-implement-stories-bulk",
  "system-implement-story",
];

const LIBRARY_ONLY_WORKFLOWS = ["system-work-swimlane-stories"];

const COMMANDS = ["claude-code", "cursor-agent"];

const SKILLS = ["stories", "work", "doc", "workflow"];

const WORKFLOW_REQUIRES = ["mise", "git"];

function promptDepsFromWorkflow(content) {
  const matches = content.matchAll(/prompt\.([a-z0-9-]+)/g);
  const ids = [...new Set([...matches].map((m) => `prompt-${m[1]}`))];
  return ids.sort();
}

function copyFile(src, dest) {
  mkdirSync(dirname(dest), { recursive: true });
  cpSync(src, dest);
}

function writeMeta(entry, seedLabels, counts) {
  const existing = readExistingMeta(ROOT, entry, counts);
  const name =
    existing?.name?.trim() || entry.name?.trim() || "";
  const description =
    existing?.description?.trim() || entry.description?.trim() || "";
  const labels =
    existing?.labels !== undefined
      ? normalizeLabels(existing.labels) ?? []
      : normalizeLabels(seedLabels) ?? [];
  const authored =
    existing?.dependencies !== undefined
      ? normalizeDeps(existing.dependencies) ?? []
      : [];
  const dependencies =
    normalizeDeps([...(entry.dependencies ?? []), ...authored]) ?? [];
  const meta = {
    id: entry.id,
    slug: entry.slug,
    category: entry.category,
    name,
    description,
    labels,
    dependencies,
  };
  const path = packageMetaPath(ROOT, entry, counts);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(meta, null, 2)}\n`);
  return meta;
}

function finalizeEntries(built) {
  const counts = slugCounts(built.map((item) => item.entry));
  return built.map((item) => {
    const { entry, seedLabels } = item;
    const meta = writeMeta(entry, seedLabels, counts);
    return {
      ...entry,
      name: meta.name,
      description: meta.description || undefined,
      labels: meta.labels,
      dependencies: meta.dependencies,
    };
  });
}

function withPresentation(entry, payloadContent) {
  const meta = extractPayloadMeta(entry.category, payloadContent);
  const { name, description } = presentationForEntry(
    {
      ...entry,
      name: meta.name || undefined,
      description: meta.description || undefined,
    },
    ROOT,
    () => payloadContent
  );
  return { ...entry, name, description: description || undefined };
}

function buildEntry(entry, seedLabels) {
  return { entry, seedLabels };
}

function promptEntry(stem) {
  const slug = stem;
  const from = `packages/${slug}/${VERSION}/prompt.json`;
  const src = join(HUB, ".s_e_e", "prompts", `${stem}.json`);
  const dest = join(ROOT, from);
  copyFile(src, dest);
  const content = readFileSync(src, "utf8");
  return buildEntry(
    withPresentation(
      {
        id: `prompt-${stem}`,
        slug,
        category: "prompt",
        version: VERSION,
        author: AUTHOR,
        verified: true,
        license: LICENSE,
        requires: [],
        dependencies: [],
        files: [{ to: `.s_e_e/prompts/${stem}.json`, from }],
      },
      content
    ),
    []
  );
}

function workflowEntryFromContent(stem, content) {
  const slug = stem;
  const from = `packages/${slug}/${VERSION}/definition.json`;
  let seedLabels = [];
  try {
    const parsed = JSON.parse(content);
    seedLabels = normalizeLabels(parsed?.labels) ?? [];
  } catch {
  }
  return buildEntry(
    withPresentation(
      {
        id: `wf-${stem}`,
        slug,
        category: "workflow",
        version: VERSION,
        author: AUTHOR,
        verified: true,
        license: LICENSE,
        requires: WORKFLOW_REQUIRES,
        dependencies: promptDepsFromWorkflow(content),
        files: [{ to: `.s_e_e/workflows/definitions/${stem}.json`, from }],
      },
      content
    ),
    seedLabels
  );
}

function workflowEntry(stem) {
  const slug = stem;
  const from = `packages/${slug}/${VERSION}/definition.json`;
  const src = join(HUB, ".s_e_e", "workflows", "definitions", `${stem}.json`);
  const dest = join(ROOT, from);
  copyFile(src, dest);
  const content = readFileSync(src, "utf8");
  return workflowEntryFromContent(stem, content);
}

function libraryOnlyWorkflowEntry(stem) {
  const from = `packages/${stem}/${VERSION}/definition.json`;
  const path = join(ROOT, from);
  if (!existsSync(path)) {
    throw new Error(`missing library-only workflow payload: ${path}`);
  }
  const content = readFileSync(path, "utf8");
  return workflowEntryFromContent(stem, content);
}

function commandEntry(id) {
  const slug = id;
  const from = `packages/${slug}/${VERSION}/command.json`;
  const src = join(HUB, ".s_e_e", "commands", `${id}.json`);
  const dest = join(ROOT, from);
  copyFile(src, dest);
  const content = readFileSync(src, "utf8");
  return buildEntry(
    withPresentation(
      {
        id: `cmd-${id}`,
        slug,
        category: "command",
        version: VERSION,
        author: AUTHOR,
        verified: true,
        license: LICENSE,
        requires: [],
        dependencies: [],
        files: [{ to: `.s_e_e/commands/${id}.json`, from }],
      },
      content
    ),
    []
  );
}

function skillEntry(slug) {
  const from = `packages/${slug}/${VERSION}/SKILL.md`;
  const src = join(HUB, ".agents", "skills", slug, "SKILL.md");
  const dest = join(ROOT, from);
  copyFile(src, dest);
  const content = readFileSync(src, "utf8");
  return buildEntry(
    withPresentation(
      {
        id: `skill-${slug}`,
        slug,
        category: "skill",
        version: VERSION,
        author: AUTHOR,
        verified: true,
        license: LICENSE,
        requires: [],
        dependencies: [],
        files: [{ to: `.agents/skills/${slug}/SKILL.md`, from }],
      },
      content
    ),
    []
  );
}

function bundleEntry(def, byId) {
  const bundleSlug = def.slug;
  const files = [];
  for (const memberId of def.members) {
    const member = byId.get(memberId);
    if (!member) {
      throw new Error(`bundle ${def.slug}: unknown member ${memberId}`);
    }
    for (const file of member.files) {
      const src = join(ROOT, file.from);
      const localFrom = `packages/${bundleSlug}/${VERSION}/${member.slug}/${basename(file.from)}`;
      const dest = join(ROOT, localFrom);
      copyFile(src, dest);
      files.push({ to: file.to, from: localFrom });
    }
  }
  files.sort((a, b) => a.to.localeCompare(b.to));
  return buildEntry(
    {
      id: `bundle-${def.slug}`,
      slug: def.slug,
      category: "bundle",
      version: VERSION,
      author: AUTHOR,
      verified: true,
      license: LICENSE,
      requires: [],
      dependencies: [],
      files,
      name: def.name,
      description: def.description || undefined,
    },
    def.labels ?? []
  );
}

const bundleDefs = discoverBundles();

const memberBuilt = [
  ...PROMPTS.map(promptEntry),
  ...WORKFLOWS.map(workflowEntry),
  ...LIBRARY_ONLY_WORKFLOWS.map(libraryOnlyWorkflowEntry),
  ...COMMANDS.map(commandEntry),
  ...SKILLS.map(skillEntry),
];

const byId = new Map(memberBuilt.map((item) => [item.entry.id, item.entry]));

const built = [
  ...memberBuilt,
  ...bundleDefs.map((def) => bundleEntry(def, byId)),
];

const packages = finalizeEntries(built);

packages.sort((a, b) => a.id.localeCompare(b.id));

const catalog = {
  schema: "see.library/v1",
  name: "S.E.E. Official Library",
  updated: new Date().toISOString(),
  packages,
};

writeFileSync(join(ROOT, "catalog.json"), `${JSON.stringify(catalog, null, 2)}\n`);
console.log(`harvested ${packages.length} packages into ${ROOT}`);
