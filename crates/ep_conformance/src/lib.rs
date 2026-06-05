//! Case and suite manifests for EnergyPlus comparison evidence.

use serde::Deserialize;
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
}

/// Supported output reporting frequencies.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFrequency {
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

/// Semantic variable groups for comparison policies.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum VariableClass {
    /// Schedule values.
    Schedule,
    /// Weather values.
    Weather,
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

#[cfg(test)]
mod tests {
    use super::{
        ComparisonClass, ManifestError, ValidationError, load_case_file, load_suite_file,
        parse_case_str,
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
        assert_eq!(manifest.outputs[0].variable, "Schedule Value");

        Ok(())
    }

    #[test]
    fn loads_foundation_suite_fixture() -> Result<(), Box<dyn std::error::Error>> {
        let manifest =
            load_suite_file(repo_root().join("data/conformance_suites/foundation.toml"))?;

        assert_eq!(manifest.id, "foundation");
        assert_eq!(manifest.oracle_version, "26.1.0");
        assert_eq!(manifest.cases.len(), 1);

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

[[tolerances]]
variable_class = "schedule"
max_abs = 0.0

[report]
format = "markdown"
path = ".runtime/conformance/schedule_constant_claim/compare-report.md"

[gate]
script = "scripts/conformance-schema-smoke.ps1"
blocking = true
"#,
        )?;

        assert!(manifest.conformance_claim);
        assert_eq!(manifest.comparison_class, ComparisonClass::Conformance);

        Ok(())
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
    }
}
