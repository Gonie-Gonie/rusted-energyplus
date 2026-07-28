//! Persistent CP329 Cooling mixed-air call state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingMixedAirCallSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingMixedAirCallRetainedRoute {
    UnitOff,
    NonCooling,
    NoOutdoorAirFallback,
}

/// Persistent bounded state and source-site counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingMixedAirCallRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub cooling_call_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub caller_source_site_execution_count: usize,
    pub child_source_site_execution_count: usize,
    pub state_reference_bind_count: usize,
    pub purchased_air_number_read_count: usize,
    pub outdoor_air_mass_flow_rate_read_count: usize,
    pub supply_mass_flow_rate_read_count: usize,
    pub mixed_air_output_reference_bind_count: usize,
    pub operating_mode_read_count: usize,
    pub mixed_air_child_call_count: usize,
    pub no_outdoor_air_fallback_count: usize,
    pub recirculation_enthalpy_projection_count: usize,
    pub mixed_air_output_assignment_count: usize,
    pub heat_recovery_output_positive_zero_assignment_count: usize,
    pub latest: Option<PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    pub(super) latest_route: Option<PurchasedAirCalcCoolingMixedAirCallRetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingMixedAirCallRuntimeState {
    /// Creates zeroed CP329 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            cooling_call_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            caller_source_site_execution_count: 0,
            child_source_site_execution_count: 0,
            state_reference_bind_count: 0,
            purchased_air_number_read_count: 0,
            outdoor_air_mass_flow_rate_read_count: 0,
            supply_mass_flow_rate_read_count: 0,
            mixed_air_output_reference_bind_count: 0,
            operating_mode_read_count: 0,
            mixed_air_child_call_count: 0,
            no_outdoor_air_fallback_count: 0,
            recirculation_enthalpy_projection_count: 0,
            mixed_air_output_assignment_count: 0,
            heat_recovery_output_positive_zero_assignment_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
