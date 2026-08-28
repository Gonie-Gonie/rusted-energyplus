//! Persistent CP430 heating-or-no-load case-entry state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot as Snapshot;
use super::transition::PurchasedAirCalcHeatingOrNoLoadCaseEntryRetainedRoute as Route;

/// Persistent bounded state and exact CP429/CP430 route accounting.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub inactive_transition_count: usize,
    pub heating_or_no_load_case_entry_count: usize,
    pub predecessor_route_counts: [usize; 36],
    pub heating_or_no_load_case_entry_route_counts: [usize; 36],
    pub source_site_execution_count: usize,
    pub cp429_supply_humidity_ratio_state_owner_count: usize,
    pub unchanged_supply_humidity_ratio_preservation_count: usize,
    pub cp429_supply_enthalpy_state_owner_count: usize,
    pub unchanged_supply_enthalpy_preservation_count: usize,
    pub cp429_supply_temperature_state_owner_count: usize,
    pub unchanged_supply_temperature_preservation_count: usize,
    pub latest: Option<Snapshot>,
    pub(super) latest_route: Option<Route>,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcHeatingOrNoLoadCaseEntryRuntimeState {
    /// Creates zeroed CP430 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            inactive_transition_count: 0,
            heating_or_no_load_case_entry_count: 0,
            predecessor_route_counts: [0; 36],
            heating_or_no_load_case_entry_route_counts: [0; 36],
            source_site_execution_count: 0,
            cp429_supply_humidity_ratio_state_owner_count: 0,
            unchanged_supply_humidity_ratio_preservation_count: 0,
            cp429_supply_enthalpy_state_owner_count: 0,
            unchanged_supply_enthalpy_preservation_count: 0,
            cp429_supply_temperature_state_owner_count: 0,
            unchanged_supply_temperature_preservation_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
