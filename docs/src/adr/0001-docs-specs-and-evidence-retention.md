---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-07
---

# Docs, Specs, and Evidence Retention

## Decision

The mdBook source tree keeps only current operating documentation, generated
reference pages, and active ADRs. Old planning documents are not retained under
`docs/src/archive`.

Historical planning is recovered from Git history and release notes. Frozen
release evidence is published as GitHub Release assets instead of being treated
as ordinary mdBook content.

## Rationale

Long-lived planning Markdown makes navigation noisy and can blur claim
boundaries. Current decisions should live in short current docs, machine-
readable specs, or ADRs. Generated reference pages may summarize specs, but
they are not evidence.

## Policy

- Current human docs live under `docs/src/current` and `docs/src/guides`.
- Machine-readable planning, claim, coverage, and algorithm state lives under
  `specs/`.
- Generated reference docs live under `docs/src/generated` and are checked for
  staleness.
- ADRs record durable process or architecture decisions.
- Old plan/readiness Markdown is removed from the tree.
- Release evidence is generated under `.runtime/release-evidence/vX.Y.Z` and
  uploaded to GitHub Releases as assets.

## Consequences

Release verification must not depend on archived planning pages. Guard scripts
must assert current/spec/release-note claim boundaries instead of old archive
files.
