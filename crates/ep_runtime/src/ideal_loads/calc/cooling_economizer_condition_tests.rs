use ep_model::{IdealLoadsAirSystemId, OutdoorAirEconomizerType, ZoneId};

use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
};

use super::{
    cooling_economizer_condition::{
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
        PurchasedAirCalcCoolingEconomizerConditionInput,
        advance_cooling_economizer_condition_state,
    },
    cooling_economizer_guard::{
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
        PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    },
};

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);
const ZONE: ZoneId = ZoneId(3);

#[test]
fn compound_condition_preserves_selector_and_or_short_circuit_order() {
    let dry_bulb = characterize(
        OutdoorAirEconomizerType::DifferentialDryBulb,
        10.0,
        20.0,
        f64::NAN,
        f64::NAN,
    );
    assert!(dry_bulb.differential_dry_bulb_economizer_type_read);
    assert_eq!(dry_bulb.differential_dry_bulb_selector_matched, Some(true));
    assert!(dry_bulb.outdoor_air_temperature_read);
    assert_eq!(
        dry_bulb.outdoor_air_temperature_below_recirculation_temperature,
        Some(true)
    );
    assert!(!dry_bulb.differential_enthalpy_economizer_type_read);
    assert!(!dry_bulb.outdoor_air_enthalpy_read);
    assert!(dry_bulb.economizer_calculation_body_entered);

    let dry_bulb_false = characterize(
        OutdoorAirEconomizerType::DifferentialDryBulb,
        20.0,
        10.0,
        1.0,
        2.0,
    );
    assert_eq!(
        dry_bulb_false.outdoor_air_temperature_below_recirculation_temperature,
        Some(false)
    );
    assert!(dry_bulb_false.differential_enthalpy_economizer_type_read);
    assert_eq!(
        dry_bulb_false.differential_enthalpy_selector_matched,
        Some(false)
    );
    assert!(!dry_bulb_false.outdoor_air_enthalpy_read);
    assert!(dry_bulb_false.economizer_condition_fallthrough);

    let enthalpy = characterize(
        OutdoorAirEconomizerType::DifferentialEnthalpy,
        f64::NAN,
        f64::NAN,
        10_000.0,
        20_000.0,
    );
    assert_eq!(enthalpy.differential_dry_bulb_selector_matched, Some(false));
    assert!(!enthalpy.outdoor_air_temperature_read);
    assert!(enthalpy.differential_enthalpy_economizer_type_read);
    assert_eq!(enthalpy.differential_enthalpy_selector_matched, Some(true));
    assert!(enthalpy.outdoor_air_enthalpy_read);
    assert_eq!(
        enthalpy.outdoor_air_enthalpy_below_recirculation_enthalpy,
        Some(true)
    );
    assert!(enthalpy.economizer_calculation_body_entered);
}

#[test]
fn raw_strict_less_than_preserves_nan_signed_zero_and_infinity() {
    let cases = [
        (f64::NAN, 0.0, false),
        (0.0, f64::NAN, false),
        (-0.0, 0.0, false),
        (0.0, -0.0, false),
        (f64::NEG_INFINITY, 0.0, true),
        (0.0, f64::INFINITY, true),
        (f64::INFINITY, 0.0, false),
    ];
    for economizer_type in [
        OutdoorAirEconomizerType::DifferentialDryBulb,
        OutdoorAirEconomizerType::DifferentialEnthalpy,
    ] {
        for (left, right, expected) in cases {
            let snapshot = if economizer_type == OutdoorAirEconomizerType::DifferentialDryBulb {
                characterize(economizer_type, left, right, f64::NAN, f64::NAN)
            } else {
                characterize(economizer_type, f64::NAN, f64::NAN, left, right)
            };
            let (stored_left, stored_right, comparison) =
                if economizer_type == OutdoorAirEconomizerType::DifferentialDryBulb {
                    (
                        snapshot.outdoor_air_temperature_c,
                        snapshot.recirculation_air_temperature_c,
                        snapshot.outdoor_air_temperature_below_recirculation_temperature,
                    )
                } else {
                    (
                        snapshot.outdoor_air_enthalpy_j_per_kg,
                        snapshot.recirculation_air_enthalpy_j_per_kg,
                        snapshot.outdoor_air_enthalpy_below_recirculation_enthalpy,
                    )
                };
            assert_eq!(stored_left.map(f64::to_bits), Some(left.to_bits()));
            assert_eq!(stored_right.map(f64::to_bits), Some(right.to_bits()));
            assert_eq!(comparison, Some(expected), "{left:?} < {right:?}");
        }
    }
}

