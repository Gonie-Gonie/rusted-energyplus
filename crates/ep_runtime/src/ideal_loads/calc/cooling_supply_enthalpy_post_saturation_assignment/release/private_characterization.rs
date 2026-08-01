//! Restricted pure CP379 counterfactual characterization.

use super::super::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Snapshot,
    advance_cooling_supply_enthalpy_post_saturation_assignment_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Predecessor;

/// Characterizes a non-public CP379 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_supply_enthalpy_post_saturation_assignment_characterization(
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_enthalpy_post_saturation_assignment_state(
        &mut state,
        predecessor,
        input,
    )
}
