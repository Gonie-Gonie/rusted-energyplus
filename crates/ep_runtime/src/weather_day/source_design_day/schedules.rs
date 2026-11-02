//! Actual DD6175–6244/6480–6552 day-schedule children on the retained manager.
use super::state::{SourceDesignDayModifierGrid, SourceSizingPeriodSiteSchedule};
use super::*;
use crate::heat_balance::source_output_registration::{
    SourceOutputUnits as Units, SourceWeatherRealOutputField as Field,
};

#[derive(Clone, Copy)]
pub(super) enum ProfileField {
    DryBulb,
    Humidity,
    Beam,
    Diffuse,
}
/// Native getDayVals is hour-major; the modifier Array2D is timestep-major.
pub(super) fn copy_values(
    modifiers: &mut Option<Vec<Option<SourceDesignDayModifierGrid>>>,
    environment: i32,
    steps: i32,
    values: &[f64],
    field: ProfileField,
) -> Result<(), Error> {
    for hour in 1..=24 {
        for timestep in 1..=steps {
            let at = ordinal(hour)?
                .checked_mul(size(steps)?)
                .and_then(|v| v.checked_add(ordinal(timestep).ok()?))
                .ok_or(Error::Unavailable("actual DD dayVals hour/timestep index"))?;
            let value = *values
                .get(at)
                .ok_or(Error::Unavailable("actual DD dayVals cell"))?;
            let grid = modifiers
                .as_mut()
                .and_then(|v| v.get_mut(ordinal(environment).ok()?))
                .and_then(Option::as_mut)
                .ok_or(Error::Unavailable("actual DD selected modifier Array2D"))?;
            let cell = grid.cell_mut(timestep, hour)?;
            match field {
                ProfileField::DryBulb => cell.outdoor_dry_bulb_temperature = value,
                ProfileField::Humidity => cell.outdoor_relative_humidity = value,
                ProfileField::Beam => cell.beam_solar_radiation = value,
                ProfileField::Diffuse => cell.diffuse_solar_radiation = value,
            }
        }
    }
    Ok(())
}

