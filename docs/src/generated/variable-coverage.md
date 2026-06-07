<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

# Variable Coverage

Variable coverage is maintained in `specs/variable_coverage.toml`.

| Variable | Domain | Status | First evidence | Boundary |
|---|---|---|---|---|
| Zone Ideal Loads Zone Total Cooling Rate | hvac | baseline | air_side_node_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Zone Ideal Loads Zone Total Heating Rate | hvac | baseline | air_side_node_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| System Node Humidity Ratio | node | baseline | air_side_node_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| District Heating Water Rate | plant | baseline | plant_loop_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Plant Load Profile Heat Transfer Rate | plant | baseline | plant_loop_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Plant Supply Side Cooling Demand Rate | plant | baseline | plant_loop_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Plant Supply Side Heating Demand Rate | plant | baseline | plant_loop_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Plant Supply Side Inlet Mass Flow Rate | plant | baseline | plant_loop_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Pump Electricity Rate | plant | baseline | plant_loop_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Zone Thermostat Cooling Setpoint Temperature | zone | baseline | air_side_node_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Zone Thermostat Heating Setpoint Temperature | zone | baseline | air_side_node_diagnostic_001 | EnergyPlus oracle baseline request only; Rust numerical parity is not claimed. |
| Construction CTF Layer Count | construction | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Construction CTF Thermal Conductance | construction | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Material CTF Summary Conductivity | construction | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Material CTF Summary Density | construction | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Material CTF Summary Specific Heat | construction | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Material CTF Summary Thermal Resistance | construction | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Material CTF Summary Thickness | construction | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Surface Inside Face Temperature | heat_balance | conformance | surface_temperature_nomass_001 | Tolerance-gated only for declared opaque no-mass surface cases; no solar, fenestration, or general surface heat-balance claim. |
| Surface Outside Face Temperature | heat_balance | conformance | surface_temperature_nomass_001 | Tolerance-gated only for declared opaque no-mass surface cases; no solar, fenestration, or general exterior boundary claim. |
| Zone Mean Air Temperature | heat_balance | conformance | heat_balance_nomass_001 | Tolerance-gated only for declared no-mass heat-balance cases; no general heat-balance or HVAC claim. |
| OtherEquipment Internal Gains Nominal Equipment Level | internal-gain | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| OtherEquipment Internal Gains Nominal Equipment per Floor Area | internal-gain | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| OtherEquipment Internal Gains Nominal Fraction Convected | internal-gain | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| OtherEquipment Internal Gains Nominal Fraction Latent | internal-gain | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| OtherEquipment Internal Gains Nominal Fraction Lost | internal-gain | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| OtherEquipment Internal Gains Nominal Fraction Radiant | internal-gain | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| OtherEquipment Internal Gains Nominal Zone Floor Area | internal-gain | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Zone Total Internal Convective Heating Rate | internal-gain | conformance | internal_gains_001 | Tolerance-gated time-series conformance only for the listed evidence case, variable, frequency, and gate. |
| Schedule Value | schedule | conformance | schedule_constant_001 | Tolerance-gated time-series conformance only for the listed evidence case, variable, frequency, and gate. |
| HeatTransfer Surface Area (Gross) | surface | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| HeatTransfer Surface Area (Net) | surface | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| HeatTransfer Surface Azimuth | surface | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| HeatTransfer Surface Class | surface | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| HeatTransfer Surface Tilt | surface | conformance | official_1zone_static_model_001 | Static EIO conformance only; no dynamic runtime response or algorithm parity claim. |
| Surface Inside Face Conduction Heat Transfer Rate | surface | conformance | surface_temperature_nomass_001 | Tolerance-gated no-mass adiabatic hourly conformance only; official ExampleFile dynamic conduction remains a promotion candidate, not a claim. |
| Surface Inside Face Conduction Heat Transfer Rate per Area | surface | conformance | surface_temperature_nomass_001 | Tolerance-gated no-mass adiabatic hourly conformance only; official ExampleFile dynamic conduction remains a promotion candidate, not a claim. |
| Surface Outside Face Conduction Heat Transfer Rate | surface | conformance | surface_temperature_nomass_001 | Tolerance-gated no-mass adiabatic hourly conformance only; official ExampleFile dynamic conduction remains a promotion candidate, not a claim. |
| Surface Outside Face Conduction Heat Transfer Rate per Area | surface | conformance | surface_temperature_nomass_001 | Tolerance-gated no-mass adiabatic hourly conformance only; official ExampleFile dynamic conduction remains a promotion candidate, not a claim. |
| Zone Opaque Surface Inside Faces Conduction Rate | surface | conformance | surface_temperature_nomass_001 | Tolerance-gated no-mass adiabatic hourly conformance only; official ExampleFile dynamic conduction remains a promotion candidate, not a claim. |
| Site Outdoor Air Drybulb Temperature | weather | conformance | weather_fields_001 | Tolerance-gated time-series conformance only for the listed evidence case, variable, frequency, and gate. |
| System Node Mass Flow Rate | hvac | diagnostic | air_side_node_diagnostic_001 | Diagnostic node-state evidence only; no HVAC flow balancing or node numerical conformance. |
| System Node Temperature | hvac | diagnostic | air_side_node_diagnostic_001 | Diagnostic node-state evidence only; no HVAC or node numerical conformance. |
| Plant Supply Side Inlet Temperature | plant | diagnostic | plant_loop_diagnostic_001 | Diagnostic plant-state projection only; no plant loop simulation conformance. |
| Plant Supply Side Outlet Temperature | plant | diagnostic | plant_loop_diagnostic_001 | Diagnostic plant-state projection only; no plant loop simulation conformance. |
| Site Outdoor Air Barometric Pressure | weather | diagnostic | weather_fields_001 | Diagnostic comparison or extraction only; not release conformance. |
| Site Outdoor Air Dewpoint Temperature | weather | diagnostic | weather_fields_001 | Diagnostic comparison or extraction only; not release conformance. |
| Site Outdoor Air Relative Humidity | weather | diagnostic | weather_fields_001 | Diagnostic comparison or extraction only; not release conformance. |
| Site Wind Direction | weather | diagnostic | weather_fields_001 | Diagnostic comparison or extraction only; not release conformance. |
| Site Wind Speed | weather | diagnostic | weather_fields_001 | Diagnostic comparison or extraction only; not release conformance. |
