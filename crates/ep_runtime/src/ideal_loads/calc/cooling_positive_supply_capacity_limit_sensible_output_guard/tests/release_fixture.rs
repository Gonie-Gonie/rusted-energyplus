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
