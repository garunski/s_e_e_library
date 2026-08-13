# Commands

A **command** package defines how the workflow engine runs an external agent CLI
against a spoke workspace. Payload file: `command.json`. Installs to
`.s_e_e/commands/&#123;id&#125;.json`.

## Fields

| Field | Purpose |
| --- | --- |
| `binary` | Executable name on `PATH` (e.g. `cursor`, `claude`) |
| `base_args` | Arguments always passed before operator overrides |
| `forced_args` | Arguments the runner appends; not overridable |
| `params` | Typed run-time knobs (`string`, `number`, `boolean`, `enum`, `string_array`) mapped to env, timeout, extra args, lock policy, or stream toggle |
| `result` | Success rule (`success_when`), captured fields, optional `applied_paths` and `run_id` strategies |
| `execution` | Working directory (`cwd`: `spoke` or `hub`), lock scope, stream format, `non_interactive` |

Additional fields: `id`, `name`, `icon`, `workspace_arg`, `prompt` delivery,
`env`, and optional `mcp` provisioning.

## Reference example

From `packages/cursor-agent/1.0.0/command.json`:

```json
{
  "id": "cursor-agent",
  "name": "Cursor Agent",
  "binary": "cursor",
  "base_args": ["agent", "-p", "--trust", "--force"],
  "forced_args": ["--output-format", "stream-json", "--stream-partial-output"],
  "params": [
    {
      "key": "cursor_model",
      "type": "string",
      "default": "composer-2.5",
      "map": { "target": "env", "name": "CURSOR_MODEL" }
    },
    {
      "key": "timeout_secs",
      "type": "number",
      "default": 1800,
      "map": { "target": "timeout" }
    }
  ],
  "result": {
    "success_when": "exit_code == 0",
    "capture": ["run_id", "exit_code", "duration_ms", "stdout_tail", "stderr_tail", "truncated"]
  },
  "execution": {
    "cwd": "spoke",
    "lock": { "scope": "spoke", "default_policy": "force" },
    "stream": { "format": "stream-json" },
    "non_interactive": true
  }
}
```

## Install path

Catalog `files[]` entry:

```json
{
  "to": ".s_e_e/commands/cursor-agent.json",
  "from": "packages/cursor-agent/1.0.0/command.json"
}
```

The `to` path must equal `.s_e_e/commands/&#123;id&#125;.json` where `id`
matches `command.json` `id`.

## Schema reference

- Catalog manifest: [`see.library/v1`](/catalog.json)
- [Repository README](https://github.com/garunski/s_e_e_library/blob/main/README.md)
- [Library Catalog Schema (doc-23)](https://github.com/garunski/s_e_e/blob/main/.s_e_e/knowledge/docs/overview/library/doc-23%20-%20Library%20Catalog%20Schema.md)
