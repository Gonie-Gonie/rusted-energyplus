//! Canonical private CP371-body CP372 reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state as advance,
};
use super::prefix_validation::{active_lineage_is_exact, assignment_links_to_predecessor};
use super::snapshot_validation::{
    predecessor_snapshot, snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds the admitted selected-`None` CP372 path from canonical CP371
/// lineage and one explicit pre-sampled scalar.
///
/// The scalar parameter is not retained owner evidence and this function does
/// not claim a live `ZoneSysMoistureDemand` service read.
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::HumidificationControlGuardFalseFallthrough)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp371 = predecessor_snapshot(direct);
    let private_cp371 = private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp371,
    )?;
    let value = pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s;
    let mut state = State::new(system.id);
    let counterfactual = advance(
        &mut state,
        private_cp371,
        Some(ActiveInput {
            zone_humidifying_setpoint_moisture_demand_kg_per_s: value,
        }),
    )?;
    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted)
        && assignment_links_to_predecessor(counterfactual, private_cp371)
        && active_lineage_is_exact(runtime, unit, system, private_cp371, counterfactual)
        && counterfactual
            .resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s
            .is_some_and(|result| result.to_bits() == value.to_bits())
        && state.source_site_execution_count == 2
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Proves that a supplied CP372 witness is the bit-exact canonical private
/// reconstruction for the supplied pre-sampled scalar.
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
) -> bool {
    private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct,
        pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s,
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
        && direct.predecessor_dehumidification_control_type
            == counterfactual.predecessor_dehumidification_control_type
        && direct.predecessor_heating_on == counterfactual.predecessor_heating_on
}
