//! Exact direct CP393 prefix validation for CP394.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_committed_latest_snapshot_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};
use ep_model::IdealLoadsAirSystem;

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_latest_witness(system.id)
    else {
        return false;
    };
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_snapshots_match_bit_exact(
        retained,
        predecessor,
    ) && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_snapshots_match_bit_exact(
        witness,
        predecessor,
    ) && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_committed_latest_snapshot_is_consistent(
        unit,
        system,
        witness,
    )
}

pub(super) fn case_entry_links_to_prefix(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(
        &mut state,
        predecessor,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
