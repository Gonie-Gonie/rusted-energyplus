//! Schedule lookup and internal-gain trace helpers.

use crate::error::RuntimeError;
use crate::geometry::zone_floor_area_m2;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use crate::time_axis::{DayType, EnvironmentTimeAxis, EnvironmentTimePoint, TimeAxis, TimePoint};
use ep_model::{
    OtherEquipment, OtherEquipmentDesignLevelCalculationMethod, People,
    PeopleNumberCalculationMethod, ScheduleCompact, ScheduleCompactDayProfile,
    ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleConstant, ScheduleDayType, ScheduleId,
    TypedModel, ZoneId,
};
use std::collections::BTreeSet;

/// EnergyPlus ESO variable for zone total internal convective sensible gains.
pub const ZONE_TOTAL_INTERNAL_CONVECTIVE_HEATING_RATE_VARIABLE: &str =
    "Zone Total Internal Convective Heating Rate";
/// EnergyPlus ESO variable for zone total internal radiant sensible gains.
pub const ZONE_TOTAL_INTERNAL_RADIANT_HEATING_RATE_VARIABLE: &str =
    "Zone Total Internal Radiant Heating Rate";

pub(crate) fn internal_gain_w(model: &TypedModel, zone_id: ZoneId, hour_ending: u32) -> f64 {
    model
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone_id)
        .map(|equipment| internal_gain_for_equipment_w(model, equipment, hour_ending))
        .sum()
}

fn internal_gain_for_equipment_w(
    model: &TypedModel,
    equipment: &OtherEquipment,
    hour_ending: u32,
) -> f64 {
    let schedule_multiplier = hour_only_schedule_multiplier(model, equipment.schedule, hour_ending);
    let sensible_fraction = (1.0 - equipment.fraction_latent - equipment.fraction_lost).max(0.0);

    other_equipment_design_level_w(model, equipment) * schedule_multiplier * sensible_fraction
}

pub(crate) fn convective_internal_gain_w(
    model: &TypedModel,
    zone_id: ZoneId,
    hour_ending: u32,
) -> f64 {
    model
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone_id)
        .map(|equipment| convective_internal_gain_for_equipment_w(model, equipment, hour_ending))
        .sum()
}

fn convective_internal_gain_for_equipment_w(
    model: &TypedModel,
    equipment: &OtherEquipment,
    hour_ending: u32,
) -> f64 {
    let schedule_multiplier = hour_only_schedule_multiplier(model, equipment.schedule, hour_ending);
    let convective_fraction =
        (1.0 - equipment.fraction_latent - equipment.fraction_radiant - equipment.fraction_lost)
            .max(0.0);

    other_equipment_design_level_w(model, equipment) * schedule_multiplier * convective_fraction
}

fn radiant_internal_gain_w(model: &TypedModel, zone_id: ZoneId, hour_ending: u32) -> f64 {
    model
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone_id)
        .map(|equipment| radiant_internal_gain_for_equipment_w(model, equipment, hour_ending))
        .sum()
}

fn radiant_internal_gain_for_equipment_w(
    model: &TypedModel,
    equipment: &OtherEquipment,
    hour_ending: u32,
) -> f64 {
    let schedule_multiplier = hour_only_schedule_multiplier(model, equipment.schedule, hour_ending);
    let radiant_fraction = equipment.fraction_radiant.max(0.0);

    other_equipment_design_level_w(model, equipment) * schedule_multiplier * radiant_fraction
}

fn hour_only_schedule_multiplier(
    model: &TypedModel,
    schedule_id: Option<ScheduleId>,
    hour_ending: u32,
) -> f64 {
    let Some(schedule_id) = schedule_id else {
        return 1.0;
    };
    schedule_value(model, schedule_id, hour_ending).unwrap_or(f64::NAN)
}

pub(crate) fn validate_hour_only_internal_gain_schedules(
    model: &TypedModel,
) -> Result<(), RuntimeError> {
    for equipment in &model.other_equipment {
        let Some(schedule_id) = equipment.schedule else {
            continue;
        };
        if model
            .schedules
            .iter()
            .any(|schedule| schedule.id == schedule_id)
        {
            continue;
        }

        let reason = match model
            .compact_schedules
            .iter()
            .find(|schedule| schedule.id == schedule_id)
        {
            Some(schedule) => match hour_only_single_period_compact_schedule_segments(schedule) {
                Ok(_) => continue,
                Err(reason) => reason,
            },
            None => format!("schedule ID {} is unresolved", schedule_id.0),
        };
        return Err(RuntimeError::InvalidInternalGainSchedule {
            equipment_name: equipment.name.0.clone(),
            schedule_id: schedule_id.0,
            reason,
        });
    }
    Ok(())
}

