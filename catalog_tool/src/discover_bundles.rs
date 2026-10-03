use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::payload_meta::title_case_slug;

const VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleDef {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub members: Vec<String>,
    pub labels: Vec<String>,
}

fn infer_meta_from_payload(root: &Path, slug: &str) -> Option<Value> {
    let version_dir = root.join("packages").join(slug).join(VERSION);
    if !version_dir.is_dir() {
        return None;
    }
    let def = version_dir.join("definition.json");
    if def.is_file() {
        let parsed: Value =
            serde_json::from_str(&std::fs::read_to_string(&def).ok()?).ok()?;
        return Some(serde_json::json!({
            "id": format!("wf-{slug}"),
            "slug": slug,
            "category": "workflow",
            "name": parsed.get("name").and_then(|v| v.as_str()).unwrap_or(slug),
            "labels": parsed.get("labels").cloned().unwrap_or(Value::Array(vec![])),
        }));
    }
    let prompt = version_dir.join("prompt.json");
    if prompt.is_file() {
        let parsed: Value =
            serde_json::from_str(&std::fs::read_to_string(&prompt).ok()?).ok()?;
        let id = parsed
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| format!("prompt-{s}"))
            .unwrap_or_else(|| format!("prompt-{slug}"));
        return Some(serde_json::json!({
            "id": id,
            "slug": slug,
            "category": "prompt",
            "name": parsed.get("name").and_then(|v| v.as_str()).unwrap_or(slug),
            "labels": [],
        }));
    }
    let command = version_dir.join("command.json");
    if command.is_file() {
        let parsed: Value =
            serde_json::from_str(&std::fs::read_to_string(&command).ok()?).ok()?;
        let id = parsed
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| format!("cmd-{s}"))
            .unwrap_or_else(|| format!("cmd-{slug}"));
        return Some(serde_json::json!({
            "id": id,
            "slug": slug,
            "category": "command",
            "name": parsed.get("name").and_then(|v| v.as_str()).unwrap_or(slug),
            "labels": [],
        }));
    }
    let skill = version_dir.join("SKILL.md");
    if skill.is_file() {
        return Some(serde_json::json!({
            "id": format!("skill-{slug}"),
            "slug": slug,
            "category": "skill",
            "name": slug,
            "labels": [],
        }));
    }
    None
}

fn read_package_metas(root: &Path) -> Result<HashMap<String, Value>, String> {
    let packages = root.join("packages");
    if !packages.is_dir() {
        return Ok(HashMap::new());
    }
    let mut by_id = HashMap::new();
    for slug_entry in std::fs::read_dir(&packages).map_err(|e| e.to_string())? {
        let slug_entry = slug_entry.map_err(|e| e.to_string())?;
        if !slug_entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            continue;
        }
        let slug = slug_entry.file_name().to_string_lossy().to_string();
        let dir = slug_entry.path();
        let mut found_meta = false;
        for name_entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())? {
            let name_entry = name_entry.map_err(|e| e.to_string())?;
            let name = name_entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("s_e_e_package") || !name.ends_with(".json") {
                continue;
            }
            let raw = std::fs::read_to_string(name_entry.path()).map_err(|e| e.to_string())?;
            let meta: Value =
                serde_json::from_str(&raw).map_err(|e| format!("invalid {name}: {e}"))?;
            if meta.get("category").and_then(|v| v.as_str()) == Some("bundle") {
                continue;
            }
            let id = meta
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let mut with_slug = meta;
            if let Some(obj) = with_slug.as_object_mut() {
                obj.insert("slug".to_string(), Value::String(slug.clone()));
            }
            by_id.insert(id, with_slug);
            found_meta = true;
        }
        if !found_meta {
            if let Some(inferred) = infer_meta_from_payload(root, &slug) {
                let id = inferred
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                by_id.insert(id, inferred);
            }
        }
    }
    Ok(by_id)
}

fn payload_paths(root: &Path, slug: &str) -> Vec<PathBuf> {
    let version_dir = root.join("packages").join(slug).join(VERSION);
    if !version_dir.is_dir() {
        return Vec::new();
    }
    std::fs::read_dir(&version_dir)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_file())
                .collect()
        })
        .unwrap_or_default()
}

fn strong_refs_from_text(text: &str) -> HashSet<String> {
    let mut refs = HashSet::new();
    let mut i = 0;
    while let Some(start) = text[i..].find("{{prompt.") {
        let abs = i + start + "{{prompt.".len();
        if let Some(end) = text[abs..].find("}}") {
            let stem = text[abs..abs + end].trim();
            if !stem.is_empty()
                && stem
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                refs.insert(format!("prompt-{stem}"));
            }
            i = abs + end + 2;
        } else {
            break;
        }
    }
    refs
}

