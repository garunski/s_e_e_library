import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Authoring",
  "How to author each S.E.E. library package type: workflows, prompts, skills, commands, bundles, cycles, and publishing to the catalog.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/"
      human={{
        kicker: "Authoring",
        title: "Write the package you need.",
        body: "One guide per package type, then Publish adds it to the catalog.",
      }}
      machine={{
        kicker: "Six package types",
        title: "Each type has a payload shape.",
        body: "The guide names the file, the fields, and the install destination.",
      }}
    >
      <p className="page-lede">
        Every package in the Official Library is one of six types. Pick the
        type, write its payload, then publish it to the catalog.
      </p>

      <h2>Package types</h2>
      <ul className="example-list named-examples">
        <li>
          <strong>
            <Link href="/authoring/workflows/">Workflows</Link>
          </strong>
          <span>
            Task graphs the engine runs (<code>definition.json</code>).
          </span>
        </li>
        <li>
          <strong>
            <Link href="/authoring/prompts/">Prompts</Link>
          </strong>
          <span>
            Named instruction text (<code>prompt.json</code>).
          </span>
        </li>
        <li>
          <strong>
            <Link href="/authoring/skills/">Skills</Link>
          </strong>
          <span>
            Activatable capability guides (<code>SKILL.md</code>).
          </span>
        </li>
        <li>
          <strong>
            <Link href="/authoring/commands/">Commands</Link>
          </strong>
          <span>
            Worker definitions for an agent CLI (<code>command.json</code>).
          </span>
        </li>
        <li>
          <strong>
            <Link href="/authoring/bundles/">Bundles</Link>
          </strong>
          <span>Install maps that deliver several packages together.</span>
        </li>
        <li>
          <strong>
            <Link href="/authoring/cycles/">Cycles</Link>
          </strong>
          <span>
            Host loops wiring workflows to stores (<code>cycle.json</code>).
          </span>
        </li>
      </ul>

      <h2>Publishing</h2>
      <p>
        <Link href="/authoring/publish/">Publish to the catalog</Link> covers
        payload layout, the <code>catalog.json</code> entry, and validation.
      </p>

      <h2>The contract</h2>
      <ul className="example-list">
        <li>
          <Link href="/schema/">Catalog schema</Link>, per-type payload schemas,
          and hub document schemas
        </li>
        <li>
          Catalog manifest:{" "}
          <a href={assetPath("/catalog.json")}>
            <code>see.library/v1</code>
          </a>
        </li>
        <li>
          Same guidance as one plain-text file for agents:{" "}
          <a href={assetPath("/llms.txt")}>
            <code>llms.txt</code>
          </a>
        </li>
      </ul>
    </AuthoringPage>
  );
}
