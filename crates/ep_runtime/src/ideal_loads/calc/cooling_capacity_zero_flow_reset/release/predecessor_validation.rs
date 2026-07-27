use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirCalcCoolingSensibleFlowSnapshot,
};

pub(super) fn predecessor_chain_and_candidates_are_consistent(
    sensible: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    dehumidification: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    humidification: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) -> bool {
    sensible.system == dehumidification.system
        && sensible.system == humidification.system
        && sensible.parent_call_ordinal == dehumidification.parent_call_ordinal
        && sensible.parent_call_ordinal == humidification.parent_call_ordinal
        && sensible.controlled_zone == dehumidification.controlled_zone
        && sensible.controlled_zone == humidification.controlled_zone
        && sensible.unit_off_skipped == dehumidification.unit_off_skipped
        && sensible.unit_off_skipped == humidification.unit_off_skipped
        && sensible.non_cooling_skipped == dehumidification.non_cooling_skipped
        && sensible.non_cooling_skipped == humidification.non_cooling_skipped
        && sensible.cooling_body_entered == dehumidification.cooling_body_entered
        && sensible.cooling_body_entered == humidification.cooling_body_entered
        && dehumidification.predecessor_cooling_body_entered == sensible.cooling_body_entered
        && humidification.predecessor_cooling_body_entered == dehumidification.cooling_body_entered
}
