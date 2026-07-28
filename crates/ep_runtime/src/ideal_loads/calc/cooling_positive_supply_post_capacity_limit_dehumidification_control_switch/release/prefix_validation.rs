//! CP319/CP345-to-CP346 retained-lineage validation.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem};

use super::super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirUnitRuntimeState, cooling_dehumidification_flow_snapshot_is_exact_direct_release,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn predecessor_is_active(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    predecessor.post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
}

pub(super) fn active_input_from_owner(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    system: &IdealLoadsAirSystem,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput,
> {
    predecessor_is_active(predecessor).then_some(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput {
            dehumidification_control_type: system.dehumidification_control_type,
        },
    )
}

pub(super) fn switch_links_to_predecessor(
    switch:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let active = predecessor_is_active(predecessor);
    switch.system == predecessor.system
        && switch.parent_call_ordinal == predecessor.parent_call_ordinal
        && switch.controlled_zone == predecessor.controlled_zone
        && switch.unit_body_entered == predecessor.unit_body_entered
        && switch.predecessor_cooling_body_entered == predecessor.predecessor_cooling_body_entered
        && switch.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && switch.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && switch.unit_off_skipped == predecessor.unit_off_skipped
        && switch.non_cooling_skipped == predecessor.non_cooling_skipped
        && switch.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && switch.predecessor_capacity_limit_guard_false_fallthrough
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && switch.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        && switch
            .predecessor_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
            == predecessor
                .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
        && switch
            .predecessor_post_capacity_limit_supply_humidity_ratio_mixed_air_assignment_executed
            == active
        && option_bits_match(
            switch.predecessor_assigned_supply_humidity_ratio,
            predecessor.assigned_supply_humidity_ratio,
        )
}

pub(super) fn predecessor_snapshots_match_bit_exact(
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

pub(super) fn active_cp319_corroborates_owner(
    unit: &PurchasedAirUnitRuntimeState,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    owner: DehumidificationControlType,
    cp319: Option<PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
    cp319_witness: Option<PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
) -> bool {
    if !predecessor_is_active(predecessor) {
        // In particular, P has a CP319 selector read but skips CP346. Never
        // equate the two aggregate selector-read histories.
        return true;
    }
    let (Some(cp319), Some(cp319_witness), Some(cp318)) =
        (cp319, cp319_witness, unit.calc_cooling_sensible_flow.latest)
    else {
        return false;
    };
    cp319 == cp319_witness
        && cp319.system == predecessor.system
        && cp319.parent_call_ordinal == predecessor.parent_call_ordinal
        && cp319.controlled_zone == predecessor.controlled_zone
        && cp319.dehumidification_control_type_read
        && cp319.dehumidification_control_type == Some(owner)
        && cooling_dehumidification_flow_snapshot_is_exact_direct_release(cp319)
        && super::super::super::cooling_dehumidification_flow::release::completed_direct_cooling_dehumidification_flow_is_consistent(
            unit,
            cp318,
            cp319,
            Some(cp319_witness),
        )
}

pub(super) fn predecessor_is_exact_direct(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
        predecessor,
    )
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
