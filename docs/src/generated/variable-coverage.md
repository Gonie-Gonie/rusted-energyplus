<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

# Variable Coverage

Variable coverage is maintained in `specs/variable_coverage.toml`.

| Variable | Domain | Status | First evidence | Boundary |
|---|---|---|---|---|
| Zone Mean Air Temperature | heat_balance | conformance | heat_balance_nomass_001 | Tolerance-gated only for declared no-mass heat-balance cases; no general heat-balance or HVAC claim. |
| Surface Inside Face Temperature | heat_balance | conformance | surface_temperature_nomass_001 | Tolerance-gated only for declared opaque no-mass surface cases; no solar, fenestration, or general surface heat-balance claim. |
| Surface Outside Face Temperature | heat_balance | conformance | surface_temperature_nomass_001 | Tolerance-gated only for declared opaque no-mass surface cases; no solar, fenestration, or general exterior boundary claim. |
| System Node Temperature | hvac | diagnostic | air_side_node_diagnostic_001 | Diagnostic node-state evidence only; no HVAC or node numerical conformance. |
| System Node Mass Flow Rate | hvac | diagnostic | air_side_node_diagnostic_001 | Diagnostic node-state evidence only; no HVAC flow balancing or node numerical conformance. |
| Plant Supply Side Inlet Temperature | plant | diagnostic | plant_loop_diagnostic_001 | Diagnostic plant-state projection only; no plant loop simulation conformance. |
| Plant Supply Side Outlet Temperature | plant | diagnostic | plant_loop_diagnostic_001 | Diagnostic plant-state projection only; no plant loop simulation conformance. |
