//! Persistent CP442 first-warning guard else-branch entry state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot as Snapshot;
use super::transition::PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRetainedRoute as Route;

/// Persistent bounded state and exact CP441/CP442 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState
{
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts:
        [usize; 36],
    pub source_site_execution_count: usize,
    pub cp441_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp441_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp441_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntryRuntimeState {
    /// Creates zeroed CP442 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_count: 0,
            predecessor_route_counts: [0; 36],
            heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_route_counts: [0;
                36],
            source_site_execution_count: 0,
            cp441_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp441_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp441_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
