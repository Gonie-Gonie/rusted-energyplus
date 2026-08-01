use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
};

mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp377_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2259",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2260",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-supply-temperature-for-saturation-humidity-ratio",
            "read-environment-outdoor-barometric-pressure-for-saturation-humidity-ratio",
            "evaluate-psy-w-fn-tdb-rh-pb-at-unity-relative-humidity",
            "assign-local-saturation-supply-humidity-ratio",
        ],
    );
}

pub(super) fn predecessor_for_route(route: usize, value: f64) -> Predecessor {
    let active = route >= 3;
    let mut predecessor = Predecessor {
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
        _ => unreachable!("eight CP377 routes"),
    }
    predecessor
}
