//! Referenced-only hourly schedule cache for `OtherEquipment` gains.

use super::constant::constant_cached_schedule_series;
use super::external_interface::external_interface_cached_schedule_series_iter;
use super::{
    CachedScheduleSeries, ScheduleSampleStorage, ScheduleSeriesCache, ScheduleSeriesKind,
    compact_schedule_value, convective_internal_gain_for_equipment_with_multiplier_w,
    hour_only_single_period_compact_schedule_segments, precompile_compact_schedule_intervals,
    radiant_internal_gain_for_equipment_with_multiplier_w,
    update_surface_radiant_internal_gain_source_terms_with,
};
use crate::error::RuntimeError;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::{ScheduleId, TypedModel, ZoneId};
use std::collections::BTreeSet;

const HOUR_ONLY_SAMPLE_COUNT: usize = 24;

pub(crate) fn precompute_hour_only_internal_gain_schedule_cache(
    model: &TypedModel,
) -> Result<ScheduleSeriesCache, RuntimeError> {
    super::validate_hour_only_internal_gain_schedules(model)?;

    let mut referenced_schedule_ids = BTreeSet::new();
    let mut entries = Vec::new();
    for equipment in &model.other_equipment {
        let Some(schedule_id) = equipment.schedule else {
            continue;
        };
        if referenced_schedule_ids.insert(schedule_id) {
            entries.push(referenced_cached_schedule_series(
                model,
                schedule_id,
                &equipment.name.0,
            )?);
        }
    }

    Ok(ScheduleSeriesCache::from_entries(
        HOUR_ONLY_SAMPLE_COUNT,
        entries,
    ))
}

fn referenced_cached_schedule_series(
    model: &TypedModel,
    schedule_id: ScheduleId,
    equipment_name: &str,
) -> Result<CachedScheduleSeries, RuntimeError> {
    if let Some(schedule) = model
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
    {
        return Ok(constant_cached_schedule_series(
            schedule,
            HOUR_ONLY_SAMPLE_COUNT,
        ));
    }
    if let Some(series) =
        external_interface_cached_schedule_series_iter(model, HOUR_ONLY_SAMPLE_COUNT)
            .find(|series| series.schedule_id == schedule_id)
    {
        return Ok(series);
    }
    if let Some(schedule) = model
        .compact_schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
    {
        let segments =
            hour_only_single_period_compact_schedule_segments(schedule).map_err(|reason| {
                invalid_internal_gain_schedule(equipment_name, schedule_id, reason)
            })?;
        let intervals = precompile_compact_schedule_intervals(segments);
        let samples = (1_u32..=24)
            .map(|hour_ending| {
                compact_schedule_value(segments, hour_ending * 60).unwrap_or(f64::NAN)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        return Ok(CachedScheduleSeries {
            schedule_id,
            schedule_name: schedule.name.0.clone(),
            kind: ScheduleSeriesKind::CompactIntervals { intervals },
            samples: ScheduleSampleStorage::Dense(samples),
        });
    }

    Err(invalid_internal_gain_schedule(
        equipment_name,
        schedule_id,
        format!("schedule ID {} is unresolved", schedule_id.0),
    ))
}

fn invalid_internal_gain_schedule(
    equipment_name: &str,
    schedule_id: ScheduleId,
    reason: String,
) -> RuntimeError {
    RuntimeError::InvalidInternalGainSchedule {
        equipment_name: equipment_name.to_string(),
        schedule_id: schedule_id.0,
        reason,
    }
}

fn hour_only_schedule_multiplier_from_cache(
    schedule_cache: &ScheduleSeriesCache,
    schedule_id: Option<ScheduleId>,
    hour_ending: u32,
) -> f64 {
    let Some(schedule_id) = schedule_id else {
        return 1.0;
    };
    let sample_index = (hour_ending.clamp(1, 24) - 1) as usize;
    schedule_cache
        .value(schedule_id, sample_index)
        .unwrap_or(f64::NAN)
}

pub(crate) fn convective_internal_gain_w_from_cache(
    model: &TypedModel,
    schedule_cache: &ScheduleSeriesCache,
    zone_id: ZoneId,
    hour_ending: u32,
) -> f64 {
    model
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone_id)
        .map(|equipment| {
            let schedule_multiplier = hour_only_schedule_multiplier_from_cache(
                schedule_cache,
                equipment.schedule,
                hour_ending,
            );
            convective_internal_gain_for_equipment_with_multiplier_w(
                model,
                equipment,
                schedule_multiplier,
            )
        })
        .sum()
}

pub(super) fn radiant_internal_gain_w_from_cache(
    model: &TypedModel,
    schedule_cache: &ScheduleSeriesCache,
    zone_id: ZoneId,
    hour_ending: u32,
) -> f64 {
    model
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone_id)
        .map(|equipment| {
            let schedule_multiplier = hour_only_schedule_multiplier_from_cache(
                schedule_cache,
                equipment.schedule,
                hour_ending,
            );
            radiant_internal_gain_for_equipment_with_multiplier_w(
                model,
                equipment,
                schedule_multiplier,
            )
        })
        .sum()
}

