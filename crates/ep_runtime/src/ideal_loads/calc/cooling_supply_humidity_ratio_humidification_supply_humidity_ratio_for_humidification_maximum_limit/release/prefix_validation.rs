//! CP373-to-CP374 retained/private lineage and owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_state,
};
use super::operand_validation::maximum_heating_supply_air_humidity_ratio_from_selected_typed_owner;
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment::{
    completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshots_match_bit_exact as cp373_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let active = matches!(
        snapshot_route(assignment),
        Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted
                | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted
        )
    );
    let active_operands = if active {
        let Some(maximum_heating_supply_air_humidity_ratio) =
            assignment.maximum_heating_supply_air_humidity_ratio
        else {
            return false;
        };
        Some(ActiveOperands {
            maximum_heating_supply_air_humidity_ratio,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_state(
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
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == predecessor.system
        && cp373_snapshots_match_bit_exact(retained, predecessor)
        && cp373_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    assignment: Snapshot,
) -> bool {
    if !matches!(
        snapshot_route(assignment),
        Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted
                | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted
        )
    ) {
        return true;
    }
    let Some(direct) = unit
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment
        .latest
    else {
        return false;
    };
    let (Some(sampled_demand), Some(sampled_zone_humidity)) = (
        predecessor.zone_humidifying_setpoint_moisture_demand_kg_per_s,
        predecessor.zone_node_humidity_ratio,
    ) else {
        return false;
    };
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        predecessor,
        sampled_demand,
        sampled_zone_humidity,
    )
}

pub(super) fn active_operands_from_selected_typed_owner(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOperands> {
    Some(ActiveOperands {
        maximum_heating_supply_air_humidity_ratio:
            maximum_heating_supply_air_humidity_ratio_from_selected_typed_owner(
                unit,
                system,
                predecessor,
            )?,
    })
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn active_operands_from_selected_typed_owner_for_test(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOperands> {
    active_operands_from_selected_typed_owner(unit, system, predecessor)
}
