## Library destinations

This machine is available only when the complete package/dependency/member closure contains prompts, commands, workflows or definition-only bundles. A mixed bundle containing a skill, starter, cycle, schedule, rule set or knowledge document is project-only. Preview the concrete destination before installing. Project: Example keeps existing `.s_e_e/` package paths; This machine maps definition files into `~/.s_e_e/definitions/` and keeps independent installed state there. Package metadata and old project installations remain compatible; startup does not migrate or seed machine definitions.

Package workflow `spoke_root` values are removed with a visible preview when copying into the machine home. Direct machine workflow saves must omit `spoke_root`. Dependencies are rewritten to machine references; explicitly review reuse or renamed definitions. Copy/install leaves the source and existing bindings intact.

The same package can be installed independently in both homes. Update/uninstall retains the reviewed home even after project switching. Local edits require acknowledgement; usage, inaccessible projects and stale previews can block removal. Project backups do not contain machine definitions: back up `~/.s_e_e/definitions/` separately. Runs, logs, schedules, cycles and execution history remain project-owned.
