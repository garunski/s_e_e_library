use std::collections::{HashMap, HashSet};
use std::path::Path;

use s_e_e_core::validate_catalog_manifest_value;
use s_e_e_library_ops::validate_package_payload;
use s_e_e_shapes::{CREATE_PROPOSAL_DEFINITION_ID, LibraryKind, SCHEMA_SEE_LIBRARY_V1, SCHEMA_SEE_LIBRARY_V2};
use serde_json::Value;

use crate::package_meta::{
    catalog_without_updated, load_marketplace_source, marketplace_ref_errors, meta_path,
    normalize_deps, normalize_labels, normalize_releases, normalize_tool_ids, slug_counts,
    stable_stringify,
};
use crate::payload_meta::presentation_for_entry;

const CATALOG_NAME: &str = "S.E.E. Official Library";
const ALLOWED_TO_PREFIXES: &[&str] = &[".s_e_e/", ".agents/", ".cursor/", "templates/"];

fn categories() -> HashSet<&'static str> {
    [
        "workflow",
        "prompt",
        "skill",
        "rule",
        "template",
        "command",
        "bundle",
        "cycle",
    ]
    .into_iter()
    .collect()
}

fn library_kind(category: &str) -> Option<LibraryKind> {
    LibraryKind::from_category_str(category)
}

fn infer_kind_from_to(to_path: &str) -> Option<LibraryKind> {
    if to_path.starts_with(".s_e_e/workflows/definitions/") {
        return Some(LibraryKind::Workflow);
    }
    if to_path.starts_with(".s_e_e/workflows/schedules/") {
        return None;
    }
    if to_path.starts_with(".s_e_e/workflows/schedule_rule_sets/") {
        return None;
    }
    if to_path.starts_with(".s_e_e/prompts/") {
        return Some(LibraryKind::Prompt);
    }
    if to_path.starts_with(".s_e_e/commands/") {
        return Some(LibraryKind::Command);
    }
    if to_path.starts_with(".agents/skills/") && to_path.ends_with("SKILL.md") {
        return Some(LibraryKind::Skill);
    }
    if to_path.ends_with(".mdc") {
        return Some(LibraryKind::Rule);
    }
    if to_path.starts_with("templates/") {
        return Some(LibraryKind::Template);
    }
    if to_path.starts_with(".s_e_e/knowledge/") {
        return None;
    }
    if to_path.starts_with(".s_e_e/cycles/") && to_path.ends_with(".json") {
        return Some(LibraryKind::Cycle);
    }
    None
}

