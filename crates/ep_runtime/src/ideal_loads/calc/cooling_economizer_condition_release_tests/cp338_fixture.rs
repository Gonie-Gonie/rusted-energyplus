//! Capacity-limited CP315 fixture used by the CP338 release tests.

use super::*;

pub(in crate::ideal_loads::calc) fn release_fixture_with_cooling_demand_availability_and_capacity_limit(
    cooling_demand_w: f64,
    overall_availability: f64,
) -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    let mut system = test_system();
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_total_cooling_capacity_w = Some(ep_model::AutosizeOrNumber::Value(10_000.0));
    let mut runtime = PurchasedAirRuntimeState::default();
    initialize_fixture_call(&mut runtime, &system, true, overall_availability);
    let guard = advance_fixture_prefix(
        &mut runtime,
        &system,
        cooling_demand_w,
        overall_availability,
    );
    (runtime, system, guard)
}
