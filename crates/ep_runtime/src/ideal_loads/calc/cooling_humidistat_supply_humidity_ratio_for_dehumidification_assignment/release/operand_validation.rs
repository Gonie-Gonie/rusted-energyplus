//! Same-call CP330 denominator-owner validation for private CP360.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::calc::cooling_supply_mass_flow_positive_guard::{
    completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent,
    cooling_supply_mass_flow_positive_guard_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};

pub(super) fn supply_mass_flow_rate_from_retained_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<f64> {
    if system.id != predecessor.system || unit.system != system.id {
        return None;
    }
    let owner = unit.calc_cooling_supply_mass_flow_positive_guard.latest?;
    let witness = runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id)?;
    let flow = owner.supply_mass_flow_rate_kg_per_s?;
    if owner.system != predecessor.system
        || owner.parent_call_ordinal != predecessor.parent_call_ordinal
        || owner.controlled_zone != predecessor.controlled_zone
        || !owner.positive_supply_mass_flow_body_entered
        || !cooling_supply_mass_flow_positive_guard_snapshots_match_bit_exact(owner, witness)
        || !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(owner)
        || !completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
            runtime,
            unit,
            system,
            owner,
            Some(witness),
        )
        || !matches!(
            flow.partial_cmp(&0.0),
            Some(std::cmp::Ordering::Greater)
        )
    {
        return None;
    }
    // No finite gate: the source guard accepts positive infinity.
    Some(flow)
}
