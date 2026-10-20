---
name: stories
description: >-
  Write S.E.E. stories and milestones that another agent can implement without
  redesigning them. Use when asked to create, revise, split, or plan stories or
  milestones, or to work with story labels, criteria, and dependencies in an
  S.E.E. project.
---

# S.E.E. story and milestone authoring

A story is an implementation contract: what to change, why, how, and what observable evidence proves it is done. Repository `AGENTS.md` supplies the local rules, commands, and quality gates; read it for each repository the story touches. This skill covers the store contract and the quality bar.

## Store access

- Use the `see` MCP server. Pass the `project` id on every call; read `see:/projects` when you do not know it. The S.E.E. section of `AGENTS.md` names the project for the current repository.
- Never create, rename, or edit story or milestone Markdown files directly. The store allocates ids, writes frontmatter, and names files. Do not use `apply_patch`, shell writes, a guessed id, or a guessed filename.
- If the MCP tools are unavailable, use the documented project HTTP API. If neither is reachable, stop and report that authoring is blocked.
- Every write takes the `conflict_token` from the latest read. Over HTTP, send it as the query parameter `?conflict_token=`, the JSON body field `conflict_token`, or the header `x-s-e-e-conflict-token`; when more than one is sent they must match. A PUT body may omit the entity `id`; the URL path is the id. After a conflict, read again and retry.
- Use titles made only of ASCII letters, numbers, and spaces so filenames stay `<id> - <title>.md`. After every create or title change, read the record back with `story_get` or `milestone_get`.
- Read `see:/projects/{project}/stories/config` for valid statuses and label conventions. Do not invent status names.

## Tools

Tool input schemas are the contract. Prefer the named tools; use `item_fetch` and `item_call` only when no named tool exists.

- Stories: `story_create` (title only, allocates the id), `story_get`, `story_update` (description, implementation plan, implementation notes, final summary), `story_set_status`, `story_set_labels`, `story_set_priority`, `story_set_milestone`, `story_set_dependencies`, `story_set_assignee`, and the criterion tools `story_add_criterion`, `story_edit_criterion`, `story_check_criterion`, `story_remove_criterion`, `story_reorder_criteria`.
- Milestones: `milestone_list`, `milestone_get`, `milestone_create`, `milestone_update`, `milestone_set_status`, `milestone_set_priority`, `milestone_set_labels`, `milestone_set_ordinal`, `milestone_set_title`, `milestone_set_description`, `milestone_add_members`, `milestone_remove_members`, and the criterion tools `milestone_add_criterion`, `milestone_edit_criterion`, `milestone_remove_criterion`, `milestone_reorder_criteria`, `milestone_check_criterion`.

## A story contains no choices

The implementing agent executes the story; it does not design it. Every decision is made at authoring time and written into the story as the thing to do.

- Never write "decide whether", "choose between", "determine whether", "either X or Y", "consider whether", "prefer X", "weigh X against Y", or "record the decision" in a description, plan, or acceptance criterion.
- When two approaches exist, pick one, name it, and give one sentence on why the other loses. The alternative does not stay in the plan as a live option.
- Thresholds, intervals, retention windows, and limits are decisions. Write the number: "prune records older than 90 days", not "prune by count or age".
- An acceptance criterion is never satisfied by making a decision. If a criterion contains "decided", "whether", "either", or "recorded", rewrite it as the behavior that results from the decision.
- If authoring surfaces a decision you cannot make from the evidence, stop and ask the user before creating the story. Never create the story with the choice embedded.

## Evidence before prescription

