//! Native6247–6467 humidity switch and actual default-profile/diagnostic order.
use super::diagnostics;
use super::getter::fill_default_profile;
use super::state::{SourceDesignDayModifierGrid, SourceSizingPeriodSiteSchedule};
use super::*;
const HUMIDITY_NAMES: [&str; 8] = [
    "WETBULB",
    "DEWPOINT",
    "ENTHALPY",
    "HUMIDITYRATIO",
    "RELATIVEHUMIDITYSCHEDULE",
    "WETBULBPROFILEDEFAULTMULTIPLIERS",
    "WETBULBPROFILEDIFFERENCESCHEDULE",
    "WETBULBPROFILEMULTIPLIERSCHEDULE",
];
// Preserve Native humidity switch cases and diagnostic ordering.
#[allow(clippy::too_many_arguments, clippy::collapsible_match)]
pub(super) fn read(
    d: &mut SourceDesignDayData,
    modifiers: &mut Option<Vec<Option<SourceDesignDayModifierGrid>>>,
    numbers: &mut Vec<i32>,
    sites: &[SourceSizingPeriodSiteSchedule],
    environment: i32,
    o: &mut SourceDesignDayInputOwners<'_, '_>,
    errors: &mut bool,
) -> Result<(), Error> {
    d.humidity_indicator_type = enum_value(
        &HUMIDITY_NAMES,
        &o.shortcut.alpha_args[4].to_ascii_uppercase(),
    );
    match d.humidity_indicator_type {
        0 | 1 => {
            if o.shortcut.numeric_field_blanks[4] {
                diagnostics::empty(
                    o.backend,
                    &d.title,
                    &o.shortcut.numeric_field_names[4],
                    &o.shortcut.alpha_field_names[4],
                    &o.shortcut.alpha_args[4],
                )?;
                *errors = true;
            } else {
                d.humidity_indicator_value = o.shortcut.numeric_args[4];
            }
            if d.humidity_indicator_value < -90.0 || d.humidity_indicator_value > 70.0 {
                diagnostics::severe_header(o.backend, &d.title)?;
                let label = if d.humidity_indicator_type == 0 {
                    "WetBulb"
                } else {
                    "DewPoint"
                };
                o.backend.show_continue_error(&format!(
                    "{} - {label} = {} is out of range [-90.0, 70.0]",
                    o.shortcut.alpha_field_names[4],
                    rounded(d.humidity_indicator_value, 2)?
                ))?;
                *errors = true;
            }
        }
        3 => {
            if o.shortcut.numeric_field_blanks[5] {
                diagnostics::empty(
                    o.backend,
                    &d.title,
                    &o.shortcut.numeric_field_names[5],
                    &o.shortcut.alpha_field_names[4],
                    &o.shortcut.alpha_args[4],
                )?;
                *errors = true;
            } else {
                d.humidity_indicator_value = o.shortcut.numeric_args[5];
            }
            if d.humidity_indicator_value < 0.0 || d.humidity_indicator_value > 0.03 {
                diagnostics::severe_header(o.backend, &d.title)?;
                o.backend.show_continue_error(&format!(
                    "{} - Humidity-Ratio = {} is out of range [0.0, 0.03]",
                    o.shortcut.alpha_field_names[4],
                    rounded(d.humidity_indicator_value, 2)?
                ))?;
                *errors = true;
            }
        }
        2 => {
            if o.shortcut.numeric_field_blanks[6] {
                diagnostics::empty(
                    o.backend,
                    &d.title,
                    &o.shortcut.numeric_field_names[6],
                    &o.shortcut.alpha_field_names[4],
                    &o.shortcut.alpha_args[4],
                )?;
                *errors = true;
            } else {
                d.humidity_indicator_value = o.shortcut.numeric_args[6];
            }
            d.humidity_indicator_type = 2; // Native6315 reached repeated write.
            if d.humidity_indicator_value < 0.0 || d.humidity_indicator_value > 130000.0 {
                diagnostics::severe_header(o.backend, &d.title)?;
                // Preserve actual malformed {.0R}, rather than silently fixing it.
                return Err(Error::Unavailable(
                    "GetDesignDayData6319 malformed Native {.0R} format boundary",
                ));
            }
        }
        4 => {} // Native unitType Percentage is used by the scheduled child below.
        5..=7 => {
            if o.shortcut.numeric_field_blanks[4] {
                diagnostics::empty(
                    o.backend,
                    &d.title,
                    &o.shortcut.numeric_field_names[4],
                    &o.shortcut.alpha_field_names[4],
                    &o.shortcut.alpha_args[4],
                )?;
                *errors = true;
            } else {
                d.humidity_indicator_value = o.shortcut.numeric_args[4];
            }
        }
        _ => {
            o.backend
                .show_warning_error(&format!("{FAMILY}=\"{}\", invalid data.", d.title))?;
            o.backend.show_continue_error(&format!(
                "..invalid field: {}=\"{}\".",
                o.shortcut.alpha_field_names[4], o.shortcut.alpha_args[4]
            ))?;
            o.backend.show_continue_error("WetBulb will be used. Maximum Dry Bulb will be used as WetBulb at Maximum Dry Bulb.")?;
            d.humidity_indicator_type = 0;
            d.humidity_indicator_value = o.shortcut.numeric_args[2];
        }
    }
    if matches!(d.humidity_indicator_type, 4 | 7 | 6) {
        if o.shortcut.alpha_field_blanks[5] {
            diagnostics::empty(
                o.backend,
                &d.title,
                &o.shortcut.alpha_field_names[5],
                &o.shortcut.alpha_field_names[2],
                &o.shortcut.alpha_args[2],
            )?;
            *errors = true;
        } else {
            let name = o.shortcut.alpha_args[5].clone();
            d.humidity_indicator_schedule = o.schedules.get_source_day_schedule(&name)?;
            if let Some(day) = d.humidity_indicator_schedule {
                super::schedules::copy_values(
                    modifiers,
                    environment,
                    o.clock.time_steps_in_hour,
                    o.schedules.source_day_values(day)?,
                    super::schedules::ProfileField::Humidity,
                )?;
                use crate::heat_balance::source_output_registration::{
                    SourceOutputUnits as Units, SourceWeatherRealOutputField as Field,
                };
                let units = if d.humidity_indicator_type == 4 {
                    Units::Percentage
                } else {
                    Units::Dimensionless
                };
                super::output::register(
                    numbers,
                    sites,
                    day,
                    Field::SizingPeriodHumiditySchedule { environment },
                    "Sizing Period Site Humidity Condition Schedule Value",
                    &name,
                    units,
                    o,
                )?;
                match d.humidity_indicator_type {
                    4 | 7 => {
                        let maximum = if d.humidity_indicator_type == 4 {
                            100.0
                        } else {
                            1.0
                        };
                        if !o
                            .schedules
                            .source_day_check_min_max(day, 0.0, maximum, o.clock)?
                        {
                            diagnostics::bad_min_max(
                                o.backend,
                                &d.title,
                                &o.shortcut.alpha_field_names[5],
                                &name,
                                0.0,
                                maximum,
                            )?;
                            *errors = true;
                        }
                    }
                    6 => {
                        if !o.schedules.source_day_check_min(day, 0.0, o.clock)? {
                            diagnostics::bad_min(
                                o.backend,
                                &d.title,
                                &o.shortcut.alpha_field_names[5],
                                &name,
                                0.0,
                            )?;
                            *errors = true;
                        }
                    }
                    _ => {}
                }
            } else {
                // Native warning only; the selector and zero constructor modifiers remain.
                diagnostics::missing_humidity(
                    o.backend,
                    &d.title,
                    &o.shortcut.alpha_field_names[5],
                    &name,
                )?;
            }
        }
    } else if d.humidity_indicator_type == 5 {
        fill_default_profile(
            modifiers,
            environment,
            o.clock.time_steps_in_hour,
            o.interpolation,
            true,
        )?;
    }
    if matches!(d.humidity_indicator_type, 1 | 0 | 7 | 5 | 6)
        && d.humidity_indicator_value > d.maximum_dry_bulb
    {
        o.backend
            .show_warning_error(&format!("{FAMILY}=\"{}\", range check data.", d.title))?;
        o.backend.show_continue_error(&format!(
            "..Humidity Indicator Temperature at Max Temperature={} > Max DryBulb={}",
            rounded(d.humidity_indicator_value, 1)?,
            rounded(d.maximum_dry_bulb, 1)?
        ))?;
        o.backend.show_continue_error(&format!(
            "..{}=\"{}\".",
            o.shortcut.alpha_field_names[4], o.shortcut.alpha_args[4]
        ))?;
        o.backend
            .show_continue_error("..Conditions for day will be set to Relative Humidity = 100%")?;
        if d.humidity_indicator_type == 1 {
            d.dew_point_needs_set = true;
        } else {
            d.humidity_indicator_value = d.maximum_dry_bulb;
        }
    }
    Ok(())
}
