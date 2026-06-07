---
status: active
claim_level: reporting-infrastructure
owner: conformance
last_reviewed: 2026-06-08
---

# Support Coverage Report

The support coverage report is the user-facing answer to three questions:

- which EnergyPlus input objects are currently parsed or typed
- which output variables are currently requested, diagnostic, or conformance-gated
- which algorithm families have limited conformance evidence versus diagnostic-only scaffolding

Generate it through the standard release wrapper:

```powershell
.\scripts\dev.cmd support-coverage-report -Version 0.29.0
```

Artifacts are written to:

```text
.runtime/release-evidence/v0.29.0/support-coverage.md
.runtime/release-evidence/v0.29.0/support-coverage-report.html
.runtime/release-evidence/v0.29.0/support-coverage-report.pdf
.runtime/release-evidence/v0.29.0/support-coverage-report.json
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
| `specs/algorithm_ledger.toml` | algorithm status, source maps, proof variables |
| `data/conformance_cases/*/case.toml` | case tiers, domains, requested outputs, gates, and claim boundaries |

## Status Meaning

| Status | User meaning |
|---|---|
| typed input support | the input object is parsed into the Rust model |
| typed graph only | the input object is represented structurally, but no numerical algorithm parity is claimed |
| tolerance-gated output | the output is promoted in a manifest, tolerance, report, and blocking gate |
| diagnostic output | the output is compared or emitted for development visibility only |
| limited algorithm conformance | the algorithm has proof variables and promoted conformance cases |
| diagnostic projection only | the algorithm family has scaffolding or baseline evidence, not user numerical conformance |

## Boundary

The report is a coverage map. It does not create new conformance claims. A
supported row is only as strong as its status, first case, tolerance, and gate.

As of v0.28.0, every tracked input object includes a first evidence reference
and a support boundary in `specs/object_coverage.toml`.

As of v0.29.0, output variable first evidence is resolved from the strongest available evidence level for that variable. For example, a variable that
appears first in a diagnostic fixture but is later promoted in a conformance
fixture reports the promoted conformance case as its first evidence.

The report explicitly does not claim full EnergyPlus
compatibility, broad ExampleFiles numerical compatibility, HVAC numerical
conformance, plant numerical conformance, meter conformance, sizing, EMS,
PythonPlugin, daylighting, fenestration, solar, or warmup compatibility.
