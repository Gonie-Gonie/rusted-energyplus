//! Exact retained initialization prerequisites for the CP317 release.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

pub(super) fn initialization_state_is_exact_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
) -> bool {
    super::super::super::cooling_economizer_condition::exact_direct_initialization_is_consistent(
        runtime, unit, system,
    )
}