#[test]
fn unit_off_non_cooling_sibling_and_outer_false_are_four_complete_skips() {
    let predecessors = [
        skipped_predecessor(SkipRoute::UnitOff),
        skipped_predecessor(SkipRoute::NonCooling),
        skipped_predecessor(SkipRoute::MaximumFlowSibling),
        reached_predecessor(OutdoorAirEconomizerType::NoEconomizer),
    ];
    let mut state = PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(SYSTEM);
    let snapshots: [PurchasedAirCalcCoolingEconomizerConditionSnapshot; 4] =
        std::array::from_fn(|index| {
            advance_cooling_economizer_condition_state(
                &mut state,
                predecessors[index],
                poison_input(),
            )
        });
    assert!(snapshots[0].unit_off_skipped);
    assert!(snapshots[1].non_cooling_skipped);
    assert!(snapshots[2].maximum_cooling_flow_body_sibling_skipped);
    assert!(snapshots[3].no_economizer_outer_guard_fallthrough_skipped);
    for snapshot in snapshots {
        assert_condition_sites_skipped(snapshot);
    }
    assert_eq!(state.transition_count, 4);
    assert_eq!(state.condition_evaluation_count, 0);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.maximum_cooling_flow_body_sibling_skip_count, 1);
    assert_eq!(state.no_economizer_outer_guard_fallthrough_skip_count, 1);
}

fn characterize(
    economizer_type: OutdoorAirEconomizerType,
    outdoor_air_temperature_c: f64,
    recirculation_air_temperature_c: f64,
    outdoor_air_enthalpy_j_per_kg: f64,
    recirculation_air_enthalpy_j_per_kg: f64,
) -> PurchasedAirCalcCoolingEconomizerConditionSnapshot {
    let mut state = PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(SYSTEM);
    advance_cooling_economizer_condition_state(
        &mut state,
        reached_predecessor(economizer_type),
        PurchasedAirCalcCoolingEconomizerConditionInput {
            economizer_type,
            outdoor_air_temperature_c,
            recirculation_air_temperature_c,
            outdoor_air_enthalpy_j_per_kg,
            recirculation_air_enthalpy_j_per_kg,
        },
    )
}

fn reached_predecessor(
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

#[derive(Clone, Copy)]
enum SkipRoute {
    UnitOff,
    NonCooling,
    MaximumFlowSibling,
}

fn skipped_predecessor(route: SkipRoute) -> PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    let unit_off = matches!(route, SkipRoute::UnitOff);
    let non_cooling = matches!(route, SkipRoute::NonCooling);
    let sibling = matches!(route, SkipRoute::MaximumFlowSibling);
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

fn poison_input() -> PurchasedAirCalcCoolingEconomizerConditionInput {
    PurchasedAirCalcCoolingEconomizerConditionInput {
        economizer_type: OutdoorAirEconomizerType::DifferentialEnthalpy,
        outdoor_air_temperature_c: f64::NAN,
        recirculation_air_temperature_c: f64::NAN,
        outdoor_air_enthalpy_j_per_kg: f64::NAN,
        recirculation_air_enthalpy_j_per_kg: f64::NAN,
    }
}

fn assert_condition_sites_skipped(snapshot: PurchasedAirCalcCoolingEconomizerConditionSnapshot) {
    assert_eq!(
        snapshot.source,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
    );
    assert_eq!(
        snapshot.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER
    );
    assert!(!snapshot.economizer_condition_evaluated);
    assert!(!snapshot.differential_dry_bulb_economizer_type_read);
    assert!(snapshot.differential_dry_bulb_economizer_type.is_none());
    assert!(!snapshot.outdoor_air_temperature_read);
    assert!(snapshot.outdoor_air_temperature_c.is_none());
    assert!(!snapshot.differential_enthalpy_economizer_type_read);
    assert!(snapshot.differential_enthalpy_economizer_type.is_none());
    assert!(!snapshot.outdoor_air_enthalpy_read);
    assert!(snapshot.outdoor_air_enthalpy_j_per_kg.is_none());
    assert!(snapshot.economizer_condition_satisfied.is_none());
    assert!(!snapshot.economizer_calculation_body_entered);
    assert!(!snapshot.economizer_condition_fallthrough);
}
