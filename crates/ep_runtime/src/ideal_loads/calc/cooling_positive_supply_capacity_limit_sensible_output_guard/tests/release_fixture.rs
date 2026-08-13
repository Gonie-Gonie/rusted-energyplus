use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_assignment::tests::release_fixture::{
    completed_cp338_case, completed_cp338_case_with_zone_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard,
};

pub(in crate::ideal_loads::calc) fn completed_cp340_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) {
    let (runtime, system, predecessor) =
        completed_cp338_case(cooling_demand_w, overall_availability, capacity_limit, 0.008);
    complete_cp340(runtime, system, predecessor)
}

pub(in crate::ideal_loads::calc) fn completed_cp340_case_with_zone_temperature(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
    humidity_ratio: f64,
    zone_temperature_c: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) {
    let (runtime, system, predecessor) =
        completed_cp338_case_with_zone_temperature(
            cooling_demand_w,
            overall_availability,
            capacity_limit,
            humidity_ratio,
            zone_temperature_c,
        );
    complete_cp340(runtime, system, predecessor)
}

fn complete_cp340(
    mut runtime: PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystem,
    predecessor:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) {
    let assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP339");
    let guard =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_guard(
            &mut runtime,
            &system,
            assignment,
        )
        .expect("CP340");
    (runtime, system, guard)
}

pub(in crate::ideal_loads::calc) fn align_completed_cp340_capacity_for_successor_tests(
    unit: &mut crate::ideal_loads::PurchasedAirUnitRuntimeState,
    capacity: f64,
) {
    assert!(capacity.is_finite() && capacity > 0.0);
    unit.sized_limits.as_mut().expect("sized limits").maximum_total_cooling_capacity_w =
        Some(ep_model::AutosizeOrNumber::Value(capacity));
    unit.calc_cooling_capacity_zero_flow_reset.latest.as_mut().expect("CP321")
        .maximum_total_cooling_capacity_w = Some(capacity);
    let state = &mut unit.calc_cooling_positive_supply_capacity_limit_sensible_output_guard;
    let latest = state.latest.as_mut().expect("CP340");
    let entered = latest.cooling_sensible_output_w.expect("CP340 output") >= capacity;
    latest.maximum_total_cooling_capacity_w = Some(capacity);
    latest.cooling_sensible_output_at_or_above_maximum_capacity = Some(entered);
    latest.capacity_limit_sensible_output_guard_false_fallthrough = !entered;
    latest.capacity_limit_sensible_output_adjustment_body_entered = entered;
    state.source_site_execution_count = 3 + usize::from(entered);
    state.capacity_limit_sensible_output_guard_false_fallthrough_count = usize::from(!entered);
    state.capacity_limit_sensible_output_adjustment_body_entry_count = usize::from(entered);
    state.witnessed_capacity_limit_sensible_output_guard_false_fallthrough_count = usize::from(!entered);
    state.witnessed_capacity_limit_sensible_output_adjustment_body_entry_count = usize::from(entered);
    state.latest_route = Some(if entered {
        super::super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::CapacityLimitSensibleOutputAdjustmentBodyEntered
    } else {
        super::super::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRetainedRoute::CapacityLimitSensibleOutputGuardFalseFallthrough
    });
}
