//! Runtime meter registry and request resolution.

use super::{
    MeterIdentity, RuntimeDiagnostic, RuntimeDiagnosticCode, RuntimeDiagnosticSeverity,
    RuntimeDiagnosticStore, RuntimeMeterDefinition, RuntimeMeterRequest, RuntimeMeterResolution,
    RuntimeOutputFrequency, RuntimeOutputSource, RuntimeResolvedMeter,
};
use ep_model::OutputHandle;
use std::collections::BTreeSet;

/// EnergyPlus-style facility electricity meter name.
pub const ELECTRICITY_FACILITY_METER: &str = "Electricity:Facility";
/// EnergyPlus-style facility gas meter name.
pub const GAS_FACILITY_METER: &str = "Gas:Facility";
/// EnergyPlus-style heating energy-transfer meter name.
pub const HEATING_ENERGY_TRANSFER_METER: &str = "Heating:EnergyTransfer";
/// EnergyPlus-style cooling energy-transfer meter name.
pub const COOLING_ENERGY_TRANSFER_METER: &str = "Cooling:EnergyTransfer";
/// Near-zero tolerance used when comparing meter energy values in J.
pub const METER_ZERO_NEAR_TOLERANCE_J: f64 = 1.0e-9;
/// EnergyPlus-compatible rate-to-energy rule for meter inputs.
pub const METER_RATE_TO_ENERGY_RULE: &str = "rate_w * reporting_interval_seconds -> J";
/// Source map label for component output to facility meter aggregation.
pub const COMPONENT_OUTPUT_TO_FACILITY_METER_SOURCE_MAP: &str =
    "component output series -> RuntimeMeterRegistry facility meter aggregation";

/// Runtime meter aggregation kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeMeterAggregationKind {
    /// `Electricity:Facility`.
    FacilityElectricity,
    /// `Gas:Facility`.
    FacilityGas,
    /// `Heating:EnergyTransfer` or the current district-heating facility proxy.
    HeatingEnergyTransfer,
    /// `Cooling:EnergyTransfer` or the current district-cooling facility proxy.
    CoolingEnergyTransfer,
    /// Registered meter without a supported aggregation path.
    Unsupported,
}

impl RuntimeMeterAggregationKind {
    /// Infers the aggregation kind from an EnergyPlus-style meter name.
    #[must_use]
    pub fn from_meter_name(name: &str) -> Self {
        match normalize_meter_name(name).as_str() {
            "electricity:facility" => Self::FacilityElectricity,
            "gas:facility" => Self::FacilityGas,
            "heating:energytransfer" | "districtheatingwater:facility" => {
                Self::HeatingEnergyTransfer
            }
            "cooling:energytransfer" | "districtcooling:facility" => Self::CoolingEnergyTransfer,
            _ => Self::Unsupported,
        }
    }
}

/// Meter aggregation reporting period.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeMeterAggregationPeriod {
    /// Hourly meter aggregation.
    Hourly,
    /// Monthly meter aggregation.
    Monthly,
    /// Annual meter aggregation.
    Annual,
    /// Run-period meter aggregation.
    RunPeriod,
    /// Other frequencies are not meter aggregation targets yet.
    Unsupported,
}

impl RuntimeMeterAggregationPeriod {
    /// Maps a runtime output frequency to a meter aggregation period.
    #[must_use]
    pub const fn from_frequency(frequency: RuntimeOutputFrequency) -> Self {
        match frequency {
            RuntimeOutputFrequency::Hourly => Self::Hourly,
            RuntimeOutputFrequency::Monthly => Self::Monthly,
            RuntimeOutputFrequency::Annual => Self::Annual,
            RuntimeOutputFrequency::RunPeriod => Self::RunPeriod,
            RuntimeOutputFrequency::Static
            | RuntimeOutputFrequency::Timestep
            | RuntimeOutputFrequency::Daily => Self::Unsupported,
        }
    }
}

/// Source map from component output series into a facility meter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeMeterSourceMap {
    /// Component output variable feeding the meter.
    pub component_output_variable: String,
    /// Facility meter being aggregated.
    pub facility_meter_name: String,
    /// Aggregation kind.
    pub aggregation_kind: RuntimeMeterAggregationKind,
    /// Source map label.
    pub source_map: &'static str,
}

/// Precompiled meter aggregation plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeMeterAggregationPlan {
    /// Aggregation kind.
    pub kind: RuntimeMeterAggregationKind,
    /// Aggregation reporting period.
    pub period: RuntimeMeterAggregationPeriod,
    /// Pre-resolved output handles that feed this meter.
    pub dependency_output_handles: Vec<OutputHandle>,
    /// EnergyPlus-compatible rate-to-energy rule.
    pub rate_to_energy_rule: &'static str,
    /// Near-zero meter tolerance in J.
    pub zero_near_tolerance_j: &'static str,
    /// Component-to-meter source map label.
    pub component_output_source_map: &'static str,
}

