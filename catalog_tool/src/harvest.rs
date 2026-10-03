use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::discover_bundles::{discover_bundles, BundleDef};
use crate::package_meta::{
    meta_path, normalize_deps, normalize_labels, normalize_releases, normalize_tool_ids,
    read_existing_meta, slug_counts, stable_stringify,
};
use crate::payload_meta::{extract_payload_meta, presentation_for_entry};
use crate::validate::{read_catalog, run_catalog};

const VERSION: &str = "1.0.0";
const AUTHOR: &str = "garunski";
const LICENSE: &str = "AGPL-3.0-only";

const PROMPTS: &[&str] = &[
    "system-implement-story",
    "system-quality-full-fix",
    "system-stories-from-doc-authoring",
    "system-story-authoring",
    "system-work-story",
    "system-workflow-authoring",
];

const WORKFLOWS: &[&str] = &[
    "system-author-stories-from-doc",
    "system-author-story",
    "system-author-workflow",
    "system-implement-stories-bulk",
    "system-implement-story",
];

const LIBRARY_ONLY_WORKFLOWS: &[&str] = &["system-work-swimlane-stories"];

const COMMANDS: &[&str] = &["claude-code", "cursor-agent"];

const SKILLS: &[&str] = &["stories", "work", "doc", "workflow"];

const WORKFLOW_REQUIRES: &[&str] = &["mise", "git"];

pub fn default_hub_root(library_root: &Path) -> PathBuf {
    library_root
        .parent()
        .map(|p| p.join("s_e_e_project"))
        .unwrap_or_else(|| library_root.join("..").join("s_e_e_project"))
}

struct BuiltEntry {
    entry: Value,
    seed_labels: Vec<String>,
}

fn prompt_deps_from_workflow(content: &str) -> Vec<String> {
    let mut ids = HashSet::new();
    let mut i = 0;
    while let Some(start) = content[i..].find("prompt.") {
        let abs = i + start + "prompt.".len();
        let mut end = abs;
        while end < content.len() {
            let c = content.as_bytes()[end];
            if c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' {
                end += 1;
            } else {
                break;
            }
        }
        if end > abs {
            ids.insert(format!("prompt-{}", &content[abs..end]));
        }
        i = end.max(abs);
    }
    let mut out: Vec<String> = ids.into_iter().collect();
    out.sort();
    out
}

fn copy_file(src: &Path, dest: &Path) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    std::fs::copy(src, dest)
        .map_err(|e| format!("copy {} -> {}: {e}", src.display(), dest.display()))?;
    Ok(())
}

fn write_meta(
    root: &Path,
    entry: &Value,
    seed_labels: &[String],
    counts: &HashMap<String, usize>,
) -> Result<Value, String> {
    let existing = read_existing_meta(root, entry, counts);
    let entry_id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let entry_slug = entry.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    let entry_category = entry.get("category").and_then(|v| v.as_str()).unwrap_or("");

    let name = existing
        .as_ref()
        .and_then(|e| e.get("name"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| entry.get("name").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()))
        .unwrap_or("")
        .to_string();
    let description = existing
        .as_ref()
        .and_then(|e| e.get("description"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| entry.get("description").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()))
        .unwrap_or("")
        .to_string();

    let labels = if existing.as_ref().map(|e| e.get("labels").is_some()).unwrap_or(false) {
        normalize_labels(existing.as_ref().unwrap().get("labels").unwrap())
            .ok_or_else(|| format!("{entry_id}: invalid labels in existing meta"))?
    } else {
        normalize_labels(&Value::Array(
            seed_labels.iter().map(|s| Value::String(s.clone())).collect(),
        ))
        .unwrap_or_default()
    };

    let authored = if existing.as_ref().map(|e| e.get("dependencies").is_some()).unwrap_or(false) {
        normalize_deps(existing.as_ref().unwrap().get("dependencies").unwrap())
            .ok_or_else(|| format!("{entry_id}: invalid dependencies in existing meta"))?
    } else {
        Vec::new()
    };

    let entry_deps: Vec<String> = entry
        .get("dependencies")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|d| d.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let mut merged_deps = entry_deps;
    merged_deps.extend(authored);
    let dependencies = normalize_deps(&Value::Array(
        merged_deps.iter().map(|s| Value::String(s.clone())).collect(),
    ))
    .ok_or_else(|| format!("{entry_id}: invalid merged dependencies"))?;

    let tool_ids = if existing.as_ref().map(|e| e.get("toolIds").is_some()).unwrap_or(false) {
        normalize_tool_ids(existing.as_ref().unwrap().get("toolIds").unwrap())
            .ok_or_else(|| format!("{entry_id}: invalid toolIds in existing meta"))?
    } else {
        Vec::new()
    };

    let releases = if existing.as_ref().map(|e| e.get("releases").is_some()).unwrap_or(false) {
        normalize_releases(existing.as_ref().unwrap().get("releases").unwrap())
            .ok_or_else(|| format!("{entry_id}: invalid releases in existing meta"))?
    } else {
        Vec::new()
    };

    let mut meta = serde_json::json!({
        "id": entry_id,
        "slug": entry_slug,
        "category": entry_category,
        "name": name,
        "description": description,
        "labels": labels,
        "dependencies": dependencies,
        "toolIds": tool_ids,
        "releases": releases,
    });
    if let Some(existing) = existing {
        if existing.get("requires").and_then(|v| v.as_array()).is_some() {
            meta.as_object_mut()
                .unwrap()
                .insert("requires".to_string(), existing.get("requires").cloned().unwrap());
        }
    }

    let path = meta_path(root, entry, counts);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, stable_stringify(&meta)).map_err(|e| e.to_string())?;
    Ok(meta)
}

