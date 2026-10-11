//! Actual selected CTF diagnostic calls into the caller-owned Source backend.

use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure, SourceFatalStop};
use super::surface_manager::ctf_caller_stability::{CtfCallerDiagnostic, CtfCallerDiagnosticLevel};
use super::surface_manager::ctf_layer_preprocessing::CtfLayerDiagnostic;
use std::fmt::{Display, Formatter};

/// A missing actual original name or unrepresentable formatter result is a scope stop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfSourceDiagnosticUnavailable {
    /// Actual one-based original layer name was not supplied at its reached callpoint.
    OriginalLayerName(usize),
    /// The formatter did not yield a representable exponent; no replacement text.
    ExponentFormat,
    /// Source log10-to-int conversion is outside its defined domain.
    UndefinedLog10Conversion,
}

/// An actual backend/fatal stop remains distinct from a Source-domain gap.
#[derive(Debug)]
pub enum CtfSourceDiagnosticStop {
    /// Missing actual diagnostic operand, never a fabricated Source fatal.
    Unavailable(CtfSourceDiagnosticUnavailable),
    /// Actual borrowed output/database operation failed.
    Backend(SourceErrorBackendFailure),
    /// Actual fatal effects stopped or completed with typed termination.
    Fatal(SourceFatalStop),
}
impl Display for CtfSourceDiagnosticStop {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(error) => write!(formatter, "CTF diagnostic unavailable: {error:?}"),
            Self::Backend(error) => Display::fmt(error, formatter),
            Self::Fatal(error) => Display::fmt(error, formatter),
        }
    }
}
impl std::error::Error for CtfSourceDiagnosticStop {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unavailable(_) => None,
            Self::Backend(error) => Some(error),
            Self::Fatal(error) => Some(error),
        }
    }
}
impl From<SourceErrorBackendFailure> for CtfSourceDiagnosticStop {
    fn from(value: SourceErrorBackendFailure) -> Self {
        Self::Backend(value)
    }
}
impl From<CtfSourceDiagnosticUnavailable> for CtfSourceDiagnosticStop {
    fn from(value: CtfSourceDiagnosticUnavailable) -> Self {
        Self::Unavailable(value)
    }
}

#[cfg(test)]
use super::source_round_format::source_next_float;

// Preserve the actual .3R caller and its existing public error variants.
fn source_round_three(value: f64) -> Result<String, CtfSourceDiagnosticUnavailable> {
    super::source_round_format::round(value, 3).map_err(|error| match error {
        super::source_round_format::SourceRoundFormatUnavailable::ExponentFormat => {
            CtfSourceDiagnosticUnavailable::ExponentFormat
        }
        super::source_round_format::SourceRoundFormatUnavailable::UndefinedLog10Conversion => {
            CtfSourceDiagnosticUnavailable::UndefinedLog10Conversion
        }
    })
}

fn layer_name<'a>(
    names: &'a [&'a str],
    ordinal: usize,
) -> Result<&'a str, CtfSourceDiagnosticUnavailable> {
    ordinal
        .checked_sub(1)
        .and_then(|index| names.get(index))
        .copied()
        .ok_or(CtfSourceDiagnosticUnavailable::OriginalLayerName(ordinal))
}

/// Dispatch exactly one actually reached loading call; no issue-ledger replay.
pub fn emit_ctf_loading_diagnostic(
    backend: &mut SourceErrorBackend<'_>,
    original_layer_names: &[&str],
    call: CtfLayerDiagnostic,
) -> Result<(), CtfSourceDiagnosticStop> {
    try_emit_ctf_loading_diagnostic(backend, call, |ordinal| {
        layer_name(original_layer_names, ordinal)
    })
}

/// Resolve only the original name actually read at the reached diagnostic call.
pub fn try_emit_ctf_loading_diagnostic<'a>(
    backend: &mut SourceErrorBackend<'_>,
    call: CtfLayerDiagnostic,
    mut original_name: impl FnMut(usize) -> Result<&'a str, CtfSourceDiagnosticUnavailable>,
) -> Result<(), CtfSourceDiagnosticStop> {
    match call {
        CtfLayerDiagnostic::MaterialTooThick { .. } => backend.show_severe_error(
            "InitConductionTransferFunctions: Material too thick for CTF calculation",
        )?,
        CtfLayerDiagnostic::MaterialName { layer_ordinal } => backend.show_continue_error(
            &format!("material name = {}", original_name(layer_ordinal)?),
        )?,
        CtfLayerDiagnostic::ResistanceBelowMinimum { layer_ordinal } => {
            backend.show_severe_error(&format!(
                "InitConductionTransferFunctions: Material={}R Value below lowest allowed value",
                original_name(layer_ordinal)?
            ))?
        }
        CtfLayerDiagnostic::ResistanceValues {
            minimum,
            resistance,
            ..
        } => backend.show_continue_error(&format!(
            "Lowest allowed value=[{}], Material R Value=[{}].",
            source_round_three(minimum)?,
            source_round_three(resistance)?
        ))?,
    }
    Ok(())
}

/// Dispatch one reached selected caller call before subsequent flags/stores.
pub fn emit_ctf_caller_diagnostic(
    backend: &mut SourceErrorBackend<'_>,
    call: &CtfCallerDiagnostic,
) -> Result<(), CtfSourceDiagnosticStop> {
    match call.level {
        CtfCallerDiagnosticLevel::Severe => backend.show_severe_error(&call.message)?,
        CtfCallerDiagnosticLevel::Continue => backend.show_continue_error(&call.message)?,
        CtfCallerDiagnosticLevel::Fatal => match backend.show_fatal_error(&call.message) {
            Ok(never) => match never {},
            Err(stop) => return Err(CtfSourceDiagnosticStop::Fatal(stop)),
        },
    }
    Ok(())
}

#[cfg(test)]
#[path = "ctf_source_diagnostics_tests.rs"]
mod tests;
