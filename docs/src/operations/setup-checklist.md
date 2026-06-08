---
status: active
claim_level: setup
owner: core
last_reviewed: 2026-06-08
---

# Setup Checklist

Required setup evidence:

- pinned Rust toolchain is installed
- Cargo workspace builds
- EnergyPlus 26.1.0 oracle exists under `.runtime`
- EnergyPlus 26.1.0 reference source exists under `.reference`
- portable Python 3.11.9 exists under `.runtime/python`
- report Python venv exists under `.runtime/python-venvs/report`
- source smoke passes
- oracle smoke passes
- python smoke passes
- docs build passes

Primary command:

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
```
