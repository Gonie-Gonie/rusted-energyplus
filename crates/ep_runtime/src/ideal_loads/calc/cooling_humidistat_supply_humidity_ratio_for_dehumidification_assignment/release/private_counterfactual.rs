//! Parametric private-Humidistat CP360 characterization.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentActiveOperands as Operands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Snapshot,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state as advance,
};
use super::operand_validation::supply_mass_flow_rate_from_retained_owner;
use super::prefix_validation::{
    active_lineage_is_exact, assignment_links_to_predecessor, private_counterfactual_matches,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_humidistat_moisture_demand_assignment::private_humidistat_counterfactual_from_direct_release as cp359_private_humidistat_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds a private Humidistat CP360 characterization from canonical CP359
/// lineage, CP330-owned flow, and two explicit pre-sampled scalars.
///
/// The demand scalar reconstructs CP359. The Zone-node humidity-ratio scalar
/// has no retained authoritative owner. Neither parameter claims a live
/// service read, and CP329 recirculation humidity is not a Zone-node owner.
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
        .latest?;
    let witness = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp359 = unit
        .calc_cooling_humidistat_moisture_demand_assignment
        .latest?;
    let private_cp359 = cp359_private_humidistat_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp359,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    )?;
    let flow = supply_mass_flow_rate_from_retained_owner(runtime, unit, system, private_cp359)?;
    let operands = Operands {
        supply_mass_flow_rate_kg_per_s: flow,
        zone_node_humidity_ratio: pre_sampled_zone_node_humidity_ratio,
    };
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp359, Some(operands))?;
    let quotient = pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s / flow;
    let calculated = quotient + pre_sampled_zone_node_humidity_ratio;
    (snapshot_route(counterfactual)
        == Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted,
        )
        && assignment_links_to_predecessor(counterfactual, private_cp359)
        && active_lineage_is_exact(runtime, unit, system, private_cp359, counterfactual)
        && route_independent_identity_matches(direct, counterfactual)
        && option_matches(
            counterfactual.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        )
        && option_matches(counterfactual.supply_mass_flow_rate_kg_per_s, flow)
        && option_matches(
            counterfactual.moisture_demand_derived_supply_humidity_ratio,
            quotient,
        )
        && option_matches(
            counterfactual.zone_node_humidity_ratio,
            pre_sampled_zone_node_humidity_ratio,
        )
        && option_matches(
            counterfactual.resulting_supply_humidity_ratio_for_dehumidification,
            calculated,
        ))
    .then_some(counterfactual)
}

/// Proves that a supplied CP360 witness is the bit-exact parametric
/// characterization for the supplied demand and Zone-node humidity ratio.
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

fn option_matches(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
