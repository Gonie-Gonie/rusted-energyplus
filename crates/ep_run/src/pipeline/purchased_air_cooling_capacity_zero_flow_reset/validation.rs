//! Fail-closed validation helpers for CP321 direct-release evidence.

use ep_model::IdealLoadsLimit;
use ep_runtime::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
};

mod snapshot;

pub(super) use snapshot::{same_option, snapshot_shape};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
) -> Result<(), String> {
    let first_selector_partition = checked_add(
        state.cooling_limit_capacity_count,
        state.second_cooling_limit_read_count,
        "first selector partition",
    )?;
    let second_selector_partition = checked_add(
        state.cooling_limit_flow_rate_and_capacity_count,
        state.cooling_limit_rejected_count,
        "second selector partition",
    )?;
    let selected_limit_count = checked_add(
        state.cooling_limit_capacity_count,
        state.cooling_limit_flow_rate_and_capacity_count,
        "selected limit partition",
    )?;
    let capacity_result_partition = checked_add(
        state.maximum_total_cooling_capacity_zero_count,
        state.maximum_total_cooling_capacity_nonzero_count,
        "capacity comparison partition",
    )?;
    for (field, expected, actual) in [
        (
            "first_cooling_limit_read_count",
            state.cooling_body_entry_count,
            state.first_cooling_limit_read_count,
        ),
        (
            "first_selector_partition",
            state.first_cooling_limit_read_count,
            first_selector_partition,
        ),
        (
            "second_selector_partition",
            state.second_cooling_limit_read_count,
            second_selector_partition,
        ),
        (
            "maximum_total_cooling_capacity_read_count",
            selected_limit_count,
            state.maximum_total_cooling_capacity_read_count,
        ),
        (
            "maximum_total_cooling_capacity_comparison_count",
            state.maximum_total_cooling_capacity_read_count,
            state.maximum_total_cooling_capacity_comparison_count,
        ),
        (
            "capacity_comparison_partition",
            state.maximum_total_cooling_capacity_comparison_count,
            capacity_result_partition,
        ),
        (
            "zero_cooling_capacity_body_entry_count",
            state.maximum_total_cooling_capacity_zero_count,
            state.zero_cooling_capacity_body_entry_count,
        ),
        (
            "supply_mass_flow_rate_for_cool_zero_assignment_count",
            state.zero_cooling_capacity_body_entry_count,
            state.supply_mass_flow_rate_for_cool_zero_assignment_count,
        ),
        (
            "supply_mass_flow_rate_for_dehumidification_zero_assignment_count",
            state.zero_cooling_capacity_body_entry_count,
            state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count,
        ),
        (
            "supply_mass_flow_rate_for_humidification_zero_assignment_count",
            state.zero_cooling_capacity_body_entry_count,
            state.supply_mass_flow_rate_for_humidification_zero_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling-capacity-zero reset invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_fixed_selector_route(
    state: &PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    latest: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> Result<(), String> {
    let active_calls = state.cooling_body_entry_count;
    if active_calls == 0 {
        return Ok(());
    }

    let cumulative_route = [
        (SelectorRoute::Capacity, state.cooling_limit_capacity_count),
        (
            SelectorRoute::FlowRateAndCapacity,
            state.cooling_limit_flow_rate_and_capacity_count,
        ),
        (SelectorRoute::Rejected, state.cooling_limit_rejected_count),
    ]
    .into_iter()
    .find_map(|(route, count)| (count == active_calls).then_some(route))
    .ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset active calls used mixed selector routes"
            .to_string()
    })?;

    if latest.cooling_body_entered {
        let latest_route = match latest.first_cooling_limit {
            Some(IdealLoadsLimit::LimitCapacity) => SelectorRoute::Capacity,
            Some(IdealLoadsLimit::LimitFlowRateAndCapacity) => SelectorRoute::FlowRateAndCapacity,
            Some(IdealLoadsLimit::NoLimit | IdealLoadsLimit::LimitFlowRate) => {
                SelectorRoute::Rejected
            }
            None => {
                return Err(
                    "direct-zone IdealLoads cooling-capacity-zero reset latest cooling selector is absent"
                        .to_string(),
                );
            }
        };
        if latest_route != cumulative_route {
            return Err(
                "direct-zone IdealLoads cooling-capacity-zero reset latest selector does not match its cumulative route"
                    .to_string(),
            );
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SelectorRoute {
    Capacity,
    FlowRateAndCapacity,
    Rejected,
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling-capacity-zero reset {label} overflowed")
    })
}
