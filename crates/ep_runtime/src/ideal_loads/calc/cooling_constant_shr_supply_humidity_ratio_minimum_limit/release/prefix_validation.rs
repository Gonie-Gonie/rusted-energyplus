//! CP354 retained/private-lineage and selected-model-owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_minimum_limit::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_overdrying_limit::{
    completed_direct_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_is_consistent,
    private_active_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_overdrying_limit_executed
        && assignment.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMinimumLimitExecuted)
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
        .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed
    {
        return true;
    }
    let Some(operands) = active_operands_from_retained_owners(runtime, unit, system, predecessor)
    else {
        return false;
    };
    let Some(predecessor_result) = predecessor.resulting_supply_humidity_ratio else {
        return false;
    };
    option_matches(
        assignment.supply_humidity_ratio_before_minimum_limit,
        predecessor_result,
    ) && option_matches(
        assignment.minimum_cooling_supply_air_humidity_ratio,
        operands.minimum_cooling_supply_air_humidity_ratio,
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
            Route::DehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioMinimumLimitExecuted,
        )
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }

    let direct = unit
        .calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit
        .latest?;
    let direct_witness = runtime
        .cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness(system.id)?;
    if !same_call(
        predecessor,
        direct.system,
        direct.parent_call_ordinal,
        direct.controlled_zone,
    ) || !predecessor_snapshots_match_bit_exact(direct, direct_witness)
        || !cooling_constant_shr_supply_humidity_ratio_overdrying_limit_snapshot_is_exact_direct_release(
            direct,
        )
        || !completed_direct_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_is_consistent(
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

    let minimum = system.minimum_cooling_supply_air_humidity_ratio;
    minimum.is_finite().then_some(ActiveOperands {
        minimum_cooling_supply_air_humidity_ratio: minimum,
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

fn option_matches(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
