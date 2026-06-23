//! Support assessment for arbitrary runs.

use std::collections::BTreeSet;

use ep_compiler::{CompileReport, DiagnosticSeverity, ObjectCoverageStatus};
use ep_model::{SimulationModel, TypedModel};
use ep_raw_model::RawModel;
use ep_runtime::classify_no_oa_sensible_subset;
use serde::{Deserialize, Serialize};

use crate::{
    PartialRunPolicy, RunDiagnostic, RunDiagnosticSeverity, RunDiagnostics, RunMode,
    RunOutputFormat, TraceLevel,
};

/// Capability registry path bundled with the repository and release package.
pub const CAPABILITY_REGISTRY_PATH: &str = "specs/capabilities.toml";
const CAPABILITY_REGISTRY_TOML: &str = include_str!("../../../specs/capabilities.toml");

/// Support status produced by the arbitrary-run gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SupportStatus {
    /// The input falls inside the current compatibility-mode runtime subset.
    SupportedCompatibility,
    /// The input can run only in a diagnostic, non-claim path.
    SupportedDiagnosticOnly,
    /// The input is outside the currently implemented runtime subset.
    Unsupported,
}

impl SupportStatus {
    /// Stable lower-case identifier for reports.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::SupportedCompatibility => "supported-compatibility",
            Self::SupportedDiagnosticOnly => "supported-diagnostic-only",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns true when Rust execution is allowed.
    #[must_use]
    pub const fn is_runnable(self) -> bool {
        matches!(
            self,
            Self::SupportedCompatibility | Self::SupportedDiagnosticOnly
        )
    }
}

/// User-facing run result state emitted by support assessment and run summaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunResultState {
    /// Unsupported active semantics prevent Rust execution.
    RunBlocked,
    /// The Rust runtime may execute only as a diagnostic/ad-hoc partial run.
    PartialSupportedRun,
    /// The Rust runtime may execute inside the declared compatibility subset.
    SupportedCompatibilityRun,
}

impl RunResultState {
    /// Stable lower-case identifier for JSON, reports, and launcher wiring.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::RunBlocked => "run_blocked",
            Self::PartialSupportedRun => "partial_supported_run",
            Self::SupportedCompatibilityRun => "supported_compatibility_run",
        }
    }

    /// Human-readable label matching `specs/run_result_states.toml`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::RunBlocked => "Cannot run",
            Self::PartialSupportedRun => "Partial supported run",
            Self::SupportedCompatibilityRun => "Supported compatibility run",
        }
    }

    /// Returns true when the Rust runtime is allowed to execute.
    #[must_use]
    pub const fn allows_rust_runtime(self) -> bool {
        matches!(
            self,
            Self::PartialSupportedRun | Self::SupportedCompatibilityRun
        )
    }

    /// Maps internal support status to the public run state for a requested mode.
    #[must_use]
    pub const fn from_support_status(
        status: SupportStatus,
        mode: RunMode,
        partial_policy: PartialRunPolicy,
    ) -> Self {
        match status {
            SupportStatus::SupportedCompatibility => Self::SupportedCompatibilityRun,
            SupportStatus::SupportedDiagnosticOnly
                if matches!(mode, RunMode::Diagnostic) && partial_policy.allows_partial() =>
            {
                Self::PartialSupportedRun
            }
            SupportStatus::SupportedDiagnosticOnly | SupportStatus::Unsupported => Self::RunBlocked,
        }
    }
}

/// Runtime class selected by support assessment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeClass {
    /// No Rust runtime can execute the input.
    None,
    /// Heat-balance zone-air diagnostic runtime.
    HeatBalanceZoneAirDiagnostic,
    /// IdealLoads node-state diagnostic projection runtime.
    IdealLoadsNodeStateProjection,
}

impl RuntimeClass {
    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::HeatBalanceZoneAirDiagnostic => "heat-balance-zone-air-diagnostic",
            Self::IdealLoadsNodeStateProjection => "ideal-loads-node-state-projection",
        }
    }
}

