use serde::Deserialize;

use super::{
    ConformanceCase, EvidenceDomain, ValidationError, normalize_identity_part,
    require_meter_non_empty, require_non_empty, require_waiver_non_empty, validate_non_negative,
    validate_output_non_negative, validate_unique_outputs,
};

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
    /// Optional timestamp ordering and uniqueness contract for the supported
    /// hourly schedule ESO series.
    pub timestamp_contract: Option<TimestampContract>,
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

    pub(super) fn validate(&self, index: usize) -> Result<(), ValidationError> {
        if self.timestamp_contract.is_some()
            && !(self.frequency == OutputFrequency::Hourly
                && self.source == SourceArtifact::Eso
                && matches!(self.class, VariableClass::Schedule | VariableClass::Weather))
        {
            return Err(ValidationError::InvalidTimestampContractOutput { index });
        }
        Ok(())
    }

    pub(super) fn validate_v2(&self, index: usize) -> Result<(), ValidationError> {
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

/// Timestamp comparison policy applied to one time-series output.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TimestampContract {
    /// Require timestamps to match in order, with no duplicate labels.
    OrderedExactUnique,
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
    pub(super) fn validate(&self, index: usize) -> Result<(), ValidationError> {
        require_meter_non_empty(index, "name", &self.name)
    }

    pub(super) fn validate_v2(&self, index: usize) -> Result<(), ValidationError> {
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
    /// Optional timestamp contract retained from the manifest.
    pub timestamp_contract: Option<TimestampContract>,
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
            timestamp_contract: output.timestamp_contract,
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
    /// Every HVAC/system call sample.
    Detailed,
    /// Every zone timestep.
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
    /// Surface-level heat-transfer coefficient variables with separate tolerance policy.
    SurfaceCoefficientState,
    /// Surface-level heat flux variables with separate tolerance policy.
    SurfaceFluxState,
    /// Surface-level incident solar flux variables with separate tolerance policy.
    SurfaceSolarFluxState,
    /// Surface-level absorbed solar heat-gain rate variables with separate tolerance policy.
    SurfaceSolarRateState,
    /// Surface-level exterior environmental heat-gain rate variables with separate tolerance policy.
    SurfaceExteriorRateState,
    /// Surface-level exterior environmental heat-gain flux variables with separate tolerance policy.
    SurfaceExteriorFluxState,
    /// Zone-level opaque surface aggregate variables with separate tolerance policy.
    SurfaceAggregateState,
    /// Surface-level heat storage variables with separate tolerance policy.
    SurfaceStorageState,
    /// Surface-level heat storage flux variables with separate tolerance policy.
    SurfaceStorageFluxState,
    /// Surface-level iteration count variables with separate tolerance policy.
    SurfaceIterationCountState,
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
    pub(super) fn validate(self, index: usize) -> Result<(), ValidationError> {
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
    pub(super) fn validate(&self) -> Result<(), ValidationError> {
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
    pub(super) fn validate(&self) -> Result<(), ValidationError> {
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
    pub(super) fn validate(&self, index: usize) -> Result<(), ValidationError> {
        require_waiver_non_empty(index, "id", &self.id)?;
        require_waiver_non_empty(index, "reason", &self.reason)?;
        require_waiver_non_empty(index, "owner", &self.owner)?;
        require_waiver_non_empty(index, "expires", &self.expires)
    }
}
