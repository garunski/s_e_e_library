use std::fs;
use std::path::{Path, PathBuf};

use see_library_site::{
    authoring_path, documentation_route_paths, schema_path, AUTHORING_SLUGS, SCHEMA_SLUGS,
};

use crate::pages_artifact::verify_documentation_routes;

/// Site path prefix used in prerendered HTML (`/s_e_e_library` on GitHub Pages).
pub const PAGES_BASE_PREFIX: &str = "/s_e_e_library";

fn with_pages_base(site_path: &str) -> String {
    let base = PAGES_BASE_PREFIX.trim_end_matches('/');
    if site_path == "/" {
        format!("{base}/")
    } else {
        format!("{base}{site_path}")
    }
}

/// Public documentation paths that must have a `Source:` line in llms.txt (24 entries).
pub fn llms_source_paths() -> Vec<&'static str> {
    let mut paths = Vec::with_capacity(24);
    paths.push("/authoring/");
    for slug in AUTHORING_SLUGS {
        paths.push(authoring_path(slug));
    }
    paths.push("/schema/");
    for slug in SCHEMA_SLUGS {
        paths.push(schema_path(slug));
    }
    paths
}

pub fn verify_llms_sections(llms_path: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    if !llms_path.is_file() {
        errors.push(format!("missing llms.txt: {llms_path:?}"));
        return errors;
    }
    let llms = fs::read_to_string(llms_path).map_err(|e| e.to_string());
    let llms = match llms {
        Ok(text) => text,
        Err(message) => {
            errors.push(format!("read {llms_path:?}: {message}"));
            return errors;
        }
    };
    for source in llms_source_paths() {
        let needle = format!("Source: {source}");
        if !llms.contains(&needle) {
            errors.push(format!(
                "{llms_path:?} has no section for {source} (expected \"{needle}\")"
            ));
        }
    }
    errors
}

pub fn verify_llms_semantics(llms_path: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let llms = match fs::read_to_string(llms_path) {
        Ok(text) => text,
        Err(e) => {
            errors.push(format!("read {llms_path:?}: {e}"));
            return errors;
        }
    };
    for phrase in [
        "S.E.E. Official Library",
        "see.library/v1",
        "usageContexts",
        "technologies[]",
        "Legacy classification inference",
        "Classification (required for new packages)",
    ] {
        if !llms.contains(phrase) {
            errors.push(format!("{llms_path:?} missing expected phrase: {phrase}"));
        }
    }
    for forbidden in ["v-pre", "maintained by hand", "/humans/"] {
        if llms.contains(forbidden) {
            errors.push(format!("{llms_path:?} contains stale copy: {forbidden}"));
        }
    }
    errors
}

pub fn verify_authoring_index_links(html: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for slug in AUTHORING_SLUGS {
        if *slug == "publish" {
            continue;
        }
        let href = with_pages_base(authoring_path(slug));
        if !html.contains(&href) {
            errors.push(format!(
                "authoring/index.html does not link to /authoring/{slug}/ (expected href {href})"
            ));
        }
    }
    let publish_href = with_pages_base(authoring_path("publish"));
    if !html.contains(&publish_href) {
        errors.push(format!(
            "authoring/index.html does not link to publish (expected href {publish_href})"
        ));
    }
    errors
}

pub fn verify_schema_index_links(html: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for slug in SCHEMA_SLUGS {
        let href = with_pages_base(schema_path(slug));
        if !html.contains(&href) {
            errors.push(format!(
                "schema/index.html does not link to /schema/{slug}/ (expected href {href})"
            ));
        }
    }
    errors
}

pub fn extract_hrefs(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find("href=\"") {
        rest = &rest[start + 6..];
        if let Some(end) = rest.find('"') {
            let href = rest[..end].to_string();
            out.push(href);
            rest = &rest[end + 1..];
        } else {
            break;
        }
    }
    out
}

pub(crate) fn normalize_href_path(href: &str) -> Option<String> {
    if href.starts_with('#') {
        return None;
    }
    if href.starts_with("http://") || href.starts_with("https://") || href.starts_with("mailto:") {
        return None;
    }
    let path = href.split('#').next().unwrap_or(href);
    let path = if path.starts_with("/./") {
        path[2..].trim_start_matches('/')
    } else {
        path.trim_start_matches('/')
    };
    let base = PAGES_BASE_PREFIX.trim_start_matches('/').trim_end_matches('/');
    let rel = if path.starts_with(base) {
        path[base.len()..].trim_start_matches('/')
    } else {
        path
    };
    Some(rel.trim_end_matches('/').to_string())
}

