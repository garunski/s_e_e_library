use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;

use crate::pages_artifact::{run_package_pages, verify_staged_bytes};

fn library_fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn package_pages_fails_when_catalog_missing() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    fs::create_dir_all(root.join("packages")).expect("packages");
    let staging = root.join("artifact");
    let err = run_package_pages(root, &staging).expect_err("missing catalog");
    assert!(err.contains("catalog.json"), "err was {err}");
    assert!(!staging.exists(), "staging must not be created on failure");
}

#[test]
fn packaging_drops_removed_package_files() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    fs::copy(
        library_fixture_root().join("catalog.json"),
        root.join("catalog.json"),
    )
    .expect("catalog");
    fs::create_dir_all(root.join("packages/stale-pkg/1.0.0")).expect("pkg dir");
    fs::write(root.join("packages/stale-pkg/1.0.0/file.txt"), "v1").expect("file");
    fs::create_dir_all(root.join("public/schema")).unwrap();
    for entry in fs::read_dir(library_fixture_root().join("public/schema")).unwrap() {
        let entry = entry.unwrap();
        fs::copy(
            entry.path(),
            root.join("public/schema").join(entry.file_name()),
        )
        .unwrap();
    }
    let staging = root.join("artifact");
    run_package_pages(root, &staging).expect("first package");
    assert!(staging.join("packages/stale-pkg/1.0.0/file.txt").is_file());
    fs::remove_dir_all(root.join("packages/stale-pkg")).expect("remove source pkg");
    run_package_pages(root, &staging).expect("second package");
    assert!(
        !staging.join("packages/stale-pkg").exists(),
        "stale package directory must not remain"
    );
}

#[test]
fn package_pages_against_real_library_tree() {
    let root = library_fixture_root();
    let tmp = TempDir::new().expect("tempdir");
    let staging = tmp.path().join("artifact");
    run_package_pages(&root, &staging).expect("package pages");
    verify_staged_bytes(&root, &staging).expect("byte identity");
}

#[test]
fn staged_schema_corruption_and_removal_fail_verification() {
    let root = library_fixture_root();
    let tmp = TempDir::new().unwrap();
    let staging = tmp.path().join("artifact");
    run_package_pages(&root, &staging).unwrap();
    let path = staging.join("schema/app-settings.schema.json");
    fs::write(&path, "{").unwrap();
    assert!(verify_staged_bytes(&root, &staging)
        .unwrap_err()
        .contains("invalid schema JSON"));
    fs::remove_file(&path).unwrap();
    assert!(verify_staged_bytes(&root, &staging)
        .unwrap_err()
        .contains("app-settings.schema.json"));
}
