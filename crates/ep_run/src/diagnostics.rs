//! Structured diagnostics for arbitrary run artifacts.

use serde::Serialize;

/// Run diagnostic severity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunDiagnosticSeverity {
    /// Informational note.
    Info,
    /// Non-blocking warning.
    Warning,
    /// Blocking error.
    Error,
}

impl RunDiagnosticSeverity {
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

/// One structured arbitrary-run diagnostic.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RunDiagnostic {
    /// Severity.
    pub severity: RunDiagnosticSeverity,
    /// Stable diagnostic code.
    pub code: String,
    /// Pipeline stage that emitted the diagnostic.
    pub stage: String,
    /// EnergyPlus object type when applicable.
    pub object_type: Option<String>,
    /// EnergyPlus object name when applicable.
    pub object_name: Option<String>,
    /// Field name when applicable.
    pub field: Option<String>,
    /// Human-readable message.
    pub message: String,
    /// True when this diagnostic blocks Rust execution.
    pub blocking: bool,
}

impl RunDiagnostic {
    /// Creates a diagnostic.
    #[must_use]
    pub fn new(
        severity: RunDiagnosticSeverity,
        code: impl Into<String>,
        stage: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        let blocking = severity == RunDiagnosticSeverity::Error;
        Self {
            severity,
            code: code.into(),
            stage: stage.into(),
            object_type: None,
            object_name: None,
            field: None,
            message: message.into(),
            blocking,
        }
    }

    /// Adds object context.
    #[must_use]
    pub fn with_object(
        mut self,
        object_type: impl Into<String>,
        object_name: Option<String>,
    ) -> Self {
        self.object_type = Some(object_type.into());
        self.object_name = object_name;
        self
    }

    /// Adds field context.
    #[must_use]
    pub fn with_field(mut self, field: Option<String>) -> Self {
        self.field = field;
        self
    }
}

/// Accumulated run diagnostics.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RunDiagnostics {
    /// Diagnostics in encounter order.
    pub diagnostics: Vec<RunDiagnostic>,
}

impl RunDiagnostics {
    /// Adds one diagnostic.
    pub fn push(&mut self, diagnostic: RunDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Adds one error diagnostic.
    pub fn error(
        &mut self,
        code: impl Into<String>,
        stage: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.push(RunDiagnostic::new(
            RunDiagnosticSeverity::Error,
            code,
            stage,
            message,
        ));
    }

    /// Adds one warning diagnostic.
    pub fn warning(
        &mut self,
        code: impl Into<String>,
        stage: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.push(RunDiagnostic::new(
            RunDiagnosticSeverity::Warning,
            code,
            stage,
            message,
        ));
    }

    /// Adds one informational diagnostic.
    pub fn info(
        &mut self,
        code: impl Into<String>,
        stage: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.push(RunDiagnostic::new(
            RunDiagnosticSeverity::Info,
            code,
            stage,
            message,
        ));
    }

    /// Returns true when any blocking diagnostic is present.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == RunDiagnosticSeverity::Error)
    }

    /// Returns true when any warning diagnostic is present.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == RunDiagnosticSeverity::Warning)
    }

    /// Number of diagnostics by severity.
    #[must_use]
    pub fn count_by_severity(&self, severity: RunDiagnosticSeverity) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == severity)
            .count()
    }
}
