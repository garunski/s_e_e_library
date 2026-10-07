use std::path::PathBuf;

use super::{all_embedded_schemas, schema_download};

fn public_schema_path(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../public/schema")
        .join(file)
}

#[test]
fn embedded_schema_bytes_match_public_files() {
    for entry in all_embedded_schemas() {
        let disk = std::fs::read_to_string(public_schema_path(entry.filename))
            .unwrap_or_else(|e| panic!("read {}: {e}", entry.filename));
        assert_eq!(
            entry.json, disk,
            "embedded bytes must match public/schema/{}",
            entry.filename
        );
    }
}

#[test]
fn download_filenames_are_unique() {
    let mut names = all_embedded_schemas()
        .map(|e| e.filename)
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();
    let count = all_embedded_schemas().count();
    assert_eq!(names.len(), count);
    assert_eq!(count, 18);
}

#[test]
fn catalog_schema_id_matches_public_filename() {
    let entry = schema_download("catalog.schema.json").expect("catalog schema");
    assert!(entry.json.contains("catalog.schema.json"));
}

#[test]
fn package_schema_downloads_include_cycle_document() {
    assert!(schema_download("cycle-document.schema.json").is_some());
}

#[test]
fn embedded_schema_json_ids_reference_public_filenames() {
    for entry in all_embedded_schemas() {
        let stem = entry
            .filename
            .strip_suffix(".schema.json")
            .unwrap_or(entry.filename);
        assert!(
            entry.json.contains(entry.filename) || entry.json.contains(stem),
            "{} must embed its public $id or filename stem",
            entry.filename
        );
    }
}
