---
name: workflow
description: >-
  Authors and edits S.E.E. workflow JSON definitions stored under the hub
  `.s_e_e/workflows/definitions/`. Validates against `workflow.schema.json`, the engine
  task handlers (cli_command, command, user_input, git_action,
  story_action, foreach, custom), and template expansion (`{{prompt.*}}`,
  `{{runtime.*}}`, `{{userinput.*}}`). Activates on `/workflow`, requests to
  "create a workflow", "add a workflow task", "edit workflow JSON", or any
  edit to `runtime_inputs` or command task prompts in S.E.E. workflow files.
---

# S.E.E. workflow authoring

Authors and edits workflow JSON for the S.E.E. engine. Output is a JSON file in `.s_e_e/workflows/definitions/` (or a bundled seed in `core/initial_data/workflows/`) that the engine can execute without further hand-editing.

## When to use

- Creating a new workflow definition.
- Adding, removing, or reordering tasks in an existing workflow.
- Editing `runtime_inputs`, command task prompts, `cli_command` args, or template placeholders.
- Converting an ad-hoc prompt into a reusable workflow.

Do NOT use for: editing prompt bodies (use the prompt store), editing stories (`stories` skill), or implementing story acceptance criteria (`work` skill).

## MUST / NEVER

- **MUST** validate the JSON against `core/schema/workflow.schema.json` before declaring done.
- **MUST** declare every `{{runtime.<key>}}` used by any task under `runtime_inputs`.
- **MUST** ensure every `{{userinput.<task_id>.value}}` references a `user_input` task that runs **earlier** in the graph.
- **MUST** set `spoke_root` on the workflow for command tasks with cwd spoke and for `git_action` tasks that omit per-task `spoke`.
- **NEVER** put literal secrets, tokens, or API keys in workflow JSON.
- **NEVER** use dots in `runtime_inputs[].key` (schema rejects them).
- **NEVER** use a `function` object with more than one handler key — the schema is `oneOf`.

## Authoritative references (load on demand)

Read only the file relevant to the task:

- Workflow design patterns and prompt-embedding conventions: `.s_e_e/knowledge/docs/overview/workflow-engine/doc-18 - 18-Workflow-Authoring-Guide.md`
- Template expansion semantics (`{{prompt.*}}`, `{{runtime.*}}`, `{{userinput.*}}`): `.s_e_e/knowledge/docs/overview/workflow-engine/doc-13 - 13-Prompts-and-Workflow-Template-Expansion.md`
- Canonical JSON schema: `core/schema/workflow.schema.json`
- Copy-paste baseline with runtime and prompt placeholders: `core/initial_data/workflows/implement-story.json`
- Bundled **author from description** seed (`system-author-workflow`): `core/initial_data/workflows/author-workflow.json` — run-start `multiline_string` (`workflow_description`) only; `spoke_root` is on the definition and **Restore default workflows** assigns the hub spoke (`core/initial_data/prompts/workflow-authoring.json` for `{{prompt.system-workflow-authoring}}`).

## File location

Hub spoke (`.s_e_e/` under the hub spoke root):

- Workflow definitions: `.s_e_e/workflows/definitions/*.json`. The on-disk file name does NOT identify the workflow; the JSON `id` and inner `content.id` do.
- Workflow schedules: `.s_e_e/workflows/schedules/*.json`
- Schedule rule sets: `.s_e_e/workflows/schedule_rule_sets/*.json`
- Bundled product defaults (repo seeds, not hub paths): `core/initial_data/workflows/*.json`. Use these as starting templates.
- There is no workflow CLI. Edit JSON directly and save; the hub watcher / API picks up the change.

## Minimal valid workflow (copy and modify)

```json
{
  "id": "system-hello-workflow",
  "name": "Hello workflow",
  "description": "Smallest valid workflow.",
  "version": "1.0.0",
  "content": {
    "id": "hello-workflow",
    "name": "Hello workflow",
    "spoke_root": "{{runtime.spoke_root}}",
    "runtime_inputs": [
      {
        "key": "spoke_root",
        "input_type": "string",
        "required": true,
        "label": "Spoke repo root"
      }
    ],
    "tasks": [
      {
        "id": "say_hello",
        "name": "Say hello",
        "function": {
          "name": "cli_command",
          "input": { "command": "echo", "args": ["hello"] }
        }
      }
    ]
  }
}
```

