//! Canonical private selected-`None` CP374 reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_state as advance,
};
use super::prefix_validation::{
    active_lineage_is_exact, active_operands_from_selected_typed_owner,
    assignment_links_to_predecessor,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release,
};

/// Rebuilds the admitted selected-`None` CP374 path from canonical CP373
/// lineage and the selected typed maximum-heating humidity-ratio owner.
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::HumidificationControlGuardFalseFallthrough)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp373 = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
        .latest?;
    let private_cp373 = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp373,
        pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )?;
    let operands =
        active_operands_from_selected_typed_owner(unit, system, private_cp373)?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp373, Some(operands))?;
    let left = private_cp373.resulting_supply_humidity_ratio_for_humidification?;
    let right = operands.maximum_heating_supply_air_humidity_ratio;
    let minimum = source_shaped_two_argument_minimum(left, right);
    (snapshot_route(counterfactual)
        == Some(
            Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted,
        )
        && assignment_links_to_predecessor(counterfactual, private_cp373)
        && active_lineage_is_exact(runtime, unit, system, private_cp373, counterfactual)
        && route_independent_identity_matches(direct, counterfactual)
        && option_matches(
            counterfactual.supply_humidity_ratio_for_humidification_before_maximum_limit,
            left,
        )
        && option_matches(
            counterfactual.maximum_heating_supply_air_humidity_ratio,
            right,
        )
        && option_matches(
            counterfactual.resulting_supply_humidity_ratio_for_humidification,
            minimum,
        )
        && state.source_site_execution_count == 4)
    .then_some(counterfactual)
}

/// Proves that a supplied CP374 witness is the bit-exact canonical private
/// selected-`None` reconstruction for the supplied pre-sampled CP373 scalars.
pub(in crate::ideal_loads) fn private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
    pre_sampled_zone_humidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> bool {
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_counterfactual_from_direct_release(
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
