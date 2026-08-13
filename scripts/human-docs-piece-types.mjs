export const HUMAN_DOC_PIECE_TYPES = [
  { key: "command", page: "commands.md", title: "Commands" },
  { key: "prompt", page: "prompts.md", title: "Prompts" },
  { key: "skill", page: "skills.md", title: "Skills" },
  { key: "workflow", page: "workflows.md", title: "Workflows" },
  { key: "bundle", page: "bundles.md", title: "Bundles" },
];

export const HUMAN_DOC_EXTRA_PAGES = [
  { page: "publish.md", title: "Publish to the catalog" },
];

export const HUMAN_DOC_PAGES = [
  ...HUMAN_DOC_PIECE_TYPES.map((entry) => entry.page),
  ...HUMAN_DOC_EXTRA_PAGES.map((entry) => entry.page),
];
