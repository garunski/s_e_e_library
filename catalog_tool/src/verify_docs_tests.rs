use std::fs;

use see_library_site::documentation_route_paths;
use tempfile::TempDir;

use crate::pages_artifact::{run_package_pages, DEFAULT_PAGES_BASE};
use crate::verify_docs::{
    extract_hrefs, llms_source_paths, normalize_href_path, run_verify_docs, verify_authoring_index_links,
    verify_internal_links, verify_llms_sections, verify_schema_index_links, verify_staged_docs,
    verify_staged_schema_downloads, PAGES_BASE_PREFIX,
};

fn library_fixture_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn write_minimal_prerender(dx_public: &std::path::Path, authoring_html: &str, schema_html: &str) {
    let site = dx_public.join(DEFAULT_PAGES_BASE);
    for route in documentation_route_paths() {
        let dir = site.join(route.trim_start_matches('/'));
        fs::create_dir_all(&dir).expect("mkdir route");
        let body = if route == "/authoring/" {
            authoring_html
        } else if route == "/schema/" {
            schema_html
        } else {
            "<html><head><title>test</title></head><body>ok</body></html>"
        };
        fs::write(dir.join("index.html"), body).expect("write html");
    }
    fs::write(
        site.join("404.html"),
        "<html><body>not found</body></html>",
    )
    .expect("404");
}

fn copy_schemas_and_logos(root: &std::path::Path, dest_public: &std::path::Path) {
    let src_public = library_fixture_root().join("public");
    fs::create_dir_all(dest_public.join("schema")).expect("schema dir");
    for entry in fs::read_dir(src_public.join("schema")).expect("read schemas") {
        let path = entry.expect("entry").path();
        if path.extension().is_some_and(|e| e == "json") {
            let name = path.file_name().expect("name");
            fs::copy(&path, dest_public.join("schema").join(name)).expect("copy schema");
        }
    }
    for name in [
        "llms.txt",
        "logo.svg",
        "logo-32.png",
        "logo-64.png",
        "logo-128.png",
        "logo-256.png",
    ] {
        fs::copy(src_public.join(name), dest_public.join(name)).expect("copy public asset");
    }
}

use crate::verify_docs::{fixture_authoring_index_html, fixture_schema_index_html};

#[test]
fn llms_source_path_count_is_twenty_four() {
    assert_eq!(llms_source_paths().len(), 24);
}

#[test]
fn verify_llms_sections_fails_with_missing_source_path() {
    let tmp = TempDir::new().expect("tempdir");
    let llms = tmp.path().join("llms.txt");
    fs::write(&llms, "Source: /authoring/\n").expect("write");
    let errors = verify_llms_sections(&llms);
    assert!(errors.iter().any(|e| e.contains("/schema/workflow/")));
    assert!(errors.iter().any(|e| e.contains("llms.txt")));
}

#[test]
fn verify_authoring_index_requires_six_guides_and_publish() {
    let errors = verify_authoring_index_links("<html></html>");
    assert_eq!(errors.len(), 7);
    let html = fixture_authoring_index_html();
    assert!(verify_authoring_index_links(&html).is_empty());
}

#[test]
fn verify_schema_index_requires_fifteen_detail_links() {
    let errors = verify_schema_index_links("<html></html>");
    assert_eq!(errors.len(), 15);
    assert!(verify_schema_index_links(&fixture_schema_index_html()).is_empty());
}

#[test]
fn removing_staged_route_html_fails_verify_with_path() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    fs::copy(
        library_fixture_root().join("catalog.json"),
        root.join("catalog.json"),
    )
    .expect("catalog");
    copy_schemas_and_logos(root, &root.join("public"));
    fs::create_dir_all(root.join("packages")).expect("packages");
    let dx = root.join("dx");
    write_minimal_prerender(&dx, &fixture_authoring_index_html(), &fixture_schema_index_html());
    let staging = root.join("artifact");
    run_package_pages(root, &dx, &staging).expect("package");
    let missing = staging.join("authoring/workflows/index.html");
    fs::remove_file(&missing).expect("remove route");
    let err = verify_staged_docs(root, &staging).expect_err("verify");
    assert!(
        err.contains("authoring/workflows") || err.contains("missing prerendered"),
        "err was {err}"
    );
}

