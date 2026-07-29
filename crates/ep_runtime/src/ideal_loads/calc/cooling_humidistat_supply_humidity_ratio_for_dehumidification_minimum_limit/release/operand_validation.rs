//! Selected typed-system right-operand validation for private CP361.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Predecessor,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<f64> {
    if system.id != predecessor.system || unit.system != system.id {
        return None;
    }
    let minimum = system.minimum_cooling_supply_air_humidity_ratio;
    // Compiler range/default validation remains upstream. This is only the
    // finite owner gate established by CP355.
    minimum.is_finite().then_some(minimum)
}
