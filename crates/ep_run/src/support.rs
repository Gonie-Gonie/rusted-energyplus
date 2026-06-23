//! Support assessment for arbitrary runs.

use std::collections::BTreeSet;

use ep_compiler::{CompileReport, DiagnosticSeverity, ObjectCoverageStatus};
use ep_model::{SimulationModel, TypedModel};
use ep_raw_model::RawModel;
use ep_runtime::{
    IdealLoadsPurchasedAirBranch, classify_no_oa_sensible_subset, select_purchased_air_branch,
    validate_ideal_loads_zone_equipment_dispatch,
};
use serde::Serialize;

use crate::{
    PartialRunPolicy, RunDiagnostic, RunDiagnosticSeverity, RunDiagnostics, RunMode,
    RunOutputFormat, TraceLevel,
    support_registry::{
        CapabilityRegistrySpec, load_embedded_capability_registry, partial_rule_for_object,
        registry_capability, registry_capability_ids_or_fallback, unsupported_rule_for_object,
    },
};

pub use crate::support_registry::CAPABILITY_REGISTRY_PATH;

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
    /// One-zone opaque heat-balance compatibility runtime.
    OneZoneHeatBalanceCompatibility,
    /// Legacy heat-balance zone-air diagnostic runtime.
    HeatBalanceZoneAirDiagnostic,
    /// IdealLoads no-OA/no-limit sensible compatibility runtime.
    IdealLoadsNoOaSensibleCompatibility,
    /// IdealLoads no-OA numeric finite-limit compatibility runtime.
    IdealLoadsFiniteLimitCompatibility,
    /// IdealLoads no-OA ConstantSensibleHeatRatio compatibility runtime.
    IdealLoadsConstantShrCompatibility,
    /// Legacy broad IdealLoads node-state diagnostic projection runtime.
    IdealLoadsNodeStateProjection,
}