impl RuntimeMeterAggregationPlan {
    /// Creates an aggregation plan for a registered meter.
    #[must_use]
    pub fn new(
        meter_name: &str,
        frequency: RuntimeOutputFrequency,
        dependency_output_handles: Vec<OutputHandle>,
    ) -> Self {
        Self {
            kind: RuntimeMeterAggregationKind::from_meter_name(meter_name),
            period: RuntimeMeterAggregationPeriod::from_frequency(frequency),
            dependency_output_handles,
            rate_to_energy_rule: METER_RATE_TO_ENERGY_RULE,
            zero_near_tolerance_j: "METER_ZERO_NEAR_TOLERANCE_J",
            component_output_source_map: COMPONENT_OUTPUT_TO_FACILITY_METER_SOURCE_MAP,
        }
    }
}

/// Converts a rate in W to EnergyPlus meter energy in J.
#[must_use]
pub fn meter_rate_to_energy_j(rate_w: f64, reporting_interval_seconds: f64) -> f64 {
    rate_w * reporting_interval_seconds
}

/// Returns true when a meter value is close enough to zero to be treated as zero.
#[must_use]
pub fn meter_value_is_zero_near_j(value_j: f64) -> bool {
    value_j.abs() <= METER_ZERO_NEAR_TOLERANCE_J
}

/// Creates a component-output to facility-meter source-map row.
#[must_use]
pub fn component_output_to_facility_meter_source_map(
    component_output_variable: impl Into<String>,
    facility_meter_name: impl Into<String>,
) -> RuntimeMeterSourceMap {
    let facility_meter_name = facility_meter_name.into();
    RuntimeMeterSourceMap {
        component_output_variable: component_output_variable.into(),
        aggregation_kind: RuntimeMeterAggregationKind::from_meter_name(&facility_meter_name),
        facility_meter_name,
        source_map: COMPONENT_OUTPUT_TO_FACILITY_METER_SOURCE_MAP,
    }
}

fn normalize_meter_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

/// Runtime meter registry.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RuntimeMeterRegistry {
    meters: Vec<RuntimeMeterDefinition>,
}

impl RuntimeMeterRegistry {
    /// Creates an empty meter registry.
    #[must_use]
    pub fn new() -> Self {
        Self { meters: Vec::new() }
    }

    /// Returns meter definitions in handle order.
    #[must_use]
    pub fn meters(&self) -> &[RuntimeMeterDefinition] {
        &self.meters
    }

    /// Returns the number of registered meters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.meters.len()
    }

    /// Returns true when the registry contains no meters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.meters.is_empty()
    }

    pub(super) fn push_meter_with_dependencies(
        &mut self,
        name: &str,
        units: &str,
        frequency: RuntimeOutputFrequency,
        source: RuntimeOutputSource,
        dependency_output_handles: Vec<OutputHandle>,
    ) {
        let identity = MeterIdentity::new(name, frequency);
        if self
            .meters
            .iter()
            .any(|definition| definition.identity() == identity)
        {
            return;
        }

        self.meters.push(RuntimeMeterDefinition {
            handle: OutputHandle(self.meters.len() as u32),
            name: name.to_string(),
            units: units.to_string(),
            frequency,
            source,
            dependency_output_handles: dependency_output_handles.clone(),
            aggregation_plan: RuntimeMeterAggregationPlan::new(
                name,
                frequency,
                dependency_output_handles,
            ),
        });
    }

    /// Resolves meter requests. v0.24 intentionally records unsupported meters
    /// as diagnostics rather than silently creating empty series.
    #[must_use]
    pub fn resolve_meter_requests(
        &self,
        requests: &[RuntimeMeterRequest],
    ) -> RuntimeMeterResolution {
        let mut seen = BTreeSet::new();
        let mut resolved = Vec::new();
        let mut diagnostics = RuntimeDiagnosticStore::new();

        for request in requests {
            let identity = request.identity();
            if !seen.insert(identity) {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::DuplicateMeterRequest,
                    message: format!(
                        "duplicate runtime meter request {} ({})",
                        request.name,
                        request.frequency.id()
                    ),
                    stage: Some("meter-resolution".to_string()),
                    surface: None,
                    zone: None,
                    timestep: None,
                    key: None,
                    variable_name: None,
                    meter_name: Some(request.name.clone()),
                    handle: None,
                });
                continue;
            }

            if let Some(definition) = self
                .meters
                .iter()
                .find(|definition| definition.identity() == request.identity())
            {
                resolved.push(RuntimeResolvedMeter {
                    request: request.clone(),
                    definition: definition.clone(),
                });
            } else {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::MeterUnavailable,
                    message: format!(
                        "runtime meter unavailable: {} ({})",
                        request.name,
                        request.frequency.id()
                    ),
                    stage: Some("meter-resolution".to_string()),
                    surface: None,
                    zone: None,
                    timestep: None,
                    key: None,
                    variable_name: None,
                    meter_name: Some(request.name.clone()),
                    handle: None,
                });
            }
        }

        RuntimeMeterResolution {
            resolved,
            diagnostics,
        }
    }
}