/// One object type entry in a support assessment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SupportObjectEntry {
    /// EnergyPlus object type.
    pub object_type: String,
    /// Instance count.
    pub count: usize,
    /// Support classification.
    pub status: String,
    /// Notes explaining the classification.
    pub note: String,
}

/// Boundary metadata copied to every arbitrary run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClaimBoundary {
    /// Whether this ad-hoc run is release conformance evidence.
    pub conformance_claim: bool,
    /// Whether this ad-hoc run is release evidence.
    pub release_evidence: bool,
    /// Human-readable boundary.
    pub statement: String,
}

/// Support assessment artifact.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SupportAssessment {
    /// Schema version.
    pub schema_version: u32,
    /// Assessment status.
    pub status: SupportStatus,
    /// User-facing run result state.
    pub run_result_state: RunResultState,
    /// Selected runtime class.
    pub runtime_class: RuntimeClass,
    /// Matched capability identifiers from the current support boundary.
    pub matched_capability_ids: Vec<String>,
    /// Requested mode.
    pub mode: String,
    /// Requested partial-run policy.
    pub partial_policy: String,
    /// Requested output format.
    pub output_format: String,
    /// Requested trace level.
    pub trace_level: String,
    /// Capability registry source.
    pub capability_registry: String,
    /// Whether the embedded capability registry parsed successfully.
    pub capability_registry_loaded: bool,
    /// Claim boundary.
    pub claim_boundary: ClaimBoundary,
    /// Typed object entries.
    pub typed_objects: Vec<SupportObjectEntry>,
    /// Raw-only object entries that are ignored by this runtime.
    pub ignored_raw_only_objects: Vec<SupportObjectEntry>,
    /// Unsupported object entries.
    pub unsupported_objects: Vec<SupportObjectEntry>,
    /// Diagnostics emitted by support assessment.
    pub diagnostics: RunDiagnostics,
}

#[derive(Debug, Default, Deserialize)]
struct CapabilityRegistrySpec {
    #[serde(default)]
    capability: Vec<CapabilitySpec>,
}

#[derive(Clone, Debug, Deserialize)]
struct CapabilitySpec {
    id: String,
}

#[derive(Debug)]
struct LoadedCapabilityRegistry {
    spec: CapabilityRegistrySpec,
    loaded: bool,
    error: Option<String>,
}

impl SupportAssessment {
    /// Returns true when this assessment permits Rust runtime execution.
    #[must_use]
    pub const fn allows_rust_runtime(&self) -> bool {
        self.run_result_state.allows_rust_runtime()
    }
}

