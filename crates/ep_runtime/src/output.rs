//! Runtime output, meter, diagnostic, and result-store primitives.

mod diagnostics;
mod meter_registry;
mod result_store;

pub use diagnostics::*;
use ep_model::{
    BranchListId, NormalizedName, OutputHandle, OutsideBoundaryCondition, PlantBranchComponent,
    ScheduleId, SimulationModel, SunExposure, TypedModel,
};
pub use meter_registry::*;
pub use result_store::*;
use std::collections::BTreeSet;

use crate::ideal_loads::{
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE, ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
    ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE, ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
    ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
    ideal_loads_facility_meter_binding,
};

/// Runtime-native output reporting frequency.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RuntimeOutputFrequency {
    /// Static input/report rows with no timestep axis.
    Static,
    /// Every simulation timestep.
    Timestep,
    /// Hourly reporting.
    Hourly,
    /// Daily reporting.
    Daily,
    /// Monthly reporting.
    Monthly,
    /// Annual reporting.
    Annual,
    /// Run-period reporting.
    RunPeriod,
}

impl RuntimeOutputFrequency {
    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Timestep => "timestep",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::Annual => "annual",
            Self::RunPeriod => "run-period",
        }
    }
}

/// Runtime-native source for a registered output.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum RuntimeOutputSource {
    /// Value is produced by Rust runtime state.
    RuntimeState,
    /// Value is read from weather input and projected through runtime helpers.
    WeatherInput,
    /// Value is produced by schedule evaluation.
    Schedule,
    /// Value is not implemented yet but has a declared meter registry entry.
    Meter,
}

/// One runtime output request resolved before execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutputRequest {
    /// EnergyPlus output key.
    pub key: String,
    /// EnergyPlus output variable name.
    pub variable_name: String,
    /// Requested frequency.
    pub frequency: RuntimeOutputFrequency,
}

impl RuntimeOutputRequest {
    /// Creates an hourly output request.
    #[must_use]
    pub fn hourly(key: impl Into<String>, variable_name: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            variable_name: variable_name.into(),
            frequency: RuntimeOutputFrequency::Hourly,
        }
    }

    fn identity(&self) -> OutputIdentity {
        OutputIdentity::new(&self.key, &self.variable_name, self.frequency)
    }
}

/// One runtime meter request resolved before execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeMeterRequest {
    /// EnergyPlus meter name.
    pub name: String,
    /// Requested frequency.
    pub frequency: RuntimeOutputFrequency,
}

impl RuntimeMeterRequest {
    /// Creates a meter request for a specific reporting frequency.
    #[must_use]
    pub fn new(name: impl Into<String>, frequency: RuntimeOutputFrequency) -> Self {
        Self {
            name: name.into(),
            frequency,
        }
    }

    /// Creates an hourly meter request.
    #[must_use]
    pub fn hourly(name: impl Into<String>) -> Self {
        Self::new(name, RuntimeOutputFrequency::Hourly)
    }

    fn identity(&self) -> MeterIdentity {
        MeterIdentity::new(&self.name, self.frequency)
    }
}

/// One output variable the runtime knows how to produce.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutputDefinition {
    /// Stable output handle for the current model and registry.
    pub handle: OutputHandle,
    /// EnergyPlus output key.
    pub key: String,
    /// EnergyPlus output variable name.
    pub variable_name: String,
    /// Display units.
    pub units: String,
    /// Reporting frequency.
    pub frequency: RuntimeOutputFrequency,
    /// Runtime source path.
    pub source: RuntimeOutputSource,
}

impl RuntimeOutputDefinition {
    fn identity(&self) -> OutputIdentity {
        OutputIdentity::new(&self.key, &self.variable_name, self.frequency)
    }
}

/// One meter the runtime knows how to produce.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeMeterDefinition {
    /// Stable meter handle for the current model and registry.
    pub handle: OutputHandle,
    /// EnergyPlus meter name.
    pub name: String,
    /// Display units.
    pub units: String,
    /// Reporting frequency.
    pub frequency: RuntimeOutputFrequency,
    /// Runtime source path.
    pub source: RuntimeOutputSource,
}

impl RuntimeMeterDefinition {
    fn identity(&self) -> MeterIdentity {
        MeterIdentity::new(&self.name, self.frequency)
    }
}

/// Runtime output registry resolved from the typed model before execution.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RuntimeOutputRegistry {
    outputs: Vec<RuntimeOutputDefinition>,
    meter_registry: RuntimeMeterRegistry,
}

