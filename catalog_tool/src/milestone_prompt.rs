const REQUIRED_PHRASES: &[&str] = &[
    "status",
    "priority",
    "labels",
    "ordinal",
    "criteria",
    "implementation_plan",
    "implementation_notes",
    "final_summary",
    "member story",
    "item_fetch",
    "conflict_token",
    "milestone_update",
    "milestone_add_criterion",
    "story_set_milestone",
    "Derived kinds",
    "Only kind manual",
    "done write is gated",
];

const FORBIDDEN_PHRASES: &[&str] = &[
    "carry no acceptance criteria",
    "milestones carry no acceptance criteria and no implementation plan",
];

pub fn milestone_authoring_contract_errors(content: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for phrase in REQUIRED_PHRASES {
        if !content.contains(phrase) {
            errors.push(format!("missing required phrase: {phrase}"));
        }
    }
    for phrase in FORBIDDEN_PHRASES {
        if content.contains(phrase) {
            errors.push(format!("forbidden obsolete copy: {phrase}"));
        }
    }
    errors
}
