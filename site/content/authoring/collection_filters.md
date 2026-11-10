### Discovery and scope

Call `list_filter_get` with `project` and `collection` before constructing a query. Collections include `stories`, `milestones`, and `knowledge_docs`. The returned descriptor supplies scope, field names, scalar/array types, operators, nullability, and enum choices. Story and milestone status choices come from the project configuration. HTTP discovery uses `GET /api/projects/{project}/list-filters/{collection}`. Application collections use `GET /api/list-filters/{collection}` under application authorization. Discover each collection through its existing authorized surface.

All predicates combine with AND. `in` supplies alternatives for one scalar field. Legacy parameters compose with predicates using AND, so contradictory selections return no rows. Authorization and mandatory scopes, including active-draft, compatible-only and running-only constraints, still apply. Fields are public metadata declared by the descriptor. Body text, secrets and arbitrary JSON paths are unavailable.

### Encoding and operators

Native tools, MCP and workflow tool inputs take a typed `filters` array. HTTP GET uses one URL-encoded JSON array in the `filters` query parameter. For example, `curl --get --data-urlencode 'filters=[{"field":"status","op":"ne","value":"Done"}]' 'http://127.0.0.1:30127/api/projects/s-e-e/stories?milestone=m-45'` encodes a story selection. Native `story_list` calls name the legacy selector `milestone_id`; the HTTP query and Rust `StoryListRequest` name it `milestone`. The descriptor field is `milestone_id` on every surface. The Rust client's `*_with_filters` methods perform the encoding and retain predicates across pages. Resource query templates are advertised by `resources/templates/list` for existing entity resources.

- `eq`, `ne`, `in`, and `not_in` compare same-type scalars. Identifier and enum equality is case sensitive.
- `contains` is a case-insensitive substring on a declared string field, using Unicode lowercase without accent folding.
- `contains_any`, `contains_all`, and `contains_none` compare array membership.
- `gte` and `lte` compare numbers, dates (`YYYY-MM-DD`) or timestamps (RFC3339 with an offset, normalized to UTC).
- `is_set` and `is_unset` omit `value` entirely and are offered only where declared. Missing/null fields fail ordinary predicates, including `ne` and `not_in`. Empty strings and arrays are present values. Null comparison operands are rejected.

Use at most 16 predicates, 100 members per nonempty membership array, 1024 Unicode characters per string operand, and 16384 UTF-8 bytes of decoded filter JSON. Request errors such as malformed JSON, duplicate `filters`, non-array input, or exceeded request bounds have a request-level code and remedy. Predicate diagnostics identify a zero-based `predicate_index`, field, code, and remedy. Correct `unknown_field` from discovery, replace an `invalid_enum` with a descriptor choice, and omit `value` for presence checks. Unsupported operators, wrong types, empty membership arrays and invalid dates are errors.

### Pages, cursors and GUI controls

Selection precedes existing ordering, matching totals and page slicing. Standard offset pages retain `items`, `total`, `offset`, `limit`, and `has_more`, with default limit 50 and maximum 200. Keep predicates on subsequent pages. Other existing responses can be unpaged or omit totals. Use their actual response contract. Native `story_list` traverses matching pages and its text JSON exposes `story_ids` in exactly item order. Pass those IDs directly to `foreach`.

The Board endpoint `GET /api/projects/{project}/stories/columns` applies `limit` and `offset` per status and returns the full matching `stories_total` per status. With `group_by_milestone=true`, paging applies per milestone within each status and `milestone_totals` reports matching counts keyed by milestone ID. The empty key represents unassigned work.

Scoped Research returns `items`, `next_cursor`, and `limit`. Its retained typed filters compose with `kind`, `tag`, `query` and catalogue `state` using AND before pagination. Its cursor is bound to that normalized selection. Keep the selection unchanged while following `next_cursor` and discard the cursor when it changes. Content `view` is separate from catalogue `state`. Protocol catalog cursors from `tools/list`, `prompts/list`, and `resources/list` are separate from entity cursors.

Existing GUI field controls use the same descriptors alongside quick filters. Choose a field and operator, enter a typed value or select enum choices, and remove individual predicates or clear the selection. Changes reset paging/cursors and persist in the URL. Lists distinguish no matching rows from load or access errors. Collection-specific eligibility and saved configuration remain in force.

### Milestone workflow release

The current `work-milestone-stories` bundle release is 1.1.0. Its `system-work-milestone-stories` resolver sends `milestone_id` plus `status ne Done` to native `story_list` and consumes `story_ids` directly. Saved Done/all-checked verification, bounded repair attempts, explicit blockers and one final repository check remain part of the workflow. The final check runs a stored command and uses its exit status.

Review the normal Library upgrade plan. Unmodified defaults can upgrade through that plan. Customized files remain conflicts until an explicit overwrite is chosen. Preserve custom selection and worker instructions when reconciling an upgrade. Historical releases remain available.
