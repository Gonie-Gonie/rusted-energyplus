<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

# Object Coverage

Object coverage is maintained in `specs/object_coverage.toml`.

| Object | Family | Status | First evidence | Boundary |
|---|---|---|---|---|
| Version | project | typed | heat_balance_nomass_001 | Accepted for oracle-version tracking; no runtime compatibility claim by itself. |
| Building | building | typed | heat_balance_nomass_001 | Typed for model metadata and simple building scope; no global building simulation claim. |
| Timestep | time | typed | heat_balance_nomass_001 | Typed for fixed time-axis plumbing; no full EnergyPlus timestep-manager parity claim. |
| RunPeriod | time | typed | heat_balance_nomass_001 | Typed for simple run-period time axes; no design-day, sizing-period, or warmup claim. |
| Site:Location | site | typed | weather_fields_001 | Typed for location metadata; weather conformance is limited to declared dry-bulb output. |
| Material | material | typed | construction_materials_001 | Typed for static material and construction evidence; no broad dynamic material algorithm claim. |
| Material:NoMass | material | typed | heat_balance_nomass_001 | Typed for declared opaque no-mass heat-balance cases only. |
| Construction | construction | typed | heat_balance_nomass_001 | Typed for surface/material links and declared static construction evidence. |
| ScheduleTypeLimits | schedule | typed | schedule_constant_001 | Typed for schedule metadata; no full schedule validation claim. |
| Schedule:Constant | schedule | typed | schedule_constant_001 | Typed and tolerance-gated for declared Schedule Value evidence. |
| Schedule:Compact | schedule | typed | ideal_loads_thermostat_001 | Typed for selected AllDays/Until segments; no broad compact-schedule grammar claim. |
| OtherEquipment | internal_gains | typed | internal_gains_001 | Typed for nominal EIO evidence and declared convective-gain trace only. |
| ThermostatSetpoint:DualSetpoint | thermostat | typed | ideal_loads_thermostat_001 | Typed for thermostat graph wiring; no HVAC control algorithm conformance. |
| ZoneControl:Thermostat | thermostat | typed | ideal_loads_thermostat_001 | Typed for thermostat references; no load-control numerical conformance. |
| ZoneHVAC:IdealLoadsAirSystem | hvac | typed_graph_only | ideal_loads_thermostat_001 | Typed graph only; no IdealLoads load or HVAC numerical conformance. |
| NodeList | hvac | typed_graph_only | air_side_node_diagnostic_001 | Typed graph and node expansion only; node outputs remain diagnostic. |
| PlantLoop | plant | typed_graph_only | plant_loop_diagnostic_001 | Typed graph and diagnostic projection only; no plant loop simulation conformance. |
| Pump:ConstantSpeed | plant | typed_graph_only | plant-loop-skeleton-smoke | Typed graph equipment reference only; no pump performance or flow-control algorithm. |
| Boiler:HotWater | plant | typed_graph_only | plant-loop-skeleton-smoke | Typed graph equipment reference only; no boiler performance algorithm. |
| Chiller:Electric:EIR | plant | typed_graph_only | plant-loop-skeleton-smoke | Typed graph equipment reference only; no chiller performance algorithm. |
