## Scope and authority

This revision replaces the single-file aggregate architecture in the earlier doc-5 and m-44. Corrective implementation belongs to m-47. Research supports research, reports, analysis and guides. This is the sole current data and interface contract; implement it directly across storage, APIs, tools, MCP and views. Existing completed stories are historical evidence, not approval of the rejected architecture. This is a plan; no code, installed data, publication state or runtime configuration is changed by it.

## Required storage boundaries

Under project Knowledge/research/{research_id}/:
- manifest.json: id, kind, profile_version, title, audience, purpose, key_message, scope, basis, tags, created/updated metadata, optional reporting_context, ordered page_keys. No page bodies, section bodies, embedded sources or editions.
- pages/{page_key}/manifest.json: stable page_key, title, summary, ordered section_keys, timestamps. No section bodies.
- pages/{page_key}/sections/{section_key}.json: one stable section_key, role, heading, exact Markdown content, source_ids, evidence_basis/note, none_reason and existing typed action_items. Section content remains a string within its own independently stored JSON file; it is never nested into the stored page/document manifest.
- sources/{source_id}.json: one complete bibliography source with existing fields. Identity and repeated chapter-specific records are preserved.
- editions/{edition_id}/: immutable manifest.json, pages/.../manifest.json, separate section files and source files with the same layout. The edition catalogue contains references and dates, never embedded document snapshots.
- .transactions/: recoverable structural-operation journals and staging; .locks/: document structural lock and per-record locks. These are internal and excluded from lists/search.

Keys retain current lowercase ASCII letter/digit/hyphen rules. Resolve and validate project/id/key segments before filesystem access; reject traversal, mismatched identity and symlink escapes. All paths above are proposed new layout. No per-page duplication of the entire bibliography. No aggregate ResearchDocument in ordinary reads, saves or KnowledgeSnapshot. Export/import may stream separate records but cannot become a hidden aggregate mutation bypass.

## Scoped saves and consistency

Each document manifest, page manifest, section file and source file has its own opaque conflict token derived from its canonical stored bytes. Updating section A changes A and its derived index entry, not section B, page manifest or document manifest. Concurrent edits to different sections succeed; stale edits to the same section fail with 409 and affected identity/current token. Metadata edits and reorder operations conflict only on the affected manifest. Metadata updated_at describes that record; library last activity is derived. A confirmed kind change validates the document metadata locally and returns needs_validation=true without scanning all sections. Complete role/evidence diagnostics come from the explicit validate operation.

Acquire shared document structural lock plus exclusive affected-record lock for independent content/metadata changes; acquire exclusive document structural lock for child membership changes, deletion and publication. Always acquire document lock before child locks; stable lexicographic child order for multi-file operations. This uses the existing fd_lock foundation, with shared/exclusive behavior covered by focused concurrent tests. Do not rely on HTTP serialization for cross-process correctness.

Atomic single-record replacement uses temporary sibling file, flush and rename. Child create/delete plus parent-order update uses a durable transaction journal, staged payloads and recoverable commit. Readers use shared structural locks and cannot observe committed order pointing at missing files. Recovery completes a durable committed transaction or rolls back a prepared one; injected failures leave one consistent tree. Delete section removes its file and parent reference; page deletion rejects nonempty pages. Source deletion rejects referenced sources. Document deletion requires confirmation and a current tree token and deletes only its own tree; never follows locators to delete external files.

A tree token is calculated from sorted record identities and their content digests for validation, publication and confirmed whole-document deletion only. It is not a conflict token required for every section edit. Publish takes exclusive structural lock, revalidates the exact tree, stages an edition directory, flushes and atomically exposes the edition; catalogue recovery handles interruption. Old editions remain immutable. Validation holds the exclusive document structural lock while streaming sections and computing the tree token, then releases it; it never materializes one giant object.

## Bounded reads and small pages

Document get returns metadata/counts and a document token. Page get returns page metadata/counts and a page token. Lists return summaries with stable IDs, position where relevant, and next_cursor; they never include Markdown bodies, full sources or nested child lists. Section get returns one complete section. Source get returns one complete source. No ordinary operation returns ResearchWorkingCopy containing the full research.

All lists default limit=25, accept 1..100, and use opaque cursors tied to filters/view and the relevant index/order revision. Invalid or stale cursors return explicit errors; no duplicate/omitted rows on an unchanged snapshot. Content reads use view=default|draft|edition (default when omitted). view=edition requires edition_id; edition_id is rejected for other views. default resolves the latest published edition, or draft when unpublished. The document catalogue instead uses state=all|draft|published (all when omitted): draft selects documents with no published edition, published selects documents with a published edition. Catalogue state is independent of content view. Resolve default once and carry the resolved edition through navigation. Filtering never rewrites stored order.

