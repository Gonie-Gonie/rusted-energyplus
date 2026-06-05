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
- schedule and weather ESO smoke comparisons

Next evidence target:

- declared conformance cases for timestamp alignment
- broader EPW field comparisons
- Schedule:Compact day-type expansion

