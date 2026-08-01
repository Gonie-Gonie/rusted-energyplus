//! CP368-to-CP369 retained and private-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Snapshot;
use super::snapshot_validation::snapshot_route;
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_case_break::{
    completed_direct_cooling_default_supply_humidity_ratio_case_break_is_consistent,
    cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact,
    private_default_supply_humidity_ratio_case_break_csh_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_heating_availability_guard::transition::{
    PredecessorRoute, predecessor_route,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_default_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release,
    cooling_humidification_flow_snapshot_is_exact_direct_release,
};

pub(super) fn guard_links_to_predecessor(
    guard: Snapshot,
    predecessor: Predecessor,
    heating_on: bool,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let active = is_active(route);
    snapshot_route(guard).is_some()
        && guard.system == predecessor.system
        && guard.parent_call_ordinal == predecessor.parent_call_ordinal
        && guard.controlled_zone == predecessor.controlled_zone
        && guard.unit_body_entered == predecessor.unit_body_entered
        && guard.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && guard.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && guard.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && guard.unit_off_skipped == predecessor.unit_off_skipped
        && guard.non_cooling_skipped == predecessor.non_cooling_skipped
        && guard.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && guard.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && guard.predecessor_dehumidification_control_none_case_completed_skip
            == predecessor.dehumidification_control_none_case_completed_skip
        && guard
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && guard.predecessor_dehumidification_control_humidistat_case_completed_skip
            == predecessor.dehumidification_control_humidistat_case_completed_skip
        && guard
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
        && guard
            .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
            == predecessor
                .dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
        && guard.dehumidification_control_none_case_completed_skip
            == (route == PredecessorRoute::NoneCaseCompleted)
        && guard.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == (route == PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted)
        && guard.dehumidification_control_humidistat_case_completed_skip
            == (route == PredecessorRoute::HumidistatCaseCompleted)
        && guard.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
            == (route == PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted)
        && guard.heating_on_read == active
        && guard.heating_on == active.then_some(heating_on)
        && guard.cooling_supply_humidity_ratio_humidification_body_entered
            == (active && heating_on)
        && guard.heating_on_guard_false_fallthrough == (active && !heating_on)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_default_supply_humidity_ratio_case_break
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_default_supply_humidity_ratio_case_break_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact(
            retained,
            predecessor,
        )
        && cooling_default_supply_humidity_ratio_case_break_snapshots_match_exact(
            witness,
            predecessor,
        )
        && cooling_default_supply_humidity_ratio_case_break_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_default_supply_humidity_ratio_case_break_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn heating_on_provenance_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    predecessor: Predecessor,
    heating_on: bool,
) -> bool {
    let Some(entry) = unit.calc_entry.latest else {
        return false;
    };
    let Some(cp320) = unit.calc_cooling_humidification_flow.latest else {
        return false;
    };
    let Some(cp320_witness) = runtime.cooling_humidification_flow_latest_witness(predecessor.system)
    else {
        return false;
    };
    entry.system == predecessor.system
        && entry.call_ordinal == predecessor.parent_call_ordinal
        && entry.controlled_zone == predecessor.controlled_zone
        && entry.heating_availability_read_site_visited
        && entry.heating_on == heating_on
        && cp320.system == predecessor.system
        && cp320.parent_call_ordinal == predecessor.parent_call_ordinal
        && cp320.controlled_zone == predecessor.controlled_zone
        && cp320 == cp320_witness
        && cooling_humidification_flow_snapshot_is_exact_direct_release(cp320)
        && cooling_humidification_flow_snapshot_is_exact_direct_release(cp320_witness)
        && (!cp320.heating_on_read || cp320.heating_on == Some(heating_on))
        && (!is_active(predecessor_route(predecessor).unwrap_or(PredecessorRoute::UnitOff))
            || (cp320.heating_on_read
                && cp320.heating_on == Some(heating_on)
                && cp320.heating_on_body_entered == heating_on))
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    guard: Snapshot,
) -> bool {
    if !guard.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip {
        return true;
    }
    let Some(direct) = unit
        .calc_cooling_default_supply_humidity_ratio_case_break
        .latest
    else {
        return false;
    };
    private_default_supply_humidity_ratio_case_break_csh_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        predecessor,
    )
}

fn is_active(route: PredecessorRoute) -> bool {
    matches!(
        route,
        PredecessorRoute::NoneCaseCompleted
            | PredecessorRoute::ConstantSensibleHeatRatioCaseCompleted
            | PredecessorRoute::HumidistatCaseCompleted
            | PredecessorRoute::ConstantSupplyHumidityRatioCaseCompleted
    )
}