fn read_package_meta(
    root: &Path,
    entry: &Value,
    label: &str,
    counts: &HashMap<String, usize>,
) -> Result<Value, String> {
    let path = meta_path(root, entry, counts);
    if !path.is_file() {
        let rel = path
            .strip_prefix(root)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| path.display().to_string());
        return Err(format!("{label}: missing {rel}"));
    }
    let raw = std::fs::read_to_string(&path)
        .map_err(|e| format!("{label}: invalid s_e_e_package.json: {e}"))?;
    let meta: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("{label}: invalid s_e_e_package.json: {e}"))?;
    let entry_id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let meta_id = meta.get("id").and_then(|v| v.as_str()).unwrap_or("");
    if meta_id != entry_id {
        return Err(format!(
            "{label}: s_e_e_package.json id mismatch: expected {entry_id}, got {meta_id}"
        ));
    }
    let entry_slug = entry.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    let meta_slug = meta.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    if meta_slug != entry_slug {
        return Err(format!(
            "{label}: s_e_e_package.json slug mismatch: expected {entry_slug}, got {meta_slug}"
        ));
    }
    let entry_category = entry.get("category").and_then(|v| v.as_str()).unwrap_or("");
    let meta_category = meta.get("category").and_then(|v| v.as_str()).unwrap_or("");
    if meta_category != entry_category {
        return Err(format!(
            "{label}: s_e_e_package.json category mismatch: expected {entry_category}, got {meta_category}"
        ));
    }
    let name = meta
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("{label}: s_e_e_package.json requires a non-empty name"))?;
    if meta.get("description").is_some() && !meta.get("description").unwrap().is_string() {
        return Err(format!("{label}: s_e_e_package.json description must be a string"));
    }
    let labels = normalize_labels(meta.get("labels").unwrap_or(&Value::Array(vec![])))
        .ok_or_else(|| {
            format!("{label}: s_e_e_package.json labels must be an array of non-empty strings")
        })?;
    let dependencies = normalize_deps(meta.get("dependencies").unwrap_or(&Value::Array(vec![])))
        .ok_or_else(|| {
            format!("{label}: s_e_e_package.json dependencies must be an array of non-empty strings")
        })?;
    if !meta.as_object().map(|o| o.contains_key("toolIds")).unwrap_or(false) {
        return Err(format!("{label}: s_e_e_package.json requires toolIds"));
    }
    let tool_ids = normalize_tool_ids(meta.get("toolIds").unwrap())
        .ok_or_else(|| format!("{label}: s_e_e_package.json toolIds must be an array of non-empty strings"))?;
    if !meta.as_object().map(|o| o.contains_key("releases")).unwrap_or(false) {
        return Err(format!("{label}: s_e_e_package.json requires releases"));
    }
    let releases = normalize_releases(meta.get("releases").unwrap()).ok_or_else(|| {
        format!("{label}: s_e_e_package.json releases must be {{ version, date, note }} objects")
    })?;
    Ok(serde_json::json!({
        "name": name,
        "description": meta.get("description").and_then(|v| v.as_str()).map(str::trim).unwrap_or(""),
        "labels": labels,
        "dependencies": dependencies,
        "toolIds": tool_ids,
        "releases": releases,
    }))
}

fn apply_package_meta(
    entry: &mut Value,
    label: &str,
    root: &Path,
    counts: &HashMap<String, usize>,
) -> Result<(), String> {
    let meta = read_package_meta(root, entry, label, counts)?;
    let name = meta.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let description = meta.get("description").and_then(|v| v.as_str()).unwrap_or("");
    let labels = meta.get("labels").cloned().unwrap_or(Value::Array(vec![]));
    let dependencies = meta.get("dependencies").cloned().unwrap_or(Value::Array(vec![]));
    let tool_ids = meta.get("toolIds").cloned().unwrap_or(Value::Array(vec![]));
    let releases = meta.get("releases").cloned().unwrap_or(Value::Array(vec![]));

    let obj = entry.as_object_mut().expect("package entry object");
    obj.insert("name".to_string(), Value::String(name.to_string()));
    if description.is_empty() {
        obj.remove("description");
    } else {
        obj.insert("description".to_string(), Value::String(description.to_string()));
    }
    obj.insert("labels".to_string(), labels);
    obj.insert("dependencies".to_string(), dependencies);
    obj.insert("toolIds".to_string(), tool_ids);
    obj.insert("releases".to_string(), releases);
    Ok(())
}

fn validate_dependency_ids(entry: &Value, label: &str) -> Result<(), String> {
    let entry_id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let deps = entry
        .get("dependencies")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("{label}: dependencies must be an array"))?;
    for dep in deps {
        let dep_id = dep.as_str().unwrap_or("").trim();
        if dep_id.is_empty() {
            return Err(format!("{label}: invalid dependency id"));
        }
        if dep_id == entry_id {
            return Err(format!("{label}: package cannot depend on itself"));
        }
    }
    Ok(())
}

