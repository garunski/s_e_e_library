## Definition homes

The home labels are This machine and Project: Example.

This machine contains definitions for the current OS user.

Machine definitions are stored under ~/.s_e_e/definitions/.

Workflow runs, logs, schedules, cycles, and execution history remain per project.

Machine definitions are outside project Git and project backups; back up ~/.s_e_e/definitions/ separately.

A machine workflow runs only after a target project is selected.

Prompts, commands and workflow definitions can live in either home. Duplicate ids and names remain distinct; neither home shadows the other. `machine:review` selects the machine definition; `project:review` selects the target project's definition. A bare `review` remains project-only. Prompt templates use `{{prompt.machine:review}}` or `{{prompt.project:review}}`; legacy `{{prompt.review}}` stays project-only. Missing or incompatible selections stay flagged for repair and are never replaced by the same id in another home.

Machine workflows must omit `spoke_root` everywhere in their content and cannot depend on project-only prompts or commands. Their working context comes from the selected project at launch. Project workflows may use either home. Schedules, cycle stages and project defaults store explicit references but keep their project owner. A run captures definition content, exact home and revision; later edits or deletion do not rewrite that snapshot. Legacy runs with no captured provenance remain unknown.

Copy previews identify the source, destination, concrete paths and dependency rewrites. Review renamed dependencies and explicit reuse before applying the preview token. Copying leaves the original and existing bindings intact. Editing a machine definition affects future uses across projects. Deletion checks project usage; stale or inaccessible usage blocks deletion. Existing runs retain captured content.

Client grants separate project access from machine definition read/edit permissions. Machine access defaults off; edit implies read. A project-only worker token cannot edit machine definitions. Machine prompt discovery requires machine read and an explicit authorized target project for expansion; project access alone does not grant machine access.

The workflow guide examples use one machine workflow with two different explicit project targets in `workflow_execute` arguments. They illustrate request structure; execution ownership is verified separately. No machine execution endpoint is introduced.

See [prompt details](/s_e_e_library/authoring/prompts/), [command details](/s_e_e_library/authoring/commands/) and [Library destinations](/s_e_e_library/authoring/bundles/).

Definition search retains exact home, kind, path and editor destination for duplicate names. Machine-only search works without selecting a project and exposes only authorized prompt, command and workflow metadata, never project business data. HTTP uses `/api/machine/definitions/search`; selected-project search stays under `/api/projects/{code}/search`. Denied machine access and unauthorized projects do not expose titles, snippets or counts. Reopening search refreshes definition revisions.
