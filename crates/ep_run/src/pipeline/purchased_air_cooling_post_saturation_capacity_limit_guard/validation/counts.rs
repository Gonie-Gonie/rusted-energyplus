use super::*;

pub(super) fn validate(
    state: &State,
    predecessor: &PredecessorState,
    cooling_limit: IdealLoadsLimit,
    calls: usize,
) -> Result<(), String> {
    let routes = route_counts(state);
    let predecessor_routes = predecessor_route_counts(predecessor);
    if routes != predecessor_routes {
        return Err("direct-zone IdealLoads CP380 carried route counters are invalid".into());
    }
    for (field, actual) in [
        (
            "private_humidistat_route_count",
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "private_none_maximum_route_count",
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "private_dehumidification_guard_fallthrough_route_count",
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
    ] {
        ensure_count(actual, 0, field)?;
    }
    let transition_partition = checked_sum(&routes, "transition partition")?;
    let active = predecessor.local_supply_enthalpy_after_saturation_limit_assignment_count;
    let active_routes = checked_sum(&routes[3..], "active route partition")?;
    let capacity_matches = usize::from(cooling_limit == IdealLoadsLimit::LimitCapacity) * active;
    let second = checked_sub(active, capacity_matches, "second-comparison partition")?;
    let combined_matches =
        usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity) * active;
    let body = checked_add(capacity_matches, combined_matches, "body-entry partition")?;
    let rejected = checked_sub(active, body, "active-false partition")?;
    let sites = checked_add(
        checked_add(
            checked_mul(active, 2, "first selector sites")?,
            checked_mul(second, 2, "second selector sites")?,
            "selector sites",
        )?,
        body,
        "source sites",
    )?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        ("active_route_partition", active, active_routes),
        (
            "capacity_limit_guard_evaluation_count",
            active,
            state.capacity_limit_guard_evaluation_count,
        ),
        (
            "configured_cooling_limit_owned_read_count",
            active,
            state.configured_cooling_limit_owned_read_count,
        ),
        (
            "cp337_same_call_selector_lineage_corroboration_count",
            active,
            state.cp337_same_call_selector_lineage_corroboration_count,
        ),
        (
            "first_cooling_limit_read_count",
            active,
            state.first_cooling_limit_read_count,
        ),
        (
            "cooling_limit_capacity_comparison_count",
            active,
            state.cooling_limit_capacity_comparison_count,
        ),
        (
            "cooling_limit_capacity_match_count",
            capacity_matches,
            state.cooling_limit_capacity_match_count,
        ),
        (
            "second_cooling_limit_read_count",
            second,
            state.second_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            second,
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_match_count",
            combined_matches,
            state.cooling_limit_flow_rate_and_capacity_match_count,
        ),
        (
            "capacity_limit_body_entry_count",
            body,
            state.capacity_limit_body_entry_count,
        ),
        (
            "cooling_limit_rejected_count",
            rejected,
            state.cooling_limit_rejected_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            rejected,
            state.active_guard_false_fallthrough_count,
        ),
        (
            "source_site_execution_count",
            sites,
            state.source_site_execution_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    validate_route_partitions(state, &routes[3..], body > 0)
}

pub(super) fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: Snapshot,
) -> bool {
    let Some(index) = route_flags(latest).into_iter().position(|route| route) else {
        return false;
    };
    route_counts(state)[index] > 0 && predecessor_route_counts(predecessor)[index] > 0
}

fn validate_route_partitions(
    state: &State,
    active_routes: &[usize],
    selected: bool,
) -> Result<(), String> {
    let partitions = [
        (
            state.heating_availability_guard_false_fallthrough_body_entry_count,
            state.heating_availability_guard_false_fallthrough_capacity_guard_false_count,
        ),
        (
            state.humidification_control_guard_false_fallthrough_body_entry_count,
            state.humidification_control_guard_false_fallthrough_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_humidistat_maximum_assignment_body_entry_count,
            state.dehumidification_control_humidistat_maximum_assignment_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_none_maximum_assignment_body_entry_count,
            state.dehumidification_control_none_maximum_assignment_capacity_guard_false_count,
        ),
        (
            state.dehumidification_control_guard_false_fallthrough_body_entry_count,
            state.dehumidification_control_guard_false_fallthrough_capacity_guard_false_count,
        ),
    ];
    for (&route, (body, rejected)) in active_routes.iter().zip(partitions) {
        ensure_count(
            body,
            if selected { route } else { 0 },
            "route_body_partition",
        )?;
        ensure_count(
            rejected,
            if selected { 0 } else { route },
            "route_false_partition",
        )?;
    }
    Ok(())
}

