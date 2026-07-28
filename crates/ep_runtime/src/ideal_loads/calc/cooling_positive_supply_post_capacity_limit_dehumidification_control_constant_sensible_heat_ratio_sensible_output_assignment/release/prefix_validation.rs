//! CP329/CP330/CP334/CP344/CP345/CP349 retained-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_route;
use crate::ideal_loads::calc::{
    cooling_mixed_air_call::completed_direct_cooling_mixed_air_call_is_consistent,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment::{
        completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent,
        private_active_counterfactual_links_to_direct_release as cp349_private_active_counterfactual_links_to_direct_release,
    },
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent,
    cooling_positive_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent,
    cooling_supply_mass_flow_positive_guard::completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod snapshot_matching;

use snapshot_matching::{
    cp330_snapshots_match, cp334_snapshots_match, cp344_snapshots_match, cp345_snapshots_match,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
        && assignment.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment
            .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned)
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
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
    {
        return true;
    }
    let Some(input) = active_input_from_retained_owners(runtime, unit, system, predecessor) else {
        return false;
    };
    let Some(cp_air) = predecessor.cp_air_j_per_kg_k else {
        return false;
    };
    option_matches(
        assignment.supply_mass_flow_rate_kg_per_s,
        input.supply_mass_flow_rate_kg_per_s,
    ) && option_matches(assignment.cp_air_j_per_kg_k, cp_air)
        && option_matches(
            assignment.mixed_air_temperature_c,
            input.mixed_air_temperature_c,
        )
        && option_matches(assignment.supply_temperature_c, input.supply_temperature_c)
}

pub(in crate::ideal_loads) fn active_input_from_retained_owners(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveInput> {
    if predecessor_route(predecessor)
        != Some(Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned)
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }
    let retained_predecessor = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
        .latest?;
    let retained_predecessor_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witness(
            system.id,
        )?;
    if !same_call(
        predecessor,
        retained_predecessor.system,
        retained_predecessor.parent_call_ordinal,
        retained_predecessor.controlled_zone,
    ) || !predecessor_snapshots_match_bit_exact(
        retained_predecessor,
        retained_predecessor_witness,
    ) || !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(
        retained_predecessor,
    ) || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_is_consistent(
        runtime,
        unit,
        system,
        retained_predecessor,
        Some(retained_predecessor_witness),
    ) {
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
        || !cp330_snapshots_match(flow_owner, flow_witness)
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

    let mixed_owner = unit.calc_cooling_mixed_air_call.latest?;
    let mixed_witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    let mixed = mixed_owner.mixed_air_temperature_c?;
    if !same_call(
        predecessor,
        mixed_owner.system,
        mixed_owner.parent_call_ordinal,
        mixed_owner.controlled_zone,
    ) || !mixed_owner.mixed_air_temperature_assigned
        || !cooling_mixed_air_call_snapshots_match_bit_exact(mixed_owner, mixed_witness)
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_owner)
        || !completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            mixed_owner,
            Some(mixed_witness),
        )
        || !cp349_private_active_counterfactual_links_to_direct_release(
            retained_predecessor,
            predecessor,
            mixed_owner,
        )
    {
        return None;
    }

    let provenance = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest?;
    let provenance_witness = runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
        )?;
    if !same_call(
        predecessor,
        provenance.system,
        provenance.parent_call_ordinal,
        provenance.controlled_zone,
    ) || !cp345_snapshots_match(provenance, provenance_witness)
        || !cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            provenance,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            provenance,
            Some(provenance_witness),
        )
    {
        return None;
    }
    let g = provenance.capacity_limit_guard_false_fallthrough_skipped;
    let f = provenance.capacity_limit_sensible_output_guard_false_fallthrough;
    let l = provenance.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    if usize::from(g) + usize::from(f) + usize::from(l) != 1 {
        return None;
    }
    let supply = if g || f {
        let owner = unit
            .calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest?;
        let witness = runtime
            .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)?;
        if !same_call(
            predecessor,
            owner.system,
            owner.parent_call_ordinal,
            owner.controlled_zone,
        ) || !owner.supply_temperature_assignment_performed
            || !cp334_snapshots_match(owner, witness)
            || !cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                owner,
            )
            || !completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
                runtime,
                unit,
                system,
                owner,
                Some(witness),
            )
        {
            return None;
        }
        owner.assigned_supply_temperature_c?
    } else {
        let owner = unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest?;
        let witness = runtime
            .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
                system.id,
            )?;
        if !same_call(
            predecessor,
            owner.system,
            owner.parent_call_ordinal,
            owner.controlled_zone,
        ) || !owner.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            || !cp344_snapshots_match(owner, witness)
            || !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                owner,
            )
            || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent(
                runtime,
                unit,
                system,
                owner,
                Some(witness),
            )
        {
            return None;
        }
        owner.resulting_supply_temperature_c?
    };
    Some(ActiveInput {
        supply_mass_flow_rate_kg_per_s: flow,
        mixed_air_temperature_c: mixed,
        supply_temperature_c: supply,
    })
}

/// Proves that a private constant-SHR CP350 witness is the exact active
/// counterfactual of the retained direct `None` release.
///
/// The retained CP350 release remains the public source of truth. The private
/// witness is accepted only after the retained CP349-to-CP350 lineage is
/// replayed from its canonical owners and every CP350 operand/result bit is
/// rechecked.
pub(in crate::ideal_loads::calc) fn private_active_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    if snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || snapshot_route(counterfactual)
            != Some(Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned)
        || !route_independent_identity_matches(direct, counterfactual)
    {
        return false;
    }
    let Some(retained_cp349) = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment
        .latest
    else {
        return false;
    };
    let Some(mixed_owner) = unit.calc_cooling_mixed_air_call.latest else {
        return false;
    };
    let Some(humidity) = mixed_owner.mixed_air_humidity_ratio else {
        return false;
    };
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    let mut private_cp349 = retained_cp349;
    private_cp349.predecessor_dehumidification_control_type =
        Some(ep_model::DehumidificationControlType::ConstantSensibleHeatRatio);
    private_cp349.predecessor_dehumidification_control_none_case_completed = false;
    private_cp349.predecessor_dehumidification_control_none_case_completed_skip = false;
    private_cp349
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered = true;
    private_cp349.dehumidification_control_none_case_completed_skip = false;
    private_cp349
        .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed = true;
    private_cp349.mixed_air_humidity_ratio_read = true;
    private_cp349.mixed_air_humidity_ratio = Some(humidity);
    private_cp349.psychrometric_cp_air_evaluated = true;
    private_cp349.psychrometric_cp_air_result_j_per_kg_k = Some(cp_air);
    private_cp349.cp_air_assigned = true;
    private_cp349.cp_air_j_per_kg_k = Some(cp_air);

    let Some(input) =
        active_input_from_retained_owners(runtime, unit, system, private_cp349)
    else {
        return false;
    };
    option_matches(
        counterfactual.supply_mass_flow_rate_kg_per_s,
        input.supply_mass_flow_rate_kg_per_s,
    ) && option_matches(counterfactual.cp_air_j_per_kg_k, cp_air)
        && option_matches(
            counterfactual.mixed_air_temperature_c,
            input.mixed_air_temperature_c,
        )
        && option_matches(
            counterfactual.supply_temperature_c,
            input.supply_temperature_c,
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
