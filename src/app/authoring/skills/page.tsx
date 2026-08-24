import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Skills",
  "A skill package teaches an agent a repeatable capability with YAML frontmatter and a markdown body.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/skills/"
      human={{
        kicker: "Repeatable capability",
        title: "A skill teaches a procedure.",
        body: "Frontmatter selects when it activates; the body documents the steps.",
      }}
      machine={{
        kicker: "Skill package",
        title: "Installs under .agents/skills.",
        body: "Payload file SKILL.md. Destination is .agents/skills/{slug}/SKILL.md.",
      }}
    >
      <p className="page-lede">
        A skill package teaches an agent a repeatable capability. Payload file:{" "}
        <code>SKILL.md</code>. Installs to{" "}
        <code>.agents/skills/{"{slug}"}/SKILL.md</code>.
      </p>

      <h2>Shape</h2>
      <p>YAML frontmatter plus markdown body:</p>
      <ul>
        <li>
          <code>name</code>: short skill id (matches the install folder name)
        </li>
        <li>
          <code>description</code>: activation text; when the user says these
          phrases, the agent should load this skill
        </li>
      </ul>
      <p>
        The body documents the procedure, constraints, and links to
        authoritative docs.
      </p>

      <h2>Reference example</h2>
      <p>
        From <code>packages/work/1.0.0/SKILL.md</code>:
      </p>
      <pre>{`---
name: work
description: >-
  Implements an S.E.E. story with tests until acceptance criteria pass...
  Use when the user says work, "start work", "implement story-N", or works
  through story IDs with acceptance criteria.
---

# Work

Implement an S.E.E. story with tests until all acceptance criteria pass...`}</pre>
      <p>
        Write the <code>description</code> as a single activation sentence
        agents can match against user intent. Keep procedures in the markdown
        body, not in frontmatter.
      </p>

      <h2>Install path</h2>
      <pre>{`{
  "to": ".agents/skills/work/SKILL.md",
  "from": "packages/work/1.0.0/SKILL.md"
}`}</pre>

      <h2>Schema reference</h2>
      <ul className="example-list">
        <li>
          Catalog manifest:{" "}
          <a href={assetPath("/catalog.json")}>
            <code>see.library/v1</code>
          </a>
        </li>
        <li>
          <Link href="/schema/skill/">Skill schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
