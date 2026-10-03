/// GitHub Pages project path (`/s_e_e_library`). Empty for local preview.
pub fn pages_base() -> &'static str {
    option_env!("SEE_LIBRARY_PAGES_BASE")
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("")
}

/// Prefix a site-relative path with the given Pages base (`/s_e_e_library` on GitHub Pages).
pub fn library_href_with_base(base: &str, path: &str) -> String {
    let path = if path.starts_with('/') {
        path
    } else {
        return path.to_string();
    };
    let base = base.trim_end_matches('/');
    if base.is_empty() {
        path.to_string()
    } else {
        format!("{base}{path}")
    }
}

/// Prefix a site-relative path with the active Pages base when set.
pub fn library_href(path: &str) -> String {
    library_href_with_base(pages_base(), path)
}
