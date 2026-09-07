import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Workflows",
  "A workflow package defines tasks the S.E.E. engine runs against a hub or spoke.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/workflows/"
      human={{
        kicker: "Execution path",
        title: "The path is a definition file.",
        body: "definition.json wraps the engine content the catalog installs.",
      }}
      machine={{
        kicker: "Workflow package",
        title: "Installs under workflow definitions.",
        body: "Destination is .s_e_e/workflows/definitions/{slug}.json.",
      }}
    >
      <p className="page-lede">
        A workflow package defines tasks the S.E.E. engine runs against a hub or
        spoke. Payload file: <code>definition.json</code>. Installs to{" "}
        <code>.s_e_e/workflows/definitions/{"{slug}"}.json</code>.
      </p>
      <p>
        Authoring rules live in the bundled <strong>workflow</strong> skill (
        <code>packages/workflow/1.0.0/SKILL.md</code>). This page covers the
        library payload shape only.
      </p>

      <h2>Envelope (definition.json)</h2>
      <dl className="decision-list compact-list">
        <dt>
          <code>id</code>
        </dt>
        <dd>
          Outer id, usually <code>system-{"<name>"}</code>.
        </dd>
        <dt>
          <code>name</code>, <code>description</code>
        </dt>
        <dd>Hub display metadata.</dd>
        <dt>
          <code>content</code>
        </dt>
        <dd>Engine workflow object.</dd>
      </dl>

      <h2>Inner content</h2>
      <dl className="decision-list compact-list">
        <dt>
          <code>runtime_inputs</code>
        </dt>
        <dd>
          Run-start values; each object with <code>key</code>,{" "}
          <code>input_type</code>, <code>required</code>, <code>label</code>;
          keys must not contain dots.
        </dd>
        <dt>
          <code>tasks</code>
        </dt>
        <dd>Ordered graph of engine tasks.</dd>
        <dt>
          <code>spoke_root</code>
        </dt>
        <dd>
          Required when command tasks use spoke cwd or git actions omit per-task
          spoke.
        </dd>
      </dl>

      <h3>Task types</h3>
      <p>
        Each task has exactly one handler under <code>function</code>:
      </p>
      <ul>
        <li>
          <code>cli_command</code>: shell command and args
        </li>
        <li>
          <code>command</code>: run a catalog command by{" "}
          <code>command_id</code> with optional <code>prompt</code>
        </li>
        <li>
          <code>user_input</code>: collect operator input at run start
        </li>
        <li>
          <code>git_action</code>, <code>story_action</code>,{" "}
          <code>foreach</code>, <code>custom</code>
        </li>
      </ul>

      <h3>Template references</h3>
      <ul>
        <li>
          <code>{"{{prompt.<id>}}"}</code>: inject stored prompt text
        </li>
        <li>
          <code>{"{{runtime.<key>}}"}</code>: value from{" "}
          <code>runtime_inputs</code>
        </li>
        <li>
          <code>{"{{userinput.<task_id>.value}}"}</code>: earlier{" "}
          <code>user_input</code> task
        </li>
        <li>
          <code>{"{{task.<task_id>.output...}}"}</code>: captured output from an
          earlier task
        </li>
      </ul>

      <h2>Reference example</h2>
      <p>
        From{" "}
        <code>
          packages/implement-story/1.0.0/system-implement-story/definition.json
        </code>{" "}
        (inner <code>content</code> pretty-printed):
      </p>
      <pre>{`{
  "id": "system-implement-story",
  "name": "Implement story",
  "description": "Runs a Cursor CLI agent to implement a single story.",
  "content": {
    "id": "implement-story",
    "name": "Implement story",
    "runtime_inputs": [
      {
        "input_type": "string",
        "key": "story_id",
        "label": "Story id",
        "required": true
      }
    ],
    "tasks": [
      {
        "id": "implement_story",
        "name": "Implement story (Cursor CLI)",
        "function": {
          "name": "command",
          "input": {
            "command_id": "cursor-agent",
            "config": { "cursor_model": "composer-2.5" },
            "prompt": "{{prompt.system-implement-story}}"
          }
        },
        "next_tasks": [
          {
            "id": "report_done",
            "name": "Report completion",
            "function": {
              "name": "cli_command",
              "input": {
                "command": "echo",
                "args": [
                  "implement-story finished for {{runtime.story_id}}"
                ]
              }
            },
            "next_tasks": []
          }
        ]
      }
    ]
  }
}`}</pre>
      <p>
        On disk <code>content</code> is a nested JSON object, not a string.
      </p>

      <h2>Install path</h2>
      <pre>{`{
  "to": ".s_e_e/workflows/definitions/system-implement-story.json",
  "from": "packages/implement-story/1.0.0/system-implement-story/definition.json"
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
          <Link href="/schema/workflow/">Workflow schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
