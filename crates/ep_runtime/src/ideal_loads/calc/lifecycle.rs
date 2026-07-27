//! Bounded `CalcPurchAirLoads` entry-prefix lifecycle.

use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneId};

use crate::zone_equipment::{ZoneSensibleDemandInputKind, ZoneSysEnergyDemand};

use super::super::PurchasedAirRuntimeState;

/// EnergyPlus source slice executed by this bounded transition.
pub const PURCHASED_AIR_CALC_ENTRY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:1967,1971-2022";

/// Source-order entry steps retained by the bounded lifecycle.
pub const PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER: &[&str] = &[
    "resolve-supply-node",
    "resolve-zone-node",
    "resolve-outdoor-air-node",
    "resolve-recirculation-node",
    "reset-12-entry-values",
    "default-unit-on",
    "default-economizer-off",
    "read-heating-setpoint-demand",
    "read-cooling-setpoint-demand",
    "availability-manager-zone-write-if-allocated",
    "availability-manager-status-copy-if-allocated",
    "availability-manager-force-off-check-if-allocated",
    "read-overall-availability",
    "default-heating-on",
    "read-heating-availability",
    "default-cooling-on",
    "read-cooling-availability",
    "gate-unit-body",
];

/// Exact source assignment targets cleared before demand is read.
pub const PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS: &[&str] = &[
    "SupplyMassFlowRate",
    "OAMassFlowRate",
    "PurchAir.MinOAMassFlowRate",
    "PurchAir.TimeEconoActive",
    "PurchAir.TimeHtRecActive",
    "SysOutputProvided",
    "MoistOutputProvided",
    "CoolSensOutput",
    "CoolLatOutput",
    "CoolTotOutput",
    "HeatSensOutput",
    "LatOutput",
];

/// Source `Avail::Status` values visible to `CalcPurchAirLoads`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PurchasedAirAvailabilityStatus {
    /// Invalid sentinel retained for direct lifecycle characterization.
    Invalid,
    /// Availability manager takes no action.
    #[default]
    NoAction,
    /// Availability manager forces the unit off.
    ForceOff,
    /// Availability manager requests cycling on.
    CycleOn,
    /// Availability manager requests only Zone fans to cycle on.
    CycleOnZoneFansOnly,
}

/// Values explicitly reset before demand and availability reads.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PurchasedAirCalcEntryResetSnapshot {
    /// Local supply mass flow reset to zero.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Local outdoor-air mass flow reset to zero.
    pub outdoor_air_mass_flow_rate_kg_per_s: f64,
    /// Retained minimum outdoor-air mass flow reset to zero.
    pub minimum_outdoor_air_mass_flow_rate_kg_per_s: f64,
    /// Retained economizer active time reset to zero.
    pub economizer_active_time_hours: f64,
    /// Retained heat-recovery active time reset to zero.
    pub heat_recovery_active_time_hours: f64,
    /// Sensible output reference reset to zero.
    pub system_output_provided_w: f64,
    /// Moisture output reference reset to zero.
    pub moisture_output_provided_kg_per_s: f64,
    /// Local sensible cooling output reset to zero.
    pub cooling_sensible_output_w: f64,
    /// Local latent cooling output reset to zero.
    pub cooling_latent_output_w: f64,
    /// Local total cooling output reset to zero.
    pub cooling_total_output_w: f64,
    /// Local sensible heating output reset to zero.
    pub heating_sensible_output_w: f64,
    /// Local latent output reset to zero.
    pub latent_output_w: f64,
}

impl PurchasedAirCalcEntryResetSnapshot {
    /// Number of source assignments represented by this zero snapshot.
    pub const FIELD_COUNT: usize = PURCHASED_AIR_CALC_ENTRY_RESET_TARGETS.len();

    /// Returns whether every represented source assignment is zero.
    #[must_use]
    pub fn all_zero(self) -> bool {
        [
            self.supply_mass_flow_rate_kg_per_s,
            self.outdoor_air_mass_flow_rate_kg_per_s,
            self.minimum_outdoor_air_mass_flow_rate_kg_per_s,
            self.economizer_active_time_hours,
            self.heat_recovery_active_time_hours,
            self.system_output_provided_w,
            self.moisture_output_provided_kg_per_s,
            self.cooling_sensible_output_w,
            self.cooling_latent_output_w,
            self.cooling_total_output_w,
            self.heating_sensible_output_w,
            self.latent_output_w,
        ]
        .into_iter()
        .all(|value| value == 0.0)
    }
}

