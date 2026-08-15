import assert from "node:assert/strict";
import { existsSync, mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import test from "node:test";
import { HUMAN_DOC_PAGES } from "./human-docs-piece-types.mjs";
import { verifyHumanDocs } from "./verify-human-docs.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const SITE = join(ROOT, ".site");
const CONFIG = join(ROOT, "docs", ".vitepress", "config.mjs");

test("Next.js runtime artifacts are absent", () => {
  assert.ok(!existsSync(join(ROOT, "next.config.mjs")));
  assert.ok(!existsSync(join(ROOT, "app")));
  const needle = "nex" + "tra";
  const rg = spawnSync(
    "rg",
    ["-n", needle, ROOT, "--glob", "!node_modules/**", "--glob", "!.next/**"],
    { encoding: "utf8" }
  );
  assert.equal(rg.status, 1);
  assert.equal(rg.stdout.trim(), "");
});

test("vitepress config sets GitHub Pages base path", async () => {
  const mod = await import(CONFIG);
  assert.equal(mod.default.base, "/s_e_e_library/");
  assert.equal(mod.default.outDir, "../.site");
});

test("vitepress config exposes human docs sidebar entries", async () => {
  const mod = await import(CONFIG);
  const sidebar = mod.default.themeConfig.sidebar["/humans/"];
  assert.ok(Array.isArray(sidebar));
  for (const page of HUMAN_DOC_PAGES) {
    const slug = page.replace(/\.md$/, "");
    assert.ok(
      sidebar.some((entry) => entry.link === `/humans/${slug}`),
      `sidebar missing /humans/${slug}`
    );
  }
});

test("verifyHumanDocs passes for the repository", () => {
  const errors = verifyHumanDocs();
  assert.deepEqual(errors, []);
});

test("verifyHumanDocs fails when a piece type page is missing", () => {
  const tempRoot = mkdtempSync(join(tmpdir(), "see-human-docs-"));
  try {
    const humansDir = join(tempRoot, "docs", "humans");
    mkdirSync(humansDir, { recursive: true });
    const index = HUMAN_DOC_PAGES.map((page) => {
      const slug = page.replace(/\.md$/, "");
      return `- [${slug}](./${page})`;
    }).join("\n");
    writeFileSync(join(humansDir, "index.md"), index);
    for (const page of HUMAN_DOC_PAGES) {
      if (page === "commands.md") {
        continue;
      }
      writeFileSync(join(humansDir, page), `# ${page}\n`);
    }
    const errors = verifyHumanDocs(tempRoot);
    assert.ok(
      errors.some((message) => message.includes("command") && message.includes("commands.md"))
    );
  } finally {
    rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("duplicate authoring tree is absent", () => {
  assert.ok(!existsSync(join(ROOT, "content", "authoring")));
  assert.ok(existsSync(join(ROOT, "docs", "humans")));
});

test("build-llms --check passes", () => {
  const check = spawnSync("node", ["scripts/build-llms.mjs", "--check"], {
    cwd: ROOT,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
  assert.equal(
    check.status,
    0,
    `build-llms --check failed:\n${check.stdout}\n${check.stderr}`
  );
});

test("vitepress config exposes schema sidebar entries", async () => {
  const mod = await import(CONFIG);
  const sidebar = mod.default.themeConfig.sidebar["/schema/"];
  assert.ok(Array.isArray(sidebar));
  for (const slug of [
    "workflow",
    "prompt",
    "skill",
    "command",
    "rule-template",
    "bundle",
  ]) {
    assert.ok(
      sidebar.some((entry) => entry.link === `/schema/${slug}`),
      `sidebar missing /schema/${slug}`
    );
  }
});

test("docs:build emits assembled deploy directory", () => {
  const build = spawnSync("npm", ["run", "docs:build"], {
    cwd: ROOT,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
  assert.equal(
    build.status,
    0,
    `docs:build failed:\n${build.stdout}\n${build.stderr}`
  );

  for (const rel of [
    "index.html",
    "catalog.json",
    "packages",
    "llms.txt",
    "humans/index.html",
    "humans/commands.html",
    "humans/prompts.html",
    "humans/skills.html",
    "humans/workflows.html",
    "humans/bundles.html",
    "humans/publish.html",
    "schema/index.html",
    "schema/workflow.html",
    ".nojekyll",
  ]) {
    assert.ok(
      existsSync(join(SITE, rel)),
      `expected ${rel} in deploy directory`
    );
  }

  const index = readFileSync(join(SITE, "index.html"), "utf8");
  assert.match(index, /\/s_e_e_library\//);
  assert.match(index, /S\.E\.E\. Official Library/);
  assert.match(index, /\/s_e_e_library\/humans\//);

  const llms = readFileSync(join(SITE, "llms.txt"), "utf8");
  assert.match(llms, /Source: docs\/humans\/commands\.md/);
  assert.match(llms, /Source: docs\/schema\/workflow\.md/);
  assert.match(llms, /Catalog schema/);

  const catalog = JSON.parse(readFileSync(join(SITE, "catalog.json"), "utf8"));
  assert.ok(Array.isArray(catalog.packages));
  assert.ok(catalog.packages.length > 0);
});
