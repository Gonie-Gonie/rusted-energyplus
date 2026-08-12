//! Release-bound CP329 Cooling mixed-air call and no-OA fallback.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE,
    PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallActiveInput, PurchasedAirCalcCoolingMixedAirCallSnapshot,
    advance_cooling_mixed_air_call_state,
};
use crate::heat_balance::state::ZoneHeatBalanceState;
use crate::ideal_loads::{
    IdealLoadsSensibleMode, PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcMinimumOaPrefixSnapshot, PurchasedAirRuntimeState,
    classify_no_oa_sensible_subset,
    cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release,
    moist_air_enthalpy_j_per_kg,
};

mod runtime_validation;

pub(in crate::ideal_loads::calc) use runtime_validation::completed_direct_cooling_mixed_air_call_is_consistent;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::counter_product_matches;
use runtime_validation::{
    call_order_is_pending, committed_no_oa_humidity_owner_state_is_consistent,
    completed_mixed_air_predecessor_is_consistent, next_mixed_air_transition_fits,
    pending_mixed_air_history_links_to_predecessor, state_is_consistent,
};

/// Returns CP329's sealed same-call `PurchAir.MixedAirHumRat` value.
///
/// The capability proves only committed route, state, witness, and bit-exact
/// ownership. CP419 remains responsible for its own finite/range admission.
pub(in crate::ideal_loads::calc) fn cooling_mixed_air_call_committed_latest_mixed_air_humidity_ratio(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> Option<f64> {
    let state = &unit.calc_cooling_mixed_air_call;
    let latest = state.latest?;
    let retained = latest.mixed_air_humidity_ratio?;
    let source = latest.recirculation_humidity_ratio?;
    (committed_no_oa_humidity_owner_state_is_consistent(unit, witness)
        && committed_no_oa_humidity_owner_snapshot_has_exact_shape(latest)
        && retained.to_bits() == source.to_bits())
    .then_some(retained)
}

fn committed_no_oa_humidity_owner_snapshot_has_exact_shape(
    snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    snapshot.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && snapshot.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && snapshot.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && (snapshot.predecessor_zero_flow_reset_body_entered
            != snapshot.predecessor_active_guard_false_fallthrough)
        && snapshot.cooling_call_executed
        && active_snapshot_is_exact(snapshot)
}
#[cfg(test)]
pub(in crate::ideal_loads::calc) use runtime_validation::{
    next_mixed_air_transition_fits as next_mixed_air_transition_fits_for_test,
    pending_mixed_air_history_links_to_predecessor as pending_mixed_air_history_links_to_predecessor_for_test,
};

/// Active CP329 recirculation input rejected because it is not finite.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcCoolingMixedAirCallRecirculationInput {
    /// Controlled Zone mean air temperature.
    Temperature,
    /// Controlled Zone air humidity ratio.
    HumidityRatio,
    /// Coherent enthalpy projection derived from temperature and humidity ratio.
    EnthalpyProjection,
}