fn weak_refs_from_text(text: &str) -> HashSet<String> {
    let mut refs = HashSet::new();
    let mut search = 0;
    while let Some(idx) = text[search..].find("command_id") {
        let base = search + idx;
        let tail = &text[base..];
        let after_key = tail.strip_prefix("command_id").unwrap_or("");
        let after_key = after_key.trim_start();
        let after_key = after_key.strip_prefix(':').unwrap_or(after_key).trim_start();
        let after_key = after_key
            .strip_prefix('"')
            .or_else(|| after_key.strip_prefix('\\'))
            .unwrap_or(after_key);
        let after_key = after_key.trim_start_matches('"').trim_start();
        if let Some(end) = after_key.find('"') {
            let id = after_key[..end].trim();
            if !id.is_empty()
                && id
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                refs.insert(format!("cmd-{id}"));
            }
        }
        search = base + "command_id".len();
    }
    search = 0;
    while let Some(idx) = text[search..].find(".agents/skills/") {
        let base = search + idx + ".agents/skills/".len();
        let tail = &text[base..];
        if let Some(end) = tail.find("/SKILL.md") {
            let slug = tail[..end].trim();
            if !slug.is_empty()
                && slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                refs.insert(format!("skill-{slug}"));
            }
            search = base + end + "/SKILL.md".len();
        } else {
            break;
        }
    }
    refs
}

fn refs_for_package(
    root: &Path,
    meta: &Value,
    by_id: &HashMap<String, Value>,
) -> (HashSet<String>, HashSet<String>) {
    let slug = meta.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    let id = meta.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let paths = payload_paths(root, slug);
    let mut strong = HashSet::new();
    let mut weak = HashSet::new();
    for path in paths {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let is_workflow = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == "definition.json")
            .unwrap_or(false);
        if is_workflow {
            for r in strong_refs_from_text(&text) {
                if r != id && by_id.contains_key(&r) {
                    strong.insert(r);
                }
            }
        }
        for r in weak_refs_from_text(&text) {
            if r != id && by_id.contains_key(&r) {
                weak.insert(r);
            }
        }
    }
    (strong, weak)
}

fn connected_components(
    by_id: &HashMap<String, Value>,
    strong_edges: &HashMap<String, HashSet<String>>,
) -> Vec<Vec<String>> {
    let mut parent: HashMap<String, String> = by_id.keys().map(|id| (id.clone(), id.clone())).collect();

    fn find(parent: &mut HashMap<String, String>, id: &str) -> String {
        let mut root = id.to_string();
        while parent.get(&root).map(|p| p.as_str()) != Some(root.as_str()) {
            root = parent.get(&root).cloned().unwrap_or(root);
        }
        let mut cur = id.to_string();
        while parent.get(&cur).map(|p| p.as_str()) != Some(root.as_str()) {
            let next = parent.get(&cur).cloned().unwrap_or(cur.clone());
            parent.insert(cur.clone(), root.clone());
            cur = next;
        }
        root
    }

    fn unite(parent: &mut HashMap<String, String>, a: &str, b: &str) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            parent.insert(rb, ra);
        }
    }

    for (id, refs) in strong_edges {
        for r in refs {
            unite(&mut parent, id, r);
        }
    }

    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for id in by_id.keys() {
        let root = find(&mut parent, id);
        groups.entry(root).or_default().push(id.clone());
    }
    groups
        .into_values()
        .filter(|g| g.len() > 1)
        .collect()
}

fn bundle_slug_for_group(ids: &[String], by_id: &HashMap<String, Value>) -> String {
    let workflows: Vec<String> = ids
        .iter()
        .filter_map(|id| by_id.get(id))
        .filter(|m| m.get("category").and_then(|v| v.as_str()) == Some("workflow"))
        .filter_map(|m| m.get("slug").and_then(|v| v.as_str()))
        .map(String::from)
        .collect();
    let mut workflows = workflows;
    workflows.sort();
    if workflows.len() == 1 {
        return workflows[0].strip_prefix("system-").unwrap_or(&workflows[0]).to_string();
    }
    if workflows.len() > 1 {
        let primary = workflows.iter().min_by_key(|w| w.len()).cloned().unwrap_or_default();
        return primary.strip_prefix("system-").unwrap_or(&primary).to_string();
    }
    let first_id = ids.iter().min().cloned().unwrap_or_default();
    let slug = by_id
        .get(&first_id)
        .and_then(|m| m.get("slug").and_then(|v| v.as_str()))
        .unwrap_or("");
    slug.strip_prefix("system-").unwrap_or(slug).to_string()
}

