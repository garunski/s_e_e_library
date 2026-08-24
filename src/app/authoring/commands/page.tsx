import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Commands",
  "A command package defines how the S.E.E. engine runs an external agent CLI against a spoke workspace.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/commands/"
      human={{
        kicker: "Worker contract",
        title: "The worker is a payload file.",
        body: "command.json names the binary, arguments, parameters, and success rule.",
      }}
      machine={{
        kicker: "Command package",
        title: "Install path is the command id.",
        body: "The catalog to path must equal .s_e_e/commands/{id}.json.",
      }}
    >
      <p className="page-lede">
        A command package defines how the S.E.E. engine runs an external agent
        CLI against a spoke workspace. Payload file: <code>command.json</code>.
        Installs to <code>.s_e_e/commands/{"{id}"}.json</code>.
      </p>

      <h2>Fields</h2>
      <dl className="decision-list compact-list">
        <dt>
          <code>binary</code>
        </dt>
        <dd>
          Executable name on <code>PATH</code> (for example <code>cursor</code>,{" "}
          <code>claude</code>).
        </dd>
        <dt>
          <code>base_args</code>
        </dt>
        <dd>Arguments always passed before operator overrides.</dd>
        <dt>
          <code>forced_args</code>
        </dt>
        <dd>Arguments the engine appends; not overridable.</dd>
        <dt>
          <code>params</code>
        </dt>
        <dd>
          Typed run-time knobs (<code>string</code>, <code>number</code>,{" "}
          <code>boolean</code>, <code>enum</code>, <code>string_array</code>)
          mapped to env, timeout, extra args, lock policy, or stream toggle.
        </dd>
        <dt>
          <code>result</code>
        </dt>
        <dd>
          Success rule (<code>success_when</code>), captured fields, optional{" "}
          <code>applied_paths</code> and <code>run_id</code> strategies.
        </dd>
        <dt>
          <code>execution</code>
        </dt>
        <dd>
          Working directory (<code>cwd</code>: <code>spoke</code> or{" "}
          <code>hub</code>), lock scope, stream format,{" "}
          <code>non_interactive</code>.
        </dd>
      </dl>
      <p>
        Additional fields: <code>id</code>, <code>name</code>,{" "}
        <code>icon</code>, <code>workspace_arg</code>, <code>prompt</code>{" "}
        delivery, <code>env</code>, and optional <code>mcp</code> provisioning.
      </p>

      <h2>Reference example</h2>
      <p>
        From <code>packages/cursor-agent/1.0.0/command.json</code>:
      </p>
      <pre>{`{
  "id": "cursor-agent",
  "name": "Cursor Agent",
  "binary": "cursor",
  "base_args": ["agent", "-p", "--trust", "--force"],
  "forced_args": ["--output-format", "stream-json", "--stream-partial-output"],
  "params": [
    {
      "key": "cursor_model",
      "type": "string",
      "default": "composer-2.5",
      "map": { "target": "env", "name": "CURSOR_MODEL" }
    },
    {
      "key": "timeout_secs",
      "type": "number",
      "default": 1800,
      "map": { "target": "timeout" }
    }
  ],
  "result": {
    "success_when": "exit_code == 0",
    "capture": ["run_id", "exit_code", "duration_ms", "stdout_tail", "stderr_tail", "truncated"]
  },
  "execution": {
    "cwd": "spoke",
    "lock": { "scope": "spoke", "default_policy": "force" },
    "stream": { "format": "stream-json" },
    "non_interactive": true
  }
}`}</pre>

      <h2>Install path</h2>
      <p>
        Catalog <code>files[]</code> entry:
      </p>
      <pre>{`{
  "to": ".s_e_e/commands/cursor-agent.json",
  "from": "packages/cursor-agent/1.0.0/command.json"
}`}</pre>
      <p>
        The <code>to</code> path must equal{" "}
        <code>.s_e_e/commands/{"{id}"}.json</code> where <code>id</code> matches{" "}
        <code>command.json</code> <code>id</code>.
      </p>

      <h2>Schema reference</h2>
      <ul className="example-list">
        <li>
          Catalog manifest:{" "}
          <a href={assetPath("/catalog.json")}>
            <code>see.library/v1</code>
          </a>
        </li>
        <li>
          <Link href="/schema/command/">Command schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
