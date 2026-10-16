//! Genuine determineFrequency354-418 including actual warning/continue order.
use super::{Error, SourceErrorBackend, SourceReportFrequency as Frequency};
pub(super) fn determine_frequency(
    text: &str,
    minimum: Frequency,
    backend: &mut SourceErrorBackend<'_>,
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
                backend.show_warning_error(&format!(
                    "DetermineFrequency: Entered frequency=\"{text}\" is not an exact match to key strings."
                ))?;
                backend.show_continue_error(&format!("Frequency={exact} will be used."))?;
            }
            frequency = value.max(minimum);
            break;
        }
    }
    Ok(frequency)
}