#[test]
fn removing_staged_schema_download_fails_verify_with_path() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    fs::copy(
        library_fixture_root().join("catalog.json"),
        root.join("catalog.json"),
    )
    .expect("catalog");
    copy_schemas_and_logos(root, &root.join("public"));
    fs::create_dir_all(root.join("packages")).expect("packages");
    let dx = root.join("dx");
    write_minimal_prerender(&dx, &fixture_authoring_index_html(), &fixture_schema_index_html());
    let staging = root.join("artifact");
    run_package_pages(root, &dx, &staging).expect("package");
    let schema_file = staging.join("schema/workflow.schema.json");
    fs::remove_file(&schema_file).expect("remove schema");
    let errors = verify_staged_schema_downloads(&staging);
    assert!(errors.iter().any(|e| e.contains("18")));
}

#[test]
fn removing_llms_source_section_fails_verify_with_path() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    let llms_path = root.join("public/llms.txt");
    fs::create_dir_all(root.join("public")).expect("public");
    let full = fs::read_to_string(library_fixture_root().join("public/llms.txt")).expect("llms");
    let trimmed = full
        .lines()
        .filter(|line| !line.contains("Source: /schema/workflow/"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&llms_path, trimmed).expect("write trimmed llms");
    let errors = verify_llms_sections(&llms_path);
    assert!(errors.iter().any(|e| e.contains("/schema/workflow/")));
}

#[test]
fn broken_internal_anchor_reports_html_path() {
    let tmp = TempDir::new().expect("tempdir");
    let staging = tmp.path();
    fs::create_dir_all(staging.join("authoring")).expect("dir");
    fs::write(
        staging.join("authoring/index.html"),
        format!(
            "<a href=\"{}/__missing-target__/\">bad</a>",
            PAGES_BASE_PREFIX
        ),
    )
    .expect("html");
    fs::write(staging.join("index.html"), "<html></html>").expect("home");
    let errors = verify_internal_links(staging);
    assert!(errors.iter().any(|e| e.contains("authoring/index.html")));
    assert!(errors.iter().any(|e| e.contains("__missing-target__")));
}

#[test]
fn extract_hrefs_reads_quoted_attributes() {
    let hrefs = extract_hrefs("<a href=\"/s_e_e_library/\">x</a>");
    assert_eq!(hrefs, vec!["/s_e_e_library/"]);
}

#[test]
fn normalize_href_strips_pages_base() {
    assert_eq!(
        normalize_href_path("/s_e_e_library/catalog.json"),
        Some("catalog.json".into())
    );
}

#[test]
fn verify_staged_docs_passes_minimal_packaged_tree() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    fs::copy(
        library_fixture_root().join("catalog.json"),
        root.join("catalog.json"),
    )
    .expect("catalog");
    copy_schemas_and_logos(root, &root.join("public"));
    fs::create_dir_all(root.join("packages")).expect("packages");
    let dx = root.join("dx");
    write_minimal_prerender(&dx, &fixture_authoring_index_html(), &fixture_schema_index_html());
    let staging = root.join("artifact");
    run_package_pages(root, &dx, &staging).expect("package");
    verify_staged_docs(root, &staging).expect("verify docs");
}

#[test]
fn verify_docs_against_real_staged_artifact_when_present() {
    let root = library_fixture_root();
    let staging = root.join("target/pages-artifact");
    if !staging.join("index.html").is_file() {
        eprintln!("skip verify_docs_against_real_staged_artifact: run mise run site-package-pages");
        return;
    }
    run_verify_docs(&root, &staging).expect("verify staged Pages artifact");
}
