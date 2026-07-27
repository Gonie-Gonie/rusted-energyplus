//! Exact retained CP310-through-CP315 release-prefix validation for CP317.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, PurchasedAirUnitRuntimeState,
};

pub(super) fn completed_direct_prefix_through_economizer_guard_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) -> bool {
    super::super::super::cooling_economizer_condition::
        completed_direct_prefix_through_economizer_guard_is_consistent(
            unit,
            system,
            predecessor,
        )
}
