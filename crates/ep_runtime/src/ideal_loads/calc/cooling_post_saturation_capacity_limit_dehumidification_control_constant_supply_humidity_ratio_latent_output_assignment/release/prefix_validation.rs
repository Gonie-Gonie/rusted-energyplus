//! Bounded CP400 predecessor and CP384/CP385 owner validation for CP401.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentActiveOwners as ActiveOwners,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state,
};
use super::snapshot_validation::{option_bits_match, snapshots_match_bit_exact};
use crate::ideal_loads::calc::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_is_consistent,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_is_consistent,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;
    let Some(latest) = state.latest else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_latest_witness(system.id)
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
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_latest_metadata_is_consistent(unit, ordinal)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshots_match_bit_exact(latest, predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshots_match_bit_exact(witness, predecessor)
        && completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn assignment_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state(
        &mut state,
        predecessor,
        active_owners,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(super) fn active_owners_from_retained_runtime(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOwners> {
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    if !route.active {
        return None;
    }
    let owner = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment
        .latest?;
    let owner_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_witness(system.id)?;
    let corroborator = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
        .latest?;
    let corroborator_witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_witness(system.id)?;
    if !same_identity(
        predecessor,
        owner.system,
        owner.parent_call_ordinal,
        owner.controlled_zone,
    ) || !same_identity(
        predecessor,
        corroborator.system,
        corroborator.parent_call_ordinal,
        corroborator.controlled_zone,
    ) || !cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshots_match_bit_exact(
        owner,
        owner_witness,
    ) || !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshots_match_bit_exact(
        corroborator,
        corroborator_witness,
    ) || !cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(owner)
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(corroborator)
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_metadata_is_consistent(
            unit,
            predecessor.parent_call_ordinal,
        )
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_metadata_is_consistent(
            unit,
            predecessor.parent_call_ordinal,
        )
        || !completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_is_consistent(
            runtime,
            unit,
            system,
            owner,
            Some(owner_witness),
        )
        || !completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            corroborator,
            Some(corroborator_witness),
        )
    {
        return None;
    }
    Some(ActiveOwners {
        cooling_total_output_owner: owner,
        cooling_total_output_corroborator: corroborator,
    })
}

pub(super) fn snapshot_operands_link_to_owners(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> bool {
    match active_owners {
        Some(owners) => {
            option_bits_match(
                snapshot.cooling_total_output_w,
                owners
                    .cooling_total_output_owner
                    .resulting_cooling_total_output_w,
            ) && option_bits_match(
                snapshot.cooling_total_output_w,
                owners
                    .cooling_total_output_corroborator
                    .cooling_total_output_w,
            ) && option_bits_match(
                snapshot.cooling_sensible_output_w,
                predecessor.cooling_sensible_output_w,
            )
        }
        None => {
            snapshot.cooling_total_output_w.is_none()
                && snapshot.cooling_sensible_output_w.is_none()
                && snapshot.calculated_cooling_latent_output_w.is_none()
                && snapshot.cooling_latent_output_w.is_none()
        }
    }
}

fn same_identity(
    predecessor: Predecessor,
    system: ep_model::IdealLoadsAirSystemId,
    ordinal: usize,
    zone: ep_model::ZoneId,
) -> bool {
    predecessor.system == system
        && predecessor.parent_call_ordinal == ordinal
        && predecessor.controlled_zone == zone
}
