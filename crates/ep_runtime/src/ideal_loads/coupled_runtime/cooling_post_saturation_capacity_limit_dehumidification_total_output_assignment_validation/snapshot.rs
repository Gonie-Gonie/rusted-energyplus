use super::*;

pub(super) fn matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor =
        output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard;
    let supply_mass_flow_owner = output.calculation_cooling_supply_mass_flow_positive_guard;
    let mixed_air_owner = output.calculation_cooling_mixed_air_call;
    let early_total_corroborator =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    let supply_enthalpy_owner =
        output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;

    metadata_matches(
        snapshot,
        predecessor,
        supply_mass_flow_owner,
        mixed_air_owner,
        early_total_corroborator,
        supply_enthalpy_owner,
        call_ordinal,
        binding,
    ) && cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(snapshot)
        && expected_snapshot(
            predecessor,
            supply_mass_flow_owner,
            mixed_air_owner,
            early_total_corroborator,
            supply_enthalpy_owner,
        )
        .is_some_and(|expected| snapshots_match_exact_bits(snapshot, expected))
}

#[allow(clippy::too_many_arguments)]
fn metadata_matches(
    snapshot: Snapshot,
    predecessor: PredecessorSnapshot,
    supply_mass_flow_owner: SupplyMassFlowSnapshot,
    mixed_air_owner: MixedAirSnapshot,
    early_total_corroborator: EarlyTotalSnapshot,
    supply_enthalpy_owner: SupplyEnthalpySnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    snapshot.system == binding.ideal_loads_air_system
        && snapshot.parent_call_ordinal == call_ordinal
        && snapshot.controlled_zone == binding.zone
        && [
            predecessor.system,
            supply_mass_flow_owner.system,
            mixed_air_owner.system,
            early_total_corroborator.system,
            supply_enthalpy_owner.system,
        ]
        .into_iter()
        .all(|system| system == snapshot.system)
        && [
            predecessor.parent_call_ordinal,
            supply_mass_flow_owner.parent_call_ordinal,
            mixed_air_owner.parent_call_ordinal,
            early_total_corroborator.parent_call_ordinal,
            supply_enthalpy_owner.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == snapshot.parent_call_ordinal)
        && [
            predecessor.controlled_zone,
            supply_mass_flow_owner.controlled_zone,
            mixed_air_owner.controlled_zone,
            early_total_corroborator.controlled_zone,
            supply_enthalpy_owner.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == snapshot.controlled_zone)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn expected_snapshot(
    predecessor: PredecessorSnapshot,
    supply_mass_flow_owner: SupplyMassFlowSnapshot,
    mixed_air_owner: MixedAirSnapshot,
    early_total_corroborator: EarlyTotalSnapshot,
    supply_enthalpy_owner: SupplyEnthalpySnapshot,
) -> Option<Snapshot> {
    let active = predecessor.dehumidification_body_entered;
    let (supply_mass_flow, mixed_air_enthalpy, supply_enthalpy) = if active {
        if !supply_mass_flow_owner.supply_mass_flow_rate_read
            || !mixed_air_owner.supply_mass_flow_rate_read
            || !mixed_air_owner.child_supply_mass_flow_rate_read
            || !early_total_corroborator.supply_mass_flow_rate_read
            || !mixed_air_owner.recirculation_enthalpy_projection_read
            || !mixed_air_owner.mixed_air_enthalpy_projection_assigned
            || !early_total_corroborator.mixed_air_enthalpy_read
            || !supply_enthalpy_owner
                .local_supply_enthalpy_after_saturation_limit_assignment_performed
        {
            return None;
        }
        let supply_mass_flow = supply_mass_flow_owner.supply_mass_flow_rate_kg_per_s?;
        for corroborator in [
            mixed_air_owner.supply_mass_flow_rate_kg_per_s?,
            mixed_air_owner.child_supply_mass_flow_rate_kg_per_s?,
            mixed_air_owner.resulting_recirculation_mass_flow_rate_kg_per_s?,
            early_total_corroborator.supply_mass_flow_rate_kg_per_s?,
        ] {
            if corroborator.to_bits() != supply_mass_flow.to_bits() {
                return None;
            }
        }
        let mixed_air_enthalpy = mixed_air_owner.mixed_air_enthalpy_projection_j_per_kg?;
        for corroborator in [
            mixed_air_owner.recirculation_enthalpy_projection_j_per_kg?,
            early_total_corroborator.mixed_air_enthalpy_j_per_kg?,
        ] {
            if corroborator.to_bits() != mixed_air_enthalpy.to_bits() {
                return None;
            }
        }
        let supply_enthalpy = supply_enthalpy_owner.resulting_supply_enthalpy_j_per_kg?;
        for corroborator in [
            supply_enthalpy_owner.assigned_supply_enthalpy_j_per_kg?,
            supply_enthalpy_owner.psychrometric_supply_enthalpy_j_per_kg?,
        ] {
            if corroborator.to_bits() != supply_enthalpy.to_bits() {
                return None;
            }
        }
        (
            Some(supply_mass_flow),
            Some(mixed_air_enthalpy),
            Some(supply_enthalpy),
        )
    } else {
        (None, None, None)
    };
    let enthalpy_difference = match (mixed_air_enthalpy, supply_enthalpy) {
        (Some(mixed_air), Some(supply)) => Some(mixed_air - supply),
        (None, None) => None,
        _ => return None,
    };
    let cooling_total_output = match (supply_mass_flow, enthalpy_difference) {
        (Some(mass_flow), Some(difference)) => Some(mass_flow * difference),
        (None, None) => None,
        _ => return None,
    };

    Some(Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor.dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .dehumidification_guard_false_fallthrough,
        dehumidification_total_output_assignment_executed: active,
        cp330_supply_mass_flow_rate_owned_read: active,
        cp329_same_call_supply_mass_flow_rate_bit_corroborated: active,
        cp339_same_call_supply_mass_flow_rate_bit_corroborated: active,
        supply_mass_flow_rate_read: active,
        supply_mass_flow_rate_kg_per_s: supply_mass_flow,
        cp329_mixed_air_enthalpy_owned_read: active,
        cp329_same_call_recirculation_enthalpy_bit_corroborated: active,
        cp339_same_call_mixed_air_enthalpy_bit_corroborated: active,
        mixed_air_enthalpy_read: active,
        mixed_air_enthalpy_j_per_kg: mixed_air_enthalpy,
        cp379_post_saturation_supply_enthalpy_owned_read: active,
        cp379_same_call_supply_enthalpy_bits_corroborated: active,
        supply_enthalpy_read: active,
        supply_enthalpy_j_per_kg: supply_enthalpy,
        enthalpy_difference_calculated: active,
        mixed_air_minus_supply_enthalpy_j_per_kg: enthalpy_difference,
        cooling_total_output_calculated: active,
        calculated_cooling_total_output_w: cooling_total_output,
        cooling_total_output_assigned: active,
        cooling_total_output_w: cooling_total_output,
    })
}

pub(super) fn snapshots_match_exact_bits(left: Snapshot, right: Snapshot) -> bool {
    let values_match = [
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ),
        (
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
        (
            left.mixed_air_minus_supply_enthalpy_j_per_kg,
            right.mixed_air_minus_supply_enthalpy_j_per_kg,
        ),
        (
            left.calculated_cooling_total_output_w,
            right.calculated_cooling_total_output_w,
        ),
        (left.cooling_total_output_w, right.cooling_total_output_w),
    ]
    .into_iter()
    .all(|(left, right)| exact_optional_f64(left, right));
    let mut left_without_values = left;
    let mut right_without_values = right;
    for snapshot in [&mut left_without_values, &mut right_without_values] {
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_minus_supply_enthalpy_j_per_kg = None;
        snapshot.calculated_cooling_total_output_w = None;
        snapshot.cooling_total_output_w = None;
    }
    values_match && left_without_values == right_without_values
}
