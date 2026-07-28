use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment::tests::release_fixture::{
    completed_cp341_case, completed_cp341_case_with_zone_temperature,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment,
};

pub(in crate::ideal_loads::calc) fn completed_cp342_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
){
    let (runtime, system, predecessor) =
        completed_cp341_case(cooling_demand_w, overall_availability, capacity_limit);
    complete_cp342(runtime, system, predecessor)
}

pub(in crate::ideal_loads::calc) fn completed_cp342_case_with_zone_temperature(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
    humidity_ratio: f64,
    zone_temperature_c: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
){
    let (runtime, system, predecessor) = completed_cp341_case_with_zone_temperature(
        cooling_demand_w,
        overall_availability,
        capacity_limit,
        humidity_ratio,
        zone_temperature_c,
    );
    complete_cp342(runtime, system, predecessor)
}

fn complete_cp342(
    mut runtime: PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystem,
    predecessor:
        crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
){
    let assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP342");
    (runtime, system, assignment)
}
