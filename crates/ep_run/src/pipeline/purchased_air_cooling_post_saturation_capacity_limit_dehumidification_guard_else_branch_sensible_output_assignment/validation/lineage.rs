//! Bounded CP419-prefix and CP329/CP330 operand-owner validation for CP420.

use ep_runtime::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as SupplyFlowOwner,
};

use crate::pipeline::{
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment::serialization::snapshot::snapshot_json as predecessor_json,
    purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment::serialization::snapshot::snapshot_json,
};

const CP419_TERMINAL_KEYS: [&str; 6] = [
    "resulting_supply_humidity_ratio",
    "resulting_supply_humidity_ratio_ieee_bits",
    "resulting_supply_enthalpy_j_per_kg",
    "resulting_supply_enthalpy_j_per_kg_ieee_bits",
    "resulting_supply_temperature_c",
    "resulting_supply_temperature_c_ieee_bits",
];

pub(super) fn lineage_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    mixed_air: Option<MixedAirOwner>,
    supply_flow: Option<SupplyFlowOwner>,
) -> bool {
    cp419_prefix_is_exact(snapshot, predecessor)
        && predecessor_tail_is_exact(snapshot, predecessor)
        && local_assignment_is_exact(snapshot, predecessor, mixed_air, supply_flow)
        && carriers_are_preserved(snapshot, predecessor)
}

fn cp419_prefix_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let snapshot_json = snapshot_json(snapshot);
    let predecessor_json = predecessor_json(predecessor);
    let (Some(snapshot), Some(predecessor)) =
        (snapshot_json.as_object(), predecessor_json.as_object())
    else {
        return false;
    };
    predecessor.iter().all(|(key, expected)| {
        matches!(
            key.as_str(),
            "source" | "first_excluded_source" | "source_order"
        ) || CP419_TERMINAL_KEYS.contains(&key.as_str())
            || snapshot.get(key) == Some(expected)
    })
}

fn predecessor_tail_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    option_bits_equal(
        snapshot.predecessor_cp419_resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.predecessor_cp419_resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.predecessor_cp419_resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    )
}

fn local_assignment_is_exact(
    snapshot: Snapshot,
    predecessor: Predecessor,
    mixed_air: Option<MixedAirOwner>,
    supply_flow: Option<SupplyFlowOwner>,
) -> bool {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed;
    let active_flags = [
        snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed,
        snapshot.cp330_retained_supply_mass_flow_rate_owned_read,
        snapshot.cp329_supply_mass_flow_rate_bit_corroborated,
        snapshot.supply_mass_flow_rate_read,
        snapshot.cp419_retained_cp_air_owned_read,
        snapshot.cp_air_read,
        snapshot.supply_mass_flow_rate_times_cp_air_calculated,
        snapshot.cp329_retained_mixed_air_temperature_for_sensible_output_owned_read,
        snapshot.mixed_air_temperature_read,
        snapshot.cp419_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_read,
        snapshot.mixed_air_minus_supply_temperature_calculated,
        snapshot.cooling_sensible_output_calculated,
        snapshot.cooling_sensible_output_assigned,
    ];
    if active_flags.into_iter().any(|flag| flag != active)
        || snapshot.cp419_retained_supply_humidity_ratio_state_owned
            != predecessor.resulting_supply_humidity_ratio.is_some()
        || snapshot.cp419_retained_supply_enthalpy_state_owned
            != predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        || snapshot.cp419_retained_supply_temperature_state_owned
            != predecessor.resulting_supply_temperature_c.is_some()
    {
        return false;
    }
    if !active {
        return [
            snapshot.supply_mass_flow_rate_kg_per_s,
            snapshot.cp419_cp_air_for_sensible_output_j_per_kg_k,
            snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
            snapshot.mixed_air_temperature_for_sensible_output_c,
            snapshot.supply_temperature_for_sensible_output_c,
            snapshot.mixed_air_minus_supply_temperature_k,
            snapshot.calculated_cooling_sensible_output_w,
            snapshot.cooling_sensible_output_w,
        ]
        .into_iter()
        .all(|value| value.is_none())
            && mixed_air.is_none()
            && supply_flow.is_none();
    }

    let (Some(mixed_air), Some(supply_flow)) = (mixed_air, supply_flow) else {
        return false;
    };
    let (
        Some(flow),
        Some(corroborating_flow),
        Some(mixed_temperature),
        Some(cp_air),
        Some(supply_temperature),
    ) = (
        supply_flow.supply_mass_flow_rate_kg_per_s,
        mixed_air.supply_mass_flow_rate_kg_per_s,
        mixed_air.mixed_air_temperature_c,
        predecessor.cp_air_j_per_kg_k,
        predecessor.resulting_supply_temperature_c,
    )
    else {
        return false;
    };
    let first_product = flow * cp_air;
    let difference = mixed_temperature - supply_temperature;
    let result = first_product * difference;
    flow.to_bits() == corroborating_flow.to_bits()
        && owners_match_identity(snapshot, mixed_air, supply_flow)
        && option_has_bits(snapshot.supply_mass_flow_rate_kg_per_s, flow)
        && option_has_bits(snapshot.cp419_cp_air_for_sensible_output_j_per_kg_k, cp_air)
        && option_has_bits(
            snapshot.supply_mass_flow_rate_times_cp_air_w_per_k,
            first_product,
        )
        && option_has_bits(
            snapshot.mixed_air_temperature_for_sensible_output_c,
            mixed_temperature,
        )
        && option_has_bits(
            snapshot.supply_temperature_for_sensible_output_c,
            supply_temperature,
        )
        && option_has_bits(snapshot.mixed_air_minus_supply_temperature_k, difference)
        && option_has_bits(snapshot.calculated_cooling_sensible_output_w, result)
        && option_has_bits(snapshot.cooling_sensible_output_w, result)
}

fn owners_match_identity(
    snapshot: Snapshot,
    mixed_air: MixedAirOwner,
    supply_flow: SupplyFlowOwner,
) -> bool {
    [mixed_air.system, supply_flow.system]
        .into_iter()
        .all(|system| system == snapshot.system)
        && mixed_air.parent_call_ordinal == snapshot.parent_call_ordinal
        && supply_flow.parent_call_ordinal == snapshot.parent_call_ordinal
        && mixed_air.controlled_zone == snapshot.controlled_zone
        && supply_flow.controlled_zone == snapshot.controlled_zone
        && mixed_air.cooling_call_executed
        && mixed_air.no_outdoor_air_fallback_entered
        && supply_flow.positive_supply_mass_flow_body_entered
}

fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    option_bits_equal(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    )
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
