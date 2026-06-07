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
| static zone | Zone Information surface count, floor area, volume, exterior gross wall area | EIO | smoke |
| static surface | HeatTransfer Surface class, net area, gross area, azimuth, tilt | EIO | smoke |
| construction/material | Construction CTF layer count and thermal conductance; Material CTF Summary thickness, conductivity, density, specific heat, thermal resistance | EIO | smoke |
| internal gains | OtherEquipment Internal Gains Nominal zone floor area, equipment level, equipment per floor area, latent/radiant/lost/convected fractions; Zone Total Internal Convective Heating Rate | EIO, ESO | smoke |
| zone heat balance | Zone Mean Air Temperature; Zone Air Heat Balance Surface Convection Rate; Zone Air Heat Balance Air Energy Storage Rate | ESO | conformance for `heat_balance_nomass_001` `Zone Mean Air Temperature`; otherwise diagnostic |
| surface heat balance | Surface Inside Face Temperature; Surface Outside Face Temperature; Surface Inside Face Conduction Heat Transfer Rate; Surface Outside Face Conduction Heat Transfer Rate | ESO | conformance for `surface_temperature_nomass_001` inside/outside face temperatures; conduction rates otherwise diagnostic |
| fenestration/solar | Surface Window Transmitted Solar Radiation Rate; Surface Inside Face Solar Radiation Heat Gain Rate; Surface Outside Face Incident Solar Radiation Rate per Area | ESO | diagnostic until a separate declared case exists |
| thermostat and IdealLoads | Zone Thermostat Heating Setpoint Temperature; Zone Thermostat Cooling Setpoint Temperature; Zone Ideal Loads Zone Total Heating Rate; Zone Ideal Loads Zone Total Cooling Rate | ESO | smoke and baseline-only for `ideal_loads_thermostat_001`; not an IdealLoads load-conformance claim |
| node | System Node Temperature; System Node Humidity Ratio; System Node Mass Flow Rate; System Node Setpoint Temperature | ESO | diagnostic-only for `air_side_node_diagnostic_001`; mapped in `node-state-source-map.md`; setpoint temperature remains future-gated; no node numerical conformance claim |
| component | Fan Electricity Rate; Cooling Coil Total Cooling Rate; Heating Coil Heating Rate | ESO | diagnostic until component port |
| plant loop and equipment | Plant Supply Side Cooling Demand Rate; Plant Supply Side Heating Demand Rate; Plant Supply Side Inlet Mass Flow Rate; Plant Supply Side Inlet Temperature; Plant Supply Side Outlet Temperature; Pump Electricity Rate; Boiler Heating Rate; Chiller Electricity Rate; Chiller Evaporator Cooling Rate | ESO | future diagnostic only; mapped in `plant-source-map.md`; no plant numerical conformance claim |
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
