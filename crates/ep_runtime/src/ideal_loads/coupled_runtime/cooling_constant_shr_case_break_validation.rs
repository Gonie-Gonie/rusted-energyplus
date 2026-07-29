//! Release validation for the bounded constant-SHR case break.

use ep_model::DehumidificationControlType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState,
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    cooling_constant_shr_case_break_snapshot_is_exact_direct_release,
    cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit;
    let snapshot = output.calculation_cooling_constant_shr_case_break;
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && cooling_constant_shr_case_break_snapshot_is_exact_direct_release(snapshot)
        && snapshots_match_exact(&snapshot, &expected_snapshot(predecessor))
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary,
    predecessor_lifecycle:
        &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    timestep_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    validate_counts(state, predecessor, timestep_count)?;

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    let predecessor_latest = predecessor
        .latest
        .as_ref()
        .ok_or_else(|| violation("predecessor_latest_release_snapshot_ready", 1, 0))?;
    // The caller validates CP356 recursively first. CP357 compares that full
    // latest snapshot to the scheduled CP356 witness without re-querying any
    // CP355 or CP329 owner.
    if binding.system.dehumidification_control_type != DehumidificationControlType::None
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE
        || predecessor_lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE
        || predecessor_lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER.len() != 1
        || state.system != binding.ideal_loads_air_system
        || predecessor.system != binding.ideal_loads_air_system
        || !cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            *predecessor_latest,
        )
        || !super::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_validation::snapshots_match_exact_bits(
            predecessor_latest,
            &latest_output
                .calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
        )
        || !cooling_constant_shr_case_break_snapshot_is_exact_direct_release(*latest)
        || !snapshots_match_exact(latest, &expected_snapshot(*predecessor_latest))
        || !snapshots_match_exact(
            latest,
            &latest_output.calculation_cooling_constant_shr_case_break,
        )
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

fn validate_counts(
    state: &PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState,
    predecessor: &PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    timestep_count: usize,
) -> Result<(), Error> {
    let executed = state.dehumidification_control_constant_sensible_heat_ratio_case_break_count;
    validate_route_partition(state)?;
    validate_source_counters(state)?;
    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "positive_guard_false_fallthrough_skip_count",
            predecessor.positive_guard_false_fallthrough_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
        ),
        (
            "none_case_completed_skip_count",
            predecessor.dehumidification_control_none_case_completed_skip_count,
            state.dehumidification_control_none_case_completed_skip_count,
        ),
        (
            "constant_shr_case_break_count",
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count,
            executed,
        ),
        (
            "humidistat_case_selected_skip_count",
            predecessor.dehumidification_control_humidistat_case_selected_skip_count,
            state.dehumidification_control_humidistat_case_selected_skip_count,
        ),
        (
            "constant_supply_humidity_ratio_case_selected_skip_count",
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
        ("direct_constant_shr_case_break_count", 0, executed),
        (
            "direct_humidistat_case_selected_skip_count",
            0,
            state.dehumidification_control_humidistat_case_selected_skip_count,
        ),
        (
            "direct_constant_supply_humidity_ratio_case_selected_skip_count",
            0,
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn validate_route_partition(
    state: &PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState,
) -> Result<(), Error> {
    let partition = checked_sum(&[
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.dehumidification_control_none_case_completed_skip_count,
        state.dehumidification_control_constant_sensible_heat_ratio_case_break_count,
        state.dehumidification_control_humidistat_case_selected_skip_count,
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
    ])?;
    ensure_count(partition, state.transition_count, "transition_partition")
}

fn validate_source_counters(
    state: &PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState,
) -> Result<(), Error> {
    let executed = state.dehumidification_control_constant_sensible_heat_ratio_case_break_count;
    let source_sites = executed
        .checked_mul(PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER.len())
        .ok_or_else(|| violation("source_site_execution_count_overflow", usize::MAX, executed))?;
    ensure_count(
        state.source_site_execution_count,
        source_sites,
        "source_site_execution_count",
    )
}

pub(super) fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
) -> PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot {
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_CASE_BREAK_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered:
            predecessor.predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered:
            predecessor.predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped:
            predecessor.positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type:
            predecessor.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed:
            predecessor
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        dehumidification_control_none_case_completed_skip:
            predecessor.dehumidification_control_none_case_completed_skip,
        dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: false,
        dehumidification_control_humidistat_case_selected_skip:
            predecessor.dehumidification_control_humidistat_case_selected_skip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
    }
}

pub(super) fn snapshots_match_exact(
    left: &PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    right: &PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
) -> bool {
    left == right
}

