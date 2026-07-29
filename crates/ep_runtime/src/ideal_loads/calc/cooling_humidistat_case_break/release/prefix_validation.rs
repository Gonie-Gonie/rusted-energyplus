//! CP362-to-CP363 retained-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingHumidistatCaseBreakRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot as Snapshot,
};
use crate::ideal_loads::calc::cooling_humidistat_case_break::transition::predecessor_route;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit::private_humidistat_counterfactual_links_to_direct_release as cp362_private_humidistat_counterfactual_links_to_direct_release;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Predecessor,
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
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == predecessor
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && case_break
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed
            == predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_executed
        && case_break
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && case_break.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && case_break.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == (route
                == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip)
        && case_break.dehumidification_control_humidistat_case_exited_via_break
            == (route == Route::DehumidificationControlHumidistatCaseBreak)
        && case_break.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == (route
                == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
}

pub(super) fn private_humidistat_predecessor_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Predecessor,
    private_humidistat: Predecessor,
    pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64,
    pre_sampled_zone_node_humidity_ratio: f64,
) -> bool {
    cp362_private_humidistat_counterfactual_links_to_direct_release(
        runtime,
        unit,
        system,
        direct,
        private_humidistat,
        pre_sampled_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        pre_sampled_zone_node_humidity_ratio,
    )
}
