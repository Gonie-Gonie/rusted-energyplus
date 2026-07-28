//! CP347-to-CP348 retained-lineage validation.

use super::super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot;
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry::transition::{
    predecessor_route,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRetainedRoute as Route;
use crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot;

pub(super) fn case_entry_links_to_predecessor(
    case_entry:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
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
        && case_entry.predecessor_dehumidification_control_none_case_completed
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && case_entry.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && case_entry.dehumidification_control_constant_sensible_heat_ratio_case_entered
            == (route == Route::DehumidificationControlConstantSensibleHeatRatioCaseEntered)
        && case_entry.dehumidification_control_humidistat_case_selected_skip
            == (route == Route::DehumidificationControlHumidistatCaseSelectedSkip)
        && case_entry.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == (route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
}
