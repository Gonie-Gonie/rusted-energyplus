//! Case and suite manifests for EnergyPlus comparison evidence.

use serde::Deserialize;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::path::Path;

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

        for (index, output) in self.outputs.iter().enumerate() {
            require_output_non_empty(index, "key", &output.key)?;
            require_output_non_empty(index, "variable", &output.variable)?;
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

/// Test taxonomy used by release and comparison reporting.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonClass {
    /// Execution or extraction-only smoke test.
    Smoke,
    /// Diagnostic extraction with no tolerance-enforced conformance claim.
    DiagnosticOnly,
    /// EnergyPlus oracle values are compared against declared tolerances.
    Conformance,
    /// Rust behavior is compared against a Rust baseline.
    Regression,
    /// Runtime, memory, or profiling counters are compared.
    Performance,
}

/// Requested EnergyPlus output variable.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OutputRequest {
    /// EnergyPlus output key such as a zone name or `*`.
    pub key: String,
    /// EnergyPlus output variable name.
    pub variable: String,
    /// Requested reporting frequency.
    pub frequency: OutputFrequency,
    /// Semantic variable group used to select tolerance rules.
    pub class: VariableClass,
    /// EnergyPlus artifact that should be used as the oracle source.
    pub source: SourceArtifact,
    /// v2 domain label used by release coverage matrices.
    pub domain: Option<EvidenceDomain>,
    /// v2 output evidence level.
    pub level: Option<OutputLevel>,
    /// v2 per-output maximum absolute tolerance.
    pub abs_tol: Option<f64>,
    /// v2 per-output maximum RMSE tolerance.
    pub rmse_tol: Option<f64>,
    /// v2 per-output maximum relative tolerance.
    pub rel_tol: Option<f64>,
}

impl OutputRequest {
    /// Returns the normalized key used for duplicate detection.
    #[must_use]
    pub fn normalized_identity(&self) -> OutputRequestIdentity {
        OutputRequestIdentity {
            key: normalize_identity_part(&self.key),
            variable: normalize_identity_part(&self.variable),
            frequency: self.frequency,
            source: self.source,
        }
    }

    fn validate_v2(&self, index: usize) -> Result<(), ValidationError> {
        if self.domain.is_none() {
            return Err(ValidationError::MissingOutputDomain { index });
        }
        if self.level.is_none() {
            return Err(ValidationError::MissingOutputLevel { index });
        }
        validate_output_non_negative(index, "abs_tol", self.abs_tol)?;
        validate_output_non_negative(index, "rmse_tol", self.rmse_tol)?;
        validate_output_non_negative(index, "rel_tol", self.rel_tol)
    }
}

/// v2 evidence level for a requested output or meter.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OutputLevel {
    /// Required for artifact coverage but not necessarily compared.
    Required,
    /// Optional when available.
    Optional,
    /// EnergyPlus oracle baseline only.
    Baseline,
    /// Diagnostic extraction or delta reporting without tolerances.
    Diagnostic,
    /// Tolerance-gated EnergyPlus conformance output.
    Conformance,
}

/// Requested EnergyPlus meter.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MeterRequest {
    /// EnergyPlus meter name.
    pub name: String,
    /// Requested reporting frequency.
    pub frequency: OutputFrequency,
    /// EnergyPlus artifact that should be used as the oracle source.
    pub source: SourceArtifact,
    /// v2 domain label used by release coverage matrices.
    pub domain: EvidenceDomain,
    /// v2 meter evidence level.
    pub level: OutputLevel,
    /// v2 per-meter maximum absolute tolerance.
    pub abs_tol: Option<f64>,
    /// v2 per-meter maximum RMSE tolerance.
    pub rmse_tol: Option<f64>,
    /// v2 per-meter maximum relative tolerance.
    pub rel_tol: Option<f64>,
}

impl MeterRequest {
    fn validate(&self, index: usize) -> Result<(), ValidationError> {
        require_meter_non_empty(index, "name", &self.name)
    }

    fn validate_v2(&self, index: usize) -> Result<(), ValidationError> {
        self.validate(index)?;
        validate_output_non_negative(index, "abs_tol", self.abs_tol)?;
        validate_output_non_negative(index, "rmse_tol", self.rmse_tol)?;
        validate_output_non_negative(index, "rel_tol", self.rel_tol)
    }

    /// Returns the normalized meter identity used for duplicate detection.
    #[must_use]
    pub fn normalized_identity(&self) -> MeterRequestIdentity {
        MeterRequestIdentity {
            name: normalize_identity_part(&self.name),
            frequency: self.frequency,
            source: self.source,
        }
    }
}

