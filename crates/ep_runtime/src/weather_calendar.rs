//! EPW calendar-policy header parsing.

use super::{EPW_HEADER_LINE_COUNT, EpwCalendarMetadata, EpwError, epw_field};

const EPW_HOLIDAYS_DAYLIGHT_SAVINGS_HEADER: &str = "HOLIDAYS/DAYLIGHT SAVINGS";
const EPW_HOLIDAYS_DAYLIGHT_SAVING_HEADER_PREFIX: &str = "HOLIDAYS/DAYLIGHT SAVING";

pub(super) fn parse_epw_calendar_metadata(contents: &str) -> Result<EpwCalendarMetadata, EpwError> {
    let (line_index, line) = contents
        .lines()
        .take(EPW_HEADER_LINE_COUNT)
        .enumerate()
        .find(|(_line_index, line)| {
            line.split(',')
                .next()
                .is_some_and(holidays_daylight_saving_header_matches)
        })
        .ok_or(EpwError::MissingHeader {
            header: EPW_HOLIDAYS_DAYLIGHT_SAVINGS_HEADER,
        })?;
    let line_number = line_index + 1;
    let fields = line.split(',').collect::<Vec<_>>();
    let value = epw_field(&fields, line_number, 1, "leap year observed")?;
    let leap_year_observed = value
        .trim()
        .chars()
        .next()
        .ok_or_else(|| EpwError::InvalidValue {
            line: line_number,
            field: "leap year observed",
            value: value.to_string(),
        })?
        .eq_ignore_ascii_case(&'y');

    Ok(EpwCalendarMetadata { leap_year_observed })
}

fn holidays_daylight_saving_header_matches(header: &str) -> bool {
    header
        .trim()
        .get(..EPW_HOLIDAYS_DAYLIGHT_SAVING_HEADER_PREFIX.len())
        .is_some_and(|prefix| {
            prefix.eq_ignore_ascii_case(EPW_HOLIDAYS_DAYLIGHT_SAVING_HEADER_PREFIX)
        })
}
