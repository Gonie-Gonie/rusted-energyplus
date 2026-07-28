//! CP329/CP343-to-CP344 retained-lineage validation.

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};

pub(super) fn mixed_air_limit_links_to_predecessor(
    limit:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let executed =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    let inherited = limit.system == predecessor.system
        && limit.parent_call_ordinal == predecessor.parent_call_ordinal
        && limit.controlled_zone == predecessor.controlled_zone
        && limit.unit_body_entered == predecessor.unit_body_entered
        && limit.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && limit.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && limit.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && limit.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && limit.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && limit.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && limit.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && limit.predecessor_capacity_limit_cp_air_assignment_executed
            == predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        && limit.predecessor_capacity_limit_sensible_output_assignment_executed
            == predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        && limit.predecessor_capacity_limit_sensible_output_guard_evaluated
            == predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated
        && limit.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && limit.predecessor_capacity_limit_sensible_output_adjustment_body_entered
            == predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && limit
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && limit.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && limit
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
            == executed
        && limit.unit_off_skipped == predecessor.unit_off_skipped
        && limit.non_cooling_skipped == predecessor.non_cooling_skipped
        && limit.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && limit.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && limit.capacity_limit_sensible_output_guard_false_fallthrough == guard_false
        && limit
            .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            == executed;
    if !inherited {
        return false;
    }

    let active_prefix = guard_false || executed;
    if !active_prefix {
        return limit.preexisting_supply_temperature_c.is_none()
            && limit.resulting_supply_temperature_c.is_none();
    }
    let (Some(owner_temperature), Some(preexisting), Some(resulting)) = (
        predecessor.resulting_supply_temperature_c,
        limit.preexisting_supply_temperature_c,
        limit.resulting_supply_temperature_c,
    ) else {
        return false;
    };
    if owner_temperature.to_bits() != preexisting.to_bits() {
        return false;
    }
    if guard_false {
        preexisting.to_bits() == resulting.to_bits()
    } else {
        option_bits_match(
            limit.supply_temperature_before_mixed_air_limit_c,
            Some(owner_temperature),
        ) && option_bits_match(limit.assigned_supply_temperature_c, Some(resulting))
    }
}

pub(super) fn retained_source_owner_lineage_is_exact(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    mixed_air: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    mixed_air_witness: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> bool {
    let executed =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    if !executed {
        return mixed_air.is_none() && mixed_air_witness.is_none();
    }
    let (Some(owner_temperature), Some(mixed_air), Some(mixed_air_witness)) = (
        predecessor.resulting_supply_temperature_c,
        mixed_air,
        mixed_air_witness,
    ) else {
        return false;
    };
    let Some(mixed_air_temperature_c) = mixed_air.mixed_air_temperature_c else {
        return false;
    };

    // The current left operand is solely CP343's resulting SupplyTemp.
    // CP329 solely owns the current right-side PurchAir.MixedAirTemp.
    let _ = owner_temperature;
    mixed_air.system == predecessor.system
        && mixed_air.parent_call_ordinal == predecessor.parent_call_ordinal
        && mixed_air.controlled_zone == predecessor.controlled_zone
        && mixed_air.cooling_call_executed
        && mixed_air.no_outdoor_air_fallback_entered
        && mixed_air.mixed_air_temperature_assigned
        && mixed_air_temperature_c.is_finite()
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed_air)
        && cooling_mixed_air_call_snapshots_match_bit_exact(mixed_air, mixed_air_witness)
}

pub(super) fn retained_input_from_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    mixed_air: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput,
> {
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let executed =
        predecessor.capacity_limit_sensible_output_supply_temperature_assignment_executed;
    if !guard_false && !executed {
        return None;
    }
    let preexisting_supply_temperature_c = predecessor.resulting_supply_temperature_c?;
    let active_operands = if executed {
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitActiveOperands {
                mixed_air_temperature_c: mixed_air?.mixed_air_temperature_c?,
            },
        )
    } else {
        None
    };
    Some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRetainedInput {
            preexisting_supply_temperature_c,
            active_operands,
        },
    )
}

pub(super) fn supply_temperature_assignment_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
        option_bits_match(left.supply_humidity_ratio, right.supply_humidity_ratio),
        option_bits_match(
            left.psychrometric_supply_temperature_result_c,
            right.psychrometric_supply_temperature_result_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
        option_bits_match(
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_temperature_c = None;
        snapshot.supply_enthalpy_j_per_kg = None;
        snapshot.supply_humidity_ratio = None;
        snapshot.psychrometric_supply_temperature_result_c = None;
        snapshot.assigned_supply_temperature_c = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
