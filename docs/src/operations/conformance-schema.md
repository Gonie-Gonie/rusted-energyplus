# Conformance Schema

Status: P1 foundation in progress.

The conformance schema is the first layer of the reset plan that turns
comparison work into auditable evidence. It is implemented in
`crates/ep_conformance` and validated by
`scripts/dev.cmd conformance-schema-smoke`.

Manifest-driven EnergyPlus baseline generation is exposed through:

```powershell
cargo run -p ep_cli -- conformance baseline <case.toml> <oracle-root> <output-root>
```

Baseline-only report skeleton generation is exposed through:

```powershell
cargo run -p ep_cli -- conformance report-skeleton <case.toml> <baseline-case-dir> <report-root>
```

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

`OutputRegistry` turns case output requests into stable series identities. It
normalizes output key, variable name, and frequency for duplicate detection, so
one report row cannot silently represent the same EnergyPlus series twice.

## Current Fixtures

`data/conformance_cases/schedule_constant_001/case.toml` is the first schema
fixture. It defines an hourly `Schedule Value` output request and points at a
small IDF, but it remains a smoke case:

```toml
comparison_class = "smoke"
conformance_claim = false
```

This is intentional. The case becomes conformance evidence only after tolerance
policy and a blocking release gate are wired.

`data/conformance_suites/foundation.toml` is the first ordered suite manifest.

## Baseline Artifacts

The first baseline smoke command is:

```powershell
.\scripts\dev.cmd conformance-baseline-smoke
```

It stages the case IDF and writes EnergyPlus artifacts under:

```text
.runtime/conformance-baseline/26.1.0/schedule_constant_001/
```

Expected artifacts include `input.idf`, `input.epJSON`, `eplusout.eso`, and
`eplusout.err`. These files prove oracle artifact generation only; they are not
yet a tolerance-gated Rust/EnergyPlus comparison report.

## Report Skeleton

The first report smoke command is:

```powershell
.\scripts\dev.cmd conformance-report-smoke
```

It writes:

```text
.runtime/conformance-report/26.1.0/schedule_constant_001/compare-report.md
```

The report is intentionally marked:

```text
tolerance_policy: none
status: baseline-only
```

It enumerates the requested output series and EnergyPlus baseline sample count.
It reads requested series through `OutputRegistry` and does not compare Rust
results yet.

## Verification

```powershell
.\scripts\dev.cmd conformance-schema-smoke
.\scripts\dev.cmd conformance-baseline-smoke
.\scripts\dev.cmd conformance-report-smoke
.\scripts\dev.cmd check
```

## Next Steps

- extend `OutputRegistry` across IDF/epJSON intake and native result comparison
- promote schedule/weather cases from smoke manifests to tolerance-gated
  conformance cases only when their reports and gates exist
