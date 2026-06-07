---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-07
---

# Setup

Prepare the local workspace with:

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
```

The setup flow keeps external tools inside repository-local directories:

- `.runtime/energyplus/26.1.0`
- `.reference/energyplus-src/26.1.0`
- `.runtime/python/3.11.9`
- `.runtime/python-venvs/report`

The project does not rely on a globally installed EnergyPlus oracle or ambient
Python reporting packages.

After setup, run:

```powershell
.\scripts\dev.cmd check
```
