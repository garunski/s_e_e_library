## Command homes and bindings

Choose This machine or Project: Example before creating a command. Project files remain under the project's `.s_e_e/commands/`; machine files are under `~/.s_e_e/definitions/commands/`. Use `machine:runner` or `project:runner` in workflow defaults, tasks, schedules and cycle stages. Bare ids remain project-only. Same-id commands remain separate; missing or CLI-incompatible choices stay flagged without substitution.

Machine commands can use only machine prompt dependencies. Copy preview identifies dependency rewrites and destination paths before applying. Machine edits affect future uses across projects; current usage and stale previews can block deletion. Authentication settings and server secrets remain application settings rather than definition metadata.

Project command create/update/history/revert use `/api/projects/{code}/definitions/commands`; machine definitions use `/api/machine/definitions/commands`. Reads and writes preserve the selected home; writes require its current conflict token. MCP qualified ids infer scope, while an explicit disagreeing scope is rejected. Machine read/edit grants are independent of project access.