fn finalize_entries(root: &Path, built: &[BuiltEntry]) -> Result<Vec<Value>, String> {
    let entries: Vec<Value> = built.iter().map(|b| b.entry.clone()).collect();
    let counts = slug_counts(&entries);
    let mut out = Vec::new();
    for item in built {
        let meta = write_meta(root, &item.entry, &item.seed_labels, &counts)?;
        let mut entry = item.entry.clone();
        let obj = entry.as_object_mut().unwrap();
        obj.insert(
            "name".to_string(),
            Value::String(meta.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string()),
        );
        let desc = meta.get("description").and_then(|v| v.as_str()).unwrap_or("");
        if desc.is_empty() {
            obj.remove("description");
        } else {
            obj.insert("description".to_string(), Value::String(desc.to_string()));
        }
        obj.insert("labels".to_string(), meta.get("labels").cloned().unwrap());
        obj.insert(
            "dependencies".to_string(),
            meta.get("dependencies").cloned().unwrap(),
        );
        obj.insert("toolIds".to_string(), meta.get("toolIds").cloned().unwrap());
        obj.insert("releases".to_string(), meta.get("releases").cloned().unwrap());
        out.push(entry);
    }
    Ok(out)
}

fn with_presentation(root: &Path, entry: Value, payload_content: &str) -> Value {
    let category = entry.get("category").and_then(|v| v.as_str()).unwrap_or("");
    let meta = extract_payload_meta(category, payload_content);
    let mut merged = entry.clone();
    if !meta.0.is_empty() {
        let obj = merged.as_object_mut().unwrap();
        if obj.get("name").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
            obj.insert("name".to_string(), Value::String(meta.0.clone()));
        }
    }
    if !meta.1.is_empty() {
        let obj = merged.as_object_mut().unwrap();
        if obj.get("description").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
            obj.insert("description".to_string(), Value::String(meta.1.clone()));
        }
    }
    let (name, description) = presentation_for_entry(&merged, root);
    let obj = merged.as_object_mut().unwrap();
    obj.insert("name".to_string(), Value::String(name));
    if description.is_empty() {
        obj.remove("description");
    } else {
        obj.insert("description".to_string(), Value::String(description));
    }
    merged
}

fn read_hub_json(hub: &Path, rel: &str) -> Result<String, String> {
    let path = hub.join(rel);
    if !path.is_file() {
        return Err(format!("missing hub file: {}", path.display()));
    }
    std::fs::read_to_string(&path)
        .map_err(|e| format!("read {}: {e}", path.display()))
}

fn prompt_entry(root: &Path, hub: &Path, stem: &str) -> Result<BuiltEntry, String> {
    let slug = stem;
    let from = format!("packages/{slug}/{VERSION}/prompt.json");
    let src = hub.join(".s_e_e/prompts").join(format!("{stem}.json"));
    let dest = root.join(&from);
    let content = read_hub_json(hub, &format!(".s_e_e/prompts/{stem}.json"))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|e| format!("invalid JSON in .s_e_e/prompts/{stem}.json: {e}"))?;
    copy_file(&src, &dest)?;
    let entry = with_presentation(
        root,
        serde_json::json!({
            "id": format!("prompt-{stem}"),
            "slug": slug,
            "category": "prompt",
            "version": VERSION,
            "author": AUTHOR,
            "verified": true,
            "license": LICENSE,
            "requires": [],
            "dependencies": [],
            "files": [{ "to": format!(".s_e_e/prompts/{stem}.json"), "from": from }],
        }),
        &content,
    );
    Ok(BuiltEntry {
        entry,
        seed_labels: Vec::new(),
    })
}

