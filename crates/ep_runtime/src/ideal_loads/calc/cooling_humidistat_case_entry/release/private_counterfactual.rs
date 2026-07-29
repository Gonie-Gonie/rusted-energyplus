//! Canonical private-Humidistat CP358 reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingHumidistatCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot as Snapshot,
    advance_cooling_humidistat_case_entry_state as advance,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_constant_shr_case_break::private_humidistat_counterfactual_from_direct_release as cp357_private_humidistat_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds the exact private Humidistat CP358 counterfactual from the
/// recursively validated private CP357 predecessor.
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit.calc_cooling_humidistat_case_entry.latest?;
    let witness = runtime.cooling_humidistat_case_entry_latest_witness(system.id)?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_exact(retained, direct)
        || !snapshots_match_exact(witness, direct)
        || !super::completed_direct_cooling_humidistat_case_entry_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp357 = unit.calc_cooling_constant_shr_case_break.latest?;
    let private_cp357 = cp357_private_humidistat_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp357,
    )?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp357)?;
    (snapshot_route(counterfactual) == Some(Route::DehumidificationControlHumidistatCaseEntered)
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Proves that a supplied CP358 witness is the bit-exact canonical private
/// Humidistat counterfactual of the retained direct `None` release.
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_humidistat_counterfactual_from_direct_release(runtime, unit, system, direct)
        .is_some_and(|expected| snapshots_match_exact(expected, counterfactual))
}

fn route_independent_identity_matches(direct: Snapshot, counterfactual: Snapshot) -> bool {
    direct.system == counterfactual.system
        && direct.parent_call_ordinal == counterfactual.parent_call_ordinal
        && direct.controlled_zone == counterfactual.controlled_zone
        && direct.unit_body_entered == counterfactual.unit_body_entered
        && direct.predecessor_cooling_body_entered
            == counterfactual.predecessor_cooling_body_entered
        && direct.predecessor_no_outdoor_air_fallback_entered
            == counterfactual.predecessor_no_outdoor_air_fallback_entered
        && direct.predecessor_positive_supply_mass_flow_body_entered
            == counterfactual.predecessor_positive_supply_mass_flow_body_entered
        && direct.unit_off_skipped == counterfactual.unit_off_skipped
        && direct.non_cooling_skipped == counterfactual.non_cooling_skipped
        && direct.positive_guard_false_fallthrough_skipped
            == counterfactual.positive_guard_false_fallthrough_skipped
}
