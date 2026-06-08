---
status: active
claim_level: none
owner: qa
last_reviewed: 2026-06-08
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
.\scripts\dev.cmd conformance-evidence-report -Version 0.32.0
.\scripts\dev.cmd conformance-index-report -Version 0.32.0
.\scripts\dev.cmd support-coverage-report -Version 0.32.0
.\scripts\dev.cmd user-coverage-handbook -Version 0.32.0
.\scripts\dev.cmd release-evidence-manifest -Version 0.32.0
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
.\scripts\dev.cmd v0.32-verify
```

The current official dynamic 1Zone tracker is diagnostic-only. The default
lane compares 41 hourly series, including zone air heat-balance latent terms,
inside/outside zone opaque conduction aggregates, roof exterior source rows,
and the floor surface heat-storage diagnostic, and the probe lanes isolate
mass-CTF seeding, EnergyPlus analytical zone-air updates, surface-first
correction order, same-timestep coupled surface/zone-air rebalance, and
quick outside-conduction boundary solves without creating a conformance claim:

```powershell
.\scripts\dev.cmd official-dynamic-heat-balance-diagnostic
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

The source-map and algorithm-ledger gate remains:

```powershell
.\scripts\dev.cmd algorithm-ledger-check
```

Numerical conformance requires a generated report plus a blocking gate. Smoke
or diagnostic commands can support development, but they cannot support a
compatibility claim.

Frozen release evidence is published as GitHub Release assets. The local
`.runtime/release-evidence` directory is a staging area, not the long-term
evidence store.
