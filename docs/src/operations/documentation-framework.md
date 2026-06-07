---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-07
---

# Documentation Framework

Author-maintained project documentation lives in the mdBook source tree under
`docs/src`. Generated release and evidence documents are built through the
repo-local Python reporting environment and `oodocs`.

## Standard Shape

Scripted documentation follows this shape:

```text
scripts/<area>/<document>.ps1
  -> scripts/lib/python.ps1
  -> .runtime/python/3.11.9
  -> .runtime/python-venvs/report
  -> tools/reporting/<document>.py
  -> oodocs + matplotlib
  -> .runtime/<artifact-root>
```

PowerShell entry points are orchestration wrappers. They may parse command-line
arguments, locate the repo root, provision Python, and invoke the generator.
They should not own document layout, chart logic, or serialization.

Python generators own document structure and artifact writing. They should use
`oodocs` for HTML/PDF document serialization and matplotlib figure objects for
charts when charts are needed.

## Dependency Rules

Reporting dependencies are pinned in:

```text
tools/python/requirements-report.txt
```

The setup path provisions portable Python and the report virtual environment so
another Windows PC can reproduce release document generation without relying on
ambient Python packages.

## Artifact Rules

Generated release documents should emit machine-readable evidence plus a
human-facing document:

| Artifact | Purpose |
|---|---|
| JSON | durable evidence data and automation input |
| HTML | inspectable local preview |
| PDF | release-facing evidence package |

Where a document is not release evidence, the generator should still make its
artifact root explicit and keep any intermediate data deterministic.

## Adding a New Generator

For a new scripted document:

1. Add a thin entry point under `scripts/release` or the relevant script area.
2. Put document logic in `tools/reporting/<name>.py`.
3. Reuse `scripts/lib/python.ps1` to locate the repo-local report Python.
4. Add dependencies only through `tools/python/requirements-report.txt`.
5. Generate JSON plus HTML/PDF when the document is release or evidence facing.
6. Document the command in `docs/src/operations/script-index.md` or the
   relevant operations page.
7. Add a smoke or release gate when the artifact supports a public claim.

Avoid one-off PowerShell renderers, unpinned local packages, and LaTeX-heavy
toolchains unless the project explicitly changes this policy.
