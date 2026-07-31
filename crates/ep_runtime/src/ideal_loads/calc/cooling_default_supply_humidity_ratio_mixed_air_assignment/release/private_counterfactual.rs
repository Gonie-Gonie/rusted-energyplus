//! Canonical private CP367 default-assignment CSH-skip reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot as Snapshot,
    advance_cooling_default_supply_humidity_ratio_mixed_air_assignment_state as advance,
};
use super::prefix_validation::{active_lineage_is_exact, assignment_links_to_predecessor};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_break::private_constant_supply_humidity_ratio_case_break_counterfactual_from_direct_release as cp366_private_csh_case_break_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds private CP367 by recursively rebuilding canonical private CP366
/// and recording only the numeric-free default-assignment skip.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
        .latest?;
    let witness =
        runtime.cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(system.id)?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_exact(retained, direct)
        || !snapshots_match_exact(witness, direct)
        || !super::completed_direct_cooling_default_supply_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp366 = unit
        .calc_cooling_constant_supply_humidity_ratio_case_break
        .latest?;
    let private_cp366 = cp366_private_csh_case_break_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp366,
    )?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp366)?;
    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip)
        && assignment_links_to_predecessor(counterfactual, private_cp366)
        && active_lineage_is_exact(runtime, unit, system, private_cp366, counterfactual)
        && !counterfactual
            .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Proves that a supplied private CP367 witness is the exact canonical
/// numeric-free characterization of the retained direct release.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_from_direct_release(
        runtime, unit, system, direct,
    )
    .is_some_and(|expected| snapshots_match_exact(expected, counterfactual))
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
