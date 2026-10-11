//! Borrowed real output/callback boundaries for selected UtilityRoutines errors.
//!
//! No process, database, callback or stream owner is invented here. Callers retain
//! every sink and the mutable state even when a fatal or backend failure returns.

use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::io::{self, Write};

/// Selected source error callback values, preserving api/TypeDefs.h identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SourceErrorLevel {
    /// ShowContinueError's unformatted callback argument.
    Continue = 0,
    /// ShowWarningError's unformatted callback argument.
    Warning = 2,
    /// ShowFatalError's final callback before termination.
    Fatal = 4,
}

/// Caller-owned ErrorTracking members read/written by these selected routines.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceErrorTracking {
    /// MatchCounts(1..20), in the original source search order.
    pub match_counts: [i32; 20],
    /// Actual AbortProcessing flag.
    pub abort_processing: bool,
    /// Actual severe count read by the fatal summary; fatal does not increment it.
    pub total_severe_errors: i32,
    /// Actual last severe text read by the fatal summary.
    pub last_severe_error: String,
    /// Actual warning total, incremented by ShowWarningError only.
    pub total_warning_errors: i32,
    /// Actual warning warmup count.
    pub total_warning_errors_during_warmup: i32,
    /// Actual warning sizing count.
    pub total_warning_errors_during_sizing: i32,
}

/// Explicit caller state; no backend absence or global flag is defaulted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceErrorState {
    /// Actual UtilityRoutines outputErrorHeader owner.
    pub output_error_header: bool,
    /// Actual VerStringVar; no fabricated version constant.
    pub version_string: String,
    /// Actual IDDVerString.
    pub idd_version_string: String,
    /// Actual input-processing output route.
    pub doing_input_processing: bool,
    /// Actual console enablement.
    pub print_console_output: bool,
    /// Actual kickoff flag; DisplayString callbacks precede this test.
    pub kickoff_simulation: bool,
    /// Actual DeveloperFlag from the caller's system-variable owner.
    pub developer_flag: bool,
    /// Actual WarmupFlag.
    pub warmup_flag: bool,
    /// Actual DoingSizing.
    pub doing_sizing: bool,
    /// Actual incoming tracking state, retained through failures.
    pub tracking: SourceErrorTracking,
}

/// Adapter to an actual optional SQLite owner, not a replacement record ledger.
/// The caller implements its real create/update/transaction/commit operations.
/// No SQLite implementation or production SQLite parity is claimed by this unit.
pub trait SourceErrorSqlite {
    /// Calls the actual owner's createSQLiteErrorRecord boundary.
    fn create_error_record(
        &mut self,
        simulation_index: i32,
        error_type: i32,
        message: &str,
        count: i32,
    ) -> io::Result<()>;
    /// Calls the actual owner's updateSQLiteErrorRecord with the raw argument.
    fn update_error_record(&mut self, message: &str) -> io::Result<()>;
    /// Reads the actual owner's sqliteWithinTransaction result.
    fn within_transaction(&mut self) -> io::Result<bool>;
    /// Calls the actual owner's sqliteCommit boundary.
    fn commit(&mut self) -> io::Result<()>;
}

/// An unavailable backend operation is distinct from the source fatal return.
#[derive(Debug)]
pub enum SourceErrorBackendFailure {
    /// A real borrowed sink/database operation failed after its reached prefix.
    Effect {
        /// Exact attempted backend operation.
        operation: &'static str,
        /// Actual adapter failure, without a Native failure claim.
        source: io::Error,
    },
    /// Signed-i32 increment lies outside the defined original source domain.
    UndefinedCounterIncrement {
        /// Actual counter at the boundary; earlier increments remain retained.
        counter: &'static str,
    },
}

impl Display for SourceErrorBackendFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Effect { operation, source } => write!(formatter, "{operation}: {source}"),
            Self::UndefinedCounterIncrement { counter } => {
                write!(formatter, "undefined source counter increment: {counter}")
            }
        }
    }
}

impl std::error::Error for SourceErrorBackendFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Effect { source, .. } => Some(source),
            Self::UndefinedCounterIncrement { .. } => None,
        }
    }
}

/// Genuine typed termination after all reached fatal backend effects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceFatalError {
    /// Original unformatted FatalError argument.
    pub message: String,
}

impl Display for SourceFatalError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for SourceFatalError {}

