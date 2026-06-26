//! Schedule lookup and internal-gain trace helpers.

use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::{OtherEquipment, ScheduleCompactSegment, ScheduleId, TypedModel, ZoneId};
use std::collections::BTreeSet;

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

    equipment.design_level_w * schedule_multiplier * sensible_fraction
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

    equipment.design_level_w * schedule_multiplier * convective_fraction
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

    equipment.design_level_w * schedule_multiplier * radiant_fraction
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

fn schedule_ids(model: &TypedModel) -> impl Iterator<Item = ScheduleId> + '_ {
    model
        .schedules
        .iter()
        .map(|schedule| schedule.id)
        .chain(model.compact_schedules.iter().map(|schedule| schedule.id))
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
    /// Sampled schedule values.
    pub values: Vec<f64>,
}

/// Precomputed schedule values for one schedule.
pub type ScheduleValueSeries = ScheduleTrace;

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
    schedule_ids(model)
        .filter_map(|schedule_id| {
            let schedule_name = schedule_name(model, schedule_id)?;
            let values = (0..sample_count)
                .map(|index| {
                    let hour_ending = u32::try_from(index % 24 + 1).unwrap_or(24);
                    schedule_value(model, schedule_id, hour_ending).unwrap_or(0.0)
                })
                .collect();
            Some(ScheduleTrace {
                schedule_id,
                schedule_name,
                values,
            })
        })
        .collect()
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

fn schedule_name(model: &TypedModel, schedule_id: ScheduleId) -> Option<String> {
    model
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .map(|schedule| schedule.name.0.clone())
        .or_else(|| {
            model
                .compact_schedules
                .iter()
                .find(|schedule| schedule.id == schedule_id)
                .map(|schedule| schedule.name.0.clone())
        })
}
