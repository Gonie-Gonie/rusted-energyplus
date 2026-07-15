use ep_model::DayOfWeek;
use std::fmt::{Display, Formatter};

/// Error returned while resolving a run period or building a time axis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeAxisError {
    /// An end year was supplied without the required start year.
    EndYearWithoutStartYear {
        /// Run period name.
        run_period_name: String,
    },
    /// EnergyPlus does not accept run-period years before 1583.
    StartYearBeforeGregorianCalendar {
        /// Run period name.
        run_period_name: String,
        /// Invalid start year.
        year: u32,
    },
    /// A run-period date was invalid.
    InvalidDate {
        /// Run period name.
        run_period_name: String,
        /// Field group, such as begin or end.
        field: &'static str,
        /// Calendar year.
        year: u32,
        /// Month number.
        month: u32,
        /// Day of month.
        day_of_month: u32,
    },
    /// The end date came before the begin date.
    InvalidRange {
        /// Run period name.
        run_period_name: String,
    },
    /// Metadata-aware actual-weather traversal is not implemented yet.
    ActualWeatherUnsupported {
        /// Run period name.
        run_period_name: String,
    },
    /// An EPW nth-weekday daylight-saving rule has no date in the resolved month.
    DaylightSavingDateRuleDoesNotExist {
        /// Run period name.
        run_period_name: String,
        /// Rule boundary, `start` or `end`.
        boundary: &'static str,
        /// One-based requested weekday occurrence.
        nth: u32,
        /// Requested weekday.
        weekday: DayOfWeek,
        /// Requested month.
        month: u32,
    },
    /// A weather-file or input-file nth-weekday special-day rule has no date in the month.
    SpecialDayDateRuleDoesNotExist {
        /// Run period name.
        run_period_name: String,
        /// Special-day object name.
        special_day_name: String,
        /// One-based requested weekday occurrence.
        nth: u32,
        /// Requested weekday.
        weekday: DayOfWeek,
        /// Requested month.
        month: u32,
    },
}

impl Display for TimeAxisError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EndYearWithoutStartYear { run_period_name } => write!(
                formatter,
                "run period {run_period_name} has an end year without a start year"
            ),
            Self::StartYearBeforeGregorianCalendar {
                run_period_name,
                year,
            } => write!(
                formatter,
                "run period {run_period_name} start year {year} is before 1583"
            ),
            Self::InvalidDate {
                run_period_name,
                field,
                year,
                month,
                day_of_month,
            } => write!(
                formatter,
                "run period {run_period_name} has invalid {field} date {year:04}-{month:02}-{day_of_month:02}"
            ),
            Self::InvalidRange { run_period_name } => {
                write!(
                    formatter,
                    "run period {run_period_name} ends before it begins"
                )
            }
            Self::ActualWeatherUnsupported { run_period_name } => write!(
                formatter,
                "run period {run_period_name} treats weather as actual, but metadata-aware EPW record traversal is not implemented"
            ),
            Self::DaylightSavingDateRuleDoesNotExist {
                run_period_name,
                boundary,
                nth,
                weekday,
                month,
            } => write!(
                formatter,
                "run period {run_period_name} has no occurrence {nth} of {weekday:?} in month {month} for its daylight-saving {boundary} rule"
            ),
            Self::SpecialDayDateRuleDoesNotExist {
                run_period_name,
                special_day_name,
                nth,
                weekday,
                month,
            } => write!(
                formatter,
                "run period {run_period_name} special day {special_day_name} has no occurrence {nth} of {weekday:?} in month {month}"
            ),
        }
    }
}

impl std::error::Error for TimeAxisError {}
