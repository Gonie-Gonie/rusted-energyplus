//! Runtime diagnostic value types.

use ep_model::OutputHandle;
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

impl RuntimeDiagnosticSeverity {
    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// Runtime diagnostic code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeDiagnosticCode {
    /// A heat-balance compatibility branch is active but unsupported.
    UnsupportedHeatBalanceBranch,
    /// A heat-balance surface boundary is unsupported or unresolved.
    UnsupportedSurfaceBoundary,
    /// A heat-balance state value is NaN or infinite.
    NonFiniteHeatBalanceState,
    /// Requested output variable is not registered for the current model.
    OutputVariableUnavailable,
    /// Compared samples do not share the same timestamp axis.
    TimestampMismatch,
    /// A conformance tolerance was exceeded.
    ToleranceFailure,
    /// Requested meter is not registered for the current model.
    MeterUnavailable,
    /// Duplicate output request.
    DuplicateOutputRequest,
    /// Duplicate output variable registration during compile/precompute.
    DuplicateOutputRegistration,
    /// Duplicate meter request.
    DuplicateMeterRequest,
    /// Duplicate output handle in a result store.
    DuplicateOutputHandle,
    /// Duplicate output key/variable identity in a result store.
    DuplicateOutputSeries,
}

impl RuntimeDiagnosticCode {
    /// Stable CamelCase identifier used in JSON and .err-like reports.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::UnsupportedHeatBalanceBranch => "UnsupportedHeatBalanceBranch",
            Self::UnsupportedSurfaceBoundary => "UnsupportedSurfaceBoundary",
            Self::NonFiniteHeatBalanceState => "NonFiniteHeatBalanceState",
            Self::OutputVariableUnavailable => "OutputVariableUnavailable",
            Self::TimestampMismatch => "TimestampMismatch",
            Self::ToleranceFailure => "ToleranceFailure",
            Self::MeterUnavailable => "MeterUnavailable",
            Self::DuplicateOutputRequest => "DuplicateOutputRequest",
            Self::DuplicateOutputRegistration => "DuplicateOutputRegistration",
            Self::DuplicateMeterRequest => "DuplicateMeterRequest",
            Self::DuplicateOutputHandle => "DuplicateOutputHandle",
            Self::DuplicateOutputSeries => "DuplicateOutputSeries",
        }
    }
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
    /// Pipeline or runtime stage, when applicable.
    pub stage: Option<String>,
    /// Surface name, when applicable.
    pub surface: Option<String>,
    /// Zone name, when applicable.
    pub zone: Option<String>,
    /// Zero-based timestep or sample index, when applicable.
    pub timestep: Option<u64>,
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
