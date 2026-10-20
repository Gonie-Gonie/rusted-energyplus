use ep_model::{
    ScheduleCompact, ScheduleCompactDayProfile, ScheduleUnitType, TypedModel,
    populate_schedule_day_values,
};

use super::{ALL_SCHEDULE_DAY_TYPES, Compiler, schedule_minutes_per_timestep};

pub(super) fn parse_schedule_unit_type(value: &str) -> Option<ScheduleUnitType> {
    match value.to_ascii_uppercase().as_str() {
        "DIMENSIONLESS" => Some(ScheduleUnitType::Dimensionless),
        "TEMPERATURE" => Some(ScheduleUnitType::Temperature),
        "DELTATEMPERATURE" => Some(ScheduleUnitType::DeltaTemperature),
        "PRECIPITATIONRATE" => Some(ScheduleUnitType::PrecipitationRate),
        "ANGLE" => Some(ScheduleUnitType::Angle),
        "CONVECTIONCOEFFICIENT" => Some(ScheduleUnitType::ConvectionCoefficient),
        "ACTIVITYLEVEL" => Some(ScheduleUnitType::ActivityLevel),
        "VELOCITY" => Some(ScheduleUnitType::Velocity),
        "CAPACITY" => Some(ScheduleUnitType::Capacity),
        "POWER" => Some(ScheduleUnitType::Power),
        "AVAILABILITY" => Some(ScheduleUnitType::Availability),
        "PERCENT" => Some(ScheduleUnitType::Percent),
        "CONTROL" => Some(ScheduleUnitType::Control),
        "MODE" => Some(ScheduleUnitType::Mode),
        _ => None,
    }
}

impl Compiler<'_> {
    pub(super) fn validate_compact_schedule_type_limits(&mut self, model: &TypedModel) {
        let minutes = schedule_minutes_per_timestep(model.timestep.number_of_timesteps_per_hour);
        for schedule in &model.compact_schedules {
            let Some(limits) = schedule
                .schedule_type_limits
                .and_then(|id| model.schedule_type_limits.get(id.0 as usize))
            else {
                continue;
            };
            let (Some(lower), Some(upper)) = (limits.lower_limit, limits.upper_limit) else {
                continue;
            };
            let Some((minimum, maximum)) = compact_schedule_extrema(schedule, minutes) else {
                continue;
            };
            // Match ScheduleBase::checkMinMaxVals: subtract first, then compare FLT_EPSILON.
            let tolerance = f64::from(f32::EPSILON);
            if !(tolerance >= lower - minimum && maximum - upper <= tolerance) {
                self.error(
                    "ScheduleValueOutsideTypeLimits",
                    "Schedule:Compact",
                    Some(&schedule.name.0),
                    Some("data"),
                    format!(
                        "Schedule:Compact/{} populated timestep range [{minimum}, {maximum}] is outside ScheduleTypeLimits/{} inclusive range [{lower}, {upper}] with f32 epsilon tolerance",
                        schedule.name.0, limits.name.0
                    ),
                );
            }
        }
    }
}

fn populated_day_extrema(
    profile: &ScheduleCompactDayProfile,
    minutes: Option<u32>,
) -> Option<(f64, f64)> {
    let (_, values) =
        populate_schedule_day_values(profile.interpolation, &profile.segments, minutes);
    let first = *values.first()?;
    let (mut minimum, mut maximum) = (first, first);
    for value in values {
        if value < minimum {
            minimum = value;
        } else if value > maximum {
            maximum = value;
        }
    }
    Some((minimum, maximum))
}

fn extend_extrema(accumulated: &mut Option<(f64, f64)>, incoming: (f64, f64)) {
    if let Some((minimum, maximum)) = accumulated {
        if incoming.0 < *minimum {
            *minimum = incoming.0;
        }
        if incoming.1 > *maximum {
            *maximum = incoming.1;
        }
    } else {
        *accumulated = Some(incoming);
    }
}

fn compact_schedule_extrema(
    schedule: &ScheduleCompact,
    minutes: Option<u32>,
) -> Option<(f64, f64)> {
    let mut annual = None;
    let mut first_day = 1_u16;
    let mut covered_end = 0_u16;
    let mut full_year_set = false;
    for period in &schedule.periods {
        let end = period.through_schedule_day_of_year;
        if !(1..=366).contains(&end) || (end == 366 && full_year_set) {
            return None;
        }
        full_year_set |= end == 366;
        // A genuine empty Through still advances StartPointer. Its Day/Week
        // owners exist, but ScheduleDetailed::setMinMaxVals3307-3346 reaches
        // only annual pointers; unused profile values must not enter extrema.
        if first_day <= end {
            if first_day != covered_end + 1 {
                return None;
            }
            let day_extrema = period
                .day_profiles
                .iter()
                .map(|profile| populated_day_extrema(profile, minutes))
                .collect::<Vec<_>>();
            let mut week = None;
            for day_type in ALL_SCHEDULE_DAY_TYPES {
                let index = period
                    .day_profiles
                    .iter()
                    .position(|profile| profile.day_types.contains(&day_type))?;
                extend_extrema(&mut week, day_extrema[index]?);
            }
            extend_extrema(&mut annual, week?);
            covered_end = end;
        }
        first_day = end + 1;
    }
    // The parser retains every authored period, including trailing empty rows.
    // Complete valid coverage is independent of the final authored endpoint.
    if covered_end != 366 {
        return None;
    }
    annual
}
