import { SchemaBlock } from "@/components/SchemaBlock";
import { SchemaPage, innerMetadata } from "@/components/InnerPage";

export const metadata = innerMetadata(
  "Global config schema",
  "User-level ~/.s_e_e/config.json. A JSON object. Validated on save.",
);

export default function Page() {
  return (
    <SchemaPage
      current="/schema/global-config/"
      human={{
        kicker: "Global document",
        title: "Home config is a JSON object.",
        body: "Stored at ~/.s_e_e/config.json. Any object keys are allowed.",
      }}
      machine={{
        kicker: "~/.s_e_e/config.json",
        title: "Validators run on save.",
        body: "The value must be a JSON object, not an array or scalar.",
      }}
    >
      <p className="page-lede">
        User-level <code>~/.s_e_e/config.json</code>. Must be a JSON object.
        Keys are free-form.
      </p>

      <h2>JSON Schema</h2>
      <SchemaBlock file="global-config.schema.json" />
    </SchemaPage>
  );
}