/// Stable identity for one requested meter series.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterRequestIdentity {
    /// Normalized meter name.
    pub name: String,
    /// Output reporting frequency.
    pub frequency: OutputFrequency,
    /// Oracle artifact source.
    pub source: SourceArtifact,
}

/// Stable identity for one requested output series.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputRequestIdentity {
    /// Normalized output key.
    pub key: String,
    /// Normalized variable name.
    pub variable: String,
    /// Output reporting frequency.
    pub frequency: OutputFrequency,
    /// Oracle artifact source.
    pub source: SourceArtifact,
}

/// Registry of output series requested by one case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputRegistry {
    series: Vec<OutputSeriesSpec>,
}

impl OutputRegistry {
    /// Builds a registry from validated case output requests.
    pub fn from_case(case: &ConformanceCase) -> Result<Self, ValidationError> {
        validate_unique_outputs(&case.outputs)?;
        Ok(Self {
            series: case
                .outputs
                .iter()
                .cloned()
                .map(OutputSeriesSpec::from)
                .collect(),
        })
    }

    /// Returns every registered output series in manifest order.
    #[must_use]
    pub fn series(&self) -> &[OutputSeriesSpec] {
        &self.series
    }

    /// Returns the number of registered series.
    #[must_use]
    pub fn len(&self) -> usize {
        self.series.len()
    }

    /// Returns true when the registry has no series.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }
}

/// Registered output series specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputSeriesSpec {
    /// EnergyPlus output key such as a zone name or schedule name.
    pub key: String,
    /// EnergyPlus output variable name.
    pub variable: String,
    /// Requested reporting frequency.
    pub frequency: OutputFrequency,
    /// Semantic variable group used to select tolerance rules.
    pub class: VariableClass,
    /// Oracle artifact source.
    pub source: SourceArtifact,
    /// Normalized identity used by comparison reports and gates.
    pub identity: OutputRequestIdentity,
}

impl From<OutputRequest> for OutputSeriesSpec {
    fn from(output: OutputRequest) -> Self {
        let identity = output.normalized_identity();
        Self {
            key: output.key,
            variable: output.variable,
            frequency: output.frequency,
            class: output.class,
            source: output.source,
            identity,
        }
    }
}

/// Supported output reporting frequencies.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFrequency {
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

/// EnergyPlus artifact that contains a requested oracle output.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum SourceArtifact {
    /// EnergyPlus input/output summary file.
    Eio,
    /// EnergyPlus time-series output file.
    Eso,
    /// EnergyPlus meter output file.
    Mtr,
    /// EnergyPlus SQLite output.
    Sql,
    /// Selected CSV extracted from one or more EnergyPlus outputs.
    Csv,
}

/// Semantic variable groups for comparison policies.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum VariableClass {
    /// Schedule values.
    Schedule,
    /// Weather values.
    Weather,
    /// Construction and material static input summaries.
    ConstructionMaterial,
    /// Internal gains and their derived trace values.
    InternalGain,
    /// Zone-level state variables.
    ZoneState,
    /// Surface-level state variables.
    SurfaceState,
    /// Air-side node state variables.
    NodeState,
    /// HVAC control or component state variables.
    HvacState,
    /// Plant loop state variables.
    PlantState,
    /// Plant equipment and demand-side component variables.
    PlantEquipment,
    /// EnergyPlus meters.
    Meter,
    /// EnergyPlus internal variables.
    InternalVariable,
    /// Development-only diagnostics.
    Diagnostic,
}

/// Numeric tolerance rule for a variable class.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ToleranceRule {
    /// Variable group covered by this tolerance.
    pub variable_class: VariableClass,
    /// Maximum absolute difference.
    pub max_abs: Option<f64>,
    /// Maximum root-mean-square error.
    pub max_rmse: Option<f64>,
    /// Maximum relative difference.
    pub max_rel: Option<f64>,
}

impl ToleranceRule {
    fn validate(self, index: usize) -> Result<(), ValidationError> {
        if self.max_abs.is_none() && self.max_rmse.is_none() && self.max_rel.is_none() {
            return Err(ValidationError::EmptyToleranceRule { index });
        }

        validate_non_negative(index, "max_abs", self.max_abs)?;
        validate_non_negative(index, "max_rmse", self.max_rmse)?;
        validate_non_negative(index, "max_rel", self.max_rel)?;

        Ok(())
    }
}