fn workflow_entry_from_content(root: &Path, stem: &str, content: &str) -> BuiltEntry {
    let slug = stem;
    let from = format!("packages/{slug}/{VERSION}/definition.json");
    let seed_labels = serde_json::from_str::<Value>(content)
        .ok()
        .and_then(|parsed| normalize_labels(parsed.get("labels").unwrap_or(&Value::Array(vec![]))))
        .unwrap_or_default();
    let entry = with_presentation(
        root,
        serde_json::json!({
            "id": format!("wf-{stem}"),
            "slug": slug,
            "category": "workflow",
            "version": VERSION,
            "author": AUTHOR,
            "verified": true,
            "license": LICENSE,
            "requires": WORKFLOW_REQUIRES,
            "dependencies": prompt_deps_from_workflow(content),
            "files": [{ "to": format!(".s_e_e/workflows/definitions/{stem}.json"), "from": from }],
        }),
        content,
    );
    BuiltEntry {
        entry,
        seed_labels,
    }
}

fn workflow_entry(root: &Path, hub: &Path, stem: &str) -> Result<BuiltEntry, String> {
    let from = format!("packages/{stem}/{VERSION}/definition.json");
    let src = hub
        .join(".s_e_e/workflows/definitions")
        .join(format!("{stem}.json"));
    let dest = root.join(&from);
    let content = read_hub_json(hub, &format!(".s_e_e/workflows/definitions/{stem}.json"))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|e| format!("invalid JSON in .s_e_e/workflows/definitions/{stem}.json: {e}"))?;
    copy_file(&src, &dest)?;
    Ok(workflow_entry_from_content(root, stem, &content))
}

fn library_only_workflow_entry(root: &Path, stem: &str) -> Result<BuiltEntry, String> {
    let from = format!("packages/{stem}/{VERSION}/definition.json");
    let path = root.join(&from);
    if !path.is_file() {
        return Err(format!("missing library-only workflow payload: {}", path.display()));
    }
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok(workflow_entry_from_content(root, stem, &content))
}

fn command_entry(root: &Path, hub: &Path, id: &str) -> Result<BuiltEntry, String> {
    let slug = id;
    let from = format!("packages/{slug}/{VERSION}/command.json");
    let src = hub.join(".s_e_e/commands").join(format!("{id}.json"));
    let dest = root.join(&from);
    let content = read_hub_json(hub, &format!(".s_e_e/commands/{id}.json"))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|e| format!("invalid JSON in .s_e_e/commands/{id}.json: {e}"))?;
    copy_file(&src, &dest)?;
    let entry = with_presentation(
        root,
        serde_json::json!({
            "id": format!("cmd-{id}"),
            "slug": slug,
            "category": "command",
            "version": VERSION,
            "author": AUTHOR,
            "verified": true,
            "license": LICENSE,
            "requires": [],
            "dependencies": [],
            "files": [{ "to": format!(".s_e_e/commands/{id}.json"), "from": from }],
        }),
        &content,
    );
    Ok(BuiltEntry {
        entry,
        seed_labels: Vec::new(),
    })
}

fn skill_entry(root: &Path, hub: &Path, slug: &str) -> Result<BuiltEntry, String> {
    let from = format!("packages/{slug}/{VERSION}/SKILL.md");
    let src = hub.join(".agents/skills").join(slug).join("SKILL.md");
    let dest = root.join(&from);
    let content = read_hub_json(hub, &format!(".agents/skills/{slug}/SKILL.md"))?;
    copy_file(&src, &dest)?;
    let entry = with_presentation(
        root,
        serde_json::json!({
            "id": format!("skill-{slug}"),
            "slug": slug,
            "category": "skill",
            "version": VERSION,
            "author": AUTHOR,
            "verified": true,
            "license": LICENSE,
            "requires": [],
            "dependencies": [],
            "files": [{ "to": format!(".agents/skills/{slug}/SKILL.md"), "from": from }],
        }),
        &content,
    );
    Ok(BuiltEntry {
        entry,
        seed_labels: Vec::new(),
    })
}

