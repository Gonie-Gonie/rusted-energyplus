//! CP329/CP330/CP351 retained/private-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_route;
use crate::ideal_loads::calc::{
    cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment::{
        completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_is_consistent,
        private_active_counterfactual_links_to_direct_release as cp351_private_active_counterfactual_links_to_direct_release,
        total_output_assignment_snapshots_match_bit_exact,
    },
    cooling_supply_mass_flow_positive_guard::{
        cooling_supply_mass_flow_positive_guard_snapshots_match_bit_exact as positive_guard_snapshots_match_bit_exact,
        completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent,
    },
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment::transition::predecessor_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};

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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed
        && assignment.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment
            .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned)
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
        .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed
    {
        return true;
    }
    let Some(operands) =
        active_operands_from_retained_owners(runtime, unit, system, predecessor)
    else {
        return false;
    };
    option_matches(
        assignment.mixed_air_enthalpy_j_per_kg,
        operands.mixed_air_enthalpy_j_per_kg,
    ) && option_matches(
        assignment.cooling_total_output_w,
        operands.cooling_total_output_w,
    ) && option_matches(
        assignment.supply_mass_flow_rate_kg_per_s,
        operands.supply_mass_flow_rate_kg_per_s,
    )
}

pub(in crate::ideal_loads::calc) fn active_operands_from_retained_owners(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOperands> {
    if predecessor_route(predecessor)
        != Some(Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned)
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }

    let direct = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment
        .latest?;
    let direct_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_latest_witness(
            system.id,
        )?;
    if !same_call(
        predecessor,
        direct.system,
        direct.parent_call_ordinal,
        direct.controlled_zone,
    ) || !total_output_assignment_snapshots_match_bit_exact(direct, direct_witness)
        || !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_snapshot_is_exact_direct_release(
            direct,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(direct_witness),
        )
        || !cp351_private_active_counterfactual_links_to_direct_release(
            runtime,
            unit,
            system,
            direct,
            predecessor,
        )
    {
        return None;
    }

    let mixed_owner = unit.calc_cooling_mixed_air_call.latest?;
    let mixed_witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    let mixed = mixed_owner.mixed_air_enthalpy_projection_j_per_kg?;
    if !same_call(
        predecessor,
        mixed_owner.system,
        mixed_owner.parent_call_ordinal,
        mixed_owner.controlled_zone,
    ) || !mixed_owner.mixed_air_enthalpy_projection_assigned
        || !cooling_mixed_air_call_snapshots_match_bit_exact(mixed_owner, mixed_witness)
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_owner)
        || !completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            mixed_owner,
            Some(mixed_witness),
        )
    {
        return None;
    }

    let flow_owner = unit.calc_cooling_supply_mass_flow_positive_guard.latest?;
    let flow_witness = runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id)?;
    let flow = flow_owner.supply_mass_flow_rate_kg_per_s?;
    if !same_call(
        predecessor,
        flow_owner.system,
        flow_owner.parent_call_ordinal,
        flow_owner.controlled_zone,
    ) || !flow_owner.positive_supply_mass_flow_body_entered
        || !positive_guard_snapshots_match_bit_exact(flow_owner, flow_witness)
        || !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(flow_owner)
        || !completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
            runtime,
            unit,
            system,
            flow_owner,
            Some(flow_witness),
        )
        || flow <= 0.0
        || flow.is_nan()
    {
        return None;
    }

    Some(ActiveOperands {
        mixed_air_enthalpy_j_per_kg: mixed,
        cooling_total_output_w: predecessor.cooling_total_output_w?,
        supply_mass_flow_rate_kg_per_s: flow,
    })
}

/// Proves that a private constant-SHR CP352 witness is the exact active
/// counterfactual of the retained direct `None` release.
///
/// The retained CP352 release remains authoritative. The private witness is
/// accepted only after a CP351 active predecessor is rebuilt from same-call
/// canonical owners and recursively validated.
pub(in crate::ideal_loads::calc) fn private_active_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    if snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || snapshot_route(counterfactual)
            != Some(
                Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned,
            )
        || !route_independent_identity_matches(direct, counterfactual)
    {
        return false;
    }

    let Some(mut private_cp351) = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment
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
    let sensible = (flow * cp_air) * (mixed - supply);
    let ratio = system.cooling_sensible_heat_ratio;
    let total = sensible / ratio;

    private_cp351.predecessor_dehumidification_control_type =
        Some(ep_model::DehumidificationControlType::ConstantSensibleHeatRatio);
    private_cp351.predecessor_dehumidification_control_none_case_completed_skip = false;
    private_cp351
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed =
        true;
    private_cp351.dehumidification_control_none_case_completed_skip = false;
    private_cp351
        .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed =
        true;
    private_cp351.cooling_sensible_output_read = true;
    private_cp351.cooling_sensible_output_w = Some(sensible);
    private_cp351.cooling_sensible_heat_ratio_read = true;
    private_cp351.cooling_sensible_heat_ratio = Some(ratio);
    private_cp351.cooling_total_output_calculated = true;
    private_cp351.calculated_cooling_total_output_w = Some(total);
    private_cp351.cooling_total_output_assigned = true;
    private_cp351.cooling_total_output_w = Some(total);

    let Some(operands) =
        active_operands_from_retained_owners(runtime, unit, system, private_cp351)
    else {
        return false;
    };
    option_matches(
        counterfactual.mixed_air_enthalpy_j_per_kg,
        operands.mixed_air_enthalpy_j_per_kg,
    ) && option_matches(
        counterfactual.cooling_total_output_w,
        operands.cooling_total_output_w,
    ) && option_matches(
        counterfactual.supply_mass_flow_rate_kg_per_s,
        operands.supply_mass_flow_rate_kg_per_s,
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