Small pages are an authoring rule: one coherent topic per page, normally 3-8 sections and roughly 1500 words or fewer. These are advisory warnings, not truncation/rejection rules. Long pages remain readable; size guidance never authorizes truncating content. Editor loads only a selected section body. Reader initially loads five sections of the selected page and offers Load more in batches of five; linked section anchors fetch their target directly. Loading another page must not fetch all other page bodies.

## Search

Research search is a named domain operation, HTTP route, native tool, MCP tool and UI destination. Index document/page titles, section headings/Markdown and linked source titles into section-level entries with document/page/section identities, kind, role, tags, draft/edition and order. Keep short snippets, no full bodies in search results. Query is required and limited to 256 characters; case-insensitive Unicode text matching uses AND across whitespace-separated terms, no regex execution. Title/heading matches rank before body-only matches, then document id/page position/section position/key stabilize ties. Snippets are at most 240 Unicode characters, with highlighted ranges or safe escaped text. Default 25/max100 hits. Filters: research_id, page_key, kind, role, tag, view, edition_id. edition_id requires research_id; omitted view searches each document's latest published edition or draft when unpublished, never both copies. A source title change updates entries for sections referencing it; source access filtering belongs to source_list.

The derived search/summary index is rebuildable from separate files and persists no second canonical content aggregate. Ready summary queries do not open section body files. A missing summary projection is rebuilt by streaming individual records and discarding bodies before serving summary queries. Section writes invalidate only their entry; structural changes update affected positions; publication indexes the new edition. Startup/recovery reconciles index revisions before serving a result or reports rebuilding; it must not silently return stale/deleted content. Existing workspace search gains section-level Research links and honors current project/access filters.

## Interfaces

HTTP base B = /api/projects/{project}/knowledge/research.
Every native operation and MCP tool below is named knowledge_research_<suffix>, requires project, and uses the same domain service. Target fields use research_id, page_key, section_key, source_id and edition_id. HTTP path names id/page/section/source correspond to these fields. Replace the current Research routes, schemas and callers directly with this single contract. Metadata endpoints reject nested child bodies. The public GUI route URLs remain stable.

| Operation suffix | HTTP | Path after B | Scope |
| --- | --- | --- | --- |
| profile_get | GET | /profiles/{kind} | Versioned profile descriptor |
| list | GET | / | Document summaries only; kind,tag,query,state filters |
| get | GET | /{id} | Document metadata and counts only |
| create | POST | / | Metadata only; allocate id |
| put | PUT | /{id} | Metadata only with document token |
| delete | DELETE | /{id} | Explicit confirm and current tree token; delete exact document |
| page_list | GET | /{id}/pages | Page summaries; query filter and explicit positions |
| page_get | GET | /{id}/pages/{page} | Page metadata and section count only |
| page_create | POST | /{id}/pages | Create metadata and append key atomically; document token |
| page_put | PUT | /{id}/pages/{page} | Page metadata only; page token |
| page_delete | DELETE | /{id}/pages/{page} | Empty page only; page and document tokens; no cascading deletion |
| pages_reorder | PUT | /{id}/page-order | Exact ordered page keys; document token |
| section_list | GET | /{id}/pages/{page}/sections | Section summaries; role,query filters and explicit positions |
| section_get | GET | /{id}/pages/{page}/sections/{section} | Exactly one section with Markdown, evidence and section token |
| section_create | POST | /{id}/pages/{page}/sections | One section and append key atomically; page token |
| section_put | PUT | /{id}/pages/{page}/sections/{section} | Exactly one section; section token |
| section_delete | DELETE | /{id}/pages/{page}/sections/{section} | Explicit confirm; section and page tokens |
| sections_reorder | PUT | /{id}/pages/{page}/section-order | Exact section keys; page token |
| source_list | GET | /{id}/sources | Source summaries; query,access_status,locator_kind filters |
| source_get | GET | /{id}/sources/{source} | Exactly one source record and source token |
| source_create | POST | /{id}/sources | One source; create-if-absent stable source_id |
| source_put | PUT | /{id}/sources/{source} | Exactly one source; source token |
| source_delete | DELETE | /{id}/sources/{source} | Confirm and source token; reject if referenced |
| search | GET | /search | Bounded section hits; document,page,kind,role,tag,view,edition filters |
| validate | POST | /{id}/validate | Consistent complete-tree validation; paged diagnostics and tree token |
| publish | POST | /{id}/publish | Current validated tree token; immutable separate-files edition |
| edition_list | GET | /{id}/editions | Edition summaries only |
| edition_get | GET | /{id}/editions/{edition} | Edition metadata and counts only |

