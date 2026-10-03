//! Machine-checkable routes and markers for catalog + package schema pages (story-498).

use crate::route_table::{schema_path, SCHEMA_SLUGS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackageSchemaPageContract {
    pub slug: &'static str,
    pub document_title: &'static str,
    pub category: &'static str,
}

pub const PACKAGE_SCHEMA_PAGES: [PackageSchemaPageContract; 7] = [
    PackageSchemaPageContract {
        slug: "workflow",
        document_title: "Workflow schema",
        category: "workflow",
    },
    PackageSchemaPageContract {
        slug: "prompt",
        document_title: "Prompt schema",
        category: "prompt",
    },
    PackageSchemaPageContract {
        slug: "skill",
        document_title: "Skill schema",
        category: "skill",
    },
    PackageSchemaPageContract {
        slug: "command",
        document_title: "Command schema",
        category: "command",
    },
    PackageSchemaPageContract {
        slug: "rule-template",
        document_title: "Rule and template schema",
        category: "rule",
    },
    PackageSchemaPageContract {
        slug: "bundle",
        document_title: "Bundle schema",
        category: "bundle",
    },
    PackageSchemaPageContract {
        slug: "cycle",
        document_title: "Cycle schema",
        category: "cycle",
    },
];

/// Markers the catalog index must expose for AC #1 (classification and destinations).
pub const CATALOG_INDEX_MARKERS: [&str; 8] = [
    "see.library/v1",
    "see.library/v2",
    "Package classification",
    "usageContexts",
    "cliScope",
    "technologyScope",
    "Install paths",
    "categories",
];

#[must_use]
pub fn package_schema_slugs_match_route_table() -> bool {
    SCHEMA_SLUGS.len() >= PACKAGE_SCHEMA_PAGES.len()
        && PACKAGE_SCHEMA_PAGES
            .iter()
            .zip(SCHEMA_SLUGS.iter())
            .all(|(page, slug)| page.slug == *slug)
}

#[must_use]
pub fn package_schema_paths() -> Vec<&'static str> {
    PACKAGE_SCHEMA_PAGES
        .iter()
        .map(|page| schema_path(page.slug))
        .collect()
}

#[cfg(test)]
#[path = "schema_package_contract_tests.rs"]
mod schema_package_contract_tests;
