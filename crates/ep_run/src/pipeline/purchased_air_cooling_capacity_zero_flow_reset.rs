//! Run-summary evidence for the bounded PurchasedAir cooling-capacity-zero reset.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{
    same_option, snapshot_shape, validate_fixed_selector_route, validate_source_counters,
};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary>,
    predecessor_cp320: Option<&PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary>,
    predecessor_cp319: Option<&PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary>,
    predecessor_cp318: Option<&PurchasedAirCalcCoolingSensibleFlowLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling-capacity-zero reset evidence"
            .to_string()
    })?;
    let predecessor_cp320 = predecessor_cp320.ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no humidification-flow evidence"
            .to_string()
    })?;
    let predecessor_cp319 = predecessor_cp319.ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no dehumidification-flow evidence"
            .to_string()
    })?;
    let predecessor_cp318 = predecessor_cp318.ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no sensible-flow evidence"
            .to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no coupling call count".to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || predecessor_cp320.source != PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE
        || predecessor_cp320.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        || predecessor_cp319.source != PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE
        || predecessor_cp319.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        || predecessor_cp318.source != PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        || predecessor_cp318.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling-capacity-zero reset provenance is invalid".to_string(),
        );
    }

    let state = &lifecycle.state;
    let cp320_state = &predecessor_cp320.state;
    let skipped_count = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let transition_partition = checked_add(
        skipped_count,
        state.cooling_body_entry_count,
        "transition partition",
    )?;
    let predecessor_skipped_count = checked_add(
        cp320_state.unit_off_skip_count,
        cp320_state.non_cooling_skip_count,
        "predecessor skip partition",
    )?;
    let predecessor_cooling_count = cp320_state
        .transition_count
        .checked_sub(predecessor_skipped_count)
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling-capacity-zero predecessor partition is invalid"
                .to_string()
        })?;
    for (field, expected, actual) in [
        (
            "transition_count",
            coupling_call_count,
            state.transition_count,
        ),
        (
            "predecessor_transition_count",
            cp320_state.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "unit_off_skip_count",
            cp320_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            cp320_state.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "cooling_body_entry_count",
            predecessor_cooling_count,
            state.cooling_body_entry_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling-capacity-zero reset invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no latest snapshot".to_string()
    })?;
    validate_fixed_selector_route(state, latest)?;
    let latest_cp320 = predecessor_cp320.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no latest CP320 snapshot"
            .to_string()
    })?;
    let latest_cp319 = predecessor_cp319.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no latest CP319 snapshot"
            .to_string()
    })?;
    let latest_cp318 = predecessor_cp318.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no latest CP318 snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling-capacity-zero reset has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling-capacity-zero reset has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor_cp320.state.system != expected_system
        || predecessor_cp319.state.system != expected_system
        || predecessor_cp318.state.system != expected_system
        || !latest_matches_release(
            latest,
            latest_cp320,
            latest_cp319,
            latest_cp318,
            expected_system,
            expected_zone,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling-capacity-zero reset latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn latest_matches_release(
    reset: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    cp320: &PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    cp319: &PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    cp318: &PurchasedAirCalcCoolingSensibleFlowSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    reset.source == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        && reset.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        && reset.source_order == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER
        && cp320.source == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE
        && cp320.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        && cp320.source_order == PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER
        && cp319.source == PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE
        && cp319.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE
        && cp319.source_order == PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER
        && cp318.source == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE
        && cp318.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE
        && cp318.source_order == PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER
        && [reset.system, cp320.system, cp319.system, cp318.system]
            .into_iter()
            .all(|system| system == expected_system)
        && [
            reset.parent_call_ordinal,
            cp320.parent_call_ordinal,
            cp319.parent_call_ordinal,
            cp318.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == call_count)
        && [
            reset.controlled_zone,
            cp320.controlled_zone,
            cp319.controlled_zone,
            cp318.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        && reset.unit_body_entered == cp320.unit_body_entered
        && reset.predecessor_cooling_body_entered == cp320.cooling_body_entered
        && reset.unit_off_skipped == cp320.unit_off_skipped
        && reset.non_cooling_skipped == cp320.non_cooling_skipped
        && reset.cooling_body_entered == cp320.cooling_body_entered
        && same_option(
            reset.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
            cp318.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        )
        && same_option(
            reset.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            cp319.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        )
        && same_option(
            reset.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
            cp320.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        )
        && snapshot_shape(reset)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling-capacity-zero reset {label} overflowed")
    })
}