impl RuntimeClass {
    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OneZoneHeatBalanceCompatibility => "one-zone-heat-balance-compatibility",
            Self::HeatBalanceZoneAirDiagnostic => "heat-balance-zone-air-diagnostic",
            Self::IdealLoadsNoOaSensibleCompatibility => "ideal-loads-no-oa-sensible-compatibility",
            Self::IdealLoadsFiniteLimitCompatibility => "ideal-loads-finite-limit-compatibility",
            Self::IdealLoadsConstantShrCompatibility => "ideal-loads-constant-shr-compatibility",
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

/// Capability metadata matched by support assessment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MatchedCapabilityEntry {
    /// Capability identifier.
    pub id: String,
    /// Runtime domain declared by the registry.
    pub domain: String,
    /// Support level declared by the registry.
    pub support_level: String,
    /// Run state declared by the registry.
    pub run_state: String,
    /// Required object families declared by the registry.
    pub required_objects: Vec<String>,
    /// Active features forbidden by the registry.
    pub forbidden_active_features: Vec<String>,
    /// Algorithm identifiers backing the capability.
    pub algorithms: Vec<String>,
    /// Capability-specific claim boundary.
    pub claim_boundary: String,
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
    /// Matched capability metadata from the capability registry.
    pub matched_capabilities: Vec<MatchedCapabilityEntry>,
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
    let capability_registry = load_embedded_capability_registry();
    if let Some(error) = capability_registry.error.as_ref() {
        diagnostics.warning(
            "CapabilityRegistryParseFailed",
            "support",
            format!("failed to parse embedded {CAPABILITY_REGISTRY_PATH}: {error}"),
        );
    }

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
            ObjectCoverageStatus::RawOnly => {
                if let Some(rule) =
                    partial_rule_for_object(&capability_registry.spec, &coverage.object_type)
                {
                    ignored_raw_only_objects.push(SupportObjectEntry {
                        object_type: coverage.object_type.clone(),
                        count: coverage.object_count,
                        status: rule.id.clone(),
                        note: rule.reason.clone(),
                    });
                    diagnostics.push(
                        RunDiagnostic::new(
                            RunDiagnosticSeverity::Warning,
                            "UnsupportedObjectIgnored",
                            "support",
                            format!(
                                "{} is preserved in RawModel and ignored by partial rule {}",
                                coverage.object_type, rule.id
                            ),
                        )
                        .with_object(coverage.object_type.clone(), None),
                    );
                } else {
                    let (code, note) =
                        unsupported_object_reason(&capability_registry.spec, &coverage.object_type);
                    unsupported_objects.push(SupportObjectEntry {
                        object_type: coverage.object_type.clone(),
                        count: coverage.object_count,
                        status: "unsupported".to_string(),
                        note,
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
    }

    assess_typed_runtime_boundaries(
        typed_model,
        raw_model,
        mode,
        &mut unsupported_objects,
        &mut diagnostics,
    );
    let (status, runtime_class, matched_capability_ids) =
        if diagnostics.has_errors() || typed_model.is_none() {
            (SupportStatus::Unsupported, RuntimeClass::None, Vec::new())
        } else {
            runtime_status_for_typed_model(typed_model, &capability_registry.spec)
        };
    let matched_capabilities =
        matched_capabilities(&matched_capability_ids, &capability_registry.spec);
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
        matched_capability_ids,
        matched_capabilities,
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

fn matched_capabilities(
    ids: &[String],
    registry: &CapabilityRegistrySpec,
) -> Vec<MatchedCapabilityEntry> {
    ids.iter()
        .filter_map(|id| registry_capability(registry, id))
        .map(|capability| MatchedCapabilityEntry {
            id: capability.id.clone(),
            domain: capability.domain.clone(),
            support_level: capability.support_level.clone(),
            run_state: capability.run_state.clone(),
            required_objects: capability.required_objects.clone(),
            forbidden_active_features: capability.forbidden_active_features.clone(),
            algorithms: capability.algorithms.clone(),
            claim_boundary: capability.claim_boundary.clone(),
        })
        .collect()
}

fn runtime_status_for_typed_model(
    typed_model: Option<&TypedModel>,
    registry: &CapabilityRegistrySpec,
) -> (SupportStatus, RuntimeClass, Vec<String>) {
    let Some(typed_model) = typed_model else {
        return (SupportStatus::Unsupported, RuntimeClass::None, Vec::new());
    };

    if !typed_model.ideal_loads_air_systems.is_empty() {
        let mut capability_ids = BTreeSet::new();
        let mut selected_runtime_class = None;
        for system in &typed_model.ideal_loads_air_systems {
            let branch = select_purchased_air_branch(system);
            capability_ids.insert(ideal_loads_capability_id_for_branch(branch).to_string());
            let branch_runtime_class = ideal_loads_runtime_class_for_branch(branch);
            selected_runtime_class = Some(match selected_runtime_class {
                Some(existing) if existing != branch_runtime_class => {
                    RuntimeClass::IdealLoadsNodeStateProjection
                }
                Some(existing) => existing,
                None => branch_runtime_class,
            });
        }

        let runtime_class =
            selected_runtime_class.unwrap_or(RuntimeClass::IdealLoadsNodeStateProjection);
        let status = if runtime_class == RuntimeClass::IdealLoadsNodeStateProjection {
            SupportStatus::SupportedDiagnosticOnly
        } else {
            SupportStatus::SupportedCompatibility
        };
        return (
            status,
            runtime_class,
            registry_capability_ids_or_fallback(
                registry,
                capability_ids.into_iter().collect::<Vec<_>>(),
            ),
        );
    }

    (
        SupportStatus::SupportedCompatibility,
        RuntimeClass::OneZoneHeatBalanceCompatibility,
        registry_capability_ids_or_fallback(
            registry,
            vec!["official_1zone_uncontrolled_declared_heat_balance".to_string()],
        ),
    )
}

const fn ideal_loads_capability_id_for_branch(
    branch: IdealLoadsPurchasedAirBranch,
) -> &'static str {
    match branch {
        IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible => "ideal_loads_no_oa_sensible",
        IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlow
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity => "ideal_loads_finite_limits",
        IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling => {
            "ideal_loads_constant_shr"
        }
        IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling
        | IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityHeating
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatDehumidification
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatHumidification => {
            "ideal_loads_humidity_selected_branches"
        }
    }
}

const fn ideal_loads_runtime_class_for_branch(
    branch: IdealLoadsPurchasedAirBranch,
) -> RuntimeClass {
    match branch {
        IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible => {
            RuntimeClass::IdealLoadsNoOaSensibleCompatibility
        }
        IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlow
        | IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity => {
            RuntimeClass::IdealLoadsFiniteLimitCompatibility
        }
        IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling => {
            RuntimeClass::IdealLoadsConstantShrCompatibility
        }
        IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling
        | IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityHeating
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatDehumidification
        | IdealLoadsPurchasedAirBranch::NoOaHumidistatHumidification => {
            RuntimeClass::IdealLoadsNodeStateProjection
        }
    }
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
            let validation =
                validate_ideal_loads_zone_equipment_dispatch(&simulation_model, system.id);
            if !validation.is_dispatchable() {
                diagnostics.push(
                    RunDiagnostic::new(
                        RunDiagnosticSeverity::Error,
                        "UnsupportedTopology",
                        "support",
                        format!(
                            "IdealLoads system '{}' is not dispatchable through ZoneEquipmentManager: {:?}",
                            system.name.0,
                            validation.issue_codes()
                        ),
                    )
                    .with_object("ZoneHVAC:IdealLoadsAirSystem", Some(system.name.0.clone())),
                );
            }

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

fn unsupported_object_reason(
    registry: &CapabilityRegistrySpec,
    object_type: &str,
) -> (String, String) {
    if let Some(rule) = unsupported_rule_for_object(registry, object_type) {
        return (
            unsupported_rule_code(rule.id.as_str(), object_type),
            rule.reason.clone(),
        );
    }

    (
        "UnsupportedObject".to_string(),
        "object type is preserved in RawModel but lacks a runtime support declaration".to_string(),
    )
}

fn unsupported_rule_code(rule_id: &str, object_type: &str) -> String {
    if object_type.starts_with("EnergyManagementSystem:") {
        return "UnsupportedEMS".to_string();
    }
    if object_type.starts_with("PythonPlugin:") {
        return "UnsupportedPythonPlugin".to_string();
    }
    if object_type.starts_with("AirflowNetwork:") {
        return "UnsupportedAirflowNetwork".to_string();
    }

    match rule_id {
        "unsupported_hvac_air_loop" => "UnsupportedHVACObject",
        "unsupported_hvac_zone_equipment" => "UnsupportedHVACObject",
        "unsupported_plant" => "UnsupportedPlantObject",
        "unsupported_ems_python_airflow" => "UnsupportedRuntimeModifier",
        "unsupported_sizing" => "UnsupportedSizing",
        "unsupported_surface_boundary" => "UnsupportedSurfaceBoundary",
        _ => "UnsupportedObject",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
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
            assessment.runtime_class,
            RuntimeClass::OneZoneHeatBalanceCompatibility
        );
        assert_eq!(
            assessment.run_result_state,
            RunResultState::SupportedCompatibilityRun
        );
        assert_eq!(
            assessment.matched_capability_ids,
            vec!["official_1zone_uncontrolled_declared_heat_balance"]
        );
        assert_eq!(assessment.matched_capabilities.len(), 1);
        assert_eq!(assessment.matched_capabilities[0].domain, "heat_balance");
        assert!(assessment.capability_registry_loaded);
        Ok(())
    }

    #[test]
    fn output_objects_use_partial_rule_from_registry() -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Output:Variable": {
                    "Zone Mean Air Temperature": {
                        "key_value": "*",
                        "variable_name": "Zone Mean Air Temperature",
                        "reporting_frequency": "Hourly"
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

        assert!(
            assessment
                .ignored_raw_only_objects
                .iter()
                .any(|entry| entry.object_type == "Output:Variable"
                    && entry.status == "ignored_reporting_objects")
        );
        Ok(())
    }

    #[test]
    fn hvac_air_loop_uses_unsupported_rule_from_registry() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "AirLoopHVAC": {"Main Air Loop": {}}
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

        assert_eq!(assessment.status, SupportStatus::Unsupported);
        assert!(
            assessment
                .unsupported_objects
                .iter()
                .any(|entry| entry.object_type == "AirLoopHVAC"
                    && entry.note == "Broad HVAC air-loop semantics are not ported.")
        );
        assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "UnsupportedHVACObject"
                && diagnostic.object_type.as_deref() == Some("AirLoopHVAC")
        }));
        Ok(())
    }

    #[test]
    fn ideal_loads_no_oa_branch_matches_registry_capability()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [{"node_name": "Zone One Inlet"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "dehumidification_control_type": "None",
                        "humidification_control_type": "None"
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
        )?;
        let result = compile_raw_model(&raw);
        let assessment = assess_support(
            &raw,
            &result.report,
            result.model.as_ref(),
            RunMode::Diagnostic,
            PartialRunPolicy::Allow,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
        );

        assert_eq!(assessment.status, SupportStatus::SupportedCompatibility);
        assert_eq!(
            assessment.run_result_state,
            RunResultState::SupportedCompatibilityRun
        );
        assert_eq!(
            assessment.runtime_class,
            RuntimeClass::IdealLoadsNoOaSensibleCompatibility
        );
        assert_eq!(
            assessment.matched_capability_ids,
            vec!["ideal_loads_no_oa_sensible"]
        );
        assert_eq!(assessment.matched_capabilities[0].domain, "ideal_loads");
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
