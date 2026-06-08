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
- `dynamic_heat_balance_probe_summary.py` summarizes the official dynamic
  heat-balance diagnostic probe lanes from existing `.runtime` compare
  summaries, including fixed MAT, zone-air heat-balance, floor conduction, and
  aggregate conduction focus metrics across lanes. The focus table also records
  RMSE movement relative to the default lane so partial isolation improvements
  that destabilize the whole zone balance are visible in one scan. Probe lanes
  include all-CTF seeding, all-CTF plus oracle-day-count warmup, third-order
  zone-air update, and an oracle-day-count warmup minimum. It is development
  evidence only and does not create a release conformance artifact.

Conformance-facing scripts should keep this split:

- `scripts/dev.ps1` exposes stable user commands and groups them by purpose.
- `scripts/release/*.ps1` wrappers locate/provision the report Python and pass
  arguments through to the generator.
- `tools/reporting/*.py` owns data shaping, chart readability, document layout,
  and durable artifact serialization.

When a Python report includes charts, review the rendered PDF/HTML output as
part of the change. Temporary screenshots or page images are useful for QA, but
the release artifact set should stay limited to promoted evidence files unless
a new artifact is intentionally added to the release contract.

Pinned dependencies live in `tools/python/requirements-report.txt`.
