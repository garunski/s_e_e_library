use std::fs;
use std::path::{Path, PathBuf};

use s_e_e_library_ops::library_catalog::{parse_manifest, resolve_manifest_url};
use s_e_e_shapes::{CatalogSource, CatalogSourceKind};

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

/// Stage the catalog and payload tree for GitHub Pages.
pub fn run_package_pages(root: &Path, staging: &Path) -> Result<(), String> {
    require_path(&root.join("catalog.json"), "catalog.json")?;
    require_path(&root.join("packages"), "packages directory")?;

    remove_dir_if_exists(staging)?;
    fs::create_dir_all(staging).map_err(|e| format!("mkdir {staging:?}: {e}"))?;
    copy_file_same_bytes(&root.join("catalog.json"), &staging.join("catalog.json"))?;
    copy_dir_recursive(&root.join("packages"), &staging.join("packages"))?;
    copy_dir_recursive(&root.join("public/schema"), &staging.join("schema"))?;
    fs::write(staging.join(".nojekyll"), "").map_err(|e| format!("write .nojekyll: {e}"))?;
    verify_staged_bytes(root, staging)
}

pub fn verify_staged_bytes(root: &Path, staging: &Path) -> Result<(), String> {
    compare_file(&root.join("catalog.json"), &staging.join("catalog.json"))?;
    compare_tree(&root.join("packages"), &staging.join("packages"))?;
    crate::pages_schema::verify(root, staging)
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
    verify_staged_bytes(library_root, staging)?;
    let base = base_url.trim_end_matches('/');
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;
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

    smoke_catalog_parser_and_packages(&client, library_root, base, &catalog_bytes)?;
    crate::pages_schema::smoke(&client, library_root, base)
}

fn smoke_catalog_parser_and_packages(
    client: &reqwest::blocking::Client,
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
    let payloads: std::collections::BTreeSet<_> = manifest
        .packages
        .iter()
        .flat_map(|package| package.files.iter().map(|file| file.from.as_str()))
        .collect();
    if payloads.is_empty() {
        return Err("catalog has no payloads".into());
    }
    for from in payloads {
        let file_url = s_e_e_library_ops::resolve_package_file_url(&source, from)?;
        let on_disk = fs::read(library_root.join(from.trim_start_matches('/')))
            .map_err(|e| format!("read {from}: {e}"))?;
        let fetched = client
            .get(&file_url)
            .send()
            .map_err(|e| format!("GET {file_url}: {e}"))?
            .error_for_status()
            .map_err(|e| format!("GET {file_url}: {e}"))?
            .bytes()
            .map_err(|e| format!("read {file_url}: {e}"))?;
        if fetched.as_ref() != on_disk {
            return Err(format!("package file bytes differ for {from}"));
        }
    }
    Ok(())
}
