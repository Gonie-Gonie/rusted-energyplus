    use super::{
        ConstructionCtfCoefficientOverride, CtfInsideFaceBalanceInput, CtfOutsideFaceBalanceInput,
        CtfOutsideQuickConductionBalanceInput,
        ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C,
        ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M,
        ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K, ENERGYPLUS_ZONE_INITIAL_TEMP_C, EpwRecord,
        FirstZoneSimulationOptions, HeatBalanceCtfInitialHistoryPolicy,
        HeatBalanceSimulationOptions, HeatBalanceStepInput,
        HeatBalanceSurfaceLoopZoneAirCorrection, HeatBalanceWarmupOptions,
        HeatBalanceWarmupSummary, HeatBalanceWeatherContext, HeatBalanceZoneAirReportSampling,
        HeatBalanceZoneConductionReportSource, InteriorLongwaveExchangeProbe,
        InteriorLongwaveSurfaceSnapshot, KELVIN_OFFSET, OutputSeries,
        QuickOutsideConductionContext, ResultStore, RuntimeError, SECONDS_PER_HOUR,
        STEFAN_BOLTZMANN_W_PER_M2_K4, SimulationMode, SimulationState,
        SurfaceBoundaryBalanceResult, SurfaceCtfState, SurfaceExteriorReportTerms,
        SurfaceOutsideBalanceDiagnostics, advance_heat_balance_state_one_timestep,
        advance_heat_balance_state_one_timestep_internal, advance_surface_ctf_histories,
        advance_surface_ctf_histories_with_outside_temperature_override,
        append_surface_incident_solar_radiation_series,
        apply_energyplus_adaptive_system_timestep_zone_air_correction,
        energyplus_analytical_zone_air_temperature_c, energyplus_anisotropic_sky_multiplier,
        energyplus_approximate_view_factors, energyplus_ashrae_tarp_natural_convection_w_per_m2_k,
        energyplus_average_solar_coefficients, energyplus_ctf_inside_face_temperature_c,
        energyplus_ctf_outside_face_temperature_c,
        energyplus_ctf_outside_face_temperature_quick_conduction_c,
        energyplus_daily_solar_coefficients,
        energyplus_doe2_outside_convection_coefficient_w_per_m2_k,
        energyplus_exterior_longwave_terms, energyplus_exterior_wet_context_fraction,
        energyplus_exterior_wet_timestep_fraction,
        energyplus_linearized_radiation_coefficient_w_per_m2_k,
        energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
        energyplus_outdoor_wet_bulb_c, energyplus_scriptf_from_view_factors,
        energyplus_shadowing_period_solar_coefficients,
        energyplus_surface_outdoor_air_temperature_c,
        energyplus_surface_outside_wind_speed_m_per_s,
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
        heat_balance_uses_balance_surface_convection_report,
        heat_balance_uses_doe2_outside_convection,
        heat_balance_uses_surface_reference_air_convection_report,
        heat_balance_uses_surface_reference_air_surface_convection_report,
        horizontal_infrared_sky_temperature_c, initialize_heat_balance_state,
        initialize_heat_balance_state_with_ctf_coefficients,
        inside_ctf_outside_temperature_history_commit_override_c, parse_epw_dry_bulb_series,
        parse_epw_records, run_heat_balance_run_period_warmup, run_surface_balance_passes,
        seed_energyplus_initial_surface_ctf_histories, seed_initial_surface_ctf_boundary_histories,
        simulate_constant_schedules, simulate_first_zone_uncontrolled,
        simulate_heat_balance_zone_air_temperatures,
        simulate_heat_balance_zone_air_temperatures_internal,
        simulate_heat_balance_zone_air_temperatures_with_weather_records, simulate_schedule_values,
        simulate_zone_internal_convective_gains, solar_position_rad_at_local_hour,
        solar_weather_interpolation_weights, surface_air_sky_radiation_split, surface_area_m2,
        surface_azimuth_deg, surface_ctf_history_slot_samples, surface_exterior_report_terms,
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
        zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w,
        zone_air_heat_balance_surface_convection_rate_w,
        zone_air_system_timestep_storage_report_rate_w, zone_geometry_summaries,
        zone_surface_report_conduction_rates_w,
    };
    use crate::heat_balance::{HeatBalanceAlgorithmLane, HeatBalanceZoneAirAlgorithm};
    use crate::node::{
        NODE_STATE_EXCLUDED_SETPOINT_VARIABLE, NODE_STATE_SOURCE_MAP_PATH,
        NODE_TEMPERATURE_SETPOINT_SENTINEL_C, NodeStateProjectionOptions, NodeStateRole,
        NodeStateStore, node_temperature_setpoint_from_energyplus,
        simulate_ideal_loads_node_state_projection,
    };
    use crate::time_axis::{Date, next_day};
    use crate::{
        ExecutionStage, ExecutionStageKind, ExecutionStep, RuntimeOutputRegistry,
        build_execution_plan, build_hourly_time_axis, build_hourly_time_axis_for_run_period,
        energyplus_heat_balance_compatibility_stages,
    };
    use crate::{
        RuntimeDiagnosticCode, RuntimeMeterRequest, RuntimeOutputFrequency, RuntimeOutputRequest,
    };
    use ep_model::{
        AutoOrNumber, AutosizeOrNumber, Construction, ConstructionId, DehumidificationControlType,
        DemandControlledVentilationType, FirstHourInterpolationStartingValues, HeatRecoveryType,
        HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
        IdealLoadsLimit, InternalGainId, LoadDistributionScheme, Material, MaterialId,
        MaterialKind, MaterialSurfaceRoughness, Node, NodeId, NodeList, NodeListId, NormalizedName,
        OtherEquipment, OutdoorAirEconomizerType, OutputHandle, OutsideBoundaryCondition,
        OutsideSurfaceConvectionAlgorithm, Point3, RunPeriod, RunPeriodId, ScheduleCompact,
        ScheduleCompactSegment, ScheduleConstant, ScheduleId, SimulationModel, SiteLocation,
        SunExposure, Surface, SurfaceId, SurfaceType, Terrain, ThermostatControlObjectType,
        ThermostatDualSetpoint, ThermostatSetpointId, TimestepConfig, TypedModel, WindExposure,
        Zone, ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
        ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId,
        ZoneThermostat, ZoneThermostatControl, ZoneThermostatId,
    };
    use std::collections::BTreeMap;

    #[test]
    fn state_defaults_to_first_timestep() {
        let state = SimulationState::new(SimulationMode::Compatibility);

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.mode, SimulationMode::Compatibility);
        assert!(state.zones.is_empty());
    }

    #[test]
    fn solar_weather_interpolation_matches_energyplus_even_timestep_weights() {
        assert_eq!(solar_weather_interpolation_weights(4, 1), (0.25, 0.75, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 2), (0.0, 1.0, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 3), (0.0, 0.75, 0.25));
        assert_eq!(solar_weather_interpolation_weights(4, 4), (0.0, 0.5, 0.5));
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
    }

    #[test]
    fn compact_schedule_trace_uses_until_segments() {
        let mut model = TypedModel::default();
        model.compact_schedules.push(ScheduleCompact {
            id: ScheduleId(0),
            name: NormalizedName::new("Office Occupancy"),
            schedule_type_limits: None,
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
        });

        let traces = simulate_schedule_values(&model, 24);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].values[7], 0.0);
        assert_eq!(traces[0].values[8], 1.0);
        assert_eq!(traces[0].values[17], 1.0);
        assert_eq!(traces[0].values[18], 0.0);
    }

    #[test]
    fn zone_internal_convective_gain_trace_excludes_radiant_fraction() {
        let mut model = cube_model();
        model.other_equipment[0].fraction_radiant = 0.25;

        let traces = simulate_zone_internal_convective_gains(&model, 2);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].zone_name, "ZONE ONE");
        assert_eq!(traces[0].values_w, vec![9.0, 9.0]);
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
        })?;

        assert_eq!(axis.sample_count(), 72);
        assert_eq!(axis.points[24].day_of_month, 29);

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
            ceiling_height: ep_model::AutoOrNumber::AutoCalculate,
            volume: ep_model::AutoOrNumber::AutoCalculate,
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

        let init_heat_balance = stage_with_kind(&plan.stages, ExecutionStageKind::InitHeatBalance);
        assert_eq!(init_heat_balance.steps[0], ExecutionStep::UpdateWeather);
        assert_eq!(
            init_heat_balance.steps[1],
            ExecutionStep::EvaluateSchedule(ScheduleId(0))
        );

        let manage_zone_air_updates =
            stage_with_kind(&plan.stages, ExecutionStageKind::ManageZoneAirUpdates);
        assert_eq!(
            manage_zone_air_updates.steps[0],
            ExecutionStep::SolveZone(ZoneId(0))
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
        assert_eq!(
            plan.compatibility_stages,
            energyplus_heat_balance_compatibility_stages()
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
