use std::fs;
use std::path::Path;

use serde_json::Value;
use tempfile::TempDir;

use crate::harvest::{package_signatures_from_catalog, run_harvest};
use crate::package_meta::catalog_without_updated;
use crate::validate::{read_catalog, run_validate};

fn library_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn hub_root() -> std::path::PathBuf {
    library_root().parent().unwrap().join("s_e_e_project")
}

fn copy_dir_filtered(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == "node_modules" || name == "target" || name == ".git" || name == "out" || name == ".next" {
            continue;
        }
        let src = entry.path();
        let dst = to.join(name);
        if src.is_dir() {
            copy_dir_filtered(&src, &dst)?;
        } else {
            fs::copy(src, dst)?;
        }
    }
    Ok(())
}

#[test]
fn harvest_matches_official_package_signatures() {
    let hub = hub_root();
    if !hub.is_dir() {
        return;
    }
    let reference: Value =
        serde_json::from_str(&fs::read_to_string(library_root().join("catalog.json")).expect("catalog"))
            .expect("json");
    let expected = package_signatures_from_catalog(&reference);

    let tmp = TempDir::new().expect("tmpdir");
    let root = tmp.path();
    copy_dir_filtered(&library_root(), root).expect("copy library");
    run_harvest(root, &hub).expect("harvest");

    let harvested: Value =
        serde_json::from_str(&fs::read_to_string(root.join("catalog.json")).expect("catalog"))
            .expect("json");
    let actual = package_signatures_from_catalog(&harvested);
    assert_eq!(expected, actual);
    run_validate(root).expect("validate after harvest");
}

#[test]
fn harvest_missing_hub_leaves_catalog_unchanged() {
    let tmp = TempDir::new().expect("tmpdir");
    let root = tmp.path();
    copy_dir_filtered(&library_root(), root).expect("copy");
    let before = fs::read_to_string(root.join("catalog.json")).expect("catalog");
    let missing = root.join("no-such-hub");
    let err = run_harvest(root, &missing).expect_err("missing hub");
    assert!(err.contains("missing hub"));
    let after = fs::read_to_string(root.join("catalog.json")).expect("catalog");
    assert_eq!(before, after);
}

#[test]
fn harvest_malformed_hub_json_leaves_catalog_unchanged() {
    let hub = hub_root();
    if !hub.is_dir() {
        return;
    }
    let tmp = TempDir::new().expect("tmpdir");
    let root = tmp.path();
    copy_dir_filtered(&library_root(), root).expect("copy library");
    let before = fs::read_to_string(root.join("catalog.json")).expect("catalog");

    let hub_tmp = TempDir::new().expect("hub tmp");
    copy_dir_filtered(&hub, hub_tmp.path()).expect("copy hub");
    let bad = hub_tmp.path().join(".s_e_e/prompts/system-implement-story.json");
    fs::write(&bad, "{").expect("broken json");

    let err = run_harvest(root, hub_tmp.path()).expect_err("bad json");
    assert!(err.contains("invalid JSON"));
    let after = fs::read_to_string(root.join("catalog.json")).expect("catalog");
    assert_eq!(before, after);
}

#[test]
fn discover_bundles_does_not_modify_bundles_json_without_write_flag() {
    let root = library_root();
    let path = root.join("scripts/bundles.json");
    let before = fs::read_to_string(&path).expect("bundles");
    crate::discover_bundles::run_discover_bundles(&root, false).expect("discover");
    let after = fs::read_to_string(&path).expect("bundles");
    assert_eq!(before, after);
}

#[test]
fn catalog_without_updated_ignores_timestamp() {
    let catalog = read_catalog(&library_root()).expect("read");
    let stripped = catalog_without_updated(&catalog);
    assert!(stripped.get("updated").is_none());
}