fn other_equipment_design_level_w(model: &TypedModel, equipment: &OtherEquipment) -> f64 {
    match equipment.design_level_calculation_method {
        OtherEquipmentDesignLevelCalculationMethod::EquipmentLevel => equipment.design_level_w,
        OtherEquipmentDesignLevelCalculationMethod::WattsPerZoneFloorArea => model
            .zones
            .iter()
            .find(|zone| zone.id == equipment.zone)
            .map(|zone| equipment.power_per_floor_area_w_per_m2 * zone_floor_area_m2(model, zone))
            .unwrap_or(0.0),
        OtherEquipmentDesignLevelCalculationMethod::WattsPerPerson => {
            equipment.power_per_person_w * zone_people_design_count(model, equipment.zone)
        }
    }
}

fn zone_people_design_count(model: &TypedModel, zone_id: ZoneId) -> f64 {
    let Some(zone) = model.zones.iter().find(|zone| zone.id == zone_id) else {
        return 0.0;
    };
    let zone_floor_area_m2 = zone_floor_area_m2(model, zone);
    model
        .people
        .iter()
        .filter(|people| people.zone == zone_id)
        .map(|people| people_design_count(people, zone_floor_area_m2))
        .sum()
}

fn people_design_count(people: &People, zone_floor_area_m2: f64) -> f64 {
    match people.number_of_people_calculation_method {
        PeopleNumberCalculationMethod::People => people.number_of_people.max(0.0),
        PeopleNumberCalculationMethod::PeoplePerArea => {
            (people.people_per_floor_area * zone_floor_area_m2).max(0.0)
        }
        PeopleNumberCalculationMethod::AreaPerPerson => {
            if people.floor_area_per_person > 0.0 {
                (zone_floor_area_m2 / people.floor_area_per_person).max(0.0)
            } else {
                0.0
            }
        }
    }
}

pub(crate) fn update_surface_radiant_internal_gain_source_terms(
    model: &TypedModel,
    surfaces: &mut [SurfaceHeatBalanceState],
    hour_ending: u32,
) {
    for surface in surfaces.iter_mut() {
        surface.inside_radiant_internal_gain_w_per_m2 = 0.0;
    }

    let zone_ids = surfaces
        .iter()
        .map(|surface| surface.zone_id)
        .collect::<BTreeSet<_>>();
    for zone_id in zone_ids {
        let radiant_gain_w = radiant_internal_gain_w(model, zone_id, hour_ending);
        if radiant_gain_w <= 0.0 {
            continue;
        }
        let area_absorptance_sum_m2 = surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone_id)
            .map(|surface| surface.area_m2 * surface.inside_thermal_absorptance.max(0.0))
            .sum::<f64>();
        if area_absorptance_sum_m2 <= 0.0 {
            continue;
        }
        let thermal_absorptance_multiplier = radiant_gain_w / area_absorptance_sum_m2;
        for surface in surfaces
            .iter_mut()
            .filter(|surface| surface.zone_id == zone_id)
        {
            surface.inside_radiant_internal_gain_w_per_m2 =
                thermal_absorptance_multiplier * surface.inside_thermal_absorptance.max(0.0);
        }
    }
}

pub(crate) fn schedule_value(
    model: &TypedModel,
    schedule_id: ScheduleId,
    hour_ending: u32,
) -> Option<f64> {
    if let Some(schedule) = model
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
    {
        return Some(schedule.hourly_value);
    }

    let minute_of_day = hour_ending.clamp(1, 24) * 60;
    model
        .compact_schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .and_then(|schedule| {
            hour_only_single_period_compact_schedule_segments(schedule)
                .ok()
                .and_then(|segments| compact_schedule_value(segments, minute_of_day))
        })
}

fn compact_schedule_value(segments: &[ScheduleCompactSegment], minute_of_day: u32) -> Option<f64> {
    segments
        .iter()
        .find(|segment| minute_of_day <= segment.until_minute_of_day)
        .map(|segment| segment.value)
        .or_else(|| segments.last().map(|segment| segment.value))
}

/// One sampled schedule output series.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleTrace {
    /// Typed schedule ID.
    pub schedule_id: ScheduleId,
    /// EnergyPlus-normalized schedule name.
    pub schedule_name: String,
    /// Compile-time schedule representation used to generate this series.
    pub kind: ScheduleSeriesKind,
    /// Sampled schedule values.
    pub values: Vec<f64>,
}

