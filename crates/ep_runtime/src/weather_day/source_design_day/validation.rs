//! Native6563–6620: literal month/day, DST and day-type validation/write order.
use super::diagnostics;
use super::*;
const DAY_TYPES: [&str; 13] = [
    "UNUSED",
    "SUNDAY",
    "MONDAY",
    "TUESDAY",
    "WEDNESDAY",
    "THURSDAY",
    "FRIDAY",
    "SATURDAY",
    "HOLIDAY",
    "SUMMERDESIGNDAY",
    "WINTERDESIGNDAY",
    "CUSTOMDAY1",
    "CUSTOMDAY2",
];
pub(super) fn calendar_and_day_type(
    d: &mut SourceDesignDayData,
    s: &SourceInputProcessorShortCut,
    backend: &mut SourceErrorBackend<'_>,
    errors: &mut bool,
) -> Result<(), Error> {
    match d.month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => {
            if d.day_of_month > 31 {
                backend.show_severe_error(&format!("{FAMILY}=\"{}\", invalid data.", d.title))?;
                backend.show_continue_error(&format!(
                    ".. invalid field: {}=[{}], Month=[{}].",
                    s.numeric_field_names[1], d.day_of_month, d.month
                ))?;
                *errors = true;
            }
        }
        4 | 6 | 9 | 11 => {
            if d.day_of_month > 30 {
                backend.show_severe_error(&format!("{FAMILY}=\"{}\", invalid data.", d.title))?;
                backend.show_continue_error(&format!(
                    ".. invalid {}=[{}], Month=[{}].",
                    s.numeric_field_names[1], d.day_of_month, d.month
                ))?;
                *errors = true;
            }
        }
        2 => {
            if d.day_of_month > 28 {
                backend.show_severe_error(&format!("{FAMILY}=\"{}\", invalid data.", d.title))?;
                backend.show_continue_error(&format!(
                    ".. invalid {}=[{}], Month=[{}].",
                    s.numeric_field_names[1], d.day_of_month, d.month
                ))?;
                *errors = true;
            }
        }
        _ => {
            backend.show_severe_error(&format!("{FAMILY}=\"{}\", invalid data.", d.title))?;
            backend.show_continue_error(&format!(
                ".. invalid {} invalid (Month) [{}].",
                s.numeric_field_names[0], d.month
            ))?;
            *errors = true;
        }
    }
    // No extra day>=1 admission here: Native delegates that earlier to InputProcessor.
    d.daylight_saving_indicator = diagnostics::boolean(
        backend,
        &d.title,
        &s.alpha_field_names[8],
        &s.alpha_args[8],
        s.alpha_field_blanks[8],
    )?;
    d.day_type = enum_value(&DAY_TYPES, &s.alpha_args[1]);
    if d.day_type <= 0 {
        diagnostics::invalid_key(backend, &d.title, &s.alpha_field_names[1], &s.alpha_args[1])?;
        *errors = true;
    }
    Ok(())
}
