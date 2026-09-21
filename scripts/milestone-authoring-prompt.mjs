const REQUIRED_PHRASES = [
  "status",
  "priority",
  "labels",
  "ordinal",
  "criteria",
  "implementation_plan",
  "implementation_notes",
  "final_summary",
  "member story",
  "milestone_get",
  "conflict_token",
  "milestone_update",
  "milestone_add_criterion",
  "story_set_milestone",
  "Derived kinds",
  "Only kind manual",
  "done write is rejected",
];

const FORBIDDEN_PHRASES = [
  "carry no acceptance criteria",
  "milestones carry no acceptance criteria and no implementation plan",
];

export function milestoneAuthoringContractErrors(content) {
  const errors = [];
  const text = String(content ?? "");
  for (const phrase of REQUIRED_PHRASES) {
    if (!text.includes(phrase)) {
      errors.push(`missing required phrase: ${phrase}`);
    }
  }
  for (const phrase of FORBIDDEN_PHRASES) {
    if (text.includes(phrase)) {
      errors.push(`forbidden obsolete copy: ${phrase}`);
    }
  }
  return errors;
}
