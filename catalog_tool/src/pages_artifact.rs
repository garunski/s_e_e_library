use std::fs;
use std::path::{Path, PathBuf};

use see_library_site::documentation_route_paths;
use s_e_e_library_ops::library_catalog::{parse_manifest, resolve_manifest_url};
use s_e_e_shapes::{CatalogSource, CatalogSourceKind};

pub const DEFAULT_PAGES_BASE: &str = "s_e_e_library";

pub fn default_dx_public(root: &Path) -> PathBuf {
    root.join("target/dx/wasm32-unknown-unknown/release/web/public")
}

pub fn default_staging(root: &Path) -> PathBuf {
    root.join("target/pages-artifact")
}

fn require_path(path: &Path, label: &str) -> Result<(), String> {
    if path.exists() {
        Ok(())
    } else {
        Err(format!("missing required {label}: {path:?}"))
    }
}

fn copy_file_same_bytes(src: &Path, dst: &Path) -> Result<(), String> {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {parent:?}: {e}"))?;
    }
    fs::copy(src, dst).map_err(|e| format!("copy {src:?} -> {dst:?}: {e}"))?;
    let a = fs::read(src).map_err(|e| e.to_string())?;
    let b = fs::read(dst).map_err(|e| e.to_string())?;
    if a != b {
        return Err(format!("byte mismatch after copy: {dst:?}"));
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if !src.is_dir() {
        return Err(format!("not a directory: {src:?}"));
    }
    fs::create_dir_all(dst).map_err(|e| format!("mkdir {dst:?}: {e}"))?;
    for entry in fs::read_dir(src).map_err(|e| format!("read_dir {src:?}: {e}"))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let from = entry.path();
        let to = dst.join(name);
        if from.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            copy_file_same_bytes(&from, &to)?;
        }
    }
    Ok(())
}

fn remove_dir_if_exists(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path).map_err(|e| format!("remove {path:?}: {e}"))?;
    }
    Ok(())
}

fn schema_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let schema_dir = root.join("public/schema");
    require_path(&schema_dir, "public/schema directory")?;
    let mut files: Vec<PathBuf> = fs::read_dir(&schema_dir)
        .map_err(|e| format!("read_dir {schema_dir:?}: {e}"))?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().is_some_and(|ext| ext == "json")
                    && path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(".schema.json"))
                {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();
    files.sort();
    if files.len() != 18 {
        return Err(format!(
            "expected 18 public/schema/*.schema.json files, found {}",
            files.len()
        ));
    }
    Ok(files)
}

fn logo_sources(root: &Path) -> Vec<PathBuf> {
    [
        "logo.svg",
        "logo-32.png",
        "logo-64.png",
        "logo-128.png",
        "logo-256.png",
    ]
    .into_iter()
    .map(|name| root.join("public").join(name))
    .collect()
}

/// Stage Dioxus prerender output and repository payloads into one GitHub Pages artifact tree.
pub fn run_package_pages(root: &Path, dx_public: &Path, staging: &Path) -> Result<(), String> {
    let site_src = dx_public.join(DEFAULT_PAGES_BASE);
    require_path(&site_src, "Dioxus public site directory")?;
    require_path(&root.join("catalog.json"), "catalog.json")?;
    require_path(&root.join("packages"), "packages directory")?;
    require_path(&root.join("public/llms.txt"), "public/llms.txt")?;
    let schemas = schema_sources(root)?;
    for logo in logo_sources(root) {
        require_path(&logo, "logo asset")?;
    }

    remove_dir_if_exists(staging)?;
    fs::create_dir_all(staging).map_err(|e| format!("mkdir {staging:?}: {e}"))?;

    copy_dir_recursive(&site_src, staging)?;
    let assets_src = dx_public.join("assets");
    if assets_src.is_dir() {
        copy_dir_recursive(&assets_src, &staging.join("assets"))?;
    }

    copy_file_same_bytes(&root.join("catalog.json"), &staging.join("catalog.json"))?;
    copy_dir_recursive(&root.join("packages"), &staging.join("packages"))?;
    copy_file_same_bytes(&root.join("public/llms.txt"), &staging.join("llms.txt"))?;
    for schema in &schemas {
        let name = schema
            .file_name()
            .ok_or_else(|| format!("schema path has no file name: {schema:?}"))?;
        copy_file_same_bytes(schema, &staging.join("schema").join(name))?;
    }
    for logo in logo_sources(root) {
        let name = logo.file_name().ok_or_else(|| format!("logo path: {logo:?}"))?;
        copy_file_same_bytes(&logo, &staging.join(name))?;
    }
    fs::write(staging.join(".nojekyll"), "")
        .map_err(|e| format!("write .nojekyll: {e}"))?;

    verify_staged_bytes(root, staging)?;
    crate::verify_docs::verify_staged_docs(root, staging)?;
    Ok(())
}

pub fn verify_documentation_routes(staging: &Path) -> Result<(), String> {
    let routes = documentation_route_paths();
    if routes.len() != 25 {
        return Err(format!("expected 25 documentation routes, got {}", routes.len()));
    }
    for route in routes {
        let rel = route.trim_start_matches('/');
        let index = staging.join(rel).join("index.html");
        if !index.is_file() {
            return Err(format!("missing prerendered page: {index:?}"));
        }
    }
    let not_found = staging.join("404.html");
    if !not_found.is_file() {
        return Err("missing 404.html in staged artifact".into());
    }
    Ok(())
}