/// Fail-closed CP329 release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingMixedAirCallError {
    UnknownSystem {
        system: IdealLoadsAirSystemId,
    },
    InitializationNotReady {
        system: IdealLoadsAirSystemId,
    },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    ZoneIdentityMismatch {
        expected: ZoneId,
        actual: ZoneId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    CoolingSupplyMassFlowVerySmallGuardBodySnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    MinimumOutdoorAirLineageMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        minimum_oa_transition_count: usize,
        cooling_supply_mass_flow_very_small_guard_body_transition_count: usize,
        cooling_mixed_air_call_transition_count: usize,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    NonFiniteRecirculationState {
        system: IdealLoadsAirSystemId,
        input: PurchasedAirCalcCoolingMixedAirCallRecirculationInput,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

/// Executes CP329 for the exact direct no-OA release route.
///
/// The active wrapper reconstructs one coherent recirculation enthalpy
/// projection from the direct return projection's temperature and humidity
/// ratio. It does not claim ownership or parity for an independently stored
/// EnergyPlus `Node.Enthalpy`.
pub fn advance_direct_no_oa_calc_cooling_mixed_air_call(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor_cp328: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    zone_state: &ZoneHeatBalanceState,
) -> Result<PurchasedAirCalcCoolingMixedAirCallSnapshot, PurchasedAirCalcCoolingMixedAirCallError> {
    let selected = predecessor_cp328.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(PurchasedAirCalcCoolingMixedAirCallError::UnknownSystem { system: selected })?;
    let predecessor_witness =
        runtime.cooling_supply_mass_flow_very_small_guard_body_latest_witness(selected);
    let mixed_air_witness = runtime.cooling_mixed_air_call_latest_witness(selected);

    if system.id != selected {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::SystemIdentityMismatch {
                expected: selected,
                actual: system.id,
            },
        );
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::SystemOutsideDirectSubset {
                system: selected,
            },
        );
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::InitializationNotReady { system: selected },
        );
    }
    let expected_zone = unit.controlled_zone.ok_or(
        PurchasedAirCalcCoolingMixedAirCallError::InitializationNotReady { system: selected },
    )?;
    let recirculation_node = unit.recirculation_node.ok_or(
        PurchasedAirCalcCoolingMixedAirCallError::InitializationNotReady { system: selected },
    )?;
    if zone_state.zone_id != expected_zone {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::ZoneIdentityMismatch {
                expected: expected_zone,
                actual: zone_state.zone_id,
            },
        );
    }
    if predecessor_cp328.controlled_zone != expected_zone
        || !unit
            .calc_cooling_supply_mass_flow_very_small_guard_body
            .latest
            .is_some_and(|latest| {
                cooling_mixed_air_call_predecessors_match_bit_exact(latest, predecessor_cp328)
            })
        || !predecessor_witness.is_some_and(|witness| {
            cooling_mixed_air_call_predecessors_match_bit_exact(witness, predecessor_cp328)
        })
    {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::
                CoolingSupplyMassFlowVerySmallGuardBodySnapshotMismatch { system: selected },
        );
    }
    if !cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(
        predecessor_cp328,
    ) {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::PredecessorOutsideDirectSubset {
                system: selected,
            },
        );
    }
    let minimum_oa = unit.calc_minimum_oa_prefix.latest.ok_or(
        PurchasedAirCalcCoolingMixedAirCallError::MinimumOutdoorAirLineageMismatch {
            system: selected,
        },
    )?;
    if !minimum_oa_links_to_predecessor(minimum_oa, predecessor_cp328) {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::MinimumOutdoorAirLineageMismatch {
                system: selected,
            },
        );
    }
    if !call_order_is_pending(unit, predecessor_cp328)
        || predecessor_cp328.parent_call_ordinal != unit.init_call_count
    {
        return Err(call_order_error(unit, selected));
    }
    if !pending_mixed_air_history_links_to_predecessor(unit, predecessor_cp328)
        || !state_is_consistent(
            &unit.calc_cooling_mixed_air_call,
            mixed_air_witness,
            selected,
        )
        || !next_mixed_air_transition_fits(&unit.calc_cooling_mixed_air_call, predecessor_cp328)
        || !completed_mixed_air_predecessor_is_consistent(runtime, unit, system, predecessor_cp328)
    {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::RuntimeStateInvariantViolation {
                system: selected,
            },
        );
    }

    let active_input = if predecessor_cp328.cooling_body_entered {
        let recirculation_temperature_c = zone_state.mean_air_temperature_c;
        let recirculation_humidity_ratio = zone_state.air_humidity_ratio;
        let recirculation_enthalpy_projection_j_per_kg =
            validate_recirculation_state_and_project_enthalpy(
                selected,
                recirculation_temperature_c,
                recirculation_humidity_ratio,
            )?;
        let supply_mass_flow_rate_kg_per_s = predecessor_cp328
            .resulting_supply_mass_flow_rate_kg_per_s
            .ok_or(
                PurchasedAirCalcCoolingMixedAirCallError::RuntimeStateInvariantViolation {
                    system: selected,
                },
            )?;
        let outdoor_air_mass_flow_rate_kg_per_s = minimum_oa
            .working_outdoor_air_mass_flow_rate_kg_per_s
            .ok_or(
                PurchasedAirCalcCoolingMixedAirCallError::MinimumOutdoorAirLineageMismatch {
                    system: selected,
                },
            )?;
        Some(PurchasedAirCalcCoolingMixedAirCallActiveInput {
            recirculation_node,
            recirculation_temperature_c,
            recirculation_humidity_ratio,
            recirculation_enthalpy_projection_j_per_kg,
            outdoor_air_mass_flow_rate_kg_per_s,
            supply_mass_flow_rate_kg_per_s,
        })
    } else {
        None
    };

    let snapshot = {
        let unit = runtime
            .units
            .get_mut(&selected)
            .ok_or(PurchasedAirCalcCoolingMixedAirCallError::UnknownSystem { system: selected })?;
        advance_cooling_mixed_air_call_state(
            &mut unit.calc_cooling_mixed_air_call,
            predecessor_cp328,
            active_input,
        )
    };
    runtime.set_cooling_mixed_air_call_latest_witness(selected, snapshot);
    debug_assert!(cooling_mixed_air_call_snapshot_is_exact_direct_release(
        snapshot
    ));
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime.cooling_mixed_air_call_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}

