---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Tolerance Policy

Each conformance variable needs an explicit tolerance.

Example:

```toml
[[tolerances]]
variable = "Zone Mean Air Temperature"
unit = "C"
abs = 1.0e-3
rel = 1.0e-5
frequency = "Hourly"
reason = "Initial heat-balance scalar comparison tolerance."
```

Tolerance definitions should include:

- variable name
- unit
- absolute tolerance
- relative tolerance
- sample frequency
- aggregation level, when applicable
- reason for the tolerance

Smoke defaults are not a release tolerance policy.

