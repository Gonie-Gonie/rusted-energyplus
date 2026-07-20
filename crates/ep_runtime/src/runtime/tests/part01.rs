    use super::{
        ConstructionCtfCoefficientOverride, CtfInsideFaceBalanceInput, CtfOutsideFaceBalanceInput,
        CtfOutsideQuickConductionBalanceInput,
        ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C,
        ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M,
        ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K, ENERGYPLUS_ZONE_INITIAL_TEMP_C,
        EpwCalendarMetadata, EpwRecord,
        ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO, FirstZoneSimulationOptions,
        HeatBalanceCtfInitialHistoryPolicy,
        HeatBalanceSimulationOptions, HeatBalanceStepInput,
        HeatBalanceSurfaceLoopZoneAirCorrection, HeatBalanceWarmupOptions,
        HeatBalanceWarmupSummary, HeatBalanceWeatherContext, HeatBalanceZoneAirReportSampling,
        HeatBalanceZoneConductionReportSource, InteriorLongwaveExchangeProbe,
        InteriorLongwaveSurfaceSnapshot, KELVIN_OFFSET, OutputSeries,
        QuickOutsideConductionContext, ResultStore, RuntimeError, SECONDS_PER_HOUR,
        STEFAN_BOLTZMANN_W_PER_M2_K4, ScheduleSeriesKind, SimulationMode, SimulationState,
        SurfaceBoundaryBalanceResult, SurfaceCtfState, SurfaceExteriorReportTerms,
        SurfaceOutsideBalanceDiagnostics, advance_heat_balance_state_one_timestep,
        advance_heat_balance_state_one_timestep_internal, advance_surface_ctf_histories,
        advance_surface_ctf_histories_with_outside_temperature_override,
        append_surface_incident_solar_radiation_series,
        apply_energyplus_adaptive_system_timestep_zone_air_correction,
        energyplus_analytical_zone_air_temperature_c, energyplus_anisotropic_sky_multiplier,
        energyplus_approximate_view_factors, energyplus_ashrae_tarp_natural_convection_branch,
        energyplus_ashrae_tarp_natural_convection_w_per_m2_k, energyplus_average_solar_coefficients,
        energyplus_ctf_inside_face_temperature_c, energyplus_ctf_outside_face_temperature_c,
        energyplus_ctf_outside_face_temperature_quick_conduction_c,
        energyplus_daily_solar_coefficients,
        energyplus_doe2_outside_convection_coefficient_w_per_m2_k,
        energyplus_exterior_longwave_terms, energyplus_exterior_wet_context_fraction,
        energyplus_exterior_wet_timestep_fraction,
        energyplus_linearized_radiation_coefficient_w_per_m2_k,
        energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
        energyplus_outdoor_wet_bulb_c, energyplus_outside_convection_branch_id,
        energyplus_scriptf_from_view_factors, energyplus_standard_zone_air_heat_capacity_j_per_k,
        energyplus_shadowing_period_solar_coefficients,
        energyplus_surface_outdoor_air_temperature_c,
        energyplus_surface_outside_wind_speed_m_per_s,
        energyplus_tarp_inside_convection_branch_id,
        energyplus_tarp_inside_convection_coefficient_w_per_m2_k,
        energyplus_third_order_zone_air_temperature_c,
        energyplus_weather_atmospheric_pressure_at_timestep,
        energyplus_weather_dry_bulb_at_timestep,
        energyplus_weather_dry_bulb_at_timestep_with_starting_values,
        energyplus_weather_horizontal_infrared_at_timestep, energyplus_weather_record_day_of_year,
        energyplus_weather_record_is_rain_at_timestep,
        energyplus_weather_relative_humidity_at_timestep,
        energyplus_weather_wind_direction_at_timestep, energyplus_weather_wind_speed_at_timestep,
        energyplus_zone_air_heat_capacity_j_per_k, energyplus_zone_air_temperature_coefficients,
        exterior_surface_energy_balance, fix_energyplus_approximate_view_factors,
        heat_balance_uses_doe2_outside_convection,
        horizontal_infrared_sky_temperature_c, initialize_heat_balance_state,
        initialize_heat_balance_state_with_ctf_coefficients,
        inside_ctf_outside_temperature_history_commit_override_c, parse_epw_dry_bulb_series,
        parse_epw_records, precompute_schedule_value_series,
        precompute_schedule_value_series_for_time_axis, precompute_weather_timestep_series,
        run_heat_balance_run_period_warmup, run_surface_balance_passes,
        seed_energyplus_initial_surface_ctf_histories, seed_initial_surface_ctf_boundary_histories,
        simulate_constant_schedules, simulate_first_zone_uncontrolled,
        simulate_heat_balance_zone_air_temperatures,
        simulate_heat_balance_zone_air_temperatures_internal,
        simulate_heat_balance_zone_air_temperatures_with_weather_records, simulate_schedule_values,
        simulate_zone_internal_convective_gains, simulate_zone_internal_radiant_gains,
        solar_position_rad_at_local_hour, solar_weather_interpolation_weights,
        surface_air_sky_radiation_split, surface_area_m2, surface_azimuth_deg,
        surface_ctf_history_slot_samples,
        surface_ctf_inside_current_inside_term_rate_w_from_sources, surface_exterior_report_terms,
        surface_geometry_summaries, surface_heat_storage_rate_w,
        surface_incident_solar_components_hourly_average_w_per_m2,
        surface_incident_solar_radiation_for_weather_context_w_per_m2,
        surface_inside_conduction_flux_w_per_m2, surface_inside_conduction_rate_w,
        surface_inside_convection_heat_gain_rate_per_area_w_per_m2,
        surface_inside_convection_report_coefficient_w_per_m2_k,
        surface_inside_ctf_source_terms_w_per_m2, surface_outside_conduction_flux_w_per_m2,
        surface_outside_conduction_rate_w, surface_sky_view_factor,
        surface_steady_u_value_w_per_m2_k, surface_tilt_deg, update_surface_ctf_history_constants,
        update_surface_inside_longwave_exchange_probe,
        update_surface_inside_scriptf_longwave_exchange_probe,
        update_surface_radiant_internal_gain_source_terms,
        update_zone_air_heat_capacities_from_weather_context,
        zone_air_heat_balance_air_storage_rate_w,
        zone_air_heat_balance_surface_convection_rate_at_air_temperature_w,
        zone_air_heat_balance_surface_convection_rate_from_balance_w,
        zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_for_indices_w,
        zone_air_heat_balance_surface_convection_rate_w,
        zone_air_system_timestep_storage_report_rate_w, zone_geometry_summaries,
        zone_surface_report_conduction_rates_for_indices_w,
    };
    use crate::diagnostic_probes::HeatBalanceZoneAirAlgorithm;
    use crate::heat_balance::HeatBalanceAlgorithmLane;
    use crate::next_solar_weather_record_within_day;
    use crate::node::{
        NODE_STATE_SETPOINT_VARIABLE, NODE_STATE_SOURCE_MAP_PATH,
        NODE_TEMPERATURE_SETPOINT_SENTINEL_C, NodeStateProjectionOptions, NodeStateRole,
        NodeStateStore, node_temperature_setpoint_from_energyplus,
        simulate_ideal_loads_node_state_projection,
    };
    use crate::schedules::{
        precompile_compact_schedule_periods,
        precompute_schedule_value_series_for_environment_time_axis,
    };
    use crate::time_axis::{Date, next_day};
    use crate::{
        ExecutionStage, ExecutionStageKind, ExecutionStep, RuntimeOutputRegistry,
        TimeAxisError, build_environment_time_axes,
        build_environment_time_axes_with_weather_metadata, build_execution_plan,
        build_hourly_time_axis, build_hourly_time_axis_for_run_period,
        build_hourly_time_axis_for_run_period_with_weather_metadata,
        energyplus_heat_balance_compatibility_stages,
        normalized_environment_timestep_timestamp_label, normalized_hourly_timestamp_label,
        precompute_runtime_data, resolve_run_period_calendar,
        resolve_weather_environment_calendar,
    };
    use crate::{
        COMPONENT_OUTPUT_TO_FACILITY_METER_SOURCE_MAP, COOLING_ENERGY_TRANSFER_METER,
        ELECTRICITY_FACILITY_METER, GAS_FACILITY_METER, HEATING_ENERGY_TRANSFER_METER,
        METER_ZERO_NEAR_TOLERANCE_J, RuntimeDiagnosticCode, RuntimeMeterAggregationKind,
        RuntimeMeterAggregationPeriod, RuntimeMeterRequest, RuntimeOutputFrequency,
        RuntimeOutputRequest, component_output_to_facility_meter_source_map,
        meter_rate_to_energy_j, meter_value_is_zero_near_j,
    };
    use ep_model::{
        AirBoundaryAirExchange, AutoOrNumber, AutosizeOrNumber, CalendarDateRule, Construction,
        ConstructionAirBoundary, ConstructionGroundFactor, ConstructionId, ConstructionKind,
        DayOfWeek, DehumidificationControlType,
        DemandControlledVentilationType,
        FirstHourInterpolationStartingValues, HeatRecoveryType, HumidificationControlType,
        IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType, IdealLoadsLimit,
        InternalGainId, LoadDistributionScheme, Material, MaterialId,
        MaterialSurfaceRoughness, Node, NodeId, NodeList, NodeListId, NormalizedName, OtherEquipment,
        OtherEquipmentDesignLevelCalculationMethod, OutdoorAirEconomizerType, OutputHandle,
        OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm, People,
        PeopleNumberCalculationMethod, Point3, RunPeriod, RunPeriodId, ScheduleCompact,
        ScheduleCompactDayProfile, ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleConstant,
        ScheduleDayType, ScheduleId, ScheduleInterpolation, SimulationModel, SiteLocation,
        SunExposure, Surface, SurfaceId, SurfaceType, Terrain, ThermostatControlObjectType,
        ThermostatDualSetpoint,
        ThermostatSetpointId, TimestepConfig, TypedModel, WindExposure, Zone,
        ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
        ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId, ZoneThermostat,
        ZoneThermostatControl, ZoneThermostatId,
    };
    use std::collections::BTreeMap;

    fn all_schedule_day_types() -> Vec<ScheduleDayType> {
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

    fn compact_day_profile(
        day_types: Vec<ScheduleDayType>,
        value: f64,
    ) -> ScheduleCompactDayProfile {
        ScheduleCompactDayProfile {
            day_types,
            interpolation: ScheduleInterpolation::No,
            segments: vec![ScheduleCompactSegment {
                until_minute_of_day: 24 * 60,
                value,
            }],
        }
    }

    fn cross_year_day_type_compact_schedule(id: ScheduleId) -> ScheduleCompact {
        let period_one_other_days = all_schedule_day_types()
            .into_iter()
            .filter(|day_type| *day_type != ScheduleDayType::Thursday)
            .collect();
        let period_two_other_days = all_schedule_day_types()
            .into_iter()
            .filter(|day_type| {
                !matches!(
                    day_type,
                    ScheduleDayType::Tuesday
                        | ScheduleDayType::Wednesday
                        | ScheduleDayType::Holiday
                )
            })
            .collect();
        ScheduleCompact {
            id,
            name: NormalizedName::new("Cross Year Day Type"),
            schedule_type_limits: None,
            periods: vec![
                ScheduleCompactPeriod {
                    through_schedule_day_of_year: 1,
                    day_profiles: vec![
                        compact_day_profile(vec![ScheduleDayType::Thursday], 105.0),
                        compact_day_profile(period_one_other_days, 199.0),
                    ],
                },
                ScheduleCompactPeriod {
                    through_schedule_day_of_year: 366,
                    day_profiles: vec![
                        compact_day_profile(vec![ScheduleDayType::Tuesday], 103.0),
                        compact_day_profile(vec![ScheduleDayType::Wednesday], 104.0),
                        compact_day_profile(vec![ScheduleDayType::Holiday], 108.0),
                        compact_day_profile(period_two_other_days, 199.0),
                    ],
                },
            ],
        }
    }

    fn day_type_varying_annual_compact_schedule(id: ScheduleId) -> ScheduleCompact {
        let other_days = all_schedule_day_types()
            .into_iter()
            .filter(|day_type| *day_type != ScheduleDayType::Tuesday)
            .collect();
        ScheduleCompact {
            id,
            name: NormalizedName::new("Day Type Varying"),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![
                    compact_day_profile(vec![ScheduleDayType::Tuesday], 1.0),
                    compact_day_profile(other_days, 2.0),
                ],
            }],
        }
    }

    fn two_day_solar_weather_records() -> Vec<EpwRecord> {
        (0..48)
            .map(|record_index| EpwRecord {
                year: 2013,
                month: 6,
                day: 20 + record_index / 24,
                hour: record_index % 24 + 1,
                minute: 60,
                dry_bulb_c: f64::from(record_index),
                dew_point_c: 0.0,
                relative_humidity_percent: 50.0,
                atmospheric_pressure_pa: 101_325.0,
                horizontal_infrared_radiation_wh_per_m2: 0.0,
                global_horizontal_radiation_wh_per_m2: 0.0,
                direct_normal_radiation_wh_per_m2: 100.0,
                diffuse_horizontal_radiation_wh_per_m2: 0.0,
                wind_direction_deg: 0.0,
                wind_speed_m_per_s: 0.0,
                liquid_precipitation_depth_mm: 0.0,
            })
            .collect()
    }

    #[test]
    fn state_defaults_to_first_timestep() {
        let state = SimulationState::new(SimulationMode::Compatibility);

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.mode, SimulationMode::Compatibility);
        assert!(state.zones.is_empty());
    }

    #[test]
    fn solar_weather_interpolation_matches_energyplus_even_timestep_weights() {
        assert_eq!(solar_weather_interpolation_weights(1, 1), (0.0, 1.0, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 1), (0.25, 0.75, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 2), (0.0, 1.0, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 3), (0.0, 0.75, 0.25));
        assert_eq!(solar_weather_interpolation_weights(4, 4), (0.0, 0.5, 0.5));
    }

    #[test]
    fn solar_next_hour_record_wraps_within_each_accepted_day() {
        let records = two_day_solar_weather_records();

        assert!(std::ptr::eq(
            next_solar_weather_record_within_day(&records, 23),
            &records[0]
        ));
        assert!(std::ptr::eq(
            next_solar_weather_record_within_day(&records, 47),
            &records[24]
        ));
    }

    #[test]
    fn hour_24_solar_is_independent_of_the_next_accepted_days_hour_1() {
        let site = SiteLocation {
            name: NormalizedName::new("Date Line Test Site"),
            latitude_deg: 0.0,
            longitude_deg: -180.0,
            time_zone_hours: 0.0,
            elevation_m: 0.0,
        };
        let roof = surface(
            100,
            "Hour 24 Roof",
            SurfaceType::Roof,
            [
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        );
        let mut records = two_day_solar_weather_records();
        records[0].direct_normal_radiation_wh_per_m2 = 300.0;
        records[23].direct_normal_radiation_wh_per_m2 = 100.0;
        records[24].direct_normal_radiation_wh_per_m2 = 0.0;
        let baseline = surface_incident_solar_radiation_for_weather_context_w_per_m2(
            &roof,
            &site,
            &records,
            23,
            4,
            Some(4),
            FirstHourInterpolationStartingValues::Hour24,
        );

        records[24].direct_normal_radiation_wh_per_m2 = 10_000.0;
        let changed_next_day = surface_incident_solar_radiation_for_weather_context_w_per_m2(
            &roof,
            &site,
            &records,
            23,
            4,
            Some(4),
            FirstHourInterpolationStartingValues::Hour24,
        );

        assert!(baseline > 0.0);
        assert!((baseline - changed_next_day).abs() < 1.0e-12);
    }

    #[test]
    fn weather_timestep_series_precomputes_full_weather_fields() {
        let previous = EpwRecord {
            year: 2013,
            month: 1,
            day: 1,
            hour: 1,
            minute: 60,
            dry_bulb_c: 10.0,
            dew_point_c: 2.0,
            relative_humidity_percent: 40.0,
            atmospheric_pressure_pa: 80_000.0,
            horizontal_infrared_radiation_wh_per_m2: 300.0,
            global_horizontal_radiation_wh_per_m2: 200.0,
            direct_normal_radiation_wh_per_m2: 100.0,
            diffuse_horizontal_radiation_wh_per_m2: 50.0,
            wind_direction_deg: 350.0,
            wind_speed_m_per_s: 2.0,
            liquid_precipitation_depth_mm: 0.0,
        };
        let current = EpwRecord {
            dry_bulb_c: 22.0,
            relative_humidity_percent: 80.0,
            atmospheric_pressure_pa: 84_000.0,
            horizontal_infrared_radiation_wh_per_m2: 500.0,
            global_horizontal_radiation_wh_per_m2: 600.0,
            direct_normal_radiation_wh_per_m2: 300.0,
            diffuse_horizontal_radiation_wh_per_m2: 150.0,
            wind_direction_deg: 10.0,
            wind_speed_m_per_s: 10.0,
            liquid_precipitation_depth_mm: 1.0,
            ..previous
        };
        let records = [previous, current];
        let series = precompute_weather_timestep_series(
            &records,
            4,
            FirstHourInterpolationStartingValues::Hour24,
        );
        let sample = series.sample_for(1, 2).expect("precomputed sample");

        assert_eq!(series.hourly_records(), &records);
        assert_eq!(series.timestep_dry_bulb_c().len(), 8);
        assert_eq!(series.timestep_wet_bulb_c().len(), 8);
        assert_eq!(series.timestep_direct_normal_radiation_w_per_m2().len(), 8);
        assert!((sample.dry_bulb_c - 16.0).abs() < 1.0e-12);
        assert!((sample.relative_humidity_percent - 60.0).abs() < 1.0e-12);
        assert!((sample.atmospheric_pressure_pa - 82_000.0).abs() < 1.0e-12);
        assert!((sample.wind_speed_m_per_s - 6.0).abs() < 1.0e-12);
        assert!((sample.wind_direction_deg - 0.0).abs() < 1.0e-12);
        assert!((sample.global_horizontal_radiation_w_per_m2 - 400.0).abs() < 1.0e-12);
        assert!((sample.direct_normal_radiation_w_per_m2 - 200.0).abs() < 1.0e-12);
        assert!((sample.diffuse_horizontal_radiation_w_per_m2 - 100.0).abs() < 1.0e-12);
        assert!((sample.horizontal_infrared_radiation_w_per_m2 - 400.0).abs() < 1.0e-12);
        assert!((sample.liquid_precipitation_depth_mm - 0.5).abs() < 1.0e-12);
        assert!(sample.wet_bulb_c.is_finite());
        assert!(sample.outdoor_humidity_ratio.is_finite());
    }

    #[test]
    fn energyplus_daily_solar_coefficients_match_reference_day() {
        let (sin_declination, _cos_declination, equation_of_time_hours) =
            energyplus_daily_solar_coefficients(1);

        assert!((sin_declination - -0.392204631085).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.055895327979).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_weather_record_day_of_year_ignores_tmy_source_leap_year() {
        let mut record = EpwRecord {
            year: 2004,
            month: 3,
            day: 1,
            hour: 1,
            minute: 60,
            dry_bulb_c: 0.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 0.0,
            atmospheric_pressure_pa: 101_325.0,
            horizontal_infrared_radiation_wh_per_m2: 0.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        };

        assert_eq!(energyplus_weather_record_day_of_year(&record), Some(60));

        record.month = 4;
        record.day = 6;
        assert_eq!(energyplus_weather_record_day_of_year(&record), Some(96));

        record.year = 2013;
        assert_eq!(energyplus_weather_record_day_of_year(&record), Some(96));
    }

    #[test]
    fn energyplus_average_solar_coefficients_match_shadowing_period() {
        let (sin_declination, cos_declination, equation_of_time_hours) =
            energyplus_average_solar_coefficients(61, 20);

        assert!((sin_declination - -0.065802703719632).abs() < 1.0e-12);
        assert!((cos_declination - 0.997832653395942).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.168373861452452).abs() < 1.0e-12);
    }

    #[test]
    fn shadowing_period_solar_coefficients_use_energyplus_update_frequency() {
        let mut records = Vec::new();
        let mut date = Date {
            year: 2013,
            month: 1,
            day_of_month: 1,
        };
        for _day in 0..80 {
            for hour in 1..=24 {
                records.push(EpwRecord {
                    year: date.year,
                    month: date.month,
                    day: date.day_of_month,
                    hour,
                    minute: 60,
                    dry_bulb_c: 0.0,
                    dew_point_c: 0.0,
                    relative_humidity_percent: 0.0,
                    atmospheric_pressure_pa: 101_325.0,
                    horizontal_infrared_radiation_wh_per_m2: 0.0,
                    global_horizontal_radiation_wh_per_m2: 0.0,
                    direct_normal_radiation_wh_per_m2: 0.0,
                    diffuse_horizontal_radiation_wh_per_m2: 0.0,
                    wind_direction_deg: 0.0,
                    wind_speed_m_per_s: 0.0,
                    liquid_precipitation_depth_mm: 0.0,
                });
            }
            date = next_day(date);
        }

        let coefficients = energyplus_shadowing_period_solar_coefficients(&records, 1450);
        assert!(coefficients.is_some());
        let (sin_declination, cos_declination, equation_of_time_hours) =
            coefficients.unwrap_or((0.0, 0.0, 0.0));

        assert!((sin_declination - -0.065802703719632).abs() < 1.0e-12);
        assert!((cos_declination - 0.997832653395942).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.168373861452452).abs() < 1.0e-12);
    }

    #[test]
    fn solar_position_uses_energyplus_hour_angle_convention() {
        let site = SiteLocation {
            name: NormalizedName::new("Chicago"),
            latitude_deg: 41.78,
            longitude_deg: -87.75,
            time_zone_hours: -6.0,
            elevation_m: 190.0,
        };
        let record = EpwRecord {
            year: 2013,
            month: 1,
            day: 1,
            hour: 12,
            minute: 60,
            dry_bulb_c: 0.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 0.0,
            atmospheric_pressure_pa: 101_325.0,
            horizontal_infrared_radiation_wh_per_m2: 0.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        };

        let position = solar_position_rad_at_local_hour(&site, &record, 12.0);
        assert!(position.is_some());
        let (altitude_rad, azimuth_rad) = position.unwrap_or((0.0, 0.0));

        assert!((altitude_rad.to_degrees() - 25.115079268192).abs() < 1.0e-12);
        assert!((azimuth_rad.to_degrees() - 181.434056277464).abs() < 1.0e-12);
    }

    #[test]
    fn surface_solar_uses_shadowing_sunlit_fraction_at_sunrise_edge() {
        let site = SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.74,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        };
        let mut records = Vec::new();
        let mut record_index = None;
        let mut date = Date {
            year: 2004,
            month: 1,
            day_of_month: 1,
        };
        for _day in 0..340 {
            for hour in 1..=24 {
                if date.month == 11 && date.day_of_month == 19 && hour == 7 {
                    record_index = Some(records.len());
                }
                let (direct_normal_radiation_wh_per_m2, diffuse_horizontal_radiation_wh_per_m2) =
                    if date.month == 11 && date.day_of_month == 19 && hour == 8 {
                        (279.0, 56.0)
                    } else {
                        (0.0, 0.0)
                    };
                records.push(EpwRecord {
                    year: date.year,
                    month: date.month,
                    day: date.day_of_month,
                    hour,
                    minute: 0,
                    dry_bulb_c: 0.0,
                    dew_point_c: 0.0,
                    relative_humidity_percent: 50.0,
                    atmospheric_pressure_pa: 82_000.0,
                    horizontal_infrared_radiation_wh_per_m2: 0.0,
                    global_horizontal_radiation_wh_per_m2: 0.0,
                    direct_normal_radiation_wh_per_m2,
                    diffuse_horizontal_radiation_wh_per_m2,
                    wind_direction_deg: 0.0,
                    wind_speed_m_per_s: 0.0,
                    liquid_precipitation_depth_mm: 0.0,
                });
            }
            date = next_day(date);
        }
        let roof = surface(
            100,
            "Sunrise Roof",
            SurfaceType::Roof,
            [
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        );

        let incident = surface_incident_solar_radiation_for_weather_context_w_per_m2(
            &roof,
            &site,
            &records,
            record_index.unwrap_or(0),
            4,
            None,
            FirstHourInterpolationStartingValues::Hour24,
        );

        assert!((incident - 6.003845309857875).abs() < 1.0e-9);
    }

    #[test]
    fn horizontal_roof_sky_diffuse_matches_energyplus_shadowing_sunrise_edge() {
        let site = SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.74,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        };
        let mut records = Vec::new();
        let mut record_index = None;
        let mut date = Date {
            year: 2004,
            month: 1,
            day_of_month: 1,
        };
        for _day in 0..117 {
            for hour in 1..=24 {
                if date.month == 4 && date.day_of_month == 26 && hour == 6 {
                    record_index = Some(records.len());
                }
                let (direct_normal_radiation_wh_per_m2, diffuse_horizontal_radiation_wh_per_m2) =
                    if date.month == 4 && date.day_of_month == 26 && hour == 6 {
                        (0.0, 42.0)
                    } else if date.month == 4 && date.day_of_month == 26 && hour == 7 {
                        (626.0, 70.0)
                    } else {
                        (0.0, 0.0)
                    };
                records.push(EpwRecord {
                    year: date.year,
                    month: date.month,
                    day: date.day_of_month,
                    hour,
                    minute: 0,
                    dry_bulb_c: 0.0,
                    dew_point_c: 0.0,
                    relative_humidity_percent: 50.0,
                    atmospheric_pressure_pa: 82_000.0,
                    horizontal_infrared_radiation_wh_per_m2: 0.0,
                    global_horizontal_radiation_wh_per_m2: 0.0,
                    direct_normal_radiation_wh_per_m2,
                    diffuse_horizontal_radiation_wh_per_m2,
                    wind_direction_deg: 0.0,
                    wind_speed_m_per_s: 0.0,
                    liquid_precipitation_depth_mm: 0.0,
                });
            }
            date = next_day(date);
        }
        let roof = surface(
            100,
            "Spring Sunrise Roof",
            SurfaceType::Roof,
            [
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        );

        let components = surface_incident_solar_components_hourly_average_w_per_m2(
            &roof,
            &site,
            &records,
            record_index.unwrap_or(0),
            4,
        );

        assert!((components.sky_diffuse_w_per_m2 - 42.517992377816).abs() < 1.0e-9);
    }

    #[test]
    fn anisotropic_sky_circumsolar_uses_sunlit_fraction() {
        let site = SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.74,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        };
        let wall = surface(
            101,
            "South Wall",
            SurfaceType::Wall,
            [
                point(0.0, 0.0, 0.0),
                point(1.0, 0.0, 0.0),
                point(1.0, 0.0, 1.0),
                point(0.0, 0.0, 1.0),
            ],
        );

        let shadowed = energyplus_anisotropic_sky_multiplier(
            &wall,
            &site,
            90.0_f64.to_radians(),
            20.0_f64.to_radians(),
            500.0,
            100.0,
            0.6,
            0.0,
        );
        let sunlit = energyplus_anisotropic_sky_multiplier(
            &wall,
            &site,
            90.0_f64.to_radians(),
            20.0_f64.to_radians(),
            500.0,
            100.0,
            0.6,
            1.0,
        );

        assert!(shadowed > 0.0);
        assert!(sunlit > shadowed);
    }

    #[test]
    fn constant_schedule_trace_repeats_hourly_value() {
        let mut model = TypedModel::default();
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });

        let traces = simulate_constant_schedules(&model, 3);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].schedule_name, "ALWAYSON");
        assert_eq!(traces[0].values, vec![1.0, 1.0, 1.0]);
        assert_eq!(
            traces[0].kind,
            ScheduleSeriesKind::ConstantScalar { value: 1.0 }
        );
    }

    #[test]
    fn compact_schedule_trace_uses_until_segments()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut model = TypedModel::default();
        model.compact_schedules.push(ScheduleCompact {
            id: ScheduleId(0),
            name: NormalizedName::new("Office Occupancy"),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![ScheduleCompactDayProfile {
                    day_types: all_schedule_day_types(),
                    interpolation: ScheduleInterpolation::No,
                    segments: vec![
                        ScheduleCompactSegment {
                            until_minute_of_day: 8 * 60,
                            value: 0.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 18 * 60,
                            value: 1.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 24 * 60,
                            value: 0.0,
                        },
                    ],
                }],
            }],
        });

        let traces = simulate_schedule_values(&model, 24)?;

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].values[7], 0.0);
        assert_eq!(traces[0].values[8], 1.0);
        assert_eq!(traces[0].values[17], 1.0);
        assert_eq!(traces[0].values[18], 0.0);
        match &traces[0].kind {
            ScheduleSeriesKind::CompactIntervals { intervals } => {
                assert_eq!(intervals.len(), 3);
                assert_eq!(intervals[0].start_minute_of_day, 1);
                assert_eq!(intervals[0].end_minute_of_day, 8 * 60);
                assert_eq!(intervals[1].start_minute_of_day, 8 * 60 + 1);
                assert_eq!(intervals[1].end_minute_of_day, 18 * 60);
            }
            other => {
                return Err(std::io::Error::other(format!(
                    "expected compact intervals, got {other:?}"
                ))
                .into());
            }
        }

        Ok(())
    }

    #[test]
    fn schedule_value_series_precomputes_supported_schedules()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut model = TypedModel::default();
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 0.75,
        });

        let series = precompute_schedule_value_series(&model, 4)?;

        assert_eq!(series.len(), 1);
        assert_eq!(series[0].schedule_name, "ALWAYSON");
        assert_eq!(series[0].values, vec![0.75, 0.75, 0.75, 0.75]);
        Ok(())
    }

    #[test]
    fn schedule_value_series_can_compile_from_time_axis() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut model = TypedModel::default();
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 0.5,
        });
        model.run_periods.push(RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Two Days"),
            begin_month: 1,
            begin_day_of_month: 1,
            begin_year: Some(2013),
            end_month: 1,
            end_day_of_month: 2,
            end_year: Some(2013),
            day_of_week_for_start_day: None,
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour1,
            use_weather_file_holidays_and_special_days: true,
            use_weather_file_daylight_saving_period: true,
            apply_weekend_holiday_rule: true,
            use_weather_file_rain_indicators: true,
            use_weather_file_snow_indicators: true,
            treat_weather_as_actual: false,
        });
        let axis = build_hourly_time_axis(&model)?;
        let series = precompute_schedule_value_series_for_time_axis(&model, &axis);

        assert_eq!(
            axis.first_hour_interpolation_starting_values,
            FirstHourInterpolationStartingValues::Hour1
        );
        assert_eq!(axis.zone_timestep.timesteps_per_hour, 6);
        assert_eq!(axis.zone_timestep.timestep_seconds, 600.0);
        assert_eq!(axis.system_timestep.nominal_timestep_seconds, 600.0);
        assert_eq!(
            axis.system_timestep.variable_system_timestep_support,
            "placeholder-state-backed"
        );
        assert!(axis.system_timestep.shorten_timestep_sys_state);
        assert!(axis.system_timestep.use_zone_timestep_history_state);
        assert_eq!(axis.sample_partitions.warmup_reported_samples, 0);
        assert_eq!(axis.sample_partitions.run_period_reported_samples, 48);
        assert_eq!(axis.sample_partitions.design_day_reported_samples, 0);
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].values.len(), axis.sample_count());
        assert_eq!(series[0].values[0], 0.5);

        Ok(())
    }

    #[test]
    fn compact_schedule_time_axis_consumes_cross_year_period_day_type_and_hour()
    -> Result<(), Box<dyn std::error::Error>> {
        let schedule_id = ScheduleId(41);
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 1,
            },
            run_periods: vec![RunPeriod {
                id: RunPeriodId(0),
                name: NormalizedName::new("Cross Year Schedule"),
                begin_month: 12,
                begin_day_of_month: 30,
                begin_year: Some(2031),
                end_month: 1,
                end_day_of_month: 3,
                end_year: Some(2032),
                day_of_week_for_start_day: Some(DayOfWeek::Tuesday),
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
                use_weather_file_holidays_and_special_days: false,
                use_weather_file_daylight_saving_period: false,
                apply_weekend_holiday_rule: false,
                use_weather_file_rain_indicators: false,
                use_weather_file_snow_indicators: false,
                treat_weather_as_actual: false,
            }],
            run_period_special_days: vec![RunPeriodSpecialDay {
                id: RunPeriodSpecialDayId(0),
                name: NormalizedName::new("January Second Holiday"),
                start_date: CalendarDateRule::MonthDay {
                    month: 1,
                    day_of_month: 2,
                },
                duration_days: 1,
                special_day_type: SpecialDayType::Holiday,
            }],
            compact_schedules: vec![cross_year_day_type_compact_schedule(schedule_id)],
            ..TypedModel::default()
        };

        let axis = build_hourly_time_axis(&model)?;
        assert_eq!(axis.sample_count(), 120);
        assert_eq!(
            axis.points
                .chunks_exact(24)
                .map(|day| day[0].schedule_day_of_year)
                .collect::<Vec<_>>(),
            vec![365, 366, 1, 2, 3]
        );
        assert_eq!(
            axis.points
                .chunks_exact(24)
                .map(|day| day[0].day_type)
                .collect::<Vec<_>>(),
            vec![
                crate::DayType::Tuesday,
                crate::DayType::Wednesday,
                crate::DayType::Thursday,
                crate::DayType::Holiday,
                crate::DayType::Saturday,
            ]
        );

        let series = precompute_schedule_value_series_for_time_axis(&model, &axis);
        assert_eq!(series.len(), 1);
        assert_eq!(series[0].schedule_id, schedule_id);
        assert_eq!(series[0].values.len(), 120);
        assert_eq!(
            series[0]
                .values
                .chunks_exact(24)
                .map(|day| day[0])
                .collect::<Vec<_>>(),
            vec![103.0, 104.0, 105.0, 108.0, 199.0]
        );
        assert!(
            series[0]
                .values
                .chunks_exact(24)
                .all(|day| day.iter().all(|value| *value == day[0]))
        );
        match &series[0].kind {
            ScheduleSeriesKind::CompactCalendarProfiles { periods } => {
                assert_eq!(periods.len(), 2);
                assert_eq!(periods[0].through_schedule_day_of_year, 1);
                assert_eq!(periods[1].through_schedule_day_of_year, 366);
            }
            other => {
                return Err(std::io::Error::other(format!(
                    "expected compact calendar profiles, got {other:?}"
                ))
                .into());
            }
        }

        Ok(())
    }

    #[test]
    fn compact_schedule_time_axis_selects_until_segment_by_hour()
    -> Result<(), Box<dyn std::error::Error>> {
        let schedule_id = ScheduleId(44);
        let model = TypedModel {
            compact_schedules: vec![ScheduleCompact {
                id: schedule_id,
                name: NormalizedName::new("Calendar Aware Until"),
                schedule_type_limits: None,
                periods: vec![ScheduleCompactPeriod {
                    through_schedule_day_of_year: 366,
                    day_profiles: vec![ScheduleCompactDayProfile {
                        day_types: all_schedule_day_types(),
                        interpolation: ScheduleInterpolation::No,
                        segments: vec![
                            ScheduleCompactSegment {
                                until_minute_of_day: 8 * 60,
                                value: 1.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 24 * 60,
                                value: 2.0,
                            },
                        ],
                    }],
                }],
            }],
            ..TypedModel::default()
        };
        let axis = build_hourly_time_axis(&model)?;
        let series = precompute_schedule_value_series_for_time_axis(&model, &axis);

        assert_eq!(series.len(), 1);
        assert_eq!(series[0].values[..8], [1.0; 8]);
        assert_eq!(series[0].values[8..], [2.0; 16]);
        Ok(())
    }

    #[test]
    fn compact_schedule_environment_axis_selects_each_zone_timestep_end_minute()
    -> Result<(), Box<dyn std::error::Error>> {
        let constant_id = ScheduleId(48);
        let detailed_id = ScheduleId(49);
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            run_periods: vec![RunPeriod {
                id: RunPeriodId(0),
                name: NormalizedName::new("Zone Timestep Schedule"),
                begin_month: 1,
                begin_day_of_month: 1,
                begin_year: Some(2017),
                end_month: 1,
                end_day_of_month: 1,
                end_year: Some(2017),
                day_of_week_for_start_day: Some(DayOfWeek::Sunday),
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
                use_weather_file_holidays_and_special_days: false,
                use_weather_file_daylight_saving_period: false,
                apply_weekend_holiday_rule: false,
                use_weather_file_rain_indicators: false,
                use_weather_file_snow_indicators: false,
                treat_weather_as_actual: false,
            }],
            schedules: vec![ScheduleConstant {
                id: constant_id,
                name: NormalizedName::new("Zone Timestep Constant"),
                schedule_type_limits: None,
                hourly_value: 0.5,
            }],
            compact_schedules: vec![ScheduleCompact {
                id: detailed_id,
                name: NormalizedName::new("Zone Timestep Detailed"),
                schedule_type_limits: None,
                periods: vec![ScheduleCompactPeriod {
                    through_schedule_day_of_year: 366,
                    day_profiles: vec![ScheduleCompactDayProfile {
                        day_types: all_schedule_day_types(),
                        interpolation: ScheduleInterpolation::No,
                        segments: vec![
                            ScheduleCompactSegment {
                                until_minute_of_day: 15,
                                value: 11.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 30,
                                value: 12.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 45,
                                value: 13.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 60,
                                value: 14.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 23 * 60,
                                value: 20.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 23 * 60 + 15,
                                value: 21.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 23 * 60 + 30,
                                value: 22.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 23 * 60 + 45,
                                value: 23.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 24 * 60,
                                value: 24.0,
                            },
                        ],
                    }],
                }],
            }],
            ..TypedModel::default()
        };

        let environment_axes = build_environment_time_axes(&model)?;
        assert_eq!(environment_axes.len(), 1);
        let environment_axis = &environment_axes[0];
        assert_eq!(environment_axis.sample_count(), 96);
        assert_eq!(environment_axis.points[0].zone_timestep, 1);
        assert_eq!(environment_axis.points[0].end_minute, 15.0);
        assert_eq!(environment_axis.points[3].zone_timestep, 4);
        assert_eq!(environment_axis.points[3].end_minute, 60.0);
        assert_eq!(environment_axis.points[4].hour, 2);
        assert_eq!(environment_axis.points[4].zone_timestep, 1);

        let series =
            precompute_schedule_value_series_for_environment_time_axis(&model, environment_axis);
        let constant = series
            .iter()
            .find(|trace| trace.schedule_id == constant_id)
            .expect("constant environment-timestep series");
        assert_eq!(constant.values, vec![0.5; 96]);

        let detailed = series
            .iter()
            .find(|trace| trace.schedule_id == detailed_id)
            .expect("detailed environment-timestep series");
        let mut expected = vec![11.0, 12.0, 13.0, 14.0];
        expected.extend(vec![20.0; 88]);
        expected.extend([21.0, 22.0, 23.0, 24.0]);
        assert_eq!(detailed.values, expected);

        assert_eq!(
            normalized_environment_timestep_timestamp_label(
                environment_axis,
                &environment_axis.points[0],
            ),
            "env=ZONE TIMESTEP SCHEDULE;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=15.00;day_type=Sunday"
        );
        assert_eq!(
            normalized_environment_timestep_timestamp_label(
                environment_axis,
                &environment_axis.points[95],
            ),
            "env=ZONE TIMESTEP SCHEDULE;day=1;month=1;date=1;dst=0;hour=24;start=45.00;end=60.00;day_type=Sunday"
        );

        Ok(())
    }

    #[test]
    fn compact_schedule_environment_axis_applies_no_average_and_linear_interpolation()
    -> Result<(), Box<dyn std::error::Error>> {
        let compact_schedule =
            |id: u32, name: &str, interpolation: ScheduleInterpolation| ScheduleCompact {
                id: ScheduleId(id),
                name: NormalizedName::new(name),
                schedule_type_limits: None,
                periods: vec![ScheduleCompactPeriod {
                    through_schedule_day_of_year: 366,
                    day_profiles: vec![ScheduleCompactDayProfile {
                        day_types: all_schedule_day_types(),
                        interpolation,
                        segments: vec![
                            ScheduleCompactSegment {
                                until_minute_of_day: 20,
                                value: 0.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 60,
                                value: 60.0,
                            },
                            ScheduleCompactSegment {
                                until_minute_of_day: 1440,
                                value: 60.0,
                            },
                        ],
                    }],
                }],
            };
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            run_periods: vec![RunPeriod {
                id: RunPeriodId(0),
                name: NormalizedName::new("Interpolation Day"),
                begin_month: 1,
                begin_day_of_month: 1,
                begin_year: Some(2032),
                end_month: 1,
                end_day_of_month: 1,
                end_year: Some(2032),
                day_of_week_for_start_day: Some(DayOfWeek::Thursday),
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
                use_weather_file_holidays_and_special_days: false,
                use_weather_file_daylight_saving_period: false,
                apply_weekend_holiday_rule: false,
                use_weather_file_rain_indicators: false,
                use_weather_file_snow_indicators: false,
                treat_weather_as_actual: false,
            }],
            compact_schedules: vec![
                compact_schedule(60, "No Interpolation", ScheduleInterpolation::No),
                compact_schedule(61, "Average Interpolation", ScheduleInterpolation::Average),
                compact_schedule(62, "Linear Interpolation", ScheduleInterpolation::Linear),
            ],
            ..TypedModel::default()
        };

        let environment_axes = build_environment_time_axes(&model)?;
        let series = precompute_schedule_value_series_for_environment_time_axis(
            &model,
            &environment_axes[0],
        );
        for (schedule_id, expected_first_hour) in [
            (ScheduleId(60), [0.0, 60.0, 60.0, 60.0]),
            (ScheduleId(61), [0.0, 40.0, 60.0, 60.0]),
            (ScheduleId(62), [0.0, 15.0, 37.5, 60.0]),
        ] {
            let trace = series
                .iter()
                .find(|trace| trace.schedule_id == schedule_id)
                .expect("interpolated schedule trace");
            assert_eq!(trace.values.len(), 96);
            assert_eq!(trace.values[..4], expected_first_hour);
            assert!(trace.values[4..].iter().all(|value| *value == 60.0));
            let ScheduleSeriesKind::CompactCalendarProfiles { periods } = &trace.kind else {
                return Err(std::io::Error::other(
                    "expected compiled compact calendar profiles",
                )
                .into());
            };
            assert_eq!(periods[0].day_profiles[0].zone_timestep_values.len(), 96);
            assert_eq!(
                periods[0].day_profiles[0].zone_timestep_values,
                trace.values
            );
        }

        Ok(())
    }

    #[test]
    fn compact_schedule_linear_interpolation_keeps_first_interval_flat_across_hour_boundary() {
        let schedule = |interpolation| ScheduleCompact {
            id: ScheduleId(63),
            name: NormalizedName::new("Linear Source Order"),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![ScheduleCompactDayProfile {
                    day_types: all_schedule_day_types(),
                    interpolation,
                    segments: vec![
                        ScheduleCompactSegment {
                            until_minute_of_day: 20,
                            value: 10.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 75,
                            value: 175.0,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 1440,
                            value: 175.0,
                        },
                    ],
                }],
            }],
        };

        for (interpolation, expected) in [
            (
                ScheduleInterpolation::No,
                [10.0, 175.0, 175.0, 175.0, 175.0],
            ),
            (
                ScheduleInterpolation::Average,
                [10.0, 120.0, 175.0, 175.0, 175.0],
            ),
            (
                ScheduleInterpolation::Linear,
                [10.0, 40.0, 85.0, 130.0, 175.0],
            ),
        ] {
            let periods = precompile_compact_schedule_periods(&schedule(interpolation), 4);
            assert_eq!(
                periods[0].day_profiles[0].zone_timestep_values[..5],
                expected
            );
        }
    }

    #[test]
    fn detailed_schedule_environment_axis_preserves_zone_timestep_across_dst_shift_and_wrap()
    -> Result<(), Box<dyn std::error::Error>> {
        let schedule_id = ScheduleId(50);
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            run_periods: vec![RunPeriod {
                id: RunPeriodId(0),
                name: NormalizedName::new("DST Zone Timestep Wrap"),
                begin_month: 12,
                begin_day_of_month: 31,
                begin_year: Some(2032),
                end_month: 12,
                end_day_of_month: 31,
                end_year: Some(2032),
                day_of_week_for_start_day: Some(DayOfWeek::Friday),
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
                use_weather_file_holidays_and_special_days: false,
                use_weather_file_daylight_saving_period: false,
                apply_weekend_holiday_rule: false,
                use_weather_file_rain_indicators: false,
                use_weather_file_snow_indicators: false,
                treat_weather_as_actual: false,
            }],
            run_period_daylight_saving_time: Some(ep_model::RunPeriodDaylightSavingTime {
                start_date: CalendarDateRule::MonthDay {
                    month: 12,
                    day_of_month: 31,
                },
                end_date: CalendarDateRule::MonthDay {
                    month: 12,
                    day_of_month: 31,
                },
            }),
            compact_schedules: vec![ScheduleCompact {
                id: schedule_id,
                name: NormalizedName::new("DST Zone Timestep Detailed"),
                schedule_type_limits: None,
                periods: vec![
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 1,
                        day_profiles: vec![ScheduleCompactDayProfile {
                            day_types: all_schedule_day_types(),
                            interpolation: ScheduleInterpolation::No,
                            segments: vec![
                                ScheduleCompactSegment {
                                    until_minute_of_day: 15,
                                    value: 101.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 30,
                                    value: 102.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 45,
                                    value: 103.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60,
                                    value: 104.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 24 * 60,
                                    value: 109.0,
                                },
                            ],
                        }],
                    },
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 366,
                        day_profiles: vec![ScheduleCompactDayProfile {
                            day_types: all_schedule_day_types(),
                            interpolation: ScheduleInterpolation::No,
                            segments: vec![
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60,
                                    value: 200.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60 + 15,
                                    value: 211.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60 + 30,
                                    value: 212.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60 + 45,
                                    value: 213.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 2 * 60,
                                    value: 214.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 24 * 60,
                                    value: 220.0,
                                },
                            ],
                        }],
                    },
                ],
            }],
            ..TypedModel::default()
        };

        let environment_axes = build_environment_time_axes(&model)?;
        let environment_axis = &environment_axes[0];
        assert_eq!(environment_axis.sample_count(), 96);
        assert!(environment_axis.points.iter().all(|point| point.dst));
        assert_eq!(environment_axis.points[95].schedule_day_of_year, 366);

        let series =
            precompute_schedule_value_series_for_environment_time_axis(&model, environment_axis);
        let mut expected = vec![211.0, 212.0, 213.0, 214.0];
        expected.extend(vec![220.0; 88]);
        expected.extend([101.0, 102.0, 103.0, 104.0]);
        assert_eq!(series[0].values, expected);

        Ok(())
    }

    #[test]
    fn detailed_schedule_dst_shift_uses_tomorrow_type_and_final_stale_type()
    -> Result<(), Box<dyn std::error::Error>> {
        let constant_id = ScheduleId(45);
        let detailed_id = ScheduleId(46);
        let non_holiday_day_types = all_schedule_day_types()
            .into_iter()
            .filter(|day_type| *day_type != ScheduleDayType::Holiday)
            .collect::<Vec<_>>();
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 1,
            },
            run_periods: vec![RunPeriod {
                id: RunPeriodId(0),
                name: NormalizedName::new("DST Schedule Rollover"),
                begin_month: 10,
                begin_day_of_month: 30,
                begin_year: Some(2032),
                end_month: 11,
                end_day_of_month: 1,
                end_year: Some(2032),
                day_of_week_for_start_day: Some(DayOfWeek::Saturday),
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
                use_weather_file_holidays_and_special_days: false,
                use_weather_file_daylight_saving_period: false,
                apply_weekend_holiday_rule: false,
                use_weather_file_rain_indicators: false,
                use_weather_file_snow_indicators: false,
                treat_weather_as_actual: false,
            }],
            run_period_daylight_saving_time: Some(ep_model::RunPeriodDaylightSavingTime {
                start_date: CalendarDateRule::MonthDay {
                    month: 10,
                    day_of_month: 31,
                },
                end_date: CalendarDateRule::MonthDay {
                    month: 11,
                    day_of_month: 1,
                },
            }),
            run_period_special_days: vec![RunPeriodSpecialDay {
                id: RunPeriodSpecialDayId(0),
                name: NormalizedName::new("Final Rollover Holiday"),
                start_date: CalendarDateRule::MonthDay {
                    month: 11,
                    day_of_month: 1,
                },
                duration_days: 1,
                special_day_type: SpecialDayType::Holiday,
            }],
            schedules: vec![ScheduleConstant {
                id: constant_id,
                name: NormalizedName::new("DST Independent Constant"),
                schedule_type_limits: None,
                hourly_value: 42.0,
            }],
            compact_schedules: vec![ScheduleCompact {
                id: detailed_id,
                name: NormalizedName::new("DST Final Rollover"),
                schedule_type_limits: None,
                periods: vec![
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 304,
                        day_profiles: vec![ScheduleCompactDayProfile {
                            day_types: all_schedule_day_types(),
                            interpolation: ScheduleInterpolation::No,
                            segments: vec![
                                ScheduleCompactSegment {
                                    until_minute_of_day: 23 * 60,
                                    value: 100.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 24 * 60,
                                    value: 124.0,
                                },
                            ],
                        }],
                    },
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 305,
                        day_profiles: vec![ScheduleCompactDayProfile {
                            day_types: all_schedule_day_types(),
                            interpolation: ScheduleInterpolation::No,
                            segments: vec![
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60,
                                    value: 201.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 24 * 60,
                                    value: 200.0,
                                },
                            ],
                        }],
                    },
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 306,
                        day_profiles: vec![
                            ScheduleCompactDayProfile {
                                day_types: vec![ScheduleDayType::Holiday],
                                interpolation: ScheduleInterpolation::No,
                                segments: vec![
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 60,
                                        value: 801.0,
                                    },
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 24 * 60,
                                        value: 800.0,
                                    },
                                ],
                            },
                            ScheduleCompactDayProfile {
                                day_types: non_holiday_day_types.clone(),
                                interpolation: ScheduleInterpolation::No,
                                segments: vec![
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 60,
                                        value: 301.0,
                                    },
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 24 * 60,
                                        value: 300.0,
                                    },
                                ],
                            },
                        ],
                    },
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 366,
                        day_profiles: vec![
                            ScheduleCompactDayProfile {
                                day_types: vec![ScheduleDayType::Holiday],
                                interpolation: ScheduleInterpolation::No,
                                segments: vec![
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 60,
                                        value: 901.0,
                                    },
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 24 * 60,
                                        value: 900.0,
                                    },
                                ],
                            },
                            ScheduleCompactDayProfile {
                                day_types: non_holiday_day_types,
                                interpolation: ScheduleInterpolation::No,
                                segments: vec![
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 60,
                                        value: 401.0,
                                    },
                                    ScheduleCompactSegment {
                                        until_minute_of_day: 24 * 60,
                                        value: 400.0,
                                    },
                                ],
                            },
                        ],
                    },
                ],
            }],
            ..TypedModel::default()
        };

        let axis = build_hourly_time_axis(&model)?;
        assert_eq!(axis.sample_count(), 72);
        assert_eq!(
            axis.points
                .chunks_exact(24)
                .map(|day| (day[0].dst, day[0].day_type, day[0].tomorrow_day_type))
                .collect::<Vec<_>>(),
            vec![
                (
                    false,
                    crate::DayType::Saturday,
                    crate::DayType::Sunday,
                ),
                (true, crate::DayType::Sunday, crate::DayType::Holiday),
                (true, crate::DayType::Holiday, crate::DayType::Holiday),
            ]
        );
        assert_eq!(
            axis.points[71].tomorrow_day_of_week,
            axis.points[71].day_of_week
        );
        assert_eq!(
            axis.points[71].tomorrow_special_day_type,
            axis.points[71].special_day_type
        );

        let series = precompute_schedule_value_series_for_time_axis(&model, &axis);
        let constant = series
            .iter()
            .find(|trace| trace.schedule_id == constant_id)
            .expect("constant schedule series");
        assert_eq!(constant.values, vec![42.0; 72]);
        let detailed = series
            .iter()
            .find(|trace| trace.schedule_id == detailed_id)
            .expect("detailed schedule series");
        let mut expected = vec![100.0; 23];
        expected.push(124.0);
        expected.extend(vec![200.0; 23]);
        expected.push(801.0);
        expected.extend(vec![800.0; 23]);
        expected.push(901.0);
        assert_eq!(detailed.values, expected);

        let environment_axes = build_environment_time_axes(&model)?;
        assert_eq!(environment_axes.len(), 1);
        let environment_axis = &environment_axes[0];
        assert_eq!(
            environment_axis.points[47].tomorrow_day_type,
            crate::DayType::Holiday
        );
        assert_eq!(
            environment_axis.points[71].tomorrow_day_type,
            environment_axis.points[71].day_type
        );
        assert_eq!(
            environment_axis.points[71].tomorrow_day_of_week,
            environment_axis.points[71].day_of_week
        );
        Ok(())
    }

    #[test]
    fn detailed_schedule_dst_hour_24_wraps_schedule_ordinal_367_to_one()
    -> Result<(), Box<dyn std::error::Error>> {
        let schedule_id = ScheduleId(47);
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 1,
            },
            run_periods: vec![RunPeriod {
                id: RunPeriodId(0),
                name: NormalizedName::new("DST Schedule Ordinal Wrap"),
                begin_month: 12,
                begin_day_of_month: 31,
                begin_year: Some(2032),
                end_month: 12,
                end_day_of_month: 31,
                end_year: Some(2032),
                day_of_week_for_start_day: Some(DayOfWeek::Friday),
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
                use_weather_file_holidays_and_special_days: false,
                use_weather_file_daylight_saving_period: false,
                apply_weekend_holiday_rule: false,
                use_weather_file_rain_indicators: false,
                use_weather_file_snow_indicators: false,
                treat_weather_as_actual: false,
            }],
            run_period_daylight_saving_time: Some(ep_model::RunPeriodDaylightSavingTime {
                start_date: CalendarDateRule::MonthDay {
                    month: 12,
                    day_of_month: 31,
                },
                end_date: CalendarDateRule::MonthDay {
                    month: 12,
                    day_of_month: 31,
                },
            }),
            compact_schedules: vec![ScheduleCompact {
                id: schedule_id,
                name: NormalizedName::new("DST Schedule Ordinal Wrap"),
                schedule_type_limits: None,
                periods: vec![
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 1,
                        day_profiles: vec![ScheduleCompactDayProfile {
                            day_types: all_schedule_day_types(),
                            interpolation: ScheduleInterpolation::No,
                            segments: vec![
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60,
                                    value: 11.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 24 * 60,
                                    value: 12.0,
                                },
                            ],
                        }],
                    },
                    ScheduleCompactPeriod {
                        through_schedule_day_of_year: 366,
                        day_profiles: vec![ScheduleCompactDayProfile {
                            day_types: all_schedule_day_types(),
                            interpolation: ScheduleInterpolation::No,
                            segments: vec![
                                ScheduleCompactSegment {
                                    until_minute_of_day: 60,
                                    value: 21.0,
                                },
                                ScheduleCompactSegment {
                                    until_minute_of_day: 24 * 60,
                                    value: 22.0,
                                },
                            ],
                        }],
                    },
                ],
            }],
            ..TypedModel::default()
        };

        let axis = build_hourly_time_axis(&model)?;
        assert!(axis.points.iter().all(|point| point.dst));
        assert_eq!(axis.points[23].schedule_day_of_year, 366);
        assert_eq!(
            axis.points[23].tomorrow_day_type,
            axis.points[23].day_type
        );
        let series = precompute_schedule_value_series_for_time_axis(&model, &axis);
        let mut expected = vec![22.0; 23];
        expected.push(11.0);
        assert_eq!(series[0].values, expected);
        Ok(())
    }

    #[test]
    fn hour_only_schedule_consumers_reject_calendar_variation_and_missing_ids()
    -> Result<(), Box<dyn std::error::Error>> {
        let schedule_id = ScheduleId(42);
        let schedule = day_type_varying_annual_compact_schedule(schedule_id);
        let model = TypedModel {
            compact_schedules: vec![schedule.clone()],
            ..TypedModel::default()
        };
        let error = match precompute_schedule_value_series(&model, 24) {
            Ok(_) => {
                return Err(std::io::Error::other(
                    "hour-only schedule series accepted calendar variation",
                )
                .into());
            }
            Err(error) => error,
        };
        assert!(error.contains("varies by day type"));

        let multi_period_model = TypedModel {
            compact_schedules: vec![cross_year_day_type_compact_schedule(ScheduleId(43))],
            ..TypedModel::default()
        };
        let multi_period_error = match precompute_schedule_value_series(&multi_period_model, 24) {
            Ok(_) => {
                return Err(std::io::Error::other(
                    "hour-only schedule series accepted multiple Through periods",
                )
                .into());
            }
            Err(error) => error,
        };
        assert!(multi_period_error.contains("hour-only consumers require one"));

        let mut gain_model = cube_model();
        gain_model.other_equipment[0].schedule = Some(schedule_id);
        gain_model.compact_schedules.push(schedule);
        let gain_error = simulate_zone_internal_convective_gains(&gain_model, 2)
            .expect_err("calendar-varying internal-gain schedule must be rejected");
        assert!(matches!(
            gain_error,
            RuntimeError::InvalidInternalGainSchedule {
                schedule_id: 42,
                ..
            }
        ));

        let simulation_model = SimulationModel::from_typed(gain_model);
        assert!(matches!(
            initialize_heat_balance_state(&simulation_model, 20.0),
            Err(RuntimeError::InvalidInternalGainSchedule {
                schedule_id: 42,
                ..
            })
        ));
        assert!(matches!(
            simulate_first_zone_uncontrolled(
                &simulation_model,
                &[10.0],
                FirstZoneSimulationOptions::hourly_samples(1),
            ),
            Err(RuntimeError::InvalidInternalGainSchedule {
                schedule_id: 42,
                ..
            })
        ));

        let mut missing_schedule_model = cube_model();
        missing_schedule_model.other_equipment[0].schedule = Some(ScheduleId(999));
        let missing_convective = simulate_zone_internal_convective_gains(&missing_schedule_model, 2)
            .expect_err("missing convective schedule must be rejected");
        let missing_radiant = simulate_zone_internal_radiant_gains(&missing_schedule_model, 2)
            .expect_err("missing radiant schedule must be rejected");
        for error in [missing_convective, missing_radiant] {
            assert!(matches!(
                error,
                RuntimeError::InvalidInternalGainSchedule {
                    schedule_id: 999,
                    ..
                }
            ));
        }

        Ok(())
    }

    #[test]
    fn hour_only_schedule_consumers_fail_closed_when_hourly_aggregation_is_required()
    -> Result<(), Box<dyn std::error::Error>> {
        for (interpolation, segments, expected_error) in [
            (
                ScheduleInterpolation::Average,
                vec![ScheduleCompactSegment {
                    until_minute_of_day: 1440,
                    value: 1.0,
                }],
                "hour-only consumers require Interpolate:No",
            ),
            (
                ScheduleInterpolation::Linear,
                vec![ScheduleCompactSegment {
                    until_minute_of_day: 1440,
                    value: 1.0,
                }],
                "hour-only consumers require Interpolate:No",
            ),
            (
                ScheduleInterpolation::No,
                vec![
                    ScheduleCompactSegment {
                        until_minute_of_day: 20,
                        value: 0.0,
                    },
                    ScheduleCompactSegment {
                        until_minute_of_day: 1440,
                        value: 1.0,
                    },
                ],
                "hour-only consumers require whole-hour boundaries",
            ),
        ] {
            let model = TypedModel {
                timestep: TimestepConfig {
                    number_of_timesteps_per_hour: 4,
                },
                compact_schedules: vec![ScheduleCompact {
                    id: ScheduleId(64),
                    name: NormalizedName::new("Subhourly Interpolation"),
                    schedule_type_limits: None,
                    periods: vec![ScheduleCompactPeriod {
                        through_schedule_day_of_year: 366,
                        day_profiles: vec![ScheduleCompactDayProfile {
                            day_types: all_schedule_day_types(),
                            interpolation,
                            segments,
                        }],
                    }],
                }],
                ..TypedModel::default()
            };

            let error = precompute_schedule_value_series(&model, 24)
                .expect_err("hour-only series must reject interpolation");
            assert!(error.contains(expected_error));

            let axis = build_hourly_time_axis(&model)?;
            let calendar_series =
                precompute_schedule_value_series_for_time_axis(&model, &axis);
            assert!(calendar_series[0].values.iter().all(|value| value.is_nan()));
        }

        Ok(())
    }

    #[test]
    fn zone_internal_convective_gain_trace_excludes_radiant_fraction()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut model = cube_model();
        model.other_equipment[0].fraction_radiant = 0.25;

        let traces = simulate_zone_internal_convective_gains(&model, 2)?;

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].zone_name, "ZONE ONE");
        assert_eq!(traces[0].values_w, vec![9.0, 9.0]);

        let radiant_traces = simulate_zone_internal_radiant_gains(&model, 2)?;

        assert_eq!(radiant_traces.len(), 1);
        assert_eq!(radiant_traces[0].zone_name, "ZONE ONE");
        assert_eq!(radiant_traces[0].values_w, vec![3.0, 3.0]);
        Ok(())
    }

    #[test]
    fn other_equipment_design_level_methods_drive_internal_gains()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut area_model = cube_model();
        area_model.other_equipment[0].design_level_calculation_method =
            OtherEquipmentDesignLevelCalculationMethod::WattsPerZoneFloorArea;
        area_model.other_equipment[0].design_level_w = 0.0;
        area_model.other_equipment[0].power_per_floor_area_w_per_m2 = 20.0;
        area_model.other_equipment[0].fraction_latent = 0.1;
        area_model.other_equipment[0].fraction_radiant = 0.2;
        area_model.other_equipment[0].fraction_lost = 0.3;

        let area_trace = simulate_zone_internal_convective_gains(&area_model, 1)?;
        let area_radiant_trace = simulate_zone_internal_radiant_gains(&area_model, 1)?;

        assert!((area_trace[0].values_w[0] - 8.0).abs() < 1.0e-12);
        assert!((area_radiant_trace[0].values_w[0] - 4.0).abs() < 1.0e-12);

        let mut people_model = cube_model();
        people_model.other_equipment[0].design_level_calculation_method =
            OtherEquipmentDesignLevelCalculationMethod::WattsPerPerson;
        people_model.other_equipment[0].design_level_w = 0.0;
        people_model.other_equipment[0].power_per_person_w = 15.0;
        people_model.other_equipment[0].fraction_latent = 0.1;
        people_model.people.push(People {
            id: InternalGainId(1),
            name: NormalizedName::new("People"),
            zone: ZoneId(0),
            number_of_people_schedule: None,
            number_of_people_calculation_method: PeopleNumberCalculationMethod::People,
            number_of_people: 3.0,
            people_per_floor_area: 0.0,
            floor_area_per_person: 0.0,
        });

        let people_trace = simulate_zone_internal_convective_gains(&people_model, 1)?;

        assert!((people_trace[0].values_w[0] - 40.5).abs() < 1.0e-12);
        Ok(())
    }

    #[test]
    fn default_time_axis_has_one_day() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis(&TypedModel::default())?;

        assert_eq!(axis.sample_count(), 24);
        assert_eq!(axis.points[0].hour, 1);
        assert_eq!(axis.points[23].hour, 24);

        Ok(())
    }

    #[test]
    fn run_period_time_axis_counts_inclusive_days() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis_for_run_period(&RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Three Days"),
            begin_month: 1,
            begin_day_of_month: 1,
            begin_year: Some(2013),
            end_month: 1,
            end_day_of_month: 3,
            end_year: Some(2013),
            day_of_week_for_start_day: None,
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
            use_weather_file_holidays_and_special_days: true,
            use_weather_file_daylight_saving_period: true,
            apply_weekend_holiday_rule: true,
            use_weather_file_rain_indicators: true,
            use_weather_file_snow_indicators: true,
            treat_weather_as_actual: false,
        })?;

        assert_eq!(axis.sample_count(), 72);
        assert_eq!(axis.points[0].day_of_month, 1);
        assert_eq!(axis.points[71].day_of_month, 3);
        assert_eq!(axis.points[71].hour, 24);

        Ok(())
    }

    #[test]
    fn run_period_time_axis_handles_leap_year() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis_for_run_period(&RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Leap Window"),
            begin_month: 2,
            begin_day_of_month: 28,
            begin_year: Some(2020),
            end_month: 3,
            end_day_of_month: 1,
            end_year: Some(2020),
            day_of_week_for_start_day: None,
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
            use_weather_file_holidays_and_special_days: true,
            use_weather_file_daylight_saving_period: true,
            apply_weekend_holiday_rule: true,
            use_weather_file_rain_indicators: true,
            use_weather_file_snow_indicators: true,
            treat_weather_as_actual: false,
        })?;

        assert_eq!(axis.sample_count(), 72);
        assert_eq!(axis.points[24].day_of_month, 29);
        assert_eq!(axis.points[0].day_of_year, 59);
        assert_eq!(axis.points[24].day_of_year, 60);
        assert_eq!(axis.points[48].day_of_year, 61);
        assert_eq!(axis.points[48].schedule_day_of_year, 61);
        assert!(axis.points[0].gregorian_year_is_leap_year);

        let mut non_leap = test_run_period("Non-Leap Window", 2, 28, 3, 1);
        non_leap.begin_year = Some(2019);
        non_leap.end_year = Some(2019);
        let non_leap_axis = build_hourly_time_axis_for_run_period(&non_leap)?;
        assert_eq!(non_leap_axis.points[24].day_of_year, 60);
        assert_eq!(non_leap_axis.points[24].schedule_day_of_year, 61);
        assert!(!non_leap_axis.points[0].gregorian_year_is_leap_year);

        Ok(())
    }

    fn stage_with_kind(stages: &[ExecutionStage], kind: ExecutionStageKind) -> &ExecutionStage {
        stages
            .iter()
            .find(|stage| stage.kind == kind)
            .expect("execution stage kind should exist")
    }

    #[test]
    fn execution_plan_uses_heat_balance_source_order_stages() {
        let mut typed = TypedModel::default();
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: ep_model::Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            list_multiplier: 1,
            list_group: None,
            ceiling_height: ep_model::AutoOrNumber::AutoCalculate,
            volume: ep_model::AutoOrNumber::AutoCalculate,
            floor_area: ep_model::AutoOrNumber::AutoCalculate,
            inside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
                ep_model::InsideSurfaceConvectionAlgorithm::Tarp,
            ),
            outside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
                ep_model::OutsideSurfaceConvectionAlgorithm::Doe2,
            ),
            is_part_of_total_floor_area: true,
            is_nominal_controlled: false,
            linked_outdoor_air_node: None,
            spaces: Vec::new(),
        });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);

        assert_eq!(plan.stages.len(), 18);
        assert_eq!(plan.step_count(), 16);
        assert!(
            plan.stages
                .iter()
                .all(|stage| stage.kind.is_source_order_barrier())
        );
        assert_eq!(
            plan.stages
                .iter()
                .map(|stage| stage.kind)
                .collect::<Vec<_>>(),
            plan.compatibility_stages
                .iter()
                .map(|stage| stage.kind)
                .collect::<Vec<_>>()
        );
        assert!(plan.source_order_stages_match());
        assert_eq!(
            plan.expected_source_order_stage_ids(),
            plan.actual_source_order_stage_ids()
        );
        assert_eq!(
            plan.runtime_policy.post_typed_model_object_lookup,
            "forbidden-after-rawmodel-typedmodel-runtime-uses-prebound-typed-ids"
        );
        assert_eq!(
            plan.runtime_policy.stage_execution_string_comparison,
            "forbidden-in-source-order-stage-execution"
        );
        assert_eq!(
            plan.runtime_policy.stage_execution_hash_map_lookup,
            "compile-and-report-only-hot-stages-use-vecs-and-typed-ids"
        );
        assert_eq!(
            plan.runtime_policy.compatibility_plan_order,
            "deterministic-energyplus-source-order-then-typed-model-order"
        );

        let init_heat_balance = stage_with_kind(&plan.stages, ExecutionStageKind::InitHeatBalance);
        assert_eq!(init_heat_balance.steps[0], ExecutionStep::UpdateWeather);
        assert_eq!(
            init_heat_balance.steps[1],
            ExecutionStep::EvaluateSchedule(ScheduleId(0))
        );
        assert_eq!(init_heat_balance.prebound.schedule_ids, vec![ScheduleId(0)]);
        assert_eq!(init_heat_balance.prebound.weather_series_indices, vec![0]);
        assert!(
            init_heat_balance
                .dependencies
                .reads
                .contains(&"weather_series")
        );

        let manage_zone_air_updates =
            stage_with_kind(&plan.stages, ExecutionStageKind::ManageZoneAirUpdates);
        assert_eq!(
            manage_zone_air_updates.steps[0],
            ExecutionStep::SolveZone(ZoneId(0))
        );
        assert_eq!(
            manage_zone_air_updates.prebound.zone_ids,
            vec![ZoneId(0)]
        );

        let report_heat_balance =
            stage_with_kind(&plan.stages, ExecutionStageKind::ReportHeatBalance);
        assert_eq!(report_heat_balance.steps.len(), 13);
        assert_eq!(
            report_heat_balance.steps[0],
            ExecutionStep::WriteOutput(OutputHandle(0))
        );
        assert_eq!(
            report_heat_balance.steps[1],
            ExecutionStep::WriteOutput(OutputHandle(1))
        );
        assert_eq!(
            report_heat_balance.steps[2],
            ExecutionStep::WriteOutput(OutputHandle(2))
        );
        assert_eq!(
            report_heat_balance.steps[10],
            ExecutionStep::WriteOutput(OutputHandle(10))
        );
        assert_eq!(report_heat_balance.prebound.output_handles.len(), 13);
        assert_eq!(
            report_heat_balance.prebound.output_handles[0],
            OutputHandle(0)
        );
        assert_eq!(
            plan.compatibility_stages,
            energyplus_heat_balance_compatibility_stages()
        );
    }

    #[test]
    fn runtime_precomputed_data_caches_registry_used_by_plan() {
        let model = SimulationModel::from_typed(cube_model());

        let precomputed = precompute_runtime_data(&model);
        let direct_plan = build_execution_plan(&model);

        assert_eq!(precomputed.execution_plan, direct_plan);
        assert_eq!(precomputed.output_registry.len(), 157);
        let report_heat_balance = stage_with_kind(
            &precomputed.execution_plan.stages,
            ExecutionStageKind::ReportHeatBalance,
        );
        let write_output_count = report_heat_balance
            .steps
            .iter()
            .filter(|step| matches!(step, ExecutionStep::WriteOutput(_)))
            .count();
        assert_eq!(write_output_count, precomputed.output_registry.len());
        assert_eq!(
            report_heat_balance.prebound.output_handles.len(),
            precomputed.output_registry.len()
        );
        let calc_inside_surface = stage_with_kind(
            &precomputed.execution_plan.stages,
            ExecutionStageKind::CalcHeatBalanceInsideSurf,
        );
        assert_eq!(
            calc_inside_surface.prebound.surface_ids.len(),
            model.typed.surfaces.len()
        );
        assert_eq!(
            calc_inside_surface.prebound.construction_ids.len(),
            model.typed.constructions.len()
        );
    }

    #[test]
    fn heat_balance_compatibility_stages_follow_energyplus_source_order() {
        let stages = energyplus_heat_balance_compatibility_stages();

        assert_eq!(stages.len(), 18);
        assert!(
            stages
                .iter()
                .all(|stage| stage.kind.is_source_order_barrier())
        );
        assert_eq!(stages[0].kind, ExecutionStageKind::GetHeatBalanceInput);
        assert_eq!(stages[0].stage_name, "get-heat-balance-input");
        assert_eq!(stages[0].source_routine, "GetHeatBalanceInput");
        assert_eq!(stages[4].kind, ExecutionStageKind::ManageSurfaceHeatBalance);
        assert_eq!(stages[4].source_routine, "ManageSurfaceHeatBalance");
        assert_eq!(stages[5].source_routine, "InitSurfaceHeatBalance");
        assert_eq!(stages[6].source_routine, "CalcHeatBalanceOutsideSurf");
        assert_eq!(stages[7].source_routine, "CalcHeatBalanceInsideSurf");
        assert_eq!(stages[8].source_routine, "ManageAirHeatBalance");
        assert_eq!(stages[9].source_routine, "ManageZoneAirUpdates");
        assert_eq!(stages[11].source_routine, "UpdateThermalHistories");
        assert_eq!(stages[12].source_routine, "ReportSurfaceHeatBalance");
        assert_eq!(stages[15].source_routine, "ReportHeatBalance");
        assert_eq!(stages[17].source_routine, "CheckWarmupConvergence");
    }

    #[test]
    fn heat_balance_source_order_wrappers_pass_through_runtime_blocks() {
        let mut calls = Vec::new();

        let result = crate::heat_balance::manager::manage_heat_balance_source_order_path(|| {
            calls.push("ManageHeatBalance");
            crate::heat_balance::manager::init_heat_balance_source_order_path(|| {
                calls.push("InitHeatBalance");
                41
            })
        });
        let result = crate::heat_balance::surface_manager::manage_surface_heat_balance_source_order_path(|| {
            calls.push("ManageSurfaceHeatBalance");
            crate::heat_balance::surface_manager::init_surface_heat_balance_source_order_path(|| {
                calls.push("InitSurfaceHeatBalance");
                crate::heat_balance::surface_manager::calc_heat_balance_outside_surf_source_order_path(|| {
                    calls.push("CalcHeatBalanceOutsideSurf");
                    result + 1
                })
            })
        });
        crate::heat_balance::surface_manager::calc_heat_balance_inside_surf_source_order_path(|| {
            calls.push("CalcHeatBalanceInsideSurf");
        });
        crate::heat_balance::air_manager::manage_air_heat_balance_source_order_path(|| {
            calls.push("ManageAirHeatBalance");
        });
        crate::heat_balance::zone_predictor_corrector::manage_zone_air_updates_source_order_path(|| {
            calls.push("ManageZoneAirUpdates");
            crate::heat_balance::zone_predictor_corrector::push_zone_timestep_histories_source_order_path(|| {
                calls.push("PushZoneTimestepHistories");
            });
            crate::heat_balance::zone_predictor_corrector::predict_step_source_order_path(|| {
                calls.push("PredictStep");
            });
            crate::heat_balance::zone_predictor_corrector::correct_step_source_order_path(|| {
                calls.push("CorrectStep");
            });
            crate::heat_balance::zone_predictor_corrector::revert_zone_timestep_histories_source_order_path(|| {
                calls.push("RevertZoneTimestepHistories");
            });
            crate::heat_balance::zone_predictor_corrector::push_system_timestep_histories_source_order_path(|| {
                calls.push("PushSystemTimestepHistories");
            });
        });
        crate::heat_balance::surface_manager::update_final_surface_heat_balance_source_order_path(|| {
            calls.push("UpdateFinalSurfaceHeatBalance");
        });
        crate::heat_balance::surface_manager::update_thermal_histories_source_order_path(|| {
            calls.push("UpdateThermalHistories");
        });
        crate::heat_balance::surface_manager::report_surface_heat_balance_source_order_path(|| {
            calls.push("ReportSurfaceHeatBalance");
        });

        assert_eq!(result, 42);
        assert_eq!(
            calls,
            vec![
                "ManageHeatBalance",
                "InitHeatBalance",
                "ManageSurfaceHeatBalance",
                "InitSurfaceHeatBalance",
                "CalcHeatBalanceOutsideSurf",
                "CalcHeatBalanceInsideSurf",
                "ManageAirHeatBalance",
                "ManageZoneAirUpdates",
                "PushZoneTimestepHistories",
                "PredictStep",
                "CorrectStep",
                "RevertZoneTimestepHistories",
                "PushSystemTimestepHistories",
                "UpdateFinalSurfaceHeatBalance",
                "UpdateThermalHistories",
                "ReportSurfaceHeatBalance",
            ]
        );
    }

    #[test]
    fn heat_balance_zone_air_algorithm_lanes_separate_compatibility_and_diagnostics() {
        let candidate = HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate;
        assert_eq!(
            candidate.lane(),
            HeatBalanceAlgorithmLane::CompatibilitySourceOrder
        );
        assert_eq!(candidate.lane().id(), "compatibility-source-order");
        assert!(candidate.is_compatibility_source_order());
        assert!(candidate.allows_conformance_promotion());

        let diagnostic_only = HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical;
        assert_eq!(
            diagnostic_only.lane(),
            HeatBalanceAlgorithmLane::DiagnosticOnly
        );
        assert_eq!(diagnostic_only.lane().id(), "diagnostic-only");
        assert!(diagnostic_only.is_diagnostic_lane());
        assert!(!diagnostic_only.allows_conformance_promotion());

        let probe = HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe;
        assert_eq!(probe.lane(), HeatBalanceAlgorithmLane::DiagnosticProbe);
        assert_eq!(probe.lane().id(), "diagnostic-probe");
        assert!(probe.is_diagnostic_lane());
        assert!(!probe.allows_conformance_promotion());
    }
