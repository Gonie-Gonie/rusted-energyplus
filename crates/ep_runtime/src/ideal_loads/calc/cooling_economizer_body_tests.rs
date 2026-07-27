use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, OutdoorAirEconomizerType, ZoneId};

use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

use super::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SMALL_TEMP_DIFF_C,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerBodyInput, PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirCalcCoolingEconomizerConditionInput,
    PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, advance_cooling_economizer_body_state,
    advance_cooling_economizer_condition_state,
    cooling_economizer_guard::{
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
    },
};

mod gate_and_assignment_tests;
mod ieee_tests;
mod skip_tests;
mod source_order_tests;

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);
const ZONE: ZoneId = ZoneId(3);

fn characterize(
    input: PurchasedAirCalcCoolingEconomizerBodyInput,
) -> (
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
) {
    let mut state = PurchasedAirCalcCoolingEconomizerBodyRuntimeState::new(SYSTEM);
    let snapshot = advance_cooling_economizer_body_state(
        &mut state,
        body_predecessor(PredecessorRoute::BodyEntered),
        input,
    );
    (snapshot, state)
}

fn base_input() -> PurchasedAirCalcCoolingEconomizerBodyInput {
    PurchasedAirCalcCoolingEconomizerBodyInput {
        zone_humidity_ratio: 0.008,
        outdoor_air_temperature_c: 17.0,
        zone_temperature_c: 20.0,
        zone_cooling_setpoint_load_w: -1.0,
        cooling_limit: IdealLoadsLimit::NoLimit,
        maximum_cooling_air_mass_flow_rate_kg_per_s: f64::NAN,
        outdoor_air_mass_flow_rate_kg_per_s: -1.0,
        system_time_step_hours: 0.25,
    }
}

fn poison_input() -> PurchasedAirCalcCoolingEconomizerBodyInput {
    PurchasedAirCalcCoolingEconomizerBodyInput {
        zone_humidity_ratio: f64::NAN,
        outdoor_air_temperature_c: f64::NAN,
        zone_temperature_c: f64::NAN,
        zone_cooling_setpoint_load_w: f64::NAN,
        cooling_limit: IdealLoadsLimit::LimitFlowRateAndCapacity,
        maximum_cooling_air_mass_flow_rate_kg_per_s: f64::NAN,
        outdoor_air_mass_flow_rate_kg_per_s: f64::NAN,
        system_time_step_hours: f64::NAN,
    }
}

#[derive(Clone, Copy)]
enum PredecessorRoute {
    UnitOff,
    NonCooling,
    MaximumFlowSibling,
    NoEconomizer,
    ConditionFallthrough,
    BodyEntered,
}

fn body_predecessor(route: PredecessorRoute) -> PurchasedAirCalcCoolingEconomizerConditionSnapshot {
    let guard = match route {
        PredecessorRoute::UnitOff
        | PredecessorRoute::NonCooling
        | PredecessorRoute::MaximumFlowSibling => skipped_guard(route),
        PredecessorRoute::NoEconomizer => reached_guard(OutdoorAirEconomizerType::NoEconomizer),
        PredecessorRoute::ConditionFallthrough | PredecessorRoute::BodyEntered => {
            reached_guard(OutdoorAirEconomizerType::DifferentialDryBulb)
        }
    };
    let (outdoor_air_temperature_c, recirculation_air_temperature_c) =
        if matches!(route, PredecessorRoute::BodyEntered) {
            (0.0, 1.0)
        } else {
            (1.0, 1.0)
        };
    let mut state = PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(SYSTEM);
    advance_cooling_economizer_condition_state(
        &mut state,
        guard,
        PurchasedAirCalcCoolingEconomizerConditionInput {
            economizer_type: if matches!(route, PredecessorRoute::NoEconomizer) {
                OutdoorAirEconomizerType::NoEconomizer
            } else {
                OutdoorAirEconomizerType::DifferentialDryBulb
            },
            outdoor_air_temperature_c,
            recirculation_air_temperature_c,
            outdoor_air_enthalpy_j_per_kg: f64::NAN,
            recirculation_air_enthalpy_j_per_kg: f64::NAN,
        },
    )
}

fn reached_guard(
    economizer_type: OutdoorAirEconomizerType,
) -> PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    let entered = economizer_type != OutdoorAirEconomizerType::NoEconomizer;
    PurchasedAirCalcCoolingEconomizerGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        system: SYSTEM,
        parent_call_ordinal: 1,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
        controlled_zone: ZONE,
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_maximum_cooling_flow_body_entered: false,
        predecessor_active_guard_false_economizer_fallthrough: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        maximum_cooling_flow_body_sibling_skipped: false,
        economizer_guard_evaluated: true,
        economizer_type_read: true,
        economizer_type: Some(economizer_type),
        no_economizer_comparison_evaluated: true,
        economizer_not_no_economizer: Some(entered),
        economizer_body_entered: entered,
        no_economizer_fallthrough: !entered,
    }
}

fn skipped_guard(route: PredecessorRoute) -> PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    let unit_off = matches!(route, PredecessorRoute::UnitOff);
    let non_cooling = matches!(route, PredecessorRoute::NonCooling);
    let sibling = matches!(route, PredecessorRoute::MaximumFlowSibling);
    PurchasedAirCalcCoolingEconomizerGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        system: SYSTEM,
        parent_call_ordinal: 1,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
        controlled_zone: ZONE,
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: sibling,
        predecessor_maximum_cooling_flow_body_entered: sibling,
        predecessor_active_guard_false_economizer_fallthrough: false,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        maximum_cooling_flow_body_sibling_skipped: sibling,
        economizer_guard_evaluated: false,
        economizer_type_read: false,
        economizer_type: None,
        no_economizer_comparison_evaluated: false,
        economizer_not_no_economizer: None,
        economizer_body_entered: false,
        no_economizer_fallthrough: false,
    }
}

fn assert_bits(actual: Option<f64>, expected: f64) {
    assert_eq!(
        actual.expect("source site value").to_bits(),
        expected.to_bits()
    );
}
