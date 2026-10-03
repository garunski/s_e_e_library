use std::path::Path;

use serde_json::Value;

pub fn title_case_slug(slug: &str) -> String {
    slug.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn split_skill_frontmatter(content: &str) -> Option<String> {
    let text = content.trim_start().trim_start_matches('\u{feff}');
    if !text.starts_with("---") {
        return None;
    }
    let after_open = text.strip_prefix("---")?.trim_start_matches(['\r', '\n']);
    let close = after_open.find("\n---")?;
    Some(after_open[..close].trim().to_string())
}

fn frontmatter_field(yaml: &str, field: &str) -> String {
    let prefix = format!("{field}:");
    for line in yaml.lines() {
        if line.starts_with(&prefix) {
            let rest = line[prefix.len()..].trim();
            if !rest.is_empty() && !rest.starts_with('>') && !rest.starts_with('-') {
                return rest.to_string();
            }
        }
    }
    if let Some(start) = yaml.find(&prefix) {
        let after = &yaml[start + prefix.len()..];
        let end = after
            .find("\n[a-z_]+:")
            .map(|i| i)
            .unwrap_or(after.len());
        let block = after[..end].trim();
        let block = block.trim_start_matches(['>', '-']).trim();
        if !block.is_empty() {
            return block.to_string();
        }
    }
    String::new()
}

pub fn extract_payload_meta(category: &str, content: &str) -> (String, String) {
    match category {
        "workflow" | "prompt" | "command" => {
            let value: Value = serde_json::from_str(content).unwrap_or(Value::Null);
            let name = value
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let description = value
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            (name, description)
        }
        "cycle" => {
            let value: Value = serde_json::from_str(content).unwrap_or(Value::Null);
            let name = value
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            (name, String::new())
        }
        "skill" => {
            let yaml = split_skill_frontmatter(content);
            match yaml {
                Some(yaml) => (
                    frontmatter_field(&yaml, "name"),
                    frontmatter_field(&yaml, "description"),
                ),
                None => (String::new(), String::new()),
            }
        }
        _ => (String::new(), String::new()),
    }
}

pub fn presentation_for_entry(entry: &Value, root: &Path) -> (String, String) {
    let category = entry
        .get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if category == "bundle" {
        let slug = entry.get("slug").and_then(|v| v.as_str()).unwrap_or("");
        let name = entry
            .get("name")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_else(|| title_case_slug(slug));
        let description = entry
            .get("description")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .unwrap_or("")
            .to_string();
        return (name, description);
    }

    let mut name = entry
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("")
        .to_string();
    let mut description = entry
        .get("description")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("")
        .to_string();

    if let Some(from) = entry
        .get("files")
        .and_then(|v| v.as_array())
        .and_then(|files| files.first())
        .and_then(|f| f.get("from"))
        .and_then(|v| v.as_str())
    {
        let path = root.join(from);
        if let Ok(content) = std::fs::read_to_string(&path) {
            let meta = extract_payload_meta(category, &content);
            if name.is_empty() {
                name = meta.0;
            }
            if description.is_empty() {
                description = meta.1;
            }
        }
    }

    if name.is_empty() {
        name = title_case_slug(entry.get("slug").and_then(|v| v.as_str()).unwrap_or(""));
    }
    (name, description)
}
