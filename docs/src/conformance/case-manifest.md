---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
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

[gate]
blocking = true
```

Diagnostic-only cases must use:

```toml
comparison_class = "diagnostic-only"
conformance_claim = false
```
