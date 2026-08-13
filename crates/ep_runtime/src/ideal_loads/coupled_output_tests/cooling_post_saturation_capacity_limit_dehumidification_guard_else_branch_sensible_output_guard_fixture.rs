use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_snapshot(
    predecessor: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot,
    capacity: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    corroborator: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot{
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed;
    if active {
        assert!(
            capacity.maximum_total_cooling_capacity_read
                && capacity.maximum_total_cooling_capacity_w.is_some(),
            "CP421 active fixture must retain the physical CP321 capacity owner"
        );
        assert!(
            corroborator.maximum_total_cooling_capacity_read
                && corroborator.maximum_total_cooling_capacity_w.is_some(),
            "CP421 active fixture must retain the same-call CP340 corroborator"
        );
    }
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_characterization(
        predecessor,
        active.then(|| PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput {
            cooling_sensible_output_w: predecessor
                .cooling_sensible_output_w
                .expect("CP421 active CP420 cooling-output fixture"),
            maximum_total_cooling_capacity_w: capacity
                .maximum_total_cooling_capacity_w
                .expect("CP421 active CP321 capacity fixture"),
            cp420_cooling_sensible_output_owned_read: true,
            cp321_maximum_total_cooling_capacity_owned_read: true,
            cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: corroborator
                .maximum_total_cooling_capacity_w
                .is_some_and(|value| {
                    value.to_bits()
                        == capacity
                            .maximum_total_cooling_capacity_w
                            .expect("CP421 active CP321 capacity fixture")
                            .to_bits()
                }),
        }),
    )
    .expect("valid CP421 coupled-output fixture")
}

#[test]
fn no_limit_coupled_fixture_cannot_forge_an_active_capacity_guard_operand() {
    let source = include_str!("cooling_capacity_zero_flow_reset_fixture.rs");
    assert!(source.contains("maximum_total_cooling_capacity_read: false"));
    assert!(source.contains("maximum_total_cooling_capacity_w: None"));
    let fixture = include_str!("../coupled_output_tests.rs");
    assert!(fixture.contains("cooling_limit: IdealLoadsLimit::NoLimit"));
}
