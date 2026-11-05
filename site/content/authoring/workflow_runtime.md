### Worker and run controls

Definitions retain their nested authoring form. Execution uses a flat graph of task IDs and materializes one foreach iteration and one repair attempt at a time. A loop's `wait_for` barrier completes only after the whole loop, including its verifiers, succeeds. An empty successful loop contributes zero action records.

Each run has a supervised worker process. The parent API owns durable execution writes and terminal status. Worker signal exits, nonzero exits, missing terminal exchange and malformed IPC produce a failed run with `worker_aborted` or `worker_protocol_error`. The saved worker failure includes the exit code or signal and a bounded stderr tail. The API and other isolated runs can continue after a worker abort. Host-wide failures can still affect the parent and workers; machine-wide fault isolation is not guaranteed.

**Pause** records a request immediately, stops new admissions and waits for already active steps. The run becomes **Paused** after those steps settle and the checkpoint is saved. Repeated pause requests are idempotent. **Cancel** stops the run and terminates its owned worker and command descendants, with a two-second grace before forced cleanup. Parent disconnect and server shutdown also clean up owned workers and descendants. A control timeout or closed pipe is recorded as cancellation or worker failure, never as a successful pause acknowledgment.

**Resume** continues a safely checkpointed paused run without replaying completed task IDs or losing their outputs. Current checkpoints contain flat graph state, loop cursors, outputs, allocation counters, resource policy and frozen run provenance. Unknown in-flight actions after an interruption are recorded as failures and are not automatically replayed. Reconcile uncertain external side effects before starting replacement work. Exactly-once external effects are not guaranteed. The engine reads the current flat checkpoint format only.

### Progress fields

HTTP execution reads and MCP `execution_get` expose the same `runtime_progress` object. `status` remains authoritative; successful worker startup does not mean Complete.

| Field | Meaning |
| --- | --- |
| `materialized_tasks`, `completed_tasks` | Actual action IDs currently materialized and completed; templates and loop control barriers are excluded |
| `total_tasks` | `null` while future loop work remains unknown; consumers must omit a percentage instead of showing 100 percent from current materialization |
| `generated_nodes`, `generated_edges` | Lifetime allocations, retained across resume |
| `retained_nodes`, `retained_edges` | Current retained graph size, including templates and controls |
| `loops[].collection_size` | Known foreach collection size, or `null` until its producer resolves |
| `loops[].current_iteration` | Zero-based active foreach index, or `null` |
| `loops[].completed_iterations` | Successfully completed foreach iterations; after item zero of 39 this is 1 of 39 and the barrier is pending |
| `loops[].active_attempt` | One-based active repair attempt, or `null` |
| `loops[].barrier_pending` | Whether the complete loop barrier is still pending |
| `pause_requested` | A request is pending; this differs from a committed Paused state |
| `worker_pid`, `worker_failure` | Owned worker identity and bounded failure diagnostics, when available |

### Resource limits

The policy is selected at launch and preserved on resume. Limits are checked before graph mutation or semantic output retention. A rejected semantic result is not silently truncated into apparently valid task output.

| Limit name | Default |
| --- | --- |
| `authored_bytes` | 8 MiB |
| `authored_depth` | 64 |
| `generated_nodes` | 100,000 over the run's lifetime |
| `generated_edges` | 200,000 over the run's lifetime |
| `foreach_items` | 10,000 per collection |
| `active_handlers` | 32 |
| `task_result_bytes` | 1 MiB encoded structured result |
| `retained_result_bytes` | 64 MiB aggregate retained results |
| `checkpoint_bytes` | 128 MiB encoded checkpoint |

`resource_limit_exceeded` identifies the limit, allowed amount, attempted amount, task ID and execution ID. An HTTP request rejected before acceptance uses the `{error, code?}` envelope. An accepted run reports finite failure details in `errors`; worker failures also use `runtime_progress.worker_failure`. Retained prior outputs and completed evidence remain available after a limit failure.

Worker IPC has a 1 MiB frame body, a 64-frame bounded queue and a 128 MiB total chunked-checkpoint limit. Worker stderr retention uses three 5 MiB files and a 64 KiB diagnostic tail, with dropped-byte accounting. Diagnostic tails may omit bytes; semantic captured JSON must remain complete or fail.
