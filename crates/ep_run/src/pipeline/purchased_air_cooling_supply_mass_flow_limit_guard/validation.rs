//! Fail-closed validation helpers for CP325 evidence.

use ep_model::IdealLoadsLimit;
use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_fixed_selector_route(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    cooling_limit: IdealLoadsLimit,
) -> Result<(), String> {
    let cooling = state.cooling_body_entry_count;
    let expected_first = usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRate) * cooling;
    let expected_combined =
        usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity) * cooling;
    let expected_rejected = cooling - expected_first - expected_combined;
    let valid = state.cooling_limit_flow_rate_match_count == expected_first
        && state.cooling_limit_flow_rate_and_capacity_match_count == expected_combined
        && state.cooling_limit_rejected_count == expected_rejected;
    if valid {
        Ok(())
    } else {
        Err(
            "direct-zone IdealLoads cooling flow-limit guard selector history does not match the model cooling limit"
                .to_string(),
        )
    }
}

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    maximum: f64,
) -> Result<(), String> {
    let cooling = state.cooling_body_entry_count;
    let first_matches = state.cooling_limit_flow_rate_match_count;
    let selected = first_matches
        .checked_add(state.cooling_limit_flow_rate_and_capacity_match_count)
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling flow-limit guard selector count overflowed".to_string()
        })?;
    let expected_second = cooling.checked_sub(first_matches).ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard second selector underflowed".to_string()
    })?;
    let positive = if maximum > 0.0 { selected } else { 0 };
    let not_positive = selected.checked_sub(positive).ok_or_else(|| {
        "direct-zone IdealLoads cooling flow-limit guard positivity partition underflowed"
            .to_string()
    })?;
    for (field, expected, actual) in [
        (
            "first_cooling_limit_read_count",
            cooling,
            state.first_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_comparison_count",
            cooling,
            state.cooling_limit_flow_rate_comparison_count,
        ),
        (
            "second_cooling_limit_read_count",
            expected_second,
            state.second_cooling_limit_read_count,
        ),
        (
            "cooling_limit_flow_rate_and_capacity_comparison_count",
            expected_second,
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_read_count",
            selected,
            state.maximum_cooling_air_mass_flow_rate_read_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_positive_comparison_count",
            selected,
            state.maximum_cooling_air_mass_flow_rate_positive_comparison_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_strictly_positive_count",
            positive,
            state.maximum_cooling_air_mass_flow_rate_strictly_positive_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_not_positive_count",
            not_positive,
            state.maximum_cooling_air_mass_flow_rate_not_positive_count,
        ),
        (
            "supply_mass_flow_limit_body_entry_count",
            positive,
            state.supply_mass_flow_limit_body_entry_count,
        ),
        (
            "active_guard_false_fallthrough_count",
            cooling - positive,
            state.active_guard_false_fallthrough_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling flow-limit guard invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}