impl RuntimeOutputRegistry {
    /// Creates an empty output registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
            meter_registry: RuntimeMeterRegistry::new(),
        }
    }

    /// Builds the runtime output registry for the currently implemented subset.
    #[must_use]
    pub fn from_model(model: &SimulationModel) -> Self {
        let mut registry = Self::new();
        registry.register_model_outputs(&model.typed);
        registry.register_model_meters(&model.typed);
        registry
    }

    /// Returns output definitions in handle order.
    #[must_use]
    pub fn outputs(&self) -> &[RuntimeOutputDefinition] {
        &self.outputs
    }

    /// Returns the meter registry.
    #[must_use]
    pub fn meter_registry(&self) -> &RuntimeMeterRegistry {
        &self.meter_registry
    }

    /// Returns the number of registered output variables.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outputs.len()
    }

    /// Returns true when the registry contains no output variables.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outputs.is_empty()
    }

    /// Finds an output definition by request identity.
    #[must_use]
    pub fn find_output(&self, request: &RuntimeOutputRequest) -> Option<&RuntimeOutputDefinition> {
        let identity = request.identity();
        self.outputs
            .iter()
            .find(|definition| definition.identity() == identity)
    }

    /// Resolves requested output variables and records unavailable/duplicate diagnostics.
    #[must_use]
    pub fn resolve_output_requests(
        &self,
        requests: &[RuntimeOutputRequest],
    ) -> RuntimeOutputResolution {
        let mut seen = BTreeSet::new();
        let mut resolved = Vec::new();
        let mut diagnostics = RuntimeDiagnosticStore::new();

        for request in requests {
            let identity = request.identity();
            if !seen.insert(identity) {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::DuplicateOutputRequest,
                    message: format!(
                        "duplicate runtime output request {} / {} ({})",
                        request.key,
                        request.variable_name,
                        request.frequency.id()
                    ),
                    key: Some(request.key.clone()),
                    variable_name: Some(request.variable_name.clone()),
                    meter_name: None,
                    handle: None,
                });
                continue;
            }

            if let Some(definition) = self.find_output(request) {
                resolved.push(RuntimeResolvedOutput {
                    request: request.clone(),
                    definition: definition.clone(),
                });
            } else {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::OutputVariableUnavailable,
                    message: format!(
                        "runtime output variable unavailable: {} / {} ({})",
                        request.key,
                        request.variable_name,
                        request.frequency.id()
                    ),
                    key: Some(request.key.clone()),
                    variable_name: Some(request.variable_name.clone()),
                    meter_name: None,
                    handle: None,
                });
            }
        }

        RuntimeOutputResolution {
            resolved,
            diagnostics,
        }
    }

    fn register_model_outputs(&mut self, model: &TypedModel) {
        for zone in &model.zones {
            self.push_output(
                &zone.name.0,
                "Zone Mean Air Temperature",
                "C",
                RuntimeOutputFrequency::Hourly,
                RuntimeOutputSource::RuntimeState,
            );
        }

        for surface in &model.surfaces {
            self.push_output(
                &surface.name.0,
                "Surface Inside Face Temperature",
                "C",
                RuntimeOutputFrequency::Hourly,
                RuntimeOutputSource::RuntimeState,
            );
            self.push_output(
                &surface.name.0,
                "Surface Inside Face Adjacent Air Temperature",
                "C",
                RuntimeOutputFrequency::Hourly,
                RuntimeOutputSource::RuntimeState,
            );
            self.push_output(
                &surface.name.0,
                "Surface Outside Face Temperature",
                "C",
                RuntimeOutputFrequency::Hourly,
                RuntimeOutputSource::RuntimeState,
            );
            if surface.sun_exposure == SunExposure::SunExposed
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
            {
                self.push_output(
                    &surface.name.0,
                    "Surface Outside Face Incident Solar Radiation Rate per Area",
                    "W/m2",
                    RuntimeOutputFrequency::Hourly,
                    RuntimeOutputSource::WeatherInput,
                );
            }
            if surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors {
                for (variable_name, units) in [
                    ("Surface Outside Face Convection Heat Gain Rate", "W"),
                    (
                        "Surface Outside Face Convection Heat Gain Rate per Area",
                        "W/m2",
                    ),
                    (
                        "Surface Outside Face Convection Heat Transfer Coefficient",
                        "W/m2-K",
                    ),
                    (
                        "Surface Outside Face Net Thermal Radiation Heat Gain Rate",
                        "W",
                    ),
                    (
                        "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area",
                        "W/m2",
                    ),
                    (
                        "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient",
                        "W/m2-K",
                    ),
                    (
                        "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient",
                        "W/m2-K",
                    ),
                    (
                        "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient",
                        "W/m2-K",
                    ),
                    ("Surface Outside Face Solar Radiation Heat Gain Rate", "W"),
                    (
                        "Surface Outside Face Solar Radiation Heat Gain Rate per Area",
                        "W/m2",
                    ),
                ] {
                    self.push_output(
                        &surface.name.0,
                        variable_name,
                        units,
                        RuntimeOutputFrequency::Hourly,
                        RuntimeOutputSource::RuntimeState,
                    );
                }
            }
            for (variable_name, units) in [
                ("Surface Inside Face Conduction Heat Transfer Rate", "W"),
                ("Surface Inside Face Conduction Heat Gain Rate", "W"),
                ("Surface Inside Face Conduction Heat Loss Rate", "W"),
                (
                    "Surface Inside Face Conduction Heat Transfer Rate per Area",
                    "W/m2",
                ),
                ("Surface Outside Face Conduction Heat Transfer Rate", "W"),
                ("Surface Outside Face Conduction Heat Gain Rate", "W"),
                ("Surface Outside Face Conduction Heat Loss Rate", "W"),
                (
                    "Surface Outside Face Conduction Heat Transfer Rate per Area",
                    "W/m2",
                ),
                ("Surface Heat Storage Rate", "W"),
                ("Surface Heat Storage Rate per Area", "W/m2"),
            ] {
                self.push_output(
                    &surface.name.0,
                    variable_name,
                    units,
                    RuntimeOutputFrequency::Hourly,
                    RuntimeOutputSource::RuntimeState,
                );
            }
        }

        for (variable_name, units) in [
            ("Site Outdoor Air Drybulb Temperature", "C"),
            ("Site Outdoor Air Wetbulb Temperature", "C"),
            ("Site Sky Temperature", "C"),
            ("Site Horizontal Infrared Radiation Rate per Area", "W/m2"),
            ("Site Rain Status", ""),
        ] {
            self.push_output(
                "Environment",
                variable_name,
                units,
                RuntimeOutputFrequency::Hourly,
                RuntimeOutputSource::WeatherInput,
            );
        }

        for zone in &model.zones {
            for variable_name in [
                "Zone Opaque Surface Inside Faces Conduction Rate",
                "Zone Opaque Surface Inside Faces Conduction Heat Gain Rate",
                "Zone Opaque Surface Inside Faces Conduction Heat Loss Rate",
                "Zone Opaque Surface Outside Faces Conduction Rate",
                "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate",
                "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate",
            ] {
                self.push_output(
                    &zone.name.0,
                    variable_name,
                    "W",
                    RuntimeOutputFrequency::Hourly,
                    RuntimeOutputSource::RuntimeState,
                );
            }
        }

        for schedule_id in schedule_ids(model) {
            if let Some(schedule_name) = schedule_name_for_id(model, schedule_id) {
                self.push_output(
                    &schedule_name,
                    "Schedule Value",
                    "",
                    RuntimeOutputFrequency::Hourly,
                    RuntimeOutputSource::Schedule,
                );
            }
        }

        for node in &model.nodes {
            for (variable_name, units) in [
                ("System Node Temperature", "C"),
                ("System Node Humidity Ratio", "kgWater/kgDryAir"),
                ("System Node Mass Flow Rate", "kg/s"),
            ] {
                self.push_output(
                    &node.name.0,
                    variable_name,
                    units,
                    RuntimeOutputFrequency::Hourly,
                    RuntimeOutputSource::RuntimeState,
                );
            }
        }

        for system in &model.ideal_loads_air_systems {
            for variable_name in [
                ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
                ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE,
                ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
                ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
                ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
                ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
            ] {
                self.push_output(
                    &system.name.0,
                    variable_name,
                    "W",
                    RuntimeOutputFrequency::Hourly,
                    RuntimeOutputSource::RuntimeState,
                );
            }
        }

        for plant_loop in &model.plant_loops {
            for (variable_name, units) in [
                ("Plant Supply Side Cooling Demand Rate", "W"),
                ("Plant Supply Side Heating Demand Rate", "W"),
                ("Plant Supply Side Inlet Mass Flow Rate", "kg/s"),
                ("Plant Supply Side Inlet Temperature", "C"),
                ("Plant Supply Side Outlet Temperature", "C"),
            ] {
                self.push_output(
                    &plant_loop.name.0,
                    variable_name,
                    units,
                    RuntimeOutputFrequency::Hourly,
                    RuntimeOutputSource::RuntimeState,
                );
            }
        }

        for component in plant_components(model) {
            let Some(variable_name) = plant_equipment_variable_name(&component.object_type.0)
            else {
                continue;
            };
            self.push_output(
                &component.name.0,
                variable_name,
                "W",
                RuntimeOutputFrequency::Hourly,
                RuntimeOutputSource::RuntimeState,
            );
        }
    }

    fn push_output(
        &mut self,
        key: &str,
        variable_name: &str,
        units: &str,
        frequency: RuntimeOutputFrequency,
        source: RuntimeOutputSource,
    ) {
        let identity = OutputIdentity::new(key, variable_name, frequency);
        if self
            .outputs
            .iter()
            .any(|definition| definition.identity() == identity)
        {
            return;
        }

        self.outputs.push(RuntimeOutputDefinition {
            handle: OutputHandle(self.outputs.len() as u32),
            key: NormalizedName::new(key).0,
            variable_name: variable_name.to_string(),
            units: units.to_string(),
            frequency,
            source,
        });
    }

    fn register_model_meters(&mut self, model: &TypedModel) {
        for system in &model.ideal_loads_air_systems {
            for fuel_type in [system.heating_fuel_type, system.cooling_fuel_type] {
                if let Some(binding) = ideal_loads_facility_meter_binding(fuel_type) {
                    for frequency in [
                        RuntimeOutputFrequency::Hourly,
                        RuntimeOutputFrequency::Monthly,
                        RuntimeOutputFrequency::Annual,
                        RuntimeOutputFrequency::RunPeriod,
                    ] {
                        self.meter_registry.push_meter(
                            binding.meter_name,
                            "J",
                            frequency,
                            RuntimeOutputSource::Meter,
                        );
                    }
                }
            }
        }
    }
}

