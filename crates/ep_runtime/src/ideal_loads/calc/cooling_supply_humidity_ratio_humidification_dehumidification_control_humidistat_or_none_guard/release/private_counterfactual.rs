//! Canonical private CP370-Humidistat/selected-None CP371 reconstruction.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state as advance,
};
use super::snapshot_validation::{predecessor_snapshot, snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || system.dehumidification_control_type != DehumidificationControlType::None
        || snapshot_route(direct) != Some(Route::HumidificationControlGuardFalseFallthrough)
        || !snapshots_match_exact(retained, direct)
        || !snapshots_match_exact(witness, direct)
        || !super::completed_direct_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp370 = predecessor_snapshot(direct);
    let private_cp370 = private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp370,
    )?;
    let mut state = State::new(system.id);
    let counterfactual = advance(
        &mut state,
        private_cp370,
        system.dehumidification_control_type,
    )?;
    (snapshot_route(counterfactual) == Some(Route::DehumidificationControlNoneBodyEntered)
        && counterfactual.dehumidification_control_type_first_read
        && counterfactual.first_dehumidification_control_type
            == Some(DehumidificationControlType::None)
        && counterfactual.dehumidification_control_type_humidistat == Some(false)
        && counterfactual.dehumidification_control_type_second_read
        && counterfactual.second_dehumidification_control_type
            == Some(DehumidificationControlType::None)
        && counterfactual.dehumidification_control_type_none == Some(true)
        && counterfactual.dehumidification_control_body_entered
        && !counterfactual.dehumidification_control_guard_false_fallthrough
        && state.source_site_execution_count == 5
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release(
        runtime, unit, system, direct,
    )
    .is_some_and(|expected| snapshots_match_exact(expected, counterfactual))
}

fn route_independent_identity_matches(direct: Snapshot, counterfactual: Snapshot) -> bool {
    direct.source == counterfactual.source
        && direct.first_excluded_source == counterfactual.first_excluded_source
        && direct.source_order == counterfactual.source_order
        && direct.system == counterfactual.system
        && direct.parent_call_ordinal == counterfactual.parent_call_ordinal
        && direct.controlled_zone == counterfactual.controlled_zone
        && direct.unit_body_entered == counterfactual.unit_body_entered
        && direct.predecessor_cooling_body_entered
            == counterfactual.predecessor_cooling_body_entered
        && direct.predecessor_no_outdoor_air_fallback_entered
            == counterfactual.predecessor_no_outdoor_air_fallback_entered
        && direct.predecessor_positive_supply_mass_flow_body_entered
            == counterfactual.predecessor_positive_supply_mass_flow_body_entered
        && direct.unit_off_skipped == counterfactual.unit_off_skipped
        && direct.non_cooling_skipped == counterfactual.non_cooling_skipped
        && direct.positive_guard_false_fallthrough_skipped
            == counterfactual.positive_guard_false_fallthrough_skipped
        && direct.predecessor_dehumidification_control_type
            == counterfactual.predecessor_dehumidification_control_type
        && direct.dehumidification_control_none_case_completed_skip
            == counterfactual.dehumidification_control_none_case_completed_skip
        && direct.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == counterfactual
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && direct.dehumidification_control_humidistat_case_completed_skip
            == counterfactual.dehumidification_control_humidistat_case_completed_skip
        && direct.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
            == counterfactual
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
        && direct.predecessor_heating_on == counterfactual.predecessor_heating_on
}
