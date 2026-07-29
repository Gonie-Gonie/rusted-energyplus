//! CP358-to-CP359 retained/private Humidistat-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Snapshot;
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::calc::cooling_humidistat_case_entry::private_humidistat_counterfactual_links_to_direct_release as cp358_private_humidistat_counterfactual_links_to_direct_release;
use crate::ideal_loads::calc::cooling_humidistat_moisture_demand_assignment::transition::predecessor_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot as Predecessor, PurchasedAirRuntimeState,
    PurchasedAirUnitRuntimeState,
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
        && assignment.predecessor_dehumidification_control_humidistat_case_entered
            == predecessor.dehumidification_control_humidistat_case_entered
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    assignment: Snapshot,
) -> bool {
    if !assignment.dehumidification_control_humidistat_moisture_demand_assignment_executed {
        return true;
    }
    let Some(direct) = unit.calc_cooling_humidistat_case_entry.latest else {
        return false;
    };
    cp358_private_humidistat_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        predecessor,
    )
}

pub(super) fn private_counterfactual_matches(expected: Snapshot, supplied: Snapshot) -> bool {
    snapshots_match_bit_exact(expected, supplied)
}
