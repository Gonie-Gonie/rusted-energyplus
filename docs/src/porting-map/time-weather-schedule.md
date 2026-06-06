---
status: active
claim_level: smoke
owner: runtime
last_reviewed: 2026-06-05
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

Next evidence target:

- declared conformance cases for timestamp alignment
- solar/radiation EPW field comparisons
- Schedule:Compact day-type expansion
