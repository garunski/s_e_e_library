---
name: work
description: Implement an existing S.E.E. story and record its verified acceptance criteria.
---

1. Read the story with `story_get`, its dependencies, and any parent milestone or references relevant to the work. Confirm prerequisite behavior from evidence. Start or resume using the configured story status.
2. Follow the story's plan and the target spoke's rules. Verify changed behavior with focused checks and fix in-scope failures. Bound headless checks and stop owned processes. Respect explicit skips and leave any criterion they prevent you from verifying unchecked.
3. **Check every verified acceptance criterion in S.E.E.** Match each criterion to observed evidence, then save its check with `story_check_criterion`. Record the evidence and any gaps with `story_update`. A launched workflow, a source scan, or an agent report cannot stand in for a required behavior check.
4. Use `story_set_status` to set Done only after every acceptance criterion is checked. Read the story back and confirm the saved status and checkboxes. If work remains, keep the story active and report the specific unmet criteria and blockers.

Preserve the story contract and unrelated records. Deferred milestone review belongs to the milestone. Use the milestone authoring procedure when assigned to reconcile it.
