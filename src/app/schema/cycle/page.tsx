import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Cycle schema",
  "Category cycle. Payload file cycle.json. Installs to .s_e_e/cycles/{id}.json.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/cycle/"
      human={{
        kicker: "Cycle payload",
        title: "Stages wire workflows to stores.",
        body: "The host decides which store keys are legal.",
      }}
      machine={{
        kicker: "Category cycle",
        title: "Installs under .s_e_e/cycles.",
        body: "Destination is .s_e_e/cycles/{id}.json where id matches the document id.",
      }}
    >
      <p className="page-lede">
        Category <code>cycle</code>. Payload file <code>cycle.json</code>.
        Installs to <code>.s_e_e/cycles/{"{id}"}.json</code> (the{" "}
        <code>to</code> path must equal <code>.s_e_e/cycles/</code> plus the
        document <code>id</code> plus <code>.json</code>).
      </p>
      <ul>
        <li>
          <code>id</code> - stable cycle id. Required, non-empty; must match the
          install file name.
        </li>
        <li>
          <code>name</code> - display name. Required, non-empty.
        </li>
        <li>
          <code>host</code> - <code>orchestrator</code> or <code>knowledge</code>{" "}
          (the hosts the app can activate today).
        </li>
        <li>
          <code>schema_version</code> - integer, currently <code>1</code>.
        </li>
        <li>
          <code>stores</code> - store keys this cycle uses; each must be declared
          for that host in the app registry.
        </li>
        <li>
          <code>stages[]</code> - ordered stages. Each stage has <code>key</code>
          , <code>verb</code>, <code>workflow_definition_id</code>,{" "}
          <code>trigger</code> (<code>tick</code>, <code>schedule</code>,{" "}
          <code>event</code>, or <code>manual</code>), <code>reads</code> (store
          keys), and <code>writes</code> (one store key).
        </li>
      </ul>
      <p>
        See the <Link href="/authoring/cycles/">cycle authoring guide</Link> for
        store registry keys, the gate stage on the orchestrator host, and
        activation checks.
      </p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="cycle-document.schema.json" />

      <p>
        See the <Link href="/schema/">catalog schema</Link>.
      </p>
    </SchemaPage>
  );
}
