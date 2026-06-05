---
status: active
claim_level: setup
owner: core
last_reviewed: 2026-06-05
---

# Setup Checklist

Required setup evidence:

- pinned Rust toolchain is installed
- Cargo workspace builds
- EnergyPlus 26.1.0 oracle exists under `.runtime`
- EnergyPlus 26.1.0 reference source exists under `.reference`
- source smoke passes
- oracle smoke passes
- docs build passes

Primary command:

```powershell
.\scripts\setup.cmd -InstallRust -InstallDocsTools
```