fn validate_package_files(entry: &Value, label: &str, root: &Path) -> Result<(), String> {
    let files = entry
        .get("files")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("{label}: files must be an array"))?;
    if files.is_empty() {
        return Err(format!("{label}: files must be a non-empty array"));
    }
    let category = entry.get("category").and_then(|v| v.as_str()).unwrap_or("");
    let mut seen_to = HashSet::new();
    for file in files {
        let to = file.get("to").and_then(|v| v.as_str());
        let from = file.get("from").and_then(|v| v.as_str());
        if to.is_none() || from.is_none() {
            return Err(format!("{label}: each files[] entry needs to and from"));
        }
        let to = to.unwrap();
        let from = from.unwrap();
        if to.contains('\\') {
            return Err(format!("{label}: install path must use forward slashes: {to}"));
        }
        if !ALLOWED_TO_PREFIXES.iter().any(|prefix| to.starts_with(prefix)) {
            return Err(format!(
                "{label}: install path must start with one of {}: {}",
                ALLOWED_TO_PREFIXES.join(", "),
                to
            ));
        }
        if seen_to.contains(to) {
            return Err(format!("{label}: duplicate install path: {to}"));
        }
        seen_to.insert(to.to_string());
        let from_path = root.join(from);
        if !from_path.is_file() {
            return Err(format!("{label}: missing payload file {from}"));
        }
        let content = std::fs::read_to_string(&from_path)
            .map_err(|e| format!("{label}: read {from}: {e}"))?;
        let kind = if category == "bundle" {
            infer_kind_from_to(to).or_else(|| {
                if to.starts_with(".s_e_e/workflows/schedules/") {
                    None
                } else if to.starts_with(".s_e_e/workflows/schedule_rule_sets/") {
                    None
                } else if to.starts_with(".s_e_e/knowledge/") {
                    None
                } else {
                    None
                }
            })
        } else {
            library_kind(category)
        };
        if category == "bundle" {
            if to.starts_with(".s_e_e/workflows/schedules/")
                || to.starts_with(".s_e_e/workflows/schedule_rule_sets/")
                || to.starts_with(".s_e_e/knowledge/")
            {
                if to.starts_with(".s_e_e/knowledge/") && content.trim().is_empty() {
                    return Err(format!("{label}: {from}: knowledge doc cannot be empty"));
                }
                continue;
            }
            if kind.is_none() {
                return Err(format!("{label}: cannot infer payload kind for install path: {to}"));
            }
        }
        let kind = kind.expect("library kind");
        if let Err(errors) = validate_package_payload(kind, &[(to, content.as_str())]) {
            let first = errors
                .errors
                .first()
                .map(|e| e.message.clone())
                .unwrap_or_else(|| "invalid payload".to_string());
            return Err(format!("{label}: {from}: {first}"));
        }
    }
    Ok(())
}

fn validate_entry(
    entry: &mut Value,
    label: &str,
    root: &Path,
    counts: &HashMap<String, usize>,
) -> Result<(), String> {
    let cats = categories();
    let category = entry.get("category").and_then(|v| v.as_str()).unwrap_or("");
    if !cats.contains(category) {
        return Err(format!("{label}: invalid category {category}"));
    }
    for key in [
        "id",
        "slug",
        "category",
        "version",
        "author",
        "verified",
        "license",
        "requires",
        "dependencies",
        "files",
    ] {
        if !entry.get(key).is_some() {
            return Err(format!("{label}: missing field {key}"));
        }
    }
    if !entry.get("requires").and_then(|v| v.as_array()).is_some() {
        return Err(format!("{label}: requires must be an array"));
    }
    validate_dependency_ids(entry, label)?;
    if category == "bundle" {
        validate_package_files(entry, label, root)?;
        return apply_package_meta(entry, label, root, counts);
    }
    validate_package_files(entry, label, root)?;
    let (name, description) = presentation_for_entry(entry, root);
    if name.trim().is_empty() {
        return Err(format!("{label}: name is required"));
    }
    if entry.get("name").is_some() && !entry.get("name").unwrap().is_string() {
        return Err(format!("{label}: name must be a string"));
    }
    if entry.get("description").is_some() && !entry.get("description").unwrap().is_string() {
        return Err(format!("{label}: description must be a string"));
    }
    let obj = entry.as_object_mut().expect("entry");
    obj.insert("name".to_string(), Value::String(name));
    if description.is_empty() {
        obj.remove("description");
    } else {
        obj.insert("description".to_string(), Value::String(description));
    }
    apply_package_meta(entry, label, root, counts)
}