Create operations allocate document identity through SEE and validate caller-provided stable child keys with create-if-absent semantics; duplicates return 409, never overwrite. Ordinary put updates existing objects only. Order requests contain exactly all current child keys once and return the affected manifest token. Source creation is serialized with membership-sensitive validation but does not rewrite the document manifest.

Mutation responses contain only affected records/tokens and bounded diagnostics. Error envelope has code, affected IDs, field path, remedy and current affected token where allowed: 400 invalid keys/order/filter/cursor/unknown child bodies; 404 absent identity/view; 409 stale token/already exists/referenced source/nonempty page/tree changed; 422 invalid content or publication. Auth/project errors retain existing semantics and disclose no foreign identities. Validate returns diagnostics pages tied to tree token; publish always recomputes and validates, never trusts a prior success. Page/section create/delete return both affected record and parent token when the parent changes.

Named tools, operation registry, HTTP bindings, typed Rust API client, item actions and in-process workflow dispatch advertise all 28 operations. No tool bypasses validation or project grants. Resource templates mirror summary index, document metadata, pages, page metadata, sections, individual sections, sources and source records, editions and exact-edition counterparts under see:/projects/{project}/knowledge/research. Paginated resources expose continuation URIs. Resource reads never mutate, publish or fetch external sources.


Exact resource templates (R = see:/projects/{project}/knowledge/research):
- R/profiles/{kind}
- R/index?limit={limit}&cursor={cursor}&kind={kind}&tag={tag}&query={query}&state={state}
- R/{research_id}/manifest?view={view}&edition_id={edition_id}
- R/{research_id}/pages?limit={limit}&cursor={cursor}&query={query}&view={view}&edition_id={edition_id}
- R/{research_id}/pages/{page_key}/manifest?view={view}&edition_id={edition_id}
- R/{research_id}/pages/{page_key}/sections?limit={limit}&cursor={cursor}&role={role}&query={query}&view={view}&edition_id={edition_id}
- R/{research_id}/pages/{page_key}/sections/{section_key}?view={view}&edition_id={edition_id}
- R/{research_id}/sources?limit={limit}&cursor={cursor}&query={query}&access_status={access_status}&locator_kind={locator_kind}&view={view}&edition_id={edition_id}
- R/{research_id}/sources/{source_id}?view={view}&edition_id={edition_id}
- R/{research_id}/editions?limit={limit}&cursor={cursor}
- R/{research_id}/editions/{edition_id}/manifest
Optional query parameters are omitted when absent, URI-encoded when present. Exact edition content uses the same page/section/source templates with view=edition and edition_id; default resolution returns the resolved view in metadata and continuation links. Search uses the named search tool/HTTP endpoint; resources do not invent a second search language. Profiles and edition listing do not accept irrelevant content-view fields.

## Existing data and source material

The application is pre-production. Existing installed-data issues are handled by the user and do not create implementation prerequisites or acceptance gates. Implement the separate-file design directly. Do not change installed research as part of these stories.

