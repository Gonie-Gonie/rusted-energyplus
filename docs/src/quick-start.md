---
status: active
claim_level: setup
owner: docs
last_reviewed: 2026-06-07
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

Development-only diagnostic scripts are listed in
`operations/script-index.md`. They are useful during porting, but they do not
create conformance evidence.