fn bundle_name_for_group(slug: &str, ids: &[String], by_id: &HashMap<String, Value>) -> String {
    let workflows: Vec<&Value> = ids
        .iter()
        .filter_map(|id| by_id.get(id))
        .filter(|m| m.get("category").and_then(|v| v.as_str()) == Some("workflow"))
        .collect();
    if workflows.len() == 1 {
        return workflows[0]
            .get("name")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .unwrap_or_else(|| title_case_slug(slug));
    }
    if workflows.len() > 1 {
        return format!("{} workflows", title_case_slug(slug));
    }
    title_case_slug(slug)
}

fn bundle_description_for_group(ids: &[String], by_id: &HashMap<String, Value>) -> String {
    let mut kinds = HashSet::new();
    for id in ids {
        if let Some(cat) = by_id.get(id).and_then(|m| m.get("category").and_then(|v| v.as_str())) {
            kinds.insert(cat);
        }
    }
    let mut parts = Vec::new();
    if kinds.contains("workflow") {
        parts.push("workflows");
    }
    if kinds.contains("prompt") {
        parts.push("prompts");
    }
    if kinds.contains("skill") {
        parts.push("skills");
    }
    if kinds.contains("command") {
        parts.push("commands");
    }
    format!("Packages linked by template references: {}.", parts.join(", "))
}

fn labels_for_group(ids: &[String], by_id: &HashMap<String, Value>) -> Vec<String> {
    let mut labels = HashSet::new();
    for id in ids {
        let meta = by_id.get(id);
        if let Some(arr) = meta.and_then(|m| m.get("labels").and_then(|v| v.as_array())) {
            for label in arr {
                if let Some(s) = label.as_str() {
                    labels.insert(s.to_string());
                }
            }
        }
    }
    let mut out: Vec<String> = labels.into_iter().collect();
    out.sort();
    out
}

pub fn discover_bundles(root: &Path) -> Result<Vec<BundleDef>, String> {
    let by_id = read_package_metas(root)?;
    let mut strong_edges = HashMap::new();
    let mut weak_edges = HashMap::new();
    for (id, meta) in &by_id {
        let (strong, weak) = refs_for_package(root, meta, &by_id);
        strong_edges.insert(id.clone(), strong);
        weak_edges.insert(id.clone(), weak);
    }
    let groups = connected_components(&by_id, &strong_edges);
    let mut used_slugs = HashSet::new();
    let mut bundles = Vec::new();
    for core_ids in groups {
        let mut ids: HashSet<String> = core_ids.iter().cloned().collect();
        for id in &core_ids {
            for r in weak_edges.get(id).cloned().unwrap_or_default() {
                ids.insert(r);
            }
            for r in strong_edges.get(id).cloned().unwrap_or_default() {
                ids.insert(r.clone());
                for nested in weak_edges.get(&r).cloned().unwrap_or_default() {
                    ids.insert(nested);
                }
            }
        }
        let mut all_ids: Vec<String> = ids.into_iter().collect();
        all_ids.sort();
        let mut slug = bundle_slug_for_group(&all_ids, &by_id);
        if used_slugs.contains(&slug) {
            slug = format!("{}-{}", slug, all_ids.len());
        }
        used_slugs.insert(slug.clone());
        bundles.push(BundleDef {
            name: bundle_name_for_group(&slug, &all_ids, &by_id),
            description: bundle_description_for_group(&all_ids, &by_id),
            labels: labels_for_group(&all_ids, &by_id),
            members: all_ids.clone(),
            slug,
        });
    }
    bundles.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(bundles)
}

pub fn bundle_defs_to_json(bundles: &[BundleDef]) -> Value {
    Value::Array(
        bundles
            .iter()
            .map(|b| {
                serde_json::json!({
                    "slug": b.slug,
                    "name": b.name,
                    "description": b.description,
                    "members": b.members,
                    "labels": b.labels,
                })
            })
            .collect(),
    )
}

pub fn run_discover_bundles(root: &Path, write: bool) -> Result<(), String> {
    let bundles = discover_bundles(root)?;
    if write {
        let path = root.join("scripts/bundles.json");
        let json = bundle_defs_to_json(&bundles);
        std::fs::write(&path, crate::package_meta::stable_stringify(&json))
            .map_err(|e| format!("write {}: {e}", path.display()))?;
        eprintln!("wrote {} bundles to {}", bundles.len(), path.display());
    } else {
        println!("{}", serde_json::to_string_pretty(&bundle_defs_to_json(&bundles)).map_err(|e| e.to_string())?);
    }
    Ok(())
}
