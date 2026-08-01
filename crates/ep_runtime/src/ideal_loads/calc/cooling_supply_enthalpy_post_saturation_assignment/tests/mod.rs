use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, ZoneId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput as ActiveInput,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput as SaturationInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as SaturationState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as TemperatureOwner,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_limit_assignment::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as LimitState,
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as OriginalSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Cp377Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Cp378Snapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
};

mod ieee;
mod overflow;
mod release_corruption;
mod routes;

#[test]
fn cp379_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2261",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2264",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-post-saturation-enthalpy",
            "read-purchased-air-supply-humidity-ratio-for-post-saturation-enthalpy",
            "evaluate-psy-h-fn-tdb-w-for-post-saturation-enthalpy",
            "assign-local-supply-enthalpy-after-saturation-limit",
        ],
    );
}

#[derive(Clone, Copy)]
pub(super) struct PurePrefix {
    pub cp377: Cp377Snapshot,
    pub cp378: Cp378Snapshot,
    pub input: Option<ActiveInput>,
}

pub(super) fn prefix_for_route(route: usize, humidity_ratio: f64) -> PurePrefix {
    prefix_for_route_with_psychrometrics(
        route,
        humidity_ratio,
        12.0 + route as f64,
        101_325.0,
    )
}

pub(super) fn prefix_for_route_with_psychrometrics(
    route: usize,
    humidity_ratio: f64,
    temperature: f64,
    pressure: f64,
) -> PurePrefix {
    let cp376 = original_snapshot_for_route(route, humidity_ratio);
    let owner = if route.is_multiple_of(2) {
        TemperatureOwner::Cp344CapacityMixedAirLimit
    } else {
        TemperatureOwner::Cp334MixedAirLimit
    };
    let saturation_input = (route >= 3).then_some(SaturationInput {
        supply_temperature_c: temperature,
        temperature_owner: owner,
        outdoor_barometric_pressure_pa: pressure,
    });
    let mut saturation_state = SaturationState::new(cp376.system);
    let cp377 = advance_cooling_supply_humidity_ratio_saturation_assignment_state(
        &mut saturation_state,
        cp376,
        saturation_input,
    )
    .expect("valid CP377 fixture");
    let mut limit_state = LimitState::new(cp377.system);
    let cp378 = advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state(
        &mut limit_state,
        cp377,
    )
    .expect("valid CP378 fixture");
    PurePrefix {
        cp377,
        cp378,
        input: (route >= 3).then_some(ActiveInput {
            supply_temperature_c: temperature,
            temperature_owner: owner,
        }),
    }
}

fn original_snapshot_for_route(route: usize, value: f64) -> OriginalSnapshot {
    let active = route >= 3;
    let mut predecessor = OriginalSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(1),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(2),
        unit_off_skipped: route == 0,
        non_cooling_skipped: route == 1,
        positive_guard_false_fallthrough_skipped: route == 2,
        heating_availability_guard_false_fallthrough: route == 3,
        humidification_control_guard_false_fallthrough: route == 4,
        dehumidification_control_humidistat_maximum_assignment_executed: route == 5,
        dehumidification_control_none_maximum_assignment_executed: route == 6,
        dehumidification_control_guard_false_fallthrough: route == 7,
        predecessor_dehumidification_control_type: active
            .then_some(DehumidificationControlType::None),
        predecessor_purchased_air_supply_humidity_ratio_assignment_performed: false,
        predecessor_resulting_supply_humidity_ratio: None,
        cp375_maximum_assignment_owned_read: false,
        cp347_none_case_owned_read: matches!(route, 3 | 4),
        cp356_constant_shr_owned_read: route == 7,
        cp362_humidistat_owned_read: false,
        cp365_constant_supply_humidity_ratio_owned_read: false,
        purchased_air_supply_humidity_ratio_read: active,
        purchased_air_supply_humidity_ratio_before_saturation_check: active.then_some(value),
        local_supply_humidity_ratio_original_assignment_performed: active,
        assigned_supply_humidity_ratio_original: active.then_some(value),
        resulting_supply_humidity_ratio_original: active.then_some(value),
    };
    match route {
        5 => {
            predecessor.predecessor_dehumidification_control_type =
                Some(DehumidificationControlType::Humidistat);
            predecessor.predecessor_purchased_air_supply_humidity_ratio_assignment_performed = true;
            predecessor.predecessor_resulting_supply_humidity_ratio = Some(value);
            predecessor.cp375_maximum_assignment_owned_read = true;
            predecessor.cp347_none_case_owned_read = false;
        }
        6 => {
            predecessor.predecessor_purchased_air_supply_humidity_ratio_assignment_performed = true;
            predecessor.predecessor_resulting_supply_humidity_ratio = Some(value);
            predecessor.cp375_maximum_assignment_owned_read = true;
            predecessor.cp347_none_case_owned_read = false;
        }
        7 => {
            predecessor.predecessor_dehumidification_control_type =
                Some(DehumidificationControlType::ConstantSensibleHeatRatio);
        }
        0..=4 => {}
        _ => unreachable!("eight CP379 routes"),
    }
    predecessor
}

pub(super) fn completed_cp378_case() -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    Cp378Snapshot,
) {
    let (mut runtime, system, cp370) =
        completed_cp370_case_for_cp372_test().expect("CP370 fixture");
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .expect("CP371 direct");
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp371,
    )
    .expect("CP372 direct");
    let cp373 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
        &mut runtime,
        &system,
        cp372,
    )
    .expect("CP373 direct");
    let cp374 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
        &mut runtime,
        &system,
        cp373,
    )
    .expect("CP374 direct");
    let cp375 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
        &mut runtime,
        &system,
        cp374,
    )
    .expect("CP375 direct");
    let cp376 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .expect("CP376 direct");
    let cp377 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(
        &mut runtime,
        &system,
        cp376,
        101_325.0,
    )
    .expect("CP377 direct");
    let cp378 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .expect("CP378 direct");
    (runtime, system, cp378)
}
