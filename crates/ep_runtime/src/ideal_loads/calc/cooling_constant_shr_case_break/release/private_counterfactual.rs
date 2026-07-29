//! Canonical private-Humidistat CP357 reconstruction.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingConstantShrCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds the exact private Humidistat CP357 counterfactual from the retained
/// direct `None` release.
///
/// CP357 is evidence-only. The route-independent call identity is retained
/// bit-exactly while the selector and the predecessor/local switch one-hot
/// fields are transformed to the unique Humidistat-selected route.
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit.calc_cooling_constant_shr_case_break.latest?;
    let witness = runtime.cooling_constant_shr_case_break_latest_witness(system.id)?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_exact(retained, direct)
        || !snapshots_match_exact(witness, direct)
        || !super::completed_direct_cooling_constant_shr_case_break_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let mut counterfactual = direct;
    counterfactual.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::Humidistat);
    counterfactual.predecessor_dehumidification_control_none_case_completed_skip = false;
    counterfactual
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed =
        false;
    counterfactual.predecessor_dehumidification_control_humidistat_case_selected_skip = true;
    counterfactual
        .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =
        false;
    counterfactual.dehumidification_control_none_case_completed_skip = false;
    counterfactual.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break =
        false;
    counterfactual.dehumidification_control_humidistat_case_selected_skip = true;
    counterfactual.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =
        false;

    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlHumidistatCaseSelectedSkip)
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Proves that a supplied CP357 witness is the bit-exact canonical private
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
