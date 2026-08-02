//! Exact direct CP397 prefix validation for CP398.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_snapshots_match_bit_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState, classify_no_oa_sensible_subset,
    cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_snapshot_is_exact_direct_release,
};
use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let state = &unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry;
    let Some(latest) = state.latest else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witness(system.id)
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
        && cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_metadata_is_consistent(
            unit,
            ordinal,
        )
        && cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_snapshots_match_bit_exact(
            latest,
            predecessor,
        )
        && cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_snapshots_match_bit_exact(
            witness,
            predecessor,
        )
}

pub(super) fn case_entry_links_to_prefix(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state(
        &mut state,
        predecessor,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
