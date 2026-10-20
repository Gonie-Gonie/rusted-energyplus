---
status: active
claim_level: reporting-infrastructure
owner: conformance
last_reviewed: 2026-10-20
---

# Support Coverage Report

The support coverage report is the user-facing answer to three questions:

- which corpus objects have typed models, raw getters, partial projections or missing implementations
- which positive families are admitted to the bounded ordinary Source A/B route
- which output variables are currently requested, diagnostic, or conformance-gated
- which algorithm families have limited conformance evidence versus diagnostic-only scaffolding

Generate it through the standard release wrapper:

```powershell
.\scripts\dev.cmd support-coverage-report -Version 0.1.0
```

Artifacts are written to:

```text
.runtime/release-evidence/v0.1.0/support-coverage.md
.runtime/release-evidence/v0.1.0/support-coverage-report.html
.runtime/release-evidence/v0.1.0/support-coverage-report.pdf
.runtime/release-evidence/v0.1.0/support-coverage-report.json
```

The PDF and HTML are generated with `oodocs` and matplotlib from repository
sources instead of from hand-maintained tables. The JSON is the durable
machine-readable release evidence.

## Source Data

The generator reads:

| Source | Purpose |
|---|---|
| `specs/object_coverage.toml` | input object support, first evidence, and support boundary |
| `specs/variable_coverage.toml` | named output variable support, first evidence, and support boundary |
| `specs/algorithm_ledger.toml` | algorithm status, source maps, proof variables, first evidence, and support boundary |
| `data/conformance_cases/*/case.toml` | case tiers, domains, requested outputs, gates, and claim boundaries |

## Status Meaning

| Status | User meaning |
|---|---|
| reviewed input projection | historical registry `typed`; inspect the separate compiler column for actual whole-object typed coverage |
| raw Source getter | real raw-input getter, without a whole-object typed compiler model |
| partial input projection | selected fields/references retained; whole-object compiler coverage remains raw-only |
| no connected implementation | no connected positive object implementation; raw retention is not support |
| typed graph only | the input object is represented structurally, but no numerical algorithm parity is claimed |
| tolerance-gated output | the output is promoted in a manifest, tolerance, report, and blocking gate |
| diagnostic output | the output is compared or emitted for development visibility only |
| limited algorithm conformance | the algorithm has proof variables and promoted conformance cases |
| diagnostic projection only | the algorithm family has scaffolding or baseline evidence, not user numerical conformance |

## Boundary

The report is a coverage map. It does not create new conformance claims. A
supported row is only as strong as its status, first case, tolerance, and gate.

The [public corpus table](../generated/example-object-union.md) classifies every canonical object type found in current repository input files. The [full object registry](../generated/object-coverage.md) also retains reviewed types outside that corpus. Every registry entry has an input-handling status, actual compiler coverage and an explicit Source admission classification; types outside the corpus can remain unreviewed for Source.

`bounded_a_b` and `bounded_b` admit only selected positive inputs on the declared one-Zone opaque/direct-IdealLoads route. `inactive_only` does not permit active design-day execution. `blocked` remains blocked even when compiler graphs, partial projections or separate fixed-case/legacy conformance exist. Code references establish implementation/admission facts, not additional Oracle evidence. No row means full-object or arbitrary-combination support, and all existing conformance identities and tolerances remain unchanged.

As of v0.29.0, output variable first evidence is resolved from the strongest available evidence level for that variable. For example, a variable that
appears first in a diagnostic fixture but is later promoted in a conformance
fixture reports the promoted conformance case as its first evidence.

As of v0.30.0, each tracked algorithm entry includes first evidence and a
support boundary, so limited-conformance algorithms and diagnostic projections
are visible without reading the developer source maps first.

As of v0.32.0, the user coverage handbook reads this generated JSON and
reorganizes it into user decision rules for supported inputs, outputs,
algorithms, promoted cases, and known gaps.

The report explicitly does not claim full EnergyPlus
compatibility, broad ExampleFiles numerical compatibility, HVAC numerical
conformance, plant numerical conformance, meter conformance, sizing, EMS,
PythonPlugin, daylighting, fenestration, solar, or warmup compatibility.