/// Resolved output request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeResolvedOutput {
    /// Original request.
    pub request: RuntimeOutputRequest,
    /// Matching output definition.
    pub definition: RuntimeOutputDefinition,
}

/// Resolved meter request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeResolvedMeter {
    /// Original request.
    pub request: RuntimeMeterRequest,
    /// Matching meter definition.
    pub definition: RuntimeMeterDefinition,
}

/// Output resolution result.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RuntimeOutputResolution {
    /// Resolved output handles.
    pub resolved: Vec<RuntimeResolvedOutput>,
    /// Resolution diagnostics.
    pub diagnostics: RuntimeDiagnosticStore,
}

/// Meter resolution result.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RuntimeMeterResolution {
    /// Resolved meter handles.
    pub resolved: Vec<RuntimeResolvedMeter>,
    /// Resolution diagnostics.
    pub diagnostics: RuntimeDiagnosticStore,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct OutputIdentity {
    key: String,
    variable_name: String,
    frequency: RuntimeOutputFrequency,
}

impl OutputIdentity {
    fn new(key: &str, variable_name: &str, frequency: RuntimeOutputFrequency) -> Self {
        Self {
            key: NormalizedName::new(key).0,
            variable_name: normalize_identity(variable_name),
            frequency,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct MeterIdentity {
    name: String,
    frequency: RuntimeOutputFrequency,
}

impl MeterIdentity {
    fn new(name: &str, frequency: RuntimeOutputFrequency) -> Self {
        Self {
            name: normalize_identity(name),
            frequency,
        }
    }
}

fn normalize_identity(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn schedule_ids(model: &TypedModel) -> impl Iterator<Item = ScheduleId> + '_ {
    model
        .schedules
        .iter()
        .map(|schedule| schedule.id)
        .chain(model.compact_schedules.iter().map(|schedule| schedule.id))
}

fn schedule_name_for_id(model: &TypedModel, schedule_id: ScheduleId) -> Option<String> {
    model
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .map(|schedule| schedule.name.0.clone())
        .or_else(|| {
            model
                .compact_schedules
                .iter()
                .find(|schedule| schedule.id == schedule_id)
                .map(|schedule| schedule.name.0.clone())
        })
}

fn plant_components(model: &TypedModel) -> Vec<&PlantBranchComponent> {
    let mut components = Vec::new();
    for plant_loop in &model.plant_loops {
        for branch_list in [
            plant_loop.plant_side_branch_list,
            plant_loop.demand_side_branch_list,
        ] {
            for branch_id in plant_branch_ids_for_list(model, branch_list) {
                let Some(branch) = model
                    .plant_branches
                    .iter()
                    .find(|branch| branch.id == branch_id)
                else {
                    continue;
                };
                components.extend(branch.components.iter());
            }
        }
    }
    components
}

fn plant_branch_ids_for_list(
    model: &TypedModel,
    branch_list_id: BranchListId,
) -> Vec<ep_model::BranchId> {
    model
        .plant_branch_lists
        .iter()
        .find(|list| list.id == branch_list_id)
        .map(|list| list.branches.clone())
        .unwrap_or_default()
}

fn plant_equipment_variable_name(object_type: &str) -> Option<&'static str> {
    match object_type.to_ascii_lowercase().as_str() {
        "pump:constantspeed" | "pump:variablespeed" => Some("Pump Electricity Rate"),
        "districtheating:water" => Some("District Heating Water Rate"),
        "loadprofile:plant" => Some("Plant Load Profile Heat Transfer Rate"),
        _ => None,
    }
}
