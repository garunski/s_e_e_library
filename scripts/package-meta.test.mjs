import assert from "node:assert/strict";
import test from "node:test";
import {
  catalogWithoutUpdated,
  marketplaceRefErrors,
  normalizeReleases,
  normalizeToolIds,
  stableStringify,
} from "./package-meta.mjs";

test("normalizeToolIds sorts unique ids and rejects blanks", () => {
  assert.deepEqual(normalizeToolIds(["claude", "cursor", "claude"]), [
    "claude",
    "cursor",
  ]);
  assert.equal(normalizeToolIds(["cursor", ""]), null);
  assert.equal(normalizeToolIds("cursor"), null);
});

test("normalizeToolIds does not infer ids from a package name", () => {
  const name = "Cursor Claude Skill";
  const toolIds = normalizeToolIds([]);
  assert.deepEqual(toolIds, []);
  assert.ok(!toolIds.some((id) => name.toLowerCase().includes(id)));
});

test("normalizeReleases keeps authored history and rejects incomplete rows", () => {
  assert.deepEqual(
    normalizeReleases([
      { version: "1.0.0", date: "2026-09-01", note: "first" },
    ]),
    [{ version: "1.0.0", date: "2026-09-01", note: "first" }],
  );
  assert.equal(normalizeReleases([{ version: "1.0.0", date: "2026-09-01" }]), null);
  assert.equal(normalizeReleases("1.0.0"), null);
});

test("marketplaceRefErrors rejects unknown ids and a featured stack that is not declared", () => {
  const errors = marketplaceRefErrors({
    tools: [
      { id: "cursor", name: "Cursor" },
      { id: "cursor", name: "Cursor again" },
    ],
    stacks: [
      {
        id: "stack-a",
        name: "A",
        sharedPackageIds: ["missing"],
        variants: [{ toolId: "ghost", packageIds: ["wf-1"] }],
      },
    ],
    featuredStackId: "stack-missing",
    packageIds: ["wf-1"],
  });
  assert.ok(errors.some((e) => e.includes("duplicate tool id: cursor")));
  assert.ok(errors.some((e) => e.includes("unknown stack package id: missing")));
  assert.ok(errors.some((e) => e.includes("undeclared tool in stack variant: ghost")));
  assert.ok(errors.some((e) => e.includes("unknown featured stack id: stack-missing")));
});

test("marketplaceRefErrors accepts a featured stack that resolves shared and variant packages", () => {
  const errors = marketplaceRefErrors({
    tools: [{ id: "cursor", name: "Cursor" }],
    stacks: [
      {
        id: "stack-a",
        name: "A",
        sharedPackageIds: ["wf-1"],
        variants: [{ toolId: "cursor", packageIds: ["cmd-cursor-agent"] }],
      },
    ],
    featuredStackId: "stack-a",
    packageIds: ["wf-1", "cmd-cursor-agent"],
  });
  assert.deepEqual(errors, []);
});

test("catalog check fails when sidecar marketplace fields are not in catalog.json", () => {
  const sidecar = {
    toolIds: ["cursor"],
    releases: [{ version: "1.0.0", date: "2026-09-01", note: "first" }],
  };
  const stale = {
    schema: "see.library/v2",
    name: "Official",
    updated: "2026-09-01T00:00:00.000Z",
    packages: [{ id: "wf-1", toolIds: [], releases: [] }],
  };
  const next = {
    schema: "see.library/v2",
    name: "Official",
    updated: "2026-09-02T00:00:00.000Z",
    packages: [{ id: "wf-1", ...sidecar }],
  };
  const publicStale = structuredClone(stale);
  assert.notEqual(
    stableStringify(catalogWithoutUpdated(stale)),
    stableStringify(catalogWithoutUpdated(next)),
  );
  assert.notEqual(
    stableStringify(catalogWithoutUpdated(publicStale)),
    stableStringify(catalogWithoutUpdated(next)),
  );
  const regenerated = structuredClone(next);
  assert.equal(
    stableStringify(catalogWithoutUpdated(regenerated)),
    stableStringify(catalogWithoutUpdated(next)),
  );
})
