use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_post_saturation_capacity_limit_guard;
    let supply_owner = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let supply_corroborator = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let mixed_air_owner = output.calculation_cooling_mixed_air_call;
    let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard;

    metadata_matches(
        snapshot,
        predecessor,
        supply_owner,
        supply_corroborator,
        mixed_air_owner,
        call_ordinal,
        binding,
    ) && cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(snapshot)
        && expected_snapshot(predecessor, supply_owner, supply_corroborator, mixed_air_owner)
            .is_some_and(|expected| snapshots_match_exact_bits(snapshot, expected))
}

#[allow(clippy::too_many_arguments)]
fn metadata_matches(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    supply_owner: SupplyOwnerSnapshot,
    supply_corroborator: SupplyCorroboratorSnapshot,
    mixed_air_owner: MixedAirSnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && [
            predecessor.system,
            supply_owner.system,
            supply_corroborator.system,
            mixed_air_owner.system,
        ]
        .into_iter()
        .all(|system| system == snapshot.system)
        && [
            predecessor.parent_call_ordinal,
            supply_owner.parent_call_ordinal,
            supply_corroborator.parent_call_ordinal,
            mixed_air_owner.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == snapshot.parent_call_ordinal)
        && [
            predecessor.controlled_zone,
            supply_owner.controlled_zone,
            supply_corroborator.controlled_zone,
            mixed_air_owner.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == snapshot.controlled_zone)
}

pub(super) fn expected_snapshot(
    predecessor: PredecessorSnapshot,
    supply_owner: SupplyOwnerSnapshot,
    supply_corroborator: SupplyCorroboratorSnapshot,
    mixed_air_owner: MixedAirSnapshot,
) -> Option<Snapshot> {
    let active = predecessor.capacity_limit_body_entered;
    let (supply_humidity_ratio, mixed_air_humidity_ratio) = if active {
        if !supply_owner.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed
            || !supply_corroborator
                .purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read
            || !mixed_air_owner.mixed_air_humidity_ratio_assigned
        {
            return None;
        }
        let supply = supply_owner.resulting_supply_humidity_ratio?;
        if supply.to_bits() != supply_corroborator.supply_humidity_ratio?.to_bits() {
            return None;
        }
        (
            Some(supply),
            Some(mixed_air_owner.mixed_air_humidity_ratio?),
        )
    } else {
        (None, None)
    };
    let comparison = match (supply_humidity_ratio, mixed_air_humidity_ratio) {
        (Some(supply), Some(mixed)) => Some(supply < mixed),
        (None, None) => None,
        _ => return None,
    };

    Some(Snapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor.capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor.capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .active_guard_false_fallthrough,
        dehumidification_guard_evaluated: active,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: active,
        cp379_same_call_supply_humidity_ratio_bit_corroborated: active,
        purchased_air_supply_humidity_ratio_read: active,
        supply_humidity_ratio,
        cp329_mixed_air_humidity_ratio_owned_read: active,
        purchased_air_mixed_air_humidity_ratio_read: active,
        mixed_air_humidity_ratio,
        supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated: active,
        supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio: comparison,
        dehumidification_body_entered: comparison == Some(true),
        dehumidification_guard_false_fallthrough: comparison == Some(false),
    })
}

pub(super) fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    left.source == right.source
        && left.first_excluded_source == right.first_excluded_source
        && left.source_order == right.source_order
        && left.system == right.system
        && left.parent_call_ordinal == right.parent_call_ordinal
        && left.controlled_zone == right.controlled_zone
        && left.unit_off_skipped == right.unit_off_skipped
        && left.non_cooling_skipped == right.non_cooling_skipped
        && left.positive_guard_false_fallthrough_skipped
            == right.positive_guard_false_fallthrough_skipped
        && left.heating_availability_guard_false_fallthrough
            == right.heating_availability_guard_false_fallthrough
        && left.humidification_control_guard_false_fallthrough
            == right.humidification_control_guard_false_fallthrough
        && left.dehumidification_control_humidistat_maximum_assignment_executed
            == right.dehumidification_control_humidistat_maximum_assignment_executed
        && left.dehumidification_control_none_maximum_assignment_executed
            == right.dehumidification_control_none_maximum_assignment_executed
        && left.dehumidification_control_guard_false_fallthrough
            == right.dehumidification_control_guard_false_fallthrough
        && left.predecessor_capacity_limit_guard_evaluated
            == right.predecessor_capacity_limit_guard_evaluated
        && left.predecessor_capacity_limit_body_entered
            == right.predecessor_capacity_limit_body_entered
        && left.predecessor_active_capacity_limit_guard_false_fallthrough
            == right.predecessor_active_capacity_limit_guard_false_fallthrough
        && left.dehumidification_guard_evaluated == right.dehumidification_guard_evaluated
        && left.cp378_supply_humidity_ratio_saturation_limit_owned_read
            == right.cp378_supply_humidity_ratio_saturation_limit_owned_read
        && left.cp379_same_call_supply_humidity_ratio_bit_corroborated
            == right.cp379_same_call_supply_humidity_ratio_bit_corroborated
        && left.purchased_air_supply_humidity_ratio_read
            == right.purchased_air_supply_humidity_ratio_read
        && exact_optional_f64(left.supply_humidity_ratio, right.supply_humidity_ratio)
        && left.cp329_mixed_air_humidity_ratio_owned_read
            == right.cp329_mixed_air_humidity_ratio_owned_read
        && left.purchased_air_mixed_air_humidity_ratio_read
            == right.purchased_air_mixed_air_humidity_ratio_read
        && exact_optional_f64(
            left.mixed_air_humidity_ratio,
            right.mixed_air_humidity_ratio,
        )
        && left.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated
            == right.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated
        && left.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio
            == right.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio
        && left.dehumidification_body_entered == right.dehumidification_body_entered
        && left.dehumidification_guard_false_fallthrough
            == right.dehumidification_guard_false_fallthrough
}
