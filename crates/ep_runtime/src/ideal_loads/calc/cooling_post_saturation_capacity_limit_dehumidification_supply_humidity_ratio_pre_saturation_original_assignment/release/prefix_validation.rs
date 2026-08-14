//! Exact direct CP410 prefix validation for CP411.

use super::super::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_state,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_committed_latest_snapshot_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};
use ep_model::IdealLoadsAirSystem;

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_latest_witness(system.id)
        .is_some_and(|witness| {
            cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshots_match_bit_exact(
                witness,
                predecessor,
            ) && cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_committed_latest_snapshot_is_consistent(
                unit,
                witness,
            )
        })
}

pub(super) fn original_assignment_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_state(&mut state, predecessor)
        .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
