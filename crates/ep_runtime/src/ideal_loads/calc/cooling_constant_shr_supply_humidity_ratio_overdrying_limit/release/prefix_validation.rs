//! CP345/CP353 retained/private-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::{
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit::{
        completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_is_consistent,
        private_active_counterfactual_links_to_direct_release,
    },
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_overdrying_limit::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Predecessor,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot as Cp345Snapshot,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
        && assignment.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted)
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
        .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed
    {
        return true;
    }
    let Some(operands) = active_operands_from_retained_owners(runtime, unit, system, predecessor)
    else {
        return false;
    };
    option_matches(
        assignment.supply_humidity_ratio_before_overdrying_limit,
        operands.supply_humidity_ratio_before_overdrying_limit,
    ) && option_matches(
        assignment.supply_temperature_c,
        operands.supply_temperature_c,
    ) && option_matches(
        assignment.supply_enthalpy_j_per_kg,
        operands.supply_enthalpy_j_per_kg,
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
            Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioOverdryingLimitExecuted,
        )
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }

    let direct = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
        .latest?;
    let direct_witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witness(
            system.id,
        )?;
    if !same_call(
        predecessor,
        direct.system,
        direct.parent_call_ordinal,
        direct.controlled_zone,
    ) || !predecessor_snapshots_match_bit_exact(direct, direct_witness)
        || !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
            direct,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(direct_witness),
        )
        || !private_active_counterfactual_links_to_direct_release(
            runtime,
            unit,
            system,
            direct,
            predecessor,
        )
    {
        return None;
    }

    let humidity_owner = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest?;
    let humidity_witness = runtime
        .cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
        )?;
    if !same_call(
        predecessor,
        humidity_owner.system,
        humidity_owner.parent_call_ordinal,
        humidity_owner.controlled_zone,
    ) || !humidity_owner.supply_humidity_ratio_assignment_performed
        || !cp345_snapshots_match(humidity_owner, humidity_witness)
        || !cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            humidity_owner,
        )
        || !completed_direct_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            humidity_owner,
            Some(humidity_witness),
        )
    {
        return None;
    }

    Some(ActiveOperands {
        supply_humidity_ratio_before_overdrying_limit: humidity_owner
            .assigned_supply_humidity_ratio?,
        supply_temperature_c: predecessor.supply_temperature_c?,
        supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg?,
    })
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
