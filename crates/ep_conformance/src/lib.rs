//! Case and suite manifests for EnergyPlus comparison evidence.

use serde::Deserialize;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::path::Path;

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
    /// Input files used by the oracle and Rust implementation.
    pub input: CaseInput,
    /// Requested output variables that define the evidence surface.
    #[serde(default)]
    pub outputs: Vec<OutputRequest>,
    /// Tolerance rules used only by tolerance-gated conformance cases.
    #[serde(default)]
    pub tolerances: Vec<ToleranceRule>,
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

        for (index, output) in self.outputs.iter().enumerate() {
            require_output_non_empty(index, "key", &output.key)?;
            require_output_non_empty(index, "variable", &output.variable)?;
        }
        validate_unique_outputs(&self.outputs)?;

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
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
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

/// Parses and validates one case manifest from TOML text.
pub fn parse_case_str(contents: &str) -> Result<ConformanceCase, ManifestError> {
    let manifest: ConformanceCase = toml::from_str(contents)?;
    manifest.validate()?;
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

fn normalize_identity_part(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::{
        ComparisonClass, ManifestError, OutputFrequency, OutputRegistry, SourceArtifact,
        ValidationError, VariableClass, load_case_file, load_suite_file, parse_case_str,
    };
    use std::path::PathBuf;

    #[test]
    fn loads_foundation_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/schedule_constant_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "schedule_constant_001");
        assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
        assert!(!manifest.conformance_claim);
        assert_eq!(manifest.outputs.len(), 1);
        assert_eq!(manifest.outputs[0].key, "ALWAYSON");
        assert_eq!(manifest.outputs[0].variable, "Schedule Value");
        assert_eq!(manifest.outputs[0].source, SourceArtifact::Eso);

        Ok(())
    }

    #[test]
    fn loads_zone_temperature_diagnostic_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/zone_temperature_diagnostic_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "zone_temperature_diagnostic_001");
        assert_eq!(manifest.comparison_class, ComparisonClass::DiagnosticOnly);
        assert!(!manifest.conformance_claim);
        assert!(manifest.tolerances.is_empty());
        assert_eq!(manifest.outputs.len(), 1);
        assert_eq!(manifest.outputs[0].key, "ZONE ONE");
        assert_eq!(manifest.outputs[0].variable, "Zone Mean Air Temperature");
        assert_eq!(manifest.outputs[0].frequency, OutputFrequency::Hourly);
        assert_eq!(manifest.outputs[0].class, VariableClass::ZoneState);
        let report = manifest.report.as_ref().ok_or_else(|| {
            std::io::Error::other("zone diagnostic case should declare diagnostic report path")
        })?;
        assert!(report.path.ends_with("compare-report.md"));
        let gate = manifest.gate.as_ref().ok_or_else(|| {
            std::io::Error::other("zone diagnostic case should declare diagnostic gate")
        })?;
        assert!(!gate.blocking);

        Ok(())
    }

    #[test]
    fn loads_heat_balance_nomass_conformance_case_fixture() -> Result<(), Box<dyn std::error::Error>>
    {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/heat_balance_nomass_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "heat_balance_nomass_001");
        assert_eq!(manifest.milestone, "v0.8-heat-balance");
        assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
        assert!(manifest.conformance_claim);
        assert_eq!(manifest.outputs.len(), 1);
        assert_eq!(manifest.outputs[0].key, "ZONE ONE");
        assert_eq!(manifest.outputs[0].variable, "Zone Mean Air Temperature");
        assert_eq!(manifest.outputs[0].frequency, OutputFrequency::Hourly);
        assert_eq!(manifest.outputs[0].class, VariableClass::ZoneState);
        assert_eq!(manifest.outputs[0].source, SourceArtifact::Eso);
        assert_eq!(manifest.tolerances.len(), 1);
        assert_eq!(
            manifest.tolerances[0].variable_class,
            VariableClass::ZoneState
        );
        assert_eq!(manifest.tolerances[0].max_abs, Some(0.000001));
        assert_eq!(manifest.tolerances[0].max_rmse, Some(0.000001));
        let gate = manifest.gate.as_ref().ok_or_else(|| {
            std::io::Error::other("heat-balance conformance should declare a gate")
        })?;
        assert!(gate.blocking);

        Ok(())
    }

    #[test]
    fn loads_surface_temperature_nomass_conformance_case_fixture()
    -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/surface_temperature_nomass_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "surface_temperature_nomass_001");
        assert_eq!(manifest.milestone, "v0.9-surface-temperature");
        assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);
        assert!(manifest.conformance_claim);
        assert_eq!(manifest.outputs.len(), 3);
        assert!(manifest.outputs.iter().any(|output| {
            output.key == "FLOOR"
                && output.variable == "Surface Inside Face Temperature"
                && output.class == VariableClass::SurfaceState
        }));
        assert!(manifest.outputs.iter().any(|output| {
            output.key == "FLOOR"
                && output.variable == "Surface Outside Face Temperature"
                && output.class == VariableClass::SurfaceState
        }));
        assert_eq!(manifest.tolerances.len(), 2);
        assert!(manifest.tolerances.iter().any(|tolerance| {
            tolerance.variable_class == VariableClass::ZoneState
                && tolerance.max_abs == Some(0.000001)
                && tolerance.max_rmse == Some(0.000001)
        }));
        assert!(manifest.tolerances.iter().any(|tolerance| {
            tolerance.variable_class == VariableClass::SurfaceState
                && tolerance.max_abs == Some(0.000001)
                && tolerance.max_rmse == Some(0.000001)
        }));
        let gate = manifest.gate.as_ref().ok_or_else(|| {
            std::io::Error::other("surface-temperature conformance should declare a gate")
        })?;
        assert!(gate.blocking);

        Ok(())
    }

    #[test]
    fn loads_weather_fields_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/weather_fields_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "weather_fields_001");
        assert_eq!(manifest.milestone, "v0.4-time-weather-schedule");
        assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
        assert!(!manifest.conformance_claim);
        assert_eq!(manifest.outputs.len(), 6);
        assert!(manifest.outputs.iter().all(|output| {
            output.key == "Environment"
                && output.frequency == OutputFrequency::Hourly
                && output.class == VariableClass::Weather
                && output.source == SourceArtifact::Eso
        }));
        assert!(
            manifest
                .outputs
                .iter()
                .any(|output| output.variable == "Site Wind Direction")
        );

        Ok(())
    }

    #[test]
    fn loads_surface_geometry_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/surface_geometry_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "surface_geometry_001");
        assert_eq!(manifest.milestone, "v0.5-geometry-internal-variables");
        assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
        assert!(!manifest.conformance_claim);
        assert_eq!(manifest.outputs.len(), 5);
        assert!(manifest.outputs.iter().all(|output| {
            output.key == "*"
                && output.frequency == OutputFrequency::Static
                && output.class == VariableClass::SurfaceState
                && output.source == SourceArtifact::Eio
        }));
        assert!(
            manifest
                .outputs
                .iter()
                .any(|output| { output.variable == "HeatTransfer Surface Azimuth" })
        );

        Ok(())
    }

    #[test]
    fn loads_construction_materials_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/construction_materials_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "construction_materials_001");
        assert_eq!(manifest.milestone, "v0.5-geometry-internal-variables");
        assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
        assert!(!manifest.conformance_claim);
        assert_eq!(manifest.outputs.len(), 7);
        assert!(manifest.outputs.iter().all(|output| {
            output.key == "*"
                && output.frequency == OutputFrequency::Static
                && output.class == VariableClass::ConstructionMaterial
                && output.source == SourceArtifact::Eio
        }));
        assert!(
            manifest
                .outputs
                .iter()
                .any(|output| { output.variable == "Material CTF Summary Thermal Resistance" })
        );

        Ok(())
    }

    #[test]
    fn loads_internal_gains_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/internal_gains_001/case.toml"),
        )?;

        assert_eq!(manifest.id, "internal_gains_001");
        assert_eq!(manifest.milestone, "v0.5-geometry-internal-variables");
        assert_eq!(manifest.comparison_class, ComparisonClass::Smoke);
        assert!(!manifest.conformance_claim);
        assert_eq!(manifest.outputs.len(), 8);
        assert!(manifest.outputs.iter().all(|output| {
            output.class == VariableClass::InternalGain
                && (output.source == SourceArtifact::Eio || output.source == SourceArtifact::Eso)
        }));
        assert!(manifest.outputs.iter().any(|output| {
            output.variable == "Zone Total Internal Convective Heating Rate"
                && output.frequency == OutputFrequency::Hourly
                && output.source == SourceArtifact::Eso
        }));

        Ok(())
    }

    #[test]
    fn loads_foundation_suite_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest =
            load_suite_file(repo_root().join("data/conformance_suites/foundation.toml"))?;

        assert_eq!(manifest.id, "foundation");
        assert_eq!(manifest.oracle_version, "26.1.0");
        assert_eq!(manifest.cases.len(), 6);
        assert!(
            manifest
                .cases
                .iter()
                .any(|case| case.ends_with("data/conformance_cases/weather_fields_001/case.toml"))
        );
        assert!(manifest.cases.iter().any(|case| {
            case.ends_with("data/conformance_cases/surface_geometry_001/case.toml")
        }));
        assert!(manifest.cases.iter().any(|case| {
            case.ends_with("data/conformance_cases/construction_materials_001/case.toml")
        }));
        assert!(
            manifest.cases.iter().any(|case| {
                case.ends_with("data/conformance_cases/internal_gains_001/case.toml")
            })
        );
        assert!(manifest.cases.iter().any(|case| {
            case.ends_with("data/conformance_cases/zone_temperature_diagnostic_001/case.toml")
        }));

        Ok(())
    }

    #[test]
    fn rejects_diagnostic_case_with_true_conformance_claim() {
        let result = parse_case_str(
            r#"
id = "bad_diagnostic_claim"
title = "Bad diagnostic claim"
milestone = "P1"
purpose = "Prove validation blocks false claims."
comparison_class = "diagnostic-only"
conformance_claim = true
oracle_version = "26.1.0"

[input]
idf = "bad.idf"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"
"#,
        );

        assert!(matches!(
            result,
            Err(ManifestError::Validation(
                ValidationError::InvalidConformanceClaim { .. }
            ))
        ));
    }

    #[test]
    fn accepts_conformance_claim_with_full_evidence_contract()
    -> Result<(), Box<dyn std::error::Error>> {
        let manifest = parse_case_str(
            r#"
id = "schedule_constant_claim"
title = "Schedule constant claim"
milestone = "P1"
purpose = "Exercise the full conformance evidence contract."
comparison_class = "conformance"
conformance_claim = true
oracle_version = "26.1.0"

[input]
idf = "schedule_constant.idf"
weather = "weather.epw"

[[outputs]]
key = "*"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"

[[tolerances]]
variable_class = "schedule"
max_abs = 0.0

[report]
format = "markdown"
path = ".runtime/conformance/schedule_constant_claim/compare-report.md"

[gate]
script = "scripts/dev.cmd conformance-schema-smoke"
blocking = true
"#,
        )?;

        assert!(manifest.conformance_claim);
        assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);

        Ok(())
    }

    #[test]
    fn builds_output_registry_from_case_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest = load_case_file(
            repo_root().join("data/conformance_cases/schedule_constant_001/case.toml"),
        )?;
        let registry = OutputRegistry::from_case(&manifest)?;

        assert_eq!(registry.len(), 1);
        assert_eq!(registry.series()[0].identity.key, "ALWAYSON");
        assert_eq!(registry.series()[0].identity.variable, "SCHEDULE VALUE");

        Ok(())
    }

    #[test]
    fn rejects_duplicate_output_requests() {
        let result = parse_case_str(
            r#"
id = "duplicate_outputs"
title = "Duplicate outputs"
milestone = "P1"
purpose = "Prove output registry rejects duplicates."
comparison_class = "smoke"
conformance_claim = false
oracle_version = "26.1.0"

[input]
idf = "duplicate.idf"

[[outputs]]
key = "AlwaysOn"
variable = "Schedule Value"
frequency = "hourly"
class = "schedule"
source = "eso"

[[outputs]]
key = " ALWAYSON "
variable = "schedule value"
frequency = "hourly"
class = "schedule"
source = "eso"
"#,
        );

        assert!(matches!(
            result,
            Err(ManifestError::Validation(
                ValidationError::DuplicateOutputRequest { .. }
            ))
        ));
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
    }
}