fn mixed_air_call_links_to_predecessor(
    call: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let common = call.system == predecessor.system
        && call.parent_call_ordinal == predecessor.parent_call_ordinal
        && call.controlled_zone == predecessor.controlled_zone
        && call.unit_body_entered == predecessor.unit_body_entered
        && call.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && call.predecessor_zero_flow_reset_body_entered
            == predecessor.zero_flow_reset_body_entered
        && call.predecessor_active_guard_false_fallthrough
            == predecessor.active_guard_false_fallthrough
        && call.unit_off_skipped == predecessor.unit_off_skipped
        && call.non_cooling_skipped == predecessor.non_cooling_skipped
        && call.cooling_call_executed == predecessor.cooling_body_entered;
    common
        && if predecessor.cooling_body_entered {
            option_bits_match(
                call.supply_mass_flow_rate_kg_per_s,
                predecessor.resulting_supply_mass_flow_rate_kg_per_s,
            )
        } else {
            call.supply_mass_flow_rate_kg_per_s.is_none()
                && predecessor
                    .resulting_supply_mass_flow_rate_kg_per_s
                    .is_none()
        }
}

fn validate_recirculation_state_and_project_enthalpy(
    system: IdealLoadsAirSystemId,
    temperature_c: f64,
    humidity_ratio: f64,
) -> Result<f64, PurchasedAirCalcCoolingMixedAirCallError> {
    for (input, value) in [
        (
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::Temperature,
            temperature_c,
        ),
        (
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::HumidityRatio,
            humidity_ratio,
        ),
    ] {
        if !value.is_finite() {
            return Err(
                PurchasedAirCalcCoolingMixedAirCallError::NonFiniteRecirculationState {
                    system,
                    input,
                },
            );
        }
    }
    let enthalpy_projection_j_per_kg = moist_air_enthalpy_j_per_kg(temperature_c, humidity_ratio);
    if !enthalpy_projection_j_per_kg.is_finite() {
        return Err(
            PurchasedAirCalcCoolingMixedAirCallError::NonFiniteRecirculationState {
                system,
                input: PurchasedAirCalcCoolingMixedAirCallRecirculationInput::EnthalpyProjection,
            },
        );
    }
    Ok(enthalpy_projection_j_per_kg)
}

