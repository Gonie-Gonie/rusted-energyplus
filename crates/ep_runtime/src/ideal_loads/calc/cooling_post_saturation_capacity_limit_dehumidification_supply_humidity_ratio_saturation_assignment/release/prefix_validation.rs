//! Exact direct CP411 prefix validation for CP412.

use ep_model::IdealLoadsAirSystem;

use super::super::transition::routes::{predecessor_route, route_is_active};
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent(
        runtime,
        unit,
        system,
        predecessor,
        runtime.cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(system.id),
    )
}

pub(super) fn snapshot_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let input = if route_is_active(route) {
        let Some(outdoor_barometric_pressure_pa) = snapshot.outdoor_barometric_pressure_pa else {
            return false;
        };
        Some(ActiveInput {
            outdoor_barometric_pressure_pa,
        })
    } else {
        None
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state(
        &mut state,
        predecessor,
        input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
