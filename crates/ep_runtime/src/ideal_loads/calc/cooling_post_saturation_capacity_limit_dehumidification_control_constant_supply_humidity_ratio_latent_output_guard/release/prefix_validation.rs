//! CP401 predecessor and CP321/CP340 operand-owner validation for CP402.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_state,
};
use super::snapshot_validation::{option_bits_match, snapshots_match_bit_exact};
use crate::ideal_loads::calc::{
    completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_is_consistent,
    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshots_match_bit_exact as cp340_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshots_match_bit_exact as cp401_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release,
    cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment;
    let Some(latest) = state.latest else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_latest_witness(system.id)
    else {
        return false;
    };
    let Some(calc_entry_latest) = unit.calc_entry.latest else {
        return false;
    };
    let ordinal = predecessor.parent_call_ordinal;
    classify_no_oa_sensible_subset(system).is_supported()
        && system.dehumidification_control_type == DehumidificationControlType::None
        && system.humidification_control_type == HumidificationControlType::None
        && system.id == predecessor.system
        && unit.system == system.id
        && state.system == system.id
        && unit.topology_completed
        && unit.topology_failure.is_none()
        && unit.controlled_zone == Some(predecessor.controlled_zone)
        && ordinal > 0
        && unit.init_call_count == ordinal
        && unit.calc_entry.call_count == ordinal
        && calc_entry_latest.system == system.id
        && calc_entry_latest.call_ordinal == ordinal
        && calc_entry_latest.controlled_zone == predecessor.controlled_zone
        && state.transition_count == ordinal
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_latest_metadata_is_consistent(unit, ordinal)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release(predecessor)
        && cp401_snapshots_match_bit_exact(latest, predecessor)
        && cp401_snapshots_match_bit_exact(witness, predecessor)
        && completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn retained_active_input(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<Option<ActiveInput>> {
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    if !route.active {
        return Some(None);
    }
    let cooling_latent_output_w = predecessor.cooling_latent_output_w?;
    let cp321 = unit.calc_cooling_capacity_zero_flow_reset.latest?;
    let cp340 = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_guard
        .latest?;
    let cp340_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witness(system.id)?;
    let maximum_total_cooling_capacity_w = cp321.maximum_total_cooling_capacity_w?;
    let same_call = [
        (cp321.system, cp321.parent_call_ordinal, cp321.controlled_zone),
        (cp340.system, cp340.parent_call_ordinal, cp340.controlled_zone),
    ]
    .into_iter()
    .all(|(owner_system, ordinal, zone)| {
        owner_system == predecessor.system
            && ordinal == predecessor.parent_call_ordinal
            && zone == predecessor.controlled_zone
    });
    let cp321_is_owner = cp321.cooling_body_entered
        && cp321.cooling_limit_condition_satisfied == Some(true)
        && cp321.maximum_total_cooling_capacity_read
        && maximum_total_cooling_capacity_w.is_finite()
        && maximum_total_cooling_capacity_w >= 0.0
        && cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(cp321);
    let cp340_corroborates = cp340.capacity_limit_sensible_output_guard_evaluated
        && cp340.maximum_total_cooling_capacity_read
        && cp340.maximum_total_cooling_capacity_w.is_some_and(|value| {
            value.to_bits() == maximum_total_cooling_capacity_w.to_bits()
        })
        && cooling_positive_supply_capacity_limit_sensible_output_guard_snapshot_is_exact_direct_release(cp340)
        && cp340_snapshots_match_bit_exact(cp340, cp340_witness)
        && completed_direct_cooling_positive_supply_capacity_limit_sensible_output_guard_is_consistent(
            runtime,
            unit,
            system,
            cp340,
            Some(cp340_witness),
        );
    if !same_call || !cp321_is_owner || !cp340_corroborates {
        return None;
    }
    Some(Some(ActiveInput {
        cooling_latent_output_w,
        maximum_total_cooling_capacity_w,
        cp401_cooling_latent_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    }))
}

pub(super) fn guard_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_state(
        &mut state,
        predecessor,
        input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(super) fn snapshot_operands_link_to_owners(
    snapshot: Snapshot,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> bool {
    match input {
        Some(input) => {
            option_bits_match(snapshot.cooling_latent_output_w, predecessor.cooling_latent_output_w)
                && option_bits_match(
                    snapshot.cooling_latent_output_w,
                    Some(input.cooling_latent_output_w),
                )
                && option_bits_match(
                    snapshot.maximum_total_cooling_capacity_w,
                    Some(input.maximum_total_cooling_capacity_w),
                )
        }
        None => {
            snapshot.cooling_latent_output_w.is_none()
                && snapshot.maximum_total_cooling_capacity_w.is_none()
                && snapshot
                    .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity
                    .is_none()
        }
    }
}
