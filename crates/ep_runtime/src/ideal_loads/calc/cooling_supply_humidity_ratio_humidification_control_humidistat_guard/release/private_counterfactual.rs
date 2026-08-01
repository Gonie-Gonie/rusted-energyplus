//! Canonical private-Humidistat CP370 reconstruction.

use ep_model::{HumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state as advance,
};
use super::prefix_validation::guard_links_to_predecessor;
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::HumidificationControlGuardFalseFallthrough)
        || direct.humidification_control_type != Some(HumidificationControlType::None)
        || !snapshots_match_exact(retained, direct)
        || !snapshots_match_exact(witness, direct)
        || !super::completed_direct_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let predecessor = unit
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
        .latest?;
    let mut state = State::new(system.id);
    let counterfactual = advance(
        &mut state,
        predecessor,
        HumidificationControlType::Humidistat,
    )?;
    (snapshot_route(counterfactual) == Some(Route::HumidificationControlBodyEntered)
        && counterfactual.humidification_control_type
            == Some(HumidificationControlType::Humidistat)
        && counterfactual.humidification_control_type_humidistat == Some(true)
        && counterfactual.humidification_control_body_entered
        && !counterfactual.humidification_control_guard_false_fallthrough
        && guard_links_to_predecessor(
            counterfactual,
            predecessor,
            HumidificationControlType::Humidistat,
        )
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_counterfactual_from_direct_release(
        runtime, unit, system, direct,
    )
    .is_some_and(|expected| snapshots_match_exact(expected, counterfactual))
}

#[allow(dead_code)]
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
        && direct.predecessor_dehumidification_control_none_case_completed_skip
            == counterfactual.predecessor_dehumidification_control_none_case_completed_skip
        && direct
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
            == counterfactual
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip
        && direct.predecessor_dehumidification_control_humidistat_case_completed_skip
            == counterfactual.predecessor_dehumidification_control_humidistat_case_completed_skip
        && direct
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
            == counterfactual
                .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
        && direct
            .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
            == counterfactual
                .predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break
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
        && direct.predecessor_heating_on_read == counterfactual.predecessor_heating_on_read
        && direct.predecessor_heating_on == counterfactual.predecessor_heating_on
        && direct.predecessor_cooling_supply_humidity_ratio_humidification_body_entered
            == counterfactual
                .predecessor_cooling_supply_humidity_ratio_humidification_body_entered
        && direct.predecessor_heating_on_guard_false_fallthrough
            == counterfactual.predecessor_heating_on_guard_false_fallthrough
}
