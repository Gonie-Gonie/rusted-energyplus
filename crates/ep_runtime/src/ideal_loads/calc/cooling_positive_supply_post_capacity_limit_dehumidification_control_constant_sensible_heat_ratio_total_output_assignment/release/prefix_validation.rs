//! CP350 retained/private-lineage and `CoolSHR` owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_route;
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::{
    completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_is_consistent,
    private_active_counterfactual_links_to_direct_release as cp350_private_active_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    assignment.system == predecessor.system
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
        && assignment.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment
            .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned)
        && assignment.dehumidification_control_humidistat_case_selected_skip
            == (route == Route::DehumidificationControlHumidistatCaseSelectedSkip)
        && assignment.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == (route
                == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    assignment: Snapshot,
) -> bool {
    if !assignment
        .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed
    {
        return true;
    }
    let Some(input) =
        active_input_from_retained_owner(runtime, unit, system, predecessor)
    else {
        return false;
    };
    let Some(sensible) = predecessor.cooling_sensible_output_w else {
        return false;
    };
    option_matches(assignment.cooling_sensible_output_w, sensible)
        && option_matches(
            assignment.cooling_sensible_heat_ratio,
            input.cooling_sensible_heat_ratio,
        )
}

pub(in crate::ideal_loads::calc) fn active_input_from_retained_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveInput> {
    if predecessor_route(predecessor)
        != Some(Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned)
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }
    let direct = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment
        .latest?;
    let direct_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_latest_witness(
            system.id,
        )?;
    if !same_call(
        predecessor,
        direct.system,
        direct.parent_call_ordinal,
        direct.controlled_zone,
    ) || !predecessor_snapshots_match_bit_exact(direct, direct_witness)
        || !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(
            direct,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(direct_witness),
        )
        || !cp350_private_active_counterfactual_links_to_direct_release(
            runtime,
            unit,
            system,
            direct,
            predecessor,
        )
    {
        return None;
    }
    Some(ActiveInput {
        cooling_sensible_heat_ratio: system.cooling_sensible_heat_ratio,
    })
}

/// Proves that a private constant-SHR CP351 witness is the exact active
/// counterfactual of the retained direct `None` release.
///
/// The retained CP351 release remains authoritative. The private witness is
/// accepted only after a CP350 active predecessor is rebuilt from the
/// same-call canonical owners and recursively validated.
pub(in crate::ideal_loads::calc) fn private_active_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    if snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || snapshot_route(counterfactual)
            != Some(Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned)
        || !route_independent_identity_matches(direct, counterfactual)
    {
        return false;
    }

    let Some(mut private_cp350) = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment
        .latest
    else {
        return false;
    };
    let Some(flow) = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .and_then(|snapshot| snapshot.supply_mass_flow_rate_kg_per_s)
    else {
        return false;
    };
    let Some(mixed_owner) = unit.calc_cooling_mixed_air_call.latest else {
        return false;
    };
    let (Some(humidity), Some(mixed)) = (
        mixed_owner.mixed_air_humidity_ratio,
        mixed_owner.mixed_air_temperature_c,
    ) else {
        return false;
    };
    let Some(provenance) = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest
    else {
        return false;
    };
    let supply = if provenance
        .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
    {
        unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest
            .and_then(|snapshot| snapshot.resulting_supply_temperature_c)
    } else {
        unit.calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest
            .and_then(|snapshot| snapshot.assigned_supply_temperature_c)
    };
    let Some(supply) = supply else {
        return false;
    };
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    let flow_times_cp_air = flow * cp_air;
    let difference = mixed - supply;
    let sensible = flow_times_cp_air * difference;

    private_cp350.predecessor_dehumidification_control_type =
        Some(ep_model::DehumidificationControlType::ConstantSensibleHeatRatio);
    private_cp350.predecessor_dehumidification_control_none_case_completed_skip = false;
    private_cp350
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed =
        true;
    private_cp350.dehumidification_control_none_case_completed_skip = false;
    private_cp350
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed =
        true;
    private_cp350.supply_mass_flow_rate_read = true;
    private_cp350.supply_mass_flow_rate_kg_per_s = Some(flow);
    private_cp350.cp_air_read = true;
    private_cp350.cp_air_j_per_kg_k = Some(cp_air);
    private_cp350.supply_mass_flow_rate_times_cp_air_calculated = true;
    private_cp350.supply_mass_flow_rate_times_cp_air_w_per_k = Some(flow_times_cp_air);
    private_cp350.mixed_air_temperature_read = true;
    private_cp350.mixed_air_temperature_c = Some(mixed);
    private_cp350.supply_temperature_read = true;
    private_cp350.supply_temperature_c = Some(supply);
    private_cp350.mixed_air_minus_supply_temperature_calculated = true;
    private_cp350.mixed_air_minus_supply_temperature_k = Some(difference);
    private_cp350.cooling_sensible_output_calculated = true;
    private_cp350.calculated_cooling_sensible_output_w = Some(sensible);
    private_cp350.cooling_sensible_output_assigned = true;
    private_cp350.cooling_sensible_output_w = Some(sensible);

    let Some(input) =
        active_input_from_retained_owner(runtime, unit, system, private_cp350)
    else {
        return false;
    };
    option_matches(counterfactual.cooling_sensible_output_w, sensible)
        && option_matches(
            counterfactual.cooling_sensible_heat_ratio,
            input.cooling_sensible_heat_ratio,
        )
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

fn option_matches(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
