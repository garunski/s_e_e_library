import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Prompts",
  "A prompt package stores reusable system prompt text referenced by workflows and commands.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/prompts/"
      human={{
        kicker: "Named instruction",
        title: "Prompt text has an identifier.",
        body: "Workflows and commands inject it with the prompt.<id> template form.",
      }}
      machine={{
        kicker: "Prompt package",
        title: "Installs under .s_e_e/prompts.",
        body: "Payload file prompt.json carries id, name, and content.",
      }}
    >
      <p className="page-lede">
        A prompt package stores reusable system prompt text referenced by
        workflows and commands via the <code>{"{{prompt.<id>}}"}</code> template
        form. Payload file: <code>prompt.json</code>. Installs to{" "}
        <code>.s_e_e/prompts/{"{id}"}.json</code>.
      </p>

      <h2>Fields</h2>
      <dl className="decision-list compact-list">
        <dt>
          <code>id</code>
        </dt>
        <dd>
          Stable prompt id (for example <code>system-implement-story</code>).
        </dd>
        <dt>
          <code>name</code>
        </dt>
        <dd>Display name in the hub.</dd>
        <dt>
          <code>content</code>
        </dt>
        <dd>
          Prompt body; may include <code>{"{{runtime.*}}"}</code> placeholders
          expanded at run time.
        </dd>
      </dl>

      <h2>Reference example</h2>
      <p>
        From <code>packages/system-implement-story/1.0.0/prompt.json</code>:
      </p>
      <pre>{`{
  "id": "system-implement-story",
  "name": "Implement story",
  "content": "You are implementing story {{runtime.story_id}}. Operate fully autonomously..."
}`}</pre>
      <p>
        Use a <code>system-*</code> id for bundled defaults. Keep template
        placeholders aligned with the workflow <code>runtime_inputs</code> that
        declare each <code>{"{{runtime.<key>}}"}</code> placeholder.
      </p>

      <h2>Install path</h2>
      <pre>{`{
  "to": ".s_e_e/prompts/system-implement-story.json",
  "from": "packages/system-implement-story/1.0.0/prompt.json"
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
          <Link href="/schema/prompt/">Prompt schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
