//! Case and suite manifests for EnergyPlus comparison evidence.

use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;

mod errors;
mod output_contract;

pub use errors::*;
pub use output_contract::*;

/// Canonical schema marker for v0.17 Case Manifest v2 documents.
pub const CASE_MANIFEST_V2_SCHEMA: &str = "rusted-energyplus.case-manifest.v2";

/// Top-level manifest for one EnergyPlus comparison case.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConformanceCase {
    /// Stable case identifier.
    pub id: String,
    /// Human-readable case title.
    pub title: String,
    /// Milestone or backlog bucket that owns the case.
    pub milestone: String,
    /// Short explanation of what the case is meant to prove.
    pub purpose: String,
    /// Taxonomy class controlling exit-code and reporting semantics.
    pub comparison_class: ComparisonClass,
    /// Whether this case is allowed to claim EnergyPlus numerical conformance.
    pub conformance_claim: bool,
    /// EnergyPlus oracle version used to generate baselines.
    pub oracle_version: String,
    /// v0.17 Case Manifest v2 metadata.
    pub manifest_v2: Option<ManifestV2Metadata>,
    /// Optional trace/report verbosity contract for generated evidence.
    pub trace: Option<TraceContract>,
    /// Domain and feature flags used by ExampleFiles coverage planning.
    pub scope: Option<CaseScope>,
    /// Input files used by the oracle and Rust implementation.
    pub input: CaseInput,
    /// Fixed dynamic-case boundary used to keep candidate promotion scoped.
    pub boundary: Option<CaseBoundary>,
    /// Requested output variables that define the evidence surface.
    #[serde(default)]
    pub outputs: Vec<OutputRequest>,
    /// Requested meters that define the evidence surface.
    #[serde(default)]
    pub meters: Vec<MeterRequest>,
    /// Tolerance rules used only by tolerance-gated conformance cases.
    #[serde(default)]
    pub tolerances: Vec<ToleranceRule>,
    /// Explicit waivers for known gaps or temporary gate exceptions.
    #[serde(default)]
    pub waivers: Vec<Waiver>,
    /// Report artifact contract for generated comparison evidence.
    pub report: Option<ReportContract>,
    /// Gate command that decides whether the case blocks a release.
    pub gate: Option<GateContract>,
    /// Free-form implementation notes.
    #[serde(default)]
    pub notes: Vec<String>,
}

impl ConformanceCase {
    /// Validates the manifest against the no-false-conformance contract.
    ///
    /// A manifest can describe smoke or diagnostic extraction without
    /// tolerances, but a true conformance claim requires a conformance class,
    /// output requests, tolerances, report contract, and release gate.
    pub fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("id", &self.id)?;
        require_non_empty("title", &self.title)?;
        require_non_empty("milestone", &self.milestone)?;
        require_non_empty("purpose", &self.purpose)?;
        require_non_empty("oracle_version", &self.oracle_version)?;
        require_non_empty("input.idf", &self.input.idf)?;
        if let Some(weather) = self.input.weather.as_deref() {
            require_non_empty("input.weather", weather)?;
        }
        if let Some(epjson) = self.input.epjson.as_deref() {
            require_non_empty("input.epjson", epjson)?;
        }
        if let Some(boundary) = self.boundary.as_ref() {
            boundary.validate()?;
        }
        if let Some(trace) = self.trace.as_ref() {
            trace.validate()?;
        }

        for (index, output) in self.outputs.iter().enumerate() {
            require_output_non_empty(index, "key", &output.key)?;
            require_output_non_empty(index, "variable", &output.variable)?;
            output.validate(index)?;
        }
        validate_unique_outputs(&self.outputs)?;

        for (index, meter) in self.meters.iter().enumerate() {
            meter.validate(index)?;
        }
        validate_unique_meters(&self.meters)?;

        for (index, tolerance) in self.tolerances.iter().enumerate() {
            tolerance.validate(index)?;
        }

        if self.comparison_class == ComparisonClass::Conformance && !self.conformance_claim {
            return Err(ValidationError::ConformanceClassWithoutClaim);
        }

        if !self.conformance_claim {
            return Ok(());
        }

        if self.comparison_class != ComparisonClass::Conformance {
            return Err(ValidationError::InvalidConformanceClaim {
                comparison_class: self.comparison_class,
            });
        }
        if self.outputs.is_empty() {
            return Err(ValidationError::MissingOutputRequests);
        }
        if self.tolerances.is_empty() {
            return Err(ValidationError::MissingToleranceRules);
        }

        let Some(report) = self.report.as_ref() else {
            return Err(ValidationError::MissingReport);
        };
        report.validate()?;

