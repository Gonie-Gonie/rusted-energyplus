---
status: active
claim_level: setup
owner: docs
last_reviewed: 2026-06-08
---

# Quick Start

Prepare the local toolchain, EnergyPlus oracle, and docs tooling:

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
```

Run the local check suite:

```powershell
.\scripts\dev.cmd check
```

Verify the current public release gate:

```powershell
.\scripts\dev.cmd v0.15-verify
.\scripts\dev.cmd v0.16-verify
.\scripts\dev.cmd v0.17-verify
.\scripts\dev.cmd v0.18-verify
.\scripts\dev.cmd v0.19-verify
.\scripts\dev.cmd v0.20-verify
.\scripts\dev.cmd v0.21-verify
.\scripts\dev.cmd v0.22-verify
.\scripts\dev.cmd v0.23-verify
.\scripts\dev.cmd v0.24-verify
.\scripts\dev.cmd v0.25-verify
.\scripts\dev.cmd v0.26-verify
.\scripts\dev.cmd v0.27-verify
.\scripts\dev.cmd v0.28-verify
```

v0.16 is the versioning/evidence cleanup gate for Road to v1.0. It keeps the
post-v0.15 plant-state projection as diagnostic-only addendum evidence, but it
does not add plant, HVAC, node, meter, sizing, autosizing, or ExampleFiles
numerical conformance.

v0.17 is the case manifest and output request schema v2 gate. It validates all
tracked case manifests.

v0.18 is the output request injection and official oracle baseline gate. It
stages an official ExampleFiles IDF with manifest-owned output requests, but
does not add ExampleFiles numerical conformance.

v0.19 is the series reader and compare engine v2 gate. It adds timestamp-aware
selected-series parsing and richer comparison metrics, but does not add a new
numerical conformance claim.

v0.20 is the conformance report generator gate. It creates the release
conformance index, coverage matrices, and companion PDF/HTML/JSON/Markdown
artifacts without adding a new numerical conformance claim.

v0.21 is the source-map and algorithm ledger gate. It validates that algorithm
ledger entries have EnergyPlus source files, source-map docs, Rust target
anchors, first-case manifests, proof variables, and claim-appropriate gates.

v0.22 is the declared time/weather/schedule conformance gate. It promotes
`Schedule Value` and dry-bulb hourly series only, using timestamp-aligned
EnergyPlus ESO comparisons.

v0.23 is the static model evidence gate. It promotes official
`1ZoneUncontrolled` static EIO fields for surface geometry,
construction/material summaries, and OtherEquipment nominal inputs only.

v0.24 is the runtime state and output registry hardening gate. It adds
registry-backed output handles, explicit unavailable-output/meter diagnostics,
and ResultStore profiling scaffolds without adding new numerical conformance.

v0.25 is the opaque no-mass heat-balance generalization gate. It adds
adiabatic and interzone boundary handling while keeping conformance limited to
declared existing no-mass variables.

v0.26 is the internal convective gains conformance gate. It promotes only the
declared `Zone Total Internal Convective Heating Rate` hourly series for
`internal_gains_001`.

v0.27 is the user support coverage report gate. It generates oodocs
PDF/HTML/JSON/Markdown coverage for supported inputs, outputs, and algorithm
families without adding a new numerical conformance claim.

v0.28 enriches input object coverage with first evidence and support-boundary
metadata so the support coverage report is readable from a user perspective.

Development-only diagnostic scripts are listed in
`operations/script-index.md`. They are useful during porting, but they do not
create conformance evidence.