/// Assesses whether a compiled model can use the arbitrary-run runtime.
#[must_use]
pub fn assess_support(
    raw_model: &RawModel,
    compile_report: &CompileReport,
    typed_model: Option<&TypedModel>,
    mode: RunMode,
    partial_policy: PartialRunPolicy,
    output_format: RunOutputFormat,
    trace_level: TraceLevel,
) -> SupportAssessment {
    let mut diagnostics = RunDiagnostics::default();
    let mut typed_objects = Vec::new();
    let mut ignored_raw_only_objects = Vec::new();
    let mut unsupported_objects = Vec::new();

    for diagnostic in &compile_report.diagnostics {
        let severity = match diagnostic.severity {
            DiagnosticSeverity::Error => RunDiagnosticSeverity::Error,
            DiagnosticSeverity::Warning => RunDiagnosticSeverity::Warning,
        };
        diagnostics.push(
            RunDiagnostic::new(
                severity,
                diagnostic.code.clone(),
                "compile",
                diagnostic.message.clone(),
            )
            .with_object(
                diagnostic.object_type.clone(),
                diagnostic.object_name.clone(),
            )
            .with_field(diagnostic.field.clone()),
        );
    }

    for coverage in &compile_report.coverage {
        match coverage.status {
            ObjectCoverageStatus::Typed => typed_objects.push(SupportObjectEntry {
                object_type: coverage.object_type.clone(),
                count: coverage.object_count,
                status: "typed".to_string(),
                note: "compiled into the current TypedModel subset".to_string(),
            }),
            ObjectCoverageStatus::RawOnly if ignored_raw_only_object(&coverage.object_type) => {
                ignored_raw_only_objects.push(SupportObjectEntry {
                    object_type: coverage.object_type.clone(),
                    count: coverage.object_count,
                    status: "ignored-runtime-control-or-reporting".to_string(),
                    note: ignored_raw_only_note(&coverage.object_type).to_string(),
                });
                diagnostics.push(
                    RunDiagnostic::new(
                        RunDiagnosticSeverity::Warning,
                        "UnsupportedObjectIgnored",
                        "support",
                        format!(
                            "{} is preserved in RawModel but ignored by the current Rust runtime",
                            coverage.object_type
                        ),
                    )
                    .with_object(coverage.object_type.clone(), None),
                );
            }
            ObjectCoverageStatus::RawOnly => {
                let (code, note) = unsupported_object_reason(&coverage.object_type);
                unsupported_objects.push(SupportObjectEntry {
                    object_type: coverage.object_type.clone(),
                    count: coverage.object_count,
                    status: "unsupported".to_string(),
                    note: note.to_string(),
                });
                diagnostics.push(
                    RunDiagnostic::new(
                        RunDiagnosticSeverity::Error,
                        code,
                        "support",
                        format!(
                            "{} is outside the current arbitrary-run runtime subset",
                            coverage.object_type
                        ),
                    )
                    .with_object(coverage.object_type.clone(), None),
                );
            }
        }
    }

    assess_typed_runtime_boundaries(
        typed_model,
        raw_model,
        mode,
        &mut unsupported_objects,
        &mut diagnostics,
    );
    let capability_registry = load_embedded_capability_registry();
    if let Some(error) = capability_registry.error.as_ref() {
        diagnostics.warning(
            "CapabilityRegistryParseFailed",
            "support",
            format!("failed to parse embedded {CAPABILITY_REGISTRY_PATH}: {error}"),
        );
    }

    let (status, runtime_class) = if diagnostics.has_errors() || typed_model.is_none() {
        (SupportStatus::Unsupported, RuntimeClass::None)
    } else {
        runtime_status_for_typed_model(typed_model)
    };
    let run_result_state = RunResultState::from_support_status(status, mode, partial_policy);
    if status == SupportStatus::SupportedDiagnosticOnly
        && run_result_state == RunResultState::RunBlocked
    {
        diagnostics.error(
            "DiagnosticOnlyRuntimeBlocked",
            "support",
            "diagnostic-only runtime classes require --mode diagnostic --partial allowed and cannot execute as compatibility evidence",
        );
    }

    SupportAssessment {
        schema_version: 1,
        status,
        run_result_state,
        runtime_class,
        matched_capability_ids: matched_capability_ids(
            status,
            runtime_class,
            &capability_registry.spec,
        ),
        mode: mode.id().to_string(),
        partial_policy: partial_policy.id().to_string(),
        output_format: output_format.id().to_string(),
        trace_level: trace_level.id().to_string(),
        capability_registry: CAPABILITY_REGISTRY_PATH.to_string(),
        capability_registry_loaded: capability_registry.loaded,
        claim_boundary: ClaimBoundary {
            conformance_claim: false,
            release_evidence: false,
            statement: "Ad-hoc arbitrary runs do not become release conformance evidence automatically; compare reports are diagnostic unless promoted by a reviewed release manifest.".to_string(),
        },
        typed_objects,
        ignored_raw_only_objects,
        unsupported_objects,
        diagnostics,
    }
}

fn load_embedded_capability_registry() -> LoadedCapabilityRegistry {
    match toml::from_str::<CapabilityRegistrySpec>(CAPABILITY_REGISTRY_TOML) {
        Ok(spec) => LoadedCapabilityRegistry {
            spec,
            loaded: true,
            error: None,
        },
        Err(error) => LoadedCapabilityRegistry {
            spec: CapabilityRegistrySpec::default(),
            loaded: false,
            error: Some(error.to_string()),
        },
    }
}

