use std::fmt::{Display, Formatter};

use super::ComparisonClass;

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
