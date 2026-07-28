//! CP329/CP345/CP346-to-CP347 retained-lineage validation.

use ep_model::DehumidificationControlType;

use super::super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn predecessor_selects_none_case(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) -> bool {
    predecessor.predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        && predecessor.dehumidification_control_type_read
        && predecessor.dehumidification_control_type == Some(DehumidificationControlType::None)
        && predecessor.dehumidification_control_switch_dispatched
}

pub(super) fn active_input_from_owner(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    owner: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
>{
    if !predecessor_selects_none_case(predecessor) {
        return None;
    }
    Some(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput {
            mixed_air_humidity_ratio: owner?.mixed_air_humidity_ratio?,
        },
    )
}

pub(super) fn owner_lineage_is_exact(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    owner: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    owner_witness: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
) -> bool {
    if !predecessor_selects_none_case(predecessor) {
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

pub(super) fn humidity_ratio_lineage_is_exact(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    owner: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    cp345: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >,
    cp345_witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    >,
) -> bool {
    if !predecessor_selects_none_case(predecessor) {
        return owner.is_none() && cp345.is_none() && cp345_witness.is_none();
    }
    let (Some(owner), Some(cp345), Some(cp345_witness)) = (owner, cp345, cp345_witness) else {
        return false;
    };
    let Some(owner_value) = owner.mixed_air_humidity_ratio else {
        return false;
    };
    cp345.system == predecessor.system
        && cp345.parent_call_ordinal == predecessor.parent_call_ordinal
        && cp345.controlled_zone == predecessor.controlled_zone
        && cp345.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        && cp345.mixed_air_humidity_ratio_read
        && option_bits_match(cp345.mixed_air_humidity_ratio, Some(owner_value))
        && cp345.supply_humidity_ratio_assignment_performed
        && option_bits_match(cp345.assigned_supply_humidity_ratio, Some(owner_value))
        && option_bits_match(
            predecessor.predecessor_assigned_supply_humidity_ratio,
            Some(owner_value),
        )
        && cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            cp345,
        )
        && cp345_snapshots_match_bit_exact(cp345, cp345_witness)
}

pub(super) fn none_case_links_to_predecessor(
    none_case:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) -> bool {
    none_case.system == predecessor.system
        && none_case.parent_call_ordinal == predecessor.parent_call_ordinal
        && none_case.controlled_zone == predecessor.controlled_zone
        && none_case.unit_body_entered == predecessor.unit_body_entered
        && none_case.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && none_case.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && none_case.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && none_case.unit_off_skipped == predecessor.unit_off_skipped
        && none_case.non_cooling_skipped == predecessor.non_cooling_skipped
        && none_case.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && none_case.predecessor_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_capacity_limit_guard_false_fallthrough
        && none_case.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && none_case
            .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        && none_case
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            == predecessor
                .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
        && option_bits_match(
            none_case.predecessor_assigned_supply_humidity_ratio,
            predecessor.predecessor_assigned_supply_humidity_ratio,
        )
        && none_case.predecessor_dehumidification_control_type_read
            == predecessor.dehumidification_control_type_read
        && none_case.predecessor_dehumidification_control_type
            == predecessor.dehumidification_control_type
        && none_case.predecessor_dehumidification_control_switch_dispatched
            == predecessor.dehumidification_control_switch_dispatched
}

pub(super) fn predecessor_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.predecessor_assigned_supply_humidity_ratio,
        right.predecessor_assigned_supply_humidity_ratio,
    );
    left.predecessor_assigned_supply_humidity_ratio = None;
    right.predecessor_assigned_supply_humidity_ratio = None;
    values_match && left == right
}

fn cp345_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
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

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
