//! Structured evidence for the `InitPurchasedAir` supply-temperature diagnostics.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit};

use super::state::PurchasedAirUnitRuntimeState;

/// Source branch that produced a supply-temperature diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirSupplyTemperatureDiagnosticKind {
    /// Minimum cooling supply temperature exceeds the active cooling setpoint.
    CoolingMinimumAboveSetpoint,
    /// Maximum heating supply temperature is below the active heating setpoint.
    HeatingMaximumBelowSetpoint,
}

/// Source API used by the first detailed diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirSupplyTemperatureInitialMessageApi {
    /// Cooling calls `ShowSevereError`, which increments the severe counter.
    ShowSevereError,
    /// Heating calls `ShowSevereMessage`, which does not increment the counter.
    ShowSevereMessage,
}

/// One source-shaped outer-gate and availability-read trace.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PurchasedAirSupplyTemperatureGateTrace {
    /// Temperature, nonzero-setpoint, and no-limit conditions entered the branch.
    pub outer_condition_met: bool,
    /// The source overall-availability schedule-read site was reached.
    ///
    /// Rust receives an already sampled value, so this characterizes source
    /// control flow rather than claiming a schedule service call.
    pub overall_availability_read_site_visited: bool,
    /// The source mode-availability schedule-read site was reached.
    ///
    /// The source reaches this site even when overall availability is off.
    pub mode_availability_read_site_visited: bool,
    /// Both supplied availability values were nominally on.
    pub active: bool,
}

/// One registered recurring identity and its accumulated call evidence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirSupplyTemperatureDiagnostic {
    /// Typed IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// One-based allocation order across the bounded Rust recurring registry.
    pub registry_registration_ordinal: usize,
    /// One-based `InitPurchasedAir` call that first activated this identity.
    pub first_init_call_ordinal: usize,
    /// One-based `InitPurchasedAir` call that most recently activated it.
    pub last_init_call_ordinal: usize,
    /// Source position within one call: cooling is one and heating is two.
    pub source_order_ordinal: usize,
    /// Cooling or heating diagnostic branch.
    pub kind: PurchasedAirSupplyTemperatureDiagnosticKind,
    /// Positive recurring index allocated on first activation and then reused.
    pub recurring_index: usize,
    /// First detailed message groups represented by this identity.
    pub first_detailed_diagnostic_count: usize,
    /// API used by the first detailed message group.
    pub initial_message_api: PurchasedAirSupplyTemperatureInitialMessageApi,
    /// Primary first-detail message count.
    pub first_detail_primary_message_count: usize,
    /// Continue-message count in the first-detail group.
    pub first_detail_continue_message_count: usize,
    /// Timestamp-message count in the first-detail group.
    pub first_detail_timestamp_count: usize,
    /// Recurring-severe calls accumulated for this identity.
    pub recurring_severe_call_count: usize,
    /// Accumulated ordinary non-warmup/non-sizing severe-counter increment.
    pub characterized_severe_error_count_increment: usize,
    /// Most recently reported supply temperature.
    pub latest_supply_temperature_c: f64,
    /// Most recently compared thermostat setpoint.
    pub latest_thermostat_setpoint_c: f64,
    /// Minimum recurring value accumulated across active calls.
    pub recurring_minimum_c: f64,
    /// Maximum recurring value accumulated across active calls.
    pub recurring_maximum_c: f64,
    /// Source recurring-report unit.
    pub temperature_unit: &'static str,
}

/// Rust-owned bounded recurring registry for deterministic lifecycle evidence.
///
/// Numeric IDs prove relative allocation and reuse only. They are not claimed
/// to equal indices in EnergyPlus's process-wide error registry.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PurchasedAirSupplyTemperatureDiagnosticRegistry {
    /// Number of distinct cooling/heating recurring identities allocated.
    pub registered_recurring_diagnostic_count: usize,
    /// Number of active recurring events recorded across all units.
    pub event_count: usize,
    /// Characterized ordinary severe-counter increment across all events.
    pub characterized_severe_error_count_increment: usize,
    /// Registered identities in one-based allocation order across all units.
    pub diagnostics: Vec<PurchasedAirSupplyTemperatureDiagnostic>,
}