fn checked_sum(values: &[usize]) -> Result<usize, Error> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| violation("transition_partition_overflow", usize::MAX, *value))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingConstantShrCaseBreakLifecycleInvariant {
        field,
        expected,
        actual,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::{IdealLoadsAirSystemId, ZoneId};

    #[derive(Clone, Copy)]
    enum Route {
        U,
        N,
        P,
        C0,
        Q,
        H,
        Csh,
    }

    #[test]
    fn inherited_direct_routes_validate_and_non_direct_routes_reject() {
        for route in [Route::U, Route::N, Route::P, Route::C0] {
            let (state, predecessor) = states(route);
            assert!(validate_counts(&state, &predecessor, 1).is_ok());
        }
        for route in [Route::Q, Route::H, Route::Csh] {
            let (state, predecessor) = states(route);
            assert!(validate_counts(&state, &predecessor, 1).is_err());
        }
    }

    #[test]
    fn partition_overflow_and_source_counter_corruption_fail_closed() {
        let mut state =
            PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState::new(IdealLoadsAirSystemId(0));
        state.unit_off_skip_count = usize::MAX;
        state.non_cooling_skip_count = 1;
        assert!(validate_route_partition(&state).is_err());

        let (mut active, _) = states(Route::Q);
        active.source_site_execution_count = 0;
        assert!(validate_source_counters(&active).is_err());
    }

    #[test]
    fn snapshot_identity_route_and_break_corruption_fail_exact_match() {
        let predecessor = direct_predecessor();
        let expected = expected_snapshot(predecessor);
        assert!(snapshots_match_exact(&expected, &expected));

        for corruption in ["source_order", "system", "call", "zone", "route", "break"] {
            let mut forged = expected;
            match corruption {
                "source_order" => forged.source_order = &["forged-cp357-source-order"],
                "system" => forged.system = IdealLoadsAirSystemId(1),
                "call" => forged.parent_call_ordinal = 2,
                "zone" => forged.controlled_zone = ZoneId(1),
                "route" => forged.dehumidification_control_none_case_completed_skip = false,
                "break" => {
                    forged
                        .dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break =
                        true;
                }
                _ => unreachable!(),
            }
            assert!(!snapshots_match_exact(&forged, &expected), "{corruption}");
        }
    }

    fn direct_predecessor()
    -> PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot {
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            source_order:
                crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_CONSTANT_SHR_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            predecessor_no_outdoor_air_fallback_entered: true,
            predecessor_positive_supply_mass_flow_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            predecessor_dehumidification_control_type:
                Some(DehumidificationControlType::None),
            predecessor_dehumidification_control_none_case_completed_skip: true,
            predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_executed:
                false,
            predecessor_dehumidification_control_humidistat_case_selected_skip: false,
            predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
                false,
            dehumidification_control_none_case_completed_skip: true,
            dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed:
                false,
            dehumidification_control_humidistat_case_selected_skip: false,
            dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
            supply_humidity_ratio_for_mixed_air_limit_minimum_read: false,
            supply_humidity_ratio_before_mixed_air_limit: None,
            mixed_air_humidity_ratio_for_minimum_read: false,
            mixed_air_humidity_ratio: None,
            source_shaped_two_argument_minimum_evaluated: false,
            minimum_supply_humidity_ratio: None,
            supply_humidity_ratio_assignment_performed: false,
            assigned_supply_humidity_ratio: None,
            resulting_supply_humidity_ratio: None,
        }
    }

    fn states(
        route: Route,
    ) -> (
        PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState,
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    ) {
        let system = IdealLoadsAirSystemId(0);
        let mut state = PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState::new(system);
        let mut predecessor =
            PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState::new(
                system,
            );
        state.transition_count = 1;
        predecessor.transition_count = 1;
        match route {
            Route::U => {
                state.unit_off_skip_count = 1;
                predecessor.unit_off_skip_count = 1;
            }
            Route::N => {
                state.non_cooling_skip_count = 1;
                predecessor.non_cooling_skip_count = 1;
            }
            Route::P => {
                state.positive_guard_false_fallthrough_skip_count = 1;
                predecessor.positive_guard_false_fallthrough_skip_count = 1;
            }
            Route::C0 => {
                state.dehumidification_control_none_case_completed_skip_count = 1;
                predecessor.dehumidification_control_none_case_completed_skip_count = 1;
            }
            Route::Q => {
                state.dehumidification_control_constant_sensible_heat_ratio_case_break_count = 1;
                state.source_site_execution_count = 1;
                predecessor
                    .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count =
                    1;
            }
            Route::H => {
                state.dehumidification_control_humidistat_case_selected_skip_count = 1;
                predecessor.dehumidification_control_humidistat_case_selected_skip_count = 1;
            }
            Route::Csh => {
                state
                    .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count =
                    1;
                predecessor
                    .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count =
                    1;
            }
        }
        (state, predecessor)
    }
}
