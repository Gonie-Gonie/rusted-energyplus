---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Roadmap

The roadmap is spec-led. The canonical milestone list is maintained in
`specs/milestones.toml`, and the readable index is generated at
`docs/src/generated/milestone-map.md`.

Current direction:

- v0.16 reset the evidence and versioning contract for Road to v1.0.
- v0.17 made manifest schema v2 a blocking validation gate.
- v0.18 connected manifest-owned output-request injection to the oracle
  baseline pipeline.
- v0.19 hardened selected series readers and comparison metrics.
- v0.20 promoted release conformance index generation and coverage matrices.
- v0.21 should make the algorithm ledger a stronger source-map gate.

The long-term targets remain:

- v1.0: substantial declared compatibility draft
- v2.0: EnergyPlus 26.1 full compatibility mode with evidence
- v3.0: faster modernized successor while preserving compatibility mode

Plans after v0.16 should be added to `specs/milestones.toml` first. Markdown
planning pages are allowed only when a human explanation is needed.

Old milestone planning pages are not retained in the docs tree. Use
`specs/milestones.toml`, release notes, and Git history for historical context.
