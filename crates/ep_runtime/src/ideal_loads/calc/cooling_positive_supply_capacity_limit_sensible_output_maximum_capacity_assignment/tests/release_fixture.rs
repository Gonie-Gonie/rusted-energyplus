use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard::tests::release_fixture::{
    completed_cp340_case, completed_cp340_case_with_zone_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment,
};

pub(in crate::ideal_loads::calc) fn completed_cp341_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) {
    let (runtime, system, predecessor) =
        completed_cp340_case(cooling_demand_w, overall_availability, capacity_limit);
    complete_cp341(runtime, system, predecessor)
}

pub(in crate::ideal_loads::calc) fn completed_cp341_case_with_zone_temperature(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
    humidity_ratio: f64,
    zone_temperature_c: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) {
    let (runtime, system, predecessor) =
        completed_cp340_case_with_zone_temperature(
            cooling_demand_w,
            overall_availability,
            capacity_limit,
            humidity_ratio,
            zone_temperature_c,
        );
    complete_cp341(runtime, system, predecessor)
}

fn complete_cp341(
    mut runtime: PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystem,
    predecessor:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) {
    let assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP341");
    (runtime, system, assignment)
}