/// Comparison report artifact contract.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReportContract {
    /// Report output format.
    pub format: ReportFormat,
    /// Report path relative to the repository root.
    pub path: String,
}

impl ReportContract {
    fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("report.path", &self.path)
    }
}

/// Supported report output formats.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ReportFormat {
    /// Markdown report.
    Markdown,
    /// JSON report.
    Json,
}

/// Release gate contract for a case.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GateContract {
    /// Script or command that runs the gate.
    pub script: String,
    /// Whether failure blocks a release.
    pub blocking: bool,
}

impl GateContract {
    fn validate(&self) -> Result<(), ValidationError> {
        require_non_empty("gate.script", &self.script)
    }
}

/// Explicit exception for a known gap or temporary gate policy.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Waiver {
    /// Stable waiver identifier.
    pub id: String,
    /// Human-readable reason for the waiver.
    pub reason: String,
    /// Owner expected to remove or renew the waiver.
    pub owner: String,
    /// Expiry marker such as a version, milestone, or date.
    pub expires: String,
}

impl Waiver {
    fn validate(&self, index: usize) -> Result<(), ValidationError> {
        require_waiver_non_empty(index, "id", &self.id)?;
        require_waiver_non_empty(index, "reason", &self.reason)?;
        require_waiver_non_empty(index, "owner", &self.owner)?;
        require_waiver_non_empty(index, "expires", &self.expires)
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

/// Error returned while loading or validating manifests.
#[derive(Debug)]
pub enum ManifestError {
    /// File read failed.
    Io(std::io::Error),
    /// TOML parsing failed.
    Toml(toml::de::Error),
    /// Manifest-level validation failed.
    Validation(ValidationError),
}

impl Display for ManifestError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read manifest: {error}"),
            Self::Toml(error) => write!(formatter, "failed to parse manifest TOML: {error}"),
            Self::Validation(error) => write!(formatter, "invalid manifest: {error}"),
        }
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Toml(error) => Some(error),
            Self::Validation(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for ManifestError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<toml::de::Error> for ManifestError {
    fn from(error: toml::de::Error) -> Self {
        Self::Toml(error)
    }
}

impl From<ValidationError> for ManifestError {
    fn from(error: ValidationError) -> Self {
        Self::Validation(error)
    }
}

/// Validation failure for one manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationError {
    /// A required string field was empty.
    MissingField {
        /// Field path.
        field: &'static str,
    },
    /// v2 metadata table was missing.
    MissingManifestV2,
    /// v2 manifest schema marker was not supported.
    UnsupportedManifestV2Schema {
        /// Actual schema marker.
        schema: String,
    },
    /// v2 scope table was missing.
    MissingScope,
    /// v2 scope table had no domains.
    EmptyScopeDomains,
    /// A true conformance claim appeared outside the conformance class.
    InvalidConformanceClaim {
        /// Actual class in the manifest.
        comparison_class: ComparisonClass,
    },
    /// Conformance class was selected without a true claim.
    ConformanceClassWithoutClaim,
    /// A conformance claim had no output requests.
    MissingOutputRequests,
    /// A conformance claim had no tolerance rules.
    MissingToleranceRules,
    /// A conformance claim had no report contract.
    MissingReport,
    /// A conformance claim had no gate contract.
    MissingGate,
    /// A conformance gate was present but non-blocking.
    NonBlockingConformanceGate,
    /// An output request had an empty field.
    EmptyOutputField {
        /// Zero-based output request index.
        index: usize,
        /// Field name inside the output request.
        field: &'static str,
    },
    /// v2 output request had no domain.
    MissingOutputDomain {
        /// Zero-based output request index.
        index: usize,
    },
    /// v2 output request had no evidence level.
    MissingOutputLevel {
        /// Zero-based output request index.
        index: usize,
    },
    /// v2 output requested conformance level without a conformance claim.
    ConformanceOutputWithoutClaim {
        /// Zero-based output request index.
        index: usize,
    },
    /// v2 meter requested conformance level without a conformance claim.
    ConformanceMeterWithoutClaim {
        /// Zero-based meter request index.
        index: usize,
    },
    /// A true conformance claim had no conformance-level output or meter.
    MissingConformanceOutputLevel,
    /// A v2 output or meter tolerance threshold was negative.
    NegativeOutputTolerance {
        /// Zero-based request index.
        index: usize,
        /// Field name inside the output or meter request.
        field: &'static str,
    },
    /// A meter request had an empty field.
    EmptyMeterField {
        /// Zero-based meter request index.
        index: usize,
        /// Field name inside the meter request.
        field: &'static str,
    },
    /// Two meter requests resolve to the same identity.
    DuplicateMeterRequest {
        /// Zero-based meter request index where the duplicate was found.
        index: usize,
        /// Normalized meter name.
        name: String,
    },
    /// A waiver had an empty field.
    EmptyWaiverField {
        /// Zero-based waiver index.
        index: usize,
        /// Field name inside the waiver.
        field: &'static str,
    },
    /// Two output requests resolve to the same identity.
    DuplicateOutputRequest {
        /// Zero-based output request index where the duplicate was found.
        index: usize,
        /// Normalized output key.
        key: String,
        /// Normalized variable name.
        variable: String,
    },
    /// A tolerance rule had no threshold.
    EmptyToleranceRule {
        /// Zero-based tolerance rule index.
        index: usize,
    },
    /// A tolerance threshold was negative.
    NegativeTolerance {
        /// Zero-based tolerance rule index.
        index: usize,
        /// Field name inside the tolerance rule.
        field: &'static str,
    },
    /// A suite manifest had no cases.
    MissingSuiteCases,
    /// A suite case path was empty.
    EmptySuiteCase {
        /// Zero-based suite case index.
        index: usize,
    },
}

impl Display for ValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField { field } => write!(formatter, "missing required field {field}"),
            Self::MissingManifestV2 => write!(formatter, "missing required table manifest_v2"),
            Self::UnsupportedManifestV2Schema { schema } => {
                write!(formatter, "unsupported manifest_v2.schema {schema}")
            }
            Self::MissingScope => write!(formatter, "missing required table scope"),
            Self::EmptyScopeDomains => write!(formatter, "scope.domains must not be empty"),
            Self::InvalidConformanceClaim { comparison_class } => write!(
                formatter,
                "conformance_claim=true is not allowed for {comparison_class:?}"
            ),
            Self::ConformanceClassWithoutClaim => {
                write!(
                    formatter,
                    "comparison_class=conformance requires conformance_claim=true"
                )
            }
            Self::MissingOutputRequests => write!(formatter, "conformance claim has no outputs"),
            Self::MissingToleranceRules => write!(formatter, "conformance claim has no tolerances"),
            Self::MissingReport => write!(formatter, "conformance claim has no report contract"),
            Self::MissingGate => write!(formatter, "conformance claim has no gate contract"),
            Self::NonBlockingConformanceGate => {
                write!(formatter, "conformance claim requires a blocking gate")
            }
            Self::EmptyOutputField { index, field } => {
                write!(formatter, "output {index} has empty field {field}")
            }
            Self::MissingOutputDomain { index } => {
                write!(formatter, "output {index} is missing v2 domain")
            }
            Self::MissingOutputLevel { index } => {
                write!(formatter, "output {index} is missing v2 level")
            }
            Self::ConformanceOutputWithoutClaim { index } => write!(
                formatter,
                "output {index} has level=conformance without a conformance claim"
            ),
            Self::ConformanceMeterWithoutClaim { index } => write!(
                formatter,
                "meter {index} has level=conformance without a conformance claim"
            ),
            Self::MissingConformanceOutputLevel => write!(
                formatter,
                "conformance claim requires at least one output or meter with level=conformance"
            ),
            Self::NegativeOutputTolerance { index, field } => {
                write!(formatter, "request {index} field {field} is negative")
            }
            Self::EmptyMeterField { index, field } => {
                write!(formatter, "meter {index} has empty field {field}")
            }
            Self::DuplicateMeterRequest { index, name } => {
                write!(formatter, "meter {index} duplicates requested meter {name}")
            }
            Self::EmptyWaiverField { index, field } => {
                write!(formatter, "waiver {index} has empty field {field}")
            }
            Self::DuplicateOutputRequest {
                index,
                key,
                variable,
            } => write!(
                formatter,
                "output {index} duplicates requested series {key}/{variable}"
            ),
            Self::EmptyToleranceRule { index } => {
                write!(formatter, "tolerance {index} has no threshold")
            }
            Self::NegativeTolerance { index, field } => {
                write!(formatter, "tolerance {index} field {field} is negative")
            }
            Self::MissingSuiteCases => write!(formatter, "suite has no cases"),
            Self::EmptySuiteCase { index } => write!(formatter, "suite case {index} is empty"),
        }
    }
}

impl std::error::Error for ValidationError {}

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