fn matched_capability_ids(
    status: SupportStatus,
    runtime_class: RuntimeClass,
    registry: &CapabilityRegistrySpec,
) -> Vec<String> {
    let target_id = match (status, runtime_class) {
        (SupportStatus::SupportedCompatibility, RuntimeClass::HeatBalanceZoneAirDiagnostic) => {
            "official_1zone_uncontrolled_declared_heat_balance"
        }
        (SupportStatus::SupportedDiagnosticOnly, RuntimeClass::IdealLoadsNodeStateProjection) => {
            "ideal_loads_no_oa_sensible"
        }
        _ => return Vec::new(),
    };

    registry
        .capability
        .iter()
        .find(|capability| capability.id == target_id)
        .map(|capability| vec![capability.id.clone()])
        .unwrap_or_else(|| vec![target_id.to_string()])
}

fn runtime_status_for_typed_model(
    typed_model: Option<&TypedModel>,
) -> (SupportStatus, RuntimeClass) {
    let Some(typed_model) = typed_model else {
        return (SupportStatus::Unsupported, RuntimeClass::None);
    };

    if !typed_model.ideal_loads_air_systems.is_empty() {
        return (
            SupportStatus::SupportedDiagnosticOnly,
            RuntimeClass::IdealLoadsNodeStateProjection,
        );
    }

    (
        SupportStatus::SupportedCompatibility,
        RuntimeClass::HeatBalanceZoneAirDiagnostic,
    )
}

fn assess_typed_runtime_boundaries(
    typed_model: Option<&TypedModel>,
    raw_model: &RawModel,
    mode: RunMode,
    unsupported_objects: &mut Vec<SupportObjectEntry>,
    diagnostics: &mut RunDiagnostics,
) {
    let Some(typed_model) = typed_model else {
        return;
    };

    if typed_model.zones.is_empty() {
        diagnostics.error(
            "UnsupportedTopology",
            "support",
            "no Zone objects are available for the current runtime",
        );
    }
    if typed_model.zones.len() > 1 {
        diagnostics.error(
            "UnsupportedTopology",
            "support",
            format!(
                "the arbitrary runtime currently supports one-zone cases; found {} zones",
                typed_model.zones.len()
            ),
        );
    }

    if !typed_model.plant_loops.is_empty()
        || !typed_model.plant_branches.is_empty()
        || !typed_model.pumps_constant_speed.is_empty()
        || !typed_model.boilers_hot_water.is_empty()
        || !typed_model.chillers_electric_eir.is_empty()
    {
        push_typed_boundary(
            unsupported_objects,
            diagnostics,
            "PlantLoop/PlantEquipment",
            typed_model.plant_loops.len()
                + typed_model.plant_branches.len()
                + typed_model.pumps_constant_speed.len()
                + typed_model.boilers_hot_water.len()
                + typed_model.chillers_electric_eir.len(),
            "UnsupportedPlantObject",
            "Plant objects are typed for graph diagnostics but not executable in arbitrary-run compatibility mode",
        );
    }

    if !typed_model.ideal_loads_air_systems.is_empty() {
        let simulation_model = SimulationModel::from_typed(typed_model.clone());
        if simulation_model.graph.zone_ideal_loads.is_empty()
            || simulation_model.graph.ideal_loads_supply_nodes.is_empty()
        {
            diagnostics.error(
                "UnsupportedTopology",
                "support",
                "IdealLoads systems require zone equipment connections and resolvable supply nodes",
            );
        }

        for system in &typed_model.ideal_loads_air_systems {
            let boundary = classify_no_oa_sensible_subset(system);
            if !boundary.is_supported() {
                diagnostics.push(
                    RunDiagnostic::new(
                        RunDiagnosticSeverity::Error,
                        "UnsupportedAlgorithm",
                        "support",
                        format!(
                            "IdealLoads system '{}' uses unsupported feature flags: {:?}",
                            system.name.0, boundary.unsupported_features
                        ),
                    )
                    .with_object("ZoneHVAC:IdealLoadsAirSystem", Some(system.name.0.clone())),
                );
            }
        }
    }

    if mode == RunMode::Fast || mode == RunMode::Experimental {
        diagnostics.warning(
            "UnsupportedAlgorithm",
            "support",
            format!(
                "{} mode is parsed but currently uses the same runtime boundary as compatibility/diagnostic mode",
                mode.id()
            ),
        );
    }

    warn_for_ignored_semantic_objects(raw_model, diagnostics);
}