/// A backend failure does not pretend the final source throw was reached.
#[derive(Debug)]
pub enum SourceFatalStop {
    /// Actual backend operation stopped before the selected throw boundary.
    Backend(SourceErrorBackendFailure),
    /// All selected effects completed, then the actual typed fatal was returned.
    Fatal(SourceFatalError),
}

impl From<SourceErrorBackendFailure> for SourceFatalStop {
    fn from(value: SourceErrorBackendFailure) -> Self {
        Self::Backend(value)
    }
}

impl Display for SourceFatalStop {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(error) => Display::fmt(error, formatter),
            Self::Fatal(error) => Display::fmt(error, formatter),
        }
    }
}
impl std::error::Error for SourceFatalStop {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(error) => Some(error),
            Self::Fatal(error) => Some(error),
        }
    }
}

/// Actual mutable callbacks can affect the same supplied state before later reads.
pub type SourceMessageCallback<'a> = dyn FnMut(&mut SourceErrorState, &str) + 'a;
/// Error callbacks receive the original level and unformatted message.
pub type SourceErrorCallback<'a> = dyn FnMut(&mut SourceErrorState, SourceErrorLevel, &str) + 'a;

/// Borrowed backend owners; optional presence is an explicit caller obligation.
pub struct SourceErrorBackend<'a> {
    /// Actual global/tracking owner, never moved into a discarded error result.
    pub state: &'a mut SourceErrorState,
    /// Actual standard error file stream, if the caller has one.
    pub error_stream: Option<&'a mut dyn Write>,
    /// Actual console stream, including DisplayString's final flush.
    pub console: &'a mut dyn Write,
    /// Actual optional first OutputFile stream; no newline is added by print.
    pub out_unit1: Option<&'a mut dyn Write>,
    /// Actual optional second OutputFile stream.
    pub out_unit2: Option<&'a mut dyn Write>,
    /// Actual fMessagePtr callback, before the messageCallback.
    pub f_message: Option<&'a mut SourceMessageCallback<'a>>,
    /// Actual messageCallback, before DisplayString's console guards.
    pub message_callback: Option<&'a mut SourceMessageCallback<'a>>,
    /// Actual errorCallback, called after SQLite effects.
    pub error_callback: Option<&'a mut SourceErrorCallback<'a>>,
    /// Actual optional SQLite owner; None is never selected internally.
    pub sqlite: Option<&'a mut dyn SourceErrorSqlite>,
}

fn effect<T>(
    operation: &'static str,
    result: io::Result<T>,
) -> Result<T, SourceErrorBackendFailure> {
    result.map_err(|source| SourceErrorBackendFailure::Effect { operation, source })
}

fn increment(value: &mut i32, counter: &'static str) -> Result<(), SourceErrorBackendFailure> {
    *value = value
        .checked_add(1)
        .ok_or(SourceErrorBackendFailure::UndefinedCounterIncrement { counter })?;
    Ok(())
}

const MESSAGE_SEARCH: [&str; 20] = [
    "InterZone Surface Areas",
    "CAUTION -- Interzone",
    "Node Connection Error",
    "InterZone Surface Azimu",
    "InterZone Surface Tilts",
    "Suspected non-planar",
    "Deprecated",
    "Floor Tilt=",
    "Roof/Ceiling Tilt=",
    "View factors not",
    "Unbalanced exhaust air",
    "Loads Initialization",
    "CalcDaylightMapPoints:",
    "Zone Air Heat Balance",
    "occupant density is ext",
    "Temperature (low) out o",
    "Temperature (high) out",
    "nominally unused",
    "InfraredTransparent",
    "No reporting elements",
];

