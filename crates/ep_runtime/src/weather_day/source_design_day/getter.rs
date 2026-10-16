//! Literal allocation/getter/field-write order from Native5997–6246 and6468–6653.
use super::diagnostics;
use super::state::{SourceDesignDayModifierGrid, SourceSizingPeriodSiteSchedule};
use super::*;
const RANGE_NAMES: [&str; 4] = [
    "DEFAULTMULTIPLIERS",
    "MULTIPLIERSCHEDULE",
    "DIFFERENCESCHEDULE",
    "TEMPERATUREPROFILESCHEDULE",
];
const SOLAR_NAMES: [&str; 5] = [
    "ASHRAECLEARSKY",
    "ZHANGHUANG",
    "SCHEDULE",
    "ASHRAETAU",
    "ASHRAETAU2017",
];
impl SourceDesignDayInputState {
    pub(crate) fn get_source_design_day_data(
        &mut self,
        count: i32,
        o: &mut SourceDesignDayInputOwners<'_, '_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        // Native EPVector::allocate resets every real record, including on retries.
        self.design_day_input = Some(vec![SourceDesignDayData::default(); size(count)?]);
        self.design_day_modifiers = Some(vec![None; size(count)?]);
        for day in 1..=count {
            let steps = size(o.clock.time_steps_in_hour)?;
            let length = steps
                .checked_mul(24)
                .ok_or(Error::Unavailable("actual DesignDay Array2D size"))?;
            let row = self
                .design_day_modifiers
                .as_mut()
                .and_then(|v| v.get_mut(ordinal(day).ok()?))
                .ok_or(Error::Unavailable(
                    "actual DesignDay outer modifier allocation",
                ))?;
            *row = Some(SourceDesignDayModifierGrid {
                time_steps_in_hour: o.clock.time_steps_in_hour,
                values: vec![SourceDesignDayModifiers::default(); length],
            });
        }
        self.sizing_period_site_schedules =
            vec![SourceSizingPeriodSiteSchedule::default(); size(count)?];
        if *o.reverse_design_days && count <= 1 {
            o.backend.show_severe_error(
                "GetDesignDayData: Reverse Design Day requested but # Design Days <=1",
            )?;
        }
        o.shortcut.current_module_object = FAMILY.into();
        for input_day in 1..=count {
            let environment_number = if !*o.reverse_design_days {
                input_day
            } else if input_day == 1 && count > 1 {
                2
            } else if input_day == 2 {
                1
            } else {
                input_day
            };
            // The actual argument is the current shared module string, not a fresh IP.
            let family = o.shortcut.current_module_object.clone();
            let _counts = o.raw.get_shared_source_object_item(
                &family,
                ordinal(input_day)?,
                o.input,
                o.shortcut,
            )?;
            let env = o
                .environments
                .get_mut(ordinal(environment_number)?)
                .ok_or(Error::Unavailable("actual DesignDay Environment row6042"))?;
            let d = self
                .design_day_input
                .as_mut()
                .and_then(|v| v.get_mut(ordinal(environment_number).ok()?))
                .ok_or(Error::Unavailable("actual DesignDay DesDayInput row6043"))?;
            d.title.clone_from(&o.shortcut.alpha_args[0]);
            env.title.clone_from(&d.title);
            d.maximum_dry_bulb = o.shortcut.numeric_args[2];
            let maximum_dry_bulb_entered = !o.shortcut.numeric_field_blanks[2];
            d.daily_dry_bulb_range = o.shortcut.numeric_args[3];
            d.barometric_pressure = o.shortcut.numeric_args[8];
            let pressure_entered = !o.shortcut.numeric_field_blanks[8];
            d.pressure_entered = pressure_entered;
            d.wind_speed = o.shortcut.numeric_args[9];
            d.wind_direction = o.shortcut.numeric_args[10] % 360.0; // Objexx double mod -> fmod.
            d.month = integer(o.shortcut.numeric_args[0])?;
            d.day_of_month = integer(o.shortcut.numeric_args[1])?;
            d.tau_beam = o.shortcut.numeric_args[11];
            d.tau_diffuse = o.shortcut.numeric_args[12];
            d.daily_wet_bulb_range = o.shortcut.numeric_args[7];
            d.sky_clearness = o.shortcut.numeric_args[13];
            d.maximum_warmup_days = if o.shortcut.numeric_field_blanks[14] {
                -1
            } else {
                integer(o.shortcut.numeric_args[14])?
            };
            if o.shortcut.alpha_field_blanks[12]
                || o.shortcut.alpha_args[12].eq_ignore_ascii_case("FullResetAtBeginEnvironment")
            {
                d.suppress_begin_environment_reset = false;
            } else if o.shortcut.alpha_args[12]
                .eq_ignore_ascii_case("SuppressThermalResetAtBeginEnvironment")
            {
                // Preserve Native spelling despite the different allowed IDD key.
                d.suppress_begin_environment_reset = true;
            }
            if *o.force_begin_env_reset_suppress {
                d.suppress_begin_environment_reset = true;
            }
            d.rain_indicator = diagnostics::boolean(
                o.backend,
                &d.title,
                &o.shortcut.alpha_field_names[6],
                &o.shortcut.alpha_args[6],
                o.shortcut.alpha_field_blanks[6],
            )?;
            d.snow_indicator = diagnostics::boolean(
                o.backend,
                &d.title,
                &o.shortcut.alpha_field_names[7],
                &o.shortcut.alpha_args[7],
                o.shortcut.alpha_field_blanks[7],
            )?;
            if o.shortcut.alpha_field_blanks[2] {
                d.dry_bulb_range_type = 0;
            } else {
                d.dry_bulb_range_type =
                    enum_value(&RANGE_NAMES, &o.shortcut.alpha_args[2].to_ascii_uppercase());
                if d.dry_bulb_range_type == -1 {
                    diagnostics::invalid_key(
                        o.backend,
                        &d.title,
                        &o.shortcut.alpha_field_names[2],
                        &o.shortcut.alpha_args[2],
                    )?;
                    *errors = true;
                    d.dry_bulb_range_type = 0;
                }
            }
            if d.dry_bulb_range_type != 3
                && !maximum_dry_bulb_entered
                && o.shortcut.alpha_args[2] != "invalid field"
            {
                diagnostics::empty(
                    o.backend,
                    &d.title,
                    &o.shortcut.numeric_field_names[2],
                    &o.shortcut.alpha_field_names[2],
                    &o.shortcut.alpha_args[2],
                )?;
                *errors = true;
            }
            if d.dry_bulb_range_type != 2 && d.dry_bulb_range_type != 3 {
                let value = d.maximum_dry_bulb - d.daily_dry_bulb_range;
                if value < -90.0 || value > 70.0 {
                    diagnostics::severe_header(o.backend, &d.title)?;
                    o.backend.show_continue_error(&format!(
                        "{} ({}) is out of range [-90.0, 70.0]",
                        o.shortcut.alpha_field_names[2],
                        rounded(value, 2)?
                    ))?;
                    *errors = true;
                }
            }
            if d.dry_bulb_range_type == 0 {
                fill_default_profile(
                    &mut self.design_day_modifiers,
                    environment_number,
                    o.clock.time_steps_in_hour,
                    o.interpolation,
                    false,
                )?;
            } else if o.shortcut.alpha_field_blanks[3] {
                diagnostics::empty(
                    o.backend,
                    &d.title,
                    &o.shortcut.alpha_field_names[3],
                    &o.shortcut.alpha_field_names[2],
                    "SCHEDULE",
                )?;
                *errors = true;
            } else {
                return Err(Error::Unavailable(
                    "GetDesignDayData6178 actual Sched::GetDaySchedule dry-bulb child",
                ));
            }
            super::humidity::read(
                d,
                &mut self.design_day_modifiers,
                environment_number,
                o,
                errors,
            )?;
            if o.shortcut.alpha_field_blanks[9] {
                d.solar_model = 0;
            } else {
                d.solar_model =
                    enum_value(&SOLAR_NAMES, &o.shortcut.alpha_args[9].to_ascii_uppercase());
                if d.solar_model == -1 {
                    diagnostics::warning_key(
                        o.backend,
                        &d.title,
                        &o.shortcut.alpha_field_names[9],
                        &o.shortcut.alpha_args[9],
                        "ASHRAE ClearSky",
                    )?;
                    d.solar_model = 0;
                }
            }
            if d.solar_model == 2 {
                if o.shortcut.alpha_field_blanks[10] {
                    diagnostics::empty(
                        o.backend,
                        &d.title,
                        &o.shortcut.alpha_field_names[10],
                        "",
                        "",
                    )?;
                    *errors = true;
                } else {
                    return Err(Error::Unavailable(
                        "GetDesignDayData6486 actual Sched::GetDaySchedule beam child",
                    ));
                }
                if o.shortcut.alpha_field_blanks[11] {
                    diagnostics::empty(
                        o.backend,
                        &d.title,
                        &o.shortcut.alpha_field_names[11],
                        "",
                        "",
                    )?;
                    *errors = true;
                } else {
                    return Err(Error::Unavailable(
                        "GetDesignDayData6523 actual Sched::GetDaySchedule diffuse child",
                    ));
                }
            } else if d.solar_model == 0 && o.shortcut.numeric_field_blanks[13] {
                // Literal6556 argument order: defaultVal is alpha *field name*,
                // depFieldName is alpha *value*, depFieldVal is the fallback text.
                diagnostics::warning_empty(
                    o.backend,
                    &d.title,
                    &o.shortcut.numeric_field_names[13],
                    &o.shortcut.alpha_field_names[9],
                    &o.shortcut.alpha_args[9],
                    "Zero clear sky (no solar)",
                )?;
            }
            super::validation::calendar_and_day_type(d, o.shortcut, o.backend, errors)?;
            let design_day = o
                .design_day_weather
                .get(ordinal(environment_number)?)
                .ok_or(Error::Unavailable("actual DesignDay earlier4766 row6622"))?;
            let env = o
                .environments
                .get_mut(ordinal(environment_number)?)
                .ok_or(Error::Unavailable("actual DesignDay Environment row6623"))?;
            env.title.clone_from(&d.title);
            env.kind_of_environment = 1; // actual Constant::KindOfSim::DesignDay.
            env.design_day_number = environment_number;
            env.run_period_design_number = 0;
            env.total_days = 1;
            env.start_month = d.month;
            env.start_day = d.day_of_month;
            env.end_month = env.start_month;
            env.end_day = env.start_day;
            env.day_of_week = 0;
            env.use_daylight_saving = false;
            env.use_holidays = false;
            env.start_julian_day = design_day.day_of_year;
            env.end_julian_day = env.start_julian_day;
            super::reports::register(d, o.registry, o.table)?;
        }
        Ok(())
    }
}
pub(super) fn fill_default_profile(
    modifiers: &mut Option<Vec<Option<SourceDesignDayModifierGrid>>>,
    environment: i32,
    steps: i32,
    interpolation: &Option<Vec<f64>>,
    humidity: bool,
) -> Result<(), Error> {
    let mut last_hour_value = DEFAULT_TEMPERATURE_RANGE_MULTIPLIERS[23];
    for hour in 1..=24 {
        for timestep in 1..=steps {
            let now = *interpolation
                .as_ref()
                .and_then(|v| v.get(ordinal(timestep).ok()?))
                .ok_or(Error::Unavailable(
                    "actual DesignDay Interpolation cell6165/6436",
                ))?;
            let previous = 1.0 - now;
            let grid = modifiers
                .as_mut()
                .and_then(|v| v.get_mut(ordinal(environment).ok()?))
                .and_then(Option::as_mut)
                .ok_or(Error::Unavailable(
                    "actual DesignDay per-environment modifier Array2D",
                ))?;
            let cell = grid.cell_mut(timestep, hour)?;
            let value = last_hour_value * previous
                + DEFAULT_TEMPERATURE_RANGE_MULTIPLIERS[ordinal(hour)?] * now;
            if humidity {
                cell.outdoor_relative_humidity = value;
            } else {
                cell.outdoor_dry_bulb_temperature = value;
            }
        }
        last_hour_value = DEFAULT_TEMPERATURE_RANGE_MULTIPLIERS[ordinal(hour)?];
    }
    Ok(())
}
