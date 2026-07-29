//! Parametric private-Humidistat CP359 characterization.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentActiveOperands as Operands,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Snapshot,
    advance_cooling_humidistat_moisture_demand_assignment_state as advance,
};
use super::prefix_validation::{
    active_lineage_is_exact, assignment_links_to_predecessor, private_counterfactual_matches,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_humidistat_case_entry::private_humidistat_counterfactual_from_direct_release as cp358_private_humidistat_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds a private Humidistat CP359 characterization from canonical CP358
/// lineage and one explicit pre-sampled scalar.
///
/// The scalar parameter is not retained owner evidence and this function does
/// not claim a live `ZoneSysMoistureDemand` service read.
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_humidistat_moisture_demand_assignment
        .latest?;
    let witness =
        runtime.cooling_humidistat_moisture_demand_assignment_latest_witness(system.id)?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp358 = unit.calc_cooling_humidistat_case_entry.latest?;
    let private_cp358 = cp358_private_humidistat_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp358,
    )?;
    let operands = Operands {
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s:
            pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    };
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp358, Some(operands))?;
    let expected_bits = pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s.to_bits();
    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted)
        && assignment_links_to_predecessor(counterfactual, private_cp358)
        && active_lineage_is_exact(runtime, unit, system, private_cp358, counterfactual)
        && route_independent_identity_matches(direct, counterfactual)
        && counterfactual
            .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
            .is_some_and(|value| value.to_bits() == expected_bits))
    .then_some(counterfactual)
}

/// Proves a supplied CP359 witness is the bit-exact parametric characterization
/// for the supplied pre-sampled scalar.
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
) -> bool {
    private_humidistat_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    )
    .is_some_and(|expected| private_counterfactual_matches(expected, counterfactual))
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
