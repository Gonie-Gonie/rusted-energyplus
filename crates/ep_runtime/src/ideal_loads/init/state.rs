//! Retained module and per-unit state for `InitPurchasedAir`.

use std::collections::BTreeMap;

use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneEquipmentListId, ZoneId};

use super::super::{
    PurchasedAirCalcEntryRuntimeState, PurchasedAirHardSizeLegacyOutcome, PurchasedAirSizedLimits,
};
use super::{
    IdealLoadsInitFlags, PurchasedAirInitTopologyDiagnostic, PurchasedAirInitTopologyError,
    PurchasedAirInitTopologyPlan, PurchasedAirRecirculationSource,
    PurchasedAirSupplyTemperatureDiagnosticRegistry,
};

/// Structured diagnostic emitted by the manager-wide equipment-list sweep.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitDiagnostic {
    /// Unit visited by the manager sweep.
    pub system: IdealLoadsAirSystemId,
    /// One-based declaration-order visit ordinal.
    pub scan_ordinal: usize,
    /// Source-shaped diagnostic category.
    pub kind: PurchasedAirInitDiagnosticKind,
}

/// Diagnostic categories retained by the bounded manager-wide sweep.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirInitDiagnosticKind {
    /// `CheckZoneEquipmentList` found no matching entry in any equipment list.
    EquipmentListMembershipMissing,
}

/// Mutable state retained across PurchasedAir initialization calls.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PurchasedAirRuntimeState {
    /// Whether the source-shaped per-unit flag arena has been allocated.
    pub module_initialized: bool,
    /// Whether Zone equipment-list membership has been checked.
    pub equipment_list_checked: bool,
    /// IdealLoads systems in immutable typed declaration order.
    pub declared_system_order: Vec<IdealLoadsAirSystemId>,
    /// Systems visited by the one manager-wide equipment-list sweep.
    pub equipment_list_scan_order: Vec<IdealLoadsAirSystemId>,
    /// Ordered source-shaped severe diagnostics emitted by the sweep.
    pub equipment_list_diagnostics: Vec<PurchasedAirInitDiagnostic>,
    /// Rust-owned bounded registry for supply-temperature recurring identities.
    pub supply_temperature_diagnostic_registry: PurchasedAirSupplyTemperatureDiagnosticRegistry,
    /// Per-system lifecycle state in typed-ID order.
    pub units: BTreeMap<IdealLoadsAirSystemId, PurchasedAirUnitRuntimeState>,
    /// Number of module arena allocations.
    pub module_initialization_count: usize,
    /// Number of completed global equipment-list checks.
    pub equipment_list_check_count: usize,
    /// Total units visited by the manager-wide equipment-list sweep.
    pub equipment_list_scanned_unit_count: usize,
    /// Units missing from every Zone equipment list during the sweep.
    pub equipment_list_missing_unit_count: usize,
}

