import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Prompt schema",
  "Category prompt. Payload file prompt.json. Installs to .s_e_e/prompts/{file}.json.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/prompt/"
      human={{
        kicker: "Prompt payload",
        title: "id and name are required.",
        body: "content carries the prompt text. A payload with tasks is a workflow instead.",
      }}
      machine={{
        kicker: "Category prompt",
        title: "Installs under .s_e_e/prompts.",
        body: "Destination is .s_e_e/prompts/{file}.json.",
      }}
    >
      <p className="page-lede">
        Category <code>prompt</code>. Payload file <code>prompt.json</code>.
        Installs to <code>.s_e_e/prompts/{"{file}"}.json</code>.
      </p>
      <ul>
        <li>
          <code>id</code> - prompt id used by <code>{"{{prompt.ID}}"}</code>.
          Required, non-empty; may carry a namespace prefix.
        </li>
        <li>
          <code>name</code> - display name. Required, non-empty.
        </li>
        <li>
          <code>content</code> - prompt body; a single string (use{" "}
          <code>{"\\n"}</code> for line breaks).
        </li>
        <li>
          <code>created_at</code> - ISO-8601 timestamp. Optional.
        </li>
      </ul>
      <p>
        A payload that contains <code>tasks</code> is treated as a workflow, not
        a prompt.
      </p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="prompt.schema.json" />

      <p>
        See the <Link href="/authoring/prompts/">prompt authoring guide</Link> and
        the <Link href="/schema/">catalog schema</Link>.
      </p>
    </SchemaPage>
  );
}
