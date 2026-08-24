import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Command schema",
  "Category command. Payload file command.json. Installs to .s_e_e/commands/{id}.json.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/command/"
      human={{
        kicker: "Command payload",
        title: "id and binary are required.",
        body: "A payload with content but no binary is treated as a prompt.",
      }}
      machine={{
        kicker: "Category command",
        title: "The to path must match the id.",
        body: "Destination is .s_e_e/commands/{id}.json, exactly.",
      }}
    >
      <p className="page-lede">
        Category <code>command</code>. Payload file <code>command.json</code>.
        Installs to <code>.s_e_e/commands/{"{id}"}.json</code> (the{" "}
        <code>to</code> path must equal this exactly).
      </p>
      <ul>
        <li>
          <code>id</code> - unique command id; also the install filename.
          Required, non-empty.
        </li>
        <li>
          <code>name</code>, <code>icon</code> - display metadata.
        </li>
        <li>
          <code>binary</code> - executable to run. Required, non-empty.
        </li>
        <li>
          <code>base_args</code> - args always passed.
        </li>
        <li>
          <code>forced_args</code> - args the engine appends; operators
          cannot override them.
        </li>
        <li>
          <code>workspace_arg</code> - <code>{"{ flag, value }"}</code> for
          passing the working directory.
        </li>
        <li>
          <code>prompt</code> - prompt delivery (for example{" "}
          <code>{`{ "delivery": "stdin" }`}</code>).
        </li>
        <li>
          <code>params[]</code> - typed parameters; each has <code>key</code>,{" "}
          <code>type</code>, optional <code>default</code>, and{" "}
          <code>map.target</code> (<code>env</code>, <code>timeout</code>,{" "}
          <code>args_append</code>, <code>lock_policy</code>,{" "}
          <code>stream_toggle</code>).
        </li>
        <li>
          <code>env</code> - static environment variables.
        </li>
        <li>
          <code>result</code> - <code>success_when</code> plus{" "}
          <code>capture[]</code>.
        </li>
        <li>
          <code>execution</code> - <code>cwd</code> (<code>spoke</code>/
          <code>hub</code>), <code>lock</code>, <code>stream</code>,{" "}
          <code>non_interactive</code>.
        </li>
        <li>
          <code>mcp</code> - optional MCP provisioning.{" "}
          <code>{"{ enabled, mechanism? }"}</code>. When <code>enabled</code> is
          true and <code>mechanism</code> is omitted, delivery defaults to{" "}
          <code>env</code>. Mechanisms:
          <ul>
            <li>
              <code>{`{ "type": "env" }`}</code> - inject <code>SEE_MCP_*</code>{" "}
              and <code>MCP_CONFIG</code>.
            </li>
            <li>
              <code>{`{ "type": "config_file", "path": ".cursor/mcp.json", "format": "mcp_servers" }`}</code>{" "}
              - write workspace-relative config (restored after the run).
            </li>
            <li>
              <code>{`{ "type": "arg", "flag": "--mcp-config", "format": "mcp_servers" }`}</code>{" "}
              - pass a temp config file via argv.
            </li>
          </ul>
        </li>
      </ul>
      <p>
        A payload with <code>content</code> but no <code>binary</code> is
        treated as a prompt.
      </p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="command.schema.json" />

      <p>
        See the <Link href="/authoring/commands/">command authoring guide</Link> and
        the <Link href="/schema/">catalog schema</Link>.
      </p>
    </SchemaPage>
  );
}
