use crate::ideal_loads::PurchasedAirCalcCoolingDehumidificationFlowSnapshot;

pub(super) fn humidification_flow_links_to_dehumidification_flow(
    predecessor: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) -> bool {
    predecessor.parent_call_ordinal > 0
        && predecessor.cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && usize::from(predecessor.unit_off_skipped)
            + usize::from(predecessor.non_cooling_skipped)
            + usize::from(predecessor.cooling_body_entered)
            == 1
}
