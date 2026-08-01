use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput as SaturationInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as SaturationState,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as TemperatureOwner,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as OriginalSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
};

mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp378_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2260",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2261",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_LIMIT_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-local-original-supply-humidity-ratio-for-saturation-limit-minimum",
            "read-local-saturation-supply-humidity-ratio-for-saturation-limit-minimum",
            "apply-source-shaped-two-argument-minimum-for-saturation-limit",
            "assign-purchased-air-supply-humidity-ratio-for-saturation-limit",
        ],
    );
}

pub(super) fn predecessor_for_route(route: usize, original: f64) -> Predecessor {
    predecessor_for_route_with_psychrometrics(route, original, 12.0 + route as f64, 101_325.0)
}

pub(super) fn predecessor_for_route_with_psychrometrics(
    route: usize,
    original: f64,
    temperature: f64,
    pressure: f64,
) -> Predecessor {
    let cp376 = original_snapshot_for_route(route, original);
    let input = (route >= 3).then_some(SaturationInput {
        supply_temperature_c: temperature,
        temperature_owner: if route.is_multiple_of(2) {
            TemperatureOwner::Cp344CapacityMixedAirLimit
        } else {
            TemperatureOwner::Cp334MixedAirLimit
        },
        outdoor_barometric_pressure_pa: pressure,
    });
    let mut state = SaturationState::new(cp376.system);
    advance_cooling_supply_humidity_ratio_saturation_assignment_state(&mut state, cp376, input)
        .expect("valid CP377 predecessor fixture")
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
        _ => unreachable!("eight CP378 routes"),
    }
    predecessor
}
