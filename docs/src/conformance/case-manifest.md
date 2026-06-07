---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Case Manifest

Every conformance case needs a manifest. Diagnostic-only cases also need to say
that they do not make a claim.

v0.17 adds Case Manifest v2 validation. The v2 schema makes case tier, source
kind, output requests, tolerance policy, waivers, and release/CI gate policy
explicit before more ExampleFiles are promoted.

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
script = "scripts/dev.cmd compare-schedule-smoke"
blocking = true
```

Diagnostic-only cases must use:

```toml
comparison_class = "diagnostic-only"
conformance_claim = false
```

## Manifest v2 Fields

The v2 contract is:

```toml
[case]
id = "tf_1zone_uncontrolled_001"
source_kind = "energyplus-testfile"
source_file = "1ZoneUncontrolled.idf"
oracle_version = "26.1.0"
weather = "USA_IL_Chicago-OHare.Intl.AP.725300_TMY3.epw"
tier = "A"
comparison_class = "conformance"
conformance_claim = true

[scope]
domains = ["weather", "schedule", "zone", "surface", "meter"]
has_zone = true
has_surface = true
has_fenestration = false
has_air_loop = false
has_plant_loop = false
has_ems = false
has_python_plugin = false

[gate]
ci_gate = true
release_gate = true
manual_review_required = false
```

The v2 validator rejects `conformance_claim = true` unless variables or meters,
tolerances, report artifacts, and a blocking gate are all present.
