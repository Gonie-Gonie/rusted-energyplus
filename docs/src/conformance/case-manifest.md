---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Case Manifest

Every conformance case needs a manifest. Diagnostic-only cases also need to say
that they do not make a claim.

Example:

```toml
id = "001_schedule_constant"
oracle_version = "26.1.0"
comparison_class = "conformance"
conformance_claim = true

[input]
idf = "input.idf"
weather = "weather.epw"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"

[gate]
blocking = true
```

Diagnostic-only cases must use:

```toml
comparison_class = "diagnostic-only"
conformance_claim = false
```
