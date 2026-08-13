# Prompts

A **prompt** package stores reusable system prompt text referenced by workflows
and commands via the `prompt.<id>` template form
(`&#123;&#123;prompt.&lt;id&gt;&#125;&#125;`). Payload file: `prompt.json`. Installs to
`.s_e_e/prompts/&#123;id&#125;.json`.

## Fields

| Field | Purpose |
| --- | --- |
| `id` | Stable prompt id (e.g. `system-implement-story`) |
| `name` | Display name in the hub |
| `content` | Prompt body; may include `runtime.*` placeholders (`&#123;&#123;runtime.*&#125;&#125;`) expanded at run time |

## Reference example

From `packages/system-implement-story/1.0.0/prompt.json`:

```json
{
  "id": "system-implement-story",
  "name": "Implement story",
  "content": "You are implementing story {{runtime.story_id}}. Operate fully autonomously..."
}
```

Use a `system-*` id for bundled defaults. Keep template placeholders aligned
with the workflow `runtime_inputs` that declare each `runtime.<key>` placeholder
(`&#123;&#123;runtime.&lt;key&gt;&#125;&#125;`).

## Install path

```json
{
  "to": ".s_e_e/prompts/system-implement-story.json",
  "from": "packages/system-implement-story/1.0.0/prompt.json"
}
```

## Schema reference

- Catalog manifest: [`see.library/v1`](/catalog.json)
- [Repository README](https://github.com/garunski/s_e_e_library/blob/main/README.md)
- [Library Catalog Schema (doc-23)](https://github.com/garunski/s_e_e/blob/main/.s_e_e/knowledge/docs/overview/library/doc-23%20-%20Library%20Catalog%20Schema.md)
