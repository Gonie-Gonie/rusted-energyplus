---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Output Requests

Output requests are part of the evidence contract. A report cannot silently
compare whichever variables happen to be present.

Example:

```toml
[[outputs]]
key = "*"
variable = "Zone Mean Air Temperature"
frequency = "Hourly"
class = "zone-state"

[[outputs]]
key = "*"
variable = "Site Outdoor Air Drybulb Temperature"
frequency = "Hourly"
class = "weather"
```

Each output request must define key, variable, frequency, and variable class.

