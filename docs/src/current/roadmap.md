---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-08
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
- v0.21 made the algorithm ledger a source-map validation gate.
- v0.22 promoted declared `Schedule Value` and dry-bulb hourly conformance
  using timestamp-aligned ESO comparisons.
- v0.23 promoted official `1ZoneUncontrolled` static EIO model evidence for
  declared surface, construction/material, and OtherEquipment nominal fields
  without claiming dynamic heat-balance compatibility.
- v0.24 hardened runtime state, output registry, meter registry, result
  storage, diagnostics, and profiling scaffolds without adding new numerical
  conformance.
- v0.25 generalized opaque no-mass heat-balance boundary handling for declared
  variables only.
- v0.26 promoted the declared `Zone Total Internal Convective Heating Rate`
  hourly series for `internal_gains_001`, without claiming zone temperature
  response, radiant/latent coupling, HVAC, plant, meter, or broad heat-balance
  compatibility.
- v0.27 added a user-facing support coverage report generated with `oodocs`
  from specs and case manifests, without adding new numerical conformance.

The long-term targets remain:

- v1.0: substantial declared compatibility draft
- v2.0: EnergyPlus 26.1 full compatibility mode with evidence
- v3.0: faster modernized successor while preserving compatibility mode

Plans after v0.16 should be added to `specs/milestones.toml` first. Markdown
planning pages are allowed only when a human explanation is needed.

Old milestone planning pages are not retained in the docs tree. Use
`specs/milestones.toml`, release notes, and Git history for historical context.
