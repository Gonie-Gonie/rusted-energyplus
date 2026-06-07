//! Runtime output, meter, diagnostic, and result-store primitives.

use ep_model::{
    BranchListId, NormalizedName, OutputHandle, PlantBranchComponent, ScheduleId, SimulationModel,
    TypedModel,
};
use std::collections::BTreeSet;

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
    /// Creates an hourly meter request.
    #[must_use]
    pub fn hourly(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            frequency: RuntimeOutputFrequency::Hourly,
        }
    }

    fn identity(&self) -> MeterIdentity {
        MeterIdentity::new(&self.name, self.frequency)
    }
}

/// One output series stored by the runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputSeries {
    /// Stable output handle for the current run.
    pub handle: OutputHandle,
    /// EnergyPlus-style output key.
    pub key: String,
    /// Output variable name.
    pub variable_name: String,
    /// Display units.
    pub units: String,
    /// Sampled output values.
    pub values: Vec<f64>,
}

/// Structured output store for runtime-native results.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResultStore {
    /// Output series in handle order.
    pub series: Vec<OutputSeries>,
}

impl ResultStore {
    /// Creates an empty result store.
    #[must_use]
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    /// Adds a complete output series.
    pub fn add_series(&mut self, series: OutputSeries) {
        self.series.push(series);
    }

    /// Returns the maximum sample count across all output series.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.series
            .iter()
            .map(|series| series.values.len())
            .max()
            .unwrap_or(0)
    }

    /// Finds one output series by EnergyPlus-style key and variable name.
    #[must_use]
    pub fn find_series(&self, key: &str, variable_name: &str) -> Option<&OutputSeries> {
        self.series.iter().find(|series| {
            series.key.eq_ignore_ascii_case(key)
                && series.variable_name.eq_ignore_ascii_case(variable_name)
        })
    }

    /// Finds one output series by runtime output handle.
    #[must_use]
    pub fn find_handle(&self, handle: OutputHandle) -> Option<&OutputSeries> {
        self.series.iter().find(|series| series.handle == handle)
    }

    /// Returns result-store diagnostics for duplicate handles or identities.
    #[must_use]
    pub fn diagnostics(&self) -> RuntimeDiagnosticStore {
        let mut diagnostics = RuntimeDiagnosticStore::new();
        let mut handles = BTreeSet::new();
        let mut identities = BTreeSet::new();

        for series in &self.series {
            if !handles.insert(series.handle.0) {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::DuplicateOutputHandle,
                    message: format!("duplicate runtime output handle {}", series.handle.0),
                    key: Some(series.key.clone()),
                    variable_name: Some(series.variable_name.clone()),
                    meter_name: None,
                    handle: Some(series.handle),
                });
            }

            let identity = OutputIdentity::new(
                &series.key,
                &series.variable_name,
                RuntimeOutputFrequency::Hourly,
            );
            if !identities.insert(identity) {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::DuplicateOutputSeries,
                    message: format!(
                        "duplicate runtime output series {} / {}",
                        series.key, series.variable_name
                    ),
                    key: Some(series.key.clone()),
                    variable_name: Some(series.variable_name.clone()),
                    meter_name: None,
                    handle: Some(series.handle),
                });
            }
        }

        diagnostics
    }

    /// Returns a compact profile snapshot for reports and release evidence.
    #[must_use]
    pub fn profile(&self) -> ResultStoreProfile {
        ResultStoreProfile {
            series_count: self.series.len(),
            sample_count: self.sample_count(),
            empty_series_count: self
                .series
                .iter()
                .filter(|series| series.values.is_empty())
                .count(),
        }
    }
}

/// Compact result-store profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResultStoreProfile {
    /// Number of output series.
    pub series_count: usize,
    /// Maximum sample count across series.
    pub sample_count: usize,
    /// Number of output series without samples.
    pub empty_series_count: usize,
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
                "Surface Outside Face Temperature",
                "C",
                RuntimeOutputFrequency::Hourly,
                RuntimeOutputSource::RuntimeState,
            );
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

        self.push_output(
            "Environment",
            "Site Outdoor Air Drybulb Temperature",
            "C",
            RuntimeOutputFrequency::Hourly,
            RuntimeOutputSource::WeatherInput,
        );

        for zone in &model.zones {
            for variable_name in [
                "Zone Opaque Surface Inside Faces Conduction Rate",
                "Zone Opaque Surface Inside Faces Conduction Heat Gain Rate",
                "Zone Opaque Surface Inside Faces Conduction Heat Loss Rate",
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

/// Runtime diagnostic severity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeDiagnosticSeverity {
    /// Informational note.
    Info,
    /// Warning that does not block execution.
    Warning,
    /// Error that should block the requested output path.
    Error,
}

/// Runtime diagnostic code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeDiagnosticCode {
    /// Requested output variable is not registered for the current model.
    OutputVariableUnavailable,
    /// Requested meter is not registered for the current model.
    MeterUnavailable,
    /// Duplicate output request.
    DuplicateOutputRequest,
    /// Duplicate meter request.
    DuplicateMeterRequest,
    /// Duplicate output handle in a result store.
    DuplicateOutputHandle,
    /// Duplicate output key/variable identity in a result store.
    DuplicateOutputSeries,
}

/// One runtime diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeDiagnostic {
    /// Severity.
    pub severity: RuntimeDiagnosticSeverity,
    /// Stable diagnostic code.
    pub code: RuntimeDiagnosticCode,
    /// Human-readable message.
    pub message: String,
    /// Output key, when applicable.
    pub key: Option<String>,
    /// Output variable name, when applicable.
    pub variable_name: Option<String>,
    /// Meter name, when applicable.
    pub meter_name: Option<String>,
    /// Output handle, when applicable.
    pub handle: Option<OutputHandle>,
}

/// Runtime diagnostic collection.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RuntimeDiagnosticStore {
    /// Stored diagnostics in encounter order.
    pub diagnostics: Vec<RuntimeDiagnostic>,
}

impl RuntimeDiagnosticStore {
    /// Creates an empty diagnostic store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Adds one diagnostic.
    pub fn push(&mut self, diagnostic: RuntimeDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns true when any error-level diagnostic is present.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == RuntimeDiagnosticSeverity::Error)
    }

    /// Returns the number of stored diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns true when no diagnostics are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
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
