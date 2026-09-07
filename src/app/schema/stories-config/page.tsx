import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Stories config schema",
  "Stories section of hub config.json. project_name is required.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/stories-config/"
      human={{
        kicker: "Hub document",
        title: "Stories config names the project.",
        body: "Statuses, labels, and the story id prefix live here.",
      }}
      machine={{
        kicker: "stories",
        title: "Lives in hub config.json.",
        body: "project_name is required and must be non-empty.",
      }}
    >
      <p className="page-lede">
        Stories section of <Link href="/schema/hub-config/">hub config</Link>.
      </p>
      <ul>
        <li>
          <code>project_name</code> - required, non-empty.
        </li>
        <li>
          <code>statuses</code> - string array.
        </li>
        <li>
          <code>labels</code> - string array.
        </li>
        <li>
          <code>story_prefix</code> - non-empty string when present.
        </li>
        <li>
          <code>date_format</code> - non-empty string when present.
        </li>
        <li>
          <code>default_create_status</code> - string.
        </li>
      </ul>

      <h2>JSON Schema</h2>
      <SchemaBlock file="stories-config.schema.json" />
    </SchemaPage>
  );
}
