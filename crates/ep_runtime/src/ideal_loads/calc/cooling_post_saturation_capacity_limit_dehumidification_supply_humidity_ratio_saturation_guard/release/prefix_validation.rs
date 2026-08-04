//! Exact direct CP412 prefix validation for CP413.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_latest_witness(system.id),
    )
}

pub(super) fn snapshot_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state(
        &mut state,
        predecessor,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
