use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    let assignment = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_executed;
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_characterization(
        predecessor,
        assignment.then(|| ActiveInput {
            mixed_air_temperature_c: predecessor
                .mixed_air_temperature_for_sensible_output_c
                .expect("CP423 active CP422 mixed-air temperature fixture"),
            cooling_sensible_output_w: predecessor
                .resulting_cooling_sensible_output_after_maximum_capacity_assignment_w
                .expect("CP423 active CP422 cooling-output fixture"),
            supply_mass_flow_rate_kg_per_s: predecessor
                .supply_mass_flow_rate_kg_per_s
                .expect("CP423 active CP422 mass-flow fixture"),
            cp_air_j_per_kg_k: predecessor
                .cp_air_j_per_kg_k
                .expect("CP423 active CP422 CpAir fixture"),
        }),
    )
    .expect("valid CP423 coupled-output fixture")
}
