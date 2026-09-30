---
name: workflow
description: Create or revise S.E.E. workflow definitions or Library workflow packages. Use for workflow task graphs, runtime inputs, command selection, user input steps, and template references.
---

# S.E.E. workflow authoring

Use this skill for the workflow graph and its execution contract. Read applicable repository `AGENTS.md` for local constraints. Keep repository-wide rules there, reusable task procedure in a skill, and run-specific instructions in the workflow task or its named prompt. Include only what a task needs to execute.

## Choose the target

- For an installed project workflow, resolve `project`, inspect nearby definitions with `workflow_list` and `workflow_get`, and write with `workflow_save`. Direct edits to the hub's stored workflow JSON are unsupported. Read the saved definition back.
- For a Library package, edit its payload under `<spoke:s_e_e_library>/packages/{slug}/{version}/definition.json` and its catalog metadata. Use a new package version for a published change and update bundles that embed the payload. An installed project definition does not change merely because a Library package changed.
- If the workflow needs a command, use a command definition installed for the project and suited to the requested technology and CLI. Do not assume `cursor-agent` fits every task. Declare the command package dependency in Library metadata where needed.

## Shape the graph

- Give each task one clear effect and explicit input. Use `command` for an installed agent command, `cli_command` for a deterministic command, `s_e_e_action` for one MCP tool, and `user_input` where the run must pause for an operator answer. Use `foreach` or `repair_loop` only when the repeated behavior needs those engine handlers.
- Declare each run-start value in `content.runtime_inputs`. A task may use `{{runtime.KEY}}`; keys contain no dots. A command with spoke cwd needs `content.spoke_root`.
- `spoke_root` may be an absolute directory or the exact `{{runtime.KEY}}` reference to a string input containing an absolute directory. The engine resolves and validates it at run start and again after a pause.
- A later task may use `{{userinput.TASK_ID.value}}` only after that `user_input` task. Use `capture_output: true` on a producer whose `{{task.TASK_ID.output...}}` fields are read later.
- When an inspection task generates questions, make it return named fields in a JSON object, capture its output, and bind `user_input.prompt` to the question field. Capture the `user_input` output too so the expanded prompt is visible in the pending input request. The following task reads the answer through `{{userinput.TASK_ID.value}}`.
- Set `run_when` and dependencies so a consumer cannot run before its producer. Keep the graph's branch and loop shape within the engine schema.

## Validate and deliver

- Validate the workflow payload against `<spoke:s_e_e>/core/schema/workflow.schema.json` and inspect the Library workflow schema when packaging. Check every referenced prompt id, command id, runtime key, task id, root path, and MCP tool against what the project actually provides.
- Use `workflow_parse` when authoring an installed definition. Save with `workflow_save` and read it back. A Library package must have a matching install path and dependency list in the catalog.
- Run the workflow only when the user requests execution or an actual run is needed to verify a specific behavior. A parse or schema check does not prove that external commands and integrations work; report that limit plainly.
- Report the workflow id, target project or package, command dependencies, required run inputs, and any unverified external behavior.
