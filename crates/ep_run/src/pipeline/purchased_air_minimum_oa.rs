//! Run-summary evidence for the bounded PurchasedAir minimum-OA prefix.

use ep_runtime::{
    PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
    PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER, PurchasedAirCalcEntryLifecycleSummary,
    PurchasedAirCalcEntrySnapshot, PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
    PurchasedAirCalcMinimumOaPrefixSnapshot, PurchasedAirInitLifecycleSummary,
};
use serde_json::{Value, json};

pub(super) fn lifecycle_json(lifecycle: &PurchasedAirCalcMinimumOaPrefixLifecycleSummary) -> Value {
    let latest = lifecycle.state.latest.map(|snapshot| {
        json!({
            "source": snapshot.source,
            "minimum_oa_child_source": snapshot.minimum_oa_child_source,
            "system": snapshot.system.0,
            "parent_call_ordinal": snapshot.parent_call_ordinal,
            "source_order": snapshot.source_order,
            "controlled_zone": snapshot.controlled_zone.0,
            "unit_body_entered": snapshot.unit_body_entered,
            "zone_heat_balance_reference_bound": snapshot.zone_heat_balance_reference_bound,
            "minimum_oa_child_called": snapshot.minimum_oa_child_called,
            "minimum_oa_child_no_outdoor_air_route": snapshot.minimum_oa_child_no_outdoor_air_route,
            "retained_minimum_outdoor_air_mass_flow_rate_kg_per_s":
                snapshot.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
            "retained_minimum_outdoor_air_write_performed":
                snapshot.retained_minimum_outdoor_air_write_performed,
            "ems_override_flag_read": snapshot.ems_override_flag_read,
            "ems_override_enabled": snapshot.ems_override_enabled,
            "ems_override_applied": snapshot.ems_override_applied,
            "working_outdoor_air_mass_flow_rate_kg_per_s":
                snapshot.working_outdoor_air_mass_flow_rate_kg_per_s,
            "outdoor_air_flag_read": snapshot.outdoor_air_flag_read,
            "outdoor_air_enabled": snapshot.outdoor_air_enabled,
            "no_outdoor_air_zero_branch_entered": snapshot.no_outdoor_air_zero_branch_entered,
            "psychrometric_call_count": snapshot.psychrometric_call_count,
            "minimum_outdoor_air_sensible_output_w":
                snapshot.minimum_outdoor_air_sensible_output_w,
            "minimum_outdoor_air_moisture_output_kg_per_s":
                snapshot.minimum_outdoor_air_moisture_output_kg_per_s,
        })
    });
    json!({
        "source": lifecycle.source,
        "minimum_oa_child_source": lifecycle.minimum_oa_child_source,
        "system": lifecycle.state.system.0,
        "transition_count": lifecycle.state.transition_count,
        "source_execution_count": lifecycle.state.source_execution_count,
        "unit_off_skip_count": lifecycle.state.unit_off_skip_count,
        "zone_heat_balance_reference_count":
            lifecycle.state.zone_heat_balance_reference_count,
        "minimum_oa_child_call_count": lifecycle.state.minimum_oa_child_call_count,
        "minimum_oa_child_no_outdoor_air_count":
            lifecycle.state.minimum_oa_child_no_outdoor_air_count,
        "retained_minimum_outdoor_air_write_count":
            lifecycle.state.retained_minimum_outdoor_air_write_count,
        "ems_override_flag_read_count": lifecycle.state.ems_override_flag_read_count,
        "ems_override_apply_count": lifecycle.state.ems_override_apply_count,
        "outdoor_air_flag_read_count": lifecycle.state.outdoor_air_flag_read_count,
        "outdoor_air_effect_count": lifecycle.state.outdoor_air_effect_count,
        "no_outdoor_air_zero_branch_count":
            lifecycle.state.no_outdoor_air_zero_branch_count,
        "psychrometric_call_count": lifecycle.state.psychrometric_call_count,
        "latest": latest,
    })
}

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcMinimumOaPrefixLifecycleSummary>,
    calculation_entry_lifecycle: Option<&PurchasedAirCalcEntryLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose minimum-OA prefix evidence".to_string()
    })?;
    let calculation_entry_lifecycle = calculation_entry_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads minimum-OA prefix has no Calc-entry evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads minimum-OA prefix has no initialization evidence".to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads minimum-OA prefix has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    let entry_state = &calculation_entry_lifecycle.state;
    let source_skip_partition = state
        .source_execution_count
        .checked_add(state.unit_off_skip_count)
        .ok_or_else(|| {
            "direct-zone IdealLoads minimum-OA prefix source/skip partition overflowed".to_string()
        })?;
    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        || lifecycle.minimum_oa_child_source != PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        || state.transition_count != coupling_call_count
    {
        return Err(
            "direct-zone IdealLoads minimum-OA prefix provenance or call count is invalid"
                .to_string(),
        );
    }
    for (field, expected, actual) in [
        (
            "source_execution_count",
            entry_state.unit_body_entry_count,
            state.source_execution_count,
        ),
        (
            "unit_off_skip_count",
            entry_state.unit_off_count,
            state.unit_off_skip_count,
        ),
        (
            "zone_heat_balance_reference_count",
            state.source_execution_count,
            state.zone_heat_balance_reference_count,
        ),
        (
            "minimum_oa_child_call_count",
            state.source_execution_count,
            state.minimum_oa_child_call_count,
        ),
        (
            "minimum_oa_child_no_outdoor_air_count",
            state.source_execution_count,
            state.minimum_oa_child_no_outdoor_air_count,
        ),
        (
            "retained_minimum_outdoor_air_write_count",
            state.source_execution_count,
            state.retained_minimum_outdoor_air_write_count,
        ),
        (
            "ems_override_flag_read_count",
            state.source_execution_count,
            state.ems_override_flag_read_count,
        ),
        (
            "ems_override_apply_count",
            0,
            state.ems_override_apply_count,
        ),
        (
            "outdoor_air_flag_read_count",
            state.source_execution_count,
            state.outdoor_air_flag_read_count,
        ),
        (
            "outdoor_air_effect_count",
            0,
            state.outdoor_air_effect_count,
        ),
        (
            "no_outdoor_air_zero_branch_count",
            state.source_execution_count,
            state.no_outdoor_air_zero_branch_count,
        ),
        (
            "psychrometric_call_count",
            0,
            state.psychrometric_call_count,
        ),
        (
            "source_skip_partition",
            coupling_call_count,
            source_skip_partition,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads minimum-OA prefix invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads minimum-OA prefix has no latest snapshot".to_string()
    })?;
    let latest_entry = entry_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads minimum-OA prefix has no latest Calc-entry snapshot".to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads minimum-OA prefix has no declared system".to_string()
        })?;
    if state.system != expected_system
        || entry_state.system != expected_system
        || !latest_matches_release(latest, latest_entry, coupling_call_count)
    {
        return Err(
            "direct-zone IdealLoads minimum-OA prefix latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    prefix: &PurchasedAirCalcMinimumOaPrefixSnapshot,
    entry: &PurchasedAirCalcEntrySnapshot,
    call_count: usize,
) -> bool {
    let common = prefix.source == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        && prefix.minimum_oa_child_source == PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        && prefix.source_order == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER
        && prefix.system == entry.system
        && prefix.parent_call_ordinal == call_count
        && prefix.parent_call_ordinal == entry.call_ordinal
        && prefix.controlled_zone == entry.controlled_zone
        && prefix.unit_body_entered == entry.unit_body_entered
        && !prefix.ems_override_applied
        && prefix.psychrometric_call_count == 0;
    if !common {
        return false;
    }
    if prefix.unit_body_entered {
        prefix.zone_heat_balance_reference_bound
            && prefix.minimum_oa_child_called
            && prefix.minimum_oa_child_no_outdoor_air_route
            && prefix.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s == Some(0.0)
            && prefix.retained_minimum_outdoor_air_write_performed
            && prefix.ems_override_flag_read
            && prefix.ems_override_enabled == Some(false)
            && prefix.working_outdoor_air_mass_flow_rate_kg_per_s == Some(0.0)
            && prefix.outdoor_air_flag_read
            && prefix.outdoor_air_enabled == Some(false)
            && prefix.no_outdoor_air_zero_branch_entered
            && prefix.minimum_outdoor_air_sensible_output_w == Some(0.0)
            && prefix.minimum_outdoor_air_moisture_output_kg_per_s == Some(0.0)
    } else {
        !prefix.zone_heat_balance_reference_bound
            && !prefix.minimum_oa_child_called
            && !prefix.minimum_oa_child_no_outdoor_air_route
            && prefix
                .retained_minimum_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !prefix.retained_minimum_outdoor_air_write_performed
            && !prefix.ems_override_flag_read
            && prefix.ems_override_enabled.is_none()
            && prefix.working_outdoor_air_mass_flow_rate_kg_per_s.is_none()
            && !prefix.outdoor_air_flag_read
            && prefix.outdoor_air_enabled.is_none()
            && !prefix.no_outdoor_air_zero_branch_entered
            && prefix.minimum_outdoor_air_sensible_output_w.is_none()
            && prefix
                .minimum_outdoor_air_moisture_output_kg_per_s
                .is_none()
    }
}