pub(crate) fn update_surface_radiant_internal_gain_source_terms_from_cache(
    model: &TypedModel,
    schedule_cache: &ScheduleSeriesCache,
    surfaces: &mut [SurfaceHeatBalanceState],
    hour_ending: u32,
) {
    update_surface_radiant_internal_gain_source_terms_with(surfaces, |zone_id| {
        radiant_internal_gain_w_from_cache(model, schedule_cache, zone_id, hour_ending)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::{
        ExternalInterfaceFmuExportSchedule, ExternalInterfaceFmuImportSchedule,
        ExternalInterfaceSchedule, InternalGainId, NormalizedName, OtherEquipment,
        OtherEquipmentDesignLevelCalculationMethod, ScheduleCompact, ScheduleCompactDayProfile,
        ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleConstant, ScheduleDayType,
        ScheduleFile, ScheduleFileColumnSeparator, ScheduleFileShading, ScheduleFileShadingColumn,
        ScheduleInterpolation, ScheduleYear, WeekScheduleId,
    };

    fn equipment(id: u32, schedule: Option<ScheduleId>) -> OtherEquipment {
        OtherEquipment {
            id: InternalGainId(id),
            name: NormalizedName::new(&format!("Equipment {id}")),
            fuel_type: NormalizedName::new("Electricity"),
            zone: ZoneId(3),
            schedule,
            design_level_calculation_method:
                OtherEquipmentDesignLevelCalculationMethod::EquipmentLevel,
            design_level_w: 100.0,
            power_per_floor_area_w_per_m2: 0.0,
            power_per_person_w: 0.0,
            fraction_latent: 0.1,
            fraction_radiant: 0.2,
            fraction_lost: 0.3,
            carbon_dioxide_generation_rate_m3_per_s_w: 0.0,
        }
    }

    fn constant_schedule(id: u32, value: f64) -> ScheduleConstant {
        ScheduleConstant {
            id: ScheduleId(id),
            name: NormalizedName::new(&format!("Constant {id}")),
            schedule_type_limits: None,
            hourly_value: value,
        }
    }

    fn external_schedule(id: u32, value: f64) -> ExternalInterfaceSchedule {
        ExternalInterfaceSchedule {
            id: ScheduleId(id),
            name: NormalizedName::new(&format!("External {id}")),
            schedule_type_limits: None,
            initial_value: value,
        }
    }

    fn fmu_import_schedule(id: u32, value: f64) -> ExternalInterfaceFmuImportSchedule {
        ExternalInterfaceFmuImportSchedule {
            id: ScheduleId(id),
            name: NormalizedName::new(&format!("FMU Import {id}")),
            schedule_type_limits: None,
            fmu_file_name: "unused.fmu".to_string(),
            fmu_instance_name: "unused".to_string(),
            fmu_variable_name: "unused".to_string(),
            initial_value: value,
        }
    }

    fn fmu_export_schedule(id: u32, value: f64) -> ExternalInterfaceFmuExportSchedule {
        ExternalInterfaceFmuExportSchedule {
            id: ScheduleId(id),
            name: NormalizedName::new(&format!("FMU Export {id}")),
            schedule_type_limits: None,
            fmu_variable_name: "unused".to_string(),
            initial_value: value,
        }
    }

    fn all_day_types() -> Vec<ScheduleDayType> {
        vec![
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
        ]
    }

    fn compact_schedule(id: u32, segments: Vec<ScheduleCompactSegment>) -> ScheduleCompact {
        ScheduleCompact {
            id: ScheduleId(id),
            name: NormalizedName::new(&format!("Compact {id}")),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![ScheduleCompactDayProfile {
                    day_types: all_day_types(),
                    interpolation: ScheduleInterpolation::No,
                    segments,
                }],
            }],
        }
    }

    fn flat_compact_schedule(id: u32, value: f64) -> ScheduleCompact {
        compact_schedule(
            id,
            vec![ScheduleCompactSegment {
                until_minute_of_day: 1440,
                value,
            }],
        )
    }

    #[test]
    fn referenced_only_cache_ignores_unrelated_calendar_schedule_families() {
        let mut model = TypedModel::default();
        model.schedules.push(constant_schedule(1, 0.25));
        model
            .other_equipment
            .push(equipment(0, Some(ScheduleId(1))));
        model.file_schedules.push(ScheduleFile {
            id: ScheduleId(20),
            name: NormalizedName::new("Unrelated File"),
            schedule_type_limits: None,
            file_name: "unused.csv".to_string(),
            column_number: 1,
            rows_to_skip_at_top: 0,
            number_of_hours_of_data: 8760,
            column_separator: ScheduleFileColumnSeparator::Comma,
            interpolate_to_timestep: false,
            minutes_per_item: 60,
            adjust_schedule_for_daylight_savings: false,
            values: Vec::new(),
        });
        model.file_shading_schedule = Some(ScheduleFileShading {
            file_name: "unused-shading.csv".to_string(),
            timesteps_per_hour: 1,
            source_day_count: 365,
            columns: vec![ScheduleFileShadingColumn {
                id: ScheduleId(21),
                surface_header: "Unrelated Surface".to_string(),
                schedule_name: NormalizedName::new("Unrelated Surface Shading"),
                values: Vec::new(),
            }],
        });
        model.year_schedules.push(ScheduleYear {
            id: ScheduleId(22),
            name: NormalizedName::new("Unrelated Year"),
            schedule_type_limits: None,
            week_schedules: [WeekScheduleId(0); 366],
        });
        let mut varying_compact = flat_compact_schedule(23, 0.5);
        varying_compact.periods[0].through_schedule_day_of_year = 100;
        varying_compact.periods.push(ScheduleCompactPeriod {
            through_schedule_day_of_year: 366,
            day_profiles: Vec::new(),
        });
        model.compact_schedules.push(varying_compact);

        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("unrelated schedule families must be ignored");

        assert_eq!(cache.sample_count(), 24);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.value(ScheduleId(1), 0), Some(0.25));
        for unrelated_id in [20, 21, 22, 23] {
            assert!(cache.get(ScheduleId(unrelated_id)).is_none());
        }
    }

    #[test]
    fn constant_external_and_compact_cache_values_are_bit_equal_to_fallback() {
        let mut model = TypedModel::default();
        model.schedules.push(constant_schedule(1, 0.25));
        model
            .external_interface_schedules
            .push(external_schedule(2, 0.5));
        model.compact_schedules.push(compact_schedule(
            3,
            vec![
                ScheduleCompactSegment {
                    until_minute_of_day: 720,
                    value: 0.75,
                },
                ScheduleCompactSegment {
                    until_minute_of_day: 1440,
                    value: 0.125,
                },
            ],
        ));
        model.other_equipment = vec![
            equipment(0, Some(ScheduleId(1))),
            equipment(1, Some(ScheduleId(2))),
            equipment(2, Some(ScheduleId(3))),
        ];
        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("supported schedules should cache");

        for hour_ending in 1_u32..=24 {
            let sample_index = (hour_ending - 1) as usize;
            for schedule_id in [ScheduleId(1), ScheduleId(2), ScheduleId(3)] {
                let cached = cache
                    .value(schedule_id, sample_index)
                    .expect("referenced schedule should be cached");
                let fallback = super::super::schedule_value(&model, schedule_id, hour_ending)
                    .expect("supported fallback schedule should resolve");
                assert_eq!(cached.to_bits(), fallback.to_bits());
            }
            assert_eq!(
                convective_internal_gain_w_from_cache(&model, &cache, ZoneId(3), hour_ending,)
                    .to_bits(),
                super::super::convective_internal_gain_w(&model, ZoneId(3), hour_ending).to_bits()
            );
        }
    }

    #[test]
    fn malformed_compact_until_order_remains_bit_equal_to_raw_fallback() {
        let mut model = TypedModel::default();
        model.compact_schedules.push(compact_schedule(
            30,
            vec![
                ScheduleCompactSegment {
                    until_minute_of_day: 180,
                    value: 0.25,
                },
                ScheduleCompactSegment {
                    until_minute_of_day: 60,
                    value: 0.75,
                },
                ScheduleCompactSegment {
                    until_minute_of_day: 1440,
                    value: 0.5,
                },
            ],
        ));
        model
            .other_equipment
            .push(equipment(0, Some(ScheduleId(30))));
        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("whole-hour malformed Until order remains accepted by fallback validation");

        for hour_ending in 1_u32..=24 {
            let cached = cache
                .value(ScheduleId(30), (hour_ending - 1) as usize)
                .expect("referenced compact schedule should be cached");
            let fallback = super::super::schedule_value(&model, ScheduleId(30), hour_ending)
                .expect("raw fallback should resolve");
            assert_eq!(cached.to_bits(), fallback.to_bits());
        }
    }

    #[test]
    fn duplicate_resolution_preserves_family_priority_and_first_wins() {
        let model = TypedModel {
            schedules: vec![constant_schedule(7, 0.1), constant_schedule(7, 0.11)],
            external_interface_schedules: vec![
                external_schedule(7, 0.2),
                external_schedule(8, 0.21),
                external_schedule(8, 0.22),
            ],
            external_interface_fmu_import_schedules: vec![
                fmu_import_schedule(7, 0.3),
                fmu_import_schedule(8, 0.31),
                fmu_import_schedule(9, 0.32),
                fmu_import_schedule(9, 0.33),
            ],
            external_interface_fmu_export_schedules: vec![
                fmu_export_schedule(7, 0.4),
                fmu_export_schedule(8, 0.41),
                fmu_export_schedule(9, 0.42),
                fmu_export_schedule(10, 0.43),
                fmu_export_schedule(10, 0.44),
            ],
            compact_schedules: (7..=10).map(|id| flat_compact_schedule(id, 0.5)).collect(),
            other_equipment: (7..=10)
                .enumerate()
                .map(|(index, id)| equipment(index as u32, Some(ScheduleId(id))))
                .collect(),
            ..TypedModel::default()
        };

        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("duplicate supported schedules should cache");

        assert_eq!(cache.value(ScheduleId(7), 0), Some(0.1));
        assert_eq!(cache.value(ScheduleId(8), 0), Some(0.21));
        assert_eq!(cache.value(ScheduleId(9), 0), Some(0.32));
        assert_eq!(cache.value(ScheduleId(10), 0), Some(0.43));
        assert_eq!(cache.len(), 4);
    }

    #[test]
    fn empty_compact_profiles_cache_nan_instead_of_zero() {
        let mut model = TypedModel::default();
        model
            .compact_schedules
            .push(compact_schedule(4, Vec::new()));
        model
            .other_equipment
            .push(equipment(0, Some(ScheduleId(4))));

        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("empty invariant compact profile is structurally supported");

        assert!(
            cache
                .get(ScheduleId(4))
                .expect("referenced compact schedule should be cached")
                .values()
                .all(f64::is_nan)
        );
        assert!(convective_internal_gain_w_from_cache(&model, &cache, ZoneId(3), 1).is_nan());
        assert!(super::super::convective_internal_gain_w(&model, ZoneId(3), 1).is_nan());
    }

    #[test]
    fn cached_multiplier_fails_closed_to_nan_on_lookup_miss() {
        let mut model = TypedModel::default();
        model.schedules.push(constant_schedule(31, 0.5));
        model
            .other_equipment
            .push(equipment(0, Some(ScheduleId(31))));
        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("referenced constant schedule should cache");
        model.other_equipment[0].schedule = Some(ScheduleId(32));

        assert!(convective_internal_gain_w_from_cache(&model, &cache, ZoneId(3), 1).is_nan());
        assert!(radiant_internal_gain_w_from_cache(&model, &cache, ZoneId(3), 1).is_nan());
    }

    #[test]
    fn cached_hour_lookup_clamps_to_one_through_twenty_four() {
        let segments = (1_u32..=24)
            .map(|hour_ending| ScheduleCompactSegment {
                until_minute_of_day: hour_ending * 60,
                value: f64::from(hour_ending),
            })
            .collect();
        let mut model = TypedModel::default();
        model.compact_schedules.push(compact_schedule(5, segments));
        let mut gain = equipment(0, Some(ScheduleId(5)));
        gain.design_level_w = 1.0;
        gain.fraction_latent = 0.0;
        gain.fraction_radiant = 0.0;
        gain.fraction_lost = 0.0;
        model.other_equipment.push(gain);
        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("hourly compact schedule should cache");

        assert_eq!(
            convective_internal_gain_w_from_cache(&model, &cache, ZoneId(3), 0),
            1.0
        );
        for hour_ending in 1_u32..=24 {
            assert_eq!(
                convective_internal_gain_w_from_cache(&model, &cache, ZoneId(3), hour_ending,),
                f64::from(hour_ending)
            );
        }
        assert_eq!(
            convective_internal_gain_w_from_cache(&model, &cache, ZoneId(3), 25),
            24.0
        );
    }

    #[test]
    fn convective_and_radiant_paths_consume_the_same_cache() {
        let mut model = TypedModel::default();
        model.schedules.push(constant_schedule(6, 0.5));
        model
            .other_equipment
            .push(equipment(0, Some(ScheduleId(6))));
        let cache = precompute_hour_only_internal_gain_schedule_cache(&model)
            .expect("constant schedule should cache");

        let convective = convective_internal_gain_w_from_cache(&model, &cache, ZoneId(3), 12);
        let radiant = radiant_internal_gain_w_from_cache(&model, &cache, ZoneId(3), 12);
        assert_eq!(
            convective.to_bits(),
            super::super::convective_internal_gain_w(&model, ZoneId(3), 12).to_bits()
        );
        assert_eq!(
            radiant.to_bits(),
            super::super::radiant_internal_gain_w(&model, ZoneId(3), 12).to_bits()
        );

        let mut surfaces: Vec<SurfaceHeatBalanceState> = Vec::new();
        update_surface_radiant_internal_gain_source_terms_from_cache(
            &model,
            &cache,
            &mut surfaces,
            12,
        );
        assert!(surfaces.is_empty());
    }
}
