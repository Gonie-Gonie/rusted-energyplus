//! Canonical private CP365 constant-supply-humidity-ratio assignment reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    advance_cooling_constant_supply_humidity_ratio_assignment_state as advance,
};
use super::prefix_validation::{
    active_lineage_is_exact, assignment_links_to_predecessor,
    minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner,
};
use super::snapshot_validation::{option_bits_match, snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_entry::private_constant_supply_humidity_ratio_case_entry_counterfactual_from_direct_release as cp364_private_constant_supply_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds private CP365 from canonical CP364 constant-supply case entry and
/// the selected typed minimum-cooling supply-humidity-ratio owner.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_constant_supply_humidity_ratio_assignment
        .latest?;
    let witness =
        runtime.cooling_constant_supply_humidity_ratio_assignment_latest_witness(system.id)?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_constant_supply_humidity_ratio_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp364 = unit
        .calc_cooling_constant_supply_humidity_ratio_case_entry
        .latest?;
    let private_cp364 = cp364_private_constant_supply_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp364,
    )?;
    let minimum = minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner(
        runtime,
        unit,
        system,
        private_cp364,
    )?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp364, Some(minimum))?;
    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlConstantSupplyHumidityRatioAssigned)
        && assignment_links_to_predecessor(counterfactual, private_cp364)
        && active_lineage_is_exact(runtime, unit, system, private_cp364, counterfactual)
        && route_independent_identity_matches(direct, counterfactual)
        && option_bits_match(
            counterfactual.minimum_cooling_supply_air_humidity_ratio,
            Some(minimum),
        )
        && option_bits_match(counterfactual.assigned_supply_humidity_ratio, Some(minimum))
        && option_bits_match(
            counterfactual.resulting_supply_humidity_ratio,
            Some(minimum),
        ))
    .then_some(counterfactual)
}

/// Proves that a supplied private CP365 witness is the bit-exact canonical
/// characterization of the retained direct release.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_constant_supply_humidity_ratio_assignment_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_constant_supply_humidity_ratio_assignment_counterfactual_from_direct_release(
        runtime, unit, system, direct,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, counterfactual))
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
