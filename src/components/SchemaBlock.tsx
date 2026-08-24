import { readFileSync } from "node:fs";
import { join } from "node:path";
import { assetPath } from "@/lib/nav";

type SchemaBlockProps = {
  file: string;
};

export function SchemaBlock({ file }: SchemaBlockProps) {
  const json = readFileSync(
    join(process.cwd(), "public", "schema", file),
    "utf8",
  );
  return (
    <>
      <div className="external-actions">
        <a className="button button-dark" href={assetPath(`/schema/${file}`)}>
          Download {file}
        </a>
      </div>
      <pre>{json}</pre>
    </>
  );
}
