//! Retained module and per-unit state for `InitPurchasedAir`.

use std::collections::BTreeMap;

use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneEquipmentListId, ZoneId};

use super::IdealLoadsInitFlags;

/// Mutable state retained across PurchasedAir initialization calls.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PurchasedAirRuntimeState {
    /// Whether the source-shaped per-unit flag arena has been allocated.
    pub module_initialized: bool,
    /// Whether Zone equipment-list membership has been checked.
    pub equipment_list_checked: bool,
    /// Per-system lifecycle state in typed-ID order.
    pub units: BTreeMap<IdealLoadsAirSystemId, PurchasedAirUnitRuntimeState>,
    /// Number of module arena allocations.
    pub module_initialization_count: usize,
    /// Number of completed global equipment-list checks.
    pub equipment_list_check_count: usize,
}

/// Persistent `InitPurchasedAir` state for one IdealLoads system.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirUnitRuntimeState {
    /// Typed system identity.
    pub system: IdealLoadsAirSystemId,
    /// Whether one-time topology binding has been latched.
    pub one_time_initialized: bool,
    /// Source `MySizeFlag`; true means the hard-size/sizing gate is pending.
    pub sizing_needed: bool,
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
    /// Cached maximum heating air mass flow from begin-environment initialization.
    pub maximum_heating_air_mass_flow_rate_kg_per_s: f64,
    /// Cached maximum cooling air mass flow from begin-environment initialization.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    /// Standard air density used for the cached environment values.
    pub standard_air_density_kg_per_m3: Option<f64>,
    /// Total calls for this unit.
    pub init_call_count: usize,
    /// Completed per-unit topology passes.
    pub one_time_initialization_count: usize,
    /// Completed hard-size/sizing gates.
    pub sizing_check_count: usize,
    /// Completed begin-environment writes.
    pub environment_initialization_count: usize,
    /// False-begin-environment calls that rearmed the environment latch.
    pub environment_rearm_count: usize,
    /// Active cooling supply-temperature recurring diagnostic count.
    pub cooling_supply_temperature_warning_count: usize,
    /// Active heating supply-temperature recurring diagnostic count.
    pub heating_supply_temperature_warning_count: usize,
}

impl PurchasedAirUnitRuntimeState {
    pub(super) const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            one_time_initialized: false,
            sizing_needed: true,
            environment_initialization_needed: true,
            controlled_zone: None,
            equipment_list: None,
            supply_node: None,
            recirculation_node: None,
            maximum_heating_air_mass_flow_rate_kg_per_s: 0.0,
            maximum_cooling_air_mass_flow_rate_kg_per_s: 0.0,
            standard_air_density_kg_per_m3: None,
            init_call_count: 0,
            one_time_initialization_count: 0,
            sizing_check_count: 0,
            environment_initialization_count: 0,
            environment_rearm_count: 0,
            cooling_supply_temperature_warning_count: 0,
            heating_supply_temperature_warning_count: 0,
        }
    }

    /// Source-shaped flag snapshot after the latest call.
    #[must_use]
    pub fn flags(&self, equipment_list_checked: bool) -> IdealLoadsInitFlags {
        IdealLoadsInitFlags {
            state_machine_used: true,
            one_time_checked: self.one_time_initialized,
            environment_initialized: self.environment_initialization_count > 0,
            environment_initialization_needed: self.environment_initialization_needed,
            sizing_checked: !self.sizing_needed,
            equipment_list_checked,
            return_plenum_inactive: true,
        }
    }
}

/// Final lifecycle counters reported by the direct release runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirInitLifecycleSummary {
    /// Persistent state-machine provenance.
    pub source: &'static str,
    /// Final flag state.
    pub flags: IdealLoadsInitFlags,
    /// Module arena initialization count.
    pub module_initialization_count: usize,
    /// Global equipment-list check count.
    pub equipment_list_check_count: usize,
    /// Per-unit call count.
    pub init_call_count: usize,
    /// Per-unit one-time topology count.
    pub one_time_initialization_count: usize,
    /// Per-unit hard-size gate count.
    pub sizing_check_count: usize,
    /// Per-unit begin-environment initialization count.
    pub environment_initialization_count: usize,
    /// Per-unit environment rearm count.
    pub environment_rearm_count: usize,
    /// Cached maximum heating mass flow.
    pub maximum_heating_air_mass_flow_rate_kg_per_s: f64,
    /// Cached maximum cooling mass flow.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    /// Standard density owning the cached begin-environment values.
    pub standard_air_density_kg_per_m3: Option<f64>,
    /// Cooling recurring diagnostic count.
    pub cooling_supply_temperature_warning_count: usize,
    /// Heating recurring diagnostic count.
    pub heating_supply_temperature_warning_count: usize,
}
