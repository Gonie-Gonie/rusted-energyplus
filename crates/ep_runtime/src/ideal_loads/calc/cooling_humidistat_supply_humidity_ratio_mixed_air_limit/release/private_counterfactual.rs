//! Explicitly parameterized private-Humidistat CP362 characterization.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, ZoneId};

use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Snapshot,
    advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_state as advance,
};
use super::snapshot_validation::{
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
    snapshot_route,
};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::{
    private_humidistat_counterfactual_from_direct_release as predecessor_private_counterfactual,
    private_humidistat_counterfactual_links_to_direct_release as predecessor_private_links,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::{
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};

/// Rebuilds private Humidistat CP362 from canonical CP361 and CP329 owners.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_humidistat_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest?;
    let witness =
        runtime.cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(system.id)?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct)
            != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            retained, direct,
        )
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            witness, direct,
        )
        || !super::completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_predecessor = unit
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
        .latest?;
    let private_predecessor = predecessor_private_counterfactual(
        runtime,
        unit,
        system,
        direct_predecessor,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )?;
    if !predecessor_private_links(
        runtime,
        unit,
        system,
        direct_predecessor,
        private_predecessor,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    ) {
        return None;
    }
    let operands = active_operands_from_retained_cp329(
        runtime,
        unit,
        system,
        private_predecessor.system,
        private_predecessor.parent_call_ordinal,
        private_predecessor.controlled_zone,
    )?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_predecessor, Some(operands))?;
    (snapshot_route(counterfactual)
        == Some(Route::DehumidificationControlHumidistatSupplyHumidityRatioMixedAirLimitExecuted)
        && cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
            counterfactual,
            private_predecessor,
        ))
    .then_some(counterfactual)
}

/// Checks a supplied private CP362 witness against its explicit parameters.
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
    .is_some_and(|expected| {
        cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            expected,
            counterfactual,
        )
    })
}

fn active_operands_from_retained_cp329(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    expected_system: IdealLoadsAirSystemId,
    expected_ordinal: usize,
    expected_zone: ZoneId,
) -> Option<ActiveOperands> {
    let mixed_air = unit.calc_cooling_mixed_air_call.latest?;
    let mixed_air_witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    if expected_system != mixed_air.system
        || expected_ordinal != mixed_air.parent_call_ordinal
        || expected_zone != mixed_air.controlled_zone
        || !cooling_mixed_air_call_snapshots_match_bit_exact(mixed_air, mixed_air_witness)
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
        || !completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            mixed_air,
            Some(mixed_air_witness),
        )
    {
        return None;
    }
    Some(ActiveOperands {
        mixed_air_humidity_ratio: mixed_air.mixed_air_humidity_ratio?,
    })
}
