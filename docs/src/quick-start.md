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
.\scripts\dev.cmd v0.1-verify
```

v0.1.0 is the public limited-conformance release gate. It packages the CLI,
launcher, oracle runtime, declared conformance evidence, support coverage, user
coverage handbook, and release evidence manifest.

Development-only diagnostic scripts are listed in
`operations/script-index.md`. They are useful during porting, but they do not
create conformance evidence.
