use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::Duration;

use see_library_site::documentation_route_paths;
use tempfile::TempDir;

use crate::pages_artifact::{
    run_package_pages, smoke_pages_artifact, verify_staged_bytes, DEFAULT_PAGES_BASE,
};
use crate::verify_docs::{fixture_authoring_index_html, fixture_schema_index_html};

fn library_fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn write_minimal_prerender(dx_public: &Path) {
    let authoring = fixture_authoring_index_html();
    let schema = fixture_schema_index_html();
    let site = dx_public.join(DEFAULT_PAGES_BASE);
    for route in documentation_route_paths() {
        let dir = site.join(route.trim_start_matches('/'));
        fs::create_dir_all(&dir).expect("mkdir route");
        let body = if route == "/authoring/" {
            authoring.as_str()
        } else if route == "/schema/" {
            schema.as_str()
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

fn copy_schemas_and_logos(root: &Path, dest_public: &Path) {
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

struct LocalHttpd {
    child: Child,
    base_url: String,
}

impl LocalHttpd {
    fn start(root: &Path) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().expect("addr").port();
        drop(listener);
        let port_arg = port.to_string();
        let root_arg = root.to_string_lossy().to_string();
        let child = Command::new("busybox")
            .args(["httpd", "-f", "-p", &port_arg, "-h", &root_arg])
            .spawn()
            .expect("start httpd");
        for _ in 0..50 {
            if reqwest::blocking::get(format!("http://127.0.0.1:{port}/"))
                .is_ok()
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        Self {
            child,
            base_url: format!("http://127.0.0.1:{port}"),
        }
    }
}

impl Drop for LocalHttpd {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn package_pages_fails_when_catalog_missing() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    let dx = root.join("dx");
    write_minimal_prerender(&dx);
    copy_schemas_and_logos(root, &root.join("public"));
    fs::create_dir_all(root.join("packages")).expect("packages");
    fs::write(root.join("public/llms.txt"), "test\n").expect("llms");
    let staging = root.join("artifact");
    let err = run_package_pages(root, &dx, &staging).expect_err("missing catalog");
    assert!(err.contains("catalog.json"), "err was {err}");
    assert!(!staging.exists(), "staging must not be created on failure");
}

#[test]
fn second_package_run_drops_removed_package_file() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    fs::copy(
        library_fixture_root().join("catalog.json"),
        root.join("catalog.json"),
    )
    .expect("catalog");
    copy_schemas_and_logos(root, &root.join("public"));
    fs::create_dir_all(root.join("packages/stale-pkg/1.0.0")).expect("pkg dir");
    fs::write(root.join("packages/stale-pkg/1.0.0/file.txt"), "v1").expect("file");
    let dx = root.join("dx");
    write_minimal_prerender(&dx);
    let staging = root.join("artifact");
    run_package_pages(root, &dx, &staging).expect("first package");
    assert!(staging
        .join("packages/stale-pkg/1.0.0/file.txt")
        .is_file());
    fs::remove_dir_all(root.join("packages/stale-pkg")).expect("remove source pkg");
    run_package_pages(root, &dx, &staging).expect("second package");
    assert!(
        !staging.join("packages/stale-pkg").exists(),
        "stale package directory must not remain"
    );
}

#[test]
fn package_pages_and_smoke_against_real_library_tree() {
    let root = library_fixture_root();
    let dx = root.join("target/dx/wasm32-unknown-unknown/release/web/public");
    if !dx.join(DEFAULT_PAGES_BASE).is_dir() {
        eprintln!("skip package_pages_and_smoke: run mise run site-build first");
        return;
    }
    let tmp = TempDir::new().expect("tempdir");
    let staging = tmp.path().join("artifact");
    run_package_pages(&root, &dx, &staging).expect("package pages");
    verify_staged_bytes(&root, &staging).expect("byte identity");
    let server = LocalHttpd::start(&staging);
    smoke_pages_artifact(&root, &staging, &server.base_url).expect("smoke");
}
