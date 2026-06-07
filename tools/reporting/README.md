# Reporting Generators

Scripted reports use the repo-local Python reporting environment.

PowerShell files under `scripts/` are entry points and orchestration wrappers.
They should locate the repository, provision or select the report virtual
environment through `scripts/lib/python.ps1`, and invoke Python.

Python files in this directory own document layout, charting, data aggregation,
and serialization. Use `oodocs` for structured HTML/PDF output and matplotlib
figure objects for charts. Emit JSON for durable evidence data when the report
supports release or conformance claims.

Pinned dependencies live in `tools/python/requirements-report.txt`.
