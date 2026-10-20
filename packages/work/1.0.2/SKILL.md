---
name: work
description: Implement an existing S.E.E. story against its plan and acceptance criteria. Use when asked to work on a story id or complete assigned story work.
---

# Implement an S.E.E. story

The story is the implementation contract. Repository `AGENTS.md` supplies local rules, commands, and quality gates; load another skill only when the task needs that capability. Do not paste all guidance into a new opening prompt or repeat generic instructions the agent already has.

## Load and check the work

1. Use the `see` MCP server and pass the project id on every call. The S.E.E. section of `AGENTS.md` names the project for this repository; otherwise read `see:/projects`. Read the story with `story_get`, including its status, description, implementation plan, acceptance criteria, dependencies, and `conflict_token`. Read referenced docs and decisions only where they affect this work.
2. Never edit story Markdown directly. If MCP is unavailable, use the documented project story HTTP API; if neither store interface works, stop before changing story metadata.
3. Read applicable `AGENTS.md` files for the code repository and inspect the files and commands named by the story. Check current code so the stated gap and plan still match reality.
4. Confirm dependencies and the configured statuses from `see:/projects/{project}/stories/config`. Continue an already active story when appropriate; otherwise set the in-progress status through `story_set_status` when beginning work. Use the current token and read again after a conflict.
5. A story contains no open design choice. If the description, plan, or a criterion asks the implementer to choose an approach, threshold, or scope, stop and ask for a corrected story before implementing that choice.

## Implement and verify

- Follow the approach the story selects. Map each acceptance criterion to the code path and observable evidence that can prove it. If new necessary work appears, surface it instead of silently widening the story.
- Make focused changes and use the repository's actual commands and quality gates from `AGENTS.md`. Follow an explicit user instruction about verification. Otherwise run the smallest checks that test the changed behavior, then the required completion gate. Do not add tests that merely restate the implementation or copy a tool's current output.
- If a check fails, investigate it. Keep the story active while a required criterion or gate remains unverified. A self-authored diff or a launched workflow run is not proof of external behavior when the criterion calls for an observed result.
- Give concise progress updates at meaningful points: changed behavior, evidence obtained, and blockers. Keep implementation details in the work report rather than copying them into `AGENTS.md` or a reusable skill.

## Update the story through the store

- Check a criterion with `story_check_criterion` only after its evidence is observed. Use `story_get` for a fresh token before writes and read again after a conflict. The tool input schema determines how a criterion is addressed.
- Use `story_update` for implementation notes and final summary. Record actual changes, verification, remaining limitations, and any deviation from the plan.
- Set Done with `story_set_status` only after every required criterion is checked and the story is complete under the configured status rules. Read the final record back. If blocked or partly complete, leave its status and criteria honest and name the blocker.
- Do not alter unrelated stories or knowledge records unless the user or this story explicitly requires it.
