import { existsSync, readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import {
  AUTHORING_EXTRA_PAGES,
  AUTHORING_PAGES,
  PACKAGE_TYPES,
  SCHEMA_PAGES,
} from "./authoring-pages.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");

function pagePath(root, slug) {
  return join(root, "src", "app", "authoring", slug, "page.tsx");
}

export function verifyAuthoringDocs(root = ROOT) {
  const indexPath = join(root, "src", "app", "authoring", "page.tsx");
  const errors = [];

  if (!existsSync(indexPath)) {
    errors.push("src/app/authoring/page.tsx is missing");
    return errors;
  }

  const index = readFileSync(indexPath, "utf8");

  for (const { key, slug, title } of PACKAGE_TYPES) {
    const file = pagePath(root, slug);
    if (!existsSync(file)) {
      errors.push(
        `missing authoring page for package type ${key}: src/app/authoring/${slug}/page.tsx`,
      );
      continue;
    }
    if (!index.includes(`/authoring/${slug}/`)) {
      errors.push(
        `src/app/authoring/page.tsx does not link to ${title} (/authoring/${slug}/)`,
      );
    }
  }

  for (const { slug, title } of AUTHORING_EXTRA_PAGES) {
    const file = pagePath(root, slug);
    if (!existsSync(file)) {
      errors.push(`missing authoring page: src/app/authoring/${slug}/page.tsx`);
      continue;
    }
    if (!index.includes(`/authoring/${slug}/`)) {
      errors.push(
        `src/app/authoring/page.tsx does not link to ${title} (/authoring/${slug}/)`,
      );
    }
  }

  return errors;
}

export function verifyLlmsCoverage(root = ROOT) {
  const llmsPath = join(root, "public", "llms.txt");
  const errors = [];

  if (!existsSync(llmsPath)) {
    errors.push("public/llms.txt is missing");
    return errors;
  }

  const llms = readFileSync(llmsPath, "utf8");
  const sources = [
    "/authoring/",
    ...AUTHORING_PAGES.map((slug) => `/authoring/${slug}/`),
    "/schema/",
    ...SCHEMA_PAGES.map((slug) => `/schema/${slug}/`),
  ];

  for (const source of sources) {
    if (!llms.includes(`Source: ${source}`)) {
      errors.push(
        `public/llms.txt has no section for ${source} (expected a "Source: ${source}" line)`,
      );
    }
  }

  return errors;
}

const isMain =
  process.argv[1] &&
  fileURLToPath(import.meta.url) === join(process.argv[1]);

if (isMain) {
  const errors = [...verifyAuthoringDocs(), ...verifyLlmsCoverage()];
  if (errors.length > 0) {
    for (const message of errors) {
      console.error(`error: ${message}`);
    }
    process.exit(1);
  }
  console.log("ok: authoring pages and llms.txt cover every package type");
}
