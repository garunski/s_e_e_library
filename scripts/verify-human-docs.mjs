import { existsSync, readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import {
  HUMAN_DOC_EXTRA_PAGES,
  HUMAN_DOC_PIECE_TYPES,
} from "./human-docs-piece-types.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const HUMANS_DIR = join(ROOT, "docs", "humans");
const INDEX_PATH = join(HUMANS_DIR, "index.md");

export function verifyHumanDocs(root = ROOT) {
  const humansDir = join(root, "docs", "humans");
  const indexPath = join(humansDir, "index.md");
  const errors = [];

  if (!existsSync(indexPath)) {
    errors.push("docs/humans/index.md is missing");
    return errors;
  }

  const index = readFileSync(indexPath, "utf8");

  for (const { key, page, title } of HUMAN_DOC_PIECE_TYPES) {
    const pagePath = join(humansDir, page);
    if (!existsSync(pagePath)) {
      errors.push(`missing human doc page for piece type ${key}: docs/humans/${page}`);
      continue;
    }
    const slug = page.replace(/\.md$/, "");
    if (!index.includes(`/${slug}`) && !index.includes(`${slug}.md`)) {
      errors.push(`docs/humans/index.md does not link to ${title} (/${slug})`);
    }
  }

  for (const { page, title } of HUMAN_DOC_EXTRA_PAGES) {
    const pagePath = join(humansDir, page);
    if (!existsSync(pagePath)) {
      errors.push(`missing human doc page: docs/humans/${page}`);
      continue;
    }
    const slug = page.replace(/\.md$/, "");
    if (!index.includes(`/${slug}`) && !index.includes(`${slug}.md`)) {
      errors.push(`docs/humans/index.md does not link to ${title} (/${slug})`);
    }
  }

  return errors;
}

const isMain =
  process.argv[1] &&
  fileURLToPath(import.meta.url) === join(process.argv[1]);

if (isMain) {
  const errors = verifyHumanDocs();
  if (errors.length > 0) {
    for (const message of errors) {
      console.error(`error: ${message}`);
    }
    process.exit(1);
  }
  console.log("ok: human docs cover every library piece type");
}