/// Caller-supplied dynamic values used by the bounded diagnostic suffix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct PurchasedAirSupplyTemperatureDiagnosticContext {
    pub cooling_setpoint_c: f64,
    pub heating_setpoint_c: f64,
    pub overall_availability: f64,
    pub cooling_availability: f64,
    pub heating_availability: f64,
}

/// Diagnostic changes produced by one source-ordered suffix evaluation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct PurchasedAirSupplyTemperatureDiagnosticTransition {
    pub cooling_gate: PurchasedAirSupplyTemperatureGateTrace,
    pub heating_gate: PurchasedAirSupplyTemperatureGateTrace,
    pub cooling_active: bool,
    pub heating_active: bool,
    pub cooling_first_diagnostic: bool,
    pub heating_first_diagnostic: bool,
    pub diagnostics_emitted: usize,
    pub characterized_severe_error_count_increment: usize,
}

impl PurchasedAirSupplyTemperatureDiagnosticRegistry {
    pub(super) fn allocate_recurring_index(&mut self) -> usize {
        self.registered_recurring_diagnostic_count += 1;
        self.registered_recurring_diagnostic_count
    }

    pub(super) fn record_event(&mut self, severe_error_count_increment: usize) -> usize {
        self.event_count += 1;
        self.characterized_severe_error_count_increment += severe_error_count_increment;
        self.event_count
    }
}

pub(super) fn advance_supply_temperature_diagnostics(
    registry: &mut PurchasedAirSupplyTemperatureDiagnosticRegistry,
    unit: &mut PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    context: PurchasedAirSupplyTemperatureDiagnosticContext,
) -> PurchasedAirSupplyTemperatureDiagnosticTransition {
    let mut transition = PurchasedAirSupplyTemperatureDiagnosticTransition::default();

    let cooling_outer_condition = system.minimum_cooling_supply_air_temperature_c
        > context.cooling_setpoint_c
        && context.cooling_setpoint_c != 0.0
        && system.cooling_limit == IdealLoadsLimit::NoLimit;
    transition.cooling_gate = gate_trace(
        cooling_outer_condition,
        context.overall_availability,
        context.cooling_availability,
    );
    transition.cooling_active = transition.cooling_gate.active;
    if transition.cooling_active {
        let first_detailed_diagnostic = unit.cooling_supply_temperature_error_index == 0;
        if first_detailed_diagnostic {
            unit.cooling_supply_temperature_error_index = registry.allocate_recurring_index();
            unit.cooling_supply_temperature_first_diagnostic_count += 1;
            transition.cooling_first_diagnostic = true;
        }
        record_supply_temperature_diagnostic(
            registry,
            unit,
            PurchasedAirSupplyTemperatureDiagnosticKind::CoolingMinimumAboveSetpoint,
            1,
            first_detailed_diagnostic,
            PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereError,
            system.minimum_cooling_supply_air_temperature_c,
            context.cooling_setpoint_c,
            unit.cooling_supply_temperature_error_index,
            &mut transition.characterized_severe_error_count_increment,
        );
        unit.cooling_supply_temperature_warning_count += 1;
        transition.diagnostics_emitted += 1;
    }

    let heating_outer_condition = system.maximum_heating_supply_air_temperature_c
        < context.heating_setpoint_c
        && context.heating_setpoint_c != 0.0
        && system.heating_limit == IdealLoadsLimit::NoLimit;
    transition.heating_gate = gate_trace(
        heating_outer_condition,
        context.overall_availability,
        context.heating_availability,
    );
    transition.heating_active = transition.heating_gate.active;
    if transition.heating_active {
        let first_detailed_diagnostic = unit.heating_supply_temperature_error_index == 0;
        if first_detailed_diagnostic {
            unit.heating_supply_temperature_error_index = registry.allocate_recurring_index();
            unit.heating_supply_temperature_first_diagnostic_count += 1;
            transition.heating_first_diagnostic = true;
        }
        record_supply_temperature_diagnostic(
            registry,
            unit,
            PurchasedAirSupplyTemperatureDiagnosticKind::HeatingMaximumBelowSetpoint,
            2,
            first_detailed_diagnostic,
            PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereMessage,
            system.maximum_heating_supply_air_temperature_c,
            context.heating_setpoint_c,
            unit.heating_supply_temperature_error_index,
            &mut transition.characterized_severe_error_count_increment,
        );
        unit.heating_supply_temperature_warning_count += 1;
        transition.diagnostics_emitted += 1;
    }

    transition
}

