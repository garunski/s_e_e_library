use std::collections::HashMap;

use s_e_e_hub::machine_definition_store::MachineDefinitionStore;
use s_e_e_library_ops::{
    library_catalog::{resolve_manifest_url, ManifestFetcher},
    library_install::resolve_package_file_url,
    library_machine_install::MachineLibraryInstaller,
};
use s_e_e_shapes::{
    CatalogSource, CatalogSourceKind, DefinitionHome, DefinitionKind, DefinitionRef,
    WorkflowDefinition,
};
use serde_json::{json, Value};

const WORKFLOW: &str = include_str!("../../site/content/authoring/machine_workflow.json");
const BUNDLE: &str = include_str!("../../site/content/authoring/machine_bundle.json");
const MIXED: &str = include_str!("../../site/content/authoring/mixed_bundle.json");
const REFERENCES: &str = include_str!("../../site/content/authoring/definition_references.json");

struct LocalFiles(HashMap<String, String>);

impl ManifestFetcher for LocalFiles {
    fn fetch(&self, url: &str) -> Result<String, String> {
        self.0
            .get(url)
            .cloned()
            .ok_or_else(|| format!("unexpected local fixture fetch: {url}"))
    }
}

#[test]
fn definition_examples_qualified_refs_and_explicit_project_launches() {
    let examples: Value = serde_json::from_str(REFERENCES).unwrap();
    for (kind, key) in [
        (DefinitionKind::Workflow, "workflow"),
        (DefinitionKind::Command, "command"),
        (DefinitionKind::Prompt, "prompt"),
    ] {
        for raw in examples[key].as_array().unwrap() {
            let raw = raw.as_str().unwrap();
            let reference = DefinitionRef::parse(kind, raw, "project-a").unwrap();
            assert_eq!(reference.canonical_ref(), raw);
            assert_eq!(
                matches!(reference.home, DefinitionHome::Machine {}),
                raw.starts_with("machine:")
            );
            if let DefinitionHome::Project { project_id } = reference.home {
                assert_eq!(project_id.as_str(), "project-a");
            }
        }
    }
    let a: Value = serde_json::from_str(include_str!(
        "../../site/content/authoring/machine_launch_a.json"
    ))
    .unwrap();
    let b: Value = serde_json::from_str(include_str!(
        "../../site/content/authoring/machine_launch_b.json"
    ))
    .unwrap();
    assert_eq!(a["project"], "project-a");
    assert_eq!(b["project"], "project-b");
    assert_ne!(a["project"], b["project"]);
    assert_eq!(a["workflow_id"], b["workflow_id"]);
    assert_eq!(a["workflow_id"], "machine:review");
}

#[tokio::test]
async fn definition_examples_machine_workflow_schema_save_and_root_rejection() {
    let temp = tempfile::tempdir().unwrap();
    let store = MachineDefinitionStore::from_data_root(temp.path().to_path_buf());
    let workflow: WorkflowDefinition = serde_json::from_str(WORKFLOW).unwrap();
    let schema: Value = serde_json::from_str(include_str!(
        "../../public/schema/workflow-definition.schema.json"
    ))
    .unwrap();
    assert!(jsonschema::validator_for(&schema)
        .unwrap()
        .is_valid(&serde_json::from_str::<Value>(WORKFLOW).unwrap()));
    let content_schema: Value =
        serde_json::from_str(include_str!("../../public/schema/workflow.schema.json")).unwrap();
    assert!(jsonschema::validator_for(&content_schema)
        .unwrap()
        .is_valid(&workflow.content));
    let saved = store.save_workflow(&workflow, None).await.unwrap();
    assert_eq!(saved.provenance.reference.canonical_ref(), "machine:review");
    assert_eq!(
        store
            .get_latest_workflow("review")
            .await
            .unwrap()
            .unwrap()
            .definition
            .content,
        workflow.content
    );
    let mut invalid = workflow;
    invalid.content["spoke_root"] = json!("/example/checkout");
    let error = store
        .save_workflow(&invalid, Some(&saved.provenance.revision))
        .await
        .unwrap_err();
    assert!(error.contains("spoke_root"), "{error}");
    assert!(store
        .get_latest_workflow("review")
        .await
        .unwrap()
        .unwrap()
        .definition
        .content
        .get("spoke_root")
        .is_none());
}

fn preview_bundle(raw: &str) -> Result<s_e_e_shapes::MachineLibraryPreview, String> {
    let temp = tempfile::tempdir().unwrap();
    let installer = MachineLibraryInstaller {
        machine_data_root: temp.path().join("machine"),
        project_data_root: temp.path().join("project/.s_e_e"),
        project_id: "project-a".into(),
    };
    let source = CatalogSource {
        id: "local".into(),
        name: "Local fixtures".into(),
        kind: CatalogSourceKind::GithubPages,
        url: "https://local.invalid/".into(),
        repo: "example/local".into(),
        manifest: "catalog.json".into(),
        branch: None,
        enabled: true,
        primary: true,
        verified: false,
        last_sync: "never".into(),
        package_count: 0,
        description: None,
    };
    std::fs::create_dir_all(&installer.project_data_root).unwrap();
    let mut config = s_e_e_hub::default_hub_config();
    config["library"] = json!({"sources":[source]});
    s_e_e_hub::save_hub_config_value(&installer.project_data_root, &config).unwrap();
    let package: Value = serde_json::from_str(raw).unwrap();
    let schema: Value = serde_json::from_str(include_str!(
        "../../public/schema/bundle-package.schema.json"
    ))
    .unwrap();
    assert!(jsonschema::validator_for(&schema)
        .unwrap()
        .is_valid(&package));
    let manifest = json!({"schema":"see.library/v2","name":"Local","updated":"2026-11-07T00:00:00Z","packages":[package]}).to_string();
    let fetcher = LocalFiles(HashMap::from([
        (resolve_manifest_url(&source).unwrap(), manifest),
        (
            resolve_package_file_url(&source, "prompt.json").unwrap(),
            json!({"id":"review","name":"Review","content":"Inspect the selected project."})
                .to_string(),
        ),
    ]));
    installer.preview(&[package["id"].as_str().unwrap().into()], &fetcher)
}

#[test]
fn definition_examples_definition_only_bundle_accepted_mixed_rejected() {
    let preview = preview_bundle(BUNDLE).unwrap();
    assert_eq!(preview.definitions.len(), 1);
    assert_eq!(
        preview.definitions[0].destination.canonical_ref(),
        "machine:review"
    );
    assert!(preview.definitions[0]
        .destination_path
        .ends_with("definitions/prompts/review.json"));
    let error = preview_bundle(MIXED).unwrap_err();
    assert!(
        error.contains("unsupported machine install destination")
            && error.contains(".agents/skills/review/SKILL.md"),
        "{error}"
    );
}