fn push_typed_boundary(
    unsupported_objects: &mut Vec<SupportObjectEntry>,
    diagnostics: &mut RunDiagnostics,
    label: &str,
    count: usize,
    code: &str,
    note: &str,
) {
    unsupported_objects.push(SupportObjectEntry {
        object_type: label.to_string(),
        count,
        status: "unsupported".to_string(),
        note: note.to_string(),
    });
    diagnostics.error(code, "support", note);
}

fn warn_for_ignored_semantic_objects(raw_model: &RawModel, diagnostics: &mut RunDiagnostics) {
    for object_type in raw_model.objects.keys() {
        if matches!(
            object_type.0.as_str(),
            "HeatBalanceAlgorithm"
                | "SimulationControl"
                | "SizingPeriod:DesignDay"
                | "Exterior:Lights"
                | "GlobalGeometryRules"
        ) {
            diagnostics.push(
                RunDiagnostic::new(
                    RunDiagnosticSeverity::Warning,
                    "UnsupportedAlgorithmIgnored",
                    "support",
                    format!(
                        "{} is recorded but not currently evaluated by the Rust arbitrary runtime",
                        object_type.0
                    ),
                )
                .with_object(object_type.0.clone(), None),
            );
        }
    }
}

fn ignored_raw_only_object(object_type: &str) -> bool {
    object_type.starts_with("Output:")
        || object_type.starts_with("OutputControl:")
        || matches!(
            object_type,
            "SimulationControl"
                | "SizingPeriod:DesignDay"
                | "GlobalGeometryRules"
                | "HeatBalanceAlgorithm"
                | "ShadowCalculation"
                | "Exterior:Lights"
        )
}

fn ignored_raw_only_note(object_type: &str) -> &'static str {
    if object_type.starts_with("Output:") || object_type.starts_with("OutputControl:") {
        "output/reporting request handled by run artifact export or oracle baseline injection"
    } else {
        "input-control object preserved for diagnostics; current Rust runtime uses fixed compatibility defaults"
    }
}

fn unsupported_object_reason(object_type: &str) -> (&'static str, &'static str) {
    if object_type.starts_with("EnergyManagementSystem:") {
        return (
            "UnsupportedEMS",
            "EnergyManagementSystem objects are not ported",
        );
    }
    if object_type.starts_with("PythonPlugin:") {
        return (
            "UnsupportedPythonPlugin",
            "PythonPlugin objects are not ported",
        );
    }
    if object_type.starts_with("AirflowNetwork:") {
        return (
            "UnsupportedAirflowNetwork",
            "AirflowNetwork objects are not ported",
        );
    }
    if object_type.starts_with("Sizing:") || object_type.starts_with("ZoneSizing") {
        return ("UnsupportedSizing", "sizing workflows are not ported");
    }
    if is_hvac_object(object_type) {
        return (
            "UnsupportedHVACObject",
            "broad HVAC object families are outside the current arbitrary runtime",
        );
    }
    if is_plant_object(object_type) {
        return (
            "UnsupportedPlantObject",
            "plant objects are outside the current arbitrary runtime",
        );
    }
    if is_surface_boundary_object(object_type) {
        return (
            "UnsupportedSurfaceBoundary",
            "fenestration, daylighting, shading, and advanced surface boundary objects are not ported",
        );
    }

    (
        "UnsupportedObject",
        "object type is preserved in RawModel but lacks a runtime support declaration",
    )
}

