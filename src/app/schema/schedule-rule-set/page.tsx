import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Schedule rule set schema",
  "Named subject-filter rules for schedules. id and name are required. Validated on save.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/schedule-rule-set/"
      human={{
        kicker: "Hub document",
        title: "A rule set is a named filter.",
        body: "Schedules reference it from story_rules instead of inlining the same rules.",
      }}
      machine={{
        kicker: ".s_e_e/workflows/schedule_rule_sets",
        title: "Validators run on save.",
        body: "id must be a safe JSON filename.",
      }}
    >
      <p className="page-lede">
        Schedule rule set. Installs to{" "}
        <code>.s_e_e/workflows/schedule_rule_sets/{"{id}"}.json</code>.
      </p>
      <ul>
        <li>
          <code>id</code>, <code>name</code> - required, non-empty.
        </li>
        <li>
          <code>description</code> - string.
        </li>
        <li>
          <code>rules</code> - object (statuses, labels, acceptance-criteria
          flags).
        </li>
      </ul>
      <p>
        See <Link href="/schema/schedule/">schedule</Link> for{" "}
        <code>story_rules</code> that point at a rule set id.
      </p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="schedule-rule-set.schema.json" />
    </SchemaPage>
  );
}
