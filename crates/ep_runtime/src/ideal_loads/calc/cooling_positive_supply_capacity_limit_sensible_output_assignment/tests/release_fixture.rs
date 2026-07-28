use crate::ideal_loads::calc::cooling_economizer_condition_release_tests::cp338_fixture::
    release_fixture_with_cooling_demand_availability_and_capacity_limit;
use crate::ideal_loads::calc::cooling_economizer_condition_release_tests::
    release_fixture_with_cooling_demand_and_availability;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_economizer_body,
    advance_direct_no_oa_calc_cooling_economizer_condition,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_sensible_flow,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body,
};

pub(super) fn completed_cp338_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
    humidity_ratio: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) {
    completed_cp338_case_with_zone_temperature(
        cooling_demand_w,
        overall_availability,
        capacity_limit,
        humidity_ratio,
        22.0,
    )
}

pub(super) fn completed_cp338_case_with_zone_temperature(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
    humidity_ratio: f64,
    zone_temperature_c: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) {
    let (mut runtime, system, guard) = if capacity_limit {
        release_fixture_with_cooling_demand_availability_and_capacity_limit(
            cooling_demand_w,
            overall_availability,
        )
    } else {
        release_fixture_with_cooling_demand_and_availability(
            cooling_demand_w,
            overall_availability,
        )
    };
    let condition =
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, &system, guard)
            .expect("CP316");
    let body =
        advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, &system, condition)
            .expect("CP317");
    let mut zone_state =
        crate::ideal_loads::calc::cooling_sensible_flow_release_tests::zone_state(
            body.controlled_zone,
        );
    zone_state.mean_air_temperature_c = zone_temperature_c;
    zone_state.air_humidity_ratio = humidity_ratio;
    let sensible = advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        body,
        &zone_state,
    )
    .expect("CP318");
    let dehumidification = advance_direct_no_oa_calc_cooling_dehumidification_flow(
        &mut runtime,
        &system,
        sensible,
    )
    .expect("CP319");
    let humidification = advance_direct_no_oa_calc_cooling_humidification_flow(
        &mut runtime,
        &system,
        dehumidification,
    )
    .expect("CP320");
    let reset = advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
        &mut runtime,
        &system,
        humidification,
    )
    .expect("CP321");
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(&mut runtime, &system, reset)
            .expect("CP322");
    let ems_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        &mut runtime,
        &system,
        maximum,
    )
    .expect("CP323");
    let ems_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
        &mut runtime,
        &system,
        ems_guard,
    )
    .expect("CP324");
    let limit_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
        &mut runtime,
        &system,
        ems_body,
    )
    .expect("CP325");
    let limit_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
        &mut runtime,
        &system,
        limit_guard,
    )
    .expect("CP326");
    let very_small_guard =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            &system,
            limit_body,
        )
        .expect("CP327");
    let very_small_body =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
            &mut runtime,
            &system,
            very_small_guard,
        )
        .expect("CP328");
    let mixed_air = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        very_small_body,
        &zone_state,
    )
    .expect("CP329");
    let positive_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
        &mut runtime,
        &system,
        mixed_air,
    )
    .expect("CP330");
    let cp_air_assignment = advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
        &mut runtime,
        &system,
        positive_guard,
        &zone_state,
    )
    .expect("CP331");
    let temperature_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut runtime,
            &system,
            cp_air_assignment,
            &zone_state,
        )
        .expect("CP332");
    let minimum_limit =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut runtime,
            &system,
            temperature_assignment,
        )
        .expect("CP333");
    let mixed_air_limit =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            minimum_limit,
        )
        .expect("CP334");
    let humidity_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            mixed_air_limit,
        )
        .expect("CP335");
    let enthalpy_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            humidity_assignment,
        )
        .expect("CP336");
    let capacity_guard =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            &mut runtime,
            &system,
            enthalpy_assignment,
        )
        .expect("CP337");
    let cp_air_capacity_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_cp_air_assignment(
            &mut runtime,
            &system,
            capacity_guard,
        )
        .expect("CP338");
    (runtime, system, cp_air_capacity_assignment)
}

pub(super) fn active_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
) {
    completed_cp338_case(-1_000.0, 1.0, true, 0.008)
}