The authored source corpus remains /home/fang/code/rust-build-research/content/chapters.json and content/*.html; dist is not canonical. The importer uses explicit scoped page, section and source operations when invoked. Preserve supplied Markdown, tables, code, citations and source context through those operations. No automatic import or rewriting of installed content is part of this milestone.

## Views and editing

Reuse current design-system components and the selected editorial concept: normal Knowledge navigation, one Research heading, document overview and numbered page rail, distinct on-page outline, Markdown reading column, previous/next/contents controls and narrow-width contents disclosure. No whole bibliography beneath every paragraph. Inline citation links open selected source details; evidence details are contextual disclosures. Unknown/deleted anchors show a clear error, never silently jump to unrelated content.

Library uses paginated document summaries; overview uses document metadata plus paginated page summaries. Reader loads selected page/section batches and referenced source details only. Search results link directly to page/section with resolved view/edition and preserve filters/back navigation.

Editor has persistent contents, a selected section writing area and separate document settings/source views. No eagerly loaded whole-document signal. Page and section navigation preserve unsaved buffers by stable identity. Explicit Save section, Save page details, Save document settings and Save source actions show their own saving/saved/error state and conflict response. Dirty navigation offers Save, Discard or Stay for that scope. Publish requires all local dirty scopes saved, runs current-tree validation and links diagnostics to affected sections. Ordering has a direct destination control and preserves all other relative order. Long metadata uses multiline controls; stable keys live in details. Sources use paginated search and a single selected record, linked-source chips plus searchable picker for a section. Unfinished editor edits in the working tree must be reconciled, not trusted as complete; current Button/ButtonVariant import errors are known source defects and belong to the first editor story.

## Profiles


Profiles are versioned machine-readable descriptors owned by the domain validation layer. Version 1 contains four profiles. The API, MCP prompt, form controls, validation and rendered type labels use that same descriptor; no second prompt-only checklist defines different rules.

| Kind | Required section roles across the complete document | Additional requirements |
| --- | --- | --- |
| research | question, method, findings, limitations, synthesis | Findings declare their evidence basis; distinguish observed behavior, reported results and hypotheses. |
| report | summary, results, exceptions, actions | Reporting context required; results refer to that period/as-of basis; actions carry an owner or explicitly unassigned ownership and a due date or condition. |
| analysis | problem, criteria, evidence, alternatives, assumptions, recommendation | Compare alternatives against criteria; state conditions that could change the recommendation. |
| guide | outcome, prerequisites, procedure, verification, troubleshooting | Ordered steps, observable success and a recovery/troubleshooting path; distinguish examples from executed procedures. |

Common optional roles are context, references, appendix, and open_questions. Required roles can appear on one page or across multiple pages and may repeat when the subject needs it. There is no fixed chapter count. Titles need not literally match role names. An empty exceptions, actions, prerequisites or troubleshooting case must explicitly state none/not applicable and give the reason instead of silently dropping the role.

Role coverage and Markdown presence are necessary, not sufficient for truthful or useful content. The producer instructions require a coherent argument or procedure, concrete context, substantive sections, appropriate support and explicit limitations. No minimum word count or repeated generic filler substitutes for that review. Each section declares an evidence_basis of source_backed, author_reasoning, illustrative, or not_applicable with a note; source_backed requires source_ids. Reports and guides are not forced to invent research questions or external citations.



## Delivery and verification boundaries

m-47 owns the corrective implementation; m-44 and its Done stories remain historical. Each new story contains one small outcome, real dependency IDs, explicit scope, affected files and automated pass/fail criteria. No browser, human appearance review, real app/server launch, installation, live content mutation or unbounded retry belongs to story acceptance. Focused Rust in-process/domain/HTTP-router/MCP-dispatch/VDOM checks use temporary fixtures and existing caches. Source text searches cannot prove runtime behavior. No separate display or human-review gate is part of this milestone.

No checks run during this planning task. Future workers run only focused affected checks with CARGO_BUILD_JOBS=2 and finite timeouts; the assigned workflow final task owns the normal repository gate once. Story-866 finishes from focused combined-case evidence and must not wait on the later workflow task. Standalone integration owns one normal-gate run. Never require quality-full per story. Existing workflow roots and final quality ownership were read; full effective command selection is not revalidated and execution compatibility is not claimed.

Evidence inspected: knowledge/src/store/research.rs embeds draft and editions and rewrites aggregate on page put; shapes/src/wire/knowledge_entities/research*.rs nests pages/sections and returns aggregate working copies; knowledge/src/snapshot.rs retains full aggregates; research_import_store.rs performs aggregate replacement; server_write/src/knowledge_http_api/research.rs and mcp/src/tools/research.rs expose 11 aggregate-based operations; gui_knowledge/src/pages/knowledge/research*.rs exposes roles/evidence inline and loads/saves all editor pages. This proves the required delta, not implementation completion.

## Implementation ownership

Story-824 supplies the shared tree-digest and atomic transaction primitives. Story-825 defines scoped reads and summary-projection consumption using seeded projections in its isolated fixtures. Story-826 owns metadata-only document creation and metadata saves, including needs_validation on kind change. Story-831 owns reload/recovery and confirmed exact-document deletion. Story-838 integrates summary-projection maintenance/rebuild. Story-832 owns full-tree diagnostics. Story-837 uses the document-creation and recovery producers for the explicit scoped importer. Story-842 exposes document operations after their domain producers. Story-848 depends on section routes for its exact-edition content fixture. GUI reader story-859 consumes HTTP/client contracts and does not depend on MCP resources. Story-861 uses validation routes after kind changes. Story-866 includes story-837 in its combined evidence prerequisites.
