//! Persistent CP386 post-saturation switch-dispatch state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot;
use super::transition::routes::RetainedRoute;

/// Persistent bounded state and exact CP385/CP386 route accounting for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub dehumidification_control_switch_count: usize,
    pub predecessor_route_counts: [usize; 23],
    pub source_site_execution_count: usize,
    pub dehumidification_control_type_read_count: usize,
    pub dehumidification_control_switch_dispatch_count: usize,
    pub dehumidification_control_constant_sensible_heat_ratio_case_selection_count: usize,
    pub dehumidification_control_humidistat_case_selection_count: usize,
    pub dehumidification_control_none_case_selection_count: usize,
    pub dehumidification_control_constant_supply_humidity_ratio_case_selection_count: usize,
    pub latest: Option<
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot,
    >,
    pub(super) latest_route: Option<RetainedRoute>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState {
    /// Creates zeroed CP386 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            dehumidification_control_switch_count: 0,
            predecessor_route_counts: [0; 23],
            source_site_execution_count: 0,
            dehumidification_control_type_read_count: 0,
            dehumidification_control_switch_dispatch_count: 0,
            dehumidification_control_constant_sensible_heat_ratio_case_selection_count: 0,
            dehumidification_control_humidistat_case_selection_count: 0,
            dehumidification_control_none_case_selection_count: 0,
            dehumidification_control_constant_supply_humidity_ratio_case_selection_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
