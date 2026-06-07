---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Versioning Reset v2

This document is the canonical 2026-06-07 reset for release numbering after the
pre-alpha evidence series. The long-term target remains a Rust-only
EnergyPlus-compatible port locked to the EnergyPlus 26.1.0 oracle, but release
numbers must no longer look like broad compatibility maturity when the actual
artifact is smoke, baseline, source-map, or diagnostic evidence.

Core rule:

```text
No compatibility claim without case + variable list + tolerance + report + gate.
```

## Reset Decision

- v0.1 through v0.15 are the Historical Pre-Alpha Evidence Series.
- The plant-state projection work already added after v0.15 is kept as an
  additional diagnostic artifact. It is not the defining purpose of v0.16.
- v0.16 starts the Road to v1.0 and is a Versioning and Evidence Cleanup
  milestone.
- v0.17 through v0.24 build the conformance infrastructure needed before broad
  numerical expansion.
- v0.25 through v0.59 expand algorithms, ExampleFiles coverage, performance
  evidence, stability evidence, and the v1.0 scope lock.
- v0.80 through v0.99 are v1.0 release-candidate and freeze milestones.
- v1.0 is a substantial compatibility draft for a declared subset, not a full
  EnergyPlus replacement.
- v2.0 is the EnergyPlus 26.1 full compatibility target.
- v3.0 is the fast modernized successor target with compatibility mode retained.

## Why This Reset Exists

The earlier v0.10 through v0.15 labels are accurate as evidence artifacts but
easy to misread as implementation completion:

| Historical label | Easy misread | Actual boundary |
|---|---|---|
| v0.10 IdealLoads | IdealLoads simulation is complete | baseline-only typed graph and output availability |
| v0.11 Air-side node | node numerical parity exists | diagnostic-only baseline/projection evidence |
| v0.13 Plant loop | plant loop simulation exists | typed graph smoke only |
| v0.15 Plant diagnostic | plant output parity exists | EnergyPlus oracle baseline-only plant rows |
| plant-state projection addendum | plant algorithm parity exists | Rust diagnostic artifact plumbing with `algorithm_parity: false` |

The reset keeps the artifacts and fixes the interpretation.

## Road to v1.0

Every new pre-v1 milestone must state three things:

| Axis | Required evidence |
|---|---|
| Infrastructure | manifest, output request, baseline, compare, report, and gate changes |
| Algorithm port | EnergyPlus source map, Rust target module, evidence level, and gaps |
| ExampleFiles coverage | official or reduced IDF, weather, variables/meters, tolerance, and report path |

The planned sequence is:

| Range | Purpose |
|---|---|
| v0.16-v0.24 | versioning cleanup, manifest v2, output injection, oracle baseline pipeline, compare v2, report generator, source-map ledger |
| v0.25-v0.33 | time/weather/schedule expansion, static model expansion, heat balance, massive constructions, internal gains, fenestration, solar, warmup, uncontrolled-zone pack, IdealLoads |
| v0.34-v0.39 | node semantics, fans, coils, zone equipment, constant-volume air loops, air-side v1 candidate report |
| v0.40-v0.47 | plant graph, pump flow, purchased energy, boilers, chillers, condenser loops, operation schemes, meters |
| v0.48-v0.59 | sizing boundary, diagnostic stability, v1.0 scope lock, stabilization, performance, platform, and documentation packs |
| v0.80-v0.99 | v1.0 RCs, evidence regeneration, release freeze |

## v1.0 Meaning

v1.0 is a substantial compatibility draft. It must include multiple
ExampleFiles/testfiles and a meaningful system subset, but it is still limited
to declared cases, variables, meters, tolerances, and known gaps.

Minimum v1.0 evidence targets:

- Tier A conformance cases: approximately 15 to 25.
- Tier B diagnostic/baseline cases: approximately 30 to 60.
- object, variable, and case coverage matrices.
- release-level conformance index.
- performance report.
- stability and failure-mode report.
- known gaps and excluded features.

v1.0 must not claim full EnergyPlus compatibility, all IDF compatibility, all
HVAC compatibility, all plant compatibility, or autosizing compatibility unless
those features are explicitly listed with evidence.

## Long-Term Targets

v2.0 is the EnergyPlus 26.1 full compatibility target. Compatibility mode
results must match the EnergyPlus 26.1.0 oracle for the declared broad/full
suite. Internal restructuring is allowed only when the compatibility contract is
preserved or the difference is explicitly documented.

v3.0 is the fast modernized successor target. It keeps compatibility mode but
adds fast or modern modes where EnergyPlus-exact numerical parity may be
relaxed only with engineering validation, deviation reports, and mode-specific
claims.

## Immediate Work Items

- Keep this document linked from the mdBook and README.
- Maintain `legacy-milestones.md` as the interpretation table for v0.1-v0.15
  and diagnostic addenda.
- Treat v0.16 as versioning/evidence cleanup.
- Add v0.17 manifest v2 and output request schema work.
- Add v0.18 output injection and oracle baseline pipeline work.
- Keep plant-state projection artifacts in release verification as additional
  diagnostic plumbing, not as the v0.16 milestone purpose.
