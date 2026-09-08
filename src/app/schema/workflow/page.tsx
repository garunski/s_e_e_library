import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Workflow schema",
  "Category workflow. Payload file definition.json. Installs to .s_e_e/workflows/definitions/{slug}.json.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/workflow/"
      human={{
        kicker: "Workflow payload",
        title: "An envelope wraps engine content.",
        body: "id, name, and content are required. content is the engine workflow object.",
      }}
      machine={{
        kicker: "Category workflow",
        title: "Installs under workflow definitions.",
        body: "Destination is .s_e_e/workflows/definitions/{slug}.json.",
      }}
    >
      <p className="page-lede">
        Category <code>workflow</code>. Payload file <code>definition.json</code>
        . Installs to <code>.s_e_e/workflows/definitions/{"{slug}"}.json</code>.
      </p>

      <h2>Envelope</h2>
      <p>An outer envelope wraps the engine <code>content</code>:</p>
      <ul>
        <li>
          <code>id</code> - <code>system-{"<name>"}</code> (defaults) or{" "}
          <code>user-{"<name>"}</code>. Required.
        </li>
        <li>
          <code>name</code>, <code>description</code>, <code>version</code> -
          strings.
        </li>
        <li>
          <code>content</code> - the engine workflow object:
          <ul>
            <li>
              <code>id</code> - required, non-empty.
            </li>
            <li>
              <code>name</code> - required.
            </li>
            <li>
              <code>tasks[]</code> - required, non-empty.
            </li>
            <li>
              <code>runtime_inputs[]</code> - optional; each{" "}
              <code>{"{ key, input_type, required, label }"}</code>;{" "}
              <code>key</code> must not contain dots.
            </li>
            <li>
              <code>spoke_root</code>, <code>hub_root</code>,{" "}
              <code>stories_root</code>, <code>knowledge_root</code> - optional
              root paths.
            </li>
          </ul>
        </li>
      </ul>

      <h2>Tasks</h2>
      <p>Each task:</p>
      <ul>
        <li>
          <code>id</code> - unique within the workflow.
        </li>
        <li>
          <code>name</code> - string.
        </li>
        <li>
          <code>function</code> - exactly one handler key:{" "}
          <code>cli_command</code>, <code>command</code>,{" "}
          <code>user_input</code>, <code>git_action</code>,{" "}
          <code>story_action</code>, <code>milestone_action</code>,{" "}
          <code>document_action</code>, <code>decision_action</code>,{" "}
          <code>component_action</code>, <code>change_action</code>,{" "}
          <code>impact_action</code>, <code>audit_action</code>,{" "}
          <code>link_action</code>, <code>foreach</code>, or{" "}
          <code>custom</code>.
        </li>
        <li>
          <code>next_tasks[]</code> - optional nested tasks (depth-first
          execution).
        </li>
        <li>
          <code>capture_output</code> - when true, record handler output for{" "}
          <code>{"{{task.TASK_ID.output}}"}</code> placeholders.
        </li>
        <li>
          <code>wait_for</code> - task ids that must complete before this task
          runs (use a foreach id to wait for all iterations).
        </li>
        <li>
          <code>continue_on_failure</code> - when true, a failed task completes
          without failing the workflow; output is still captured when{" "}
          <code>capture_output</code> is true.
        </li>
        <li>
          <code>run_when</code> - object{" "}
          <code>{"{ task, path?, op, value? }"}</code> gating this task on the
          captured output of a prior task (<code>op</code>: <code>eq</code>,{" "}
          <code>ne</code>, <code>gt</code>, <code>lt</code>, <code>ge</code>,{" "}
          <code>le</code>, <code>exists</code>).
        </li>
      </ul>
      <p>A payload containing <code>tasks</code> is treated as a workflow (not a prompt).</p>

      <h2>Template expansion</h2>
      <pre>{`{{prompt.ID}}              injects a stored prompt; the id must exist in the hub
{{runtime.KEY}}            run-start value; declared in runtime_inputs, no dots
{{userinput.TASK_ID.value}} value from an earlier user_input task
{{task.TASK_ID.output}}    captured output from an earlier task with capture_output: true
                           (cli_command output is { exit_code, stdout, stderr })`}</pre>

      <h2>JSON Schema</h2>
      <p>Library package envelope (id, name, content):</p>
      <SchemaBlock file="workflow.schema.json" />

      <h2>Stored definition envelope</h2>
      <p>
        The hub stores a definition object. <code>content</code> is the engine
        workflow object. <code>version</code> is an integer.
      </p>
      <SchemaBlock file="workflow-definition.schema.json" />

      <h2>Engine content schema</h2>
      <p>
        The object in <code>content</code>: tasks, handlers, runtime inputs, and
        template expansion.
      </p>
      <SchemaBlock file="workflow-engine.schema.json" />

      <p>
        See the <Link href="/authoring/workflows/">workflow authoring guide</Link>{" "}
        and the <Link href="/schema/">catalog schema</Link>.
      </p>
    </SchemaPage>
  );
}
