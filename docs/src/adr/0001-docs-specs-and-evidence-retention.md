---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-10-20
---

# Docs, Specs, and Evidence Retention

## Decision

The mdBook source tree keeps only current operating documentation, generated
reference pages, and active ADRs. Old planning documents are not retained under
`docs/src/archive`.

Historical planning is recovered from Git history and release notes. Frozen
release evidence is generated outside the mdBook source tree; curated public
PDFs are published as GitHub Release assets instead of treating every evidence
format as ordinary mdBook content.

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
- Keep current scope, durable state/branch contracts, known limitations and the
  command needed to verify a result. Replace obsolete explanations instead of
  appending progress journals or repeated checkpoint tables.
- Each decision/table has one authority: specs for machine-readable state,
  current docs for operating context, guides for procedures, and source maps
  for durable ownership details. Other pages link to it rather than copying it.
- Accepted scientific evidence retains the input/source identities, policy,
  actual commands, result and limitations needed to substantiate the claim.
  Raw execution logs live outside the mdBook source tree. Failed outcomes that
  still limit the current claim remain explicit; chronological attempts and
  replaced prose are recovered from Git or the relevant immutable evidence.
- Release evidence is generated under `.runtime/release-evidence/vX.Y.Z`.
- GitHub Releases upload only the binary package and curated public evidence
  PDFs by default; local HTML, JSON, and Markdown evidence remains available for
  auditing and debugging.

## Consequences

Release verification must not depend on archived planning pages. Guard scripts
must assert current/spec/release-note claim boundaries instead of old archive
files.
