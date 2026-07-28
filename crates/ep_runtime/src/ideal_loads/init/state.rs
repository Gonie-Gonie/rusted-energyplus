//! Retained module and per-unit state for `InitPurchasedAir`.

mod unit;
mod witnesses;

use std::collections::BTreeMap;

use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneEquipmentListId, ZoneId};

use super::super::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    PurchasedAirCalcCoolingEntryGateRuntimeState,
    PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState, PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingSensibleFlowRuntimeState, PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot, PurchasedAirCalcEntryRuntimeState,
    PurchasedAirCalcMinimumOaPrefixRuntimeState, PurchasedAirHardSizeLegacyOutcome,
    PurchasedAirSizedLimits,
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
    cooling_economizer_condition_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingEconomizerConditionSnapshot>,
    cooling_economizer_body_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingEconomizerBodySnapshot>,
    cooling_sensible_flow_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSensibleFlowSnapshot>,
    cooling_dehumidification_flow_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
    cooling_humidification_flow_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidificationFlowSnapshot>,
    cooling_capacity_zero_flow_reset_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>,
    cooling_supply_mass_flow_maximum_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>,
    cooling_supply_mass_flow_ems_override_guard_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    >,
    cooling_supply_mass_flow_ems_override_body_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    >,
    cooling_supply_mass_flow_limit_guard_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot>,
    cooling_supply_mass_flow_limit_body_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot>,
    cooling_supply_mass_flow_very_small_guard_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    >,
    cooling_supply_mass_flow_very_small_guard_body_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    >,
    cooling_mixed_air_call_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    cooling_supply_mass_flow_positive_guard_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>,
    cooling_positive_supply_cp_air_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    >,
    cooling_positive_supply_temperature_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    >,
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
    /// Persistent bounded minimum-outdoor-air prefix state.
    pub calc_minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixRuntimeState,
    /// Persistent bounded cooling-entry gate state.
    pub calc_cooling_entry_gate: PurchasedAirCalcCoolingEntryGateRuntimeState,
    /// Persistent bounded cooling OA/max-flow gate state.
    pub calc_cooling_oa_max_flow_gate: PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    /// Persistent bounded cooling OA/max-flow warning-and-clamp body state.
    pub calc_cooling_oa_max_flow_body: PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    /// Persistent bounded cooling economizer outer-guard state.
    pub calc_cooling_economizer_guard: PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    /// Persistent bounded cooling economizer inner-condition state.
    pub calc_cooling_economizer_condition: PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    /// Persistent bounded cooling economizer true-body state.
    pub calc_cooling_economizer_body: PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    /// Persistent bounded cooling sensible-flow state.
    pub calc_cooling_sensible_flow: PurchasedAirCalcCoolingSensibleFlowRuntimeState,
    /// Persistent bounded cooling dehumidification-flow state.
    pub calc_cooling_dehumidification_flow: PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    /// Persistent bounded cooling humidification-flow state.
    pub calc_cooling_humidification_flow: PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    /// Persistent bounded cooling capacity-zero candidate-reset state.
    pub calc_cooling_capacity_zero_flow_reset:
        PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    /// Persistent bounded cooling supply-mass-flow maximum state.
    pub calc_cooling_supply_mass_flow_maximum:
        PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
    /// Persistent bounded cooling supply-mass-flow EMS-override guard state.
    pub calc_cooling_supply_mass_flow_ems_override_guard:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    /// Persistent bounded cooling supply-mass-flow EMS-override body state.
    pub calc_cooling_supply_mass_flow_ems_override_body:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    /// Persistent bounded cooling supply-mass-flow limit-guard state.
    pub calc_cooling_supply_mass_flow_limit_guard:
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    /// Persistent bounded cooling supply-mass-flow limit-body state.
    pub calc_cooling_supply_mass_flow_limit_body:
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    /// Persistent bounded cooling supply-mass-flow very-small-guard state.
    pub calc_cooling_supply_mass_flow_very_small_guard:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    /// Persistent bounded cooling supply-mass-flow positive-zero reset-body state.
    pub calc_cooling_supply_mass_flow_very_small_guard_body:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    /// Persistent bounded Cooling mixed-air call and no-OA fallback state.
    pub calc_cooling_mixed_air_call: PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    /// Persistent bounded cooling positive supply-mass-flow guard state.
    pub calc_cooling_supply_mass_flow_positive_guard:
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    /// Persistent bounded Cooling positive-supply `CpAir` assignment state.
    pub calc_cooling_positive_supply_cp_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    /// Persistent bounded Cooling positive-supply temperature-assignment state.
    pub calc_cooling_positive_supply_temperature_assignment:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
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