        let Some(gate) = self.gate.as_ref() else {
            return Err(ValidationError::MissingGate);
        };
        gate.validate()?;
        if !gate.blocking {
            return Err(ValidationError::NonBlockingConformanceGate);
        }

        Ok(())
    }

    /// Validates the v0.17 Case Manifest and Output Request Schema v2 contract.
    ///
    /// This is intentionally stricter than `validate` and is used by the
    /// Road-to-v1 release gate. It keeps old manifests readable while allowing
    /// the v2 gate to require source/tier/scope and per-output evidence levels.
    pub fn validate_v2(&self) -> Result<(), ValidationError> {
        self.validate()?;

        let Some(metadata) = self.manifest_v2.as_ref() else {
            return Err(ValidationError::MissingManifestV2);
        };
        metadata.validate()?;

        let Some(scope) = self.scope.as_ref() else {
            return Err(ValidationError::MissingScope);
        };
        scope.validate()?;

        for (index, output) in self.outputs.iter().enumerate() {
            output.validate_v2(index)?;
            if output.level == Some(OutputLevel::Conformance)
                && (!self.conformance_claim
                    || self.comparison_class != ComparisonClass::Conformance)
            {
                return Err(ValidationError::ConformanceOutputWithoutClaim { index });
            }
        }

        for (index, meter) in self.meters.iter().enumerate() {
            meter.validate_v2(index)?;
            if meter.level == OutputLevel::Conformance
                && (!self.conformance_claim
                    || self.comparison_class != ComparisonClass::Conformance)
            {
                return Err(ValidationError::ConformanceMeterWithoutClaim { index });
            }
        }

        for (index, waiver) in self.waivers.iter().enumerate() {
            waiver.validate(index)?;
        }

        if self.conformance_claim
            && !self
                .outputs
                .iter()
                .any(|output| output.level == Some(OutputLevel::Conformance))
            && !self
                .meters
                .iter()
                .any(|meter| meter.level == OutputLevel::Conformance)
        {
            return Err(ValidationError::MissingConformanceOutputLevel);
        }

        Ok(())
    }
}

/// Optional trace/report verbosity selected by one case manifest.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TraceContract {
    /// Stable trace level label consumed by case-specific report generators.
    pub level: String,
}

impl TraceContract {
    fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("trace.level", &self.level)
    }
}

/// v0.17 metadata that makes source, tier, and schema version explicit.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ManifestV2Metadata {
    /// Schema marker used by validation gates and migration scripts.
    pub schema: String,
    /// Kind of source input that owns this case.
    pub source_kind: CaseSourceKind,
    /// Source IDF or epJSON file path before any output-request patching.
    pub source_file: String,
    /// Case tier used by release and CI policy.
    pub tier: CaseTier,
}

impl ManifestV2Metadata {
    fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("manifest_v2.schema", &self.schema)?;
        if self.schema != CASE_MANIFEST_V2_SCHEMA {
            return Err(ValidationError::UnsupportedManifestV2Schema {
                schema: self.schema.clone(),
            });
        }
        require_non_empty("manifest_v2.source_file", &self.source_file)
    }
}

/// Source family for a conformance or diagnostic case.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CaseSourceKind {
    /// Repository-local reduced fixture.
    LocalFixture,
    /// Official EnergyPlus ExampleFiles input.
    EnergyPlusExamplefile,
    /// Official EnergyPlus testfile input.
    EnergyPlusTestfile,
    /// Minimal epJSON fixture without an IDF source.
    MinimalEpjson,
}

/// Release tier for a case.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum CaseTier {
    /// Small deterministic release-gate candidate.
    #[serde(rename = "A")]
    A,
    /// Scheduled diagnostic or broader coverage case.
    #[serde(rename = "B")]
    B,
    /// Complex coverage exploration case.
    #[serde(rename = "C")]
    C,
}

/// Domain and feature flags for case coverage reports.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaseScope {
    /// Domains intentionally touched by this case.
    pub domains: Vec<EvidenceDomain>,
    /// Whether the case includes zone objects.
    pub has_zone: bool,
    /// Whether the case includes surface objects.
    pub has_surface: bool,
    /// Whether the case includes fenestration objects.
    pub has_fenestration: bool,
    /// Whether the case includes an air loop.
    pub has_air_loop: bool,
    /// Whether the case includes a plant loop.
    pub has_plant_loop: bool,
    /// Whether the case includes EMS.
    pub has_ems: bool,
    /// Whether the case includes PythonPlugin objects.
    pub has_python_plugin: bool,
    /// Whether the case includes daylighting objects.
    pub has_daylighting: bool,
}

