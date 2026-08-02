//! Bounded CP399 predecessor and CP329/CP330 owner validation for CP400.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentActiveOwners as ActiveOwners,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state,
};
use super::snapshot_validation::{option_bits_match, snapshots_match_bit_exact};
use crate::ideal_loads::calc::{
    completed_direct_cooling_mixed_air_call_is_consistent,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_is_consistent,
    completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshots_match_bit_exact,
    cooling_supply_mass_flow_positive_guard_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
    let Some(latest) = state.latest else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_latest_witness(system.id)
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
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_latest_metadata_is_consistent(unit, ordinal)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshots_match_bit_exact(latest, predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshots_match_bit_exact(witness, predecessor)
        && completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_is_consistent(
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
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state(
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
    let mixed = unit.calc_cooling_mixed_air_call.latest?;
    let mixed_witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    let flow = unit.calc_cooling_supply_mass_flow_positive_guard.latest?;
    let flow_witness = runtime.cooling_supply_mass_flow_positive_guard_latest_witness(system.id)?;
    if !same_identity(
        predecessor,
        mixed.system,
        mixed.parent_call_ordinal,
        mixed.controlled_zone,
    ) || !same_identity(
        predecessor,
        flow.system,
        flow.parent_call_ordinal,
        flow.controlled_zone,
    ) || !cooling_mixed_air_call_snapshots_match_bit_exact(mixed, mixed_witness)
        || !cooling_supply_mass_flow_positive_guard_snapshots_match_bit_exact(flow, flow_witness)
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed)
        || !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(flow)
        || !completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            mixed,
            Some(mixed_witness),
        )
        || !completed_direct_cooling_supply_mass_flow_positive_guard_is_consistent(
            runtime,
            unit,
            system,
            flow,
            Some(flow_witness),
        )
    {
        return None;
    }
    Some(ActiveOwners {
        mixed_air_owner: mixed,
        supply_mass_flow_owner: flow,
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
                snapshot.supply_mass_flow_rate_kg_per_s,
                owners.supply_mass_flow_owner.supply_mass_flow_rate_kg_per_s,
            ) && option_bits_match(
                snapshot.mixed_air_temperature_c,
                owners.mixed_air_owner.mixed_air_temperature_c,
            ) && option_bits_match(snapshot.cp_air_j_per_kg_k, predecessor.cp_air_j_per_kg_k)
                && option_bits_match(
                    snapshot.supply_temperature_c,
                    predecessor.resulting_supply_temperature_c,
                )
        }
        None => {
            snapshot.supply_mass_flow_rate_kg_per_s.is_none()
                && snapshot.mixed_air_temperature_c.is_none()
                && snapshot.cp_air_j_per_kg_k.is_none()
                && snapshot.supply_temperature_c.is_none()
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
