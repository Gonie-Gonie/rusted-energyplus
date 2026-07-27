//! Exact CP315-to-CP316 predecessor-link validation.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
};

pub(super) fn economizer_condition_links_to_guard(
    condition: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    guard: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    condition.system == guard.system
        && condition.parent_call_ordinal == guard.parent_call_ordinal
        && condition.controlled_zone == guard.controlled_zone
        && condition.unit_body_entered == guard.unit_body_entered
        && condition.predecessor_cooling_body_entered == guard.predecessor_cooling_body_entered
        && condition.predecessor_maximum_cooling_flow_body_entered
            == guard.predecessor_maximum_cooling_flow_body_entered
        && condition.predecessor_active_guard_false_economizer_fallthrough
            == guard.predecessor_active_guard_false_economizer_fallthrough
        && condition.predecessor_economizer_guard_evaluated == guard.economizer_guard_evaluated
        && condition.predecessor_economizer_body_entered == guard.economizer_body_entered
        && condition.predecessor_no_economizer_fallthrough == guard.no_economizer_fallthrough
        && condition.unit_off_skipped == guard.unit_off_skipped
        && condition.non_cooling_skipped == guard.non_cooling_skipped
        && condition.maximum_cooling_flow_body_sibling_skipped
            == guard.maximum_cooling_flow_body_sibling_skipped
        && condition.no_economizer_outer_guard_fallthrough_skipped
            == guard.no_economizer_fallthrough
        && condition.economizer_condition_evaluated == guard.economizer_body_entered
}
