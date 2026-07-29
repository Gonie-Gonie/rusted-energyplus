//! CP357-to-CP358 retained/private Humidistat-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingHumidistatCaseEntryRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_constant_shr_case_break::private_humidistat_counterfactual_links_to_direct_release as cp357_private_humidistat_counterfactual_links_to_direct_release;
use crate::ideal_loads::calc::cooling_humidistat_case_entry::transition::predecessor_route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot as Predecessor, PurchasedAirRuntimeState,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
        && case_entry.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && case_entry
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && case_entry.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && case_entry.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == (route == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip)
        && case_entry.dehumidification_control_humidistat_case_entered
            == (route == Route::DehumidificationControlHumidistatCaseEntered)
        && case_entry.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == (route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    case_entry: Snapshot,
) -> bool {
    if !case_entry.dehumidification_control_humidistat_case_entered {
        return true;
    }
    let Some(direct) = unit.calc_cooling_constant_shr_case_break.latest else {
        return false;
    };
    private_humidistat_predecessor_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        predecessor,
    )
}

pub(in crate::ideal_loads::calc) fn private_humidistat_predecessor_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Predecessor,
    private_humidistat: Predecessor,
) -> bool {
    cp357_private_humidistat_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        private_humidistat,
    )
}
