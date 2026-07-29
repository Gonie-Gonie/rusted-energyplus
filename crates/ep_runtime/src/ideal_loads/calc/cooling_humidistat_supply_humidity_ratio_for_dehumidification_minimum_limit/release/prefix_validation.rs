//! CP360-to-CP361 retained/private Humidistat-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Snapshot,
};
use super::operand_validation::minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner;
use super::snapshot_validation::{
    option_bits_match, snapshot_route, snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::{
    completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent,
    private_humidistat_counterfactual_links_to_direct_release as cp360_private_humidistat_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    snapshot_route(assignment) == Some(route)
        && assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && assignment.predecessor_dehumidification_control_none_case_completed_skip
            == predecessor.dehumidification_control_none_case_completed_skip
        && assignment
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && assignment
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed
            == predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && option_bits_match(
            assignment.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            predecessor.resulting_supply_humidity_ratio_for_dehumidification,
        )
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    assignment: Snapshot,
) -> bool {
    if !assignment
        .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed
    {
        return true;
    }
    let Some(operands) = active_operands_from_retained_owners(runtime, unit, system, predecessor)
    else {
        return false;
    };
    option_bits_match(
        assignment.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
        predecessor.resulting_supply_humidity_ratio_for_dehumidification,
    ) && option_bits_match(
        assignment.minimum_cooling_supply_air_humidity_ratio,
        Some(operands.minimum_cooling_supply_air_humidity_ratio),
    )
}

pub(in crate::ideal_loads::calc) fn active_operands_from_retained_owners(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOperands> {
    if predecessor_route(predecessor)
        != Some(
            Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted,
        )
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }

    let direct = unit
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
        .latest?;
    let direct_witness = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(
            system.id,
        )?;
    let demand = predecessor.zone_dehumidifying_setpoint_moisture_demand_kg_per_s?;
    let zone_humidity = predecessor.zone_node_humidity_ratio?;
    if !same_call(
        predecessor,
        direct.system,
        direct.parent_call_ordinal,
        direct.controlled_zone,
    ) || !predecessor_snapshots_match_bit_exact(direct, direct_witness)
        || !cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
            direct,
        )
        || !completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(direct_witness),
        )
        || !cp360_private_humidistat_counterfactual_links_to_direct_release(
            runtime,
            unit,
            system,
            direct,
            predecessor,
            demand,
            zone_humidity,
        )
    {
        return None;
    }
    let minimum = minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner(
        unit,
        system,
        predecessor,
    )?;
    Some(ActiveOperands {
        minimum_cooling_supply_air_humidity_ratio: minimum,
    })
}

pub(super) fn private_counterfactual_matches(expected: Snapshot, supplied: Snapshot) -> bool {
    snapshots_match_bit_exact(expected, supplied)
}

fn same_call(
    predecessor: Predecessor,
    system: ep_model::IdealLoadsAirSystemId,
    ordinal: usize,
    zone: ep_model::ZoneId,
) -> bool {
    predecessor.system == system
        && predecessor.parent_call_ordinal == ordinal
        && predecessor.controlled_zone == zone
}
