---
status: active
claim_level: baseline-only
owner: conformance
last_reviewed: 2026-06-07
---

# Baseline Generation

EnergyPlus baseline generation proves that the oracle artifacts can be created
from a case manifest. It does not prove Rust conformance by itself.

Current command:

```powershell
cargo run -p ep_cli -- conformance baseline data\conformance_cases\schedule_constant_001\case.toml .runtime\energyplus\26.1.0 .runtime\conformance-baseline\26.1.0
```

Baseline artifacts must include EnergyPlus input, converted epJSON, ERR, the
requested oracle output files, and an expanded manifest:

```text
.runtime/conformance-baseline/<oracle-version>/<case-id>/
  input.idf
  input.epJSON
  eplusout.err
  eplusout.eso
  eplusout.eio
  case-expanded.toml
```

`case-expanded.toml` records the original case identity, staged inputs,
generated oracle artifacts, and every requested output's source target. It is
baseline-only evidence and does not imply Rust numerical conformance.
