//! Canonical private-active CP356 reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot as Snapshot,
    advance_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_state as advance,
};
use super::prefix_validation::active_operands_from_retained_owners;
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_minimum_limit::private_active_counterfactual_from_direct_release as cp355_private_active_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds the exact private constant-SHR CP356 counterfactual from the
/// recursively validated private CP355 predecessor and same-call owners.
pub(in crate::ideal_loads::calc) fn private_active_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
        .latest?;
    let witness = runtime
        .cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witness(system.id)?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp355 = unit
        .calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit
        .latest?;
    let private_cp355 = cp355_private_active_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp355,
    )?;
    let operands = active_operands_from_retained_owners(runtime, unit, system, private_cp355)?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp355, Some(operands))?;
    route_independent_identity_matches(direct, counterfactual).then_some(counterfactual)
}

/// Proves a supplied CP356 witness is the bit-exact canonical private-active
/// counterfactual of the retained direct `None` release.
pub(in crate::ideal_loads::calc) fn private_active_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_active_counterfactual_from_direct_release(runtime, unit, system, direct)
        .is_some_and(|expected| snapshots_match_bit_exact(expected, counterfactual))
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
