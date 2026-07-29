//! CP334/CP344/CP345/CP352 retained/private-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_route;
use crate::ideal_loads::calc::{
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment::{
        completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_is_consistent,
        private_active_counterfactual_links_to_direct_release as cp352_private_active_counterfactual_links_to_direct_release,
    },
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent,
    cooling_positive_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot as Cp344Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot as Cp345Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot as Cp334Snapshot,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed
        && assignment.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment
            .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted)
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
    if !assignment.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed {
        return true;
    }
    let Some(operands) = active_operands_from_retained_owners(runtime, unit, system, predecessor)
    else {
        return false;
    };
    option_matches(
        assignment.supply_enthalpy_before_overdrying_limit_j_per_kg,
        operands.supply_enthalpy_before_overdrying_limit_j_per_kg,
    ) && option_matches(
        assignment.supply_temperature_c,
        operands.supply_temperature_c,
    )
}

pub(in crate::ideal_loads::calc) fn active_operands_from_retained_owners(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOperands> {
    if predecessor_route(predecessor)
        != Some(Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted)
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }

    let direct = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment
        .latest?;
    let direct_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_latest_witness(
            system.id,
        )?;
    if !same_call(
        predecessor,
        direct.system,
        direct.parent_call_ordinal,
        direct.controlled_zone,
    ) || !predecessor_snapshots_match_bit_exact(direct, direct_witness)
        || !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_snapshot_is_exact_direct_release(
            direct,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(direct_witness),
        )
        || !cp352_private_active_counterfactual_links_to_direct_release(
            runtime,
            unit,
            system,
            direct,
            predecessor,
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
    let guard_false = provenance.capacity_limit_guard_false_fallthrough_skipped;
    let sensible_false = provenance.capacity_limit_sensible_output_guard_false_fallthrough;
    let capacity_limit =
        provenance.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    if usize::from(guard_false) + usize::from(sensible_false) + usize::from(capacity_limit) != 1 {
        return None;
    }

    let supply_temperature_c = if guard_false || sensible_false {
        let owner = unit
            .calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest?;
        let owner_witness = runtime
            .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)?;
        if !same_call(
            predecessor,
            owner.system,
            owner.parent_call_ordinal,
            owner.controlled_zone,
        ) || !owner.supply_temperature_assignment_performed
            || !cp334_snapshots_match(owner, owner_witness)
            || !cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                owner,
            )
            || !completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
                runtime,
                unit,
                system,
                owner,
                Some(owner_witness),
            )
        {
            return None;
        }
        owner.assigned_supply_temperature_c?
    } else {
        let owner = unit
            .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest?;
        let owner_witness = runtime
            .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
                system.id,
            )?;
        if !same_call(
            predecessor,
            owner.system,
            owner.parent_call_ordinal,
            owner.controlled_zone,
        ) || !owner
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            || !cp344_snapshots_match(owner, owner_witness)
            || !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                owner,
            )
            || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent(
                runtime,
                unit,
                system,
                owner,
                Some(owner_witness),
            )
        {
            return None;
        }
        owner.resulting_supply_temperature_c?
    };

    Some(ActiveOperands {
        supply_enthalpy_before_overdrying_limit_j_per_kg: predecessor
            .resulting_supply_enthalpy_j_per_kg?,
        supply_temperature_c,
    })
}

/// Proves that a private constant-SHR CP353 witness is the exact active
/// counterfactual of the retained direct `None` release.
///
/// The retained CP353 release remains authoritative. The private witness is
/// accepted only after a CP352 active predecessor is rebuilt from same-call
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
            != Some(Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted)
        || !route_independent_identity_matches(direct, counterfactual)
    {
        return false;
    }

    let Some(mut private_cp352) = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment
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
    let (Some(humidity), Some(mixed_temperature), Some(mixed_enthalpy)) = (
        mixed_owner.mixed_air_humidity_ratio,
        mixed_owner.mixed_air_temperature_c,
        mixed_owner.mixed_air_enthalpy_projection_j_per_kg,
    ) else {
        return false;
    };
    let Some(provenance) = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest
    else {
        return false;
    };
    let supply_temperature = if provenance
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
    let Some(supply_temperature) = supply_temperature else {
        return false;
    };
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    let sensible = (flow * cp_air) * (mixed_temperature - supply_temperature);
    let total = sensible / system.cooling_sensible_heat_ratio;
    let specific = total / flow;
    let supply_enthalpy = mixed_enthalpy - specific;

    private_cp352.predecessor_dehumidification_control_type =
        Some(ep_model::DehumidificationControlType::ConstantSensibleHeatRatio);
    private_cp352.predecessor_dehumidification_control_none_case_completed_skip = false;
    private_cp352
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed =
        true;
    private_cp352.dehumidification_control_none_case_completed_skip = false;
    private_cp352
        .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed =
        true;
    private_cp352.mixed_air_enthalpy_read = true;
    private_cp352.mixed_air_enthalpy_j_per_kg = Some(mixed_enthalpy);
    private_cp352.cooling_total_output_read = true;
    private_cp352.cooling_total_output_w = Some(total);
    private_cp352.supply_mass_flow_rate_read = true;
    private_cp352.supply_mass_flow_rate_kg_per_s = Some(flow);
    private_cp352.specific_cooling_output_calculated = true;
    private_cp352.specific_cooling_output_j_per_kg = Some(specific);
    private_cp352.supply_enthalpy_calculated = true;
    private_cp352.calculated_supply_enthalpy_j_per_kg = Some(supply_enthalpy);
    private_cp352.supply_enthalpy_assigned = true;
    private_cp352.assigned_supply_enthalpy_j_per_kg = Some(supply_enthalpy);
    private_cp352.resulting_supply_enthalpy_j_per_kg = Some(supply_enthalpy);

    let Some(operands) = active_operands_from_retained_owners(runtime, unit, system, private_cp352)
    else {
        return false;
    };
    option_matches(
        counterfactual.supply_enthalpy_before_overdrying_limit_j_per_kg,
        operands.supply_enthalpy_before_overdrying_limit_j_per_kg,
    ) && option_matches(
        counterfactual.supply_temperature_c,
        operands.supply_temperature_c,
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

fn cp345_snapshots_match(mut left: Cp345Snapshot, mut right: Cp345Snapshot) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.assigned_supply_humidity_ratio,
        right.assigned_supply_humidity_ratio,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn cp334_snapshots_match(mut left: Cp334Snapshot, mut right: Cp334Snapshot) -> bool {
    let values_match = [
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
    }
    values_match && left == right
}

fn cp344_snapshots_match(mut left: Cp344Snapshot, mut right: Cp344Snapshot) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
        option_bits_match(
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_temperature_c = None;
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left == right
}

fn option_matches(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
