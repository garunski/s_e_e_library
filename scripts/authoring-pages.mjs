export const PACKAGE_TYPES = [
  { key: "workflow", slug: "workflows", title: "Workflows" },
  { key: "prompt", slug: "prompts", title: "Prompts" },
  { key: "skill", slug: "skills", title: "Skills" },
  { key: "command", slug: "commands", title: "Commands" },
  { key: "bundle", slug: "bundles", title: "Bundles" },
];

export const AUTHORING_EXTRA_PAGES = [
  { slug: "publish", title: "Publish to the catalog" },
];

export const AUTHORING_PAGES = [
  ...PACKAGE_TYPES.map((entry) => entry.slug),
  ...AUTHORING_EXTRA_PAGES.map((entry) => entry.slug),
];

export const SCHEMA_PAGES = [
  "workflow",
  "prompt",
  "skill",
  "command",
  "rule-template",
  "bundle",
];
