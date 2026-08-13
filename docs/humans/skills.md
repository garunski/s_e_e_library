# Skills

A **skill** package teaches an agent a repeatable capability. Payload file:
`SKILL.md`. Installs to `.agents/skills/&#123;slug&#125;/SKILL.md`.

## Shape

YAML frontmatter plus markdown body:

- `name`: short skill id (matches the install folder name)
- `description`: activation text; when the user says these phrases, the agent
  should load this skill

The body documents the procedure, constraints, and links to authoritative docs.

## Reference example

From `packages/work/1.0.0/SKILL.md`:

```markdown
---
name: work
description: >-
  Implements an S.E.E. story with tests until acceptance criteria pass...
  Use when the user says work, "start work", "implement story-N", or works
  through story IDs with acceptance criteria.
---

# Work

Implement an S.E.E. story with tests until all acceptance criteria pass...
```

Write the `description` as a single activation sentence agents can match against
user intent. Keep procedures in the markdown body, not in frontmatter.

## Install path

```json
{
  "to": ".agents/skills/work/SKILL.md",
  "from": "packages/work/1.0.0/SKILL.md"
}
```

## Schema reference

- Catalog manifest: [`see.library/v1`](/catalog.json)
- [Repository README](https://github.com/garunski/s_e_e_library/blob/main/README.md)
- [Library Catalog Schema (doc-23)](https://github.com/garunski/s_e_e/blob/main/.s_e_e/knowledge/docs/overview/library/doc-23%20-%20Library%20Catalog%20Schema.md)
