import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Bundles",
  "A bundle installs several library packages together using a shared catalog files map.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/bundles/"
      human={{
        kicker: "Grouped install",
        title: "A bundle is an install map.",
        body: "It has no payload of its own; files[] lists every member destination.",
      }}
      machine={{
        kicker: "Bundle package",
        title: "The install path decides the kind.",
        body: "The build decides payload kind from the destination prefix.",
      }}
    >
      <p className="page-lede">
        A bundle installs several library packages together. It has no payload
        file of its own; the catalog entry <code>files[]</code> lists every
        member install map.
      </p>

      <h2>Metadata (s_e_e_package.json)</h2>
      <p>
        Each bundle carries a sidecar at{" "}
        <code>packages/{"{slug}"}/s_e_e_package.json</code>:
      </p>
      <pre>{`{
  "id": "bundle-implement-story",
  "slug": "implement-story",
  "category": "bundle",
  "name": "Implement Story workflows",
  "description": "Packages linked by template references: workflows, prompts, skills, commands.",
  "labels": ["bulk", "story"],
  "dependencies": []
}`}</pre>
      <dl className="decision-list compact-list">
        <dt>
          <code>id</code>
        </dt>
        <dd>
          Globally unique package id (<code>bundle-{"<slug>"}</code>).
        </dd>
        <dt>
          <code>slug</code>
        </dt>
        <dd>
          URL-safe folder name under <code>packages/</code>.
        </dd>
        <dt>
          <code>category</code>
        </dt>
        <dd>
          Must be <code>bundle</code>.
        </dd>
        <dt>
          <code>name</code>, <code>description</code>
        </dt>
        <dd>Catalog presentation.</dd>
        <dt>
          <code>labels</code>
        </dt>
        <dd>Discovery tags.</dd>
        <dt>
          <code>dependencies</code>
        </dt>
        <dd>Other package ids installed alongside this bundle.</dd>
      </dl>

      <h2>Members</h2>
      <p>
        When harvesting from a local hub, <code>scripts/bundles.json</code>{" "}
        lists member package ids per bundle slug. The harvest script copies each
        member payload into <code>packages/{"{bundle-slug}"}/{"{version}"}/</code>{" "}
        and builds the catalog <code>files[]</code> array. Hand-authored bundles
        list every <code>to</code>/<code>from</code> pair directly in{" "}
        <code>catalog.json</code>.
      </p>
      <p>
        Example members for <code>implement-story</code> (from{" "}
        <code>scripts/bundles.json</code>):
      </p>
      <ul>
        <li>
          <code>cmd-cursor-agent</code>
        </li>
        <li>
          <code>prompt-system-implement-story</code>
        </li>
        <li>
          <code>skill-work</code>
        </li>
        <li>
          <code>wf-system-implement-story</code>
        </li>
        <li>
          <code>wf-system-implement-stories-bulk</code>
        </li>
      </ul>

      <h2>Install map</h2>
      <p>Each entry maps a repo path to an install target:</p>
      <pre>{`{
  "to": ".s_e_e/prompts/system-implement-story.json",
  "from": "packages/implement-story/1.0.0/system-implement-story/prompt.json"
}`}</pre>
      <p>
        The build infers payload kind from the <code>to</code> prefix:
      </p>
      <ul>
        <li>
          <code>.s_e_e/workflows/definitions/</code> is a workflow;
        </li>
        <li>
          <code>.s_e_e/workflows/schedules/</code> is a schedule;
        </li>
        <li>
          <code>.s_e_e/workflows/schedule_rule_sets/</code> is a schedule rule
          set;
        </li>
        <li>
          <code>.s_e_e/prompts/</code> is a prompt;
        </li>
        <li>
          <code>.s_e_e/commands/</code> is a command;
        </li>
        <li>
          <code>.s_e_e/knowledge/</code> is a knowledge markdown payload (for example
          the component inventory template);
        </li>
        <li>
          <code>.agents/skills/</code> is a skill;
        </li>
        <li>
          <code>.cursor/</code> is a rule;
        </li>
        <li>
          <code>templates/</code> is a template.
        </li>
      </ul>

      <h2>Install path rules</h2>
      <p>
        Every <code>files[]</code> <code>to</code> path must start with one of:
      </p>
      <ul>
        <li>
          <code>.s_e_e/</code>
        </li>
        <li>
          <code>.agents/</code>
        </li>
        <li>
          <code>.cursor/</code>
        </li>
        <li>
          <code>templates/</code>
        </li>
      </ul>
      <p>Paths use forward slashes and must be unique within the package.</p>

      <h2>Schema reference</h2>
      <ul className="example-list">
        <li>
          Catalog manifest:{" "}
          <a href={assetPath("/catalog.json")}>
            <code>see.library/v1</code>
          </a>
        </li>
        <li>
          <Link href="/schema/bundle/">Bundle schema</Link>
        </li>
        <li>
          <Link href="/schema/schedule/">Schedule schema</Link>
        </li>
        <li>
          <Link href="/schema/schedule-rule-set/">Schedule rule set schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
