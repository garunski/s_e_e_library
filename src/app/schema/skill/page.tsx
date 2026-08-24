import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Skill schema",
  "Category skill. Payload file SKILL.md. Installs to .agents/skills/{slug}/SKILL.md.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/skill/"
      human={{
        kicker: "Skill payload",
        title: "name and description are required.",
        body: "Frontmatter selects when the skill activates. The body is the procedure.",
      }}
      machine={{
        kicker: "Category skill",
        title: "Installs under .agents/skills.",
        body: "Destination is .agents/skills/{slug}/SKILL.md.",
      }}
    >
      <p className="page-lede">
        Category <code>skill</code>. Payload file <code>SKILL.md</code>.
        Installs to <code>.agents/skills/{"{slug}"}/SKILL.md</code>.
      </p>
      <p>
        <code>SKILL.md</code> is YAML frontmatter plus a markdown body.
        Frontmatter:
      </p>
      <ul>
        <li>
          <code>name</code> - non-empty skill name.
        </li>
        <li>
          <code>description</code> - non-empty activation description an agent
          matches against.
        </li>
      </ul>
      <p>The markdown after the frontmatter is the skill instructions.</p>

      <h2>JSON Schema</h2>
      <p>
        The payload is a markdown file; this schema describes its YAML
        frontmatter block.
      </p>
      <SchemaBlock file="skill-frontmatter.schema.json" />

      <p>
        See the <Link href="/authoring/skills/">skill authoring guide</Link> and the{" "}
        <Link href="/schema/">catalog schema</Link>.
      </p>
    </SchemaPage>
  );
}