## Outer envelope

```json
{
  "id": "<namespace>:<kebab-name>",
  "name": "Human title",
  "description": "Optional summary",
  "version": "1.0.0",
  "content": { /* engine workflow */ }
}
```

- Outer `id` convention: `system-name>` for product defaults, `user-<name>` for user-created.
- Inner `content.id` is the engine-local id used in logs and runtime references; keep it stable across edits.

## Engine workflow (`content`) shape

Required:

- `id`, `name`, `tasks` (non-empty array).

Optional:

- `runtime_inputs` (array of declarations).
- `spoke_root`, `hub_root`, `stories_root`, `knowledge_root` (template-expandable strings).

Each task:

- `id` (unique within workflow), `name`, `function` (single-handler object).
- Optional `next_tasks`: nested array continuing the graph (depth-first execution).

## Task handlers

**Default first reach**: `command` with `command_id` `cursor-agent` for AI-driven steps, `cli_command` for deterministic shell. Reach for the others only when listed below.

`cli_command` — deterministic shell command.
- `input.command` (string, required), `input.args` (string array, optional).

`command` — run an installed command definition from `.s_e_e/commands/`.
- `input.command_id` (string, required).
- `input.prompt` (string, optional; required when the command definition uses prompt delivery).
- `input.config` (object, optional): parameter values keyed by the command definition `params`.

`user_input` — pause for operator input.
- `input.prompt` (string), `input.input_type` (`string` | `number` | `boolean`), `input.required` (bool), optional `input.default`.
- Later tasks read the captured value via `{{userinput.<this_task_id>.value}}`.

`git_action` — git operation. Use only when story-driven workflows need it.
- `input.action` plus paths per schema.

`story_action` — stories / knowledge store read and mutation. Set workflow `stories_root` / `knowledge_root`; per-task `input.stories_root` / `input.knowledge_root` override for that step. `delete_*` requires `confirm: true`. Optional `client_mtime_ms` on `update_story` and `delete_*` (optimistic lock). Set `capture_output: true` on list tasks to feed `foreach` via `collection_task_id` (+ optional `collection_task_path`, e.g. `["story_ids"]`).

| `action` | Required input | Optional input | Output |
| --- | --- | --- | --- |
| `create_story` | `title` | `status`, `labels`, `description`, `priority` (`high`\|`medium`\|`low`), `milestone`, `dependencies`, `assignee`, `implementation_plan`, `implementation_notes`, `final_summary`, root overrides | full `Story` JSON |
| `create_milestone` | `title` | `description`, root overrides | `{ id, title }` |
| `create_document` | `title`, `category` | `body`, `tags`, `type` (`readme`\|`guide`\|`specification`\|`other`), root overrides | `{ id, title, category }` |
| `create_decision` | `title` | `context`, `decision`, `consequences`, `status` (`proposed`\|`accepted`\|`rejected`\|`superseded`), `author`, `alternatives`, `related_story_ids`, root overrides | `{ id, title }` |
| `set_status` | `story_id`, `status` | root overrides | `{ story_id }` |
| `set_labels` | `story_id`, `labels` | root overrides | `{ story_id }` |
| `update_story` | `story_id` | `title`, `description`, `implementation_plan`, `implementation_notes`, `final_summary`, `status`, `labels`, `priority`, `dependencies`, `assignee`, `milestone`, `client_mtime_ms`, root overrides | `{ story_id }` |
| `archive_story` | `story_id` | root overrides | `{ story_id }` |
| `list_stories` | — | `status`, `labels`, root overrides | `{ story_ids, count }` |
| `list_stories_by_milestone` | `milestone_id` | root overrides | `{ milestone_id, story_ids, count }` |
| `delete_story` | `id`, `confirm: true` | `client_mtime_ms`, root overrides | `{ id }` |
| `delete_document` | `id`, `confirm: true` | `client_mtime_ms`, root overrides | `{ id }` |
| `delete_decision` | `id`, `confirm: true` | `client_mtime_ms`, root overrides | `{ id }` |
| `delete_milestone` | `id`, `confirm: true` | `client_mtime_ms`, root overrides | `{ id }` |

`create_story` defaults omitted `status` to `To Do`. `update_story` applies only present fields. List outputs sort `story_ids` numerically by id suffix.

