//! CP316-to-CP317 predecessor-link validation for CP318.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
};

pub(super) fn economizer_body_links_to_condition(
    body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    condition: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
) -> bool {
    body.system == condition.system
        && body.parent_call_ordinal == condition.parent_call_ordinal
        && body.controlled_zone == condition.controlled_zone
        && body.unit_body_entered == condition.unit_body_entered
        && body.predecessor_cooling_body_entered == condition.predecessor_cooling_body_entered
        && body.predecessor_maximum_cooling_flow_body_entered
            == condition.predecessor_maximum_cooling_flow_body_entered
        && body.predecessor_active_guard_false_economizer_fallthrough
            == condition.predecessor_active_guard_false_economizer_fallthrough
        && body.predecessor_economizer_guard_evaluated
            == condition.predecessor_economizer_guard_evaluated
        && body.predecessor_economizer_body_entered == condition.predecessor_economizer_body_entered
        && body.predecessor_no_economizer_fallthrough
            == condition.predecessor_no_economizer_fallthrough
        && body.predecessor_economizer_condition_evaluated
            == condition.economizer_condition_evaluated
        && body.predecessor_economizer_condition_satisfied
            == condition.economizer_condition_satisfied
        && body.predecessor_economizer_calculation_body_entered
            == condition.economizer_calculation_body_entered
        && body.unit_off_skipped == condition.unit_off_skipped
        && body.non_cooling_skipped == condition.non_cooling_skipped
        && body.maximum_cooling_flow_body_sibling_skipped
            == condition.maximum_cooling_flow_body_sibling_skipped
        && body.no_economizer_outer_guard_fallthrough_skipped
            == condition.no_economizer_outer_guard_fallthrough_skipped
        && body.economizer_condition_fallthrough_skipped
            == condition.economizer_condition_fallthrough
        && body.economizer_calculation_body_executed
            == condition.economizer_calculation_body_entered
}
