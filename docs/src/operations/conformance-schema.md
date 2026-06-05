# Conformance Schema

Status: P1 foundation in progress.

The conformance schema is the first layer of the reset plan that turns
comparison work into auditable evidence. It is implemented in
`crates/ep_conformance` and validated by
`scripts/conformance-schema-smoke.cmd`.

## Rule

```text
No conformance claim without case + variable list + tolerance + report + gate.
```

`ConformanceCase` enforces that rule before a case can be treated as valid:

- `comparison_class = "conformance"` requires `conformance_claim = true`
- `conformance_claim = true` requires at least one output request
- `conformance_claim = true` requires at least one tolerance rule
- `conformance_claim = true` requires a report contract
- `conformance_claim = true` requires a blocking gate
- smoke and diagnostic-only cases can define output requests without claiming
  numerical compatibility

## Current Fixtures

`data/conformance_cases/schedule_constant_001/case.toml` is the first schema
fixture. It defines an hourly `Schedule Value` output request and points at a
small IDF, but it remains a smoke case:

```toml
comparison_class = "smoke"
conformance_claim = false
```

This is intentional. The case becomes conformance evidence only after baseline
generation, multi-series reporting, tolerance policy, and a blocking release
gate are wired.

`data/conformance_suites/foundation.toml` is the first ordered suite manifest.

## Verification

```powershell
.\scripts\conformance-schema-smoke.cmd
.\scripts\check.cmd
```

## Next Steps

- generate EnergyPlus baseline artifacts from case manifests
- add a multi-series compare report skeleton tied to output requests
- introduce an `Output:Variable` registry so requested variables are tracked
  from IDF/epJSON intake through result comparison
- promote schedule/weather cases from smoke manifests to tolerance-gated
  conformance cases only when their reports and gates exist
