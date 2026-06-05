---
status: draft
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# ExampleFile Suite

The suite should use both minimal reduced cases and selected EnergyPlus
ExampleFiles/test files.

| Tier | Purpose | Case type |
|---|---|---|
| T0 | parser/schema smoke | minimal IDF or epJSON |
| T1 | schedule/weather | controlled output variables |
| T2 | geometry/static properties | one-zone simple geometry |
| T3 | uncontrolled heat balance | no-HVAC or HVAC-disabled cases |
| T4 | IdealLoads | IdealLoads examples |
| T5 | simple zone equipment | selected fan/coil/PTAC cases |
| T6 | air loop | selected AirLoopHVAC cases |
| T7 | plant loop | selected boiler/chiller/pump cases |
| T8 | issue regression | cases derived from known issues |

Each tier must declare its variable classes before it supports a claim.

