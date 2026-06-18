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
.\scripts\dev.cmd v0.32-verify
```

v0.32 adds the user coverage handbook. It reorganizes the support coverage
matrix into user decision rules for currently supported inputs, outputs,
algorithms, promoted cases, and known gaps.

Development-only diagnostic scripts are listed in
`operations/script-index.md`. They are useful during porting, but they do not
create conformance evidence.
