import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const PACKAGES = join(ROOT, "packages");
const VERSION = "1.0.0";

const PROMPT_REF = /\{\{prompt\.([a-z0-9-]+)\}\}/g;
const COMMAND_REF = /command_id\\?"\s*:\s*\\?"([a-z0-9-]+)/g;
const SKILL_REF = /\.agents\/skills\/([a-z0-9-]+)\/SKILL\.md/g;

function titleCaseSlug(slug) {
  return slug
    .split("-")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

function inferMetaFromPayload(slug) {
  const versionDir = join(PACKAGES, slug, VERSION);
  if (!existsSync(versionDir)) {
    return null;
  }
  const def = join(versionDir, "definition.json");
  if (existsSync(def)) {
    const parsed = JSON.parse(readFileSync(def, "utf8"));
    return {
      id: `wf-${slug}`,
      slug,
      category: "workflow",
      name: parsed.name ?? slug,
      labels: parsed.labels ?? [],
    };
  }
  const prompt = join(versionDir, "prompt.json");
  if (existsSync(prompt)) {
    const parsed = JSON.parse(readFileSync(prompt, "utf8"));
    return {
      id: `prompt-${parsed.id ?? slug}`,
      slug,
      category: "prompt",
      name: parsed.name ?? slug,
      labels: [],
    };
  }
  const command = join(versionDir, "command.json");
  if (existsSync(command)) {
    const parsed = JSON.parse(readFileSync(command, "utf8"));
    return {
      id: `cmd-${parsed.id ?? slug}`,
      slug,
      category: "command",
      name: parsed.name ?? slug,
      labels: [],
    };
  }
  const skill = join(versionDir, "SKILL.md");
  if (existsSync(skill)) {
    return {
      id: `skill-${slug}`,
      slug,
      category: "skill",
      name: slug,
      labels: [],
    };
  }
  return null;
}

function readPackageMetas() {
  const byId = new Map();
  for (const slug of readdirSync(PACKAGES)) {
    const dir = join(PACKAGES, slug);
    let foundMeta = false;
    for (const name of readdirSync(dir)) {
      if (!name.startsWith("s_e_e_package") || !name.endsWith(".json")) {
        continue;
      }
      const meta = JSON.parse(readFileSync(join(dir, name), "utf8"));
      if (meta.category === "bundle") {
        continue;
      }
      byId.set(meta.id, { ...meta, slug });
      foundMeta = true;
    }
    if (!foundMeta) {
      const inferred = inferMetaFromPayload(slug);
      if (inferred) {
        byId.set(inferred.id, inferred);
      }
    }
  }
  return byId;
}

function payloadPaths(slug) {
  const versionDir = join(PACKAGES, slug, VERSION);
  if (!existsSync(versionDir)) {
    return [];
  }
  return readdirSync(versionDir).map((name) => join(versionDir, name));
}

function strongRefsFromText(text) {
  const refs = new Set();
  for (const match of text.matchAll(PROMPT_REF)) {
    refs.add(`prompt-${match[1]}`);
  }
  return refs;
}

function weakRefsFromText(text) {
  const refs = new Set();
  for (const match of text.matchAll(COMMAND_REF)) {
    refs.add(`cmd-${match[1]}`);
  }
  for (const match of text.matchAll(SKILL_REF)) {
    refs.add(`skill-${match[1]}`);
  }
  return refs;
}

function refsForPackage(meta, byId) {
  const paths = payloadPaths(meta.slug);
  const strong = new Set();
  const weak = new Set();
  for (const path of paths) {
    const text = readFileSync(path, "utf8");
    const isWorkflowDef = path.endsWith("definition.json");
    if (isWorkflowDef) {
      for (const id of strongRefsFromText(text)) {
        if (id !== meta.id && byId.has(id)) {
          strong.add(id);
        }
      }
    }
    for (const id of weakRefsFromText(text)) {
      if (id !== meta.id && byId.has(id)) {
        weak.add(id);
      }
    }
  }
  return { strong, weak };
}

function buildGraph(byId) {
  const strongEdges = new Map();
  const weakEdges = new Map();
  for (const [id, meta] of byId) {
    const { strong, weak } = refsForPackage(meta, byId);
    strongEdges.set(id, strong);
    weakEdges.set(id, weak);
  }
  return { strongEdges, weakEdges };
}

function connectedComponents(byId, strongEdges) {
  const parent = new Map([...byId.keys()].map((id) => [id, id]));

  function find(id) {
    let root = id;
    while (parent.get(root) !== root) {
      root = parent.get(root);
    }
    let cur = id;
    while (parent.get(cur) !== root) {
      const next = parent.get(cur);
      parent.set(cur, root);
      cur = next;
    }
    return root;
  }

  function unite(a, b) {
    const ra = find(a);
    const rb = find(b);
    if (ra !== rb) {
      parent.set(rb, ra);
    }
  }

  for (const [id, refs] of strongEdges) {
    for (const ref of refs) {
      unite(id, ref);
    }
  }

  const groups = new Map();
  for (const id of byId.keys()) {
    const root = find(id);
    if (!groups.has(root)) {
      groups.set(root, []);
    }
    groups.get(root).push(id);
  }
  return [...groups.values()].filter((g) => g.length > 1);
}

function bundleSlugForGroup(ids, byId) {
  const workflows = ids
    .map((id) => byId.get(id))
    .filter((m) => m?.category === "workflow")
    .map((m) => m.slug)
    .sort();
  if (workflows.length === 1) {
    return workflows[0].replace(/^system-/, "");
  }
  if (workflows.length > 1) {
    const primary = [...workflows].sort((a, b) => a.length - b.length)[0];
    return primary.replace(/^system-/, "");
  }
  const first = byId.get(ids.sort()[0]);
  return first.slug.replace(/^system-/, "");
}

function bundleNameForGroup(slug, ids, byId) {
  const workflows = ids
    .map((id) => byId.get(id))
    .filter((m) => m?.category === "workflow");
  if (workflows.length === 1) {
    return workflows[0].name || titleCaseSlug(slug);
  }
  if (workflows.length > 1) {
    return `${titleCaseSlug(slug)} workflows`;
  }
  return titleCaseSlug(slug);
}

function bundleDescriptionForGroup(ids, byId) {
  const kinds = new Set(ids.map((id) => byId.get(id)?.category).filter(Boolean));
  const parts = [];
  if (kinds.has("workflow")) {
    parts.push("workflows");
  }
  if (kinds.has("prompt")) {
    parts.push("prompts");
  }
  if (kinds.has("skill")) {
    parts.push("skills");
  }
  if (kinds.has("command")) {
    parts.push("commands");
  }
  return `Packages linked by template references: ${parts.join(", ")}.`;
}

function labelsForGroup(ids, byId) {
  const labels = new Set();
  for (const id of ids) {
    const meta = byId.get(id);
    for (const label of meta?.labels ?? []) {
      labels.add(label);
    }
  }
  return [...labels].sort();
}

export function discoverBundles() {
  const byId = readPackageMetas();
  const { strongEdges, weakEdges } = buildGraph(byId);
  const groups = connectedComponents(byId, strongEdges);
  const usedSlugs = new Set();

  return groups
    .map((coreIds) => {
      const ids = new Set(coreIds);
      for (const id of coreIds) {
        for (const ref of weakEdges.get(id) ?? []) {
          ids.add(ref);
        }
        for (const ref of strongEdges.get(id) ?? []) {
          ids.add(ref);
          for (const nested of weakEdges.get(ref) ?? []) {
            ids.add(nested);
          }
        }
      }
      const allIds = [...ids].sort();
      let slug = bundleSlugForGroup(allIds, byId);
      if (usedSlugs.has(slug)) {
        slug = `${slug}-${ids.length}`;
      }
      usedSlugs.add(slug);
      return {
        slug,
        name: bundleNameForGroup(slug, allIds, byId),
        description: bundleDescriptionForGroup(allIds, byId),
        members: allIds,
        labels: labelsForGroup(allIds, byId),
      };
    })
    .sort((a, b) => a.slug.localeCompare(b.slug));
}

const isMain =
  process.argv[1] &&
  fileURLToPath(import.meta.url) === process.argv[1];

if (isMain) {
  const write = process.argv.includes("--write");
  const bundles = discoverBundles();
  if (write) {
    const out = join(dirname(fileURLToPath(import.meta.url)), "bundles.json");
    writeFileSync(out, `${JSON.stringify(bundles, null, 2)}\n`);
    console.log(`wrote ${bundles.length} bundles to ${out}`);
  } else {
    console.log(JSON.stringify(bundles, null, 2));
  }
}