impl CaseScope {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.domains.is_empty() {
            return Err(ValidationError::EmptyScopeDomains);
        }
        Ok(())
    }
}

/// High-level evidence domain.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceDomain {
    /// Weather input or weather output variables.
    Weather,
    /// Schedule input or schedule output variables.
    Schedule,
    /// Zone state or zone heat balance values.
    Zone,
    /// Surface geometry or heat balance values.
    Surface,
    /// Construction or material static data.
    Construction,
    /// Internal gains and related heat-gain splits.
    InternalGain,
    /// Air-side node state values.
    Node,
    /// HVAC component or control values.
    Hvac,
    /// Plant loop or plant equipment values.
    Plant,
    /// EnergyPlus meters.
    Meter,
    /// Development diagnostics.
    Diagnostic,
}

/// Input file contract for one case.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaseInput {
    /// IDF path relative to the case directory or repository root.
    pub idf: String,
    /// Optional weather path used by the EnergyPlus oracle run.
    pub weather: Option<String>,
    /// Optional epJSON path produced from the IDF.
    pub epjson: Option<String>,
}

/// Explicit boundary for a dynamic candidate or diagnostic case.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaseBoundary {
    /// Stable case id whose outputs and gates own this boundary.
    pub target_case_id: String,
    /// Human-readable EnergyPlus source IDF contract.
    pub source_idf: String,
    /// Human-readable EnergyPlus weather contract.
    pub weather_file: String,
    /// RunPeriod used for the compared output series.
    pub run_period: CaseRunPeriod,
    /// Zone timesteps per hour from the input object.
    pub timesteps_per_hour: u32,
    /// Reporting frequency used by the compared dynamic outputs.
    pub reporting_frequency: OutputFrequency,
    /// Warmup-output inclusion policy for the comparison.
    pub warmup_output: WarmupOutputPolicy,
    /// Declared surface keys used by named-key comparisons.
    pub declared_surface_keys: DeclaredSurfaceKeys,
}

impl CaseBoundary {
    fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("boundary.target_case_id", &self.target_case_id)?;
        require_non_empty("boundary.source_idf", &self.source_idf)?;
        require_non_empty("boundary.weather_file", &self.weather_file)?;
        self.run_period.validate()?;
        self.declared_surface_keys.validate()
    }
}

/// RunPeriod identity and date range for a dynamic case boundary.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaseRunPeriod {
    /// EnergyPlus RunPeriod name.
    pub name: String,
    /// Inclusive begin month.
    pub begin_month: u32,
    /// Inclusive begin day of month.
    pub begin_day: u32,
    /// Inclusive end month.
    pub end_month: u32,
    /// Inclusive end day of month.
    pub end_day: u32,
    /// Start day-of-week label from the IDF.
    pub start_day_of_week: String,
}

impl CaseRunPeriod {
    fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("boundary.run_period.name", &self.name)?;
        require_non_empty(
            "boundary.run_period.start_day_of_week",
            &self.start_day_of_week,
        )
    }
}

/// Whether warmup samples are included in the compared output stream.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WarmupOutputPolicy {
    /// Compare only run-period outputs while preserving warmup as diagnostic trace.
    RunPeriodOnlyWithDiagnosticTrace,
}

/// Surface keys that must stay stable for named surface comparisons.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeclaredSurfaceKeys {
    /// Roof surface keys.
    pub roof: Vec<String>,
    /// Wall surface keys.
    pub wall: Vec<String>,
    /// Floor surface keys.
    pub floor: Vec<String>,
    /// Whether wildcard `*` request expansion is part of the diagnostic.
    pub wildcard_comparison: bool,
    /// Whether named-key comparison is part of the diagnostic.
    pub named_key_comparison: bool,
    /// Whether reports sort surfaces by top RMSE.
    pub top_rmse_sorted: bool,
}

impl DeclaredSurfaceKeys {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.roof.is_empty() {
            return Err(ValidationError::MissingField {
                field: "boundary.declared_surface_keys.roof",
            });
        }
        if self.wall.is_empty() {
            return Err(ValidationError::MissingField {
                field: "boundary.declared_surface_keys.wall",
            });
        }
        if self.floor.is_empty() {
            return Err(ValidationError::MissingField {
                field: "boundary.declared_surface_keys.floor",
            });
        }
        for key in self
            .roof
            .iter()
            .chain(self.wall.iter())
            .chain(self.floor.iter())
        {
            require_non_empty("boundary.declared_surface_keys", key)?;
        }
        Ok(())
    }
}

