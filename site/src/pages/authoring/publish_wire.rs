//! Plain-text snippets shared by the publish page and machine checks.

pub const VALIDATE_AND_CATALOG_COMMANDS: &str =
    "mise run validate\nmise run catalog";

pub const DOCS_CHECK_COMMANDS: &str = "mise run quality";

pub const PAYLOAD_LAYOUT: &str = "packages/{slug}/{version}/";

pub const PACKAGE_SIDECAR_PATH: &str = "packages/{slug}/s_e_e_package.json";

pub const CLI_SCOPE_ANY: &str = r#"{ "scope": "any" }"#;

pub const CLI_SCOPE_SPECIFIC: &str = r#"{ "scope": "specific", "cliIds": ["cursor"] }"#;

pub const TECH_SCOPE_GENERAL: &str = r#"{ "scope": "general" }"#;

pub const TECH_SCOPE_SPECIFIC: &str = r#"{ "scope": "specific" }"#;

pub const RELEASE_HISTORY_SHAPE: &str = "{ version, date, note }";

pub const CATALOG_IDENTITY: &str =
    "After merge, installers resolve the package by catalog id and version. The files[] install map is authoritative: each from path under packages/ maps to a hub to destination. Keep id, version, and files[] stable across releases unless you intend a breaking catalog change.";
