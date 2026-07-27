//! Pure CP316 cooling economizer inner-condition transition.

use ep_model::OutdoorAirEconomizerType;

use super::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerConditionInput,
    PurchasedAirCalcCoolingEconomizerConditionRetainedRoute,
    PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingEconomizerGuardSnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_economizer_condition_state(
    state: &mut PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    input: PurchasedAirCalcCoolingEconomizerConditionInput,
) -> PurchasedAirCalcCoolingEconomizerConditionSnapshot {
    state.transition_count += 1;

    let economizer_condition_evaluated = predecessor.economizer_body_entered;
    let unit_off_skipped = !economizer_condition_evaluated && predecessor.unit_off_skipped;
    let non_cooling_skipped = !economizer_condition_evaluated && predecessor.non_cooling_skipped;
    let maximum_cooling_flow_body_sibling_skipped =
        !economizer_condition_evaluated && predecessor.maximum_cooling_flow_body_sibling_skipped;
    let no_economizer_outer_guard_fallthrough_skipped =
        !economizer_condition_evaluated && predecessor.no_economizer_fallthrough;
    let retained_route = if economizer_condition_evaluated {
        PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::Evaluated
    } else if unit_off_skipped {
        PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::UnitOff
    } else if non_cooling_skipped {
        PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::NonCooling
    } else if maximum_cooling_flow_body_sibling_skipped {
        PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::MaximumCoolingFlowBodySibling
    } else {
        PurchasedAirCalcCoolingEconomizerConditionRetainedRoute::NoEconomizerOuterGuardFallthrough
    };

    let dry_bulb_economizer_type = economizer_condition_evaluated.then_some(input.economizer_type);
    let differential_dry_bulb_selector_matched = dry_bulb_economizer_type
        .map(|value| value == OutdoorAirEconomizerType::DifferentialDryBulb);
    let dry_bulb_operands_read = differential_dry_bulb_selector_matched == Some(true);
    let (outdoor_air_temperature_c, recirculation_air_temperature_c) = if dry_bulb_operands_read {
        (
            Some(input.outdoor_air_temperature_c),
            Some(input.recirculation_air_temperature_c),
        )
    } else {
        (None, None)
    };
    let outdoor_air_temperature_below_recirculation_temperature = if dry_bulb_operands_read {
        Some(input.outdoor_air_temperature_c < input.recirculation_air_temperature_c)
    } else {
        None
    };

    let differential_enthalpy_selector_comparison_evaluated = economizer_condition_evaluated
        && outdoor_air_temperature_below_recirculation_temperature != Some(true);
    let differential_enthalpy_economizer_type =
        differential_enthalpy_selector_comparison_evaluated.then_some(input.economizer_type);
    let differential_enthalpy_selector_matched = differential_enthalpy_economizer_type
        .map(|value| value == OutdoorAirEconomizerType::DifferentialEnthalpy);
    let enthalpy_operands_read = differential_enthalpy_selector_matched == Some(true);
    let (outdoor_air_enthalpy_j_per_kg, recirculation_air_enthalpy_j_per_kg) =
        if enthalpy_operands_read {
            (
                Some(input.outdoor_air_enthalpy_j_per_kg),
                Some(input.recirculation_air_enthalpy_j_per_kg),
            )
        } else {
            (None, None)
        };
    let outdoor_air_enthalpy_below_recirculation_enthalpy = if enthalpy_operands_read {
        Some(input.outdoor_air_enthalpy_j_per_kg < input.recirculation_air_enthalpy_j_per_kg)
    } else {
        None
    };

    let economizer_condition_satisfied = economizer_condition_evaluated.then_some(
        outdoor_air_temperature_below_recirculation_temperature == Some(true)
            || outdoor_air_enthalpy_below_recirculation_enthalpy == Some(true),
    );
    let economizer_calculation_body_entered = economizer_condition_satisfied == Some(true);
    let economizer_condition_fallthrough = economizer_condition_satisfied == Some(false);

    if economizer_condition_evaluated {
        state.condition_evaluation_count += 1;
        state.differential_dry_bulb_economizer_type_read_count += 1;
        state.differential_dry_bulb_selector_comparison_count += 1;
        if differential_dry_bulb_selector_matched == Some(true) {
            state.differential_dry_bulb_selector_match_count += 1;
        }
        if dry_bulb_operands_read {
            state.outdoor_air_temperature_read_count += 1;
            state.recirculation_air_temperature_read_count += 1;
            state.dry_bulb_temperature_comparison_count += 1;
        }
        if outdoor_air_temperature_below_recirculation_temperature == Some(true) {
            state.dry_bulb_temperature_comparison_satisfied_count += 1;
        }
        if differential_enthalpy_selector_comparison_evaluated {
            state.differential_enthalpy_economizer_type_read_count += 1;
            state.differential_enthalpy_selector_comparison_count += 1;
        }
        if differential_enthalpy_selector_matched == Some(true) {
            state.differential_enthalpy_selector_match_count += 1;
        }
        if enthalpy_operands_read {
            state.outdoor_air_enthalpy_read_count += 1;
            state.recirculation_air_enthalpy_read_count += 1;
            state.enthalpy_comparison_count += 1;
        }
        if outdoor_air_enthalpy_below_recirculation_enthalpy == Some(true) {
            state.enthalpy_comparison_satisfied_count += 1;
        }
        if economizer_calculation_body_entered {
            state.economizer_calculation_body_entry_count += 1;
        } else {
            state.economizer_condition_fallthrough_count += 1;
        }
    } else if unit_off_skipped {
        state.unit_off_skip_count += 1;
    } else if non_cooling_skipped {
        state.non_cooling_skip_count += 1;
    } else if maximum_cooling_flow_body_sibling_skipped {
        state.maximum_cooling_flow_body_sibling_skip_count += 1;
    } else if no_economizer_outer_guard_fallthrough_skipped {
        state.no_economizer_outer_guard_fallthrough_skip_count += 1;
    }

    let snapshot = PurchasedAirCalcCoolingEconomizerConditionSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_entered: predecessor
            .predecessor_maximum_cooling_flow_body_entered,
        predecessor_active_guard_false_economizer_fallthrough: predecessor
            .predecessor_active_guard_false_economizer_fallthrough,
        predecessor_economizer_guard_evaluated: predecessor.economizer_guard_evaluated,
        predecessor_economizer_body_entered: predecessor.economizer_body_entered,
        predecessor_no_economizer_fallthrough: predecessor.no_economizer_fallthrough,
        unit_off_skipped,
        non_cooling_skipped,
        maximum_cooling_flow_body_sibling_skipped,
        no_economizer_outer_guard_fallthrough_skipped,
        economizer_condition_evaluated,
        differential_dry_bulb_economizer_type_read: economizer_condition_evaluated,
        differential_dry_bulb_economizer_type: dry_bulb_economizer_type,
        differential_dry_bulb_selector_comparison_evaluated: economizer_condition_evaluated,
        differential_dry_bulb_selector_matched,
        outdoor_air_temperature_read: dry_bulb_operands_read,
        outdoor_air_temperature_c,
        recirculation_air_temperature_read: dry_bulb_operands_read,
        recirculation_air_temperature_c,
        dry_bulb_temperature_comparison_evaluated: dry_bulb_operands_read,
        outdoor_air_temperature_below_recirculation_temperature,
        differential_enthalpy_economizer_type_read:
            differential_enthalpy_selector_comparison_evaluated,
        differential_enthalpy_economizer_type,
        differential_enthalpy_selector_comparison_evaluated,
        differential_enthalpy_selector_matched,
        outdoor_air_enthalpy_read: enthalpy_operands_read,
        outdoor_air_enthalpy_j_per_kg,
        recirculation_air_enthalpy_read: enthalpy_operands_read,
        recirculation_air_enthalpy_j_per_kg,
        enthalpy_comparison_evaluated: enthalpy_operands_read,
        outdoor_air_enthalpy_below_recirculation_enthalpy,
        economizer_condition_satisfied,
        economizer_calculation_body_entered,
        economizer_condition_fallthrough,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(retained_route);
    state.latest_transition_ordinal = Some(state.transition_count);
    snapshot
}
