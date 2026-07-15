//! Schedule lookup and internal-gain trace helpers.

use crate::error::RuntimeError;
use crate::geometry::zone_floor_area_m2;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use crate::time_axis::{DayType, EnvironmentTimeAxis, EnvironmentTimePoint, TimeAxis, TimePoint};
use ep_model::{
    OtherEquipment, OtherEquipmentDesignLevelCalculationMethod, People,
    PeopleNumberCalculationMethod, ScheduleCompact, ScheduleCompactDayProfile,
    ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleDayType, ScheduleFile, ScheduleId,
    ScheduleInterpolation, TypedModel, ZoneId,
};
use std::collections::BTreeSet;

mod cache;
mod constant;
mod day_table;
mod external_interface;
mod file_shading;

pub use cache::{
    CachedScheduleSeries, ScheduleCacheProfile, ScheduleSampleIter, ScheduleSampleStorage,
    ScheduleSeriesCache, ScheduleSeriesIndexKind, precompute_schedule_cache,
    precompute_schedule_cache_for_environment_time_axis, precompute_schedule_cache_for_time_axis,
};
pub use constant::simulate_constant_schedules;
#[cfg(test)]
use day_table::{
    compiled_day_schedule_value, precompile_day_schedule_table, year_schedule_hourly_value,
};

