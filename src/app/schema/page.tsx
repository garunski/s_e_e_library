import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";
import { assetPath } from "@/lib/nav";

export const metadata = innerMetadata(
  "Catalog schema",
  "The catalog is a single JSON manifest plus versioned payload files under packages/. Schema tag see.library/v1.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/"
      human={{
        kicker: "Manifest contract",
        title: "One file lists every package.",
        body: "catalog.json is the install map. packages/ holds the payloads.",
      }}
      machine={{
        kicker: "see.library/v1",
        title: "The build enforces the schema.",
        body: "scripts/build-catalog.mjs checks payloads against this contract.",
      }}
    >
      <p className="page-lede">
        The catalog is a single JSON manifest (<code>catalog.json</code>) plus
        versioned payload files under <code>packages/</code>. The manifest
        schema tag is <code>see.library/v1</code>. This section is the
        authoritative contract for this repository;{" "}
        <code>scripts/build-catalog.mjs</code> enforces it.
      </p>

      <h2>Per-type payload schemas</h2>
      <ul className="example-list named-examples">
        <li>
          <strong>
            <Link href="/schema/workflow/">Workflow</Link>
          </strong>
          <span>definition.json envelope and engine content.</span>
        </li>
        <li>
          <strong>
            <Link href="/schema/prompt/">Prompt</Link>
          </strong>
          <span>id, name, and content.</span>
        </li>
        <li>
          <strong>
            <Link href="/schema/skill/">Skill</Link>
          </strong>
          <span>SKILL.md frontmatter.</span>
        </li>
        <li>
          <strong>
            <Link href="/schema/command/">Command</Link>
          </strong>
          <span>binary, args, params, result, execution.</span>
        </li>
        <li>
          <strong>
            <Link href="/schema/rule-template/">Rule and template</Link>
          </strong>
          <span>Lightweight file-only categories.</span>
        </li>
        <li>
          <strong>
            <Link href="/schema/bundle/">Bundle</Link>
          </strong>
          <span>Install map with no payload of its own.</span>
        </li>
      </ul>

      <h2>Manifest</h2>
      <p>
        <code>catalog.json</code>:
      </p>
      <ul>
        <li>
          <code>schema</code> - must be <code>see.library/v1</code>.
        </li>
        <li>
          <code>name</code> - catalog display name.
        </li>
        <li>
          <code>updated</code> - ISO-8601 timestamp (refreshed by{" "}
          <code>npm run catalog</code>).
        </li>
        <li>
          <code>packages[]</code> - array of package entries.
        </li>
      </ul>

      <h2>Package entry</h2>
      <p>
        Each entry in <code>packages[]</code>:
      </p>
      <ul>
        <li>
          <code>id</code> - string, globally unique within the catalog.
        </li>
        <li>
          <code>slug</code> - string, url/file-safe.
        </li>
        <li>
          <code>category</code> - one of <code>workflow</code>,{" "}
          <code>prompt</code>, <code>skill</code>, <code>rule</code>,{" "}
          <code>template</code>, <code>command</code>, <code>bundle</code>.
        </li>
        <li>
          <code>version</code> - semver string.
        </li>
        <li>
          <code>author</code> - string.
        </li>
        <li>
          <code>verified</code> - bool.
        </li>
        <li>
          <code>license</code> - SPDX id (for example <code>AGPL-3.0-only</code>
          ).
        </li>
        <li>
          <code>requires</code> - string array of external tools that must be
          present (empty when none).
        </li>
        <li>
          <code>dependencies</code> - string array of other package{" "}
          <code>id</code>s installed alongside this one.
        </li>
        <li>
          <code>files[]</code> - install map; each entry is{" "}
          <code>{"{ to, from }"}</code>:
          <ul>
            <li>
              <code>from</code> - source path in this repo,{" "}
              <code>packages/{"{slug}"}/{"{version}"}/{"{file}"}</code>.
            </li>
            <li>
              <code>to</code> - destination path written on the install target,
              relative to the target root.
            </li>
          </ul>
        </li>
      </ul>

      <h2>Package metadata sidecar</h2>
      <p>
        Every package carries a <code>packages/{"{slug}"}/s_e_e_package.json</code>{" "}
        (or <code>s_e_e_package.{"{id}"}.json</code> when a slug has multiple
        entries); the build cross-checks it against the catalog entry:
      </p>
      <ul>
        <li>
          <code>id</code> - package id; matches the catalog entry.
        </li>
        <li>
          <code>slug</code> - url/file-safe slug.
        </li>
        <li>
          <code>category</code> - one of the category values above.
        </li>
        <li>
          <code>name</code> - display name.
        </li>
        <li>
          <code>description</code> - one-line summary.
        </li>
        <li>
          <code>labels</code> - string array (the build dedupes and sorts it).
        </li>
        <li>
          <code>dependencies</code> - string array of other package{" "}
          <code>id</code>s.
        </li>
      </ul>

      <h2>Categories</h2>
      <p>Each category has a fixed payload shape and install destination:</p>
      <ul>
        <li>
          <code>workflow</code> - a workflow definition JSON. Installs to{" "}
          <code>.s_e_e/workflows/definitions/{"{slug}"}.json</code>.
        </li>
        <li>
          <code>prompt</code> - a prompt JSON (<code>id</code>, <code>name</code>
          , <code>content</code>). Installs to{" "}
          <code>.s_e_e/prompts/{"{file}"}.json</code>.
        </li>
        <li>
          <code>skill</code> - a <code>SKILL.md</code> with <code>name</code> +{" "}
          <code>description</code> frontmatter. Installs to{" "}
          <code>.agents/skills/{"{slug}"}/SKILL.md</code>.
        </li>
        <li>
          <code>rule</code> - a Cursor rule <code>.mdc</code> file. Installs to{" "}
          <code>.cursor/rules/{"{slug}"}.mdc</code>.
        </li>
        <li>
          <code>template</code> - a scaffold file. Installs under{" "}
          <code>templates/</code>.
        </li>
        <li>
          <code>command</code> - a command definition JSON (<code>id</code>,{" "}
          <code>binary</code>). Installs to{" "}
          <code>.s_e_e/commands/{"{id}"}.json</code> (the <code>to</code> path
          must equal this exactly).
        </li>
        <li>
          <code>bundle</code> - no payload of its own; its <code>files[]</code>{" "}
          install several other packages together. The build infers each
          file&apos;s kind from its <code>to</code> path.
        </li>
      </ul>

      <h2 id="install-paths">Install paths</h2>
      <p>
        Every <code>files[]</code> <code>to</code> path must:
      </p>
      <ul>
        <li>
          start with one of <code>.s_e_e/</code>, <code>.agents/</code>,{" "}
          <code>.cursor/</code>, or <code>templates/</code>;
        </li>
        <li>use forward slashes;</li>
        <li>be unique within the package.</li>
      </ul>
      <p>
        For a <code>bundle</code>, the <code>to</code> prefix determines the
        payload kind that is validated:
      </p>
      <ul>
        <li>
          <code>.s_e_e/workflows/definitions/…</code> → workflow
        </li>
        <li>
          <code>.s_e_e/prompts/…</code> → prompt
        </li>
        <li>
          <code>.s_e_e/commands/…</code> → command
        </li>
        <li>
          <code>.agents/skills/…/SKILL.md</code> → skill
        </li>
        <li>
          <code>*.mdc</code> → rule
        </li>
        <li>
          <code>templates/…</code> → template
        </li>
      </ul>

      <h2>Validation</h2>
      <p>
        <code>dependencies</code> must reference package <code>id</code>s
        present in the catalog, and a package cannot depend on itself. Run{" "}
        <code>npm run catalog</code> to validate payloads and refresh the
        manifest, or <code>npm run validate</code> (<code>--check</code>, no
        writes) in CI.
      </p>

      <h2>JSON Schema</h2>
      <p>
        Live catalog:{" "}
        <a href={assetPath("/catalog.json")}>
          <code>catalog.json</code>
        </a>
        .
      </p>
      <SchemaBlock file="catalog.schema.json" />

      <h2>Package metadata sidecar schema</h2>
      <SchemaBlock file="package-meta.schema.json" />
    </SchemaPage>
  );
}
