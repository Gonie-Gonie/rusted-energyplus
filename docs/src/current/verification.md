---
status: active
claim_level: none
owner: qa
last_reviewed: 2026-06-20
---

# Verification

The standard local gate is:

```powershell
.\scripts\dev.cmd check
```

Documentation and spec references are maintained with:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
```

Case manifest schema v2 is checked with:

```powershell
.\scripts\dev.cmd manifest-validate-all
```

The v0.26 IDF-backed dynamic evidence inventory is:

```powershell
.\scripts\dev.cmd v0.26-dynamic-idf-inventory
```

The false-claim guard is:

```powershell
.\scripts\dev.cmd strict-no-false-conformance
```

Release evidence documents use the repo-local Python environment and oodocs:

```powershell
.\scripts\dev.cmd conformance-evidence-report -Version 0.1.0
.\scripts\dev.cmd conformance-index-report -Version 0.1.0
.\scripts\dev.cmd support-coverage-report -Version 0.1.0
.\scripts\dev.cmd user-coverage-handbook -Version 0.1.0
.\scripts\dev.cmd release-evidence-manifest -Version 0.1.0
```

The current declared conformance gates are:

```powershell
.\scripts\dev.cmd compare-heat-balance-conformance
.\scripts\dev.cmd compare-surface-temperature-conformance
.\scripts\dev.cmd compare-schedule-conformance
.\scripts\dev.cmd compare-weather-conformance
.\scripts\dev.cmd compare-static-model-conformance
.\scripts\dev.cmd compare-internal-convective-gain-conformance
```

Current supporting release and infrastructure gates include:

```powershell
.\scripts\dev.cmd runtime-registry-smoke
.\scripts\dev.cmd heat-balance-generalization-smoke
.\scripts\dev.cmd v0.1-verify
```

The broad official dynamic 1Zone tracker remains diagnostic-only. The default
lane compares 99 hourly series, including zone air heat-balance latent terms,
inside/outside zone opaque conduction aggregates, wall/floor conduction
decomposition rows, wall/roof exterior source rows, wall/roof exterior incident
solar decomposition rows, and the floor surface heat-storage diagnostics in
whole-surface and per-area form, and the probe lanes isolate
mass-CTF seeding, EnergyPlus analytical zone-air updates, surface-first
correction order, same-timestep coupled surface/zone-air rebalance, and
quick outside-conduction boundary solves without broad compatibility claims.
The separate compatibility-candidate command pins the narrower promoted
variable set, all-EIO CTF seed, EnergyPlus initial CTF histories, 20-day
minimum warmup, and 20 surface iterations. That candidate now carries
`conformance_claim = true` with a blocking gate for the promoted weather,
zone-air, surface-temperature, surface-conduction, and declared floor
surface-heat-storage and declared wall/roof exterior convection/net-radiation outputs while leaving broad decomposition rows
diagnostic-only.
Heat-balance diagnostic and conformance report writers also emit a compact
`compare-digest.json` next to the full `compare-summary.json` and
`compare-report.md`; active gates read the digest for metadata, bottlenecks, and
series-level deltas, including first/max delta samples on each bottleneck row,
plus first reported sample bottlenecks for run-period handoff diagnosis, while
also carrying Rust-only first-sample CTF component rows for inside/outside
current-temperature and history-term isolation plus oracle-inferred first-sample
CTF history deltas when EIO zero coefficients and matching oracle series are
available. The full summary preserves hourly sample rows for deeper inspection.
The diagnostic artifacts now also include `rust-zone-air-diagnostics.json`,
which captures the Rust run-period initial zone-air state, a first-reported-hour
`zone_air_first_sample_trace`, `warmup_day_end_states`, and hourly Rust-only
zone-air current/average/history, humidity, air-capacity, and system-timestep
count series. The markdown report mirrors the first-sample and warmup day-end
zone-air traces so the first four 15-minute substeps and the repeated-day
warmup fixed point can be inspected without opening the full JSON:

As of the 2026-06-22 humidity, adjacent-air, and outside-longwave coefficient promotion, the promoted
`official_1zone_uncontrolled_dynamic_conformance_candidate_001` gate passes with
`status = pass`, `conformance_claim = true`, 126 declared conformance series, and
20 EnergyPlus/Rust warmup days. The previously blocking `ZONE ONE` `Zone Air
Heat Balance Air Energy Storage Rate` max absolute delta dropped from
`0.533385790012 W` to `0.182078359183 W` against the `0.5 W` zone-state
max-absolute tolerance; RMSE is `0.023340057930 W` against the `0.1 W`
zone-state RMSE tolerance. The promoted set now also includes `Zone Mean Air Humidity Ratio` (max absolute delta `0.000000094494`), all six `Surface Inside Face Adjacent Air Temperature` rows (worst max absolute delta `0.000648652782 C`), and the roof outside longwave air/sky/ground split coefficient rows (sky coefficient max absolute delta `0.000023095019 W/m2-K`; air and ground split coefficients exact-zero). Weather rows are exact for `Site Sky Temperature`, `Site Horizontal Infrared Radiation Rate per Area`, roof wind speed, and roof wind direction; roof surface-local dry-bulb max absolute delta is `0.000000019076 C`, and roof surface-local wet-bulb max absolute delta is `0.000009907934 C` against the `0.00001` weather tolerance. The promoted exterior source rows are the named wall/roof outside convection and net thermal radiation heat-gain rates plus the roof per-area versions; the broad diagnostic still keeps surface iteration-count parity and the remaining floor CTF history handoff outside the claim. This supports only the declared limited claim above.
The 2026-06-21 current broad-diagnostic rerun confirms that this is not yet a
complete conformance claim: the compatibility-candidate diagnostic still has
`status = fail`, `conformance_claim = false`, 119 diagnostic series, and
`active_blocker_summary = ZN001:FLR001/FLOOR sample=1156
dominant=outside-history-total dominant_delta_w=9.527199725017
storage_delta_w=0.663522464624`. Raising the same lane from 20 to 50 surface
iterations produced identical floor storage/history deltas, so this is not an
inside-surface iteration-count limit. Two adiabatic handoff probes were also
rejected as fixes: the ScriptF-flat adiabatic-report probe regressed floor
storage to `2195.200776475414 W`, and the adiabatic-history-commit probe
regressed floor storage to `1454.593723095763 W`. The remaining blocker is a
small surface/zone fixed-point temperature offset that is amplified by the mass
floor CTF history coefficients, not a simple adiabatic outside-face sync switch.
It still cannot support a broad or "complete" EnergyPlus conformance statement
because the broad diagnostic boundary remains diagnostic-only and still shows:

- The roof outdoor-air reference path is now promoted in the candidate boundary: `ZN001:ROOF001` surface
  outdoor dry-bulb max absolute delta is `0.000000019076 C`, wet-bulb is
  `0.000009907934 C`, and wind speed/direction remain exact against the oracle.
- `ZN001:ROOF001` outside face temperature max absolute delta fell from
  `0.019893003133 C` to `0.000944839218 C`.
- The newly promoted exterior source rows now fit dedicated exterior tolerances: named wall/roof outside convection/net-radiation total-rate rows are under `2.5 W` max absolute and `0.6 W` RMSE, and roof exterior per-area rows are under `0.011 W/m2` max absolute and `0.002 W/m2` RMSE. the promoted exterior set tops out at `1.893813807998 W` max absolute delta and `0.579970374604 W` RMSE, so it is claimable only under the dedicated exterior-rate tolerance, not as a broad exact-parity statement.
- `ZN001:ROOF001` outside convection heat-gain max absolute delta fell from
  `25.397397824985 W` to `1.902520148418 W`; outside net thermal radiation
  heat-gain fell from `25.044153522467 W` to `1.242262187458 W`.
- `ZN001:ROOF001` solar radiation heat-gain remains a separate residual at
  `2.429489654271 W` max absolute delta.
- `ZONE ONE` mean air temperature max absolute delta is now
  `0.000798369301 C`, and `ZN001:FLR001` floor storage max absolute delta is
  `0.663522464624 W`.
- The active report-level target is `outside-ctf-history-handoff`: the floor
  CTF diagnostic now reports dominant source `outside-history-total` at sample
  `1156`, while roof exterior splits are much smaller secondary residuals.
- First-run-period-substep evidence rules out hourly averaging as the source of
  the current offset. In the compatibility-candidate lane, Rust records
  `ZONE ONE` MAT `-0.620477202798 C` and a stored third-order solution
  `-0.620476954864 C` for timestep 1, while a timestep-frequency EnergyPlus
  oracle probe reports `-0.627042747070 C`. The corresponding floor inside-face
  temperatures are `-0.065350444313 C` (Rust) and `-0.071842487381 C`
  (EnergyPlus).
- Warmup day-end evidence now shows that the delta is already present at the
  end of the repeated run-period warmup, before the first reported timestep.
  EnergyPlus `Warmup {20} RUN PERIOD 1` ends at `ZONE ONE` MAT
  `-0.606928229693 C`; Rust day 20 ends and run-period initial state both
  record `-0.600360486247 C`, a `+0.006567743446 C` Rust-minus-oracle offset.
  The previous MAT history slots show the same offset (`+0.0065626`,
  `+0.0065561`, `+0.0065467 C`). The remaining blocker is therefore the
  repeated-day surface/zone source model fixed point, not the handoff copy
  itself, hourly reporting, warmup day count, or first-hour weather seed.
- `ZONE ONE` mean air humidity ratio was added as a diagnostic-only row. In
  the regenerated candidate baseline it matches exactly through the first
  run-period day and differs by about `1.2e-6 kgWater/kgDryAir` at representative
  later samples (`4213`, `6567`, `8426`, `8759`), so the remaining
  `~0.0065 C` mean-air-temperature offset is not currently explained by a large
  humidity or air-capacity mismatch.
- The Rust zone-air diagnostics show the compatibility-candidate lane staying
  at a zone/system timestep count of `1` at the representative blocker samples,
  which narrows the next implementation target away from adaptive system
  subdivision and toward run-period zone-temperature history/handoff or the
  floor CTF current/history split.

```powershell
.\scripts\dev.cmd official-dynamic-heat-balance-diagnostic
.\scripts\dev.cmd official-dynamic-heat-balance-compat-candidate
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-warmup-20-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-surface-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-analytical-probe
.\scripts\dev.cmd official-dynamic-heat-balance-analytical-surface-first-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-surface-first-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-doe2-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter5-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter8-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter8-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter20-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-iter20-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-epseed-iter5-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter5-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interior-longwave-iter5-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-iter5-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-scriptf-lw-iter5-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-lw-iter5-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-boundary-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-all-ctf-analytical-surface-first-iter3-probe
.\scripts\dev.cmd official-dynamic-heat-balance-third-order-probe
.\scripts\dev.cmd official-dynamic-heat-balance-warmup-20-probe
.\scripts\dev.cmd official-dynamic-heat-balance-probe-summary
.\scripts\dev.cmd official-dynamic-heat-balance-probe-suite
```

For convergence isolation, the base diagnostic script also accepts
`-SurfaceIterations` up to `200`; wrapper probes keep their smaller named counts
so regular local gates remain bounded.

The source-map and algorithm-ledger gate remains:

```powershell
.\scripts\dev.cmd algorithm-ledger-check
```

Numerical conformance requires a generated report plus a blocking gate. Smoke
or diagnostic commands can support development, but they cannot support a
compatibility claim.

Frozen release evidence is generated under `.runtime/release-evidence`. The
GitHub Release page publishes the binary package plus curated public evidence
PDFs; local HTML, JSON, and Markdown files remain staging artifacts for
inspection and debugging.