#[cfg(test)]
mod tests;

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
        if external_interface::external_interface_schedule_value(model, schedule_id).is_some() {
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
            None if model
                .file_schedules
                .iter()
                .any(|schedule| schedule.id == schedule_id) =>
            {
                format!(
                    "Schedule:File ID {} requires a calendar-aware precomputed schedule series",
                    schedule_id.0
                )
            }
            None if model
                .file_shading_schedule
                .as_ref()
                .is_some_and(|schedule| {
                    schedule
                        .columns
                        .iter()
                        .any(|column| column.id == schedule_id)
                }) =>
            {
                format!(
                    "Schedule:File:Shading generated schedule ID {} requires a calendar- and zone-timestep-aware precomputed schedule series",
                    schedule_id.0
                )
            }
            None if model
                .year_schedules
                .iter()
                .any(|schedule| schedule.id == schedule_id) =>
            {
                format!(
                    "Schedule:Year ID {} requires a calendar-aware precomputed schedule series",
                    schedule_id.0
                )
            }
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
    if let Some(value) = external_interface::external_interface_schedule_value(model, schedule_id) {
        return Some(value);
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
    /// Immutable initial value of an external-interface schedule family.
    ExternalInterfaceInitialValue {
        /// Initial value reused until an external interface update is ported.
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
    /// Hourly `Schedule:File` values with EnergyPlus' 8760-row leap-day expansion.
    FileHourly8760 {
        /// Immutable source value count loaded during compilation.
        source_value_count: usize,
    },
    /// One surface column generated from immutable `Schedule:File:Shading` zone-timestep values.
    FileShadingZoneTimestep {
        /// Number of source calendar days represented by the immutable values.
        source_day_count: u32,
        /// Source values per hour.
        timesteps_per_hour: u32,
        /// Immutable source value count loaded during compilation.
        source_value_count: usize,
    },
    /// Direct `Schedule:Year` -> `Schedule:Week:Daily` -> `Schedule:Day:Hourly` lookup.
    YearWeekDayHourlyDirect {
        /// Immutable leap-shaped annual pointer count.
        schedule_day_count: usize,
    },
    /// Direct annual lookup through an immutable zone-timestep day-profile cache.
    YearWeekDayCompiledProfiles {
        /// Immutable leap-shaped annual pointer count.
        schedule_day_count: usize,
        /// Number of unambiguous hourly and interval day profiles in the cache.
        compiled_day_schedule_count: usize,
        /// Whole minutes represented by every cached zone-timestep value.
        minutes_per_timestep: u32,
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
    /// Interpolation mode used to prepare zone-timestep values.
    pub interpolation: ScheduleInterpolation,
    /// Ordered daily intervals in minute-of-day space.
    pub intervals: Vec<CompiledScheduleInterval>,
    /// Whole minutes represented by each prepared zone-timestep value.
    pub minutes_per_timestep: u32,
    /// Immutable daily values at zone-timestep resolution.
    pub zone_timestep_values: Vec<f64>,
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

/// Simulates constant and calendar-invariant compact schedules for hourly samples.
pub fn simulate_schedule_values(
    model: &TypedModel,
    sample_count: usize,
) -> Result<Vec<ScheduleTrace>, String> {
    precompute_schedule_value_series(model, sample_count)
}

/// Precomputes constant and calendar-invariant compact schedules for hourly samples.
///
/// Calendar-varying compact, file-backed, and annual schedules require a
/// TimeAxis and are rejected by this hour-only API rather than being evaluated
/// against an implicit calendar state.
pub fn precompute_schedule_value_series(
    model: &TypedModel,
    sample_count: usize,
) -> Result<Vec<ScheduleValueSeries>, String> {
    precompute_schedule_cache(model, sample_count).map(ScheduleSeriesCache::into_traces)
}

/// Precomputes every supported typed schedule for a run-period time axis.
///
/// This axis has hourly samples, so compact profiles using Average or Linear
/// interpolation or subhourly Until boundaries return NaN values until hourly
/// schedule aggregation is ported. File and annual schedules use the axis'
/// schedule ordinal, while annual schedules also select the active day type.
/// File:Shading columns average their immutable zone-timestep source values
/// when the axis timestep count matches and fail closed to NaN otherwise.
#[must_use]
pub fn precompute_schedule_value_series_for_time_axis(
    model: &TypedModel,
    time_axis: &TimeAxis,
) -> Vec<ScheduleValueSeries> {
    precompute_schedule_cache_for_time_axis(model, time_axis).into_traces()
}

/// Precomputes every supported typed schedule for every zone timestep in one
/// simulation environment.
///
/// Compact profiles are expanded to EnergyPlus' minute lattice and reduced to
/// zone-timestep values before calendar lookup. The current DST state shifts
/// only the lookup hour, preserving the one-based zone-timestep position within
/// that hour.
#[must_use]
pub fn precompute_schedule_value_series_for_environment_time_axis(
    model: &TypedModel,
    time_axis: &EnvironmentTimeAxis,
) -> Vec<ScheduleValueSeries> {
    precompute_schedule_cache_for_environment_time_axis(model, time_axis).into_traces()
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
    let periods =
        precompile_compact_schedule_periods(schedule, time_axis.zone_timestep.timesteps_per_hour);
    let requires_hourly_aggregation = schedule.periods.iter().any(|period| {
        period.day_profiles.iter().any(|profile| {
            profile.interpolation != ScheduleInterpolation::No
                || profile
                    .segments
                    .iter()
                    .any(|segment| segment.until_minute_of_day % 60 != 0)
        })
    });
    let values = time_axis
        .points
        .iter()
        .map(|point| {
            if requires_hourly_aggregation {
                return f64::NAN;
            }
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
    let periods =
        precompile_compact_schedule_periods(schedule, time_axis.zone_timestep.timesteps_per_hour);
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

fn file_schedule_series_for_time_axis(
    schedule: &ScheduleFile,
    time_axis: &TimeAxis,
) -> ScheduleValueSeries {
    let values = time_axis
        .points
        .iter()
        .map(|point| {
            file_schedule_hourly_8760_value(schedule, point.schedule_day_of_year, point.hour)
                .unwrap_or(f64::NAN)
        })
        .collect();
    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::FileHourly8760 {
            source_value_count: schedule.values.len(),
        },
        values,
    }
}

fn file_schedule_series_for_environment_time_axis(
    schedule: &ScheduleFile,
    time_axis: &EnvironmentTimeAxis,
) -> ScheduleValueSeries {
    let values = time_axis
        .points
        .iter()
        .map(|point| {
            file_schedule_hourly_8760_value(schedule, point.schedule_day_of_year, point.hour)
                .unwrap_or(f64::NAN)
        })
        .collect();
    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::FileHourly8760 {
            source_value_count: schedule.values.len(),
        },
        values,
    }
}

fn file_schedule_hourly_8760_value(
    schedule: &ScheduleFile,
    schedule_day_of_year: u32,
    hour_ending: u32,
) -> Option<f64> {
    let schedule_day = schedule_day_of_year.clamp(1, 366);
    let source_day = match schedule_day {
        60 => 59,
        day if day > 60 => day - 1,
        day => day,
    };
    let source_index = (source_day - 1)
        .checked_mul(24)?
        .checked_add(hour_ending.clamp(1, 24) - 1)?;
    schedule.values.get(source_index as usize).copied()
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

    if let Some(profile) = period
        .day_profiles
        .iter()
        .find(|profile| profile.interpolation != ScheduleInterpolation::No)
    {
        return Err(format!(
            "Schedule:Compact {} uses Interpolate:{:?}; hour-only consumers require Interpolate:No",
            schedule.name.0, profile.interpolation
        ));
    }
    if period.day_profiles.iter().any(|profile| {
        profile
            .segments
            .iter()
            .any(|segment| segment.until_minute_of_day % 60 != 0)
    }) {
        return Err(format!(
            "Schedule:Compact {} uses subhourly Until boundaries; hour-only consumers require whole-hour boundaries until hourly aggregation is ported",
            schedule.name.0
        ));
    }

    let mut baseline: Option<(ScheduleInterpolation, &[ScheduleCompactSegment])> = None;
    for day_type in ALL_SCHEDULE_DAY_TYPES {
        let Some(profile) = unique_day_profile(period, day_type) else {
            return Err(format!(
                "Schedule:Compact {} must assign day type {day_type:?} exactly once for hour-only consumption",
                schedule.name.0
            ));
        };
        if let Some((expected_interpolation, expected_segments)) = baseline {
            if profile.interpolation != expected_interpolation
                || profile.segments.as_slice() != expected_segments
            {
                return Err(format!(
                    "Schedule:Compact {} varies by day type; hour-only consumers require equivalent profiles",
                    schedule.name.0
                ));
            }
        } else {
            baseline = Some((profile.interpolation, &profile.segments));
        }
    }

    baseline
        .map(|(_interpolation, segments)| segments)
        .ok_or_else(|| {
            format!(
                "Schedule:Compact {} has no calendar-invariant day profile",
                schedule.name.0
            )
        })
}

/// Precompiles all annual periods and day-type profiles at zone-timestep resolution.
///
/// A timestep count that does not divide 60 produces empty prepared profiles so
/// lookup fails closed instead of approximating fractional-minute windows.
#[must_use]
pub fn precompile_compact_schedule_periods(
    schedule: &ScheduleCompact,
    timesteps_per_hour: u32,
) -> Vec<CompiledSchedulePeriod> {
    let minutes_per_timestep = schedule_minutes_per_timestep(timesteps_per_hour);
    schedule
        .periods
        .iter()
        .map(|period| CompiledSchedulePeriod {
            through_schedule_day_of_year: period.through_schedule_day_of_year,
            day_profiles: period
                .day_profiles
                .iter()
                .map(|profile| {
                    precompile_compact_schedule_day_profile(profile, minutes_per_timestep)
                })
                .collect(),
        })
        .collect()
}

fn precompile_compact_schedule_day_profile(
    profile: &ScheduleCompactDayProfile,
    minutes_per_timestep: Option<u32>,
) -> CompiledScheduleDayProfile {
    let minute_values = expand_compact_schedule_minute_values(profile);
    let (minutes_per_timestep, zone_timestep_values) =
        reduce_schedule_minute_values(profile.interpolation, &minute_values, minutes_per_timestep);

    CompiledScheduleDayProfile {
        day_types: profile.day_types.clone(),
        interpolation: profile.interpolation,
        intervals: precompile_compact_schedule_intervals(&profile.segments),
        minutes_per_timestep,
        zone_timestep_values,
    }
}

fn precompile_schedule_day_values(
    interpolation: ScheduleInterpolation,
    segments: &[ScheduleCompactSegment],
    minutes_per_timestep: Option<u32>,
) -> (u32, Vec<f64>) {
    let minute_values = expand_schedule_minute_values(interpolation, segments);
    reduce_schedule_minute_values(interpolation, &minute_values, minutes_per_timestep)
}

fn reduce_schedule_minute_values(
    interpolation: ScheduleInterpolation,
    minute_values: &[f64],
    minutes_per_timestep: Option<u32>,
) -> (u32, Vec<f64>) {
    minutes_per_timestep.map_or_else(
        || (0, Vec::new()),
        |minutes_per_timestep| {
            let values = minute_values
                .chunks_exact(minutes_per_timestep as usize)
                .map(|window| match interpolation {
                    ScheduleInterpolation::Average => {
                        window.iter().sum::<f64>() / f64::from(minutes_per_timestep)
                    }
                    ScheduleInterpolation::No | ScheduleInterpolation::Linear => {
                        window.last().copied().unwrap_or(f64::NAN)
                    }
                })
                .collect();
            (minutes_per_timestep, values)
        },
    )
}

fn expand_compact_schedule_minute_values(profile: &ScheduleCompactDayProfile) -> Vec<f64> {
    expand_schedule_minute_values(profile.interpolation, &profile.segments)
}

fn expand_schedule_minute_values(
    interpolation: ScheduleInterpolation,
    segments: &[ScheduleCompactSegment],
) -> Vec<f64> {
    let mut minute_values = Vec::with_capacity(1440);
    let mut previous_until_minute = 0_u32;
    let mut previous_value = None;

    for segment in segments {
        let until_minute = segment
            .until_minute_of_day
            .clamp(previous_until_minute, 1440);
        let duration_minutes = until_minute - previous_until_minute;
        if interpolation == ScheduleInterpolation::Linear {
            if let Some(start_value) = previous_value {
                let increment = (segment.value - start_value) / f64::from(duration_minutes.max(1));
                let mut current_value = start_value;
                for _minute in 1..=duration_minutes {
                    current_value += increment;
                    minute_values.push(current_value);
                }
            } else {
                minute_values.resize(until_minute as usize, segment.value);
            }
        } else {
            minute_values.resize(until_minute as usize, segment.value);
        }
        previous_until_minute = until_minute;
        previous_value = Some(segment.value);
    }

    minute_values.resize(1440, previous_value.unwrap_or(f64::NAN));
    minute_values
}

fn schedule_minutes_per_timestep(timesteps_per_hour: u32) -> Option<u32> {
    (timesteps_per_hour > 0 && 60 % timesteps_per_hour == 0).then(|| 60 / timesteps_per_hour)
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

const fn schedule_day_type_index(day_type: ScheduleDayType) -> usize {
    match day_type {
        ScheduleDayType::Sunday => 0,
        ScheduleDayType::Monday => 1,
        ScheduleDayType::Tuesday => 2,
        ScheduleDayType::Wednesday => 3,
        ScheduleDayType::Thursday => 4,
        ScheduleDayType::Friday => 5,
        ScheduleDayType::Saturday => 6,
        ScheduleDayType::Holiday => 7,
        ScheduleDayType::SummerDesignDay => 8,
        ScheduleDayType::WinterDesignDay => 9,
        ScheduleDayType::CustomDay1 => 10,
        ScheduleDayType::CustomDay2 => 11,
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
        .and_then(|profile| {
            let minute = minute_of_day.clamp(1, 1440);
            let value_index = minute
                .checked_sub(1)?
                .checked_div(profile.minutes_per_timestep)?;
            profile
                .zone_timestep_values
                .get(value_index as usize)
                .copied()
        })
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