fn call_order_error(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: IdealLoadsAirSystemId,
) -> PurchasedAirCalcCoolingMixedAirCallError {
    PurchasedAirCalcCoolingMixedAirCallError::PredecessorCallOrder {
        system,
        init_call_count: unit.init_call_count,
        calculation_entry_call_count: unit.calc_entry.call_count,
        minimum_oa_transition_count: unit.calc_minimum_oa_prefix.transition_count,
        cooling_supply_mass_flow_very_small_guard_body_transition_count: unit
            .calc_cooling_supply_mass_flow_very_small_guard_body
            .transition_count,
        cooling_mixed_air_call_transition_count: unit.calc_cooling_mixed_air_call.transition_count,
    }
}

fn minimum_oa_links_to_predecessor(
    minimum_oa: PurchasedAirCalcMinimumOaPrefixSnapshot,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let common = minimum_oa.system == predecessor.system
        && minimum_oa.parent_call_ordinal == predecessor.parent_call_ordinal
        && minimum_oa.controlled_zone == predecessor.controlled_zone
        && minimum_oa.unit_body_entered == predecessor.unit_body_entered;
    if !common {
        return false;
    }
    if predecessor.unit_body_entered {
        minimum_oa.minimum_oa_child_called
            && minimum_oa.minimum_oa_child_no_outdoor_air_route
            && minimum_oa.retained_minimum_outdoor_air_write_performed
            && option_is_positive_zero(
                minimum_oa.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
            )
            && minimum_oa.ems_override_flag_read
            && minimum_oa.ems_override_enabled == Some(false)
            && !minimum_oa.ems_override_applied
            && option_is_positive_zero(minimum_oa.working_outdoor_air_mass_flow_rate_kg_per_s)
            && minimum_oa.outdoor_air_flag_read
            && minimum_oa.outdoor_air_enabled == Some(false)
            && minimum_oa.no_outdoor_air_zero_branch_entered
            && minimum_oa.psychrometric_call_count == 0
    } else {
        !minimum_oa.minimum_oa_child_called
            && !minimum_oa.minimum_oa_child_no_outdoor_air_route
            && minimum_oa
                .retained_minimum_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !minimum_oa.retained_minimum_outdoor_air_write_performed
            && !minimum_oa.ems_override_flag_read
            && minimum_oa.ems_override_enabled.is_none()
            && !minimum_oa.ems_override_applied
            && minimum_oa
                .working_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !minimum_oa.outdoor_air_flag_read
            && minimum_oa.outdoor_air_enabled.is_none()
            && !minimum_oa.no_outdoor_air_zero_branch_entered
            && minimum_oa.psychrometric_call_count == 0
    }
}

pub(in crate::ideal_loads) fn cooling_mixed_air_call_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE
        && snapshot.child_source == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_CHILD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_SOURCE_ORDER
        && snapshot.no_oa_child_source_order
            == PURCHASED_AIR_CALC_COOLING_MIXED_AIR_CALL_NO_OA_CHILD_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_zero_flow_reset_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.cooling_call_executed;
    let non_cooling = !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_zero_flow_reset_body_entered
        && !snapshot.predecessor_active_guard_false_fallthrough
        && !snapshot.cooling_call_executed;
    let active = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && (snapshot.predecessor_zero_flow_reset_body_entered
            != snapshot.predecessor_active_guard_false_fallthrough)
        && snapshot.cooling_call_executed;
    provenance
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(active) == 1
        && if active {
            active_snapshot_is_exact(snapshot)
        } else {
            skipped_snapshot_is_exact(snapshot)
        }
}

