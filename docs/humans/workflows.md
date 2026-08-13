# Workflows

A **workflow** package defines tasks the S.E.E. engine runs against a hub or
spoke. Payload file: `definition.json`. Installs to
`.s_e_e/workflows/definitions/&#123;slug&#125;.json`.

Authoring rules live in the bundled **workflow** skill
(`packages/workflow/1.0.0/SKILL.md`) and S.E.E. knowledge docs **doc-13**
(template expansion) and **doc-18** (workflow authoring). This page covers the
library payload shape only.

## Envelope (`definition.json`)

| Field | Purpose |
| --- | --- |
| `id` | Outer id, usually `system-<name>` |
| `name`, `description` | Hub display metadata |
| `content` | Engine workflow object (object or JSON-encoded string on disk) |

## Inner `content`

| Field | Purpose |
| --- | --- |
| `runtime_inputs` | Run-start values; each object with `key`, `input_type`, `required`, `label`; keys must not contain dots |
| `tasks` | Ordered graph of engine tasks |
| `spoke_root` | Required when command tasks use spoke cwd or git actions omit per-task spoke |

### Task types

Each task has exactly one handler under `function`:

- `cli_command`: shell command and args
- `command`: run a catalog command by `command_id` with optional `prompt`
- `user_input`: collect operator input at run start
- `git_action`, `story_action`, `foreach`, `custom`

### Template references

- `prompt.<id>` (`&#123;&#123;prompt.&lt;id&gt;&#125;&#125;`): inject stored prompt text
- `runtime.<key>` (`&#123;&#123;runtime.&lt;key&gt;&#125;&#125;`): value from `runtime_inputs`
- `userinput.<task_id>.value` (`&#123;&#123;userinput.&lt;task_id&gt;.value&#125;&#125;`): earlier `user_input` task
- `task.<task_id>.output...` (`&#123;&#123;task.&lt;task_id&gt;.output...&#125;&#125;`): captured output from an earlier task

## Reference example

From `packages/implement-story/1.0.0/system-implement-story/definition.json`
(inner `content` pretty-printed):

```json
{
  "id": "system-implement-story",
  "name": "Implement story",
  "description": "Runs a Cursor CLI agent to implement a single story.",
  "content": {
    "id": "implement-story",
    "name": "Implement story",
    "runtime_inputs": [
      {
        "input_type": "string",
        "key": "story_id",
        "label": "Story id",
        "required": true
      }
    ],
    "tasks": [
      {
        "id": "implement_story",
        "name": "Implement story (Cursor CLI)",
        "function": {
          "name": "command",
          "input": {
            "command_id": "cursor-agent",
            "config": { "cursor_model": "composer-2.5" },
            "prompt": "{{prompt.system-implement-story}}"
          }
        },
        "next_tasks": [
          {
            "id": "report_done",
            "name": "Report completion",
            "function": {
              "name": "cli_command",
              "input": {
                "command": "echo",
                "args": [
                  "implement-story finished for {{runtime.story_id}}"
                ]
              }
            },
            "next_tasks": []
          }
        ]
      }
    ]
  }
}
```

On disk the engine may store `content` as a JSON string; both forms validate.

## Install path

```json
{
  "to": ".s_e_e/workflows/definitions/system-implement-story.json",
  "from": "packages/implement-story/1.0.0/system-implement-story/definition.json"
}
```

## Schema reference

- Catalog manifest: [`see.library/v1`](/catalog.json)
- [Repository README](https://github.com/garunski/s_e_e_library/blob/main/README.md)
- [Library Catalog Schema (doc-23)](https://github.com/garunski/s_e_e/blob/main/.s_e_e/knowledge/docs/overview/library/doc-23%20-%20Library%20Catalog%20Schema.md)