fn route_counts(state: &State) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn predecessor_route_counts(state: &PredecessorState) -> [usize; 8] {
    [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ]
}

fn route_flags(snapshot: Snapshot) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}

fn checked_sum(values: &[usize], label: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value).ok_or_else(|| overflow(label))
    })
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| overflow(label))
}

fn checked_sub(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_sub(right).ok_or_else(|| overflow(label))
}

fn checked_mul(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_mul(right).ok_or_else(|| overflow(label))
}

fn overflow(label: &str) -> String {
    format!("direct-zone IdealLoads CP380 {label} overflowed")
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState;

    use super::*;

    #[test]
    fn all_four_fixed_selectors_validate_exact_lazy_counter_shapes() {
        for limit in [
            IdealLoadsLimit::LimitCapacity,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            IdealLoadsLimit::NoLimit,
            IdealLoadsLimit::LimitFlowRate,
        ] {
            let (state, predecessor) = states(limit);
            validate(&state, &predecessor, limit, 1).expect("valid CP380 selector counters");
        }
    }

    #[test]
    fn selector_counter_overflow_and_mixed_history_fail_closed() {
        let (mut state, predecessor) = states(IdealLoadsLimit::LimitCapacity);
        state.capacity_limit_guard_evaluation_count = usize::MAX;
        assert!(
            validate(&state, &predecessor, IdealLoadsLimit::LimitCapacity, 1)
                .expect_err("overflow must fail closed")
                .contains("expected")
        );
        let (mut state, predecessor) = states(IdealLoadsLimit::LimitCapacity);
        state.cooling_limit_capacity_match_count = 0;
        assert!(
            validate(&state, &predecessor, IdealLoadsLimit::LimitCapacity, 1)
                .expect_err("mixed selector history must fail closed")
                .contains("cooling_limit_capacity_match_count")
        );
    }

    fn states(
        limit: IdealLoadsLimit,
    ) -> (
        State,
        PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState,
    ) {
        let capacity = limit == IdealLoadsLimit::LimitCapacity;
        let second = !capacity;
        let combined = second && limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
        let selected = capacity || combined;
        let mut predecessor =
            PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            );
        predecessor.transition_count = 1;
        predecessor.heating_availability_guard_false_fallthrough_count = 1;
        predecessor.local_supply_enthalpy_after_saturation_limit_assignment_count = 1;
        let mut state = State::new(IdealLoadsAirSystemId(0));
        state.transition_count = 1;
        state.heating_availability_guard_false_fallthrough_count = 1;
        state.capacity_limit_guard_evaluation_count = 1;
        state.source_site_execution_count = 2 + 2 * usize::from(second) + usize::from(selected);
        state.configured_cooling_limit_owned_read_count = 1;
        state.cp337_same_call_selector_lineage_corroboration_count = 1;
        state.first_cooling_limit_read_count = 1;
        state.cooling_limit_capacity_comparison_count = 1;
        state.cooling_limit_capacity_match_count = usize::from(capacity);
        state.second_cooling_limit_read_count = usize::from(second);
        state.cooling_limit_flow_rate_and_capacity_comparison_count = usize::from(second);
        state.cooling_limit_flow_rate_and_capacity_match_count = usize::from(combined);
        state.cooling_limit_rejected_count = usize::from(!selected);
        state.capacity_limit_body_entry_count = usize::from(selected);
        state.active_guard_false_fallthrough_count = usize::from(!selected);
        state.heating_availability_guard_false_fallthrough_body_entry_count = usize::from(selected);
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count =
            usize::from(!selected);
        (state, predecessor)
    }
}
