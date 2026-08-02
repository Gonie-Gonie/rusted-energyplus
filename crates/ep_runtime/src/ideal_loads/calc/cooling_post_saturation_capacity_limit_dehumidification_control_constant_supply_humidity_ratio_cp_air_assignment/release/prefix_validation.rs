//! Bounded CP398 predecessor and CP329 operand-owner validation for CP399.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state,
};
use super::snapshot_validation::{option_bits_match, snapshots_match_bit_exact};
use crate::ideal_loads::calc::completed_direct_cooling_mixed_air_call_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let state = &unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry;
    let Some(latest) = state.latest else { return false; };
    let Some(witness) = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_latest_witness(system.id) else { return false; };
    let Some(calc_entry_latest) = unit.calc_entry.latest else { return false; };
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
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_latest_metadata_is_consistent(unit, ordinal)
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(predecessor)
        && crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact(latest, predecessor)
        && crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshots_match_bit_exact(witness, predecessor)
}

pub(super) fn assignment_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state(
        &mut state,
        predecessor,
        active_input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}

pub(super) fn active_input_from_retained_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<ActiveInput> {
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    if !route.active {
        return None;
    }
    let owner = unit.calc_cooling_mixed_air_call.latest?;
    let owner_witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    let operand = owner.mixed_air_humidity_ratio?;
    if owner.system != predecessor.system
        || owner.parent_call_ordinal != predecessor.parent_call_ordinal
        || owner.controlled_zone != predecessor.controlled_zone
        || !owner.cooling_call_executed
        || !owner.no_outdoor_air_fallback_entered
        || !owner.mixed_air_humidity_ratio_assigned
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(owner)
        || !cooling_mixed_air_call_snapshots_match_bit_exact(owner, owner_witness)
        || !completed_direct_cooling_mixed_air_call_is_consistent(
            runtime,
            unit,
            system,
            owner,
            Some(owner_witness),
        )
        || !operand.is_finite()
        || operand < 0.0
        || !energyplus_psy_cp_air_fn_w(operand).is_finite()
    {
        return None;
    }
    Some(ActiveInput { mixed_air_humidity_ratio: operand })
}

pub(super) fn snapshot_operand_links_to_owner(snapshot: Snapshot, active_input: Option<ActiveInput>) -> bool {
    match active_input {
        Some(active) => option_bits_match(snapshot.mixed_air_humidity_ratio, Some(active.mixed_air_humidity_ratio)),
        None => snapshot.mixed_air_humidity_ratio.is_none(),
    }
}
