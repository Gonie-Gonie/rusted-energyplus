//! Canonical private CP363 Humidistat and constant-supply reconstructions.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingHumidistatCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot as Snapshot,
    advance_cooling_humidistat_case_break_state as advance,
};
use super::prefix_validation::{
    case_break_links_to_predecessor,
    private_humidistat_predecessor_links_to_direct_release,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit::private_humidistat_counterfactual_from_direct_release as cp362_private_humidistat_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds private Humidistat CP363 from the canonical explicit-parameter
/// CP362 Humidistat predecessor and executes the sole case-break source site.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<Snapshot> {
    if !direct_release_is_retained_and_complete(runtime, unit, system, direct) {
        return None;
    }

    let direct_cp362 = unit
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest?;
    let private_cp362 = cp362_private_humidistat_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp362,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )?;
    if !private_humidistat_predecessor_links_to_direct_release(
        runtime,
        unit,
        system,
        direct_cp362,
        private_cp362,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    ) {
        return None;
    }

    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp362)?;
    (snapshot_route(counterfactual) == Some(Route::DehumidificationControlHumidistatCaseBreak)
        && case_break_links_to_predecessor(counterfactual, private_cp362)
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Checks a supplied private Humidistat CP363 witness against the exact
/// explicit parameters and retained direct release.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> bool {
    private_humidistat_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )
    .is_some_and(|expected| snapshots_match_exact(expected, counterfactual))
}

/// Rebuilds CP363's canonical private constant-supply selected-skip witness
/// from the retained direct `None` release for CP364.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_constant_supply_humidity_ratio_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    if !direct_release_is_retained_and_complete(runtime, unit, system, direct) {
        return None;
    }

    let mut counterfactual = direct;
    counterfactual.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio);
    counterfactual.predecessor_dehumidification_control_none_case_completed_skip = false;
    counterfactual
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =
        false;
    counterfactual
        .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed =
        false;
    counterfactual
        .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =
        true;
    counterfactual.dehumidification_control_none_case_completed_skip = false;
    counterfactual.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip =
        false;
    counterfactual.dehumidification_control_humidistat_case_exited_via_break = false;
    counterfactual.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip =
        true;

    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Proves that a supplied CP363 CSH witness is the canonical selected-skip
/// counterfactual of the retained direct release.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_constant_supply_humidity_ratio_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_constant_supply_humidity_ratio_counterfactual_from_direct_release(
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
    let Some(retained) = unit.calc_cooling_humidistat_case_break.latest else {
        return false;
    };
    let Some(witness) = runtime.cooling_humidistat_case_break_latest_witness(system.id) else {
        return false;
    };
    system.id == direct.system
        && unit.system == system.id
        && snapshot_route(direct)
            == Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        && snapshots_match_exact(retained, direct)
        && snapshots_match_exact(witness, direct)
        && super::completed_direct_cooling_humidistat_case_break_is_consistent(
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