fn staging_target_exists(staging: &Path, rel: &str) -> bool {
    if rel.is_empty() {
        return staging.join("index.html").is_file();
    }
    let direct = staging.join(rel);
    if direct.is_file() {
        return true;
    }
    if direct.join("index.html").is_file() {
        return true;
    }
    direct.is_dir()
}

pub fn verify_internal_links(staging: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let mut html_files: Vec<PathBuf> = Vec::new();
    for route in documentation_route_paths() {
        let rel = route.trim_start_matches('/');
        let index = if rel.is_empty() {
            staging.join("index.html")
        } else {
            staging.join(rel).join("index.html")
        };
        if index.is_file() {
            html_files.push(index);
        }
    }
    if staging.join("404.html").is_file() {
        html_files.push(staging.join("404.html"));
    }

    for html_path in html_files {
        let html = match fs::read_to_string(&html_path) {
            Ok(text) => text,
            Err(e) => {
                errors.push(format!("read {html_path:?}: {e}"));
                continue;
            }
        };
        for href in extract_hrefs(&html) {
            let rel = match normalize_href_path(&href) {
                Some(rel) => rel,
                None => continue,
            };
            if !staging_target_exists(staging, &rel) {
                errors.push(format!(
                    "broken internal link in {html_path:?}: href=\"{href}\" (no staged target for \"{rel}\")"
                ));
            }
        }
    }
    errors
}

pub fn verify_staged_schema_downloads(staging: &Path) -> Vec<String> {
    let schema_dir = staging.join("schema");
    if !schema_dir.is_dir() {
        return vec![format!("missing staged schema directory: {schema_dir:?}")];
    }
    let entries = match fs::read_dir(&schema_dir) {
        Ok(entries) => entries,
        Err(e) => return vec![format!("read_dir {schema_dir:?}: {e}")],
    };
    let mut files: Vec<PathBuf> = entries
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
        return vec![format!(
            "expected 18 staged schema/*.schema.json files, found {} under {schema_dir:?}",
            files.len()
        )];
    }
    Vec::new()
}

/// Full documentation and machine-text verification for a staged Pages artifact.
pub fn verify_staged_docs(root: &Path, staging: &Path) -> Result<(), String> {
    let mut errors: Vec<String> = Vec::new();

    if let Err(message) = verify_documentation_routes(staging) {
        errors.push(message);
    }

    let llms_repo = root.join("public/llms.txt");
    errors.extend(verify_llms_sections(&llms_repo));
    errors.extend(verify_llms_semantics(&llms_repo));

    let staged_llms = staging.join("llms.txt");
    errors.extend(verify_llms_sections(&staged_llms));

    errors.extend(verify_staged_schema_downloads(staging));

    let authoring_index = staging.join("authoring/index.html");
    if authoring_index.is_file() {
        match fs::read_to_string(&authoring_index) {
            Ok(html) => errors.extend(verify_authoring_index_links(&html)),
            Err(e) => errors.push(format!("read {authoring_index:?}: {e}")),
        }
    } else {
        errors.push(format!("missing prerendered page: {authoring_index:?}"));
    }

    let schema_index = staging.join("schema/index.html");
    if schema_index.is_file() {
        match fs::read_to_string(&schema_index) {
            Ok(html) => errors.extend(verify_schema_index_links(&html)),
            Err(e) => errors.push(format!("read {schema_index:?}: {e}")),
        }
    } else {
        errors.push(format!("missing prerendered page: {schema_index:?}"));
    }

    errors.extend(verify_internal_links(staging));

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

#[cfg(test)]
pub(crate) fn fixture_authoring_index_html() -> String {
    let mut html = String::from("<html><body>");
    for slug in ["workflows", "prompts", "skills", "commands", "bundles", "cycles"] {
        html.push_str(&format!(
            "<a href=\"{}/authoring/{}/\">{slug}</a>",
            PAGES_BASE_PREFIX,
            slug
        ));
    }
    html.push_str(&format!(
        "<a href=\"{}/authoring/publish/\">publish</a>",
        PAGES_BASE_PREFIX
    ));
    html.push_str("</body></html>");
    html
}

#[cfg(test)]
pub(crate) fn fixture_schema_index_html() -> String {
    let mut html = String::from("<html><body>");
    for slug in SCHEMA_SLUGS {
        html.push_str(&format!(
            "<a href=\"{}/schema/{}/\">{slug}</a>",
            PAGES_BASE_PREFIX,
            slug
        ));
    }
    html.push_str("</body></html>");
    html
}

pub fn run_verify_docs(root: &Path, staging: &Path) -> Result<(), String> {
    if !staging.join("index.html").is_file() {
        return Err(format!(
            "staged artifact missing index.html under {staging:?}; run site-package-pages first"
        ));
    }
    verify_staged_docs(root, staging)
}
