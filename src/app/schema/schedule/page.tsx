import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Schedule schema",
  "Workflow schedule JSON. id, name, workflow_id, kind, and enabled are required. Validated on save.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/schedule/"
      human={{
        kicker: "Hub document",
        title: "A schedule runs a workflow.",
        body: "kind is a tagged object: cron, interval, or entity_event.",
      }}
      machine={{
        kicker: ".s_e_e/workflows/schedules",
        title: "Validators run on save.",
        body: "workflow_id must name an existing workflow definition.",
      }}
    >
      <p className="page-lede">
        Workflow schedule. Installs to{" "}
        <code>.s_e_e/workflows/schedules/{"{id}"}.json</code>. Bundles may
        include schedule files; kind is inferred from the <code>to</code>{" "}
        prefix.
      </p>
      <ul>
        <li>
          <code>id</code>, <code>name</code>, <code>workflow_id</code> -
          required, non-empty.
        </li>
        <li>
          <code>kind</code> - required object with a <code>kind</code> tag (
          <code>cron</code>, <code>interval</code>, or <code>entity_event</code>
          ).
        </li>
        <li>
          <code>enabled</code> - required boolean.
        </li>
        <li>
          <code>concurrency</code>, <code>runtime_inputs</code>,{" "}
          <code>story_rules</code>, <code>catchup_policy</code>,{" "}
          <code>notifications</code> - objects.
        </li>
      </ul>
      <p>
        Subject filters can point at a{" "}
        <Link href="/schema/schedule-rule-set/">rule set</Link> by id.
      </p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="schedule.schema.json" />
    </SchemaPage>
  );
}