/// The two sensible setpoint-demand fields copied by this source prefix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcEntryDemandSnapshot {
    /// Zone owning the source demand entry.
    pub zone: ZoneId,
    /// Interpretation retained for downstream demand reconciliation.
    pub sensible_input_kind: ZoneSensibleDemandInputKind,
    /// Heating-setpoint remaining output copied by the prefix.
    pub remaining_output_req_to_heat_sp_w: f64,
    /// Cooling-setpoint remaining output copied by the prefix.
    pub remaining_output_req_to_cool_sp_w: f64,
}

impl From<ZoneSysEnergyDemand> for PurchasedAirCalcEntryDemandSnapshot {
    fn from(demand: ZoneSysEnergyDemand) -> Self {
        Self {
            zone: demand.zone,
            sensible_input_kind: demand.sensible_input_kind,
            remaining_output_req_to_heat_sp_w: demand.remaining_output_req_to_heat_sp_w,
            remaining_output_req_to_cool_sp_w: demand.remaining_output_req_to_cool_sp_w,
        }
    }
}

/// Inputs observed by the source entry prefix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcEntryContext {
    /// Controlled Zone selected by the caller.
    pub controlled_zone: ZoneId,
    /// PurchasedAir supply node.
    pub supply_node: NodeId,
    /// Controlled Zone air node.
    pub zone_node: NodeId,
    /// Optional outdoor-air node; absent in the release lane.
    pub outdoor_air_node: Option<NodeId>,
    /// Exhaust-or-return recirculation node.
    pub recirculation_node: NodeId,
    /// Current Zone system demand state.
    pub demand: ZoneSysEnergyDemand,
    /// `None` represents an unallocated Zone-component availability arena.
    pub zone_component_availability: Option<PurchasedAirAvailabilityStatus>,
    /// Current overall availability schedule value.
    pub overall_availability: f64,
    /// Current heating availability schedule value.
    pub heating_availability: f64,
    /// Current cooling availability schedule value.
    pub cooling_availability: f64,
}

/// One source-ordered entry-prefix result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcEntrySnapshot {
    /// EnergyPlus source slice represented by this snapshot.
    pub source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// One-based call ordinal for the selected system.
    pub call_ordinal: usize,
    /// Exact bounded source-order stage names.
    pub source_order: &'static [&'static str],
    /// Controlled Zone resolved at entry.
    pub controlled_zone: ZoneId,
    /// Supply node resolved at entry.
    pub supply_node: NodeId,
    /// Zone air node resolved at entry.
    pub zone_node: NodeId,
    /// Outdoor-air node resolved at entry.
    pub outdoor_air_node: Option<NodeId>,
    /// Recirculation node resolved at entry.
    pub recirculation_node: NodeId,
    /// Values cleared before demand and availability reads.
    pub reset: PurchasedAirCalcEntryResetSnapshot,
    /// Two sensible setpoint-demand values copied by the prefix.
    pub demand: PurchasedAirCalcEntryDemandSnapshot,
    /// Whether the source defaulted `UnitOn` to true.
    pub unit_defaulted_on: bool,
    /// Whether the source defaulted `EconoOn` to true.
    pub economizer_defaulted_on: bool,
    /// Whether an allocated Zone-component manager was visited.
    pub availability_manager_read_site_visited: bool,
    /// Whether the manager's controlled Zone was written.
    pub availability_manager_zone_written: bool,
    /// Availability status copied from the manager, when allocated.
    pub copied_availability_status: Option<PurchasedAirAvailabilityStatus>,
    /// Whether exact `ForceOff` cleared `UnitOn`.
    pub force_off_applied: bool,
    /// Whether the overall availability read site was visited.
    pub overall_availability_read_site_visited: bool,
    /// Whether the heating availability read site was visited.
    pub heating_availability_read_site_visited: bool,
    /// Whether the cooling availability read site was visited.
    pub cooling_availability_read_site_visited: bool,
    /// Sampled overall availability value.
    pub overall_availability: f64,
    /// Sampled heating availability value.
    pub heating_availability: f64,
    /// Sampled cooling availability value.
    pub cooling_availability: f64,
    /// Final `UnitOn` value at the line-2022 gate.
    pub unit_on: bool,
    /// Independent heating availability result.
    pub heating_on: bool,
    /// Independent cooling availability result.
    pub cooling_on: bool,
    /// Whether execution enters the active body at line 2022.
    pub unit_body_entered: bool,
}

