//! Retained CP406 prefix and exact CP378/CP385 operand-owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::transition::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentActiveOwners as ActiveOwners;
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_state as advance,
};
use crate::ideal_loads::calc::{
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent,
    completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_committed_latest_snapshot_is_consistent,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_latest_witness(system.id) else {
        return false;
    };
    crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshots_match_bit_exact(
            retained,
            predecessor,
        )
        && crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshots_match_bit_exact(
                witness,
                predecessor,
            )
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_committed_latest_snapshot_is_consistent(
            unit,
            system,
            retained,
            witness,
        )
}

pub(super) fn active_owners_from_retained_runtime(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveOwners> {
    let humidity = unit
        .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .latest?;
    let enthalpy = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
        .latest?;
    let humidity_witness =
        runtime.cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(system.id);
    let enthalpy_witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_witness(system.id);
    if !cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
        humidity,
    ) || !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(enthalpy)
        || !completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent(
            runtime,
            unit,
            system,
            humidity,
            humidity_witness,
        )
        || !completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            enthalpy,
            enthalpy_witness,
        )
        || !same_identity(predecessor, humidity.system, humidity.parent_call_ordinal, humidity.controlled_zone)
        || !same_identity(predecessor, enthalpy.system, enthalpy.parent_call_ordinal, enthalpy.controlled_zone)
        || !humidity_witness.is_some_and(|witness| {
            crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact(
                witness,
                humidity,
            )
        })
        || !enthalpy_witness.is_some_and(|witness| {
            crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshots_match_bit_exact(
                witness,
                enthalpy,
            )
        })
        || !active_owner_bits_are_exact(predecessor, humidity, enthalpy)
    {
        return None;
    }
    Some(ActiveOwners {
        supply_humidity_ratio_owner: humidity,
        supply_enthalpy_owner: enthalpy,
    })
}

pub(super) fn assignment_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, active_owners).is_some_and(|expected| {
        super::snapshot_validation::snapshots_match_bit_exact(expected, snapshot)
    })
}

pub(super) fn snapshot_operands_link_to_owners(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> bool {
    match active_owners {
        None => !snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
        Some(owners) => {
            active_owner_bits_are_exact(
                predecessor,
                owners.supply_humidity_ratio_owner,
                owners.supply_enthalpy_owner,
            ) && option_matches(
                snapshot.supply_humidity_ratio,
                owners
                    .supply_humidity_ratio_owner
                    .resulting_supply_humidity_ratio,
            ) && option_matches(
                snapshot.supply_enthalpy_j_per_kg,
                owners
                    .supply_enthalpy_owner
                    .resulting_supply_enthalpy_j_per_kg,
            )
        }
    }
}

fn active_owner_bits_are_exact(
    predecessor: Predecessor,
    humidity: crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    enthalpy: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    predecessor.resulting_supply_humidity_ratio.is_none()
        && option_matches(
            predecessor.resulting_supply_enthalpy_j_per_kg,
            enthalpy.resulting_supply_enthalpy_j_per_kg,
        )
        && humidity.resulting_supply_humidity_ratio.is_some()
        && enthalpy.resulting_supply_enthalpy_j_per_kg.is_some()
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

fn option_matches(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
