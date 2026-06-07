---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-05
---

# Release Process

Before any release:

- state the oracle version
- state the evidence level
- list supported evidence
- include a `Claim Boundary` section with supported claims and non-claims
- generate the release PDF/HTML/JSON evidence pack for promoted numerical cases
- include docs in the release artifact
- run `scripts/dev.cmd check`
- run `scripts/dev.cmd strict-no-false-conformance`
- run the milestone-specific verify script

Compatibility claims require conformance-level evidence. Engineering progress
can be released without such a claim if the release notes say so clearly.

## Required Claim Boundary Section

Every release note must include a section named `Claim Boundary`. It must state:

- new numerical conformance cases, if any.
- promoted historical cases, if any.
- evidence level for smoke, baseline-only, diagnostic-only, conformance,
  performance, or regression artifacts.
- explicit non-claims for major domains that could be misread from the release
  title or feature work.