impl SourceErrorBackend<'_> {
    /// Literal ShowErrorMessage stream/header/input-processing/optional-output order.
    pub fn show_error_message(&mut self, message: &str) -> Result<(), SourceErrorBackendFailure> {
        if self.state.output_error_header {
            if let Some(stream) = self.error_stream.as_deref_mut() {
                effect(
                    "error header",
                    writeln!(
                        stream,
                        "Program Version,{},{}",
                        self.state.version_string, self.state.idd_version_string
                    ),
                )?;
                self.state.output_error_header = false;
            }
        }
        if !self.state.doing_input_processing {
            if let Some(stream) = self.error_stream.as_deref_mut() {
                effect("error message", writeln!(stream, "  {message}"))?;
            }
        } else if self.state.print_console_output {
            effect(
                "input-processing console",
                writeln!(self.console, "{message}"),
            )?;
        }
        if let Some(stream) = self.out_unit1.as_deref_mut() {
            effect("optional output1", write!(stream, "  {message}"))?;
        }
        if let Some(stream) = self.out_unit2.as_deref_mut() {
            effect("optional output2", write!(stream, "  {message}"))?;
        }
        Ok(())
    }

    /// Literal std::string DisplayString order, including callbacks before guards.
    pub fn display_string(&mut self, message: &str) -> Result<(), SourceErrorBackendFailure> {
        if let Some(callback) = self.f_message.as_deref_mut() {
            callback(self.state, message);
        }
        // Native passes String.c_str() to std::function<void(const std::string&)>.
        // That temporary string ends at the first embedded NUL; fMessagePtr above
        // and the subsequent std::string console write retain the complete text.
        let callback_message = match message.find('\0') {
            Some(end) => &message[..end],
            None => message,
        };
        if let Some(callback) = self.message_callback.as_deref_mut() {
            callback(self.state, callback_message);
        }
        if self.state.kickoff_simulation && !self.state.developer_flag {
            return Ok(());
        }
        if !self.state.print_console_output {
            return Ok(());
        }
        effect("display console", writeln!(self.console, "{message}"))?;
        effect("display console flush", self.console.flush())
    }

    /// Genuine warning counters, output, optional SQLite and final callback.
    pub fn show_warning_error(&mut self, message: &str) -> Result<(), SourceErrorBackendFailure> {
        for (&needle, count) in MESSAGE_SEARCH
            .iter()
            .zip(&mut self.state.tracking.match_counts)
        {
            if message.contains(needle) {
                increment(count, "MatchCounts")?;
            }
        }
        increment(
            &mut self.state.tracking.total_warning_errors,
            "TotalWarningErrors",
        )?;
        if self.state.warmup_flag
            && !self.state.doing_sizing
            && !self.state.kickoff_simulation
            && !self.state.tracking.abort_processing
        {
            increment(
                &mut self.state.tracking.total_warning_errors_during_warmup,
                "TotalWarningErrorsDuringWarmup",
            )?;
        }
        if self.state.doing_sizing {
            increment(
                &mut self.state.tracking.total_warning_errors_during_sizing,
                "TotalWarningErrorsDuringSizing",
            )?;
        }
        self.show_error_message(&format!(" ** Warning ** {message}"))?;
        if let Some(sqlite) = self.sqlite.as_deref_mut() {
            effect(
                "SQLite create warning",
                sqlite.create_error_record(1, 0, message, 1),
            )?;
        }
        if let Some(callback) = self.error_callback.as_deref_mut() {
            callback(self.state, SourceErrorLevel::Warning, message);
        }
        Ok(())
    }

    /// Genuine continued output, optional SQLite update and final callback.
    pub fn show_continue_error(&mut self, message: &str) -> Result<(), SourceErrorBackendFailure> {
        self.show_error_message(&format!(" **   ~~~   ** {message}"))?;
        if let Some(sqlite) = self.sqlite.as_deref_mut() {
            effect(
                "SQLite update continue",
                sqlite.update_error_record(message),
            )?;
        }
        if let Some(callback) = self.error_callback.as_deref_mut() {
            callback(self.state, SourceErrorLevel::Continue, message);
        }
        Ok(())
    }

    /// Performs real fatal effects in source order, then returns typed termination.
    /// It cannot return successful function completion or increment severe totals.
    pub fn show_fatal_error(&mut self, message: &str) -> Result<Infallible, SourceFatalStop> {
        self.show_error_message(&format!(" **  Fatal  ** {message}"))?;
        self.display_string(&format!("**FATAL:{message}"))?;
        self.show_error_message(" ...Summary of Errors that led to program termination:")?;
        self.show_error_message(&format!(
            " ..... Reference severe error count={}",
            self.state.tracking.total_severe_errors
        ))?;
        self.show_error_message(&format!(
            " ..... Last severe error={}",
            self.state.tracking.last_severe_error
        ))?;
        if let Some(sqlite) = self.sqlite.as_deref_mut() {
            effect(
                "SQLite create fatal",
                sqlite.create_error_record(1, 2, message, 1),
            )?;
            if effect("SQLite transaction state", sqlite.within_transaction())? {
                effect("SQLite commit", sqlite.commit())?;
            }
        }
        if let Some(callback) = self.error_callback.as_deref_mut() {
            callback(self.state, SourceErrorLevel::Fatal, message);
        }
        Err(SourceFatalStop::Fatal(SourceFatalError {
            message: message.to_owned(),
        }))
    }
}

#[cfg(test)]
#[path = "error_backend_tests.rs"]
mod tests;
