import { readFileSync } from "node:fs";

export function titleCaseSlug(slug) {
  return slug
    .split("-")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

function splitSkillFrontmatter(content) {
  const text = content.trimStart().replace(/^\uFEFF/, "");
  if (!text.startsWith("---")) {
    return null;
  }
  const afterOpen = text.slice(3).replace(/^\r?\n/, "");
  const close = afterOpen.indexOf("\n---");
  if (close < 0) {
    return null;
  }
  return afterOpen.slice(0, close).trim();
}

function frontmatterField(yaml, field) {
  const folded = yaml.match(
    new RegExp(`^${field}:\\s*>?-?\\s*([\\s\\S]*?)(?:\\n[a-z_]+:|$)`, "m")
  );
  if (folded?.[1]?.trim()) {
    return folded[1].trim();
  }
  const plain = yaml.match(new RegExp(`^${field}:\\s*(.+)$`, "m"));
  return plain?.[1]?.trim() ?? "";
}

export function extractPayloadMeta(category, content) {
  switch (category) {
    case "workflow":
    case "prompt":
    case "command": {
      let value;
      try {
        value = JSON.parse(content);
      } catch {
        return { name: "", description: "" };
      }
      return {
        name: value?.name?.trim() ?? "",
        description: value?.description?.trim() ?? "",
      };
    }
    case "skill": {
      const yaml = splitSkillFrontmatter(content);
      if (!yaml) {
        return { name: "", description: "" };
      }
      return {
        name: frontmatterField(yaml, "name"),
        description: frontmatterField(yaml, "description"),
      };
    }
    default:
      return { name: "", description: "" };
  }
}

export function presentationForEntry(entry, root, read = readFileSync) {
  if (entry.category === "bundle") {
    const name = entry.name?.trim() || titleCaseSlug(entry.slug);
    const description = entry.description?.trim() ?? "";
    return { name, description };
  }
  const file = entry.files?.[0];
  let name = entry.name?.trim() ?? "";
  let description = entry.description?.trim() ?? "";
  if (file?.from) {
    try {
      const content = read(`${root}/${file.from}`, "utf8");
      const meta = extractPayloadMeta(entry.category, content);
      if (!name) {
        name = meta.name;
      }
      if (!description) {
        description = meta.description;
      }
    } catch {
    }
  }
  if (!name) {
    name = titleCaseSlug(entry.slug);
  }
  return { name, description };
}
