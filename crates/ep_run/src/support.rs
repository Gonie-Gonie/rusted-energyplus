//! Support assessment for arbitrary runs.

use ep_compiler::{CompileReport, DiagnosticSeverity, ObjectCoverageStatus};
use ep_model::TypedModel;
use ep_raw_model::RawModel;
use serde::Serialize;

use crate::{
    PartialRunPolicy, RunDiagnostic, RunDiagnosticSeverity, RunDiagnostics, RunMode,
    RunOutputFormat, TraceLevel,
    support_registry::{load_embedded_capability_registry, partial_rule_for_object},
};

pub use crate::support_registry::CAPABILITY_REGISTRY_PATH;

mod runtime_boundaries;

use runtime_boundaries::{
    assess_typed_runtime_boundaries, matched_capabilities, runtime_status_for_typed_model,
    unsupported_object_reason,
};

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
    /// IdealLoads no-OA selected humidity-control compatibility runtime.
    IdealLoadsHumiditySelectedBranchesCompatibility,
    /// Mixed declared IdealLoads PurchasedAir compatibility runtime.
    IdealLoadsMixedDeclaredCompatibility,
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
            Self::IdealLoadsHumiditySelectedBranchesCompatibility => {
                "ideal-loads-humidity-selected-branches-compatibility"
            }
            Self::IdealLoadsMixedDeclaredCompatibility => {
                "ideal-loads-mixed-declared-compatibility"
            }
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
    /// Human-readable reason tying matched capabilities to the selected runtime and run state.
    pub runtime_selection_note: String,
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

fn runtime_selection_note(
    status: SupportStatus,
    runtime_class: RuntimeClass,
    run_result_state: RunResultState,
) -> String {
    match run_result_state {
        RunResultState::SupportedCompatibilityRun => format!(
            "selected runtime '{}' executes the matched capability boundary; conformance_claim remains false for arbitrary runs",
            runtime_class.id()
        ),
        RunResultState::PartialSupportedRun => format!(
            "matched capabilities require diagnostic/ad-hoc execution through '{}'; matched capability metadata does not create a compatibility claim for this run",
            runtime_class.id()
        ),
        RunResultState::RunBlocked if status == SupportStatus::SupportedDiagnosticOnly => format!(
            "diagnostic-only runtime '{}' is blocked unless mode=diagnostic and partial_policy=allow are both selected",
            runtime_class.id()
        ),
        RunResultState::RunBlocked => {
            "support assessment blocked Rust execution before runtime".to_string()
        }
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
    let (mut status, runtime_class, matched_capability_ids, missing_capability_ids) =
        if diagnostics.has_errors() || typed_model.is_none() {
            (
                SupportStatus::Unsupported,
                RuntimeClass::None,
                Vec::new(),
                Vec::new(),
            )
        } else {
            runtime_status_for_typed_model(typed_model, &capability_registry.spec)
        };
    for capability_id in &missing_capability_ids {
        diagnostics.error(
            "CapabilityRegistryCapabilityMissing",
            "support",
            format!(
                "selected runtime capability '{capability_id}' is not declared in {CAPABILITY_REGISTRY_PATH}"
            ),
        );
    }
    if status == SupportStatus::SupportedCompatibility
        && !ignored_raw_only_objects.is_empty()
        && mode == RunMode::Diagnostic
        && partial_policy.allows_partial()
    {
        status = SupportStatus::SupportedDiagnosticOnly;
    }

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
        runtime_selection_note: runtime_selection_note(status, runtime_class, run_result_state),
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

#[cfg(test)]
mod tests;
