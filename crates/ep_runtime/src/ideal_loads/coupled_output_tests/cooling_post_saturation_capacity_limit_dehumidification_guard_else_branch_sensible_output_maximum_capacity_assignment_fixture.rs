use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_guard_evaluated;
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_characterization(
        predecessor,
        active.then(|| ActiveInput {
            preexisting_cooling_sensible_output_w: predecessor
                .cp420_cooling_sensible_output_for_capacity_guard_w
                .expect("CP422 active CP421 cooling-output fixture"),
            maximum_total_cooling_capacity_w: predecessor
                .maximum_total_cooling_capacity_w
                .expect("CP422 active CP421 capacity fixture"),
            cp421_retained_maximum_total_cooling_capacity_owned_read: true,
        }),
    )
    .expect("valid CP422 coupled-output fixture")
}
