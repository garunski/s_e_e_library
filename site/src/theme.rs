use s_e_e_shapes::Theme;

#[allow(dead_code)]
pub const THEME_STORAGE_KEY: &str = "see.theme";

#[allow(dead_code)]
#[must_use]
pub fn theme_storage_label(theme: &Theme) -> &'static str {
    match theme {
        Theme::Light => "Light",
        Theme::Dark => "Dark",
        Theme::System => "System",
    }
}

#[allow(dead_code)]
#[must_use]
pub fn parse_theme_storage_label(raw: &str) -> Option<Theme> {
    match raw {
        "Light" => Some(Theme::Light),
        "Dark" => Some(Theme::Dark),
        "System" => Some(Theme::System),
        _ => None,
    }
}

#[must_use]
pub fn theme_appears_dark(theme: &Theme, system_dark: bool) -> bool {
    match theme {
        Theme::Dark => true,
        Theme::Light => false,
        Theme::System => system_dark,
    }
}

#[cfg(all(feature = "web", target_family = "wasm"))]
#[must_use]
pub fn detect_system_dark_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false)
}

#[cfg(not(all(feature = "web", target_family = "wasm")))]
#[must_use]
pub fn detect_system_dark_mode() -> bool {
    false
}

#[cfg(all(feature = "web", target_family = "wasm"))]
#[must_use]
pub fn read_cached_theme() -> Option<Theme> {
    let raw = web_sys::window()?
        .local_storage()
        .ok()
        .flatten()?
        .get_item(THEME_STORAGE_KEY)
        .ok()
        .flatten()?;
    parse_theme_storage_label(&raw)
}

#[cfg(not(all(feature = "web", target_family = "wasm")))]
#[must_use]
pub fn read_cached_theme() -> Option<Theme> {
    None
}

#[cfg(all(feature = "web", target_family = "wasm"))]
pub fn write_cached_theme(theme: &Theme) {
    if let Some(Some(storage)) = web_sys::window().map(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item(THEME_STORAGE_KEY, theme_storage_label(theme));
    }
}

#[cfg(not(all(feature = "web", target_family = "wasm")))]
pub fn write_cached_theme(_theme: &Theme) {}

#[cfg(all(feature = "web", target_family = "wasm"))]
pub fn apply_theme_to_html(theme: &Theme) {
    use wasm_bindgen::JsCast;
    let dark = theme_appears_dark(theme, detect_system_dark_mode());
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let Some(root) = doc.document_element() else {
        return;
    };
    let list = root.class_list();
    if dark {
        let _ = list.add_1("dark");
        let _ = list.remove_1("light");
        let _ = root.set_attribute("data-theme", "dark");
    } else {
        let _ = list.add_1("light");
        let _ = list.remove_1("dark");
        let _ = root.set_attribute("data-theme", "light");
    }
    if let Ok(html) = root.dyn_into::<web_sys::HtmlElement>() {
        let scheme = if dark { "dark" } else { "light" };
        let _ = html.style().set_property("color-scheme", scheme);
    }
}

#[cfg(not(all(feature = "web", target_family = "wasm")))]
pub fn apply_theme_to_html(_theme: &Theme) {}

pub const THEME_BOOT_SCRIPT: &str = r#"
(function(){
  try {
    var t = localStorage.getItem("see.theme") || "System";
    var dark = t === "Dark" || (t === "System" && window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches);
    var cl = document.documentElement.classList;
    if (dark) { cl.add("dark"); cl.remove("light"); document.documentElement.setAttribute("data-theme", "dark"); document.documentElement.style.colorScheme = "dark"; }
    else      { cl.add("light"); cl.remove("dark"); document.documentElement.setAttribute("data-theme", "light"); document.documentElement.style.colorScheme = "light"; }
  } catch (e) {}
  function duplicateScript(parent, node) {
    if (!node || node.nodeType !== 1 || node.tagName !== "SCRIPT" || typeof parent.querySelector !== "function") return false;
    var src = node.getAttribute("src");
    if (!src) return false;
    return parent.querySelector("script[src=\"" + src.replace(/"/g, "") + "\"]") !== null;
  }
  var append = Element.prototype.appendChild;
  Element.prototype.appendChild = function(node) {
    if (duplicateScript(this, node)) return node;
    return append.call(this, node);
  };
  var insert = Element.prototype.insertBefore;
  Element.prototype.insertBefore = function(node, ref) {
    if (duplicateScript(this, node)) return node;
    return insert.call(this, node, ref);
  };
})();
"#;

#[cfg(test)]
#[path = "theme_tests.rs"]
mod theme_tests;
