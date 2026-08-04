//! Persistent CP413 saturation-guard state.

use ep_model::IdealLoadsAirSystemId;

use super::transition::routes::RetainedRoute;
use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot;

/// Persistent bounded state and exact CP412/CP413 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub saturation_supply_humidity_ratio_guard_evaluation_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub guard_false_fallthrough_route_counts: [usize; 36],
    pub guard_body_entry_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp412_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp412_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp412_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub cp412_saturation_supply_humidity_ratio_owned_read_count: usize,
    pub saturation_supply_humidity_ratio_for_guard_read_count: usize,
    pub cp411_original_supply_humidity_ratio_owned_read_count: usize,
    pub cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count: usize,
    pub original_supply_humidity_ratio_for_guard_read_count: usize,
    pub saturation_original_supply_humidity_ratio_comparison_count: usize,
    pub saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count:
        usize,
    pub saturation_supply_humidity_ratio_guard_body_entry_count: usize,
    pub saturation_supply_humidity_ratio_guard_false_fallthrough_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot>,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState {
    /// Creates zeroed CP413 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            saturation_supply_humidity_ratio_guard_evaluation_count: 0,
            predecessor_route_counts: [0; 36],
            guard_false_fallthrough_route_counts: [0; 36],
            guard_body_entry_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp412_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp412_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp412_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            cp412_saturation_supply_humidity_ratio_owned_read_count: 0,
            saturation_supply_humidity_ratio_for_guard_read_count: 0,
            cp411_original_supply_humidity_ratio_owned_read_count: 0,
            cp412_same_call_original_supply_humidity_ratio_bit_corroboration_count: 0,
            original_supply_humidity_ratio_for_guard_read_count: 0,
            saturation_original_supply_humidity_ratio_comparison_count: 0,
            saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count: 0,
            saturation_supply_humidity_ratio_guard_body_entry_count: 0,
            saturation_supply_humidity_ratio_guard_false_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
