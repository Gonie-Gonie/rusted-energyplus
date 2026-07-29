//! CP356-to-CP357 retained-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingConstantShrCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_constant_shr_case_break::transition::predecessor_route;
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::private_active_counterfactual_links_to_direct_release as cp356_private_active_counterfactual_links_to_direct_release;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

pub(super) fn case_break_links_to_predecessor(
    case_break: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    case_break.system == predecessor.system
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed
        && case_break.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && case_break
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && case_break.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && case_break
            .dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
            == (route == Route::DehumidificationControlConstantSensibleHeatRatioCaseBreak)
        && case_break.dehumidification_control_humidistat_case_selected_skip
            == (route == Route::DehumidificationControlHumidistatCaseSelectedSkip)
        && case_break.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == (route
                == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
}

pub(super) fn active_lineage_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    case_break: Snapshot,
) -> bool {
    if !case_break.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break {
        return true;
    }
    let Some(direct) = unit
        .calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit
        .latest
    else {
        return false;
    };
    private_active_predecessor_links_to_direct_release(runtime, unit, system, direct, predecessor)
}

pub(in crate::ideal_loads::calc) fn private_active_predecessor_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Predecessor,
    private_active: Predecessor,
) -> bool {
    cp356_private_active_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        private_active,
    )
}
