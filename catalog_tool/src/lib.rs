mod discover_bundles;
mod harvest;
mod milestone_prompt;
mod package_meta;
mod payload_meta;
mod pages_artifact;
mod validate;
mod verify_docs;

pub use discover_bundles::{discover_bundles, run_discover_bundles, BundleDef};
pub use harvest::{default_hub_root, package_signatures_from_catalog, run_harvest};
pub use milestone_prompt::milestone_authoring_contract_errors;
pub use package_meta::{
    catalog_without_updated, marketplace_ref_errors, normalize_releases, normalize_tool_ids,
    stable_stringify,
};
pub use pages_artifact::{
    default_dx_public, default_staging, run_package_pages, smoke_pages_artifact,
    verify_staged_bytes,
};
pub use validate::{run_catalog, run_validate};
pub use verify_docs::{run_verify_docs, verify_staged_docs};

#[cfg(test)]
#[path = "package_meta_tests.rs"]
mod package_meta_tests;

#[cfg(test)]
#[path = "milestone_prompt_tests.rs"]
mod milestone_prompt_tests;

#[cfg(test)]
#[path = "validate_integration_tests.rs"]
mod validate_integration_tests;

#[cfg(test)]
#[path = "discover_bundles_tests.rs"]
mod discover_bundles_tests;

#[cfg(test)]
#[path = "harvest_tests.rs"]
mod harvest_tests;

#[cfg(test)]
#[path = "pages_artifact_tests.rs"]
mod pages_artifact_tests;

#[cfg(test)]
#[path = "verify_docs_tests.rs"]
mod verify_docs_tests;
