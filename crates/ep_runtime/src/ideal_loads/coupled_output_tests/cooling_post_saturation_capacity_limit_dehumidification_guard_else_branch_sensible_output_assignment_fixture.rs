use crate::ideal_loads::{
    DirectZonePurchasedAirScheduledCouplingOutput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput,
};

// Match the production lazy owner gate: inactive routes must not read CP329/CP330.
#[allow(dead_code)]
pub(super) fn cp420_owner_input(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput>{
    output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        .then(|| {
            Some(PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput {
                supply_mass_flow_rate_kg_per_s: output
                    .calculation_cooling_supply_mass_flow_positive_guard
                    .supply_mass_flow_rate_kg_per_s?,
                mixed_air_temperature_c: output
                    .calculation_cooling_mixed_air_call
                    .mixed_air_temperature_c?,
            })
        })
        .flatten()
}

#[test]
fn cp420_fixture_and_hot_validator_are_owner_gated_and_bounded() {
    let source = include_str!(
        "../coupled_runtime/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_validation.rs"
    );
    let production_source = source
        .split_once("#[cfg(test)]")
        .map_or(source, |(production, _)| production);
    assert!(production_source.contains(".then(|| {"));
    assert!(production_source.contains("Some(ActiveInput"));
    assert!(production_source.contains("snapshot_has_exact_cp419_prefix_and_local_assignment"));
    for forbidden in [
        "private_characterization",
        "snapshot_is_exact(",
        "predecessor_route(",
    ] {
        assert!(!production_source.contains(forbidden), "{forbidden}");
    }
}
