//! Genuine determineFrequency354-418 and ordered minimum-override EIO notices.
use super::SourceReportFrequency as Frequency;
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use std::io::{self, Write};

pub(super) fn determine_frequency(
    text: &str,
    minimum: Frequency,
    backend: &mut SourceErrorBackend<'_>,
    flags: Option<&SourceDiagnosticFlags<'_>>,
) -> Result<Frequency, Error> {
    let mut frequency = Frequency::Hour;
    let upper = text.to_ascii_uppercase();
    let length = text.len().min(4);
    if length < 4 {
        return Ok(frequency);
    }
    let prefix = &upper.as_bytes()[..length];
    for (possible, exact, exact_upper, value) in [
        ("DETA", "Detailed", "DETAILED", Frequency::EachCall),
        ("TIME", "Timestep", "TIMESTEP", Frequency::TimeStep),
        ("HOUR", "Hourly", "HOURLY", Frequency::Hour),
        ("DAIL", "Daily", "DAILY", Frequency::Day),
        ("MONT", "Monthly", "MONTHLY", Frequency::Month),
        ("RUNP", "RunPeriod", "RUNPERIOD", Frequency::Simulation),
        ("ENVI", "Environment", "ENVIRONMENT", Frequency::Simulation),
        ("ANNU", "Annual", "ANNUAL", Frequency::Year),
    ] {
        if prefix == possible.as_bytes() {
            if upper != exact_upper {
                let flags = flags.ok_or(Error::Unavailable(
                    "actual determineFrequency warning phase owners",
                ))?;
                backend.show_source_warning_error(&format!(
                    "DetermineFrequency: Entered frequency=\"{text}\" is not an exact match to key strings."
                ), flags)?;
                backend.show_continue_error(&format!("Frequency={exact} will be used."))?;
            }
            frequency = value.max(minimum);
            break;
        }
    }
    Ok(frequency)
}

/// Caller assigns minimumReportFreq before these two Source print operations.
/// Retain the notice's leading space/exclamation mark, and propagate each real
/// stream error before proceeding to the next write or request-object getter.
pub(super) fn write_minimum_frequency_override(
    eio: &mut dyn Write,
    frequency: Frequency,
    text: &str,
) -> io::Result<()> {
    eio.write_all(
        b"! <Minimum Reporting Frequency (overriding input value)>, Value, Input Value\n",
    )?;
    eio.write_all(
        format!(
            " Minimum Reporting Frequency, {},{}\n",
            frequency.dictionary_format().2,
            text,
        )
        .as_bytes(),
    )
}
