//! CP374-to-CP375 retained lineage and branch-specific owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit::{
    completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent,
    private_humidistat_counterfactual_from_direct_release as cp362_private_counterfactual,
    private_humidistat_counterfactual_links_to_direct_release as cp362_private_links,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::{
    completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshots_match_bit_exact as cp345_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshots_match_bit_exact as cp374_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let active = matches!(
        snapshot_route(assignment),
        Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted
                | Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted
        )
    );
    let active_operands = if active {
        let Some(purchased_air_supply_humidity_ratio) =
            assignment.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum
        else {
            return false;
        };
        Some(ActiveOperands {
            purchased_air_supply_humidity_ratio,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_state(
        &mut state,
        predecessor,
        active_operands,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, assignment))
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == predecessor.system
        && cp374_snapshots_match_bit_exact(retained, predecessor)
        && cp374_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

#[allow(dead_code)]
pub(super) fn active_none_operands_from_retained_cp345(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOperands> {
    let owner = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest?;
    let witness = runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
        )?;
    if system.id != predecessor.system
        || unit.system != system.id
        || owner.system != predecessor.system
        || owner.parent_call_ordinal != predecessor.parent_call_ordinal
        || owner.controlled_zone != predecessor.controlled_zone
        || !predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed
        || predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed
        || !cp345_snapshots_match_bit_exact(owner, witness)
        || !cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(owner)
        || !completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            owner,
            Some(witness),
        )
    {
        return None;
    }
    Some(ActiveOperands {
        purchased_air_supply_humidity_ratio: owner.assigned_supply_humidity_ratio?,
    })
}

#[allow(dead_code)]
pub(super) fn none_owner_links_to_assignment(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    assignment: Snapshot,
) -> bool {
    let Some(operands) =
        active_none_operands_from_retained_cp345(runtime, unit, system, predecessor)
    else {
        return false;
    };
    snapshot_route(assignment)
        == Some(Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted)
        && assignment
            .purchased_air_supply_humidity_ratio_before_humidification_supply_maximum
            .is_some_and(|value| {
                value.to_bits() == operands.purchased_air_supply_humidity_ratio.to_bits()
            })
}

/// Resolves the Humidistat branch's same-call CP362 result-store owner through
/// its retained direct witness and canonical private counterfactual bridge.
#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn active_humidistat_operands_from_cp362_counterfactual(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> Option<ActiveOperands> {
    let direct = unit
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest?;
    let witness = runtime
        .cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(system.id)?;
    if system.id != predecessor.system
        || unit.system != system.id
        || direct.system != predecessor.system
        || direct.parent_call_ordinal != predecessor.parent_call_ordinal
        || direct.controlled_zone != predecessor.controlled_zone
        || !predecessor.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed
        || predecessor.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact(
            direct, witness,
        )
        || !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            direct,
        )
        || !completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }
    let private = cp362_private_counterfactual(
        runtime,
        unit,
        system,
        direct,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )?;
    if private.system != predecessor.system
        || private.parent_call_ordinal != predecessor.parent_call_ordinal
        || private.controlled_zone != predecessor.controlled_zone
        || !private.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed
        || !cp362_private_links(
            runtime,
            unit,
            system,
            direct,
            private,
            pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            pre_sampled_zone_node_humidity_ratio,
        )
    {
        return None;
    }
    Some(ActiveOperands {
        purchased_air_supply_humidity_ratio: private.resulting_supply_humidity_ratio?,
    })
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn active_none_operands_from_retained_cp345_for_test(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOperands> {
    active_none_operands_from_retained_cp345(runtime, unit, system, predecessor)
}
