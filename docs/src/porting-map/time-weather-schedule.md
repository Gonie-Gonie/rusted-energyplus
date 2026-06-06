---
status: active
claim_level: smoke
owner: runtime
last_reviewed: 2026-06-07
---

# Time, Weather, and Schedule

Implemented foundations:

- `RunPeriod` typed intake
- hourly time-axis expansion
- EPW weather record parsing beyond dry-bulb
- `Schedule:Constant`
- `Schedule:Compact` all-days `Until` subset
- schedule ESO smoke comparison
- weather ESO smoke comparison for dry-bulb, dew point, relative humidity,
  barometric pressure, wind speed, and wind direction

Timestamp semantics:

- hourly samples are hour-ending values in EnergyPlus output order
- `RunPeriod` begin and end dates are inclusive
- `hour = 1` is the first hour-ending sample of a day; `hour = 24` is the last
- missing run-period year currently resolves to 2013 for deterministic
  foundations
- daylight-saving, leap-year, design-day, warmup, and subhourly alignment are
  not yet conformance claims

Weather field status:

- comparison smoke: dry-bulb, dew point, relative humidity, barometric
  pressure, wind speed, wind direction
- parser-only radiation fields: horizontal infrared, global horizontal, direct
  normal, diffuse horizontal radiation
- future ESO solar diagnostics: direct solar radiation rate per area, diffuse
  solar radiation rate per area, solar altitude angle, solar azimuth angle

Next evidence target:

- declared conformance cases for timestamp alignment
- solar/radiation EPW field comparisons
- Schedule:Compact day-type expansion
