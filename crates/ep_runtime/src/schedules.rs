//! Schedule lookup and internal-gain trace helpers.

use crate::geometry::zone_floor_area_m2;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use crate::time_axis::TimeAxis;
use ep_model::{
    OtherEquipment, OtherEquipmentDesignLevelCalculationMethod, People,
    PeopleNumberCalculationMethod, ScheduleCompact, ScheduleCompactSegment, ScheduleConstant,
    ScheduleId, TypedModel, ZoneId,
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
    let schedule_multiplier = equipment
        .schedule
        .and_then(|schedule_id| schedule_value(model, schedule_id, hour_ending))
        .unwrap_or(1.0);
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
    let schedule_multiplier = equipment
        .schedule
        .and_then(|schedule_id| schedule_value(model, schedule_id, hour_ending))
        .unwrap_or(1.0);
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
    let schedule_multiplier = equipment
        .schedule
        .and_then(|schedule_id| schedule_value(model, schedule_id, hour_ending))
        .unwrap_or(1.0);
    let radiant_fraction = equipment.fraction_radiant.max(0.0);

    other_equipment_design_level_w(model, equipment) * schedule_multiplier * radiant_fraction
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
        .and_then(|schedule| compact_schedule_value(&schedule.segments, minute_of_day))
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

/// Simulates constant and supported compact schedules for a fixed number of hourly samples.
#[must_use]
pub fn simulate_schedule_values(model: &TypedModel, sample_count: usize) -> Vec<ScheduleTrace> {
    precompute_schedule_value_series(model, sample_count)
}

/// Precomputes constant and supported compact schedules for hourly samples.
#[must_use]
pub fn precompute_schedule_value_series(
    model: &TypedModel,
    sample_count: usize,
) -> Vec<ScheduleValueSeries> {
    let hours = (0..sample_count).map(|index| u32::try_from(index % 24 + 1).unwrap_or(24));
    precompute_schedule_value_series_for_hours(model, hours)
}

/// Precomputes constant and supported compact schedules for a run-period time axis.
#[must_use]
pub fn precompute_schedule_value_series_for_time_axis(
    model: &TypedModel,
    time_axis: &TimeAxis,
) -> Vec<ScheduleValueSeries> {
    precompute_schedule_value_series_for_hours(
        model,
        time_axis.points.iter().map(|point| point.hour),
    )
}

fn precompute_schedule_value_series_for_hours(
    model: &TypedModel,
    hours: impl IntoIterator<Item = u32> + Clone,
) -> Vec<ScheduleValueSeries> {
    model
        .schedules
        .iter()
        .map(|schedule| constant_schedule_series(schedule, hours.clone()))
        .chain(
            model
                .compact_schedules
                .iter()
                .map(|schedule| compact_schedule_series(schedule, hours.clone())),
        )
        .collect()
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

fn compact_schedule_series(
    schedule: &ScheduleCompact,
    hours: impl IntoIterator<Item = u32>,
) -> ScheduleValueSeries {
    let intervals = precompile_compact_schedule_intervals(&schedule.segments);
    let values = hours
        .into_iter()
        .map(|hour_ending| {
            let minute_of_day = hour_ending.clamp(1, 24) * 60;
            compact_interval_value(&intervals, minute_of_day).unwrap_or(0.0)
        })
        .collect();

    ScheduleTrace {
        schedule_id: schedule.id,
        schedule_name: schedule.name.0.clone(),
        kind: ScheduleSeriesKind::CompactIntervals { intervals },
        values,
    }
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
#[must_use]
pub fn simulate_zone_internal_convective_gains(
    model: &TypedModel,
    sample_count: usize,
) -> Vec<ZoneInternalGainTrace> {
    model
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
        .collect()
}

/// Simulates zone total internal radiant heating rates for hourly samples.
#[must_use]
pub fn simulate_zone_internal_radiant_gains(
    model: &TypedModel,
    sample_count: usize,
) -> Vec<ZoneInternalGainTrace> {
    model
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
        .collect()
}
