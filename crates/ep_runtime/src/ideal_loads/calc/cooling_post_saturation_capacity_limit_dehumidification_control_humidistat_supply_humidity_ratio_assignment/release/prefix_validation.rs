//! Exact direct CP394 prefix validation for CP395.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};
use ep_model::IdealLoadsAirSystem;

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_latest_witness(system.id),
    )
}

pub(super) fn assignment_links_to_prefix(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
        &mut state,
        predecessor,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