fn gate_trace(
    outer_condition_met: bool,
    overall_availability: f64,
    mode_availability: f64,
) -> PurchasedAirSupplyTemperatureGateTrace {
    if !outer_condition_met {
        return PurchasedAirSupplyTemperatureGateTrace::default();
    }
    PurchasedAirSupplyTemperatureGateTrace {
        outer_condition_met: true,
        overall_availability_read_site_visited: true,
        mode_availability_read_site_visited: true,
        active: nominally_on(overall_availability) && nominally_on(mode_availability),
    }
}

fn nominally_on(value: f64) -> bool {
    value > 0.0 || value.is_nan()
}

#[allow(clippy::too_many_arguments)]
fn record_supply_temperature_diagnostic(
    registry: &mut PurchasedAirSupplyTemperatureDiagnosticRegistry,
    unit: &mut PurchasedAirUnitRuntimeState,
    kind: PurchasedAirSupplyTemperatureDiagnosticKind,
    source_order_ordinal: usize,
    first_detailed_diagnostic: bool,
    initial_message_api: PurchasedAirSupplyTemperatureInitialMessageApi,
    supply_temperature_c: f64,
    thermostat_setpoint_c: f64,
    recurring_index: usize,
    transition_severe_error_count_increment: &mut usize,
) {
    let characterized_severe_error_count_increment = 1 + usize::from(
        first_detailed_diagnostic
            && initial_message_api
                == PurchasedAirSupplyTemperatureInitialMessageApi::ShowSevereError,
    );
    if first_detailed_diagnostic {
        debug_assert_eq!(registry.diagnostics.len() + 1, recurring_index);
        registry
            .diagnostics
            .push(PurchasedAirSupplyTemperatureDiagnostic {
                system: unit.system,
                registry_registration_ordinal: recurring_index,
                first_init_call_ordinal: unit.init_call_count,
                last_init_call_ordinal: unit.init_call_count,
                source_order_ordinal,
                kind,
                recurring_index,
                first_detailed_diagnostic_count: 1,
                initial_message_api,
                first_detail_primary_message_count: 1,
                first_detail_continue_message_count: 5,
                first_detail_timestamp_count: 1,
                recurring_severe_call_count: 1,
                characterized_severe_error_count_increment,
                latest_supply_temperature_c: supply_temperature_c,
                latest_thermostat_setpoint_c: thermostat_setpoint_c,
                recurring_minimum_c: supply_temperature_c,
                recurring_maximum_c: supply_temperature_c,
                temperature_unit: PURCHASED_AIR_SUPPLY_TEMPERATURE_UNIT_C,
            });
    } else {
        let Some(diagnostic_index) = recurring_index.checked_sub(1) else {
            debug_assert_ne!(
                recurring_index, 0,
                "recurring identity index must be positive"
            );
            return;
        };
        let diagnostic_count = registry.diagnostics.len();
        let Some(diagnostic) = registry.diagnostics.get_mut(diagnostic_index) else {
            debug_assert!(
                recurring_index <= diagnostic_count,
                "allocated recurring index must identify a registry record"
            );
            return;
        };
        debug_assert_eq!(diagnostic.system, unit.system);
        debug_assert_eq!(diagnostic.kind, kind);
        diagnostic.last_init_call_ordinal = unit.init_call_count;
        diagnostic.recurring_severe_call_count += 1;
        diagnostic.characterized_severe_error_count_increment +=
            characterized_severe_error_count_increment;
        diagnostic.latest_supply_temperature_c = supply_temperature_c;
        diagnostic.latest_thermostat_setpoint_c = thermostat_setpoint_c;
        diagnostic.recurring_minimum_c = diagnostic.recurring_minimum_c.min(supply_temperature_c);
        diagnostic.recurring_maximum_c = diagnostic.recurring_maximum_c.max(supply_temperature_c);
    }
    registry.record_event(characterized_severe_error_count_increment);
    *transition_severe_error_count_increment += characterized_severe_error_count_increment;
}

/// Unit emitted by both recurring diagnostic calls.
pub const PURCHASED_AIR_SUPPLY_TEMPERATURE_UNIT_C: &str = "C";
