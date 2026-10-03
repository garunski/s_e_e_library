use std::path::Path;

use crate::discover_bundles::discover_bundles;

fn library_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn discover_bundles_reports_current_bundle_slugs() {
    let bundles = discover_bundles(&library_root()).expect("discover");
    let slugs: Vec<String> = bundles.iter().map(|b| b.slug.clone()).collect();
    assert_eq!(
        slugs,
        vec![
            "author-stories-from-doc",
            "author-story",
            "author-workflow",
            "generate-milestone-summary-doc",
            "implement-story",
            "work-swimlane-stories",
        ]
    );
    for bundle in bundles {
        assert!(!bundle.members.is_empty());
        let sorted = bundle.members.clone();
        let mut members = bundle.members.clone();
        members.sort();
        assert_eq!(members, sorted);
    }
}
