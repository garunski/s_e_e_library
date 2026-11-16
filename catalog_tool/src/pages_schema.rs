use std::{fs, path::Path};

const REQUIRED: &[&str] = &[
    "workflow.schema.json",
    "workflow-execution.schema.json",
    "execution-error.schema.json",
    "app-settings.schema.json",
    "hub-config.schema.json",
    "knowledge-research.schema.json",
];

fn files(root: &Path) -> Result<Vec<String>, String> {
    let dir = root.join("public/schema");
    let mut names = Vec::new();
    for entry in fs::read_dir(&dir).map_err(|e| format!("read {dir:?}: {e}"))? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            names.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }
    for name in REQUIRED {
        if !names.iter().any(|found| found == name) {
            return Err(format!("missing required schema {name}"));
        }
    }
    names.sort();
    Ok(names)
}

fn read_schema(path: &Path) -> Result<Vec<u8>, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {path:?}: {e}"))?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid schema JSON {path:?}: {e}"))?;
    if !value.is_object() || !value["$schema"].is_string() {
        return Err(format!("missing schema object or dialect: {path:?}"));
    }
    Ok(bytes)
}

pub(super) fn verify(root: &Path, staging: &Path) -> Result<(), String> {
    for name in files(root)? {
        let source = read_schema(&root.join("public/schema").join(&name))?;
        let staged = read_schema(&staging.join("schema").join(&name))?;
        if source != staged {
            return Err(format!("staged schema bytes differ: {name}"));
        }
    }
    Ok(())
}

pub(super) fn smoke(
    client: &reqwest::blocking::Client,
    root: &Path,
    base: &str,
) -> Result<(), String> {
    for name in files(root)? {
        let url = format!("{base}/schema/{name}");
        let bytes = client
            .get(&url)
            .send()
            .and_then(|r| r.error_for_status())
            .map_err(|e| format!("GET {url}: {e}"))?
            .bytes()
            .map_err(|e| e.to_string())?;
        if bytes.as_ref() != read_schema(&root.join("public/schema").join(name))? {
            return Err(format!("HTTP schema bytes differ: {url}"));
        }
    }
    for path in [
        "/",
        "/llms.txt",
        "/authoring/",
        "/authoring/research/",
        "/schema/",
        "/docs/",
    ] {
        let url = format!("{base}{path}");
        let status = client
            .get(&url)
            .send()
            .map_err(|e| format!("GET {url}: {e}"))?
            .status();
        if status != reqwest::StatusCode::NOT_FOUND {
            return Err(format!("retired route {url}: expected 404, got {status}"));
        }
    }
    Ok(())
}
