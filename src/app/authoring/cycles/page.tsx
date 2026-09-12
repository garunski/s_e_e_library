import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Cycles",
  "A cycle package describes stages, stores, and triggers for an orchestrator or knowledge host.",
);

const ORCHESTRATOR_STORES = [
  ["world", "Live stories, workflows, and spoke heads (source)."],
  ["objectives", "Authored objectives with measurable criteria."],
  ["deliberations", "Shaping steps per objective."],
  ["ranking", "Candidate order and weights for the next tick."],
  ["proposals", "Gated proposals; at most one per tick."],
  ["executions", "Runs with frozen workflow snapshots."],
  ["learning", "Judged outcomes per finished run."],
];

const KNOWLEDGE_STORES = [
  ["spokes", "Current git on each spoke (source)."],
  ["changes", "Recorded git ranges."],
  ["components", "Notes for named pieces."],
  ["impacts", "Consumer compatibility (gated)."],
  ["audits", "Notes vs live code (gated)."],
  ["docs", "Published topic pages."],
  ["decisions", "ADRs (authored)."],
  ["links", "Broken citations (gated)."],
];

const BLOCKING_CHECKS = [
  [
    "unknown_store",
    "A store key is not declared for this host. Install rejects unknown keys; activation checks repeat the rule.",
  ],
  [
    "store_has_two_writers",
    "Two stages write the same store, so provenance is ambiguous.",
  ],
  [
    "stage_has_no_reads",
    "A stage lists no reads, so it cannot be replayed or judged.",
  ],
  [
    "workflow_not_installed",
    "A stage names a workflow definition id with no file on the hub.",
  ],
  [
    "gate_missing",
    "The orchestrator gate stage was removed.",
  ],
  [
    "gate_rewired",
    "The orchestrator gate stage no longer matches the compiled reads, writes, workflow, or trigger.",
  ],
];

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/cycles/"
      human={{
        kicker: "Host-bound loop",
        title: "Stages move data between named stores.",
        body: "Store keys come from a fixed registry per host, not free-form names.",
      }}
      machine={{
        kicker: "Cycle package",
        title: "Installs under .s_e_e/cycles.",
        body: "Payload file cycle.json; to path must match the document id.",
      }}
    >
      <p className="page-lede">
        A <strong>cycle</strong> package describes how workflows connect stores on
        a host. Payload file: <code>cycle.json</code>. Installs to{" "}
        <code>.s_e_e/cycles/{"{id}"}.json</code> where <code>id</code> matches
        the document <code>id</code> field.
      </p>

      <h2>Stages and stores</h2>
      <p>
        Each <strong>stage</strong> runs one workflow definition. It{" "}
        <code>reads</code> zero or more store keys and <code>writes</code> exactly
        one store key. The <code>stores</code> array lists every key the cycle
        touches; each key must already exist for that host in the app.
      </p>
      <p>
        <strong>One writer per store:</strong> at most one stage may write a given
        store key (orchestrator host). Two writers make it impossible to say which
        stage filled the store.
      </p>

      <h2>Triggers</h2>
      <p>Each stage has exactly one trigger:</p>
      <ul>
        <li>
          <code>tick</code> - runs on the host tick (orchestrator default loop).
        </li>
        <li>
          <code>schedule</code> - runs when a schedule fires (knowledge audit and
          publish stages).
        </li>
        <li>
          <code>event</code> - runs when the host raises an event (knowledge
          refresh after a recorded change).
        </li>
        <li>
          <code>manual</code> - runs only when an operator starts it; nothing will
          schedule it automatically.
        </li>
      </ul>

      <h2>Store registry</h2>
      <p>
        Store keys are <strong>not</strong> free-form. The app ships a fixed
        registry per host. A cycle document may only name keys from that list; a
        key outside the list is rejected on install and reported again at
        activation. If the app later adds a store, this guide and the registry in
        the app must be updated together.
      </p>

      <h3>Orchestrator host</h3>
      <dl className="decision-list compact-list">
        {ORCHESTRATOR_STORES.flatMap(([key, detail]) => [
          <dt key={`${key}-dt`}>
            <code>{key}</code>
          </dt>,
          <dd key={`${key}-dd`}>{detail}</dd>,
        ])}
      </dl>

      <h3>Knowledge host</h3>
      <dl className="decision-list compact-list">
        {KNOWLEDGE_STORES.flatMap(([key, detail]) => [
          <dt key={`${key}-dt`}>
            <code>{key}</code>
          </dt>,
          <dd key={`${key}-dd`}>{detail}</dd>,
        ])}
      </dl>

      <p>
        The JSON Schema allows <code>stories</code> and <code>events</code> as host
        values, but the app does not install or activate cycles for those hosts
        yet.
      </p>

      <h2>The orchestrator gate</h2>
      <p>
        On the orchestrator host, the <code>propose</code> stage is the{" "}
        <strong>gate</strong>. It is compiled into the app: you cannot declare a
        different gate stage, remove it, or change its reads, writes, workflow, or
        trigger. Without that fixed gate, work could run without an approved
        proposal.
      </p>

      <h2>Activation checks</h2>
      <p>
        After install, the app runs cycle document checks before activation.
        <strong> Errors</strong> block activation; warnings do not. Fix errors
        before publishing a cycle you expect operators to turn on:
      </p>
      <dl className="decision-list compact-list">
        {BLOCKING_CHECKS.flatMap(([id, detail]) => [
          <dt key={`${id}-dt`}>
            <code>{id}</code>
          </dt>,
          <dd key={`${id}-dd`}>{detail}</dd>,
        ])}
      </dl>
      <p>
        Warnings (for example a store nothing reads, or a manual stage you never
        run) still appear in the UI but do not block activation.
      </p>

      <h2>Reference example</h2>
      <p>
        From <code>packages/cycle-knowledge-default/1.0.0/cycle.json</code> (seven
        knowledge stages, mixed triggers):
      </p>
      <pre>{`{
  "id": "cycle-knowledge-default",
  "name": "Knowledge default",
  "host": "knowledge",
  "schema_version": 1,
  "stores": ["spokes", "changes", "components", "impacts", "audits", "docs", "decisions", "links"],
  "stages": [
    {
      "key": "record",
      "verb": "Record",
      "workflow_definition_id": "system-record-git-change",
      "trigger": "manual",
      "reads": ["spokes"],
      "writes": "changes"
    }
  ]
}`}</pre>
      <p>
        The full file in the catalog lists every stage. Install workflows the
        stages reference (or depend on a bundle that installs them) before you
        expect activation to succeed.
      </p>

      <h2>Install path</h2>
      <pre>{`{
  "to": ".s_e_e/cycles/cycle-knowledge-default.json",
  "from": "packages/cycle-knowledge-default/1.0.0/cycle.json"
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
          <Link href="/schema/cycle/">Cycle schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
