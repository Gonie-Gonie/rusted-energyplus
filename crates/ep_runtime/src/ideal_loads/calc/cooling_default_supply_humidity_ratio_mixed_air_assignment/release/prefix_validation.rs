//! CP366-to-CP367 retained/private constant-supply lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::snapshot_route;
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_break::{
    completed_direct_cooling_constant_supply_humidity_ratio_case_break_is_consistent,
    cooling_constant_supply_humidity_ratio_case_break_snapshots_match_exact,
    private_constant_supply_humidity_ratio_case_break_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_mixed_air_assignment::transition::{
    predecessor_route, predecessor_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release,
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
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && assignment.predecessor_dehumidification_control_humidistat_case_completed_skip
            == predecessor.dehumidification_control_humidistat_case_completed_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_exited_via_break
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip)
        && assignment.dehumidification_control_humidistat_case_completed_skip
            == (route == Route::DehumidificationControlHumidistatCaseCompletedSkip)
        && assignment.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
            == (route
                == Route::DehumidificationControlConstantSupplyHumidityRatioCaseCompletedSkip)
        && !assignment
            .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    assignment: Snapshot,
) -> bool {
    if !assignment
        .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
    {
        return true;
    }
    let Some(direct) = unit
        .calc_cooling_constant_supply_humidity_ratio_case_break
        .latest
    else {
        return false;
    };
    private_constant_supply_humidity_ratio_case_break_counterfactual_links_to_direct_release(
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
        .calc_cooling_constant_supply_humidity_ratio_case_break
        .latest
    else {
        return false;
    };
    let Some(witness) =
        runtime.cooling_constant_supply_humidity_ratio_case_break_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && unit.system == system.id
        && predecessor_snapshots_match_bit_exact(retained, predecessor)
        && predecessor_snapshots_match_bit_exact(witness, predecessor)
        && cooling_constant_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_constant_supply_humidity_ratio_case_break_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
        && cooling_constant_supply_humidity_ratio_case_break_snapshots_match_exact(
            retained, witness,
        )
}
