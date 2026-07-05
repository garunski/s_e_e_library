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
const CONTENT = join(ROOT, "content");
const PUBLIC = join(ROOT, "public");
const PAGE = join(CONTENT, "for-llms.mdx");
const INDEX = join(PUBLIC, "llms.txt");
const SITE = "S.E.E. Official Library";
const SITE_URL = "https://garunski.github.io/s_e_e_library";
const SUMMARY =
  "Every authoring guide for the S.E.E. Official Library, combined on one page. Learn how to create workflows, prompts, skills, commands, and bundles and publish them to the catalog.";

const checkOnly = process.argv.includes("--check");

const SKIP = new Set([join(CONTENT, "index.mdx"), PAGE]);

function walk(dir) {
  const out = [];
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (statSync(full).isDirectory()) {
      out.push(...walk(full));
    } else if (name.endsWith(".mdx") && !SKIP.has(full)) {
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

function stripMdx(body) {
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
  return relative(CONTENT, file)
    .replace(/\.mdx$/, "")
    .split(sep)
    .join("-")
    .replace(/[^a-z0-9-]/gi, "-")
    .toLowerCase();
}

function stripLeadingH1(body) {
  return body.replace(/^#\s+.+\n+/, "");
}

function pagePath(relPath) {
  let p = relPath.split(sep).join("/").replace(/\.mdx$/, "");
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
    const resolved = posix.join(baseDir, path).replace(/\.mdx$/, "");
    return `](${resolved}${hash ? `#${hash}` : ""})`;
  });
}

const files = walk(CONTENT).sort();
const docs = files.map((file) => {
  const raw = readFileSync(file, "utf8");
  const { frontmatter, body } = stripFrontmatter(raw);
  const relPath = relative(CONTENT, file);
  const fallback = relPath.replace(/\.mdx$/, "");
  const cleanBody = rewriteRelativeLinks(
    stripLeadingH1(stripMdx(body)),
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

const page = `---
title: For LLMs
---

# For LLMs

Every doc in one file. Copy this URL and fetch it:

\`\`\`txt
${SITE_URL}/llms.txt
\`\`\`
`;

const rawContents = docs.map((d) => `- ${d.title}`).join("\n");
const rawSections = docs
  .map((d) => `# ${d.title}\n\nSource: content/${d.path}\n\n${d.body}`)
  .join("\n\n---\n\n");
const rawText = `# ${SITE}\n\n> ${SUMMARY}\n\n## Contents\n\n${rawContents}\n\n---\n\n${rawSections}\n`;

const outputs = [
  { path: PAGE, label: "content/for-llms.mdx", content: page },
  { path: INDEX, label: "public/llms.txt", content: rawText },
];

if (checkOnly) {
  const stale = [];
  for (const out of outputs) {
    const current = existsSync(out.path) ? readFileSync(out.path, "utf8") : null;
    if (current !== out.content) {
      stale.push(out.label);
    }
  }
  if (stale.length > 0) {
    console.error(
      `error: LLM docs are out of date; run npm run docs:build. Stale: ${stale.join(", ")}`
    );
    process.exit(1);
  }
  console.log(`ok: LLM docs in sync (${docs.length} docs)`);
} else {
  mkdirSync(PUBLIC, { recursive: true });
  for (const out of outputs) {
    writeFileSync(out.path, out.content);
  }
  console.log(
    `ok: generated content/llms.mdx and public/llms.txt (all docs) from ${docs.length} docs`
  );
}