/// Persistent `InitPurchasedAir` state for one IdealLoads system.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirUnitRuntimeState {
    /// Typed system identity.
    pub system: IdealLoadsAirSystemId,
    /// Source one-time latch, committed before semantic topology validation.
    pub one_time_latched: bool,
    /// Whether the selected-unit topology pass reached its normal tail.
    pub topology_completed: bool,
    /// Source `MySizeFlag`; true means the hard-size/sizing gate is pending.
    pub sizing_needed: bool,
    /// Four-field PurchasedAir object overlay seeded after topology succeeds.
    pub sized_limits: Option<PurchasedAirSizedLimits>,
    /// Successful direct hard-size child outcome retained for downstream stages.
    pub sizing_outcome: Option<PurchasedAirHardSizeLegacyOutcome>,
    /// Source `MyEnvrnFlag`; true means a begin-environment write is pending.
    pub environment_initialization_needed: bool,
    /// Controlled Zone captured by the one-time topology pass.
    pub controlled_zone: Option<ZoneId>,
    /// Equipment list captured by the one-time topology pass.
    pub equipment_list: Option<ZoneEquipmentListId>,
    /// Supply node captured by the one-time topology pass.
    pub supply_node: Option<NodeId>,
    /// Exhaust-or-return recirculation node captured by the one-time pass.
    pub recirculation_node: Option<NodeId>,
    /// Source branch that selected or left recirculation unassigned.
    pub recirculation_source: Option<PurchasedAirRecirculationSource>,
    /// Persistent bounded `CalcPurchAirLoads` entry-prefix state.
    pub calc_entry: PurchasedAirCalcEntryRuntimeState,
    /// Configured exhaust rejected before return fallback.
    pub rejected_exhaust_node: Option<NodeId>,
    /// First return node named by the source multiple-return warning.
    pub reported_first_return_node: Option<NodeId>,
    /// Immutable selected-unit topology retained by the first call.
    pub topology_plan: Option<PurchasedAirInitTopologyPlan>,
    /// Ordered diagnostics retained from the one-time topology block.
    pub topology_diagnostics: Vec<PurchasedAirInitTopologyDiagnostic>,
    /// Fatal topology result retained after the source latch is committed.
    pub topology_failure: Option<PurchasedAirInitTopologyError>,
    /// Immutable first-match result captured when the manager arena is allocated.
    pub planned_first_matching_equipment_list: Option<ZoneEquipmentListId>,
    /// One-based manager sweep ordinal, once Zone equipment input is ready.
    pub equipment_list_scan_ordinal: Option<usize>,
    /// First controlled-Zone-referenced equipment list containing this unit.
    pub first_matching_equipment_list: Option<ZoneEquipmentListId>,
    /// Whether the manager sweep found this unit in any equipment list.
    pub equipment_list_membership_found: Option<bool>,
    /// Cached maximum heating air mass flow from begin-environment initialization.
    pub maximum_heating_air_mass_flow_rate_kg_per_s: f64,
    /// Cached maximum cooling air mass flow from begin-environment initialization.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    /// Standard air density used for the cached environment values.
    pub standard_air_density_kg_per_m3: Option<f64>,
    /// Total calls for this unit.
    pub init_call_count: usize,
    /// Source one-time latch transitions.
    pub one_time_initialization_count: usize,
    /// One-time topology blocks that reached their normal tail.
    pub topology_completion_count: usize,
    /// Completed hard-size/sizing gates.
    pub sizing_check_count: usize,
    /// Hard-size/sizing child attempts, including fail-closed returns.
    pub sizing_attempt_count: usize,
    /// Completed begin-environment writes.
    pub environment_initialization_count: usize,
    /// False-begin-environment calls that rearmed the environment latch.
    pub environment_rearm_count: usize,
    /// Source `CoolErrIndex`; zero means no recurring identity is registered.
    pub cooling_supply_temperature_error_index: usize,
    /// Source `HeatErrIndex`; zero means no recurring identity is registered.
    pub heating_supply_temperature_error_index: usize,
    /// First detailed cooling diagnostic groups emitted.
    pub cooling_supply_temperature_first_diagnostic_count: usize,
    /// First detailed heating diagnostic groups emitted.
    pub heating_supply_temperature_first_diagnostic_count: usize,
    /// Active cooling supply-temperature recurring diagnostic count.
    pub cooling_supply_temperature_warning_count: usize,
    /// Active heating supply-temperature recurring diagnostic count.
    pub heating_supply_temperature_warning_count: usize,
    /// Nonfatal OA/economizer flow-limit advisories emitted once.
    pub economizer_flow_limit_warning_count: usize,
}

impl PurchasedAirUnitRuntimeState {
    pub(super) const fn new(
        system: IdealLoadsAirSystemId,
        planned_first_matching_equipment_list: Option<ZoneEquipmentListId>,
    ) -> Self {
        Self {
            system,
            one_time_latched: false,
            topology_completed: false,
            sizing_needed: true,
            sized_limits: None,
            sizing_outcome: None,
            environment_initialization_needed: true,
            controlled_zone: None,
            equipment_list: None,
            supply_node: None,
            recirculation_node: None,
            recirculation_source: None,
            calc_entry: PurchasedAirCalcEntryRuntimeState::new(system),
            rejected_exhaust_node: None,
            reported_first_return_node: None,
            topology_plan: None,
            topology_diagnostics: Vec::new(),
            topology_failure: None,
            planned_first_matching_equipment_list,
            equipment_list_scan_ordinal: None,
            first_matching_equipment_list: None,
            equipment_list_membership_found: None,
            maximum_heating_air_mass_flow_rate_kg_per_s: 0.0,
            maximum_cooling_air_mass_flow_rate_kg_per_s: 0.0,
            standard_air_density_kg_per_m3: None,
            init_call_count: 0,
            one_time_initialization_count: 0,
            topology_completion_count: 0,
            sizing_check_count: 0,
            sizing_attempt_count: 0,
            environment_initialization_count: 0,
            environment_rearm_count: 0,
            cooling_supply_temperature_error_index: 0,
            heating_supply_temperature_error_index: 0,
            cooling_supply_temperature_first_diagnostic_count: 0,
            heating_supply_temperature_first_diagnostic_count: 0,
            cooling_supply_temperature_warning_count: 0,
            heating_supply_temperature_warning_count: 0,
            economizer_flow_limit_warning_count: 0,
        }
    }

    /// Source-shaped flag snapshot after the latest call.
    #[must_use]
    pub fn flags(&self, equipment_list_checked: bool) -> IdealLoadsInitFlags {
        IdealLoadsInitFlags {
            state_machine_used: true,
            one_time_checked: self.one_time_latched,
            topology_ready: self.topology_completed && self.recirculation_node.is_some(),
            environment_initialized: self.environment_initialization_count > 0,
            environment_initialization_needed: self.environment_initialization_needed,
            sizing_checked: !self.sizing_needed,
            equipment_list_checked,
            return_plenum_inactive: true,
        }
    }
}
