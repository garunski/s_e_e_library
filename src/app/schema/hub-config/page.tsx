import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Hub config schema",
  "Hub data-root config.json. A JSON object with at least one property. Validated on save.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/hub-config/"
      human={{
        kicker: "Hub document",
        title: "config.json is a JSON object.",
        body: "Stored at the hub data root. An empty object is rejected.",
      }}
      machine={{
        kicker: ".s_e_e/config.json",
        title: "Validators run on save.",
        body: "additionalProperties is allowed so project-specific keys persist.",
      }}
    >
      <p className="page-lede">
        Hub data-root <code>config.json</code>. Must be a non-empty JSON object.
      </p>
      <ul>
        <li>
          <code>project</code> - object; typically includes <code>id</code>.
        </li>
        <li>
          <code>overrides</code> - object.
        </li>
        <li>
          <code>library</code> - catalog sources and collections. See{" "}
          <Link href="/schema/">catalog schema</Link>.
        </li>
        <li>
          <code>stories</code> - see{" "}
          <Link href="/schema/stories-config/">stories config</Link>.
        </li>
        <li>
          <code>llm.routing</code> - see{" "}
          <Link href="/schema/routing-rules/">routing rules</Link>.
        </li>
        <li>
          <code>orchestrator</code> - see{" "}
          <Link href="/schema/orchestrator-policy/">orchestrator policy</Link>.
        </li>
      </ul>
      <p>Unknown top-level keys are kept.</p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="hub-config.schema.json" />
    </SchemaPage>
  );
}
