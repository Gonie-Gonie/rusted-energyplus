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
| ZoneHVAC:IdealLoadsAirSystem | hvac | typed | ideal_loads_no_oa_sensible_conformance_001 | Typed input support plus limited no-OA/no-limit IdealLoads sensible conformance evidence, diagnostic finite flow/capacity evidence with return-node proof rows and tracked gaps, diagnostic ReportPurchasedAir energy and blank/constant Schedule:Constant fuel-efficiency evidence, diagnostic hourly facility meter oracle-MTR vs Rust aggregated fuel-energy evidence with RuntimeMeterRegistry request resolution, and diagnostic outdoor-air design-flow/supply-air/mixed-air/inactive-economizer-heat-recovery evidence for declared output variables only; finite-limit conformance, outdoor-air conformance, humidity, active economizer, active heat recovery, adaptive system timestep, sizing, non-constant efficiency schedules, broad meter conformance, and broad HVAC compatibility remain outside the claim. |
| DesignSpecification:OutdoorAir | hvac | diagnostic | ideal_loads_outdoor_air_design_flow_diagnostic_001 | Typed for IdealLoads outdoor-air reference/schedule preservation plus diagnostic-only Flow/Zone design-flow mass, standard-density volume flow, no-humidity outdoor-air report-rate, supply-air state, no-heat-recovery mixed-air state, and inactive economizer/heat-recovery report parity; no active DCV, active economizer, active heat recovery, or numerical OA conformance claim. |
| NodeList | hvac | typed_graph_only | air_side_node_diagnostic_001 | Typed graph and node expansion only; node outputs remain diagnostic. |
| PlantLoop | plant | typed_graph_only | plant_loop_diagnostic_001 | Typed graph and diagnostic projection only; no plant loop simulation conformance. |
| Pump:ConstantSpeed | plant | typed_graph_only | plant-loop-skeleton-smoke | Typed graph equipment reference only; no pump performance or flow-control algorithm. |
| Boiler:HotWater | plant | typed_graph_only | plant-loop-skeleton-smoke | Typed graph equipment reference only; no boiler performance algorithm. |
| Chiller:Electric:EIR | plant | typed_graph_only | plant-loop-skeleton-smoke | Typed graph equipment reference only; no chiller performance algorithm. |
