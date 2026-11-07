use super::*;

const VALID: &str = "The home labels are This machine and Project: Example.\nThis machine contains definitions for the current OS user.\nMachine definitions are stored under ~/.s_e_e/definitions/.\nWorkflow runs, logs, schedules, cycles, and execution history remain per project.\nMachine definitions are outside project Git and project backups; back up ~/.s_e_e/definitions/ separately.\nA machine workflow runs only after a target project is selected.";

#[test]
fn definition_home_valid_and_each_missing_or_contradictory_fact() {
    assert!(check_facts("valid", VALID).is_empty());
    assert!(check_facts(
        "wrong machine label",
        &VALID.replace("This machine and", "This machinery and")
    )
    .iter()
    .any(|error| error.contains("machine label")));
    for ((fact, expected), wrong) in FACTS.iter().zip([
        "Global",
        "Workspace: Example",
        "This machine contains definitions for all OS users.",
        "Machine definitions are stored under .s_e_e/.",
        "Workflow runs, logs, schedules, cycles, and execution history remain on the machine.",
        "Machine definitions are included in project Git and project backups.",
        "A machine workflow runs without a target project.",
    ]) {
        for replacement in ["", wrong] {
            let invalid = VALID.replace(expected, replacement);
            let errors = check_facts("isolated-artifact", &invalid);
            assert!(
                errors
                    .iter()
                    .any(|error| error.contains("isolated-artifact") && error.contains(fact)),
                "{fact}: {errors:?}"
            );
        }
    }
}

#[test]
fn definition_home_checks_visible_html_and_exact_workflow_section() {
    let html = format!("<html><head><script>{VALID}</script></head><body><div hidden>{VALID}</div><div aria-hidden=\"true\">{VALID}</div></body></html>");
    assert_eq!(check_facts("hidden", &visible_text(&html)).len(), 7);
    let html = format!(
        "<html><body><p>{}</p></body></html>",
        VALID
            .replace("Project: Example", "Project:&#32;Example")
            .replace("OS user", "OS\n user")
    );
    assert!(check_facts("visible", &visible_text(&html)).is_empty());
    let llms = format!("# Other\nSource: /authoring/prompts/\n{VALID}\n# Workflows\nSource: /authoring/workflows/\nwrong\n# Next\nSource: /schema/workflow/\n{VALID}");
    assert_eq!(
        check_facts("wrong-section", workflow_section(&llms)).len(),
        7
    );
    assert!(check_facts(
        "right-section",
        workflow_section(&format!(
            "# Workflows\nSource: /authoring/workflows/\n{VALID}\n# Next\nother"
        ))
    )
    .is_empty());
}

#[test]
fn definition_home_actual_repository_workflow_section() {
    let llms = include_str!("../../public/llms.txt");
    assert!(check_facts("repository llms workflow section", workflow_section(llms)).is_empty());
}

#[test]
fn definition_home_each_published_artifact_reports_isolated_fact_failure() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let staging = root.join("staged");
    std::fs::create_dir_all(root.join("public")).unwrap();
    std::fs::create_dir_all(staging.join("authoring/workflows")).unwrap();
    let paths = [
        staging.join("authoring/workflows/index.html"),
        root.join("public/llms.txt"),
        staging.join("llms.txt"),
    ];
    let artifacts = [
        format!("<html><body>{VALID}</body></html>"),
        format!("# Workflows\nSource: /authoring/workflows/\n{VALID}\n# Other\nignored"),
        format!("# Workflows\nSource: /authoring/workflows/\n{VALID}\n# Other\nignored"),
    ];
    for (path, artifact) in paths.iter().zip(&artifacts) {
        std::fs::write(path, artifact).unwrap();
    }
    assert!(verify_definition_home_docs(root, &staging).is_empty());
    for (path, artifact) in paths.iter().zip(&artifacts) {
        for (fact, expected) in FACTS {
            std::fs::write(path, artifact.replace(expected, "contradictory fact")).unwrap();
            let errors = verify_definition_home_docs(root, &staging);
            assert_eq!(errors.len(), 1, "{path:?}: {fact}: {errors:?}");
            assert!(errors[0].contains(&path.display().to_string()));
            assert!(errors[0].contains(fact));
            std::fs::write(path, artifact).unwrap();
        }
    }
}