// Preserve Native switch cases and their ordered diagnostic bodies.
#[allow(clippy::too_many_arguments, clippy::collapsible_match)]
pub(super) fn dry_bulb(
    d: &mut SourceDesignDayData,
    modifiers: &mut Option<Vec<Option<SourceDesignDayModifierGrid>>>,
    numbers: &mut Vec<i32>,
    sites: &[SourceSizingPeriodSiteSchedule],
    environment: i32,
    o: &mut SourceDesignDayInputOwners<'_, '_>,
    errors: &mut bool,
) -> Result<(), Error> {
    if o.shortcut.alpha_field_blanks[3] {
        diagnostics::empty(
            o.backend,
            &d.title,
            &o.shortcut.alpha_field_names[3],
            &o.shortcut.alpha_field_names[2],
            "SCHEDULE",
        )?;
        *errors = true;
        return Ok(());
    }
    let name = o.shortcut.alpha_args[3].clone();
    d.temperature_range_schedule = o.schedules.get_source_day_schedule(&name)?;
    let Some(day) = d.temperature_range_schedule else {
        diagnostics::item_not_found(o.backend, &d.title, &o.shortcut.alpha_field_names[3], &name)?;
        *errors = true;
        return Ok(());
    };
    copy_values(
        modifiers,
        environment,
        o.clock.time_steps_in_hour,
        o.schedules.source_day_values(day)?,
        ProfileField::DryBulb,
    )?;
    let units = match d.dry_bulb_range_type {
        1 => Units::Dimensionless,
        2 => Units::DeltaC,
        3 => Units::C,
        _ => return Err(Error::Unavailable("actual DD scheduled dry-bulb selector")),
    };
    super::output::register(
        numbers,
        sites,
        day,
        Field::SizingPeriodDryBulbSchedule { environment },
        "Sizing Period Site Drybulb Temperature Range Modifier Schedule Value",
        &name,
        units,
        o,
    )?;
    match d.dry_bulb_range_type {
        1 => {
            if !o
                .schedules
                .source_day_check_min_max(day, 0.0, 1.0, o.clock)?
            {
                diagnostics::bad_min_max(
                    o.backend,
                    &d.title,
                    &o.shortcut.alpha_field_names[3],
                    &name,
                    0.0,
                    1.0,
                )?;
                *errors = true;
            }
        }
        2 => {
            if !o.schedules.source_day_check_min(day, 0.0, o.clock)? {
                diagnostics::bad_min(
                    o.backend,
                    &d.title,
                    &o.shortcut.alpha_field_names[3],
                    &name,
                    0.0,
                )?;
                *errors = true;
            }
        }
        _ => {}
    }
    // Native numeric_limits<Real64>::min() is the smallest POSITIVE value.
    let mut test_value = f64::MIN_POSITIVE;
    let grid = modifiers
        .as_ref()
        .and_then(|v| v.get(ordinal(environment).ok()?))
        .and_then(Option::as_ref)
        .ok_or(Error::Unavailable(
            "actual DD scheduled maximum modifier Array2D",
        ))?;
    for hour in 1..=24 {
        for timestep in 1..=o.clock.time_steps_in_hour {
            let value = grid.cell(timestep, hour)?.outdoor_dry_bulb_temperature;
            if value > test_value {
                test_value = value;
            }
        }
    }
    if d.dry_bulb_range_type == 3 {
        if !o.shortcut.numeric_field_blanks[2] {
            o.backend
                .show_warning_error(&format!("{FAMILY}=\"{}\", data override.", d.title))?;
            o.backend.show_continue_error(&format!(
                "..{}=[{}] will be overwritten.",
                o.shortcut.numeric_field_names[2],
                rounded(d.maximum_dry_bulb, 2)?
            ))?;
            o.backend.show_continue_error(&format!(
                "..{}=\"{}\".",
                o.shortcut.alpha_field_names[2], o.shortcut.alpha_args[2]
            ))?;
            o.backend
                .show_continue_error(&format!("..with max value=[{}].", rounded(test_value, 2)?))?;
        }
        d.maximum_dry_bulb = test_value;
    }
    test_value = d.maximum_dry_bulb - test_value;
    #[allow(clippy::manual_range_contains)]
    if test_value < -90.0 || test_value > 70.0 {
        diagnostics::severe_header(o.backend, &d.title)?;
        o.backend.show_continue_error(&format!(
            "{} = ({}) is out of range [-90.0, 70.0]",
            o.shortcut.alpha_field_names[3],
            rounded(test_value, 2)?
        ))?;
        *errors = true;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn solar(
    d: &mut SourceDesignDayData,
    modifiers: &mut Option<Vec<Option<SourceDesignDayModifierGrid>>>,
    numbers: &mut Vec<i32>,
    sites: &[SourceSizingPeriodSiteSchedule],
    environment: i32,
    o: &mut SourceDesignDayInputOwners<'_, '_>,
    errors: &mut bool,
) -> Result<(), Error> {
    for (alpha, pointer, profile, field, label) in [
        (
            10,
            &mut d.beam_solar_schedule,
            ProfileField::Beam,
            Field::SizingPeriodBeamSolarSchedule { environment },
            "Sizing Period Site Beam Solar Schedule Value",
        ),
        (
            11,
            &mut d.diffuse_solar_schedule,
            ProfileField::Diffuse,
            Field::SizingPeriodDiffuseSolarSchedule { environment },
            "Sizing Period Site Diffuse Solar Schedule Value",
        ),
    ] {
        if o.shortcut.alpha_field_blanks[alpha] {
            diagnostics::empty(
                o.backend,
                &d.title,
                &o.shortcut.alpha_field_names[alpha],
                "",
                "",
            )?;
            *errors = true;
            continue;
        }
        let name = o.shortcut.alpha_args[alpha].clone();
        *pointer = o.schedules.get_source_day_schedule(&name)?;
        let Some(day) = *pointer else {
            diagnostics::item_not_found(
                o.backend,
                &d.title,
                &o.shortcut.alpha_field_names[alpha],
                &name,
            )?;
            *errors = true;
            continue;
        };
        copy_values(
            modifiers,
            environment,
            o.clock.time_steps_in_hour,
            o.schedules.source_day_values(day)?,
            profile,
        )?;
        super::output::register(numbers, sites, day, field, label, &name, Units::WPerM2, o)?;
        if !o.schedules.source_day_check_min(day, 0.0, o.clock)? {
            diagnostics::bad_min(
                o.backend,
                &d.title,
                &o.shortcut.alpha_field_names[alpha],
                &name,
                0.0,
            )?;
            *errors = true;
        }
    }
    Ok(())
}
