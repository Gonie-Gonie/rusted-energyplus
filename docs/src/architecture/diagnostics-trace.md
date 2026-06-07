---
status: active
claim_level: diagnostic
owner: runtime
last_reviewed: 2026-06-07
---

# Diagnostics and Trace

Diagnostics and traces are allowed before conformance evidence exists.

They must identify their class:

```text
comparison_class: smoke | diagnostic-only | conformance | regression | performance
conformance_claim: true | false
tolerance_policy: none | case.toml | default-<milestone>
```

Diagnostic traces help locate differences. They do not by themselves support a
compatibility claim.

v0.6 diagnostic artifacts include:

- `compare-zone` MAT `compare-summary.json` and `compare-report.md`
- manifest-driven MAT diagnostic reports under `.runtime/conformance-diagnostic`
- compare regression `trace.json`, `compare-summary.json`,
  `compare-report.md`, and `profile-summary.json`

Zone-temperature diagnostics must keep extraction-only semantics until a future
heat-balance conformance milestone declares tolerances and blocking gates.

## Typed Diagnostic Classes

Unsupported or invalid runtime conditions should be reported with typed
diagnostics rather than panics or silent defaults. Required classes include:

| Class | Use |
|---|---|
| `UnsupportedObject` | object family is outside the implemented subset |
| `UnsupportedTopology` | graph shape is not implemented |
| `MissingReference` | a referenced object cannot be resolved |
| `DuplicateNormalizedName` | normalized names collide |
| `InvalidNumericRange` | parsed value is outside an allowed range |
| `OutputVariableUnavailable` | requested output is not produced |
| `OracleArtifactMissing` | required EnergyPlus artifact is absent |
| `TimestampMismatch` | Rust and oracle timestamps do not align |
| `SampleCountMismatch` | Rust and oracle series lengths differ |
| `ToleranceFailure` | declared tolerance is exceeded |
| `NonFiniteValue` | NaN or infinity appears in evidence data |
| `SolverDidNotConverge` | a numerical solver failed to converge |
