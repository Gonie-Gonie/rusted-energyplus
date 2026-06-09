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
- `v026_dynamic_idf_inventory.py` inventories IDF-backed case manifests through
  v0.26 and flags which ones currently have dynamic conformance-gated evidence
  versus static, baseline, smoke, or diagnostic-only gaps. It also assigns each
  case a target domain, role, source kind, and IDF path so official ExampleFile
  heat-balance work stays separated from HVAC, plant, local fixtures, and
  static intake gaps. It is a planning aid for the v0.26-example
  dynamic-conformance expansion target, not a release claim artifact.
- `dynamic_heat_balance_probe_summary.py` summarizes the official dynamic
  heat-balance diagnostic probe lanes from existing `.runtime` compare
  summaries, including fixed MAT, zone-air heat-balance, floor conduction, and
  inside/outside aggregate conduction focus metrics across lanes. The focus set also includes
  latent diagnostic state such as internal convective gain, floor/roof
  inside/outside face temperatures, and roof outside convection/radiation/solar
  source rows, including incident solar beam/sky/ground decomposition, so rate
  regressions can be reviewed against their surface-state drivers. The focus
  table also records RMSE movement
  relative to the default lane so partial isolation improvements that
  destabilize the whole zone balance are visible in one scan. The probe
  interpretation table compares each lane against its nearest reference lane,
  so all-CTF warmup and surface-iteration probes can be reviewed separately
  from the larger default-to-all-CTF seed movement. The best-focus table picks
  the lowest-RMSE lane per tracked metric so structural optimizations can be
  aimed at the probe that actually improves that state. The surface conduction
  driver tables split each lane's inside-face and outside-face conduction
  deltas by wall, floor, and roof surface, and record the best lane for each
  surface so zone aggregate improvements or regressions can be traced back to
  source rows. The lane table
  marks stale artifacts when older `.runtime` summaries still have the previous
  series count, so missing focus rows are not mistaken for runtime support
  gaps. Probe lanes include
  all-CTF seeding, all-CTF plus oracle-day-count warmup, all-CTF plus a
  surface-iteration probe, EnergyPlus analytical, analytical surface-first,
  combined all-CTF analytical surface-first, combined all-CTF analytical
  coupled surface rebalance, combined all-CTF analytical coupled surface
  rebalance with three surface passes, combined all-CTF analytical coupled
  previous-inside outdoor boundary solves with three surface passes, combined
  all-CTF analytical coupled previous-inside quick outside-conduction solves
  with three, five, and eight surface passes, eight- and twenty-pass
  interleaved surface/zone-air correction forks, a twenty-pass interleaved
  grey interior-longwave fork, plus an EnergyPlus initial
  CTF-history seed fork, DOE-2 exterior-convection, grey interior-longwave, ScriptF
  interior-longwave, and combined DOE-2/longwave forks,
  combined
  all-CTF analytical coupled previous-inside outdoor/adiabatic boundary solves
  with three surface passes, a third-order balance-surface-convection
  frozen-reference-air cadence probe, a third-order balance-surface-convection
  frozen-reference-air current-longwave cadence probe, a third-order
  frozen-reference-air current-longwave convergence-cutoff probe, a third-order
  converged-surface adiabatic history-only commit rejection probe, a third-order
  balance-surface-convection current-adiabatic-history rejection probe, combined
  all-CTF analytical surface-first with three surface passes, third-order
  zone-air updates, and an
  oracle-day-count warmup minimum. It is
  development evidence only and does not create a release conformance artifact.
  `scripts\dev.cmd official-dynamic-heat-balance-probe-summary` invokes this
  generator through the repo-local report Python environment. Use
  `scripts\dev.cmd official-dynamic-heat-balance-probe-suite` to refresh every
  tracked probe lane before regenerating the summary. The summary also pairs
  per-surface heat-storage RMSE with inside- and outside-face conduction and
  temperature RMSE, then reports storage/conduction RMSE per face-temperature
  RMSE. This keeps CTF amplification cases, where small temperature misses
  become large storage-rate deltas, visible before a runtime candidate is
  promoted. It also prints signed `ZN001:FLR001` CTF current/history
  first-sample deltas and annual current/history RMSE so cancellation can be
  reviewed directly when a probe improves reported storage but leaves latent
  mass-floor history mismatch behind.

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
