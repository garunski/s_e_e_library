Research in S.E.E. supports research, reports, analysis and guides through 28 scoped operations.

## Project availability

Enable `knowledge_research` in Project settings > Experimental for the explicit target project. It defaults to false independently of Knowledge Cycle and Orchestrator. Disabling preserves documents, pages and immutable editions while omitting Research views, shared search hits, exact-project resources and qualified Research prompts. Subsequent HTTP and MCP operations return `EXPERIMENTAL_FEATURE_DISABLED`. Ordinary knowledge docs and decisions remain available. Mixed-project MCP discovery may advertise tools enabled in another authorized project; invocation still checks the explicit target project.

Discover `knowledge_research_profile_get` and the current tool schemas before authoring. The four runtime profile descriptors are version 1 and define required roles and field rules. Use the qualified `{project}.research-authoring` prompt for the producer procedure.

## Separate files and bounded reads

Under Knowledge/research/{research_id}/, `manifest.json` stores document metadata and ordered page_keys. Each `pages/{page_key}/manifest.json` stores page metadata and ordered section_keys. Section Markdown lives independently in `pages/{page_key}/sections/{section_key}.json`; bibliography records live in `sources/{source_id}.json`. Immutable editions use this same separate-file layout under `editions/{edition_id}/`. Internal .locks and .transactions are excluded from lists and search. Never send a full ResearchDocument or ResearchWorkingCopy to edit one scope.

Document get returns metadata, counts and document_token; page get returns metadata, section count and page_token. Neither read returns child bodies or embedded sources. Lists return summaries, stable identities, positions where relevant, limit and next_cursor. Section get returns exactly one complete section and section_token; source get returns exactly one complete source and source_token.

Lists default to 25 and accept 1..100. Keep filters and the resolved view unchanged while following opaque next_cursor or resource continuation URIs; restart after invalid_cursor or stale_cursor. Content view=default resolves the latest published edition or draft when unpublished. Use view=draft for edits; view=edition requires edition_id, which is rejected for other views. Pin the resolved edition across navigation and continuations. Catalogue state=all|draft|published is independent of content view: draft means never published.

Use list_filter_get for knowledge_research to discover the retained typed metadata predicate envelope: `filters: [{field, op, value}]`. It composes with kind, tag, query and state using AND before pagination. Keep the normalized predicates and selectors unchanged across cursors. HTTP and resource queries encode the predicate array once as JSON in filters; MCP sends the array directly. This is the shared collection-filter language, not a new Research filter DSL. Scoped list responses use items, next_cursor and limit.

Research has its own `knowledge_research_search` and GET /api/projects/{project}/knowledge/research/search endpoint. Query is required, at most 256 Unicode characters, with case-insensitive AND matching of whitespace-separated terms and no regex. Filters are research_id, page_key, kind, role, tag, view and edition_id; edition_id also requires research_id. Hits contain section identities and snippets of at most 240 Unicode characters, never complete bodies. Search defaults to 25, maximum 100. Resource reads never mutate, publish or fetch external sources.

## Authoring and token scopes

Search before creating duplicate material. Plan one coherent topic per page, normally 3-8 sections and roughly 1500 words or fewer. These are advisory warnings. Preserve supplied substantive Markdown, tables, code, citations and long content exactly: do not truncate, summarize or automatically split content to satisfy the guidance. Required profile roles may span pages; keep stable lowercase ASCII letter/digit/hyphen keys when titles change.

Create document metadata only and use the allocated research_id. Create page metadata separately with the current document_token. Create each section separately with the current page_token. Create sources independently before referencing their stable source_ids. Create is create-if-absent; put updates existing records only. Metadata endpoints reject nested child bodies.

| Action | Latest required token fields |
| --- | --- |
| Document settings put | document_token from document get |
| Page create or pages_reorder | document_token |
| Page details put | page_token from page get |
| Empty page delete | page_token and document_token, confirm=true |
| Section create or sections_reorder | page_token |
| Section put | section_token from section get |
| Section delete | section_token and page_token, confirm=true |
| Source create | none; stable create-if-absent source_id |
| Source put or delete | source_token from source get; delete also confirm=true |
| Publish or whole-document delete | tree_token from current validation; delete also confirm=true |

Do not send generic conflict_token to scoped Research operations. Reorder requests contain every current child key exactly once. Page/section membership changes return the affected record and changed parent token. Source creation does not rewrite the document manifest. Nonempty pages and referenced sources cannot be deleted; deletion never cascades.

Read the selected draft section, save exactly that section with section_put and its current section_token, then read back exact Markdown, source IDs, evidence and resulting token. Save document settings, page details and sources independently with their own puts and tokens. On stale_token, reread and reconcile only the affected scope, preserve intervening and unrelated content, then retry with that record's current token. A tree token is never required for an ordinary section edit.

Keep source access truth: inspected, provided, metadata_only or unavailable, with locator, inspected scope and limitations. Never invent inspection claims, measurements or successful procedures. Source-backed sections name existing source IDs; distinguish author_reasoning, illustrative and not_applicable evidence. Reports require reporting context and typed actions with owner/unassigned ownership and due date/condition.

Run knowledge_research_validate, follow every diagnostics page tied to the tree token and repair only affected records at their field paths. If the tree changes during pagination, restart validation. Revalidate after repairs. Structural validation does not prove source truth or argument quality. Publish only when requested, using the current validated tree_token. Publication recomputes and validates the exact tree and creates an immutable edition; Draft save is not publication. Read edition metadata with edition_get and selected content with view=edition and the returned edition_id. Report affected IDs, exact saved readback, remaining diagnostics and Draft or edition state.

## Errors

Scoped errors contain code, affected_ids, field_path and remedy, plus current_token when disclosure is allowed. HTTP uses 400 for invalid_key, invalid_order, invalid_filter, invalid_cursor, stale_cursor, unknown_fields and missing_token; 404 for not_found and view_not_found; 409 for stale_token, already_exists, referenced_source, nonempty_page and tree_changed; 422 for invalid_content and invalid_publication. Authentication and project-grant errors retain existing semantics without foreign identity disclosure. MISSING_PROJECT, PROJECT_NOT_GRANTED and EXPERIMENTAL_FEATURE_DISABLED remain admission errors.

Correct malformed fields or order, restart stale cursors, reread only the affected record for token conflicts, choose an unused identity for duplicate creates, remove references explicitly before deleting a source, and empty a page explicitly before deleting it. A tree_changed publication requires current validation again. Do not publish as automatic conflict or diagnostic repair.
