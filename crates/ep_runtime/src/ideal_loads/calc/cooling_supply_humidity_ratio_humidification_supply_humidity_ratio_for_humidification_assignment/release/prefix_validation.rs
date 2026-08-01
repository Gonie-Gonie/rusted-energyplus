//! CP372-to-CP373 retained/private lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_moisture_demand_assignment::{
    completed_direct_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_is_consistent,
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshots_match_bit_exact as cp372_snapshots_match_bit_exact,
    private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let active = matches!(
        snapshot_route(assignment),
        Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted
                | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted
        )
    );
    let active_operands = if active {
        let (Some(flow), Some(zone_node_humidity_ratio)) = (
            assignment.supply_mass_flow_rate_kg_per_s,
            assignment.zone_node_humidity_ratio,
        ) else {
            return false;
        };
        Some(ActiveOperands {
            supply_mass_flow_rate_kg_per_s: flow,
            zone_node_humidity_ratio,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state(
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
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == predecessor.system
        && cp372_snapshots_match_bit_exact(retained, predecessor)
        && cp372_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(predecessor)
        && completed_direct_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_is_consistent(
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
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted
                | Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted
        )
    ) {
        return true;
    }
    let Some(direct) = unit
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment
        .latest
    else {
        return false;
    };
    let Some(sampled_demand) =
        predecessor.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s
    else {
        return false;
    };
    private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        predecessor,
        sampled_demand,
    )
}
