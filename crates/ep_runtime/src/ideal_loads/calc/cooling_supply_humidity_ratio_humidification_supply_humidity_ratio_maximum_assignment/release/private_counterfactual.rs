//! Canonical private selected-`None` CP375 reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_state as advance,
};
use super::prefix_validation::{
    active_none_operands_from_retained_cp345, assignment_links_to_predecessor,
    none_owner_links_to_assignment,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;
use crate::ideal_loads::{
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_links_to_direct_release,
};

/// Rebuilds selected-`None` CP375 from canonical CP374 and CP345 owners.
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::HumidificationControlGuardFalseFallthrough)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp374 = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit
        .latest?;
    let private_cp374 = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp374,
        pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )?;
    if !private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct_cp374,
        private_cp374,
        pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    ) {
        return None;
    }
    let operands =
        active_none_operands_from_retained_cp345(runtime, unit, system, private_cp374)?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp374, Some(operands))?;
    let right = private_cp374.resulting_supply_humidity_ratio_for_humidification?;
    let left = operands.purchased_air_supply_humidity_ratio;
    let maximum = source_shaped_two_argument_maximum(left, right);
    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted)
        && assignment_links_to_predecessor(counterfactual, private_cp374)
        && none_owner_links_to_assignment(
            runtime,
            unit,
            system,
            private_cp374,
            counterfactual,
        )
        && route_independent_identity_matches(direct, counterfactual)
        && option_matches(
            counterfactual.supply_humidity_ratio_for_humidification_for_supply_maximum,
            right,
        )
        && option_matches(counterfactual.resulting_supply_humidity_ratio, maximum)
        && state.source_site_execution_count == 4)
    .then_some(counterfactual)
}

/// Proves a supplied CP375 witness is the canonical selected-`None` reconstruction.
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> bool {
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release(
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

fn option_matches(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
