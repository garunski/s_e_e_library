import { cpSync, existsSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const PUBLIC = join(ROOT, "public");

const assets = ["catalog.json", "packages"];

for (const asset of assets) {
  const src = join(ROOT, asset);
  const dest = join(PUBLIC, asset);
  if (!existsSync(src)) {
    console.error(`error: missing ${asset}`);
    process.exit(1);
  }
  rmSync(dest, { recursive: true, force: true });
  cpSync(src, dest, { recursive: true });
}

console.log(`ok: copied ${assets.join(", ")} into public/`);
