import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Skills",
  "A skill package teaches an agent a repeatable capability with YAML frontmatter and a markdown body.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/skills/"
      human={{
        kicker: "Repeatable capability",
        title: "A skill teaches a procedure.",
        body: "Frontmatter selects when it activates; the body documents the steps.",
      }}
      machine={{
        kicker: "Skill package",
        title: "Installs under .agents/skills.",
        body: "Payload file SKILL.md. Destination is .agents/skills/{slug}/SKILL.md.",
      }}
    >
      <p className="page-lede">
        A skill package teaches an agent a repeatable capability. Payload file:{" "}
        <code>SKILL.md</code>. Installs to{" "}
        <code>.agents/skills/{"{slug}"}/SKILL.md</code>.
      </p>

      <h2>Shape</h2>
      <p>YAML frontmatter plus markdown body:</p>
      <ul>
        <li>
          <code>name</code>: short skill id (matches the install folder name)
        </li>
        <li>
          <code>description</code>: activation text; when the user says these
          phrases, the agent should load this skill
        </li>
      </ul>
      <p>
        The body documents the procedure, constraints, and links to
        authoritative docs.
      </p>

      <h2>Reference example</h2>
      <p>
        From <code>packages/work/1.0.0/SKILL.md</code>:
      </p>
      <pre>{`---
name: work
description: >-
  Implements an S.E.E. story with tests until acceptance criteria pass...
  Use when the user says work, "start work", "implement story-N", or works
  through story IDs with acceptance criteria.
---

# Work

Implement an S.E.E. story with tests until all acceptance criteria pass...`}</pre>
      <p>
        Write the <code>description</code> as a single activation sentence
        agents can match against user intent. Keep procedures in the markdown
        body, not in frontmatter.
      </p>

      <h2>Install path</h2>
      <pre>{`{
  "to": ".agents/skills/work/SKILL.md",
  "from": "packages/work/1.0.0/SKILL.md"
}`}</pre>

      <h2>Story authoring packages</h2>
      <p>
        The bundled <code>stories</code> skill and <code>system-story-authoring</code>
        prompt carry the S.E.E. hub's accepted decision <code>decision-24</code>.
        Broad stories hide affected surfaces and can make partial work look
        complete. Apply the specificity review whenever a story is created or
        materially revised, before it is handed off, scheduled, or moved into
        active work.
      </p>
      <p>
        Verify the current gap and count affected screens, stored objects,
        application code paths, APIs, runtime components, Library definitions,
        and existing installations. Name each affected path and exact behavior
        in the implementation plan. Split independently deliverable work into
        separate stories. Give each affected surface an observable acceptance
        criterion with evidence beyond the authored change. Complete and read
        back any store-created shell in the same authoring session.
      </p>
      <p>
        When publishing either authoring package, keep this rule in its payload
        so installed agents receive the same guidance. S.E.E. Help, under
        <strong> Writing implementation ready stories</strong>, explains the
        review for people writing or reviewing stories.
      </p>

      <h2>Milestone authoring packages</h2>
      <p>
        The <code>stories</code> skill and <code>system-milestone-authoring</code>
        prompt carry the S.E.E. hub's accepted decision <code>decision-25</code>.
        Apply it when creating or revising a milestone and before declaring it
        Done. A milestone cut names one observable outcome, includes the stories
        necessary to deliver it, and states why other work is deferred.
      </p>
      <p>
        Assign stories through their milestone field and read membership back.
        Add a <code>milestone_progress</code> criterion targeting the milestone
        at 100 percent, then add separate criteria for the combined outcome and
        compatibility behavior. Story acceptance criteria verify individual
        changes; milestone criteria prove the release outcome. Check manual
        criteria only after reviewing their named evidence. If scope changes,
        revise the cut, membership, and criteria before claiming completion.
      </p>
      <p>
        The Done transition requires at least one member story, self progress
        at 100 percent, a separate outcome criterion, and every criterion met.
        The detailed <code>milestone_get</code> result shows each criterion&apos;s
        evaluation. The milestone store cannot observe project execution,
        issue, or spoke quality facts, so a <code>quality_gate</code> criterion
        is unevaluable for milestone Done.
      </p>
      <p>
        Keep this guidance in the published skill and prompt payloads. S.E.E.
        Help, under <strong>Authoring milestones and making the cut</strong>,
        explains the membership and completion review.
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
          <Link href="/schema/skill/">Skill schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