/// Top-level manifest for a named suite of cases.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConformanceSuite {
    /// Stable suite identifier.
    pub id: String,
    /// Human-readable suite title.
    pub title: String,
    /// EnergyPlus oracle version expected by the suite.
    pub oracle_version: String,
    /// Case manifest paths included in suite order.
    pub cases: Vec<String>,
}

impl ConformanceSuite {
    /// Validates suite identity and referenced case list shape.
    pub fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("id", &self.id)?;
        require_non_empty("title", &self.title)?;
        require_non_empty("oracle_version", &self.oracle_version)?;
        if self.cases.is_empty() {
            return Err(ValidationError::MissingSuiteCases);
        }
        for (index, case) in self.cases.iter().enumerate() {
            if case.trim().is_empty() {
                return Err(ValidationError::EmptySuiteCase { index });
            }
        }
        Ok(())
    }
}

/// Loads and validates one case manifest from a TOML file.
pub fn load_case_file(path: impl AsRef<Path>) -> Result<ConformanceCase, ManifestError> {
    let contents = std::fs::read_to_string(path)?;
    parse_case_str(&contents)
}

/// Loads and validates one v2 case manifest from a TOML file.
pub fn load_case_v2_file(path: impl AsRef<Path>) -> Result<ConformanceCase, ManifestError> {
    let contents = std::fs::read_to_string(path)?;
    parse_case_v2_str(&contents)
}

/// Parses and validates one case manifest from TOML text.
pub fn parse_case_str(contents: &str) -> Result<ConformanceCase, ManifestError> {
    let manifest: ConformanceCase = toml::from_str(contents)?;
    manifest.validate()?;
    Ok(manifest)
}

/// Parses and validates one v2 case manifest from TOML text.
pub fn parse_case_v2_str(contents: &str) -> Result<ConformanceCase, ManifestError> {
    let manifest: ConformanceCase = toml::from_str(contents)?;
    manifest.validate_v2()?;
    Ok(manifest)
}

/// Loads and validates one suite manifest from a TOML file.
pub fn load_suite_file(path: impl AsRef<Path>) -> Result<ConformanceSuite, ManifestError> {
    let contents = std::fs::read_to_string(path)?;
    parse_suite_str(&contents)
}

/// Parses and validates one suite manifest from TOML text.
pub fn parse_suite_str(contents: &str) -> Result<ConformanceSuite, ManifestError> {
    let manifest: ConformanceSuite = toml::from_str(contents)?;
    manifest.validate()?;
    Ok(manifest)
}

fn require_non_empty(field: &'static str, value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::MissingField { field });
    }
    Ok(())
}

fn require_output_non_empty(
    index: usize,
    field: &'static str,
    value: &str,
) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::EmptyOutputField { index, field });
    }
    Ok(())
}

fn require_meter_non_empty(
    index: usize,
    field: &'static str,
    value: &str,
) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::EmptyMeterField { index, field });
    }
    Ok(())
}

fn require_waiver_non_empty(
    index: usize,
    field: &'static str,
    value: &str,
) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::EmptyWaiverField { index, field });
    }
    Ok(())
}

fn validate_non_negative(
    index: usize,
    field: &'static str,
    value: Option<f64>,
) -> Result<(), ValidationError> {
    if value.is_some_and(|number| number < 0.0) {
        return Err(ValidationError::NegativeTolerance { index, field });
    }
    Ok(())
}

fn validate_output_non_negative(
    index: usize,
    field: &'static str,
    value: Option<f64>,
) -> Result<(), ValidationError> {
    if value.is_some_and(|number| number < 0.0) {
        return Err(ValidationError::NegativeOutputTolerance { index, field });
    }
    Ok(())
}

fn validate_unique_outputs(outputs: &[OutputRequest]) -> Result<(), ValidationError> {
    let mut identities = BTreeSet::new();
    for (index, output) in outputs.iter().enumerate() {
        let identity = output.normalized_identity();
        if !identities.insert(identity.clone()) {
            return Err(ValidationError::DuplicateOutputRequest {
                index,
                key: identity.key,
                variable: identity.variable,
            });
        }
    }
    Ok(())
}

fn validate_unique_meters(meters: &[MeterRequest]) -> Result<(), ValidationError> {
    let mut identities = BTreeSet::new();
    for (index, meter) in meters.iter().enumerate() {
        let identity = meter.normalized_identity();
        if !identities.insert(identity.clone()) {
            return Err(ValidationError::DuplicateMeterRequest {
                index,
                name: identity.name,
            });
        }
    }
    Ok(())
}

fn normalize_identity_part(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}
