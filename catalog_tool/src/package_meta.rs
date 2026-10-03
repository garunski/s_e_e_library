use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

pub fn normalize_labels(labels: &Value) -> Option<Vec<String>> {
    let arr = labels.as_array()?;
    let mut out = Vec::new();
    for item in arr {
        let s = item.as_str()?.trim();
        if s.is_empty() {
            return None;
        }
        out.push(s.to_string());
    }
    out.sort();
    out.dedup();
    Some(out)
}

pub fn normalize_deps(deps: &Value) -> Option<Vec<String>> {
    let arr = deps.as_array()?;
    let mut out = Vec::new();
    for item in arr {
        let s = item.as_str()?.trim();
        if s.is_empty() {
            return None;
        }
        out.push(s.to_string());
    }
    out.sort();
    out.dedup();
    Some(out)
}

pub fn normalize_tool_ids(tool_ids: &Value) -> Option<Vec<String>> {
    let arr = tool_ids.as_array()?;
    let mut out = Vec::new();
    for item in arr {
        let s = item.as_str()?.trim();
        if s.is_empty() {
            return None;
        }
        out.push(s.to_string());
    }
    out.sort();
    out.dedup();
    Some(out)
}

pub fn normalize_releases(releases: &Value) -> Option<Vec<Value>> {
    let arr = releases.as_array()?;
    let mut out = Vec::new();
    for release in arr {
        let obj = release.as_object()?;
        let version = obj.get("version")?.as_str()?.trim();
        let date = obj.get("date")?.as_str()?.trim();
        let note = obj.get("note")?.as_str()?.trim();
        if version.is_empty() || date.is_empty() || note.is_empty() {
            return None;
        }
        out.push(serde_json::json!({
            "version": version,
            "date": date,
            "note": note,
        }));
    }
    Some(out)
}

pub fn catalog_without_updated(catalog: &Value) -> Value {
    let mut copy = catalog.clone();
    if let Some(obj) = copy.as_object_mut() {
        obj.remove("updated");
    }
    copy
}

pub fn stable_stringify(value: &Value) -> String {
    format!("{}\n", serde_json::to_string_pretty(value).expect("serialize catalog"))
}

pub fn marketplace_ref_errors(
    tools: &[Value],
    stacks: &[Value],
    featured_stack_id: Option<&str>,
    package_ids: &HashSet<String>,
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut tool_ids = HashSet::new();
    for tool in tools {
        let id = tool.get("id").and_then(|v| v.as_str()).unwrap_or("").trim();
        let name = tool.get("name").and_then(|v| v.as_str()).unwrap_or("").trim();
        if id.is_empty() || name.is_empty() {
            errors.push("tool requires id and name".to_string());
            continue;
        }
        if tool_ids.contains(id) {
            errors.push(format!("duplicate tool id: {id}"));
        }
        tool_ids.insert(id.to_string());
    }

    let mut stack_ids = HashSet::new();
    for stack in stacks {
        let id = stack.get("id").and_then(|v| v.as_str()).unwrap_or("").trim();
        let name = stack.get("name").and_then(|v| v.as_str()).unwrap_or("").trim();
        if id.is_empty() || name.is_empty() {
            errors.push("stack requires id and name".to_string());
            continue;
        }
        if stack_ids.contains(id) {
            errors.push(format!("duplicate stack id: {id}"));
        }
        stack_ids.insert(id.to_string());

        let shared = stack.get("sharedPackageIds");
        let shared_arr = match shared.and_then(|v| v.as_array()) {
            Some(a) => a,
            None => {
                errors.push(format!("{id}: sharedPackageIds must be an array"));
                continue;
            }
        };
        for package_id in shared_arr {
            let pid = package_id.as_str().unwrap_or("").trim();
            if pid.is_empty() {
                errors.push(format!("{id}: invalid shared package id"));
                continue;
            }
            if !package_ids.contains(pid) {
                errors.push(format!("{id}: unknown stack package id: {pid}"));
            }
        }

        let variants = stack.get("variants").and_then(|v| v.as_array());
        let variants = match variants {
            Some(v) => v,
            None => {
                errors.push(format!("{id}: variants must be an array"));
                continue;
            }
        };
        for variant in variants {
            let tool_id = variant
                .get("toolId")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim();
            if tool_id.is_empty() {
                errors.push(format!("{id}: variant requires toolId"));
                continue;
            }
            if !tool_ids.contains(tool_id) {
                errors.push(format!("{id}: undeclared tool in stack variant: {tool_id}"));
            }
            let variant_packages = variant.get("packageIds").and_then(|v| v.as_array());
            let variant_packages = match variant_packages {
                Some(v) => v,
                None => {
                    errors.push(format!("{id}: variant packageIds must be an array"));
                    continue;
                }
            };
            for package_id in variant_packages {
                let pid = package_id.as_str().unwrap_or("").trim();
                if pid.is_empty() {
                    errors.push(format!("{id}: invalid variant package id"));
                    continue;
                }
                if !package_ids.contains(pid) {
                    errors.push(format!("{id}: unknown stack package id: {pid}"));
                }
            }
        }
    }

    if let Some(featured) = featured_stack_id {
        let featured = featured.trim();
        if featured.is_empty() {
            errors.push("featuredStackId must be a non-empty string".to_string());
        } else if !stack_ids.contains(featured) {
            errors.push(format!("unknown featured stack id: {featured}"));
        }
    }

    errors
}

pub fn load_marketplace_source(root: &Path) -> Result<Value, String> {
    let path = root.join("scripts/stacks.json");
    let raw = std::fs::read_to_string(&path)
        .map_err(|_| "missing scripts/stacks.json".to_string())?;
    let source: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("invalid scripts/stacks.json: {e}"))?;
    if !source.get("tools").and_then(|v| v.as_array()).is_some() {
        return Err("scripts/stacks.json tools must be an array".to_string());
    }
    if !source.get("stacks").and_then(|v| v.as_array()).is_some() {
        return Err("scripts/stacks.json stacks must be an array".to_string());
    }
    Ok(source)
}

pub fn slug_counts(entries: &[Value]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        if let Some(slug) = entry.get("slug").and_then(|v| v.as_str()) {
            counts.insert(slug.to_string(), counts.get(slug).copied().unwrap_or(0) + 1);
        }
    }
    counts
}

pub fn meta_file_name(entry: &Value, counts: &HashMap<String, usize>) -> String {
    let slug = entry.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    if counts.get(slug).copied().unwrap_or(0) <= 1 {
        "s_e_e_package.json".to_string()
    } else {
        let id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("missing");
        format!("s_e_e_package.{id}.json")
    }
}

pub fn meta_path(root: &Path, entry: &Value, counts: &HashMap<String, usize>) -> PathBuf {
    let slug = entry.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    root.join("packages")
        .join(slug)
        .join(meta_file_name(entry, counts))
}

pub fn read_existing_meta(root: &Path, entry: &Value, counts: &HashMap<String, usize>) -> Option<Value> {
    let path = meta_path(root, entry, counts);
    if !path.is_file() {
        return None;
    }
    let raw = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&raw).ok()
}
