use std::{fs, path::Path};

use scraper::{Html, Node};

pub(crate) const FACTS: [(&str, &str); 7] = [
    ("machine label", "The home labels are This machine and "),
    ("project label", "Project: Example."),
    ("OS-user scope", "This machine contains definitions for the current OS user."),
    ("machine root", "Machine definitions are stored under ~/.s_e_e/definitions/."),
    ("run ownership", "Workflow runs, logs, schedules, cycles, and execution history remain per project."),
    ("backup responsibility", "Machine definitions are outside project Git and project backups; back up ~/.s_e_e/definitions/ separately."),
    ("target selection", "A machine workflow runs only after a target project is selected."),
];

fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn check_facts(artifact: &str, text: &str) -> Vec<String> {
    let text = normalize(text);
    FACTS
        .iter()
        .filter_map(|(fact, expected)| {
            (!text.contains(expected))
                .then(|| format!("{artifact}: missing/mismatched {fact}: {expected}"))
        })
        .collect()
}

pub(crate) fn visible_text(html: &str) -> String {
    let document = Html::parse_document(html);
    let mut text = String::new();
    for node in document.tree.root().descendants() {
        let Node::Text(value) = node.value() else {
            continue;
        };
        let hidden = node.ancestors().any(|ancestor| {
            let Node::Element(element) = ancestor.value() else {
                return false;
            };
            matches!(element.name(), "head" | "script" | "style" | "template")
                || element.attr("hidden").is_some()
                || element.attr("aria-hidden") == Some("true")
        });
        if !hidden {
            text.push_str(value);
            text.push(' ');
        }
    }
    normalize(&text)
}

pub(crate) fn workflow_section(llms: &str) -> &str {
    let mut offset = 0;
    let mut start = None;
    for line in llms.split_inclusive('\n') {
        if line.trim() == "Source: /authoring/workflows/" {
            start = Some(offset + line.len());
        } else if start.is_some() && (line.starts_with("# ") || line.starts_with("Source: ")) {
            return &llms[start.unwrap()..offset];
        }
        offset += line.len();
    }
    start.map(|start| &llms[start..]).unwrap_or("")
}

pub(crate) fn verify_definition_home_docs(root: &Path, staging: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let page = staging.join("authoring/workflows/index.html");
    match fs::read_to_string(&page) {
        Ok(html) => errors.extend(check_facts(
            &page.display().to_string(),
            &visible_text(&html),
        )),
        Err(error) => errors.push(format!("{}: {error}", page.display())),
    }
    for path in [root.join("public/llms.txt"), staging.join("llms.txt")] {
        match fs::read_to_string(&path) {
            Ok(llms) => errors.extend(check_facts(
                &format!("{} Source: /authoring/workflows/", path.display()),
                workflow_section(&llms),
            )),
            Err(error) => errors.push(format!("{}: {error}", path.display())),
        }
    }
    errors
}

#[cfg(test)]
#[path = "definition_home_docs_tests.rs"]
mod definition_home_docs_tests;
