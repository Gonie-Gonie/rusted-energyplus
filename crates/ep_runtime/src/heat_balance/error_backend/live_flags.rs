//! Short per-diagnostic views of the actual Source phase cells.
//!
//! These borrows are constructed at each diagnostic call, never retained on the
//! backend or copied into its legacy callback state. Positive legacy callbacks
//! remain explicit Source boundaries at their original reached positions.

use super::super::selected_heat_balance_input::HeatBalanceInputError;
use super::SourceErrorBackend;

/// References to the same Globals, SystemVariables and WeatherGlobal cells.
#[derive(Debug)]
pub(crate) struct SourceDiagnosticFlags<'call> {
    pub(crate) kickoff: &'call bool,
    pub(crate) developer: &'call bool,
    pub(crate) warmup: &'call bool,
    pub(crate) doing_sizing: &'call bool,
}

impl SourceErrorBackend<'_> {
    /// Genuine ShowFatalError with live Source DisplayString phase operands.
    /// Existing callback/host boundaries remain explicit if DisplayString stops.
    pub(crate) fn show_source_fatal_error(
        &mut self,
        message: &str,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<std::convert::Infallible, HeatBalanceInputError> {
        self.show_error_message(&format!(" **  Fatal  ** {message}"))?;
        self.display_source_string(&format!("**FATAL:{message}"), flags)?;
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
            super::effect(
                "SQLite create fatal",
                sqlite.create_error_record(1, 2, message, 1),
            )?;
            if super::effect("SQLite transaction state", sqlite.within_transaction())? {
                super::effect("SQLite commit", sqlite.commit())?;
            }
        }
        if let Some(callback) = self.error_callback.as_deref_mut() {
            callback(
                self.state,
                super::SourceErrorLevel::Fatal,
                message.as_bytes(),
            );
        }
        Err(HeatBalanceInputError::Fatal(super::SourceFatalStop::Fatal(
            super::SourceFatalError {
                message: message.to_owned(),
            },
        )))
    }

    /// Source DisplayString63–92; callbacks precede every phase-flag read.
    pub(crate) fn display_source_string(
        &mut self,
        message: &str,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), HeatBalanceInputError> {
        if self.f_message.is_some() {
            return Err(HeatBalanceInputError::Unavailable(
                "DisplayString78 actual fMessagePtr callback with live Source owners",
            ));
        }
        if self.message_callback.is_some() {
            return Err(HeatBalanceInputError::Unavailable(
                "DisplayString81 actual messageCallback with live Source owners",
            ));
        }
        Self::write_display_console(
            self.console,
            message,
            flags.kickoff,
            flags.developer,
            &self.state.print_console_output,
        )?;
        Ok(())
    }

    /// Source ShowWarningError980–1021 with actual ordered phase reads.
    pub(crate) fn show_source_warning_error(
        &mut self,
        message: &str,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), HeatBalanceInputError> {
        Self::count_warning_error(
            &mut self.state.tracking,
            message,
            flags.warmup,
            flags.doing_sizing,
            flags.kickoff,
        )?;
        self.write_warning_error(message)?;
        if self.error_callback.is_some() {
            return Err(HeatBalanceInputError::Unavailable(
                "ShowWarningError1014 actual errorCallback with live Source owners",
            ));
        }
        Ok(())
    }

    /// Source ShowSevereError796–837; last text and SQLite precede callback.
    pub(crate) fn show_source_severe_error(
        &mut self,
        message: &str,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), HeatBalanceInputError> {
        Self::count_severe_error(
            &mut self.state.tracking,
            message,
            flags.warmup,
            flags.doing_sizing,
            flags.kickoff,
        )?;
        self.write_severe_error_using(message, Self::show_error_message)?;
        if self.error_callback.is_some() {
            return Err(HeatBalanceInputError::Unavailable(
                "ShowSevereError833 actual errorCallback with live Source owners",
            ));
        }
        Ok(())
    }
}