fn active_snapshot_is_exact(snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot) -> bool {
    let Some(outdoor_air_mass_flow) = snapshot.outdoor_air_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(supply_mass_flow) = snapshot.supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(child_supply_mass_flow) = snapshot.child_supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(recirculation_mass_flow) = snapshot.resulting_recirculation_mass_flow_rate_kg_per_s
    else {
        return false;
    };
    let Some(recirculation_temperature) = snapshot.recirculation_temperature_c else {
        return false;
    };
    let Some(recirculation_humidity) = snapshot.recirculation_humidity_ratio else {
        return false;
    };
    let Some(recirculation_enthalpy) = snapshot.recirculation_enthalpy_projection_j_per_kg else {
        return false;
    };
    snapshot.state_reference_bound
        && snapshot.purchased_air_number_read
        && snapshot.outdoor_air_mass_flow_rate_read
        && outdoor_air_mass_flow.to_bits() == 0
        && snapshot.supply_mass_flow_rate_read
        && snapshot.mixed_air_temperature_output_reference_bound
        && snapshot.mixed_air_humidity_ratio_output_reference_bound
        && snapshot.mixed_air_enthalpy_output_reference_bound
        && snapshot.operating_mode_read
        && snapshot.operating_mode == Some(IdealLoadsSensibleMode::Cooling)
        && snapshot.calc_purch_air_mixed_air_called
        && snapshot.purchased_air_alias_bound
        && snapshot.outdoor_air_node_number_copied
        && snapshot.outdoor_air_node.is_none()
        && snapshot.recirculation_node_number_copied
        && snapshot.recirculation_node.is_some()
        && snapshot.recirculation_mass_flow_rate_initialized
        && option_is_positive_zero(snapshot.initial_recirculation_mass_flow_rate_kg_per_s)
        && snapshot.recirculation_temperature_read
        && snapshot.recirculation_humidity_ratio_read
        && snapshot.recirculation_enthalpy_projection_read
        && recirculation_temperature.is_finite()
        && recirculation_humidity.is_finite()
        && recirculation_enthalpy.is_finite()
        && recirculation_enthalpy.to_bits()
            == moist_air_enthalpy_j_per_kg(recirculation_temperature, recirculation_humidity)
                .to_bits()
        && snapshot.outdoor_air_initialization_guard_evaluated
        && snapshot.outdoor_air_enabled == Some(false)
        && [
            snapshot.outdoor_air_inlet_temperature_c,
            snapshot.outdoor_air_inlet_humidity_ratio,
            snapshot.outdoor_air_inlet_enthalpy_j_per_kg,
            snapshot.outdoor_air_after_heat_recovery_temperature_c,
            snapshot.outdoor_air_after_heat_recovery_humidity_ratio,
            snapshot.outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
        ]
        .into_iter()
        .all(option_is_positive_zero)
        && snapshot.heat_recovery_on_false_assigned
        && snapshot.heat_recovery_on == Some(false)
        && snapshot.outdoor_air_active_guard_first_operand_evaluated
        && !snapshot.outdoor_air_mass_flow_positive_comparison_evaluated
        && snapshot.no_outdoor_air_fallback_entered
        && snapshot.child_supply_mass_flow_rate_read
        && child_supply_mass_flow.to_bits() == supply_mass_flow.to_bits()
        && snapshot.recirculation_mass_flow_rate_assigned_from_supply
        && recirculation_mass_flow.to_bits() == supply_mass_flow.to_bits()
        && snapshot.mixed_air_temperature_assigned
        && option_bits_match(
            snapshot.mixed_air_temperature_c,
            Some(recirculation_temperature),
        )
        && snapshot.mixed_air_humidity_ratio_assigned
        && option_bits_match(
            snapshot.mixed_air_humidity_ratio,
            Some(recirculation_humidity),
        )
        && snapshot.mixed_air_enthalpy_projection_assigned
        && option_bits_match(
            snapshot.mixed_air_enthalpy_projection_j_per_kg,
            Some(recirculation_enthalpy),
        )
        && snapshot.heat_recovery_sensible_output_positive_zero_assigned
        && option_is_positive_zero(snapshot.heat_recovery_sensible_output_w)
        && snapshot.heat_recovery_latent_output_positive_zero_assigned
        && option_is_positive_zero(snapshot.heat_recovery_latent_output_w)
}

