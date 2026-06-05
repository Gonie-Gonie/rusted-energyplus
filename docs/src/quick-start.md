---
status: active
claim_level: setup
owner: docs
last_reviewed: 2026-06-05
---

# Quick Start

Prepare the local toolchain, EnergyPlus oracle, and docs tooling:

```powershell
.\scripts\setup.cmd -InstallRust -InstallDocsTools
```

Run the local check suite:

```powershell
.\scripts\check.cmd
```

Verify the current public foundation/model-intake release gate:

```powershell
.\scripts\v0.1-verify.cmd
```

Development-only diagnostic scripts are listed in
`operations/script-index.md`. They are useful during porting, but they do not
create conformance evidence.

