//! Fail-closed validation helpers for CP341 evidence.

use ep_runtime::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
) -> Result<(), String> {
    let assignments = state.capacity_limit_sensible_output_maximum_capacity_assignment_count;
    let expected_sites = assignments.checked_mul(2).ok_or_else(|| {
        "direct-zone IdealLoads sensible-output maximum-capacity assignment source-site count overflowed"
            .to_string()
    })?;
    for (field, expected, actual) in [
        (
            "source_site_execution_count",
            expected_sites,
            state.source_site_execution_count,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            assignments,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "cooling_sensible_output_assignment_write_count",
            assignments,
            state.cooling_sensible_output_assignment_write_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads sensible-output maximum-capacity assignment invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn snapshot_shape(
    snapshot:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    predecessor: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
) -> bool {
    let evaluated = predecessor.capacity_limit_sensible_output_guard_evaluated;
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assignment = predecessor.capacity_limit_sensible_output_adjustment_body_entered;
    if snapshot.unit_body_entered != predecessor.unit_body_entered
        || snapshot.predecessor_cooling_body_entered != predecessor.predecessor_cooling_body_entered
        || snapshot.predecessor_no_outdoor_air_fallback_entered
            != predecessor.predecessor_no_outdoor_air_fallback_entered
        || snapshot.predecessor_positive_supply_mass_flow_body_entered
            != predecessor.predecessor_positive_supply_mass_flow_body_entered
        || snapshot.predecessor_active_guard_false_fallthrough
            != predecessor.predecessor_active_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_guard_evaluated
            != predecessor.predecessor_capacity_limit_guard_evaluated
        || snapshot.predecessor_capacity_limit_body_entered
            != predecessor.predecessor_capacity_limit_body_entered
        || snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            != predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        || snapshot.predecessor_capacity_limit_cp_air_assignment_executed
            != predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        || snapshot.predecessor_capacity_limit_sensible_output_assignment_executed
            != predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        || snapshot.predecessor_capacity_limit_sensible_output_guard_evaluated != evaluated
        || snapshot.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            != guard_false
        || snapshot.predecessor_capacity_limit_sensible_output_adjustment_body_entered != assignment
        || snapshot.unit_off_skipped != predecessor.unit_off_skipped
        || snapshot.non_cooling_skipped != predecessor.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped
            != predecessor.positive_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_guard_false_fallthrough_skipped
            != predecessor.capacity_limit_guard_false_fallthrough_skipped
        || snapshot.capacity_limit_sensible_output_guard_false_fallthrough != guard_false
        || snapshot.capacity_limit_sensible_output_maximum_capacity_assignment_executed
            != assignment
        || evaluated != (guard_false || assignment)
        || (evaluated && guard_false == assignment)
    {
        return false;
    }

    if !evaluated {
        return !predecessor.cooling_sensible_output_read
            && predecessor.cooling_sensible_output_w.is_none()
            && !predecessor.maximum_total_cooling_capacity_read
            && predecessor.maximum_total_cooling_capacity_w.is_none()
            && !predecessor.cooling_sensible_output_maximum_capacity_comparison_evaluated
            && predecessor
                .cooling_sensible_output_at_or_above_maximum_capacity
                .is_none()
            && snapshot.preexisting_cooling_sensible_output_w.is_none()
            && !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot.cooling_sensible_output_assigned
            && snapshot.assigned_cooling_sensible_output_w.is_none()
            && snapshot.resulting_cooling_sensible_output_w.is_none();
    }

    let (Some(preexisting), Some(retained_capacity)) = (
        predecessor.cooling_sensible_output_w,
        predecessor.maximum_total_cooling_capacity_w,
    ) else {
        return false;
    };
    let comparison = preexisting >= retained_capacity;
    if !predecessor.cooling_sensible_output_read
        || !predecessor.maximum_total_cooling_capacity_read
        || !predecessor.cooling_sensible_output_maximum_capacity_comparison_evaluated
        || predecessor.cooling_sensible_output_at_or_above_maximum_capacity != Some(comparison)
        || guard_false == comparison
        || assignment != comparison
        || !active_capacity_is_reachable(retained_capacity)
        || !option_has_bits(snapshot.preexisting_cooling_sensible_output_w, preexisting)
    {
        return false;
    }

    if guard_false {
        !snapshot.maximum_total_cooling_capacity_read
            && snapshot.maximum_total_cooling_capacity_w.is_none()
            && !snapshot.cooling_sensible_output_assigned
            && snapshot.assigned_cooling_sensible_output_w.is_none()
            && option_has_bits(snapshot.resulting_cooling_sensible_output_w, preexisting)
    } else {
        snapshot.maximum_total_cooling_capacity_read
            && option_has_bits(snapshot.maximum_total_cooling_capacity_w, retained_capacity)
            && snapshot.cooling_sensible_output_assigned
            && option_has_bits(
                snapshot.assigned_cooling_sensible_output_w,
                retained_capacity,
            )
            && option_has_bits(
                snapshot.resulting_cooling_sensible_output_w,
                retained_capacity,
            )
    }
}

fn active_capacity_is_reachable(capacity_w: f64) -> bool {
    capacity_w.is_finite() && capacity_w > 0.0
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
    };

    use super::*;

    #[test]
    fn source_counter_overflow_fails_closed() {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        state.capacity_limit_sensible_output_maximum_capacity_assignment_count = usize::MAX;
        let error = validate_source_counters(&state).expect_err("overflow must be rejected");
        assert!(error.contains("overflowed"));
    }

    #[test]
    fn exact_bits_preserve_nan_and_distinguish_signed_zero() {
        let nan = f64::from_bits(0x7ff8_0000_0000_0042);
        assert!(option_has_bits(Some(nan), nan));
        assert!(option_has_bits(Some(-0.0), -0.0));
        assert!(!option_has_bits(Some(-0.0), 0.0));
    }

    #[test]
    fn forged_active_capacity_domain_is_rejected() {
        for capacity in [0.0, -0.0, -1.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            assert!(!active_capacity_is_reachable(capacity));
        }
        assert!(active_capacity_is_reachable(f64::MIN_POSITIVE));
    }

    #[test]
    fn snapshot_validator_accepts_true_false_and_inherited_skip_routes() {
        for route in [
            Route::InheritedSkip,
            Route::GuardFalse {
                preexisting: f64::from_bits(0x7ff8_0000_0000_0042),
                maximum: 10.0,
            },
            Route::Assignment {
                preexisting: f64::INFINITY,
                maximum: 10.0,
            },
        ] {
            let predecessor = predecessor(route);
            let snapshot = assignment(predecessor);
            assert!(snapshot_shape(&snapshot, &predecessor));
        }
    }

    #[test]
    fn snapshot_validator_rejects_forged_active_capacity_and_assigned_bits() {
        for maximum in [0.0, -0.0, -1.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let predecessor = predecessor(Route::Assignment {
                preexisting: f64::INFINITY,
                maximum,
            });
            let snapshot = assignment(predecessor);
            assert!(!snapshot_shape(&snapshot, &predecessor));
        }

        let predecessor = predecessor(Route::Assignment {
            preexisting: f64::INFINITY,
            maximum: 10.0,
        });
        let mut snapshot = assignment(predecessor);
        snapshot.assigned_cooling_sensible_output_w = Some(f64::from_bits(10.0_f64.to_bits() ^ 1));
        assert!(!snapshot_shape(&snapshot, &predecessor));
    }

    #[derive(Clone, Copy)]
    enum Route {
        InheritedSkip,
        GuardFalse { preexisting: f64, maximum: f64 },
        Assignment { preexisting: f64, maximum: f64 },
    }

    fn predecessor(
        route: Route,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
        let evaluated = !matches!(route, Route::InheritedSkip);
        let guard_false = matches!(route, Route::GuardFalse { .. });
        let body_entered = matches!(route, Route::Assignment { .. });
        let values = match route {
            Route::InheritedSkip => None,
            Route::GuardFalse {
                preexisting,
                maximum,
            }
            | Route::Assignment {
                preexisting,
                maximum,
            } => Some((preexisting, maximum)),
        };
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: evaluated,
            predecessor_cooling_body_entered: evaluated,
            predecessor_no_outdoor_air_fallback_entered: evaluated,
            predecessor_positive_supply_mass_flow_body_entered: evaluated,
            predecessor_active_guard_false_fallthrough: false,
            predecessor_capacity_limit_guard_evaluated: evaluated,
            predecessor_capacity_limit_body_entered: evaluated,
            predecessor_active_capacity_limit_guard_false_fallthrough: false,
            predecessor_capacity_limit_cp_air_assignment_executed: evaluated,
            predecessor_capacity_limit_sensible_output_assignment_executed: evaluated,
            unit_off_skipped: !evaluated,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            capacity_limit_guard_false_fallthrough_skipped: false,
            capacity_limit_sensible_output_guard_evaluated: evaluated,
            cooling_sensible_output_read: evaluated,
            cooling_sensible_output_w: values.map(|(preexisting, _)| preexisting),
            maximum_total_cooling_capacity_read: evaluated,
            maximum_total_cooling_capacity_w: values.map(|(_, maximum)| maximum),
            cooling_sensible_output_maximum_capacity_comparison_evaluated: evaluated,
            cooling_sensible_output_at_or_above_maximum_capacity:
                evaluated.then_some(body_entered),
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_adjustment_body_entered: body_entered,
        }
    }

    fn assignment(
        predecessor: PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    ) -> PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot
    {
        let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
        let assignment = predecessor.capacity_limit_sensible_output_adjustment_body_entered;
        let preexisting = (guard_false || assignment)
            .then_some(predecessor.cooling_sensible_output_w)
            .flatten();
        let maximum = assignment
            .then_some(predecessor.maximum_total_cooling_capacity_w)
            .flatten();
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_SENSIBLE_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
            system: predecessor.system,
            parent_call_ordinal: predecessor.parent_call_ordinal,
            controlled_zone: predecessor.controlled_zone,
            unit_body_entered: predecessor.unit_body_entered,
            predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
            predecessor_no_outdoor_air_fallback_entered: predecessor
                .predecessor_no_outdoor_air_fallback_entered,
            predecessor_positive_supply_mass_flow_body_entered: predecessor
                .predecessor_positive_supply_mass_flow_body_entered,
            predecessor_active_guard_false_fallthrough: predecessor
                .predecessor_active_guard_false_fallthrough,
            predecessor_capacity_limit_guard_evaluated: predecessor
                .predecessor_capacity_limit_guard_evaluated,
            predecessor_capacity_limit_body_entered: predecessor
                .predecessor_capacity_limit_body_entered,
            predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
                .predecessor_active_capacity_limit_guard_false_fallthrough,
            predecessor_capacity_limit_cp_air_assignment_executed: predecessor
                .predecessor_capacity_limit_cp_air_assignment_executed,
            predecessor_capacity_limit_sensible_output_assignment_executed: predecessor
                .predecessor_capacity_limit_sensible_output_assignment_executed,
            predecessor_capacity_limit_sensible_output_guard_evaluated: predecessor
                .capacity_limit_sensible_output_guard_evaluated,
            predecessor_capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            predecessor_capacity_limit_sensible_output_adjustment_body_entered: assignment,
            unit_off_skipped: predecessor.unit_off_skipped,
            non_cooling_skipped: predecessor.non_cooling_skipped,
            positive_guard_false_fallthrough_skipped: predecessor
                .positive_guard_false_fallthrough_skipped,
            capacity_limit_guard_false_fallthrough_skipped: predecessor
                .capacity_limit_guard_false_fallthrough_skipped,
            capacity_limit_sensible_output_guard_false_fallthrough: guard_false,
            capacity_limit_sensible_output_maximum_capacity_assignment_executed: assignment,
            preexisting_cooling_sensible_output_w: preexisting,
            maximum_total_cooling_capacity_read: assignment,
            maximum_total_cooling_capacity_w: maximum,
            cooling_sensible_output_assigned: assignment,
            assigned_cooling_sensible_output_w: maximum,
            resulting_cooling_sensible_output_w: if assignment {
                maximum
            } else {
                preexisting
            },
        }
    }
}