fn skipped_snapshot_is_exact(snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot) -> bool {
    !snapshot.state_reference_bound
        && !snapshot.purchased_air_number_read
        && !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.mixed_air_temperature_output_reference_bound
        && !snapshot.mixed_air_humidity_ratio_output_reference_bound
        && !snapshot.mixed_air_enthalpy_output_reference_bound
        && !snapshot.operating_mode_read
        && snapshot.operating_mode.is_none()
        && !snapshot.calc_purch_air_mixed_air_called
        && !snapshot.purchased_air_alias_bound
        && !snapshot.outdoor_air_node_number_copied
        && snapshot.outdoor_air_node.is_none()
        && !snapshot.recirculation_node_number_copied
        && snapshot.recirculation_node.is_none()
        && !snapshot.recirculation_mass_flow_rate_initialized
        && snapshot
            .initial_recirculation_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.recirculation_temperature_read
        && snapshot.recirculation_temperature_c.is_none()
        && !snapshot.recirculation_humidity_ratio_read
        && snapshot.recirculation_humidity_ratio.is_none()
        && !snapshot.recirculation_enthalpy_projection_read
        && snapshot
            .recirculation_enthalpy_projection_j_per_kg
            .is_none()
        && !snapshot.outdoor_air_initialization_guard_evaluated
        && snapshot.outdoor_air_enabled.is_none()
        && snapshot.outdoor_air_inlet_temperature_c.is_none()
        && snapshot.outdoor_air_inlet_humidity_ratio.is_none()
        && snapshot.outdoor_air_inlet_enthalpy_j_per_kg.is_none()
        && snapshot
            .outdoor_air_after_heat_recovery_temperature_c
            .is_none()
        && snapshot
            .outdoor_air_after_heat_recovery_humidity_ratio
            .is_none()
        && snapshot
            .outdoor_air_after_heat_recovery_enthalpy_j_per_kg
            .is_none()
        && !snapshot.heat_recovery_on_false_assigned
        && snapshot.heat_recovery_on.is_none()
        && !snapshot.outdoor_air_active_guard_first_operand_evaluated
        && !snapshot.outdoor_air_mass_flow_positive_comparison_evaluated
        && !snapshot.no_outdoor_air_fallback_entered
        && !snapshot.child_supply_mass_flow_rate_read
        && snapshot.child_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.recirculation_mass_flow_rate_assigned_from_supply
        && snapshot
            .resulting_recirculation_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.mixed_air_temperature_assigned
        && snapshot.mixed_air_temperature_c.is_none()
        && !snapshot.mixed_air_humidity_ratio_assigned
        && snapshot.mixed_air_humidity_ratio.is_none()
        && !snapshot.mixed_air_enthalpy_projection_assigned
        && snapshot.mixed_air_enthalpy_projection_j_per_kg.is_none()
        && !snapshot.heat_recovery_sensible_output_positive_zero_assigned
        && snapshot.heat_recovery_sensible_output_w.is_none()
        && !snapshot.heat_recovery_latent_output_positive_zero_assigned
        && snapshot.heat_recovery_latent_output_w.is_none()
}

