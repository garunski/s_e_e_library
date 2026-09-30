---
name: work
description: Implement an existing S.E.E. story against its plan and acceptance criteria. Use when asked to work on a story id or complete assigned story work.
---

# Implement an S.E.E. story

The story is the implementation contract. Repository `AGENTS.md` supplies local rules and commands; load another skill only when the task needs that capability. Do not paste all guidance into a new opening prompt or repeat generic instructions that the agent already has.

## Load and check the work

1. Resolve the project and use `story_get` to read the story, including its status, description, implementation plan, acceptance criteria, dependencies, and `conflict_token`. Read referenced decisions and docs only where they affect this work. Do not edit store Markdown directly. If MCP is unavailable, use the documented project story HTTP API; if neither store interface works, stop before changing story metadata.
2. Read applicable `AGENTS.md` files for the code repository and inspect the files and commands named by the story. Check current code so the stated gap and plan still match reality.
3. Confirm dependencies and the configured story status. Continue an already active story when appropriate; set an available in-progress status through `story_set_status` when beginning work. Use the current token and reread after a conflict.
4. A story contains no open design choice. If the description, plan, or criterion asks the implementer to choose an approach, threshold, or scope, stop and ask for a corrected story before implementing that choice.

## Implement and verify

- Follow the selected approach in the story. Map each acceptance criterion to the code path and observable evidence that can prove it. If new necessary work appears, surface it before silently widening the story.
- Make focused changes and use the repository's actual commands and quality gates. Follow an explicit user instruction about verification. Otherwise run the smallest checks that test the changed behavior and the required completion gate from the repository. Do not add tests that merely restate implementation or test infrastructure by copying its current output.
- If a check fails, investigate the failure. Keep the story active while a required criterion or gate remains unverified. Do not treat a self-authored diff or a launched workflow run as proof of external behavior when the criterion calls for an observed result.
- Give concise progress updates at meaningful points: changed behavior, evidence obtained, and blockers. Keep implementation details in the work report rather than copying them into `AGENTS.md` or a reusable skill.

## Update the story through the store

- Check a criterion with `story_check_criterion` only after its evidence is observed. Use `story_get` for a fresh token before writes and reread after a conflict. The tool input schema determines how a criterion is addressed.
- Use `story_update` for implementation notes and final summary. Record actual changes, verification, remaining limitations, and any deviation from the plan.
- Set Done with `story_set_status` only after every required criterion is checked and the story is complete under the configured status rules. Read the final record back. If blocked or partly complete, leave its status and criteria honest and name the blocker.
- Do not alter unrelated stories or knowledge records unless the user or this story explicitly requires it.
