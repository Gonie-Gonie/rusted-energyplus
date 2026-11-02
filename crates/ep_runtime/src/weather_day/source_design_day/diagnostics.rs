//! Exact UtilityRoutines1554/1589/1698/1712/1719 helper bodies on the same backend.
use super::{Error, FAMILY, SourceErrorBackend};
pub(super) fn severe_header(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
) -> Result<(), Error> {
    backend.show_severe_error(&format!("GetDesignDayData: {FAMILY} = {title}"))?;
    Ok(())
}
pub(super) fn warning_header(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
) -> Result<(), Error> {
    backend.show_warning_error(&format!("GetDesignDayData: {FAMILY} = {title}"))?;
    Ok(())
}
pub(super) fn empty(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    dependent: &str,
    value: &str,
) -> Result<(), Error> {
    severe_header(backend, title)?;
    let suffix = if dependent.is_empty() {
        String::new()
    } else {
        format!(" when {dependent} = {value}")
    };
    backend.show_continue_error(&format!("{field} cannot be empty{suffix}."))?;
    Ok(())
}
pub(super) fn invalid_key(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    value: &str,
) -> Result<(), Error> {
    severe_header(backend, title)?;
    backend.show_continue_error(&format!("{field} = {value}, invalid key."))?;
    Ok(())
}
pub(super) fn warning_key(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    value: &str,
    default: &str,
) -> Result<(), Error> {
    warning_header(backend, title)?;
    backend.show_continue_error(&format!(
        "{field} = {value}, invalid key, {default} will be used."
    ))?;
    Ok(())
}
pub(super) fn boolean(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    value: &str,
    blank: bool,
) -> Result<i32, Error> {
    if blank {
        return Ok(0);
    }
    match value.to_ascii_uppercase().as_str() {
        "NO" => Ok(0),
        "YES" => Ok(1),
        _ => {
            warning_header(backend, title)?;
            backend.show_continue_error(&format!(
                "{field} = {value}, invalid boolean (\"Yes\"/\"No\"), No will be used."
            ))?;
            Ok(0)
        }
    }
}
pub(super) fn warning_empty(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    default: &str,
    dependent: &str,
    value: &str,
) -> Result<(), Error> {
    warning_header(backend, title)?;
    backend.show_continue_error(&format!("{field} is empty."))?;
    if !dependent.is_empty() {
        backend.show_continue_error(&format!("Cannot be empty when {dependent} = {value}"))?;
    }
    if !default.is_empty() {
        backend.show_continue_error(&format!("{default} will be used."))?;
    }
    Ok(())
}

/// UtilityRoutines1564–1568.
pub(super) fn item_not_found(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    value: &str,
) -> Result<(), Error> {
    severe_header(backend, title)?;
    backend.show_continue_error(&format!("{field} = {value}, item not found."))?;
    Ok(())
}
/// UtilityRoutines1747–1755, with the actual DD humidity fallback argument.
pub(super) fn missing_humidity(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    value: &str,
) -> Result<(), Error> {
    warning_header(backend, title)?;
    backend.show_continue_error(&format!("{field} = {value}, item not found, Default Humidity (constant for day using Humidity Indicator Temp). will be used."))?;
    Ok(())
}
/// ScheduleManager3937–3952, inclusive minimum and no trailing message.
pub(super) fn bad_min(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    value: &str,
    minimum: f64,
) -> Result<(), Error> {
    severe_header(backend, title)?;
    backend.show_continue_error(&format!(
        "{field} = {value}, schedule contains values that are < {minimum}"
    ))?;
    Ok(())
}
/// ScheduleManager3971–3993, two inclusive bounds and no trailing message.
pub(super) fn bad_min_max(
    backend: &mut SourceErrorBackend<'_>,
    title: &str,
    field: &str,
    value: &str,
    minimum: f64,
    maximum: f64,
) -> Result<(), Error> {
    severe_header(backend, title)?;
    backend.show_continue_error(&format!(
        "{field} = {value}, schedule contains values that are < {minimum} and/or > {maximum}"
    ))?;
    Ok(())
}
