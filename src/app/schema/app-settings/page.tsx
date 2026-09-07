import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "App settings schema",
  "User-level ~/.s_e_e/settings.json. Theme, auth, retention, and command allowlist. Validated on save.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/app-settings/"
      human={{
        kicker: "App document",
        title: "settings.json holds operator preferences.",
        body: "Theme, auth, retention, MCP, and the command binary allowlist.",
      }}
      machine={{
        kicker: "~/.s_e_e/settings.json",
        title: "Validators run on save.",
        body: "theme is light, dark, or system. Extra keys are kept.",
      }}
    >
      <p className="page-lede">
        User-level <code>~/.s_e_e/settings.json</code>.
      </p>
      <ul>
        <li>
          <code>theme</code> - <code>light</code>, <code>dark</code>, or{" "}
          <code>system</code>.
        </li>
        <li>
          <code>auto_save</code>, <code>notifications</code>,{" "}
          <code>auth_enabled</code>, <code>mcp_enabled</code>,{" "}
          <code>llm_content_capture_enabled</code> - booleans.
        </li>
        <li>
          <code>default_workflow</code> - string or null.
        </li>
        <li>
          <code>server_url</code>, <code>api_port</code>,{" "}
          <code>cors_origins</code> - strings.
        </li>
        <li>
          <code>audit_retention_hours</code>, <code>log_retention_hours</code>,{" "}
          <code>session_ttl_hours</code>, <code>command_timeout_secs</code> -
          integer or null.
        </li>
        <li>
          <code>schedule_retention</code> - object.
        </li>
        <li>
          <code>command_binary_allowlist</code> - string array.
        </li>
        <li>
          <code>command_secrets</code> - string map.
        </li>
      </ul>

      <h2>JSON Schema</h2>
      <SchemaBlock file="app-settings.schema.json" />
    </SchemaPage>
  );
}