/// Bounded per-unit state retained across entry-prefix calls.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcEntryRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// Completed prefix calls.
    pub call_count: usize,
    /// Completed 12-field reset groups.
    pub reset_count: usize,
    /// Completed two-field sensible demand reads.
    pub demand_read_count: usize,
    /// Allocated availability-manager visits.
    pub availability_manager_read_count: usize,
    /// Availability-manager Zone writes.
    pub availability_manager_zone_write_count: usize,
    /// Availability-status copies.
    pub availability_status_copy_count: usize,
    /// Overall availability read-site visits.
    pub overall_availability_read_count: usize,
    /// Heating availability read-site visits.
    pub heating_availability_read_count: usize,
    /// Cooling availability read-site visits.
    pub cooling_availability_read_count: usize,
    /// Exact `ForceOff` applications.
    pub force_off_count: usize,
    /// Overall schedule values that turned the unit off.
    pub overall_schedule_off_count: usize,
    /// Calls entering the active body.
    pub unit_body_entry_count: usize,
    /// Calls stopped at the active-body gate.
    pub unit_off_count: usize,
    /// Calls with heating independently available.
    pub heating_on_count: usize,
    /// Calls with cooling independently available.
    pub cooling_on_count: usize,
    /// Last manager Zone write, when any.
    pub availability_manager_zone: Option<ZoneId>,
    /// Last copied manager status, retained when the manager is absent.
    pub availability_status: PurchasedAirAvailabilityStatus,
    /// Retained source minimum outdoor-air flow reset each call.
    pub minimum_outdoor_air_mass_flow_rate_kg_per_s: f64,
    /// Retained source economizer time reset each call.
    pub economizer_active_time_hours: f64,
    /// Retained source heat-recovery time reset each call.
    pub heat_recovery_active_time_hours: f64,
    /// Latest bounded snapshot; no per-timestep log is retained.
    pub latest: Option<PurchasedAirCalcEntrySnapshot>,
}

impl PurchasedAirCalcEntryRuntimeState {
    /// Creates bounded state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            call_count: 0,
            reset_count: 0,
            demand_read_count: 0,
            availability_manager_read_count: 0,
            availability_manager_zone_write_count: 0,
            availability_status_copy_count: 0,
            overall_availability_read_count: 0,
            heating_availability_read_count: 0,
            cooling_availability_read_count: 0,
            force_off_count: 0,
            overall_schedule_off_count: 0,
            unit_body_entry_count: 0,
            unit_off_count: 0,
            heating_on_count: 0,
            cooling_on_count: 0,
            availability_manager_zone: None,
            availability_status: PurchasedAirAvailabilityStatus::NoAction,
            minimum_outdoor_air_mass_flow_rate_kg_per_s: 0.0,
            economizer_active_time_hours: 0.0,
            heat_recovery_active_time_hours: 0.0,
            latest: None,
        }
    }
}

/// Final selected-unit entry-prefix summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcEntryLifecycleSummary {
    /// EnergyPlus source slice represented by the summary.
    pub source: &'static str,
    /// Final bounded state for the selected system.
    pub state: PurchasedAirCalcEntryRuntimeState,
}

/// Bounded lookup failure before the source prefix can be entered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcEntryError {
    /// The selected unit is absent from the persistent arena.
    UnknownSystem {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The selected unit has not completed its bounded topology pass.
    InitializationNotReady {
        /// Unready typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Init and Calc-entry calls are not in one-for-one source order.
    InitializationCallOrder {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Completed initialization calls.
        init_call_count: usize,
        /// Completed Calc-entry calls.
        calc_call_count: usize,
    },
    /// Runtime-owned topology or demand identity disagrees with the caller.
    IdentityMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Mismatched relation.
        relation: PurchasedAirCalcEntryIdentityRelation,
    },
}

/// Runtime-owned identities that must agree before the source prefix is entered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcEntryIdentityRelation {
    /// Controlled Zone identity.
    ControlledZone,
    /// PurchasedAir supply-node identity.
    SupplyNode,
    /// PurchasedAir recirculation-node identity.
    RecirculationNode,
    /// Demand Zone versus controlled Zone identity.
    DemandZone,
}

