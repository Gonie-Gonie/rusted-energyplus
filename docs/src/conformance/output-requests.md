---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Output Requests

Output requests are part of the evidence contract. A report cannot silently
compare whichever variables happen to be present.

Example:

```toml
[[outputs]]
key = "*"
variable = "Zone Mean Air Temperature"
frequency = "hourly"
class = "zone-state"
source = "eso"

[[outputs]]
key = "*"
variable = "Site Outdoor Air Drybulb Temperature"
frequency = "hourly"
class = "weather"
source = "eso"

[[outputs]]
key = "*"
variable = "HeatTransfer Surface Azimuth"
frequency = "static"
class = "surface-state"
source = "eio"

[[outputs]]
key = "*"
variable = "Construction CTF Thermal Conductance"
frequency = "static"
class = "construction-material"
source = "eio"

[[outputs]]
key = "*"
variable = "OtherEquipment Internal Gains Nominal Equipment Level"
frequency = "static"
class = "internal-gain"
source = "eio"
```

Each output request must define key, variable, frequency, variable class, and
source artifact. Supported variable class values are `schedule`, `weather`,
`construction-material`, `internal-gain`, `zone-state`, `surface-state`,
`meter`, `internal-variable`, and `diagnostic`. Supported source values are
`eso`, `eio`, `mtr`, `sql`, and `csv`.

Use `static` for EIO/static report rows that do not have a timestep axis, such
as surface geometry fields.
