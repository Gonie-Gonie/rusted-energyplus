//! CP329/CP335/CP344-to-CP345 retained-lineage validation.

use super::super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let active = predecessor.capacity_limit_guard_false_fallthrough_skipped
        || predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed;
    assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && assignment.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && assignment.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_cp_air_assignment_executed
            == predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_assignment_executed
            == predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_guard_evaluated
            == predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated
        && assignment.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_sensible_output_adjustment_body_entered
            == predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && assignment
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
        && assignment
            .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_supply_temperature_assignment_executed
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && assignment.capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        && assignment.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            == predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        && assignment.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            == active
}

pub(super) fn owner_lineage_is_exact(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    owner: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    owner_witness: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> bool {
    if !predecessor_is_active(predecessor) {
        return owner.is_none() && owner_witness.is_none();
    }
    let (Some(owner), Some(owner_witness)) = (owner, owner_witness) else {
        return false;
    };
    owner.system == predecessor.system
        && owner.parent_call_ordinal == predecessor.parent_call_ordinal
        && owner.controlled_zone == predecessor.controlled_zone
        && owner.cooling_call_executed
        && owner.no_outdoor_air_fallback_entered
        && owner.mixed_air_humidity_ratio_assigned
        && owner.mixed_air_humidity_ratio.is_some()
        && cooling_mixed_air_call_snapshot_is_exact_direct_release(owner)
        && cooling_mixed_air_call_snapshots_match_bit_exact(owner, owner_witness)
}

pub(super) fn corroboration_lineage_is_exact(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    owner: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    corroboration: Option<
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    >,
    corroboration_witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    >,
) -> bool {
    if !predecessor_is_active(predecessor) {
        return corroboration.is_none() && corroboration_witness.is_none();
    }
    let (Some(owner), Some(corroboration), Some(corroboration_witness)) =
        (owner, corroboration, corroboration_witness)
    else {
        return false;
    };
    let Some(owner_value) = owner.mixed_air_humidity_ratio else {
        return false;
    };
    corroboration.system == predecessor.system
        && corroboration.parent_call_ordinal == predecessor.parent_call_ordinal
        && corroboration.controlled_zone == predecessor.controlled_zone
        && corroboration.supply_humidity_ratio_mixed_air_assignment_executed
        && corroboration.mixed_air_humidity_ratio_read
        && option_bits_match(
            corroboration.mixed_air_humidity_ratio,
            Some(owner_value),
        )
        && corroboration.supply_humidity_ratio_assignment_performed
        && option_bits_match(
            corroboration.assigned_supply_humidity_ratio,
            Some(owner_value),
        )
        && cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            corroboration,
        )
        && humidity_assignment_snapshots_match_bit_exact(
            corroboration,
            corroboration_witness,
        )
}

pub(super) fn active_input_from_owner(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    owner: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput,
>{
    if !predecessor_is_active(predecessor) {
        return None;
    }
    Some(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentActiveInput {
            mixed_air_humidity_ratio: owner?.mixed_air_humidity_ratio?,
        },
    )
}

pub(super) fn predecessor_is_active(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    predecessor.capacity_limit_guard_false_fallthrough_skipped
        || predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
}

pub(super) fn predecessor_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
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
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left == right
}

pub(super) fn humidity_assignment_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.assigned_supply_humidity_ratio,
        right.assigned_supply_humidity_ratio,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
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
