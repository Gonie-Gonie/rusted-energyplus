//! CP317-to-CP318 predecessor-link validation for CP319.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirCalcCoolingSensibleFlowSnapshot,
};

pub(super) fn sensible_flow_links_to_economizer_body(
    flow: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
) -> bool {
    flow.system == body.system
        && flow.parent_call_ordinal == body.parent_call_ordinal
        && flow.controlled_zone == body.controlled_zone
        && flow.unit_body_entered == body.unit_body_entered
        && flow.predecessor_cooling_body_entered == body.predecessor_cooling_body_entered
        && flow.predecessor_maximum_cooling_flow_body_sibling_skipped
            == body.maximum_cooling_flow_body_sibling_skipped
        && flow.predecessor_no_economizer_outer_guard_fallthrough_skipped
            == body.no_economizer_outer_guard_fallthrough_skipped
        && flow.predecessor_economizer_condition_fallthrough_skipped
            == body.economizer_condition_fallthrough_skipped
        && flow.predecessor_economizer_calculation_body_executed
            == body.economizer_calculation_body_executed
        && flow.unit_off_skipped == body.unit_off_skipped
        && flow.non_cooling_skipped == body.non_cooling_skipped
        && flow.cooling_body_entered == body.predecessor_cooling_body_entered
}
