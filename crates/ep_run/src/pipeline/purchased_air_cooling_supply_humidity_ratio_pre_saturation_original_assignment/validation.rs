//! Fail-closed validation for CP376 direct-release evidence.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    PurchasedAirInitLifecycleSummary,
};

type Lifecycle =
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentLifecycleSummary;
type State = PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState;
type Snapshot = PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot;
type PredecessorLifecycle = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentLifecycleSummary;
type PredecessorState = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState;
type PredecessorSnapshot = PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot;
type OwnerLifecycle = PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary;
type OwnerState = PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState;
type OwnerSnapshot =
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot;

pub(in crate::pipeline) struct DirectLifecyclePredecessors<'a> {
    pub(in crate::pipeline) maximum_assignment_cp375: Option<&'a PredecessorLifecycle>,
    pub(in crate::pipeline) none_case_cp347: Option<&'a OwnerLifecycle>,
}

pub(in crate::pipeline) fn validate_direct_lifecycle(
    lifecycle: Option<&Lifecycle>,
    predecessors: DirectLifecyclePredecessors<'_>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose CP376 pre-saturation original-assignment evidence"
            .to_string()
    })?;
    let predecessor = predecessors.maximum_assignment_cp375.ok_or_else(|| {
        "direct-zone IdealLoads CP376 pre-saturation original assignment has no CP375 evidence"
            .to_string()
    })?;
    let owner = predecessors.none_case_cp347.ok_or_else(|| {
        "direct-zone IdealLoads CP376 pre-saturation original assignment has no CP347 owner evidence"
            .to_string()
    })?;
    let init = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads CP376 pre-saturation original assignment has no initialization evidence"
            .to_string()
    })?;
    let calls = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads CP376 pre-saturation original assignment has no coupling call count"
            .to_string()
    })?;
    let expected_system = init.declared_system_order.first().copied().ok_or_else(|| {
        "direct-zone IdealLoads CP376 pre-saturation original assignment has no declared system"
            .to_string()
    })?;
    let expected_zone = init.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads CP376 pre-saturation original assignment has no controlled Zone"
            .to_string()
    })?;

    validate_release_state(
        lifecycle,
        predecessor,
        owner,
        expected_system,
        expected_zone,
        calls,
    )
}

fn validate_release_state(
    lifecycle: &Lifecycle,
    predecessor: &PredecessorLifecycle,
    owner: &OwnerLifecycle,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> Result<(), String> {
    if calls == 0
        || lifecycle.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || owner.source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        || owner.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        || PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER.len()
            != 2
        || lifecycle.state.system != expected_system
        || predecessor.state.system != expected_system
        || owner.state.system != expected_system
    {
        return Err("direct-zone IdealLoads CP376 provenance is invalid".into());
    }

    validate_counts(&lifecycle.state, &predecessor.state, &owner.state, calls)?;

    let latest = lifecycle
        .state
        .latest
        .ok_or_else(|| "direct-zone IdealLoads CP376 latest evidence is missing".to_string())?;
    let predecessor_latest = predecessor.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP376 CP375 latest evidence is missing".to_string()
    })?;
    let owner_latest = owner.state.latest.ok_or_else(|| {
        "direct-zone IdealLoads CP376 CP347 latest evidence is missing".to_string()
    })?;
    if !latest_metadata_is_exact(
        latest,
        predecessor_latest,
        owner_latest,
        expected_system,
        expected_zone,
        calls,
    ) || !snapshot_links_exactly(latest, predecessor_latest, owner_latest)
        || !latest_route_has_cumulative_evidence(
            &lifecycle.state,
            &predecessor.state,
            predecessor_latest,
        )
    {
        return Err("direct-zone IdealLoads CP376 latest lineage is invalid".into());
    }
    Ok(())
}