`foreach` — structural loop over a string array. **Exactly one** `next_tasks` entry (linear body subtree) and `input.item_runtime_key` (per-iteration `{{runtime.<key>}}`). Source the array **one** of two ways: `input.collection_runtime_key` (a `runtime_inputs` key of type `string_array`, expanded at run start), or `input.collection_task_id` (+ optional `input.collection_task_path`, e.g. `["story_ids"]`) referencing the captured output of an earlier task (`capture_output: true`). The task-output form expands lazily once that task completes, so `foreach` can consume ids computed during the run (e.g. a `story_action` `list_stories_by_milestone` resolver).

`custom` — escape hatch when no handler fits. Avoid unless the product defines a runtime for it.

Schema constraint: exactly one handler key per `function` object (`oneOf`).

## Template expansion

Three placeholder kinds (expanded at task ready-to-execute time):

- `{{prompt.<namespace>-<id>}}` — injects stored prompt content (e.g. `{{prompt.system-implement-story}}`). The prompt id must already exist in the hub, or be created in the same change.
- `{{runtime.<key>}}` — value supplied at run start. Every key must be declared in `runtime_inputs`. Keys cannot contain dots.
- `{{userinput.<task_id>.value}}` — value collected by a `user_input` task with that exact `id`. Must appear in a task that runs **after** the producing `user_input` task.

## `runtime_inputs` rules

Each declaration:

- `key`: string, no dots, matches every `{{runtime.<key>}}` use in the workflow.
- `input_type`: `string` | `number` | `boolean` | `string_array` (JSON array of strings at execute time; used with `foreach`) | `multiline_string` (same wire type as `string`; Automation shows a textarea at run start).
- `required`: bool.
- `label`: optional human label for the run modal.
- `default`: optional default value (prefer providing one for optional fields so dry-runs stay fast).

## Graph design

- Model the workflow as a tree via nested `next_tasks`. Execution is depth-first; keep order obvious.
- One concern per task. Split "lint", "test", "agent review" into separate tasks instead of one mega-task, unless the story explicitly demands a single atomic step.
- Command task prompts (for example `cursor-agent`) must name the **files**, **acceptance criteria**, and **constraints**. Reference template values only when the producing task is guaranteed to run earlier.

## Authoring checklist (copy and tick off)

```
Workflow author checklist:
- [ ] JSON parses; outer id, name, content present
- [ ] content.id, content.name, content.tasks present
- [ ] schema validation passes (workflow.schema.json)
- [ ] every {{runtime.*}} key declared in runtime_inputs
- [ ] every {{userinput.*}} references an earlier user_input task id
- [ ] every {{prompt.*}} id exists in the hub prompt store
- [ ] spoke_root set for command/git_action tasks that need a spoke cwd
- [ ] no secrets in JSON
- [ ] dry-run from Automation succeeds with minimal runtime inputs
```

## Anti-patterns

- Using `command` tasks with cwd spoke without workflow `spoke_root`. Execution fails at run time.
- Referencing `{{userinput.foo.value}}` from a task that runs before or in parallel with task `foo`.
- Misspelled `userinput` task id (silent miss-expansion).
- Dots in `runtime_inputs[].key`.
- Multiple handler keys in one `function` object.
- One workflow doing several unrelated jobs. Split into separate definitions.
- Magic constants in command args without justification.
- Long prose prompts in command `input.prompt` instead of `{{prompt.*}}` reuse.

## Authoring loop

1. Read `doc-18` (design) and `doc-13` (template expansion) — only the sections relevant to the change.
2. Copy `core/initial_data/workflows/implement-story.json`, `author-workflow.json`, or the closest existing seed as the baseline.
3. Edit ids, names, `runtime_inputs`, tasks, handlers, and placeholders.
4. Validate against `core/schema/workflow.schema.json` (IDE schema association or `jsonschema` CLI if available).
5. Save under `.s_e_e/workflows/definitions/` on the hub (or use the GUI Workflow editor; keep JSON in sync).
6. Dry-run from Automation with minimum `runtime_inputs`. Fix any engine validation error and repeat.

## Related skills

- Implement a story end-to-end: `../work/SKILL.md`
- Author or edit prompt bodies referenced by `{{prompt.*}}`: prompts under `.s_e_e/prompts/` (see `doc` and `stories` skills).
- Seed workflows index: `core/initial_data/workflows/README.md`
