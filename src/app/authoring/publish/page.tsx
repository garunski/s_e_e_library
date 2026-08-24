import Link from "next/link";
import { AuthoringPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Publish to the catalog",
  "Add a package payload to this repository, update catalog.json, and validate before opening a pull request.",
);

export default function Page() {
  return (
    <AuthoringPage
      current="/authoring/publish/"
      human={{
        kicker: "Release path",
        title: "Publishing is a catalog entry.",
        body: "The payload lives under packages/; catalog.json is the install map.",
      }}
      machine={{
        kicker: "Catalog entry",
        title: "Validation gates the entry.",
        body: "npm run validate fails on drift. npm run catalog refreshes updated.",
      }}
    >
      <p className="page-lede">
        After authoring a package payload, add it to this repository and
        validate before opening a pull request.
      </p>

      <h2>Payload layout</h2>
      <p>Place files under:</p>
      <pre>{`packages/{slug}/{version}/`}</pre>
      <p>Examples:</p>
      <ul>
        <li>
          <code>packages/cursor-agent/1.0.0/command.json</code>
        </li>
        <li>
          <code>packages/system-implement-story/1.0.0/prompt.json</code>
        </li>
        <li>
          <code>packages/work/1.0.0/SKILL.md</code>
        </li>
        <li>
          <code>
            packages/implement-story/1.0.0/system-implement-story/definition.json
          </code>
        </li>
      </ul>
      <p>
        Add <code>packages/{"{slug}"}/s_e_e_package.json</code> metadata for
        every catalog entry.
      </p>
      <p>
        Bundles copy member payloads under the bundle slug folder, for example{" "}
        <code>packages/implement-story/1.0.0/work/SKILL.md</code>.
      </p>

      <h2>catalog.json entry</h2>
      <p>
        Append a package object to <code>catalog.json</code>{" "}
        <code>packages[]</code>:
      </p>
      <dl className="decision-list compact-list">
        <dt>
          <code>id</code>
        </dt>
        <dd>
          Globally unique id (for example <code>cmd-cursor-agent</code>,{" "}
          <code>bundle-implement-story</code>).
        </dd>
        <dt>
          <code>slug</code>
        </dt>
        <dd>
          URL-safe slug matching the <code>packages/</code> folder.
        </dd>
        <dt>
          <code>category</code>
        </dt>
        <dd>
          <code>workflow</code>, <code>prompt</code>, <code>skill</code>,{" "}
          <code>command</code>, or <code>bundle</code>.
        </dd>
        <dt>
          <code>version</code>
        </dt>
        <dd>
          Semver string (for example <code>1.0.0</code>).
        </dd>
        <dt>
          <code>author</code>
        </dt>
        <dd>Publisher id.</dd>
        <dt>
          <code>verified</code>
        </dt>
        <dd>
          <code>true</code> when signed off by a maintainer.
        </dd>
        <dt>
          <code>license</code>
        </dt>
        <dd>
          SPDX id (for example <code>AGPL-3.0-only</code>).
        </dd>
        <dt>
          <code>requires</code>
        </dt>
        <dd>External tools required at install time (empty array when none).</dd>
        <dt>
          <code>dependencies</code>
        </dt>
        <dd>
          Other package <code>id</code>s installed alongside this one.
        </dd>
        <dt>
          <code>files[]</code>
        </dt>
        <dd>
          Install map with <code>to</code> and <code>from</code> paths.
        </dd>
      </dl>
      <p>
        Optional presentation fields: <code>name</code>,{" "}
        <code>description</code>, <code>labels</code>.
      </p>
      <p>Example command entry:</p>
      <pre>{`{
  "id": "cmd-cursor-agent",
  "slug": "cursor-agent",
  "category": "command",
  "version": "1.0.0",
  "author": "garunski",
  "verified": true,
  "license": "AGPL-3.0-only",
  "requires": [],
  "dependencies": [],
  "files": [
    {
      "to": ".s_e_e/commands/cursor-agent.json",
      "from": "packages/cursor-agent/1.0.0/command.json"
    }
  ],
  "name": "Cursor Agent",
  "description": "Cursor CLI agent run against a spoke workspace."
}`}</pre>

      <h2>Validate and build</h2>
      <pre>{`npm run validate
npm run catalog`}</pre>
      <ul>
        <li>
          <code>npm run validate</code> runs{" "}
          <code>node scripts/build-catalog.mjs --check</code> (read-only; fails
          when payloads or metadata are invalid or out of sync).
        </li>
        <li>
          <code>npm run catalog</code> validates payloads, refreshes{" "}
          <code>catalog.json</code> <code>updated</code>, and writes any derived
          fields.
        </li>
      </ul>
      <p>
        Commit both the payload under <code>packages/</code> and the updated{" "}
        <code>catalog.json</code>.
      </p>

      <h2>Docs site</h2>
      <pre>{`npm run copy
npm run build
npm test`}</pre>
      <p>
        <code>npm run copy</code> copies root <code>catalog.json</code> and{" "}
        <code>packages/</code> into <code>public/</code> so Next.js serves them
        at the published URLs. <code>npm run build</code> runs that copy, then
        exports the static site.
      </p>
      <p>
        A new package type also needs a section in <code>public/llms.txt</code>,
        which carries the same guidance as one plain-text file for agents.{" "}
        <code>npm test</code> fails when a page has no matching{" "}
        <code>Source:</code> line, so both change together.
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
          <Link href="/schema/">Catalog schema</Link>
        </li>
      </ul>
    </AuthoringPage>
  );
}
