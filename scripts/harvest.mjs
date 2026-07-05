import { cpSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

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

function promptEntry(stem) {
  const slug = stem;
  const from = `packages/${slug}/${VERSION}/prompt.json`;
  const src = join(HUB, ".s_e_e", "prompts", `${stem}.json`);
  const dest = join(ROOT, from);
  copyFile(src, dest);
  return {
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
  };
}

function workflowEntry(stem) {
  const slug = stem;
  const from = `packages/${slug}/${VERSION}/definition.json`;
  const src = join(HUB, ".s_e_e", "workflows", "definitions", `${stem}.json`);
  const dest = join(ROOT, from);
  copyFile(src, dest);
  const content = readFileSync(src, "utf8");
  return {
    id: `wf-${stem}`,
    slug,
    category: "workflow",
    version: VERSION,
    author: AUTHOR,
    verified: true,
    license: LICENSE,
    requires: WORKFLOW_REQUIRES,
    dependencies: promptDepsFromWorkflow(content),
    files: [
      { to: `.s_e_e/workflows/definitions/${stem}.json`, from },
    ],
  };
}

function commandEntry(id) {
  const slug = id;
  const from = `packages/${slug}/${VERSION}/command.json`;
  const src = join(HUB, ".s_e_e", "commands", `${id}.json`);
  const dest = join(ROOT, from);
  copyFile(src, dest);
  return {
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
  };
}

function skillEntry(slug) {
  const from = `packages/${slug}/${VERSION}/SKILL.md`;
  const src = join(HUB, ".agents", "skills", slug, "SKILL.md");
  const dest = join(ROOT, from);
  copyFile(src, dest);
  return {
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
  };
}

const packages = [
  ...PROMPTS.map(promptEntry),
  ...WORKFLOWS.map(workflowEntry),
  ...COMMANDS.map(commandEntry),
  ...SKILLS.map(skillEntry),
];

packages.sort((a, b) => a.id.localeCompare(b.id));

const catalog = {
  schema: "see.library/v1",
  name: "S.E.E. Official Library",
  updated: new Date().toISOString(),
  packages,
};

writeFileSync(join(ROOT, "catalog.json"), `${JSON.stringify(catalog, null, 2)}\n`);
console.log(`harvested ${packages.length} packages into ${ROOT}`);