/// Precomputed schedule values for one schedule.
pub type ScheduleValueSeries = ScheduleTrace;

/// Compile-time schedule representation used by runtime schedule series.
#[derive(Clone, Debug, PartialEq)]
pub enum ScheduleSeriesKind {
    /// Schedule:Constant scalar fast path.
    ConstantScalar {
        /// Constant value reused for every timestep.
        value: f64,
    },
    /// Schedule:Compact intervals precompiled from Until segments.
    CompactIntervals {
        /// Ordered daily intervals in minute-of-day space.
        intervals: Vec<CompiledScheduleInterval>,
    },
    /// Schedule:Compact annual periods and day-type profiles.
    CompactCalendarProfiles {
        /// Source-ordered annual periods compiled for calendar-aware lookup.
        periods: Vec<CompiledSchedulePeriod>,
    },
}

/// One precompiled daily schedule interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompiledScheduleInterval {
    /// First minute covered by this interval, inclusive.
    pub start_minute_of_day: u32,
    /// Last minute covered by this interval, inclusive.
    pub end_minute_of_day: u32,
    /// Interval schedule value.
    pub value: f64,
}

/// One compiled day-type profile within a compact-schedule period.
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledScheduleDayProfile {
    /// Expanded schedule day types that consume these intervals.
    pub day_types: Vec<ScheduleDayType>,
    /// Ordered daily intervals in minute-of-day space.
    pub intervals: Vec<CompiledScheduleInterval>,
}

/// One compiled annual period in a compact schedule.
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledSchedulePeriod {
    /// Inclusive leap-shaped schedule ordinal at which this period ends.
    pub through_schedule_day_of_year: u16,
    /// Source-ordered day-type profiles within this period.
    pub day_profiles: Vec<CompiledScheduleDayProfile>,
}

/// One sampled zone internal-gain output series.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneInternalGainTrace {
    /// Typed zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Sampled convective internal gain values in W.
    pub values_w: Vec<f64>,
}

/// Simulates constant schedules for a fixed number of samples.
#[must_use]
pub fn simulate_constant_schedules(model: &TypedModel, sample_count: usize) -> Vec<ScheduleTrace> {
    model
        .schedules
        .iter()
        .map(|schedule| ScheduleTrace {
            schedule_id: schedule.id,
            schedule_name: schedule.name.0.clone(),
            kind: ScheduleSeriesKind::ConstantScalar {
                value: schedule.hourly_value,
            },
            values: vec![schedule.hourly_value; sample_count],
        })
        .collect()
}

/// Simulates constant and calendar-invariant compact schedules for hourly samples.
pub fn simulate_schedule_values(
    model: &TypedModel,
    sample_count: usize,
) -> Result<Vec<ScheduleTrace>, String> {
    precompute_schedule_value_series(model, sample_count)
}

/// Precomputes constant and calendar-invariant compact schedules for hourly samples.
///
/// Calendar-varying compact schedules require a TimeAxis and are rejected by
/// this hour-only API rather than being evaluated against an implicit day type.
pub fn precompute_schedule_value_series(
    model: &TypedModel,
    sample_count: usize,
) -> Result<Vec<ScheduleValueSeries>, String> {
    let hours = (0..sample_count).map(|index| u32::try_from(index % 24 + 1).unwrap_or(24));
    precompute_schedule_value_series_for_hours(model, hours)
}

/// Precomputes constant and supported compact schedules for a run-period time axis.
#[must_use]
pub fn precompute_schedule_value_series_for_time_axis(
    model: &TypedModel,
    time_axis: &TimeAxis,
) -> Vec<ScheduleValueSeries> {
    model
        .schedules
        .iter()
        .map(|schedule| {
            constant_schedule_series(schedule, time_axis.points.iter().map(|point| point.hour))
        })
        .chain(
            model
                .compact_schedules
                .iter()
                .map(|schedule| compact_schedule_series_for_time_axis(schedule, time_axis)),
        )
        .collect()
}

