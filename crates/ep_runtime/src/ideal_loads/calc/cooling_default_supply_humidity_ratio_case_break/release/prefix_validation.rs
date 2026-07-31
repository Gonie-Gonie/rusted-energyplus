//! CP367-to-CP368 retained/private CSH-skip lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_route;
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_case_break::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_mixed_air_assignment::{
    completed_direct_cooling_default_supply_humidity_ratio_mixed_air_assignment_is_consistent,
    cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshots_match_exact,
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn case_break_links_to_predecessor(
    case_break: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    snapshot_route(case_break) == Some(route)
        && case_break.system == predecessor.system
        && case_break.parent_call_ordinal == predecessor.parent_call_ordinal
        && case_break.controlled_zone == predecessor.controlled_zone
        && case_break.unit_body_entered == predecessor.unit_body_entered
        && case_break.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && case_break.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && case_break.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && case_break.unit_off_skipped == predecessor.unit_off_skipped
        && case_break.non_cooling_skipped == predecessor.non_cooling_skipped
        && case_break.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && case_break.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && case_break.predecessor_dehumidification_control_none_case_completed_skip
            == predecessor.dehumidification_control_none_case_completed_skip
        && case_break
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && case_break.predecessor_dehumidification_control_humidistat_case_completed_skip
            == predecessor.dehumidification_control_humidistat_case_completed_skip
        && case_break
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
        && case_break
            .predecessor_dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
            == predecessor
                .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
        && case_break.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && case_break.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip)
        && case_break.dehumidification_control_humidistat_case_completed_skip
            == (route == Route::DehumidificationControlHumidistatCaseCompletedSkip)
        && case_break.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
            == (route
                == Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip)
        && !case_break
            .dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    case_break: Snapshot,
) -> bool {
    if !case_break.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip {
        return true;
    }
    let Some(direct) = unit
        .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
        .latest
    else {
        return false;
    };
    private_default_supply_humidity_ratio_mixed_air_assignment_csh_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        predecessor,
    )
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && unit.system == system.id
        && predecessor_snapshots_match_bit_exact(retained, predecessor)
        && predecessor_snapshots_match_bit_exact(witness, predecessor)
        && cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_default_supply_humidity_ratio_mixed_air_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
        && cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshots_match_exact(
            retained, witness,
        )
}