pub fn verify_staged_bytes(root: &Path, staging: &Path) -> Result<(), String> {
    compare_file(&root.join("catalog.json"), &staging.join("catalog.json"))?;
    compare_file(&root.join("public/llms.txt"), &staging.join("llms.txt"))?;
    for schema in schema_sources(root)? {
        let name = schema.file_name().expect("schema name");
        compare_file(&schema, &staging.join("schema").join(name))?;
    }
    compare_tree(&root.join("packages"), &staging.join("packages"))?;
    Ok(())
}

fn compare_file(src: &Path, staged: &Path) -> Result<(), String> {
    let a = fs::read(src).map_err(|e| format!("read {src:?}: {e}"))?;
    let b = fs::read(staged).map_err(|e| format!("read {staged:?}: {e}"))?;
    if a != b {
        return Err(format!("staged bytes differ from source: {staged:?}"));
    }
    Ok(())
}

fn compare_tree(src: &Path, staged: &Path) -> Result<(), String> {
    if !src.is_dir() {
        return Err(format!("not a directory: {src:?}"));
    }
    for entry in fs::read_dir(src).map_err(|e| format!("read_dir {src:?}: {e}"))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let from = entry.path();
        let rel = from.strip_prefix(src).map_err(|e| e.to_string())?;
        let to = staged.join(rel);
        if from.is_dir() {
            compare_tree(&from, &to)?;
        } else {
            compare_file(&from, &to)?;
        }
    }
    Ok(())
}

pub fn smoke_pages_artifact(
    library_root: &Path,
    staging: &Path,
    base_url: &str,
) -> Result<(), String> {
    verify_documentation_routes(staging)?;
    let base = base_url.trim_end_matches('/');
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;
    for route in documentation_route_paths() {
        let url = if route == "/" {
            format!("{base}/")
        } else {
            format!("{base}{}", route.trim_end_matches('/'))
        };
        let body = client
            .get(&url)
            .send()
            .map_err(|e| format!("GET {url}: {e}"))?
            .error_for_status()
            .map_err(|e| format!("GET {url}: {e}"))?
            .text()
            .map_err(|e| format!("read {url}: {e}"))?;
        if !body.contains("<title>") {
            return Err(format!("documentation page missing title: {url}"));
        }
    }
    let unknown = format!("{base}/__artifact-smoke-unknown-path__/");
    let unknown_body = client
        .get(&unknown)
        .send()
        .map_err(|e| format!("GET {unknown}: {e}"))?
        .text()
        .map_err(|e| format!("read {unknown}: {e}"))?;
    if unknown_body.to_ascii_lowercase().contains("not found") {
        // dedicated not-found route or server 404 page
    } else {
        let not_found_file = fs::read_to_string(staging.join("404.html"))
            .map_err(|e| format!("read staged 404.html: {e}"))?;
        if !not_found_file.to_ascii_lowercase().contains("not found") {
            return Err("staged 404.html missing not-found copy".into());
        }
    }

    let catalog_bytes = fs::read(staging.join("catalog.json")).map_err(|e| e.to_string())?;
    let catalog_url = format!("{base}/catalog.json");
    let fetched = client
        .get(&catalog_url)
        .send()
        .map_err(|e| format!("GET {catalog_url}: {e}"))?
        .error_for_status()
        .map_err(|e| format!("GET {catalog_url}: {e}"))?
        .bytes()
        .map_err(|e| format!("read {catalog_url}: {e}"))?;
    if fetched.as_ref() != catalog_bytes {
        return Err("staged catalog.json HTTP body differs from artifact file".into());
    }

    smoke_catalog_parser_and_package(library_root, base, &catalog_bytes)?;
    Ok(())
}

fn smoke_catalog_parser_and_package(
    library_root: &Path,
    base_url: &str,
    catalog_body: &[u8],
) -> Result<(), String> {
    let source = CatalogSource {
        id: "smoke".into(),
        name: "smoke".into(),
        kind: CatalogSourceKind::GithubPages,
        url: format!("{}/", base_url.trim_end_matches('/')),
        repo: "smoke/smoke".into(),
        manifest: "catalog.json".into(),
        branch: None,
        enabled: true,
        primary: true,
        verified: false,
        last_sync: "never".into(),
        package_count: 0,
        description: None,
    };
    let body = std::str::from_utf8(catalog_body)
        .map_err(|_| "catalog.json is not valid UTF-8".to_string())?;
    let manifest = parse_manifest(&source.id, body)?;
    let manifest_url = resolve_manifest_url(&source)?;
    if manifest_url != format!("{}/catalog.json", base_url.trim_end_matches('/')) {
        return Err(format!(
            "unexpected manifest url {manifest_url} for smoke base {base_url}"
        ));
    }
    let first = manifest
        .packages
        .first()
        .ok_or_else(|| "catalog has no packages".to_string())?;
    let from = first
        .files
        .first()
        .map(|f| f.from.clone())
        .ok_or_else(|| "first package has no files".to_string())?;
    let file_url = s_e_e_library_ops::resolve_package_file_url(&source, &from)?;
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;
    let rel = from.trim_start_matches('/');
    let on_disk = fs::read(library_root.join(rel));
    let fetched = client
        .get(&file_url)
        .send()
        .map_err(|e| format!("GET {file_url}: {e}"))?
        .error_for_status()
        .map_err(|e| format!("GET {file_url}: {e}"))?
        .bytes()
        .map_err(|e| format!("read {file_url}: {e}"))?;
    if let Ok(disk) = on_disk {
        if fetched.as_ref() != disk {
            return Err(format!("package file bytes differ for {from}"));
        }
    }
    Ok(())
}
