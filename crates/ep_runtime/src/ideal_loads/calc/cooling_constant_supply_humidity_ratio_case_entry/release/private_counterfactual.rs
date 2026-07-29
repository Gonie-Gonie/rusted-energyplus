//! Canonical private CP364 constant-supply-humidity-ratio reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState as State,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Snapshot,
    advance_cooling_constant_supply_humidity_ratio_case_entry_state as advance,
};
use super::prefix_validation::{
    case_entry_links_to_predecessor, private_constant_supply_predecessor_links_to_direct_release,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_humidistat_case_break::private_constant_supply_humidity_ratio_counterfactual_from_direct_release as cp363_private_constant_supply_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds private CP364 from CP363's canonical private constant-supply
/// selected-skip predecessor, entering the sole CP364 source construct.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_constant_supply_humidity_ratio_case_entry_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    if !direct_release_is_retained_and_complete(runtime, unit, system, direct) {
        return None;
    }

    let direct_cp363 = unit.calc_cooling_humidistat_case_break.latest?;
    let private_cp363 = cp363_private_constant_supply_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp363,
    )?;
    if !private_constant_supply_predecessor_links_to_direct_release(
        runtime,
        unit,
        system,
        direct_cp363,
        private_cp363,
    ) {
        return None;
    }

    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp363)?;
    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseEntered)
        && case_entry_links_to_predecessor(counterfactual, private_cp363)
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Proves that a supplied private CP364 witness is the canonical line-2234
/// counterfactual of the retained direct release.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_constant_supply_humidity_ratio_case_entry_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_constant_supply_humidity_ratio_case_entry_counterfactual_from_direct_release(
        runtime, unit, system, direct,
    )
    .is_some_and(|expected| snapshots_match_exact(expected, counterfactual))
}

fn direct_release_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_constant_supply_humidity_ratio_case_entry
        .latest
    else {
        return false;
    };
    let Some(witness) =
        runtime.cooling_constant_supply_humidity_ratio_case_entry_latest_witness(system.id)
    else {
        return false;
    };
    system.id == direct.system
        && unit.system == system.id
        && snapshot_route(direct) == Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        && snapshots_match_exact(retained, direct)
        && snapshots_match_exact(witness, direct)
        && super::completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
}

fn route_independent_identity_matches(direct: Snapshot, counterfactual: Snapshot) -> bool {
    direct.source == counterfactual.source
        && direct.first_excluded_source == counterfactual.first_excluded_source
        && direct.source_order == counterfactual.source_order
        && direct.system == counterfactual.system
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
