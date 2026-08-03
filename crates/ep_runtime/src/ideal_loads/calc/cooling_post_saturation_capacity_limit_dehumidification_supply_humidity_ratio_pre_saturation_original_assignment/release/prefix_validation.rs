//! Exact direct CP410 prefix validation for CP411.

use super::super::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_state,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_is_consistent;
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
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_is_consistent(
        runtime, unit, system, predecessor,
        runtime.cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_latest_witness(system.id),
    )
}

pub(super) fn original_assignment_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_state(&mut state, predecessor)
        .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
