import { defineConfig } from "vitepress";
import {
  HUMAN_DOC_EXTRA_PAGES,
  HUMAN_DOC_PIECE_TYPES,
} from "../../scripts/human-docs-piece-types.mjs";

const humanSidebar = [
  { text: "Overview", link: "/humans/" },
  ...HUMAN_DOC_PIECE_TYPES.map(({ page, title }) => ({
    text: title,
    link: `/humans/${page.replace(/\.md$/, "")}`,
  })),
  ...HUMAN_DOC_EXTRA_PAGES.map(({ page, title }) => ({
    text: title,
    link: `/humans/${page.replace(/\.md$/, "")}`,
  })),
];

const schemaSidebar = [
  { text: "Catalog schema", link: "/schema/" },
  { text: "Workflow", link: "/schema/workflow" },
  { text: "Prompt", link: "/schema/prompt" },
  { text: "Skill", link: "/schema/skill" },
  { text: "Command", link: "/schema/command" },
  { text: "Rule & template", link: "/schema/rule-template" },
  { text: "Bundle", link: "/schema/bundle" },
];

export default defineConfig({
  title: "S.E.E. Official Library",
  description:
    "Versioned workflows, prompts, skills, commands, and bundles for hub and spoke projects.",
  base: "/s_e_e_library/",
  outDir: "../.site",
  themeConfig: {
    nav: [
      { text: "Catalog", link: "/catalog.json" },
      { text: "Human docs", link: "/humans/" },
      { text: "Schema", link: "/schema/" },
      { text: "LLM docs", link: "/llms.txt" },
    ],
    sidebar: {
      "/humans/": humanSidebar,
      "/schema/": schemaSidebar,
    },
  },
});
