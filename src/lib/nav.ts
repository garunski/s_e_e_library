export type NavLink = {
  href: string;
  label: string;
  external?: boolean;
  file?: boolean;
};

export type NavSection = {
  id: "authoring" | "schema";
  label: string;
  ariaLabel: string;
  links: readonly NavLink[];
};

export const siteName = "S.E.E. Library";

export const brandLockup = {
  primary: "S.E.E.",
  secondary: "Library",
};

export const siteUrl = "https://garunski.github.io/s_e_e_site/";
export const libraryRepoUrl = "https://github.com/garunski/s_e_e_library";

export const footerLinks: readonly NavLink[] = [
  { href: siteUrl, label: "S.E.E. product site", external: true },
  { href: libraryRepoUrl, label: "Catalog repository", external: true },
];

export const primaryNav: readonly NavLink[] = [
  { href: "/authoring/", label: "Authoring" },
  { href: "/schema/", label: "Schema" },
  { href: "/llms.txt", label: "llms.txt", file: true },
];

export const headerCta: NavLink = {
  href: "/catalog.json",
  label: "Open catalog.json",
  file: true,
};

export const authoringSection: NavSection = {
  id: "authoring",
  label: "Authoring",
  ariaLabel: "Authoring guides",
  links: [
    { href: "/authoring/", label: "Overview" },
    { href: "/authoring/workflows/", label: "Workflows" },
    { href: "/authoring/prompts/", label: "Prompts" },
    { href: "/authoring/skills/", label: "Skills" },
    { href: "/authoring/commands/", label: "Commands" },
    { href: "/authoring/bundles/", label: "Bundles" },
    { href: "/authoring/publish/", label: "Publish" },
  ],
};

export const schemaSection: NavSection = {
  id: "schema",
  label: "Schema",
  ariaLabel: "Schema pages",
  links: [
    { href: "/schema/", label: "Catalog" },
    { href: "/schema/workflow/", label: "Workflow" },
    { href: "/schema/prompt/", label: "Prompt" },
    { href: "/schema/skill/", label: "Skill" },
    { href: "/schema/command/", label: "Command" },
    { href: "/schema/rule-template/", label: "Rule and template" },
    { href: "/schema/bundle/", label: "Bundle" },
    { href: "/schema/hub-config/", label: "Hub config" },
    { href: "/schema/global-config/", label: "Global config" },
    { href: "/schema/app-settings/", label: "App settings" },
    { href: "/schema/routing-rules/", label: "Routing rules" },
    { href: "/schema/stories-config/", label: "Stories config" },
    { href: "/schema/orchestrator-policy/", label: "Orchestrator policy" },
    { href: "/schema/schedule/", label: "Schedule" },
    { href: "/schema/schedule-rule-set/", label: "Schedule rule set" },
  ],
};

function sectionRoot(href: string): string {
  const first = href.split("/").filter(Boolean)[0];
  return first ? `/${first}/` : "/";
}

export function isCurrentHref(href: string, current: string): boolean {
  return href === current;
}

export function isPrimaryCurrent(href: string, current: string): boolean {
  if (href.endsWith(".json") || href.endsWith(".txt")) {
    return href === current;
  }
  const section = sectionRoot(href);
  return current === section || current.startsWith(section);
}

export function assetPath(path: string): string {
  const base = process.env.PAGES_BASE_PATH ?? "";
  return `${base}${path}`;
}