fn validate_counts(
    state: &State,
    predecessor: &PredecessorState,
    owner: &OwnerState,
    calls: usize,
) -> Result<(), String> {
    let carried = [
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        state.positive_guard_false_fallthrough_skip_count,
        state.heating_availability_guard_false_fallthrough_count,
        state.humidification_control_guard_false_fallthrough_count,
        state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        state.dehumidification_control_guard_false_fallthrough_count,
    ];
    let expected = [
        predecessor.unit_off_skip_count,
        predecessor.non_cooling_skip_count,
        predecessor.positive_guard_false_fallthrough_skip_count,
        predecessor.heating_availability_guard_false_fallthrough_count,
        predecessor.humidification_control_guard_false_fallthrough_count,
        predecessor
            .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        predecessor.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        predecessor.dehumidification_control_guard_false_fallthrough_count,
    ];
    if carried != expected {
        return Err("direct-zone IdealLoads CP376 carried CP375 counters are invalid".into());
    }

    let partition = checked_sum(&carried, "transition partition")?;
    let assignments = checked_sum(&carried[3..], "active assignment partition")?;
    let owner_reads = checked_sum(
        &[
            state.cp375_maximum_assignment_owner_count,
            state.cp347_none_case_owner_count,
            state.cp356_constant_shr_owner_count,
            state.cp362_humidistat_owner_count,
            state.cp365_constant_supply_humidity_ratio_owner_count,
        ],
        "owner partition",
    )?;
    let source_sites = assignments
        .checked_mul(
            PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER
                .len(),
        )
        .ok_or_else(|| "CP376 source-site count overflowed".to_string())?;
    for (field, expected, actual) in [
        ("transition_count", calls, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        ("transition_partition", state.transition_count, partition),
        ("owner_partition", assignments, owner_reads),
        (
            "cp347_owner_completion_count",
            owner.dehumidification_control_none_case_completion_count,
            state.cp347_none_case_owner_count,
        ),
        (
            "direct_cp347_owner_count",
            assignments,
            state.cp347_none_case_owner_count,
        ),
        (
            "direct_cp375_owner_count",
            0,
            state.cp375_maximum_assignment_owner_count,
        ),
        (
            "direct_cp356_owner_count",
            0,
            state.cp356_constant_shr_owner_count,
        ),
        (
            "direct_cp362_owner_count",
            0,
            state.cp362_humidistat_owner_count,
        ),
        (
            "direct_cp365_owner_count",
            0,
            state.cp365_constant_supply_humidity_ratio_owner_count,
        ),
        (
            "direct_humidistat_maximum_assignment_count",
            0,
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "direct_none_maximum_assignment_count",
            0,
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        ),
        (
            "direct_dehumidification_guard_false_fallthrough_count",
            0,
            state.dehumidification_control_guard_false_fallthrough_count,
        ),
        (
            "source_site_execution_count",
            source_sites,
            state.source_site_execution_count,
        ),
        (
            "purchased_air_supply_humidity_ratio_before_saturation_limit_read_count",
            assignments,
            state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
        ),
        (
            "local_original_supply_humidity_ratio_before_saturation_limit_assignment_count",
            assignments,
            state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }
    Ok(())
}

fn latest_metadata_is_exact(
    latest: Snapshot,
    predecessor: PredecessorSnapshot,
    owner: OwnerSnapshot,
    expected_system: IdealLoadsAirSystemId,
    expected_zone: ZoneId,
    calls: usize,
) -> bool {
    latest.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE
        && latest.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && latest.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE_ORDER
        && predecessor.source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_MAXIMUM_ASSIGNMENT_SOURCE_ORDER
        && owner.source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE
        && owner.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE
        && owner.source_order
            == PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE_ORDER
        && [latest.system, predecessor.system, owner.system]
            .into_iter()
            .all(|system| system == expected_system)
        && [latest.controlled_zone, predecessor.controlled_zone, owner.controlled_zone]
            .into_iter()
            .all(|zone| zone == expected_zone)
        && [
            latest.parent_call_ordinal,
            predecessor.parent_call_ordinal,
            owner.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == calls)
}

fn snapshot_links_exactly(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    owner: OwnerSnapshot,
) -> bool {
    let routes_match = snapshot.unit_off_skipped == predecessor.unit_off_skipped
        && snapshot.non_cooling_skipped == predecessor.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && snapshot.heating_availability_guard_false_fallthrough
            == predecessor.predecessor_heating_on_guard_false_fallthrough
        && snapshot.humidification_control_guard_false_fallthrough
            == predecessor.predecessor_humidification_control_guard_false_fallthrough
        && snapshot.dehumidification_control_humidistat_maximum_assignment_executed
            == predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed
        && snapshot.dehumidification_control_none_maximum_assignment_executed
            == predecessor
                .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed
        && snapshot.dehumidification_control_guard_false_fallthrough
            == predecessor.predecessor_dehumidification_control_guard_false_fallthrough;
    let routes = [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    let route_count = routes.into_iter().filter(|active| *active).count();
    let predecessor_matches = snapshot.predecessor_dehumidification_control_type
        == predecessor.predecessor_dehumidification_control_type
        && snapshot.predecessor_purchased_air_supply_humidity_ratio_assignment_performed
            == predecessor.purchased_air_supply_humidity_ratio_assignment_performed
        && option_bits_equal(
            snapshot.predecessor_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
    let active = !(snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || snapshot.positive_guard_false_fallthrough_skipped);
    let values_match = if active {
        owner
            .resulting_supply_humidity_ratio
            .is_some_and(|owner_value| {
                !snapshot.cp375_maximum_assignment_owned_read
                    && snapshot.cp347_none_case_owned_read
                    && !snapshot.cp356_constant_shr_owned_read
                    && !snapshot.cp362_humidistat_owned_read
                    && !snapshot.cp365_constant_supply_humidity_ratio_owned_read
                    && snapshot.purchased_air_supply_humidity_ratio_read
                    && snapshot.local_supply_humidity_ratio_original_assignment_performed
                    && [
                        snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
                        snapshot.assigned_supply_humidity_ratio_original,
                        snapshot.resulting_supply_humidity_ratio_original,
                    ]
                    .into_iter()
                    .all(|value| option_bits_equal(value, Some(owner_value)))
            })
    } else {
        !snapshot.cp375_maximum_assignment_owned_read
            && !snapshot.cp347_none_case_owned_read
            && !snapshot.cp356_constant_shr_owned_read
            && !snapshot.cp362_humidistat_owned_read
            && !snapshot.cp365_constant_supply_humidity_ratio_owned_read
            && !snapshot.purchased_air_supply_humidity_ratio_read
            && !snapshot.local_supply_humidity_ratio_original_assignment_performed
            && snapshot
                .purchased_air_supply_humidity_ratio_before_saturation_check
                .is_none()
            && snapshot.assigned_supply_humidity_ratio_original.is_none()
            && snapshot.resulting_supply_humidity_ratio_original.is_none()
    };
    let public_direct_shape = public_direct_route_shape(
        routes.into_iter().position(|active| active),
        (
            snapshot.predecessor_dehumidification_control_type,
            predecessor.predecessor_dehumidification_control_type,
        ),
        [
            snapshot.cp375_maximum_assignment_owned_read,
            snapshot.cp347_none_case_owned_read,
            snapshot.cp356_constant_shr_owned_read,
            snapshot.cp362_humidistat_owned_read,
            snapshot.cp365_constant_supply_humidity_ratio_owned_read,
        ],
    );
    route_count == 1 && routes_match && predecessor_matches && public_direct_shape && values_match
}

#[rustfmt::skip]
fn public_direct_route_shape(route: Option<usize>, selectors: (Option<DehumidificationControlType>, Option<DehumidificationControlType>), owners: [bool; 5]) -> bool {
    match route { Some(0..=2) => selectors == (None, None) && owners == [false; 5],
        Some(3..=4) => selectors == (Some(DehumidificationControlType::None), Some(DehumidificationControlType::None)) && owners == [false, true, false, false, false],
        _ => false }
}

fn latest_route_has_cumulative_evidence(
    state: &State,
    predecessor: &PredecessorState,
    latest: PredecessorSnapshot,
) -> bool {
    let pair = if latest.unit_off_skipped {
        (state.unit_off_skip_count, predecessor.unit_off_skip_count)
    } else if latest.non_cooling_skipped {
        (
            state.non_cooling_skip_count,
            predecessor.non_cooling_skip_count,
        )
    } else if latest.positive_guard_false_fallthrough_skipped {
        (
            state.positive_guard_false_fallthrough_skip_count,
            predecessor.positive_guard_false_fallthrough_skip_count,
        )
    } else if latest.predecessor_heating_on_guard_false_fallthrough {
        (
            state.heating_availability_guard_false_fallthrough_count,
            predecessor.heating_availability_guard_false_fallthrough_count,
        )
    } else if latest.predecessor_humidification_control_guard_false_fallthrough {
        (
            state.humidification_control_guard_false_fallthrough_count,
            predecessor.humidification_control_guard_false_fallthrough_count,
        )
    } else if latest
        .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed
    {
        (
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            predecessor
                .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        )
    } else if latest.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed
    {
        (
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            predecessor
                .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        )
    } else if latest.predecessor_dehumidification_control_guard_false_fallthrough {
        (
            state.dehumidification_control_guard_false_fallthrough_count,
            predecessor.dehumidification_control_guard_false_fallthrough_count,
        )
    } else {
        return false;
    };
    pair.0 > 0 && pair.1 > 0
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn checked_sum(values: &[usize], partition: &str) -> Result<usize, String> {
    values.iter().try_fold(0usize, |sum, value| {
        sum.checked_add(*value)
            .ok_or_else(|| format!("CP376 {partition} overflowed"))
    })
}

fn ensure_count(actual: usize, expected: usize, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads CP376 pre-saturation original assignment {field} expected {expected}, got {actual}"
        ))
    }
}

#[cfg(test)]
mod tests;
