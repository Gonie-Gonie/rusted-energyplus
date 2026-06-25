    #[test]
    fn energyplus_outdoor_wet_bulb_uses_psychrometric_formula() {
        let wet_bulb_c = energyplus_outdoor_wet_bulb_c(3.0, 68.0, 82_800.0)
            .expect("valid psychrometric wet-bulb");

        assert!(
            (wet_bulb_c - 0.648_294_941_184).abs() < 1.0e-7,
            "wet_bulb_c={wet_bulb_c}"
        );
    }

    #[test]
    fn energyplus_outdoor_wet_bulb_uses_energyplus_iterate_branch_near_freezing() {
        let wet_bulb_c = energyplus_outdoor_wet_bulb_c(8.0, 20.0, 81_500.0)
            .expect("valid psychrometric wet-bulb");

        assert!(
            (wet_bulb_c - 0.227_141_685_581).abs() < 2.0e-9,
            "wet_bulb_c={wet_bulb_c}"
        );
    }

    #[test]
    fn exterior_report_terms_use_energyplus_wet_surface_rain_override()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_state = state
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof test surface"))?;
        surface_state.outside_face_temperature_c = 10.0;

        let records = [weather_record_with_precipitation(1.0)];
        let reference_temperature_c = energyplus_outdoor_wet_bulb_c(
            records[0].dry_bulb_c,
            records[0].relative_humidity_percent,
            records[0].atmospheric_pressure_pa,
        )
        .unwrap_or(8.0);
        let typed_roof = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing typed roof test surface"))?;
        let expected_reference_temperature_c =
            energyplus_surface_outdoor_air_temperature_c(typed_roof, reference_temperature_c);

        let terms = surface_exterior_report_terms(
            &typed,
            surface_state,
            8.0,
            10.0,
            Some(HeatBalanceWeatherContext {
                records: &records,
                record_index: 0,
                zone_steps_per_hour: 4,
                zone_timestep: None,
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
            }),
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
        );

        assert_eq!(
            terms.convection_coefficient_w_per_m2_k,
            ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K
        );
        assert!(
            expected_reference_temperature_c < 8.0,
            "rain path should use wet-bulb reference below dry-bulb"
        );
        assert!(
            (terms.convection_heat_gain_rate_per_area_w_per_m2
                - -ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K
                    * (10.0 - expected_reference_temperature_c))
                .abs()
                < 1.0e-9
        );

        Ok(())
    }

    #[test]
    fn exterior_longwave_terms_use_energyplus_sky_air_ground_split()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_state = state
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof test surface"))?;
        surface_state.outside_face_temperature_c = 60.0;
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing typed roof test surface"))?;
        let record = EpwRecord {
            dry_bulb_c: 24.0,
            horizontal_infrared_radiation_wh_per_m2: 358.0,
            wind_speed_m_per_s: 4.6,
            wind_direction_deg: 310.0,
            ..weather_record_with_precipitation(0.0)
        };
        let tilt_rad =
            surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices).to_radians();

        let terms = energyplus_exterior_longwave_terms(
            surface_state,
            typed_surface,
            record.horizontal_infrared_radiation_wh_per_m2,
            60.0,
            24.0,
            24.0,
            tilt_rad,
        );
        let expected_sky_temperature_c = horizontal_infrared_sky_temperature_c(
            record.horizontal_infrared_radiation_wh_per_m2,
            24.0,
        );
        let expected_sky_coefficient = energyplus_linearized_radiation_coefficient_w_per_m2_k(
            0.9,
            60.0 + KELVIN_OFFSET,
            expected_sky_temperature_c + KELVIN_OFFSET,
        );
        let expected_gain = -expected_sky_coefficient * (60.0 - expected_sky_temperature_c);

        assert!((terms.sky_coefficient_w_per_m2_k - expected_sky_coefficient).abs() < 1.0e-12);
        assert!(terms.air_coefficient_w_per_m2_k.abs() < 1.0e-12);
        assert!(terms.ground_coefficient_w_per_m2_k.abs() < 1.0e-12);
        assert!((terms.net_heat_gain_per_area_w_per_m2(60.0) - expected_gain).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn exterior_longwave_air_component_uses_air_reference_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_state = state
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall test surface"))?;
        surface_state.outside_face_temperature_c = 30.0;
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing typed wall test surface"))?;
        let tilt_rad =
            surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices).to_radians();

        let terms = energyplus_exterior_longwave_terms(
            surface_state,
            typed_surface,
            360.0,
            30.0,
            10.0,
            20.0,
            tilt_rad,
        );
        let air_split = surface_air_sky_radiation_split(tilt_rad);
        let expected_air_coefficient = energyplus_linearized_radiation_coefficient_w_per_m2_k(
            surface_state.thermal_absorptance
                * surface_sky_view_factor(typed_surface, tilt_rad)
                * (1.0 - air_split),
            30.0 + KELVIN_OFFSET,
            10.0 + KELVIN_OFFSET,
        );

        assert!((terms.air_coefficient_w_per_m2_k - expected_air_coefficient).abs() < 1.0e-12);
        assert_eq!(terms.air_temperature_c, 10.0);
        assert_eq!(terms.ground_temperature_c, 20.0);

        Ok(())
    }

    #[test]
    fn heat_balance_warmup_minimum_override_preserves_disabled_boundary() {
        let disabled = HeatBalanceSimulationOptions::hourly_samples(3).with_warmup_minimum_days(20);
        assert!(!disabled.warmup.enabled);
        assert_eq!(disabled.warmup.minimum_days, 0);

        let mut enabled = HeatBalanceSimulationOptions::hourly_samples(3);
        enabled.warmup = HeatBalanceWarmupOptions {
            enabled: true,
            minimum_days: 6,
            maximum_days: 10,
            temperature_convergence_tolerance_delta_c: 0.1,
        };
        let overridden = enabled.with_warmup_minimum_days(20);
        assert_eq!(overridden.warmup.minimum_days, 20);
        assert_eq!(overridden.warmup.maximum_days, 20);
    }

    #[test]
    fn heat_balance_warmup_uses_weather_context_for_exterior_forcing()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.timestep = TimestepConfig {
            number_of_timesteps_per_hour: 1,
        };
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed.clone());
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_dry_bulb_c = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let options = HeatBalanceWarmupOptions {
            enabled: true,
            minimum_days: 1,
            maximum_days: 1,
            temperature_convergence_tolerance_delta_c: 0.0,
        };
        let mut dry_only_state = initialize_heat_balance_state(&model, 20.0)?;
        let mut weather_context_state = initialize_heat_balance_state(&model, 20.0)?;
        let mut dry_only_warmup_day_end_states = Vec::new();
        let mut weather_context_warmup_day_end_states = Vec::new();

        let dry_only_summary = run_heat_balance_run_period_warmup(
            &typed,
            &mut dry_only_state,
            &weather_dry_bulb_c,
            None,
            1,
            SECONDS_PER_HOUR,
            options,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            1,
            None,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
            FirstHourInterpolationStartingValues::Hour24,
            &mut dry_only_warmup_day_end_states,
            advance_heat_balance_state_one_timestep_internal,
        );
        let weather_context_summary = run_heat_balance_run_period_warmup(
            &typed,
            &mut weather_context_state,
            &weather_dry_bulb_c,
            Some(&records),
            1,
            SECONDS_PER_HOUR,
            options,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            1,
            None,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
            FirstHourInterpolationStartingValues::Hour24,
            &mut weather_context_warmup_day_end_states,
            advance_heat_balance_state_one_timestep_internal,
        );

        assert_eq!(dry_only_summary.day_count, 1);
        assert_eq!(weather_context_summary.day_count, 1);
        assert_eq!(dry_only_warmup_day_end_states.len(), 1);
        assert_eq!(weather_context_warmup_day_end_states.len(), 1);
        let dry_only_roof = dry_only_state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing dry-only roof"))?;
        let weather_context_roof = weather_context_state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing weather-context roof"))?;

        assert!(
            weather_context_roof.outside_face_temperature_c
                > dry_only_roof.outside_face_temperature_c + 1.0
        );

        Ok(())
    }

    #[test]
    fn heat_balance_third_order_probe_runs_as_diagnostic_option()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe),
        )?;

        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.timestep_count, 12);
        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert_eq!(zone_series.values.len(), 2);
        assert!(zone_series.values.iter().all(|value| value.is_finite()));
        assert_eq!(
            simulation.summary.warmup,
            HeatBalanceWarmupSummary::disabled()
        );

        Ok(())
    }

    #[test]
    fn heat_balance_surface_first_probe_uses_distinct_zone_air_order()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let analytical = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe),
        )?;
        let surface_first = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2).with_zone_air_algorithm(
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe,
            ),
        )?;
        let coupled = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2).with_zone_air_algorithm(
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe,
            ),
        )?;

        let Some(analytical_zone_series) = analytical
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing analytical zone series").into());
        };
        let Some(surface_first_zone_series) = surface_first
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing surface-first zone series").into());
        };
        let Some(coupled_zone_series) = coupled
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing coupled zone series").into());
        };

        assert_eq!(analytical_zone_series.values.len(), 2);
        assert_eq!(surface_first_zone_series.values.len(), 2);
        assert_eq!(coupled_zone_series.values.len(), 2);
        assert!(
            analytical_zone_series
                .values
                .iter()
                .chain(surface_first_zone_series.values.iter())
                .chain(coupled_zone_series.values.iter())
                .all(|value| value.is_finite())
        );
        assert!(
            (analytical_zone_series.values[0] - surface_first_zone_series.values[0]).abs() > 1.0e-6
        );
        assert!(
            (surface_first_zone_series.values[0] - coupled_zone_series.values[0]).abs() > 1.0e-6
        );

        Ok(())
    }

    #[test]
    fn surface_incident_solar_diagnostic_appends_roof_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Solar Test Site"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed);
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_values = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let mut simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;

        let added = append_surface_incident_solar_radiation_series(
            &mut simulation.results,
            &model,
            &records,
            2,
        );

        assert_eq!(added, 20);
        assert!(
            simulation
                .results
                .find_series(
                    "FLOOR",
                    "Surface Outside Face Incident Solar Radiation Rate per Area"
                )
                .is_none()
        );
        let Some(roof_solar) = simulation.results.find_series(
            "ROOF",
            "Surface Outside Face Incident Solar Radiation Rate per Area",
        ) else {
            return Err(std::io::Error::other("missing roof solar series").into());
        };
        assert_eq!(roof_solar.units, "W/m2");
        assert_eq!(roof_solar.values.len(), 2);
        assert!(roof_solar.values[0].is_finite());
        assert!(roof_solar.values[0] > 600.0);
        for variable in [
            "Surface Outside Face Incident Beam Solar Radiation Rate per Area",
            "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
            "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area",
        ] {
            let Some(series) = simulation.results.find_series("ROOF", variable) else {
                return Err(
                    std::io::Error::other(format!("missing roof {variable} series")).into(),
                );
            };
            assert_eq!(series.units, "W/m2");
            assert_eq!(series.values.len(), 2);
            assert!(series.values[0].is_finite());
        }

        Ok(())
    }

    #[test]
    fn weather_record_exterior_balance_forces_exterior_conduction()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Solar Test Site"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed);
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_values = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let dry_bulb_only = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;
        let weather_forced = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;
        let coupled = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_inside = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_inside_doe2 = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(dry_roof_conduction) = dry_bulb_only
            .results
            .find_series("ROOF", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing dry roof conduction series").into());
        };
        let Some(forced_roof_conduction) = weather_forced
            .results
            .find_series("ROOF", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing forced roof conduction series").into());
        };
        let Some(dry_wall_conduction) = dry_bulb_only.results.find_series(
            "WALL Y0",
            "Surface Inside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing dry wall conduction series").into());
        };
        let Some(forced_wall_conduction) = weather_forced.results.find_series(
            "WALL Y0",
            "Surface Inside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing forced wall conduction series").into());
        };
        let Some(coupled_roof_temperature) = coupled
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled roof temperature series").into());
        };
        let Some(previous_inside_roof_temperature) = previous_inside
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(
                std::io::Error::other("missing previous-inside roof temperature series").into(),
            );
        };
        let Some(previous_inside_doe2_roof_temperature) = previous_inside_doe2
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other(
                "missing previous-inside DOE-2 roof temperature series",
            )
            .into());
        };

        assert_eq!(dry_roof_conduction.values.len(), 2);
        assert_eq!(forced_roof_conduction.values.len(), 2);
        assert_eq!(dry_wall_conduction.values.len(), 2);
        assert_eq!(forced_wall_conduction.values.len(), 2);
        assert_eq!(coupled_roof_temperature.values.len(), 2);
        assert_eq!(previous_inside_roof_temperature.values.len(), 2);
        assert_eq!(previous_inside_doe2_roof_temperature.values.len(), 2);
        assert!((dry_roof_conduction.values[0] - forced_roof_conduction.values[0]).abs() > 1.0e-3);
        assert!((dry_wall_conduction.values[0] - forced_wall_conduction.values[0]).abs() > 1.0e-3);
        assert!(
            (coupled_roof_temperature.values[0] - previous_inside_roof_temperature.values[0]).abs()
                > 1.0e-6
        );
        assert!(
            (previous_inside_doe2_roof_temperature.values[0]
                - previous_inside_roof_temperature.values[0])
                .abs()
                > 1.0e-6
        );

        Ok(())
    }
