//! Persistent CP411 post-capacity-limit pre-saturation local-copy state.

use ep_model::IdealLoadsAirSystemId;

use super::transition::routes::RetainedRoute;
use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot;

/// Persistent bounded state and exact CP410/CP411 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub predecessor_guard_false_fallthrough_count: usize,
    pub predecessor_maximum_capacity_assignment_count: usize,
    pub supply_humidity_ratio_pre_saturation_original_assignment_count: usize,
    pub predecessor_route_counts: [usize; 30],
    pub predecessor_guard_false_fallthrough_route_counts: [usize; 30],
    pub predecessor_maximum_capacity_assignment_route_counts: [usize; 30],
    pub supply_humidity_ratio_pre_saturation_original_assignment_route_counts: [usize; 30],
    pub source_site_execution_count: usize,
    pub cp410_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp410_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp410_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp410_retained_supply_humidity_ratio_owned_read_count: usize,
    pub purchased_air_supply_humidity_ratio_before_saturation_limit_read_count: usize,
    pub local_supply_humidity_ratio_original_assignment_write_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState {
    /// Creates zeroed CP411 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            predecessor_guard_false_fallthrough_count: 0,
            predecessor_maximum_capacity_assignment_count: 0,
            supply_humidity_ratio_pre_saturation_original_assignment_count: 0,
            predecessor_route_counts: [0; 30],
            predecessor_guard_false_fallthrough_route_counts: [0; 30],
            predecessor_maximum_capacity_assignment_route_counts: [0; 30],
            supply_humidity_ratio_pre_saturation_original_assignment_route_counts: [0; 30],
            source_site_execution_count: 0,
            cp410_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp410_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp410_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp410_retained_supply_humidity_ratio_owned_read_count: 0,
            purchased_air_supply_humidity_ratio_before_saturation_limit_read_count: 0,
            local_supply_humidity_ratio_original_assignment_write_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