fn bundle_entry(
    root: &Path,
    def: &BundleDef,
    by_id: &HashMap<String, Value>,
) -> Result<BuiltEntry, String> {
    let bundle_slug = &def.slug;
    let mut files = Vec::new();
    for member_id in &def.members {
        let member = by_id
            .get(member_id)
            .ok_or_else(|| format!("bundle {bundle_slug}: unknown member {member_id}"))?;
        let member_slug = member.get("slug").and_then(|v| v.as_str()).unwrap_or("");
        let member_files = member
            .get("files")
            .and_then(|v| v.as_array())
            .ok_or_else(|| format!("bundle {bundle_slug}: member {member_id} missing files"))?;
        for file in member_files {
            let from_rel = file.get("from").and_then(|v| v.as_str()).unwrap_or("");
            let to = file.get("to").and_then(|v| v.as_str()).unwrap_or("");
            let src = root.join(from_rel);
            let base = Path::new(from_rel)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("payload");
            let local_from = format!("packages/{bundle_slug}/{VERSION}/{member_slug}/{base}");
            let dest = root.join(&local_from);
            copy_file(&src, &dest)?;
            files.push(serde_json::json!({ "to": to, "from": local_from }));
        }
    }
    files.sort_by(|a, b| {
        let at = a.get("to").and_then(|v| v.as_str()).unwrap_or("");
        let bt = b.get("to").and_then(|v| v.as_str()).unwrap_or("");
        at.cmp(bt)
    });
    Ok(BuiltEntry {
        entry: serde_json::json!({
            "id": format!("bundle-{bundle_slug}"),
            "slug": bundle_slug,
            "category": "bundle",
            "version": VERSION,
            "author": AUTHOR,
            "verified": true,
            "license": LICENSE,
            "requires": [],
            "dependencies": [],
            "files": files,
            "name": def.name,
            "description": def.description,
        }),
        seed_labels: def.labels.clone(),
    })
}

fn package_signature(entry: &Value) -> Value {
    serde_json::json!({
        "id": entry.get("id"),
        "version": entry.get("version"),
        "category": entry.get("category"),
        "files": entry.get("files"),
    })
}

pub fn run_harvest(library_root: &Path, hub: &Path) -> Result<(), String> {
    if !hub.is_dir() {
        return Err(format!("missing hub path: {}", hub.display()));
    }

    let _existing_catalog = read_catalog(library_root)?;
    let bundle_defs = discover_bundles(library_root)?;

    let mut member_built = Vec::new();
    for stem in PROMPTS {
        member_built.push(prompt_entry(library_root, hub, stem)?);
    }
    for stem in WORKFLOWS {
        member_built.push(workflow_entry(library_root, hub, stem)?);
    }
    for stem in LIBRARY_ONLY_WORKFLOWS {
        member_built.push(library_only_workflow_entry(library_root, stem)?);
    }
    for id in COMMANDS {
        member_built.push(command_entry(library_root, hub, id)?);
    }
    for slug in SKILLS {
        member_built.push(skill_entry(library_root, hub, slug)?);
    }

    let by_id: HashMap<String, Value> = member_built
        .iter()
        .map(|b| {
            (
                b.entry
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                b.entry.clone(),
            )
        })
        .collect();

    for def in &bundle_defs {
        member_built.push(bundle_entry(library_root, def, &by_id)?);
    }

    let packages = finalize_entries(library_root, &member_built)?;
    let mut packages = packages;
    packages.sort_by(|a, b| {
        let ai = a.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let bi = b.get("id").and_then(|v| v.as_str()).unwrap_or("");
        ai.cmp(bi)
    });

    let catalog = serde_json::json!({
        "schema": "see.library/v1",
        "name": "S.E.E. Official Library",
        "updated": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        "packages": packages,
    });
    let catalog_path = library_root.join("catalog.json");
    std::fs::write(&catalog_path, stable_stringify(&catalog)).map_err(|e| e.to_string())?;

    run_catalog(library_root)?;
    let count = packages.len();
    eprintln!("harvested {count} packages into {}", library_root.display());
    Ok(())
}

pub fn package_signatures_from_catalog(catalog: &Value) -> Vec<Value> {
    let mut sigs: Vec<Value> = catalog
        .get("packages")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(package_signature).collect())
        .unwrap_or_default();
    sigs.sort_by(|a, b| {
        let ai = a.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let bi = b.get("id").and_then(|v| v.as_str()).unwrap_or("");
        ai.cmp(bi)
    });
    sigs
}
