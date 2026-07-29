//! CP364-to-CP365 retained/private constant-supply lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::{option_bits_match, snapshot_route};
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_assignment::transition::{
    predecessor_route, predecessor_snapshots_match_exact,
};
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_entry::{
    completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent,
    cooling_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact,
    private_constant_supply_humidity_ratio_case_entry_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release,
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
        && assignment.predecessor_dehumidification_control_humidistat_case_completed_skip
            == predecessor.dehumidification_control_humidistat_case_completed_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_entered
            == predecessor.dehumidification_control_constant_supply_humidity_ratio_case_entered
}

pub(in crate::ideal_loads) fn cooling_constant_supply_humidity_ratio_assignment_snapshot_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    assignment_links_to_predecessor(assignment, predecessor)
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    assignment: Snapshot,
) -> bool {
    if !assignment.dehumidification_control_constant_supply_humidity_ratio_assignment_executed {
        return true;
    }
    minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner(
        runtime,
        unit,
        system,
        predecessor,
    )
    .is_some_and(|minimum| {
        option_bits_match(
            assignment.minimum_cooling_supply_air_humidity_ratio,
            Some(minimum),
        ) && option_bits_match(assignment.assigned_supply_humidity_ratio, Some(minimum))
            && option_bits_match(assignment.resulting_supply_humidity_ratio, Some(minimum))
    })
}

pub(super) fn minimum_cooling_supply_air_humidity_ratio_from_selected_typed_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<f64> {
    if predecessor_route(predecessor)
        != Some(Route::DehumidificationControlConstantSupplyHumidityRatioAssigned)
        || system.id != predecessor.system
        || unit.system != system.id
    {
        return None;
    }
    let direct = unit
        .calc_cooling_constant_supply_humidity_ratio_case_entry
        .latest?;
    let direct_witness =
        runtime.cooling_constant_supply_humidity_ratio_case_entry_latest_witness(system.id)?;
    if !cooling_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact(
        direct,
        direct_witness,
    ) || !cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
        direct,
    ) || !completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent(
        runtime,
        unit,
        system,
        direct,
        Some(direct_witness),
    )
        || !private_constant_supply_humidity_ratio_case_entry_counterfactual_links_to_direct_release(
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
    minimum.is_finite().then_some(minimum)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_constant_supply_humidity_ratio_case_entry
        .latest
    else {
        return false;
    };
    let Some(witness) =
        runtime.cooling_constant_supply_humidity_ratio_case_entry_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && unit.system == system.id
        && predecessor_snapshots_match_exact(retained, predecessor)
        && predecessor_snapshots_match_exact(witness, predecessor)
        && cooling_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_constant_supply_humidity_ratio_case_entry_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}
