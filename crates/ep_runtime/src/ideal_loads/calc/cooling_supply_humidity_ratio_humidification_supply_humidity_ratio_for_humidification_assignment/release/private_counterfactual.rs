//! Canonical private selected-`None` CP373 reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state as advance,
};
use super::operand_validation::supply_mass_flow_rate_from_retained_owner;
use super::prefix_validation::{active_lineage_is_exact, assignment_links_to_predecessor};
use super::snapshot_validation::{
    predecessor_snapshot, snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_moisture_demand_assignment::private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release;
use crate::ideal_loads::{
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

/// Rebuilds the admitted selected-`None` CP373 path from canonical direct
/// lineage, CP330's retained denominator owner, and two explicit pre-sampled
/// scalars.
///
/// Neither scalar is retained owner evidence, and this function does not claim
/// a live `ZoneSysMoistureDemand` or `Node(ZoneNodeNum)` service read.
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::HumidificationControlGuardFalseFallthrough)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp372 = predecessor_snapshot(direct);
    let private_cp372 = private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp372,
        pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s,
    )?;
    let supply_mass_flow_rate_kg_per_s = supply_mass_flow_rate_from_retained_owner(
        runtime,
        unit,
        system,
        private_cp372,
    )?;
    let mut state = State::new(system.id);
    let counterfactual = advance(
        &mut state,
        private_cp372,
        Some(ActiveOperands {
            supply_mass_flow_rate_kg_per_s,
            zone_node_humidity_ratio: pre_sampled_zone_node_humidity_ratio,
        }),
    )?;
    let expected_quotient = pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s
        / supply_mass_flow_rate_kg_per_s;
    let expected = expected_quotient + pre_sampled_zone_node_humidity_ratio;
    (snapshot_route(counterfactual)
        == Some(
            Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted,
        )
        && assignment_links_to_predecessor(counterfactual, private_cp372)
        && active_lineage_is_exact(runtime, unit, system, private_cp372, counterfactual)
        && counterfactual
            .moisture_demand_derived_supply_humidity_ratio
            .is_some_and(|value| value.to_bits() == expected_quotient.to_bits())
        && counterfactual
            .resulting_supply_humidity_ratio_for_humidification
            .is_some_and(|value| value.to_bits() == expected.to_bits())
        && state.source_site_execution_count == 6
        && state.supply_humidity_ratio_for_humidification_assignment_count == 1
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

/// Proves that a supplied CP373 witness is the bit-exact canonical private
/// selected-`None` reconstruction for the supplied pre-sampled scalars.
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> bool {
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct,
        pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
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