pub(in crate::ideal_loads) fn cooling_mixed_air_call_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    mut right: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> bool {
    macro_rules! option_bits {
        ($field:ident) => {
            option_bits_match(left.$field, right.$field)
        };
    }
    let float_fields_match = option_bits!(outdoor_air_mass_flow_rate_kg_per_s)
        && option_bits!(supply_mass_flow_rate_kg_per_s)
        && option_bits!(initial_recirculation_mass_flow_rate_kg_per_s)
        && option_bits!(recirculation_temperature_c)
        && option_bits!(recirculation_humidity_ratio)
        && option_bits!(recirculation_enthalpy_projection_j_per_kg)
        && option_bits!(outdoor_air_inlet_temperature_c)
        && option_bits!(outdoor_air_inlet_humidity_ratio)
        && option_bits!(outdoor_air_inlet_enthalpy_j_per_kg)
        && option_bits!(outdoor_air_after_heat_recovery_temperature_c)
        && option_bits!(outdoor_air_after_heat_recovery_humidity_ratio)
        && option_bits!(outdoor_air_after_heat_recovery_enthalpy_j_per_kg)
        && option_bits!(child_supply_mass_flow_rate_kg_per_s)
        && option_bits!(resulting_recirculation_mass_flow_rate_kg_per_s)
        && option_bits!(mixed_air_temperature_c)
        && option_bits!(mixed_air_humidity_ratio)
        && option_bits!(mixed_air_enthalpy_projection_j_per_kg)
        && option_bits!(heat_recovery_sensible_output_w)
        && option_bits!(heat_recovery_latent_output_w);
    macro_rules! clear {
        ($snapshot:ident, $($field:ident),+ $(,)?) => {
            $(
                $snapshot.$field = None;
            )+
        };
    }
    clear!(
        left,
        outdoor_air_mass_flow_rate_kg_per_s,
        supply_mass_flow_rate_kg_per_s,
        initial_recirculation_mass_flow_rate_kg_per_s,
        recirculation_temperature_c,
        recirculation_humidity_ratio,
        recirculation_enthalpy_projection_j_per_kg,
        outdoor_air_inlet_temperature_c,
        outdoor_air_inlet_humidity_ratio,
        outdoor_air_inlet_enthalpy_j_per_kg,
        outdoor_air_after_heat_recovery_temperature_c,
        outdoor_air_after_heat_recovery_humidity_ratio,
        outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
        child_supply_mass_flow_rate_kg_per_s,
        resulting_recirculation_mass_flow_rate_kg_per_s,
        mixed_air_temperature_c,
        mixed_air_humidity_ratio,
        mixed_air_enthalpy_projection_j_per_kg,
        heat_recovery_sensible_output_w,
        heat_recovery_latent_output_w,
    );
    clear!(
        right,
        outdoor_air_mass_flow_rate_kg_per_s,
        supply_mass_flow_rate_kg_per_s,
        initial_recirculation_mass_flow_rate_kg_per_s,
        recirculation_temperature_c,
        recirculation_humidity_ratio,
        recirculation_enthalpy_projection_j_per_kg,
        outdoor_air_inlet_temperature_c,
        outdoor_air_inlet_humidity_ratio,
        outdoor_air_inlet_enthalpy_j_per_kg,
        outdoor_air_after_heat_recovery_temperature_c,
        outdoor_air_after_heat_recovery_humidity_ratio,
        outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
        child_supply_mass_flow_rate_kg_per_s,
        resulting_recirculation_mass_flow_rate_kg_per_s,
        mixed_air_temperature_c,
        mixed_air_humidity_ratio,
        mixed_air_enthalpy_projection_j_per_kg,
        heat_recovery_sensible_output_w,
        heat_recovery_latent_output_w,
    );
    float_fields_match && left == right
}

fn cooling_mixed_air_call_predecessors_match_bit_exact(
    mut left: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    mut right: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.predecessor_supply_mass_flow_rate_kg_per_s,
        right.predecessor_supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.assigned_supply_mass_flow_rate_kg_per_s,
        right.assigned_supply_mass_flow_rate_kg_per_s,
    ) && option_bits_match(
        left.resulting_supply_mass_flow_rate_kg_per_s,
        right.resulting_supply_mass_flow_rate_kg_per_s,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_supply_mass_flow_rate_kg_per_s = None;
        snapshot.assigned_supply_mass_flow_rate_kg_per_s = None;
        snapshot.resulting_supply_mass_flow_rate_kg_per_s = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn option_is_positive_zero(value: Option<f64>) -> bool {
    value.is_some_and(|value| value.to_bits() == 0)
}
