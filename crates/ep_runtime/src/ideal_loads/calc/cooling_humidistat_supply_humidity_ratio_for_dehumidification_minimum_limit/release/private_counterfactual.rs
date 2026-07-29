//! Parametric private-Humidistat CP361 characterization.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Snapshot,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_state as advance,
};
use super::prefix_validation::{
    active_lineage_is_exact, active_operands_from_retained_owners,
    assignment_links_to_predecessor, private_counterfactual_matches,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::private_humidistat_counterfactual_from_direct_release as cp360_private_humidistat_counterfactual_from_direct_release;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

/// Rebuilds private Humidistat CP361 from canonical CP360 lineage and the
/// selected typed minimum-cooling humidity-ratio owner.
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
        .latest?;
    let witness = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp360 = unit
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
        .latest?;
    let private_cp360 = cp360_private_humidistat_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp360,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )?;
    let operands = active_operands_from_retained_owners(runtime, unit, system, private_cp360)?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp360, Some(operands))?;
    let left = private_cp360.resulting_supply_humidity_ratio_for_dehumidification?;
    let right = operands.minimum_cooling_supply_air_humidity_ratio;
    let maximum = source_shaped_two_argument_maximum(left, right);
    (snapshot_route(counterfactual)
        == Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted,
        )
        && assignment_links_to_predecessor(counterfactual, private_cp360)
        && active_lineage_is_exact(runtime, unit, system, private_cp360, counterfactual)
        && route_independent_identity_matches(direct, counterfactual)
        && option_matches(
            counterfactual.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
            left,
        )
        && option_matches(
            counterfactual.minimum_cooling_supply_air_humidity_ratio,
            right,
        )
        && option_matches(
            counterfactual.resulting_supply_humidity_ratio_for_dehumidification,
            maximum,
        ))
    .then_some(counterfactual)
}

/// Proves that a supplied CP361 witness is the bit-exact private
/// characterization for the supplied CP360 pre-sampled parameters.
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
