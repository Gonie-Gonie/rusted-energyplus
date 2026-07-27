//! Final evidence snapshot for the bounded persistent initialization lifecycle.

use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneEquipmentListId, ZoneId};

use super::{
    IdealLoadsInitFlags, PurchasedAirInitDiagnostic, PurchasedAirInitTopologyDiagnostic,
    PurchasedAirInitTopologyError, PurchasedAirRecirculationSource,
};

/// Final lifecycle counters reported by the direct release runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirInitLifecycleSummary {
    /// Persistent state-machine provenance.
    pub source: &'static str,
    /// Final flag state.
    pub flags: IdealLoadsInitFlags,
    /// Module arena initialization count.
    pub module_initialization_count: usize,
    /// Global equipment-list check count.
    pub equipment_list_check_count: usize,
    /// IdealLoads systems in immutable typed declaration order.
    pub declared_system_order: Vec<IdealLoadsAirSystemId>,
    /// Systems visited by the manager-wide equipment-list sweep.
    pub equipment_list_scan_order: Vec<IdealLoadsAirSystemId>,
    /// Total units visited by the manager-wide equipment-list sweep.
    pub equipment_list_scanned_unit_count: usize,
    /// Units missing from every equipment list.
    pub equipment_list_missing_unit_count: usize,
    /// Ordered diagnostics retained from the sweep.
    pub equipment_list_diagnostics: Vec<PurchasedAirInitDiagnostic>,
    /// Selected unit's one-based manager sweep ordinal.
    pub equipment_list_scan_ordinal: Option<usize>,
    /// First controlled-Zone-referenced equipment list containing the selected unit.
    pub first_matching_equipment_list: Option<ZoneEquipmentListId>,
    /// Whether the selected unit was found in an equipment list.
    pub equipment_list_membership_found: Option<bool>,
    /// Selected controlled Zone retained by the one-time pass.
    pub controlled_zone: Option<ZoneId>,
    /// Selected equipment list retained by the Rust topology plan.
    pub equipment_list: Option<ZoneEquipmentListId>,
    /// Supply node retained by the one-time pass.
    pub supply_node: Option<NodeId>,
    /// Recirculation node assigned by the source branch, when any.
    pub recirculation_node: Option<NodeId>,
    /// Source branch that selected or left recirculation unassigned.
    pub recirculation_source: Option<PurchasedAirRecirculationSource>,
    /// Configured exhaust rejected before return fallback.
    pub rejected_exhaust_node: Option<NodeId>,
    /// First return node named by the multiple-return warning.
    pub reported_first_return_node: Option<NodeId>,
    /// Ordered one-time topology diagnostics.
    pub topology_diagnostics: Vec<PurchasedAirInitTopologyDiagnostic>,
    /// Fatal topology state retained after the source latch.
    pub topology_failure: Option<PurchasedAirInitTopologyError>,
    /// Per-unit call count.
    pub init_call_count: usize,
    /// Source one-time latch count.
    pub one_time_initialization_count: usize,
    /// One-time topology completion count.
    pub topology_completion_count: usize,
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
    /// One-time OA/economizer flow-limit advisory count.
    pub economizer_flow_limit_warning_count: usize,
}
