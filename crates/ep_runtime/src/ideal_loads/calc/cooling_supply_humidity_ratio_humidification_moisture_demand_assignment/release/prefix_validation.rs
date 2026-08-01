//! CP371 lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state,
};
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::{
    completed_direct_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_is_consistent,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshots_match_exact,
    private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let active_input = if assignment.humidification_moisture_demand_assignment_executed {
        let Some(value) = assignment.zone_humidifying_setpoint_moisture_demand_kg_per_s else {
            return false;
        };
        Some(ActiveInput {
            zone_humidifying_setpoint_moisture_demand_kg_per_s: value,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state(
        &mut state,
        predecessor,
        active_input,
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
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == predecessor.system
        && cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshots_match_exact(
            retained,
            predecessor,
        )
        && cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshots_match_exact(
            witness,
            predecessor,
        )
        && cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_is_consistent(
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
            Route::DehumidificationControlHumidistatMoistureDemandAssignmentExecuted
                | Route::DehumidificationControlNoneMoistureDemandAssignmentExecuted
        )
    ) {
        return true;
    }
    let Some(direct) = unit
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
        .latest
    else {
        return false;
    };
    private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        predecessor,
    )
}
