import Link from "next/link";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Rule and template schema",
  "Two lightweight, file-only categories: Cursor rule .mdc files and scaffold templates.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/rule-template/"
      human={{
        kicker: "File-only payloads",
        title: "Rules and templates have no JSON envelope.",
        body: "The catalog entry is the contract; the payload is the file itself.",
      }}
      machine={{
        kicker: "Two categories",
        title: "Install prefixes are fixed.",
        body: "Rules go to .cursor/rules. Templates go under templates/.",
      }}
    >
      <p className="page-lede">Two lightweight, file-only categories.</p>

      <h2>Rule</h2>
      <p>
        Category <code>rule</code>. Payload is a Cursor rule <code>.mdc</code>{" "}
        file (markdown with rule frontmatter). Installs to{" "}
        <code>.cursor/rules/{"{slug}"}.mdc</code>.
      </p>

      <h2>Template</h2>
      <p>
        Category <code>template</code>. Payload is an arbitrary scaffold file.
        Installs under <code>templates/</code>.
      </p>

      <p>
        See the <Link href="/schema/">catalog schema</Link> for the manifest and
        install-path rules.
      </p>
    </SchemaPage>
  );
}
