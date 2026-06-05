---
status: active
claim_level: baseline-only
owner: conformance
last_reviewed: 2026-06-05
---

# Baseline Generation

EnergyPlus baseline generation proves that the oracle artifacts can be created
from a case manifest. It does not prove Rust conformance by itself.

Current command:

```powershell
cargo run -p ep_cli -- conformance baseline data\conformance_cases\schedule_constant_001\case.toml .runtime\energyplus\26.1.0 .runtime\conformance-baseline\26.1.0
```

Baseline artifacts should include EnergyPlus input, converted epJSON, ESO, ERR,
and other requested oracle files.

