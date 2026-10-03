//! Machine-checkable metadata for the six package authoring guides.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(test), allow(dead_code))]
pub struct AuthoringGuideContract {
    pub slug: &'static str,
    pub document_title: &'static str,
    pub payload_file: &'static str,
    pub install_destination_hint: &'static str,
    pub schema_path: &'static str,
    pub catalog_schema_label: &'static str,
}

#[cfg_attr(not(test), allow(dead_code))]
pub const PACKAGE_AUTHORING_GUIDES: [AuthoringGuideContract; 6] = [
    AuthoringGuideContract {
        slug: "workflows",
        document_title: "Workflows",
        payload_file: "definition.json",
        install_destination_hint: ".s_e_e/workflows/definitions/",
        schema_path: "/schema/workflow/",
        catalog_schema_label: "see.library/v1",
    },
    AuthoringGuideContract {
        slug: "prompts",
        document_title: "Prompts",
        payload_file: "prompt.json",
        install_destination_hint: ".s_e_e/prompts/",
        schema_path: "/schema/prompt/",
        catalog_schema_label: "see.library/v1",
    },
    AuthoringGuideContract {
        slug: "skills",
        document_title: "Skills",
        payload_file: "SKILL.md",
        install_destination_hint: ".agents/skills/",
        schema_path: "/schema/skill/",
        catalog_schema_label: "see.library/v1",
    },
    AuthoringGuideContract {
        slug: "commands",
        document_title: "Commands",
        payload_file: "command.json",
        install_destination_hint: ".s_e_e/commands/",
        schema_path: "/schema/command/",
        catalog_schema_label: "see.library/v1",
    },
    AuthoringGuideContract {
        slug: "bundles",
        document_title: "Bundles",
        payload_file: "",
        install_destination_hint: "files[]",
        schema_path: "/schema/bundle/",
        catalog_schema_label: "see.library/v2",
    },
    AuthoringGuideContract {
        slug: "cycles",
        document_title: "Cycles",
        payload_file: "cycle.json",
        install_destination_hint: ".s_e_e/cycles/",
        schema_path: "/schema/cycle/",
        catalog_schema_label: "see.library/v1",
    },
];

#[must_use]
#[cfg_attr(not(test), allow(dead_code))]
pub fn guide_contract(slug: &str) -> Option<&'static AuthoringGuideContract> {
    PACKAGE_AUTHORING_GUIDES
        .iter()
        .find(|guide| guide.slug == slug)
}

#[cfg(test)]
#[path = "authoring_guides_contract_tests.rs"]
mod authoring_guides_contract_tests;
