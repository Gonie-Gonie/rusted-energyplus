//! Canonical private CP369 ConstantSupplyHumidityRatio heating-on reconstruction.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Snapshot,
    advance_cooling_supply_humidity_ratio_humidification_heating_availability_guard_state as advance,
};
use super::prefix_validation::{active_lineage_is_exact, guard_links_to_predecessor};
use super::snapshot_validation::{snapshot_route, snapshots_match_exact};
use crate::ideal_loads::calc::cooling_default_supply_humidity_ratio_case_break::private_default_supply_humidity_ratio_case_break_csh_counterfactual_from_direct_release as cp368_private_csh_from_direct_release;
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::HeatingAvailabilityBodyEntered)
        || !direct.dehumidification_control_none_case_completed_skip
        || direct.heating_on != Some(true)
        || !snapshots_match_exact(retained, direct)
        || !snapshots_match_exact(witness, direct)
        || !super::completed_direct_cooling_supply_humidity_ratio_humidification_heating_availability_guard_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let direct_cp368 = unit
        .calc_cooling_default_supply_humidity_ratio_case_break
        .latest?;
    let private_cp368 = cp368_private_csh_from_direct_release(
        runtime,
        unit,
        system,
        direct_cp368,
    )?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp368, true)?;
    (snapshot_route(counterfactual) == Some(Route::HeatingAvailabilityBodyEntered)
        && counterfactual
            .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
        && counterfactual.heating_on == Some(true)
        && guard_links_to_predecessor(counterfactual, private_cp368, true)
        && active_lineage_is_exact(runtime, unit, system, private_cp368, counterfactual)
        && route_independent_identity_matches(direct, counterfactual))
    .then_some(counterfactual)
}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_from_direct_release(
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
        && direct.heating_on_read == counterfactual.heating_on_read
        && direct.heating_on == counterfactual.heating_on
        && direct.cooling_supply_humidity_ratio_humidification_body_entered
            == counterfactual.cooling_supply_humidity_ratio_humidification_body_entered
        && direct.heating_on_guard_false_fallthrough
            == counterfactual.heating_on_guard_false_fallthrough
}