fn package_provides_workflow(
    packages: &[Value],
    package_id: &str,
    definition_id: &str,
    seen: &mut HashSet<String>,
) -> bool {
    if seen.contains(package_id) {
        return false;
    }
    seen.insert(package_id.to_string());
    let entry = packages.iter().find(|p| p.get("id").and_then(|v| v.as_str()) == Some(package_id));
    if entry.is_none() {
        return false;
    }
    let entry = entry.unwrap();
    let path = format!(".s_e_e/workflows/definitions/{definition_id}.json");
    let files = entry.get("files").and_then(|v| v.as_array());
    if files
        .map(|files| {
            files.iter().any(|f| f.get("to").and_then(|v| v.as_str()) == Some(path.as_str()))
        })
        .unwrap_or(false)
    {
        return true;
    }
    let members = entry
        .get("memberPackageIds")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    members.iter().any(|m| {
        m.as_str()
            .is_some_and(|id| package_provides_workflow(packages, id, definition_id, seen))
    })
}

fn validate_standard_cycle_workflow_deps(catalog: &Value, root: &Path) -> Result<(), String> {
    let packages = catalog
        .get("packages")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    for entry in &packages {
        if entry.get("category").and_then(|v| v.as_str()) != Some("cycle") {
            continue;
        }
        if entry.get("verified").and_then(|v| v.as_bool()) != Some(true) {
            continue;
        }
        let entry_id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let cycle_file = entry
            .get("files")
            .and_then(|v| v.as_array())
            .and_then(|files| {
                files.iter().find(|f| {
                    f.get("to")
                        .and_then(|v| v.as_str())
                        .is_some_and(|to| to.starts_with(".s_e_e/cycles/"))
                })
            });
        let from = cycle_file
            .and_then(|f| f.get("from"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("{entry_id}: verified cycle is missing a cycle document file"))?;
        let from_path = root.join(from);
        if !from_path.is_file() {
            return Err(format!("{entry_id}: missing cycle payload {from}"));
        }
        let raw = std::fs::read_to_string(&from_path)
            .map_err(|e| format!("{entry_id}: read cycle payload: {e}"))?;
        let document: Value = serde_json::from_str(&raw)
            .map_err(|e| format!("{entry_id}: invalid cycle JSON: {e}"))?;
        let host = document.get("host").and_then(|v| v.as_str()).unwrap_or("");
        if host != "knowledge" && host != "orchestrator" {
            continue;
        }
        let deps: Vec<String> = entry
            .get("dependencies")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|d| d.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let stages = document
            .get("stages")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for stage in stages {
            let definition_id = stage
                .get("workflow_definition_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if definition_id.is_empty() || definition_id == CREATE_PROPOSAL_DEFINITION_ID {
                continue;
            }
            let ok = deps.iter().any(|dep| {
                package_provides_workflow(&packages, dep, definition_id, &mut HashSet::new())
            });
            if !ok {
                let key = stage.get("key").and_then(|v| v.as_str()).unwrap_or("?");
                return Err(format!(
                    "{entry_id}: stage {key} refers to {definition_id} without a package dependency"
                ));
            }
        }
    }
    Ok(())
}

pub fn read_catalog(root: &Path) -> Result<Value, String> {
    let path = root.join("catalog.json");
    let raw = std::fs::read_to_string(&path).map_err(|_| "catalog.json not found".to_string())?;
    let catalog: Value =
        serde_json::from_str(&raw).map_err(|e| format!("invalid catalog.json: {e}"))?;
    let schema = catalog.get("schema").and_then(|v| v.as_str());
    match schema {
        Some(SCHEMA_SEE_LIBRARY_V1) | Some(SCHEMA_SEE_LIBRARY_V2) => {}
        Some(other) => {
            return Err(format!(
                "unsupported schema: expected {SCHEMA_SEE_LIBRARY_V1} or {SCHEMA_SEE_LIBRARY_V2}, got {other}"
            ));
        }
        None => {
            return Err(format!(
                "unsupported schema: expected {SCHEMA_SEE_LIBRARY_V1} or {SCHEMA_SEE_LIBRARY_V2}, got (missing)"
            ));
        }
    }
    if !catalog.get("packages").and_then(|v| v.as_array()).is_some() {
        return Err("catalog.packages must be an array".to_string());
    }
    Ok(catalog)
}

pub fn build_next_catalog(root: &Path, catalog: &mut Value) -> Result<Value, String> {
    let packages = catalog
        .get("packages")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let counts = slug_counts(&packages);
    let mut ids = HashSet::new();
    let mut package_values = packages;
    for entry in &mut package_values {
        let label = entry
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("(missing id)")
            .to_string();
        if ids.contains(&label) {
            return Err(format!("duplicate package id: {label}"));
        }
        ids.insert(label.clone());
        validate_entry(entry, &label, root, &counts)?;
    }
    for entry in &package_values {
        let entry_id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("");
        let deps = entry
            .get("dependencies")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for dep in deps {
            let dep_id = dep.as_str().unwrap_or("");
            if !ids.contains(dep_id) {
                return Err(format!("{entry_id}: dependency not found in catalog: {dep_id}"));
            }
        }
    }
    catalog.as_object_mut().unwrap().insert(
        "packages".to_string(),
        Value::Array(package_values),
    );
    validate_standard_cycle_workflow_deps(catalog, root)?;

    let marketplace = load_marketplace_source(root)?;
    let tools = marketplace.get("tools").cloned().unwrap_or(Value::Array(vec![]));
    let stacks = marketplace.get("stacks").cloned().unwrap_or(Value::Array(vec![]));
    let featured = marketplace
        .get("featuredStackId")
        .and_then(|v| v.as_str())
        .map(String::from);

    let package_ids: HashSet<String> = catalog
        .get("packages")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.get("id").and_then(|v| v.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let ref_errors = marketplace_ref_errors(
        tools.as_array().map(|a| a.as_slice()).unwrap_or(&[]),
        stacks.as_array().map(|a| a.as_slice()).unwrap_or(&[]),
        featured.as_deref(),
        &package_ids,
    );
    if let Some(first) = ref_errors.first() {
        return Err(first.clone());
    }

    let name = catalog
        .get("name")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or(CATALOG_NAME);
    let mut next = serde_json::json!({
        "schema": SCHEMA_SEE_LIBRARY_V2,
        "name": name,
        "updated": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        "tools": tools,
        "stacks": stacks,
        "packages": catalog.get("packages").cloned().unwrap_or(Value::Array(vec![])),
    });
    if let Some(featured) = featured {
        next.as_object_mut()
            .unwrap()
            .insert("featuredStackId".to_string(), Value::String(featured));
    }

    validate_catalog_manifest_value(&next).map_err(|e| e.to_string())?;
    Ok(next)
}

pub fn assert_catalog_matches_next(raw: &str, next: &Value, label: &str) -> Result<(), String> {
    let current: Value =
        serde_json::from_str(raw).map_err(|e| format!("{label}: invalid JSON: {e}"))?;
    let current_body = stable_stringify(&catalog_without_updated(&current));
    let next_body = stable_stringify(&catalog_without_updated(next));
    if current_body != next_body {
        return Err(format!("{label} is out of date; run catalog && copy"));
    }
    Ok(())
}

pub fn run_validate(root: &Path) -> Result<(), String> {
    let mut catalog = read_catalog(root)?;
    let next = build_next_catalog(root, &mut catalog)?;
    let catalog_path = root.join("catalog.json");
    let raw = std::fs::read_to_string(&catalog_path).map_err(|e| e.to_string())?;
    assert_catalog_matches_next(&raw, &next, "catalog.json")?;
    let public_path = root.join("public/catalog.json");
    if public_path.is_file() {
        let public_raw = std::fs::read_to_string(&public_path).map_err(|e| e.to_string())?;
        assert_catalog_matches_next(&public_raw, &next, "public/catalog.json")?;
    }
    let current: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    let count = current
        .get("packages")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let updated = current
        .get("updated")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    eprintln!("ok: {count} packages validated ({updated})");
    Ok(())
}

pub fn run_catalog(root: &Path) -> Result<(), String> {
    let mut catalog = read_catalog(root)?;
    let next = build_next_catalog(root, &mut catalog)?;
    let catalog_path = root.join("catalog.json");
    std::fs::write(&catalog_path, stable_stringify(&next)).map_err(|e| e.to_string())?;
    let count = next
        .get("packages")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    eprintln!("wrote catalog.json ({count} packages)");
    Ok(())
}
