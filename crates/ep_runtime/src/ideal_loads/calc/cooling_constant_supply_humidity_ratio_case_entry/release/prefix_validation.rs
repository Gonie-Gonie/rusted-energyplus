//! CP363-to-CP364 retained-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_constant_supply_humidity_ratio_case_entry::transition::predecessor_route;
use crate::ideal_loads::calc::cooling_humidistat_case_break::private_constant_supply_humidity_ratio_counterfactual_links_to_direct_release as cp363_private_constant_supply_counterfactual_links_to_direct_release;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot as Predecessor, PurchasedAirRuntimeState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn case_entry_links_to_predecessor(
    case_entry: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    case_entry.system == predecessor.system
        && case_entry.parent_call_ordinal == predecessor.parent_call_ordinal
        && case_entry.controlled_zone == predecessor.controlled_zone
        && case_entry.unit_body_entered == predecessor.unit_body_entered
        && case_entry.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && case_entry.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && case_entry.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && case_entry.unit_off_skipped == predecessor.unit_off_skipped
        && case_entry.non_cooling_skipped == predecessor.non_cooling_skipped
        && case_entry.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && case_entry.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && case_entry.predecessor_dehumidification_control_none_case_completed_skip
            == predecessor.dehumidification_control_none_case_completed_skip
        && case_entry
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && case_entry.predecessor_dehumidification_control_humidistat_case_exited_via_break
            == predecessor.dehumidification_control_humidistat_case_exited_via_break
        && case_entry
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && case_entry.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && case_entry.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == (route == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip)
        && case_entry.dehumidification_control_humidistat_case_completed_skip
            == (route == Route::DehumidificationControlHumidistatCaseCompletedSkip)
        && case_entry.dehumidification_control_constant_supply_humidity_ratio_case_entered
            == (route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseEntered)
}

pub(in crate::ideal_loads) fn cooling_constant_supply_humidity_ratio_case_entry_snapshot_links_to_predecessor(
    case_entry: Snapshot,
    predecessor: Predecessor,
) -> bool {
    case_entry_links_to_predecessor(case_entry, predecessor)
}

pub(super) fn private_constant_supply_predecessor_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Predecessor,
    private_constant_supply: Predecessor,
) -> bool {
    cp363_private_constant_supply_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        private_constant_supply,
    )
}
