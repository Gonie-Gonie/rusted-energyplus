//! Run configuration and exit-code contracts.

use std::path::PathBuf;

/// Simulation mode requested by the CLI.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunMode {
    /// Only execute the currently declared supported compatibility subset.
    Compatibility,
    /// Allow diagnostic-only supported paths that do not make conformance claims.
    Diagnostic,
    /// Reserved for future performance-oriented execution.
    Fast,
    /// Reserved for explicitly experimental branches.
    Experimental,
}

impl RunMode {
    /// Parses a CLI mode token.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "compatibility" | "compat" => Some(Self::Compatibility),
            "diagnostic" | "diag" => Some(Self::Diagnostic),
            "fast" => Some(Self::Fast),
            "experimental" | "exp" => Some(Self::Experimental),
            _ => None,
        }
    }

    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Compatibility => "compatibility",
            Self::Diagnostic => "diagnostic",
            Self::Fast => "fast",
            Self::Experimental => "experimental",
        }
    }
}

/// Policy for diagnostic/ad-hoc partial runtime execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartialRunPolicy {
    /// Do not execute diagnostic-only partial runtime paths.
    Deny,
    /// Allow diagnostic-only partial runtime paths when the requested mode permits them.
    Allow,
}

impl PartialRunPolicy {
    /// Parses a CLI partial-run policy token.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "deny" | "denied" | "false" => Some(Self::Deny),
            "allow" | "allowed" | "true" => Some(Self::Allow),
            _ => None,
        }
    }

    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Deny => "deny",
            Self::Allow => "allow",
        }
    }

    /// Returns true when partial diagnostic runtime execution is allowed.
    #[must_use]
    pub const fn allows_partial(self) -> bool {
        matches!(self, Self::Allow)
    }
}

/// Output serialization format requested for Rust-native results.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunOutputFormat {
    /// Write only Rust-native JSON/CSV result artifacts.
    RustNative,
    /// Write Rust-native artifacts and compatibility comparison reports.
    Both,
}

impl RunOutputFormat {
    /// Parses a CLI output-format token.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "rust-native" | "rust" => Some(Self::RustNative),
            "both" => Some(Self::Both),
            _ => None,
        }
    }

    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::RustNative => "rust-native",
            Self::Both => "both",
        }
    }
}

/// CLI trace verbosity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceLevel {
    /// Disable optional diagnostic trace artifacts.
    Off,
    /// Write release-safe summaries only.
    Summary,
    /// Include source-order stage metadata.
    Stage,
    /// Include selected zone trace payloads.
    Zone,
    /// Include selected surface trace payloads.
    Surface,
    /// Include selected CTF split trace payloads.
    Ctf,
    /// Include all available opt-in diagnostic trace payloads.
    Full,
}

impl TraceLevel {
    /// Backward-compatible alias for summary-level traces.
    #[allow(non_upper_case_globals)]
    pub const Normal: Self = Self::Summary;
    /// Backward-compatible alias for stage-level traces.
    #[allow(non_upper_case_globals)]
    pub const Detailed: Self = Self::Stage;
    /// Backward-compatible alias for full debug traces.
    #[allow(non_upper_case_globals)]
    pub const Debug: Self = Self::Full;

    /// Parses a CLI trace-level token.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "summary" | "normal" => Some(Self::Summary),
            "stage" | "detailed" | "detail" => Some(Self::Stage),
            "zone" => Some(Self::Zone),
            "surface" => Some(Self::Surface),
            "ctf" => Some(Self::Ctf),
            "full" | "debug" => Some(Self::Full),
            _ => None,
        }
    }

    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Summary => "summary",
            Self::Stage => "stage",
            Self::Zone => "zone",
            Self::Surface => "surface",
            Self::Ctf => "ctf",
            Self::Full => "full",
        }
    }
}

/// Explicit surface/node filters for diagnostic trace payloads.
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Serialize)]
pub struct TraceSelection {
    /// Surface names requested for detailed diagnostic trace payloads.
    pub surface_names: Vec<String>,
    /// Node names requested for detailed diagnostic trace payloads.
    pub node_names: Vec<String>,
}

impl TraceSelection {
    /// Returns true when no targeted trace names were requested.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.surface_names.is_empty() && self.node_names.is_empty()
    }

    /// Adds a requested surface name.
    pub fn push_surface(&mut self, name: impl Into<String>) {
        self.surface_names.push(name.into());
    }

    /// Adds a requested node name.
    pub fn push_node(&mut self, name: impl Into<String>) {
        self.node_names.push(name.into());
    }
}

/// Top-level arbitrary-run configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunConfig {
    /// User-provided IDF or epJSON input path.
    pub input_path: PathBuf,
    /// User-provided EPW weather path.
    pub weather_path: Option<PathBuf>,
    /// Output directory root.
    pub output_dir: PathBuf,
    /// Requested run mode.
    pub mode: RunMode,
    /// Diagnostic/ad-hoc partial runtime policy.
    pub partial_policy: PartialRunPolicy,
    /// Requested output format.
    pub output_format: RunOutputFormat,
    /// Replace an existing non-empty output directory.
    pub overwrite: bool,
    /// Keep intermediate input/model artifacts.
    pub keep_intermediate: bool,
    /// Trace verbosity.
    pub trace_level: TraceLevel,
    /// Explicit surface/node trace targets.
    pub trace_selection: TraceSelection,
    /// Promote warning diagnostics to a failing exit.
    pub fail_on_warning: bool,
    /// Stop after import/compile/support assessment.
    pub dry_run: bool,
    /// Generate an EnergyPlus oracle baseline under `output/oracle`.
    pub oracle_baseline: bool,
    /// Compare Rust result artifacts with the EnergyPlus oracle baseline.
    pub compare_oracle: bool,
    /// Print the run summary JSON to stdout.
    pub json_stdout: bool,
    /// Optional EnergyPlus oracle root override.
    pub oracle_root: Option<PathBuf>,
    /// Optional hourly sample-count override for smoke/debug runs.
    pub hours: Option<usize>,
}

/// EnergyPlus-like arbitrary-run exit code contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunExitCode {
    /// Completed successfully.
    Success,
    /// CLI arguments were invalid.
    Args,
    /// Input import or parse failed.
    ImportParse,
    /// Typed compile or reference resolution failed.
    CompileReference,
    /// Support assessment rejected the input.
    Unsupported,
    /// Execution-plan construction failed.
    Plan,
    /// Runtime execution failed.
    Runtime,
    /// Output export failed.
    OutputExport,
    /// Oracle baseline or comparison failed.
    OracleCompare,
}

impl RunExitCode {
    /// Numeric process exit code.
    #[must_use]
    pub const fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Args => 1,
            Self::ImportParse => 2,
            Self::CompileReference => 3,
            Self::Unsupported => 4,
            Self::Plan => 5,
            Self::Runtime => 6,
            Self::OutputExport => 7,
            Self::OracleCompare => 8,
        }
    }

    /// Stable lower-case identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Args => "args",
            Self::ImportParse => "import-parse",
            Self::CompileReference => "compile-reference",
            Self::Unsupported => "unsupported",
            Self::Plan => "plan",
            Self::Runtime => "runtime",
            Self::OutputExport => "output-export",
            Self::OracleCompare => "oracle-compare",
        }
    }
}
