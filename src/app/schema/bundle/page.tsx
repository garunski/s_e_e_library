import Link from "next/link";
import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Bundle schema",
  "Category bundle. No payload of its own. catalog files[] installs member payloads together.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/bundle/"
      human={{
        kicker: "Bundle payload",
        title: "There is no bundle payload file.",
        body: "files[] installs member payloads. Kind is inferred from the to prefix.",
      }}
      machine={{
        kicker: "Category bundle",
        title: "Metadata lives in the sidecar.",
        body: "s_e_e_package.json carries id, slug, category, name, and dependencies.",
      }}
    >
      <p className="page-lede">
        Category <code>bundle</code>. No payload of its own.
      </p>
      <p>
        A bundle&apos;s catalog <code>files[]</code> array installs the payloads
        of several other packages together; the build infers each file&apos;s
        kind from its <code>to</code> prefix (see{" "}
        <Link href="/schema/#install-paths">Install paths</Link>).
      </p>
      <p>
        Bundle metadata lives in its <code>s_e_e_package.json</code>:
      </p>
      <ul>
        <li>
          <code>id</code> - bundle id (for example{" "}
          <code>bundle-author-story</code>).
        </li>
        <li>
          <code>slug</code> - url/file-safe slug.
        </li>
        <li>
          <code>category</code> - <code>bundle</code>.
        </li>
        <li>
          <code>name</code> - display name.
        </li>
        <li>
          <code>description</code> - one-line summary.
        </li>
        <li>
          <code>labels</code> - string array.
        </li>
        <li>
          <code>dependencies</code> - string array of the package{" "}
          <code>id</code>s it pulls in.
        </li>
      </ul>

      <h2>JSON Schema</h2>
      <p>
        Schema for a bundle&apos;s <code>s_e_e_package.json</code>.
      </p>
      <SchemaBlock file="bundle-package.schema.json" />

      <p>
        The bundle&apos;s <code>files[]</code> entries follow the{" "}
        <code>file</code> definition in the{" "}
        <Link href="/schema/">catalog schema</Link>.
      </p>
      <p>
        See the <Link href="/authoring/bundles/">bundle authoring guide</Link> and
        the <Link href="/schema/">catalog schema</Link>.
      </p>
    </SchemaPage>
  );
}
