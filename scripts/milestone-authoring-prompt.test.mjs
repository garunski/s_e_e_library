import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";
import { milestoneAuthoringContractErrors } from "./milestone-authoring-prompt.mjs";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const PROMPT_FROM = "packages/system-milestone-authoring/1.0.0/prompt.json";

function promptContentFromCatalog(catalogPath) {
  const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));
  const entry = catalog.packages.find(
    (pkg) => pkg.slug === "system-milestone-authoring",
  );
  assert.ok(entry, `missing system-milestone-authoring in ${catalogPath}`);
  const file = entry.files.find((row) => row.from === PROMPT_FROM);
  assert.ok(file, `catalog ${catalogPath} must point at ${PROMPT_FROM}`);
  const payload = JSON.parse(readFileSync(join(ROOT, file.from), "utf8"));
  return payload.content;
}

test("stale minimal-milestone copy fails verification", () => {
  const stale =
    "MCP milestone tools: milestone_list, milestone_create.\nMinimal - milestones carry no acceptance criteria and no implementation plan.";
  const errors = milestoneAuthoringContractErrors(stale);
  assert.ok(errors.some((e) => e.includes("forbidden obsolete copy")));
  assert.ok(errors.some((e) => e.includes("milestone_get")));
  assert.ok(errors.some((e) => e.includes("milestone_add_criterion")));
});

test("installable milestone authoring prompt matches the typed record", () => {
  const payload = JSON.parse(readFileSync(join(ROOT, PROMPT_FROM), "utf8"));
  assert.deepEqual(milestoneAuthoringContractErrors(payload.content), []);
});

test("root catalog points at the corrected prompt payload", () => {
  const content = promptContentFromCatalog(join(ROOT, "catalog.json"));
  assert.deepEqual(milestoneAuthoringContractErrors(content), []);
});

test("public catalog points at the corrected prompt payload", () => {
  const content = promptContentFromCatalog(join(ROOT, "public", "catalog.json"));
  assert.deepEqual(milestoneAuthoringContractErrors(content), []);
});