/// Executes the entry prefix for one unit retained by the PurchasedAir arena.
pub fn advance_purchased_air_calc_entry(
    runtime: &mut PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
    context: PurchasedAirCalcEntryContext,
) -> Result<PurchasedAirCalcEntrySnapshot, PurchasedAirCalcEntryError> {
    let unit = runtime
        .units
        .get_mut(&system)
        .ok_or(PurchasedAirCalcEntryError::UnknownSystem { system })?;
    if !unit.topology_completed || unit.topology_failure.is_some() {
        return Err(PurchasedAirCalcEntryError::InitializationNotReady { system });
    }
    if unit.calc_entry.call_count.checked_add(1) != Some(unit.init_call_count) {
        return Err(PurchasedAirCalcEntryError::InitializationCallOrder {
            system,
            init_call_count: unit.init_call_count,
            calc_call_count: unit.calc_entry.call_count,
        });
    }
    for (matches, relation) in [
        (
            unit.controlled_zone == Some(context.controlled_zone),
            PurchasedAirCalcEntryIdentityRelation::ControlledZone,
        ),
        (
            unit.supply_node == Some(context.supply_node),
            PurchasedAirCalcEntryIdentityRelation::SupplyNode,
        ),
        (
            unit.recirculation_node == Some(context.recirculation_node),
            PurchasedAirCalcEntryIdentityRelation::RecirculationNode,
        ),
        (
            context.demand.zone == context.controlled_zone,
            PurchasedAirCalcEntryIdentityRelation::DemandZone,
        ),
    ] {
        if !matches {
            return Err(PurchasedAirCalcEntryError::IdentityMismatch { system, relation });
        }
    }
    Ok(advance_entry_state(&mut unit.calc_entry, context))
}

/// Returns the bounded selected-unit entry-prefix summary.
pub fn purchased_air_calc_entry_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcEntryLifecycleSummary, PurchasedAirCalcEntryError> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcEntryError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcEntryLifecycleSummary {
        source: PURCHASED_AIR_CALC_ENTRY_SOURCE,
        state: unit.calc_entry.clone(),
    })
}

pub(super) fn advance_entry_state(
    state: &mut PurchasedAirCalcEntryRuntimeState,
    context: PurchasedAirCalcEntryContext,
) -> PurchasedAirCalcEntrySnapshot {
    state.call_count += 1;
    state.reset_count += 1;
    state.demand_read_count += 1;
    state.minimum_outdoor_air_mass_flow_rate_kg_per_s = 0.0;
    state.economizer_active_time_hours = 0.0;
    state.heat_recovery_active_time_hours = 0.0;

    let manager_visited = context.zone_component_availability.is_some();
    let force_off_applied = if let Some(status) = context.zone_component_availability {
        state.availability_manager_read_count += 1;
        state.availability_manager_zone_write_count += 1;
        state.availability_status_copy_count += 1;
        state.availability_manager_zone = Some(context.controlled_zone);
        state.availability_status = status;
        status == PurchasedAirAvailabilityStatus::ForceOff
    } else {
        false
    };
    if force_off_applied {
        state.force_off_count += 1;
    }

    state.overall_availability_read_count += 1;
    state.heating_availability_read_count += 1;
    state.cooling_availability_read_count += 1;
    let overall_on = nominally_on(context.overall_availability);
    let heating_on = nominally_on(context.heating_availability);
    let cooling_on = nominally_on(context.cooling_availability);
    if !overall_on {
        state.overall_schedule_off_count += 1;
    }
    if heating_on {
        state.heating_on_count += 1;
    }
    if cooling_on {
        state.cooling_on_count += 1;
    }
    let unit_on = !force_off_applied && overall_on;
    if unit_on {
        state.unit_body_entry_count += 1;
    } else {
        state.unit_off_count += 1;
    }

    let snapshot = PurchasedAirCalcEntrySnapshot {
        source: PURCHASED_AIR_CALC_ENTRY_SOURCE,
        system: state.system,
        call_ordinal: state.call_count,
        source_order: PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER,
        controlled_zone: context.controlled_zone,
        supply_node: context.supply_node,
        zone_node: context.zone_node,
        outdoor_air_node: context.outdoor_air_node,
        recirculation_node: context.recirculation_node,
        reset: PurchasedAirCalcEntryResetSnapshot::default(),
        demand: context.demand.into(),
        unit_defaulted_on: true,
        economizer_defaulted_on: false,
        availability_manager_read_site_visited: manager_visited,
        availability_manager_zone_written: manager_visited,
        copied_availability_status: context.zone_component_availability,
        force_off_applied,
        overall_availability_read_site_visited: true,
        heating_availability_read_site_visited: true,
        cooling_availability_read_site_visited: true,
        overall_availability: context.overall_availability,
        heating_availability: context.heating_availability,
        cooling_availability: context.cooling_availability,
        unit_on,
        heating_on,
        cooling_on,
        unit_body_entered: unit_on,
    };
    state.latest = Some(snapshot);
    snapshot
}

fn nominally_on(value: f64) -> bool {
    value > 0.0 || value.is_nan()
}
