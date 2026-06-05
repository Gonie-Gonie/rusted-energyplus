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
- list non-claims
- include docs in the release artifact
- run `scripts/dev.cmd check`
- run `scripts/dev.cmd strict-no-false-conformance`
- run the milestone-specific verify script

Compatibility claims require conformance-level evidence. Engineering progress
can be released without such a claim if the release notes say so clearly.