/// Precomputes constant and supported compact schedules for every zone timestep
/// in one simulation environment.
///
/// Compact schedules currently use EnergyPlus' no-interpolation endpoint
/// semantics: each zone timestep reads the schedule value at that timestep's
/// ending minute. The current DST state shifts only the lookup hour, preserving
/// the one-based zone-timestep position within that hour.
#[must_use]
pub fn precompute_schedule_value_series_for_environment_time_axis(
    model: &TypedModel,
    time_axis: &EnvironmentTimeAxis,
) -> Vec<ScheduleValueSeries> {
    model
        .schedules
        .iter()
        .map(|schedule| {
            constant_schedule_series(schedule, time_axis.points.iter().map(|point| point.hour))
        })
        .chain(
            model.compact_schedules.iter().map(|schedule| {
                compact_schedule_series_for_environment_time_axis(schedule, time_axis)
            }),
        )
        .collect()
}

fn precompute_schedule_value_series_for_hours(
    model: &TypedModel,
    hours: impl IntoIterator<Item = u32> + Clone,
) -> Result<Vec<ScheduleValueSeries>, String> {
    let constants = model
        .schedules
        .iter()
        .map(|schedule| constant_schedule_series(schedule, hours.clone()))
        .collect::<Vec<_>>();
    let compact = model
        .compact_schedules
        .iter()
        .map(|schedule| compact_schedule_series_for_hours(schedule, hours.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(constants.into_iter().chain(compact).collect())
}

fn constant_schedule_series(
    schedule: &ScheduleConstant,
    hours: impl IntoIterator<Item = u32>,
) -> ScheduleValueSeries {
    let values = hours
        .into_iter()
        .map(|_hour| schedule.hourly_value)
        .collect();
    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::ConstantScalar {
            value: schedule.hourly_value,
        },
        values,
    }
}

fn compact_schedule_series_for_hours(
    schedule: &ScheduleCompact,
    hours: impl IntoIterator<Item = u32>,
) -> Result<ScheduleValueSeries, String> {
    let segments = hour_only_single_period_compact_schedule_segments(schedule)?;
    let intervals = precompile_compact_schedule_intervals(segments);
    let values = hours
        .into_iter()
        .map(|hour_ending| {
            let minute_of_day = hour_ending.clamp(1, 24) * 60;
            compact_interval_value(&intervals, minute_of_day).unwrap_or(0.0)
        })
        .collect();

    Ok(ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::CompactIntervals { intervals },
        values,
    })
}

fn compact_schedule_series_for_time_axis(
    schedule: &ScheduleCompact,
    time_axis: &TimeAxis,
) -> ScheduleValueSeries {
    let periods = precompile_compact_schedule_periods(schedule);
    let values = time_axis
        .points
        .iter()
        .map(|point| {
            let (schedule_day_of_year, day_type, minute_of_day) =
                detailed_schedule_lookup_state(point);
            compiled_compact_schedule_value(&periods, schedule_day_of_year, day_type, minute_of_day)
                .unwrap_or(f64::NAN)
        })
        .collect();

    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::CompactCalendarProfiles { periods },
        values,
    }
}

