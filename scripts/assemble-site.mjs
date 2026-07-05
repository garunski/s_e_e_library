import { cpSync, existsSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const SITE = join(ROOT, "out");

if (!existsSync(SITE)) {
  console.error("error: out/ not found; run next build first");
  process.exit(1);
}

writeFileSync(join(SITE, ".nojekyll"), "");

const assets = ["catalog.json", "packages"];

for (const asset of assets) {
  const src = join(ROOT, asset);
  if (!existsSync(src)) {
    console.error(`error: missing ${asset}`);
    process.exit(1);
  }
  cpSync(src, join(SITE, asset), { recursive: true });
}

console.log(`ok: assembled site at ${SITE} (${assets.join(", ")}, .nojekyll)`);
