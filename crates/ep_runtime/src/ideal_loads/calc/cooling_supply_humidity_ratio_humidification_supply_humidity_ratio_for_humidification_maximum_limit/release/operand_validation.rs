//! Selected typed-system right-operand validation for private CP374.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn maximum_heating_supply_air_humidity_ratio_from_selected_typed_owner(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<f64> {
    if system.id != predecessor.system || unit.system != system.id {
        return None;
    }
    let maximum = system.maximum_heating_supply_air_humidity_ratio;
    // Compiler range/default validation remains upstream. This is only the
    // canonical selected typed-owner finite gate.
    maximum.is_finite().then_some(maximum)
}
