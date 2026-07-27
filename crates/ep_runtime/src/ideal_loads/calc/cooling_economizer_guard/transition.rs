//! Pure CP315 cooling economizer outer-guard transition.

use ep_model::OutdoorAirEconomizerType;

use super::{
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingOaMaxFlowBodySnapshot;

pub(in crate::ideal_loads::calc) fn advance_cooling_economizer_guard_state(
    state: &mut PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    economizer_type: OutdoorAirEconomizerType,
) -> PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    state.transition_count += 1;

    let economizer_guard_evaluated = predecessor.active_guard_false_economizer_fallthrough;
    let unit_off_skipped = !economizer_guard_evaluated && predecessor.unit_off_skipped;
    let non_cooling_skipped = !economizer_guard_evaluated && predecessor.non_cooling_skipped;
    let maximum_cooling_flow_body_sibling_skipped =
        !economizer_guard_evaluated && predecessor.predecessor_maximum_cooling_flow_body_entered;
    let economizer_type_value = economizer_guard_evaluated.then_some(economizer_type);
    let economizer_not_no_economizer =
        economizer_type_value.map(|value| value != OutdoorAirEconomizerType::NoEconomizer);
    let economizer_body_entered = economizer_not_no_economizer == Some(true);
    let no_economizer_fallthrough = economizer_not_no_economizer == Some(false);

    if economizer_guard_evaluated {
        state.guard_evaluation_count += 1;
        state.economizer_type_read_count += 1;
        state.no_economizer_comparison_count += 1;
        if economizer_body_entered {
            state.economizer_body_entry_count += 1;
        } else {
            state.no_economizer_fallthrough_count += 1;
        }
    } else if unit_off_skipped {
        state.unit_off_skip_count += 1;
    } else if non_cooling_skipped {
        state.non_cooling_skip_count += 1;
    } else if maximum_cooling_flow_body_sibling_skipped {
        state.maximum_cooling_flow_body_sibling_skip_count += 1;
    }

    let snapshot = PurchasedAirCalcCoolingEconomizerGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_entered: predecessor
            .predecessor_maximum_cooling_flow_body_entered,
        predecessor_active_guard_false_economizer_fallthrough: predecessor
            .active_guard_false_economizer_fallthrough,
        unit_off_skipped,
        non_cooling_skipped,
        maximum_cooling_flow_body_sibling_skipped,
        economizer_guard_evaluated,
        economizer_type_read: economizer_guard_evaluated,
        economizer_type: economizer_type_value,
        no_economizer_comparison_evaluated: economizer_guard_evaluated,
        economizer_not_no_economizer,
        economizer_body_entered,
        no_economizer_fallthrough,
    };
    state.latest = Some(snapshot);
    snapshot
}
