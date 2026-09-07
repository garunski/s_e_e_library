import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Routing rules schema",
  "LLM task_profiles and cost_caps. Stored as llm.routing in hub config.json. Validated on save.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/routing-rules/"
      human={{
        kicker: "Hub document",
        title: "Routing maps tasks to models.",
        body: "task_profiles is required. Each profile has provider and model.",
      }}
      machine={{
        kicker: "llm.routing",
        title: "Validators run on save.",
        body: "Lives in hub config.json under llm.routing.",
      }}
    >
      <p className="page-lede">
        LLM routing rules. Stored as <code>llm.routing</code> in{" "}
        <Link href="/schema/hub-config/">hub config</Link>.
      </p>
      <ul>
        <li>
          <code>task_profiles</code> - required object. Each value is{" "}
          <code>{"{ provider, model, requires_tool_calling? }"}</code>.{" "}
          <code>provider</code> and <code>model</code> are required.
        </li>
        <li>
          <code>cost_caps</code> - object. Each value is{" "}
          <code>{"{ max_usd }"}</code>.
        </li>
      </ul>
      <p>At least one task profile is required at save time.</p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="routing-rules.schema.json" />
    </SchemaPage>
  );
}
