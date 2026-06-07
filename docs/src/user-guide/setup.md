---
status: active
claim_level: setup
owner: docs
last_reviewed: 2026-06-07
---

# Setup

Use the setup script to install the pinned Rust toolchain and docs tools, then
prepare repo-local EnergyPlus and report-generation assets:

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
```

The setup script keeps the external runtime surface inside the repository:

```text
.runtime/energyplus/26.1.0
.reference/energyplus-src/26.1.0
.runtime/python/3.11.9
.runtime/python-venvs/report
```

The report virtual environment is pinned by
`tools/python/requirements-report.txt` and is used by the numerical conformance
PDF/HTML/JSON evidence generator. Verify it directly with:

```powershell
.\scripts\dev.cmd python-smoke
```

For the release-oriented checklist, see `operations/setup-checklist.md`.
