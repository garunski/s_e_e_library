use std::fs;
use std::path::{Path, PathBuf};

use s_e_e_library_ops::library_catalog::parse_manifest;
use serde_json::json;
use tempfile::TempDir;

use crate::validate::{build_next_catalog, read_catalog, run_catalog, run_validate};

fn library_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn validate_succeeds_on_official_library_checkout() {
    run_validate(&library_root()).expect("official library validate");
}

#[test]
fn invalid_cases_return_path_and_reason() {
    let tmp = TempDir::new().expect("tmpdir");
    let root = tmp.path();
    fs::create_dir_all(root.join("packages/bad-pkg/1.0.0")).expect("mkdir");
    fs::create_dir_all(root.join("scripts")).expect("scripts dir");
    fs::write(
        root.join("scripts/stacks.json"),
        r#"{"tools":[],"stacks":[],"featuredStackId":null}"#,
    )
    .expect("stacks");
    fs::write(
        root.join("packages/bad-pkg/s_e_e_package.json"),
        r#"{"id":"bad-pkg","slug":"bad-pkg","category":"prompt","name":"Bad","labels":[],"dependencies":[],"toolIds":[],"releases":[{"version":"1.0.0","date":"2026-09-01","note":"x"}]}"#,
    )
    .expect("meta");
    fs::write(root.join("packages/bad-pkg/1.0.0/prompt.json"), r#"{"id":"","name":""}"#)
        .expect("payload");

    let catalog = json!({
        "schema": "see.library/v2",
        "name": "Test",
        "updated": "2026-09-01T00:00:00.000Z",
        "packages": [{
            "id": "bad-pkg",
            "slug": "bad-pkg",
            "category": "prompt",
            "version": "1.0.0",
            "author": "test",
            "verified": false,
            "license": "AGPL-3.0-only",
            "requires": [],
            "dependencies": [],
            "files": [{ "to": ".s_e_e/prompts/bad.json", "from": "packages/bad-pkg/1.0.0/prompt.json" }]
        }]
    });
    fs::write(
        root.join("catalog.json"),
        serde_json::to_string_pretty(&catalog).expect("json"),
    )
    .expect("catalog");
    let err = run_validate(root).expect_err("invalid metadata");
    assert!(err.contains("bad-pkg"));

    fs::write(root.join("catalog.json"), "{").expect("broken catalog");
    let err = read_catalog(root).expect_err("malformed");
    assert!(err.contains("catalog.json"));

    let catalog = json!({
        "schema": "see.library/v2",
        "name": "Test",
        "updated": "2026-09-01T00:00:00.000Z",
        "packages": [{
            "id": "missing-file",
            "slug": "missing-file",
            "category": "prompt",
            "version": "1.0.0",
            "author": "test",
            "verified": false,
            "license": "AGPL-3.0-only",
            "requires": [],
            "dependencies": [],
            "files": [{ "to": ".s_e_e/prompts/x.json", "from": "packages/missing-file/1.0.0/prompt.json" }]
        }]
    });
    fs::write(
        root.join("catalog.json"),
        serde_json::to_string_pretty(&catalog).expect("json"),
    )
    .expect("catalog");
    fs::create_dir_all(root.join("packages/missing-file")).expect("mkdir missing");
    fs::write(
        root.join("packages/missing-file/s_e_e_package.json"),
        r#"{"id":"missing-file","slug":"missing-file","category":"prompt","name":"X","labels":[],"dependencies":[],"toolIds":[],"releases":[{"version":"1.0.0","date":"2026-09-01","note":"x"}]}"#,
    )
    .expect("meta");
    let err = run_validate(root).expect_err("missing payload");
    assert!(err.contains("missing payload file"));

    let catalog = json!({
        "schema": "see.library/v2",
        "name": "Test",
        "updated": "2026-09-01T00:00:00.000Z",
        "packages": [{
            "id": "bad-dest",
            "slug": "bad-dest",
            "category": "command",
            "version": "1.0.0",
            "author": "test",
            "verified": false,
            "license": "AGPL-3.0-only",
            "requires": [],
            "dependencies": [],
            "files": [{ "to": "templates/cmd.json", "from": "packages/bad-dest/1.0.0/cmd.json" }]
        }]
    });
    fs::create_dir_all(root.join("packages/bad-dest/1.0.0")).expect("mkdir");
    fs::write(
        root.join("packages/bad-dest/s_e_e_package.json"),
        r#"{"id":"bad-dest","slug":"bad-dest","category":"command","name":"X","labels":[],"dependencies":[],"toolIds":[],"releases":[{"version":"1.0.0","date":"2026-09-01","note":"x"}]}"#,
    )
    .expect("meta");
    fs::write(
        root.join("packages/bad-dest/1.0.0/cmd.json"),
        r#"{"id":"bad-dest","name":"X","binary":"echo"}"#,
    )
    .expect("payload");
    fs::write(
        root.join("catalog.json"),
        serde_json::to_string_pretty(&catalog).expect("json"),
    )
    .expect("catalog");
    let err = run_validate(root).expect_err("bad install path");
    assert!(err.contains("install path"));
}

#[test]
fn invalid_package_metadata_returns_path_and_reason() {
    let tmp = TempDir::new().expect("tmpdir");
    let root = tmp.path();
    fs::create_dir_all(root.join("scripts")).expect("scripts");
    fs::create_dir_all(root.join("packages/bad-pkg/1.0.0")).expect("pkg");
    fs::write(
        root.join("scripts/stacks.json"),
        r#"{"tools":[],"stacks":[],"featuredStackId":null}"#,
    )
    .expect("stacks");
    fs::write(
        root.join("packages/bad-pkg/1.0.0/prompt.json"),
        r#"{"id":"bad-pkg","name":"Bad"}"#,
    )
    .expect("payload");
    fs::write(
        root.join("packages/bad-pkg/s_e_e_package.json"),
        r#"{"id":"bad-pkg","slug":"bad-pkg","category":"prompt","name":"Bad","labels":[],"dependencies":[]}"#,
    )
    .expect("meta");
    let catalog = json!({
        "schema": "see.library/v2",
        "name": "Test",
        "updated": "2026-09-01T00:00:00.000Z",
        "packages": [{
            "id": "bad-pkg",
            "slug": "bad-pkg",
            "category": "prompt",
            "version": "1.0.0",
            "author": "test",
            "verified": false,
            "license": "AGPL-3.0-only",
            "requires": [],
            "dependencies": [],
            "files": [{ "to": ".s_e_e/prompts/bad.json", "from": "packages/bad-pkg/1.0.0/prompt.json" }]
        }]
    });
    fs::write(
        root.join("catalog.json"),
        serde_json::to_string_pretty(&catalog).expect("json"),
    )
    .expect("catalog");
    let err = run_validate(root).expect_err("invalid package metadata");
    assert!(err.contains("bad-pkg"));
    assert!(err.contains("toolIds"));
}

#[test]
fn catalog_command_preserves_manifest_fields_except_updated() {
    let root = library_root();
    let before: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join("catalog.json")).expect("catalog"))
            .expect("json");
    let before_packages = before.get("packages").cloned().expect("packages");
    let before_without_updated = {
        let mut copy = before.clone();
        copy.as_object_mut().unwrap().remove("updated");
        copy
    };

    let tmp = TempDir::new().expect("tmpdir");
    let copy_root = tmp.path();
    copy_dir_filtered(&root, copy_root).expect("copy tree");

    run_catalog(copy_root).expect("catalog");
    let after: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(copy_root.join("catalog.json")).expect("catalog"))
            .expect("json");
    let after_without_updated = {
        let mut copy = after.clone();
        copy.as_object_mut().unwrap().remove("updated");
        copy
    };

    assert_eq!(before_packages, after.get("packages").cloned().expect("packages"));
    assert_eq!(
        serde_json::to_string(&before_without_updated).expect("serialize"),
        serde_json::to_string(&after_without_updated).expect("serialize")
    );
}

#[test]
fn app_catalog_parser_accepts_rust_generated_manifest() {
    let root = library_root();
    let before_count = read_catalog(&root)
        .expect("read")
        .get("packages")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .expect("count");

    let tmp = TempDir::new().expect("tmpdir");
    let copy_root = tmp.path();
    copy_dir_filtered(&root, copy_root).expect("copy");
    run_catalog(copy_root).expect("catalog");
    let body = fs::read_to_string(copy_root.join("catalog.json")).expect("catalog");
    let manifest = parse_manifest("generated", &body).expect("parse");
    assert_eq!(manifest.packages.len(), before_count);
}

fn copy_dir_filtered(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == "node_modules"
            || name == ".git"
            || name == "out"
            || name == ".next"
            || name == "target"
        {
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
fn build_next_catalog_mutates_entries_like_node() {
    let root = library_root();
    let mut catalog = read_catalog(&root).expect("read");
    build_next_catalog(&root, &mut catalog).expect("build");
}