fn is_hvac_object(object_type: &str) -> bool {
    let allowed: BTreeSet<&str> = [
        "ZoneHVAC:IdealLoadsAirSystem",
        "ZoneHVAC:EquipmentList",
        "ZoneHVAC:EquipmentConnections",
    ]
    .into_iter()
    .collect();
    (object_type.starts_with("AirLoopHVAC")
        || object_type.starts_with("Fan:")
        || object_type.starts_with("Coil:")
        || object_type.starts_with("Controller:")
        || object_type.starts_with("SetpointManager:")
        || object_type.starts_with("AirTerminal:")
        || object_type.starts_with("OutdoorAir:")
        || object_type.starts_with("ZoneHVAC:"))
        && !allowed.contains(object_type)
}

fn is_plant_object(object_type: &str) -> bool {
    object_type == "PlantLoop"
        || object_type == "Branch"
        || object_type == "BranchList"
        || object_type.starts_with("Connector:")
        || object_type == "ConnectorList"
        || object_type.starts_with("Pump:")
        || object_type.starts_with("Boiler:")
        || object_type.starts_with("Chiller:")
}

fn is_surface_boundary_object(object_type: &str) -> bool {
    object_type.starts_with("FenestrationSurface:")
        || object_type.starts_with("Window")
        || object_type.starts_with("Daylighting:")
        || object_type.starts_with("Shading:")
        || object_type.starts_with("Site:Ground")
}

#[cfg(test)]
mod tests {
    use super::{RunResultState, SupportStatus, assess_support};
    use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
    use ep_compiler::compile_raw_model;
    use ep_raw_model::parse_epjson_str;

    #[test]
    fn simple_one_zone_model_is_supported() -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Material:NoMass": {"R13": {"thermal_resistance": 2.29}},
                "Construction": {"Wall": {"outside_layer": "R13"}},
                "BuildingSurface:Detailed": {
                    "Wall One": {
                        "surface_type": "Wall",
                        "construction_name": "Wall",
                        "zone_name": "Zone One",
                        "outside_boundary_condition": "Outdoors",
                        "vertices": [
                            {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                            {"vertex_x_coordinate": 1, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                            {"vertex_x_coordinate": 1, "vertex_y_coordinate": 0, "vertex_z_coordinate": 1},
                            {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 1}
                        ]
                    }
                }
            }"#,
        )?;
        let result = compile_raw_model(&raw);
        let assessment = assess_support(
            &raw,
            &result.report,
            result.model.as_ref(),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
        );

        assert_eq!(assessment.status, SupportStatus::SupportedCompatibility);
        assert_eq!(
            assessment.run_result_state,
            RunResultState::SupportedCompatibilityRun
        );
        assert_eq!(
            assessment.matched_capability_ids,
            vec!["official_1zone_uncontrolled_declared_heat_balance"]
        );
        assert!(assessment.capability_registry_loaded);
        Ok(())
    }

    #[test]
    fn diagnostic_only_support_is_blocked_outside_diagnostic_mode() {
        assert_eq!(
            RunResultState::from_support_status(
                SupportStatus::SupportedDiagnosticOnly,
                RunMode::Compatibility,
                PartialRunPolicy::Deny
            ),
            RunResultState::RunBlocked
        );
        assert_eq!(
            RunResultState::from_support_status(
                SupportStatus::SupportedDiagnosticOnly,
                RunMode::Diagnostic,
                PartialRunPolicy::Allow
            ),
            RunResultState::PartialSupportedRun
        );
        assert_eq!(
            RunResultState::from_support_status(
                SupportStatus::SupportedDiagnosticOnly,
                RunMode::Diagnostic,
                PartialRunPolicy::Deny
            ),
            RunResultState::RunBlocked
        );
    }
}
