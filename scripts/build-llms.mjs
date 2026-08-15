import {
  readFileSync,
  writeFileSync,
  mkdirSync,
  readdirSync,
  statSync,
  existsSync,
} from "node:fs";
import { join, dirname, relative, sep, posix } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const DOCS = join(ROOT, "docs");
const WALK_ROOTS = [join(DOCS, "humans"), join(DOCS, "schema")];
const INDEX = join(DOCS, "public", "llms.txt");
const SITE = "S.E.E. Official Library";
const SITE_URL = "https://garunski.github.io/s_e_e_library";
const SUMMARY =
  "Every authoring guide for the S.E.E. Official Library, combined on one page. Learn how to create workflows, prompts, skills, commands, and bundles and publish them to the catalog.";

const checkOnly = process.argv.includes("--check");

function walk(dir) {
  const out = [];
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (statSync(full).isDirectory()) {
      out.push(...walk(full));
    } else if (name.endsWith(".md")) {
      out.push(full);
    }
  }
  return out;
}

function stripFrontmatter(text) {
  const t = text.replace(/^\uFEFF/, "");
  if (!t.startsWith("---")) {
    return { frontmatter: "", body: t };
  }
  const end = t.indexOf("\n---", 3);
  if (end < 0) {
    return { frontmatter: "", body: t };
  }
  const frontmatter = t.slice(3, end).trim();
  const body = t.slice(t.indexOf("\n", end + 1) + 1).replace(/^\s+/, "");
  return { frontmatter, body };
}

function stripMarkup(body) {
  return body
    .split("\n")
    .filter((line) => !/^import\s.+from\s.+;?\s*$/.test(line.trim()))
    .filter((line) => !/^<[A-Z][\w]*\s*\/?>$/.test(line.trim()))
    .join("\n");
}

function titleOf(frontmatter, body, fallback) {
  const fm = frontmatter.match(/^title:\s*(.+)$/m);
  if (fm?.[1]) {
    return fm[1].trim().replace(/^['"]|['"]$/g, "");
  }
  const h1 = body.match(/^#\s+(.+)$/m);
  if (h1?.[1]) {
    return h1[1].trim();
  }
  return fallback;
}

function slugFor(file) {
  return relative(DOCS, file)
    .replace(/\.md$/, "")
    .split(sep)
    .join("-")
    .replace(/[^a-z0-9-]/gi, "-")
    .toLowerCase();
}

function stripLeadingH1(body) {
  return body.replace(/^#\s+.+\n+/, "");
}

function pagePath(relPath) {
  let p = relPath.split(sep).join("/").replace(/\.md$/, "");
  if (p.endsWith("/index")) {
    p = p.slice(0, -"index".length);
  } else if (p === "index") {
    p = "";
  }
  return `/${p}`;
}

function summaryOf(body) {
  const lines = body.split("\n").map((l) => l.trim());
  const start = lines.findIndex(
    (l) => l && !l.startsWith("#") && !l.startsWith("-")
  );
  if (start < 0) {
    return "";
  }
  const paragraph = [];
  for (let i = start; i < lines.length && lines[i]; i += 1) {
    if (lines[i].startsWith("#") || lines[i].startsWith("-")) {
      break;
    }
    paragraph.push(lines[i]);
  }
  return paragraph
    .join(" ")
    .replace(/\[([^\]]+)\]\([^)]+\)/g, "$1")
    .replace(/[*`]/g, "")
    .trim();
}

function rewriteRelativeLinks(body, relPath) {
  const dir = dirname(relPath).split(sep).join("/");
  const baseDir = dir === "." ? "/" : `/${dir}`;
  return body.replace(/\]\((\.\.?\/[^)]+)\)/g, (_m, target) => {
    const [path, hash] = target.split("#");
    const resolved = posix.join(baseDir, path).replace(/\.md$/, "");
    return `](${resolved}${hash ? `#${hash}` : ""})`;
  });
}

const files = WALK_ROOTS.flatMap((root) => walk(root)).sort();
const docs = files.map((file) => {
  const raw = readFileSync(file, "utf8");
  const { frontmatter, body } = stripFrontmatter(raw);
  const relPath = relative(DOCS, file);
  const fallback = relPath.replace(/\.md$/, "");
  const cleanBody = rewriteRelativeLinks(
    stripLeadingH1(stripMarkup(body)),
    relPath
  ).trim();
  return {
    slug: slugFor(file),
    title: titleOf(frontmatter, body, fallback),
    path: relPath,
    url: `${SITE_URL}${pagePath(relPath)}`,
    summary: summaryOf(cleanBody),
    body: cleanBody,
  };
});

const rawContents = docs.map((d) => `- ${d.title}`).join("\n");
const rawSections = docs
  .map((d) => `# ${d.title}\n\nSource: docs/${d.path}\n\n${d.body}`)
  .join("\n\n---\n\n");
const rawText = `# ${SITE}\n\n> ${SUMMARY}\n\n## Contents\n\n${rawContents}\n\n---\n\n${rawSections}\n`;

if (checkOnly) {
  const current = existsSync(INDEX) ? readFileSync(INDEX, "utf8") : null;
  if (current !== rawText) {
    console.error(
      "error: LLM docs are out of date; run node scripts/build-llms.mjs or npm run docs:build"
    );
    process.exit(1);
  }
  console.log(`ok: LLM docs in sync (${docs.length} docs)`);
} else {
  mkdirSync(dirname(INDEX), { recursive: true });
  writeFileSync(INDEX, rawText);
  console.log(
    `ok: generated docs/public/llms.txt from ${docs.length} docs`
  );
}