- Verify the gap still exists and record in the description the path, command, or behavior that shows it. A story written against stale state wastes a full implementation run.
- Record each observed fact and where it was checked before prescribing an approach. A hypothesis is not a fact; if a required fact is missing, ask instead of guessing.
- Verify every command you prescribe: the tool, subcommand, option combination, working directory, and environment. Take them from the repository `AGENTS.md`, its build files, or installed help. Never copy paths, ids, endpoints, or status names from another project.
- Fetch each dependency's full record. A Done status with an unchecked required criterion or a named blocker does not make a dependency ready.
- Name prerequisites an unattended agent cannot satisfy on its own, such as credentials, interactive sign-in, special hardware, or unusual resource budgets, together with who resolves them.
- When a workflow will execute the story, inspect its stored definition and command configuration without launching a run. Confirm the agent works in the intended repository and that store writes go through the `see` tools. Report a contradictory workflow as a blocker instead of calling the story ready.

## Scope and specificity

- One story is one independently deliverable behavior. Layers that must ship together stay in one story, with each layer named. Split anything that can ship alone, and set dependencies between the parts.
- Inventory every affected surface: screens, stored objects and fields, API and MCP operations, runtime components, published or seeded definitions, and behavior on installations that already hold the data. Map each surface to a plan step and an acceptance criterion, or state "not applicable" where omission could look like an oversight.
- Put implementation detail in the implementation plan, not in the description or the criteria. Name real files, modules, and commands.
- Dependencies point only at stories that already exist.
- `story_create` makes a temporary shell. Complete the description, plan, metadata, and criteria in the same session and read the full story back before handoff. Leave implementation notes and final summary empty while authoring.

## Acceptance criteria that cannot be faked

Number each criterion and state one outcome per criterion, in present tense: "`parse_config` rejects an entry with an empty key", not "add validation for empty keys".

1. State a behavior, not an artifact. "The config declares a flag" proves nothing; "a run with the flag set refuses to start until a subject is chosen" does.
2. Name evidence the implementer does not author: a test result, a process log line, an HTTP response, a rendered screen, a stored record. A criterion satisfied by reading one's own diff is not a criterion.
3. When integrating with a third-party tool, one criterion states the mechanism that tool actually uses to receive the input, and the plan records where that mechanism was confirmed. Setting an environment variable is not the same as the tool reading it.
4. When seeded or default data changes, one criterion states the outcome on an installation that already holds that data.
5. A red completion gate means the story is not Done unless a criterion explicitly allows the named failure with evidence that it fails identically before the change.
6. Leaving a story In Progress with named blockers is a valid outcome. Say so in the story when a criterion is expected to be hard to verify.
7. Never require launching a workflow execution, a probe workflow, or reading a live execution as proof.

## Milestones

A milestone is a record with status, priority, labels, ordinal, typed criteria, plan, notes, and summary. Membership is the story's `milestone` field, set with `story_set_milestone` or `milestone_add_members`; a label never implies membership.

- Write the observable outcome and its boundary first. Inventory every necessary behavior and map each to an implementation-ready story. Include a story only when the outcome or an explicit milestone criterion needs it, and state what is outside the cut.
- Add a `milestone_progress` criterion targeting the milestone itself with `min_completion_percent` 100, plus separate typed criteria for cross-story outcomes. Use `manual` only for evidence the evaluator cannot derive, and name that evidence. Address milestone criteria by `criterion_id`, never by position.
- After assigning members, read the milestone back and compare the actual members with the scope inventory. Revise scope, criteria, and membership together whenever any of them changes.
- Before setting Done, verify every member story's criteria and status, inspect each milestone criterion result, and fill the final summary.

## Authoring procedure

1. Resolve the project and read the stories config resource.
2. Complete the evidence, scope, and choice reviews above before creating anything.
3. `story_create` with the title, then `story_update` with the description and implementation plan, using the token from `story_get`.
4. Add numbered criteria with `story_add_criterion`.
5. Set labels, priority, milestone, and dependencies.
6. Read the full story back: description, plan, criteria, dependencies, milestone, and the configured initial status.
7. Report the story id and title, and state whether it is ready, waiting on dependencies, or blocked, and why.

Use the `work` skill to implement a story and the `doc` skill for knowledge docs and decisions.
