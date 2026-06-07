# Reporting Generators

Scripted reports use the repo-local Python reporting environment.

PowerShell files under `scripts/` are entry points and orchestration wrappers.
They should locate the repository, provision or select the report virtual
environment through `scripts/lib/python.ps1`, and invoke Python.

Python files in this directory own document layout, charting, data aggregation,
and serialization. Use `oodocs` for structured HTML/PDF output and matplotlib
figure objects for charts. Emit JSON for durable evidence data when the report
supports release or conformance claims.

Current generators:

- `conformance_evidence_report.py` builds numerical conformance evidence for
  promoted tolerance-gated cases.
- `conformance_index_report.py` builds the release conformance index and
  coverage matrices for all tracked manifests.
- `support_coverage_report.py` builds the user-facing input, output, and
  algorithm support coverage report from specs plus case manifests.
- `user_coverage_handbook.py` builds the user-facing support scope handbook
  from support coverage and conformance index JSON.
- `release_evidence_manifest.py` builds the release asset manifest from the
  binary package plus generated evidence JSON/PDF/HTML/Markdown artifacts.

Pinned dependencies live in `tools/python/requirements-report.txt`.