fn compact_schedule_series_for_environment_time_axis(
    schedule: &ScheduleCompact,
    time_axis: &EnvironmentTimeAxis,
) -> ScheduleValueSeries {
    let periods = precompile_compact_schedule_periods(schedule);
    let values = time_axis
        .points
        .iter()
        .map(|point| {
            let (schedule_day_of_year, day_type, minute_of_day) =
                detailed_schedule_environment_lookup_state(point);
            compiled_compact_schedule_value(&periods, schedule_day_of_year, day_type, minute_of_day)
                .unwrap_or(f64::NAN)
        })
        .collect();

    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::CompactCalendarProfiles { periods },
        values,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DetailedScheduleLookupInput {
    schedule_day_of_year: u32,
    current_day_type: ScheduleDayType,
    tomorrow_day_type: ScheduleDayType,
    dst: bool,
    hour: u32,
    timestep_end_minute: u32,
}

fn detailed_schedule_lookup_state(point: &TimePoint) -> (u32, ScheduleDayType, u32) {
    detailed_schedule_lookup_state_from_input(DetailedScheduleLookupInput {
        schedule_day_of_year: point.schedule_day_of_year,
        current_day_type: model_schedule_day_type(point.day_type),
        tomorrow_day_type: model_schedule_day_type(
            point
                .tomorrow_special_day_type
                .unwrap_or_else(|| point.tomorrow_day_of_week.into()),
        ),
        dst: point.dst,
        hour: point.hour,
        timestep_end_minute: 60,
    })
}

fn detailed_schedule_environment_lookup_state(
    point: &EnvironmentTimePoint,
) -> (u32, ScheduleDayType, u32) {
    detailed_schedule_lookup_state_from_input(DetailedScheduleLookupInput {
        schedule_day_of_year: point.schedule_day_of_year,
        current_day_type: model_schedule_day_type(point.day_type),
        tomorrow_day_type: model_schedule_day_type(
            point
                .tomorrow_special_day_type
                .unwrap_or_else(|| point.tomorrow_day_of_week.into()),
        ),
        dst: point.dst,
        hour: point.hour,
        timestep_end_minute: point.end_minute.round().clamp(1.0, 60.0) as u32,
    })
}

fn detailed_schedule_lookup_state_from_input(
    input: DetailedScheduleLookupInput,
) -> (u32, ScheduleDayType, u32) {
    let mut lookup_hour = input.hour.clamp(1, 24) + u32::from(input.dst);
    let mut schedule_day_of_year = input.schedule_day_of_year;
    let mut day_type = input.current_day_type;

    if lookup_hour > 24 {
        lookup_hour -= 24;
        schedule_day_of_year = if schedule_day_of_year >= 366 {
            1
        } else {
            schedule_day_of_year + 1
        };
        day_type = input.tomorrow_day_type;
    }

    let minute_of_day = (lookup_hour - 1) * 60 + input.timestep_end_minute;
    (schedule_day_of_year, day_type, minute_of_day)
}

/// Returns the shared daily profile accepted by hour-only schedule consumers.
///
/// Such consumers can evaluate only a single annual period ending on schedule
/// ordinal 366 whose twelve day types all resolve to equivalent Until segments.
pub fn hour_only_single_period_compact_schedule_segments(
    schedule: &ScheduleCompact,
) -> Result<&[ScheduleCompactSegment], String> {
    if schedule.periods.len() != 1 {
        return Err(format!(
            "Schedule:Compact {} has {} Through periods; hour-only consumers require one",
            schedule.name.0,
            schedule.periods.len()
        ));
    }
    let period = &schedule.periods[0];
    if period.through_schedule_day_of_year != 366 {
        return Err(format!(
            "Schedule:Compact {} ends at schedule ordinal {}; hour-only consumers require 366",
            schedule.name.0, period.through_schedule_day_of_year
        ));
    }

    let mut baseline: Option<&[ScheduleCompactSegment]> = None;
    for day_type in ALL_SCHEDULE_DAY_TYPES {
        let Some(profile) = unique_day_profile(period, day_type) else {
            return Err(format!(
                "Schedule:Compact {} must assign day type {day_type:?} exactly once for hour-only consumption",
                schedule.name.0
            ));
        };
        if let Some(expected) = baseline {
            if profile.segments.as_slice() != expected {
                return Err(format!(
                    "Schedule:Compact {} varies by day type; hour-only consumers require equivalent profiles",
                    schedule.name.0
                ));
            }
        } else {
            baseline = Some(&profile.segments);
        }
    }

    baseline.ok_or_else(|| {
        format!(
            "Schedule:Compact {} has no calendar-invariant day profile",
            schedule.name.0
        )
    })
}

/// Precompiles all annual periods and day-type profiles in one compact schedule.
#[must_use]
pub fn precompile_compact_schedule_periods(
    schedule: &ScheduleCompact,
) -> Vec<CompiledSchedulePeriod> {
    schedule
        .periods
        .iter()
        .map(|period| CompiledSchedulePeriod {
            through_schedule_day_of_year: period.through_schedule_day_of_year,
            day_profiles: period
                .day_profiles
                .iter()
                .map(|profile| CompiledScheduleDayProfile {
                    day_types: profile.day_types.clone(),
                    intervals: precompile_compact_schedule_intervals(&profile.segments),
                })
                .collect(),
        })
        .collect()
}

const ALL_SCHEDULE_DAY_TYPES: [ScheduleDayType; 12] = [
    ScheduleDayType::Sunday,
    ScheduleDayType::Monday,
    ScheduleDayType::Tuesday,
    ScheduleDayType::Wednesday,
    ScheduleDayType::Thursday,
    ScheduleDayType::Friday,
    ScheduleDayType::Saturday,
    ScheduleDayType::Holiday,
    ScheduleDayType::SummerDesignDay,
    ScheduleDayType::WinterDesignDay,
    ScheduleDayType::CustomDay1,
    ScheduleDayType::CustomDay2,
];

fn model_schedule_day_type(day_type: DayType) -> ScheduleDayType {
    match day_type {
        DayType::Sunday => ScheduleDayType::Sunday,
        DayType::Monday => ScheduleDayType::Monday,
        DayType::Tuesday => ScheduleDayType::Tuesday,
        DayType::Wednesday => ScheduleDayType::Wednesday,
        DayType::Thursday => ScheduleDayType::Thursday,
        DayType::Friday => ScheduleDayType::Friday,
        DayType::Saturday => ScheduleDayType::Saturday,
        DayType::Holiday => ScheduleDayType::Holiday,
        DayType::SummerDesignDay => ScheduleDayType::SummerDesignDay,
        DayType::WinterDesignDay => ScheduleDayType::WinterDesignDay,
        DayType::CustomDay1 => ScheduleDayType::CustomDay1,
        DayType::CustomDay2 => ScheduleDayType::CustomDay2,
    }
}

fn unique_day_profile(
    period: &ScheduleCompactPeriod,
    day_type: ScheduleDayType,
) -> Option<&ScheduleCompactDayProfile> {
    let mut matches = period
        .day_profiles
        .iter()
        .filter(|profile| profile.day_types.contains(&day_type));
    let profile = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    Some(profile)
}

fn compiled_compact_schedule_value(
    periods: &[CompiledSchedulePeriod],
    schedule_day_of_year: u32,
    day_type: ScheduleDayType,
    minute_of_day: u32,
) -> Option<f64> {
    periods
        .iter()
        .find(|period| schedule_day_of_year <= u32::from(period.through_schedule_day_of_year))
        .and_then(|period| {
            period
                .day_profiles
                .iter()
                .find(|profile| profile.day_types.contains(&day_type))
        })
        .and_then(|profile| compact_interval_value(&profile.intervals, minute_of_day))
}

/// Precompiles Schedule:Compact Until segments into closed daily intervals.
#[must_use]
pub fn precompile_compact_schedule_intervals(
    segments: &[ScheduleCompactSegment],
) -> Vec<CompiledScheduleInterval> {
    let mut start_minute_of_day = 1;
    segments
        .iter()
        .map(|segment| {
            let end_minute_of_day = segment.until_minute_of_day.clamp(start_minute_of_day, 1440);
            let interval = CompiledScheduleInterval {
                start_minute_of_day,
                end_minute_of_day,
                value: segment.value,
            };
            start_minute_of_day = end_minute_of_day.saturating_add(1).min(1440);
            interval
        })
        .collect()
}

fn compact_interval_value(
    intervals: &[CompiledScheduleInterval],
    minute_of_day: u32,
) -> Option<f64> {
    let minute = minute_of_day.clamp(1, 1440);
    intervals
        .iter()
        .find(|interval| {
            minute >= interval.start_minute_of_day && minute <= interval.end_minute_of_day
        })
        .map(|interval| interval.value)
        .or_else(|| intervals.last().map(|interval| interval.value))
}

/// Simulates zone total internal convective heating rates for hourly samples.
///
/// Calendar-varying or unresolved schedules are rejected explicitly because
/// this API has no calendar axis with which to evaluate them.
pub fn simulate_zone_internal_convective_gains(
    model: &TypedModel,
    sample_count: usize,
) -> Result<Vec<ZoneInternalGainTrace>, RuntimeError> {
    validate_hour_only_internal_gain_schedules(model)?;
    Ok(model
        .zones
        .iter()
        .map(|zone| {
            let values_w = (0..sample_count)
                .map(|index| {
                    let hour_ending = u32::try_from(index % 24 + 1).unwrap_or(24);
                    convective_internal_gain_w(model, zone.id, hour_ending)
                })
                .collect();
            ZoneInternalGainTrace {
                zone_id: zone.id,
                zone_name: zone.name.0.clone(),
                values_w,
            }
        })
        .collect())
}

/// Simulates zone total internal radiant heating rates for hourly samples.
///
/// Calendar-varying or unresolved schedules are rejected explicitly because
/// this API has no calendar axis with which to evaluate them.
pub fn simulate_zone_internal_radiant_gains(
    model: &TypedModel,
    sample_count: usize,
) -> Result<Vec<ZoneInternalGainTrace>, RuntimeError> {
    validate_hour_only_internal_gain_schedules(model)?;
    Ok(model
        .zones
        .iter()
        .map(|zone| ZoneInternalGainTrace {
            zone_id: zone.id,
            zone_name: zone.name.0.clone(),
            values_w: (0..sample_count)
                .map(|index| {
                    let hour_ending = u32::try_from(index % 24 + 1).unwrap_or(24);
                    radiant_internal_gain_w(model, zone.id, hour_ending)
                })
                .collect(),
        })
        .collect())
}
