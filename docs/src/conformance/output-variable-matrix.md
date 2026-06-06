---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Output Variable Matrix

Every ExampleFiles-based case must declare output variables and meters before
it can be used as compatibility evidence. Output availability alone is not a
conformance claim.

| Domain | Initial variables or meters | Source | Earliest intended level |
|---|---|---|---|
| weather | Site Outdoor Air Drybulb Temperature; Site Outdoor Air Dewpoint Temperature; Site Outdoor Air Relative Humidity; Site Outdoor Air Barometric Pressure; Site Wind Speed; Site Wind Direction | ESO | smoke, then conformance candidate |
| weather radiation parser | horizontal infrared radiation intensity; global horizontal radiation; direct normal radiation; diffuse horizontal radiation | EPW | parser-only smoke |
| solar weather | Site Direct Solar Radiation Rate per Area; Site Diffuse Solar Radiation Rate per Area; Site Solar Altitude Angle; Site Solar Azimuth Angle | ESO | diagnostic |
| schedule | Schedule Value | ESO | smoke, then conformance candidate |
| static zone | Zone Information fields such as floor area, volume, and surface count | EIO | smoke |
| static surface | HeatTransfer Surface class, area, azimuth, and tilt | EIO | smoke |
| construction/material | Construction CTF and Material CTF Summary fields | EIO | smoke |
| internal gains | OtherEquipment Internal Gains Nominal; Zone Total Internal Convective Heating Rate | EIO, ESO | smoke or diagnostic |
| zone heat balance | Zone Mean Air Temperature; Zone Air Heat Balance Surface Convection Rate; Zone Air Heat Balance Air Energy Storage Rate | ESO | diagnostic until v0.8 |
| surface heat balance | Surface Inside Face Temperature; Surface Outside Face Temperature; Surface Inside Face Conduction Heat Transfer Rate; Surface Outside Face Conduction Heat Transfer Rate | ESO | diagnostic until v0.8 |
| fenestration/solar | Surface Window Transmitted Solar Radiation Rate; Surface Inside Face Solar Radiation Heat Gain Rate; Surface Outside Face Incident Solar Radiation Rate per Area | ESO | diagnostic until v0.9 |
| node | System Node Temperature; System Node Humidity Ratio; System Node Mass Flow Rate; System Node Setpoint Temperature | ESO | diagnostic until v0.11 |
| component | Fan Electricity Rate; Cooling Coil Total Cooling Rate; Heating Coil Heating Rate | ESO | diagnostic until component port |
| facility meter | Electricity:Facility; Gas:Facility; Heating:EnergyTransfer; Cooling:EnergyTransfer | MTR, SQL, CSV | diagnostic until meter contract |

## Request Policy

Requested variables must record:

- key
- variable or meter name
- frequency
- domain
- evidence level
- absolute and relative tolerance, when conformance-level
- source file target through `source = "eio"`, `source = "eso"`,
  `source = "mtr"`, `source = "sql"`, or `source = "csv"`

`level = "diagnostic"` means values may be compared and deltas may be reported,
but the result must not be described as an EnergyPlus compatibility pass.

## Injection Policy

ExampleFiles do not share a stable output set. A future output patching step
must create a patched IDF that injects required `Output:Variable`,
`Output:Meter`, `Output:SQLite`, and file-output controls. The patch step must
also write a patch report listing added and already-present output requests.
