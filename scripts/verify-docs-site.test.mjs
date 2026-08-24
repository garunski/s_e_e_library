import assert from "node:assert/strict";
import {
  existsSync,
  mkdtempSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import test from "node:test";
import { AUTHORING_PAGES, SCHEMA_PAGES } from "./authoring-pages.mjs";
import {
  verifyAuthoringDocs,
  verifyLlmsCoverage,
} from "./verify-authoring-docs.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const OUT = join(ROOT, "out");

test("legacy docs publisher artifacts are absent", () => {
  assert.ok(!existsSync(join(ROOT, "docs")));
  assert.ok(!existsSync(join(ROOT, "scripts", "build-llms.mjs")));
  assert.ok(!existsSync(join(ROOT, "scripts", "assemble-site.mjs")));
});

test("Next.js config exports a static GitHub Pages site", () => {
  const src = readFileSync(join(ROOT, "next.config.ts"), "utf8");
  assert.match(src, /output:\s*"export"/);
  assert.match(src, /trailingSlash:\s*true/);
});

test("authoring pages exist for every package type", () => {
  for (const slug of AUTHORING_PAGES) {
    assert.ok(
      existsSync(join(ROOT, "src", "app", "authoring", slug, "page.tsx")),
      `missing src/app/authoring/${slug}/page.tsx`,
    );
  }
});

test("schema pages exist", () => {
  for (const slug of SCHEMA_PAGES) {
    assert.ok(
      existsSync(join(ROOT, "src", "app", "schema", slug, "page.tsx")),
      `missing src/app/schema/${slug}/page.tsx`,
    );
  }
});

test("the authoring section has no leftover humans route", () => {
  assert.ok(!existsSync(join(ROOT, "src", "app", "humans")));
  const nav = readFileSync(join(ROOT, "src", "lib", "nav.ts"), "utf8");
  assert.ok(!nav.includes("/humans/"));
});

test("verifyAuthoringDocs passes for the repository", () => {
  assert.deepEqual(verifyAuthoringDocs(), []);
});

test("verifyAuthoringDocs fails when a package type page is missing", () => {
  const tempRoot = mkdtempSync(join(tmpdir(), "see-authoring-docs-"));
  try {
    const authoringDir = join(tempRoot, "src", "app", "authoring");
    mkdirSync(authoringDir, { recursive: true });
    const index = AUTHORING_PAGES.map(
      (slug) => `<Link href="/authoring/${slug}/">${slug}</Link>`,
    ).join("\n");
    writeFileSync(join(authoringDir, "page.tsx"), index);
    for (const slug of AUTHORING_PAGES) {
      if (slug === "commands") {
        continue;
      }
      mkdirSync(join(authoringDir, slug), { recursive: true });
      writeFileSync(
        join(authoringDir, slug, "page.tsx"),
        `export default function Page() { return null; }\n`,
      );
    }
    const errors = verifyAuthoringDocs(tempRoot);
    assert.ok(
      errors.some(
        (message) =>
          message.includes("command") && message.includes("commands"),
      ),
    );
  } finally {
    rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("llms.txt carries a section for every page", () => {
  assert.deepEqual(verifyLlmsCoverage(), []);
});

test("verifyLlmsCoverage fails when a section is missing", () => {
  const tempRoot = mkdtempSync(join(tmpdir(), "see-llms-"));
  try {
    mkdirSync(join(tempRoot, "public"), { recursive: true });
    writeFileSync(join(tempRoot, "public", "llms.txt"), "Source: /authoring/\n");
    const errors = verifyLlmsCoverage(tempRoot);
    assert.ok(errors.some((message) => message.includes("/schema/workflow/")));
  } finally {
    rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("llms.txt states the catalog contract without stale markup", () => {
  const llms = readFileSync(join(ROOT, "public", "llms.txt"), "utf8");
  assert.match(llms, /S\.E\.E\. Official Library/);
  assert.match(llms, /see\.library\/v1/);
  assert.ok(!llms.includes("v-pre"));
  assert.ok(!llms.includes("maintained by hand"));
  assert.ok(!llms.includes("/humans/"));
});

test("copy-public-assets copies catalog.json and packages into public/", () => {
  const copy = spawnSync("node", ["scripts/copy-public-assets.mjs"], {
    cwd: ROOT,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
  assert.equal(
    copy.status,
    0,
    `copy-public-assets failed:\n${copy.stdout}\n${copy.stderr}`,
  );
  assert.ok(existsSync(join(ROOT, "public", "catalog.json")));
  assert.ok(existsSync(join(ROOT, "public", "packages")));
});

test("npm run build emits assembled deploy directory", () => {
  const build = spawnSync("npm", ["run", "build"], {
    cwd: ROOT,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
  assert.equal(
    build.status,
    0,
    `build failed:\n${build.stdout}\n${build.stderr}`,
  );

  for (const rel of [
    "index.html",
    "catalog.json",
    "packages",
    "llms.txt",
    "authoring/index.html",
    ...AUTHORING_PAGES.map((slug) => `authoring/${slug}/index.html`),
    "schema/index.html",
    ...SCHEMA_PAGES.map((slug) => `schema/${slug}/index.html`),
    ".nojekyll",
  ]) {
    assert.ok(existsSync(join(OUT, rel)), `expected ${rel} in deploy directory`);
  }

  assert.ok(!existsSync(join(OUT, "humans")));

  const index = readFileSync(join(OUT, "index.html"), "utf8");
  assert.match(index, /S\.E\.E\. Official Library/);
  assert.match(index, /authoring\//);

  const catalog = JSON.parse(readFileSync(join(OUT, "catalog.json"), "utf8"));
  assert.ok(Array.isArray(catalog.packages));
  assert.ok(catalog.packages.length > 0);
});
