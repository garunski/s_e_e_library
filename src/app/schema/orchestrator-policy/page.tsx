import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Orchestrator policy schema",
  "auto_mode, policy.levels, and budgets. Stored as orchestrator in hub config.json. Validated on save.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/orchestrator-policy/"
      human={{
        kicker: "Hub document",
        title: "Policy and budgets gate autonomy.",
        body: "auto_mode is off, assisted, or auto. levels map actions to auto, propose, or forbid.",
      }}
      machine={{
        kicker: "orchestrator",
        title: "Validators run on save.",
        body: "policy and budgets are required. Lives in hub config.json.",
      }}
    >
      <p className="page-lede">
        Orchestrator policy. Stored as <code>orchestrator</code> in{" "}
        <Link href="/schema/hub-config/">hub config</Link>.
      </p>
      <ul>
        <li>
          <code>auto_mode</code> - <code>off</code>, <code>assisted</code>, or{" "}
          <code>auto</code>.
        </li>
        <li>
          <code>policy</code> - required. <code>levels</code> maps action
          classes to <code>auto</code>, <code>propose</code>, or{" "}
          <code>forbid</code>.
        </li>
        <li>
          <code>budgets</code> - required object (run caps, spend caps, window).
        </li>
      </ul>

      <h2>JSON Schema</h2>
      <SchemaBlock file="orchestrator-policy.schema.json" />
    </SchemaPage>
  );
}
