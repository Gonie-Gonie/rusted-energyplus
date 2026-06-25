    #[test]
    fn zone_air_heat_balance_surface_convection_can_use_report_air_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone_id = {
            let zone = &mut state.zones[0];
            zone.mean_air_temperature_c = 21.0;
            zone.previous_mean_air_temperatures_c = [20.0, 19.0, 18.0];
            zone.sum_ha_w_per_k = 10.0;
            zone.sum_hat_surf_w = 250.0;
            zone.sum_hat_ref_w = 5.0;

            assert!((zone_air_heat_balance_surface_convection_rate_w(zone) - 35.0).abs() < 1.0e-12);
            assert!(
                (zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
                    zone,
                    zone.previous_mean_air_temperatures_c[0]
                ) - 45.0)
                    .abs()
                    < 1.0e-12
            );
            zone.convective_internal_gain_w = 7.0;
            assert!(
                (zone_air_heat_balance_surface_convection_rate_from_balance_w(zone, 45.0) - 38.0)
                    .abs()
                    < 1.0e-12
            );
            zone.zone_id
        };

        state.surfaces[0].inside_convection_coefficient_w_per_m2_k = 2.0;
        state.surfaces[0].area_m2 = 3.0;
        state.surfaces[0].inside_face_temperature_c = 22.0;
        state.surfaces[0].inside_reference_air_temperature_c = 20.0;
        assert!(
            (zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w(
                &state.surfaces,
                zone_id
            ) - 12.0)
                .abs()
                < 1.0e-12
        );
        assert!(
            (surface_inside_convection_heat_gain_rate_per_area_w_per_m2(
                &state.surfaces[0],
                &state.zones,
                true,
                false,
            ) + 4.0)
                .abs()
                < 1.0e-12
        );
        let scriptf_flat_probe =
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe;
        let converged_surface_probe =
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe;
        assert!(heat_balance_uses_balance_surface_convection_report(
            converged_surface_probe
        ));
        assert!(!heat_balance_uses_balance_surface_convection_report(
            scriptf_flat_probe
        ));
        assert!(
            heat_balance_uses_surface_reference_air_surface_convection_report(scriptf_flat_probe)
        );
        assert!(!heat_balance_uses_surface_reference_air_convection_report(
            scriptf_flat_probe
        ));
        let final_coefficient = surface_inside_convection_report_coefficient_w_per_m2_k(
            &state.surfaces[0],
            &state.zones,
            false,
            true,
        );
        assert!(
            (final_coefficient
                - energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
                    &state.surfaces[0],
                    state.surfaces[0].inside_face_temperature_c,
                    state.zones[0].mean_air_temperature_c,
                ))
            .abs()
                < 1.0e-12
        );

        Ok(())
    }

    #[test]
    fn heat_balance_timestep_uses_previous_surface_temperature_for_ctf_damping()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        state.surfaces[0].inside_face_temperature_c = 40.0;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 20.0,
                hour_ending: 1,
                timestep_seconds: 60.0,
            },
        );

        assert!(
            state.surfaces[0].inside_face_temperature_c > 25.0,
            "CTF damping should use the previous surface temperature, not the overwritten zone temperature"
        );

        Ok(())
    }

    #[test]
    fn heat_balance_adiabatic_surfaces_do_not_create_artificial_losses()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        for surface in &mut typed.surfaces {
            surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
            surface.outside_boundary_condition_object = None;
        }
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: -10.0,
                hour_ending: 1,
                timestep_seconds: 600.0,
            },
        );

        assert!(state.zones[0].mean_air_temperature_c > 20.0);
        assert!((state.zones[0].opaque_surface_heat_gain_w).abs() < 1.0e-9);
        assert!((state.zones[0].opaque_surface_outside_conduction_w).abs() < 1.0e-9);
        for surface in &state.surfaces {
            assert_eq!(
                surface.outside_boundary_condition,
                OutsideBoundaryCondition::Adiabatic
            );
            assert_eq!(
                surface.outside_face_temperature_c,
                surface.inside_face_temperature_c
            );
            assert!(surface.heat_gain_to_zone_w.abs() < 1.0e-9);
        }

        Ok(())
    }

    #[test]
    fn heat_balance_interzone_surface_uses_adjacent_zone_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = two_zone_interzone_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        state.zones[0].mean_air_temperature_c = 20.0;
        state.zones[1].mean_air_temperature_c = 10.0;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 0.0,
                hour_ending: 1,
                timestep_seconds: 60.0,
            },
        );

        let warm_zone = state
            .zones
            .iter()
            .find(|zone| zone.zone_name == "ZONE A")
            .ok_or_else(|| std::io::Error::other("missing warm zone"))?;
        let cool_zone = state
            .zones
            .iter()
            .find(|zone| zone.zone_name == "ZONE B")
            .ok_or_else(|| std::io::Error::other("missing cool zone"))?;
        assert!(warm_zone.mean_air_temperature_c < 20.0);
        assert!(cool_zone.mean_air_temperature_c > 10.0);

        let warm_surface = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "A WALL")
            .ok_or_else(|| std::io::Error::other("missing A WALL"))?;
        assert_eq!(
            warm_surface.outside_boundary_target_surface_id,
            Some(SurfaceId(1))
        );
        assert_eq!(
            warm_surface.outside_boundary_target_zone_id,
            Some(ZoneId(1))
        );
        assert_eq!(
            warm_surface.outside_face_temperature_c,
            cool_zone.mean_air_temperature_c
        );
        assert!(warm_surface.heat_gain_to_zone_w < 0.0);

        let cool_surface = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "B WALL")
            .ok_or_else(|| std::io::Error::other("missing B WALL"))?;
        assert_eq!(
            cool_surface.outside_face_temperature_c,
            warm_zone.mean_air_temperature_c
        );
        assert!(cool_surface.heat_gain_to_zone_w > 0.0);

        Ok(())
    }

    #[test]
    fn heat_balance_missing_interzone_surface_target_fails() {
        let mut typed = two_zone_interzone_model();
        typed.surfaces[0].outside_boundary_condition_object =
            Some(NormalizedName::new("Missing Surface"));
        let model = SimulationModel::from_typed(typed);

        assert!(matches!(
            initialize_heat_balance_state(&model, 20.0),
            Err(RuntimeError::MissingSurfaceBoundaryTarget { .. })
        ));
    }

    #[test]
    fn heat_balance_trace_writes_zone_air_temperature_results()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());

        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;

        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.timestep_count, 12);
        assert_eq!(simulation.summary.zone_count, 1);
        assert_eq!(simulation.summary.surface_count, 6);
        assert_eq!(simulation.state.timestep_index, 12);
        assert_eq!(simulation.results.sample_count(), 2);
        assert_eq!(simulation.results.series.len(), 329);
        assert_eq!(
            simulation.summary.run_period_initial_zone_air_states.len(),
            1
        );

        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert!(zone_series.values[0] > 11.9);
        assert!(zone_series.values[0] < 20.0);
        assert!(zone_series.values[1] > zone_series.values[0]);

        let Some(zone_humidity_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Humidity Ratio")
        else {
            return Err(std::io::Error::other("missing zone humidity series").into());
        };
        assert_eq!(zone_humidity_series.values.len(), 2);

        let Some(sky_temperature_series) = simulation
            .results
            .find_series("Environment", "Site Sky Temperature")
        else {
            return Err(std::io::Error::other("missing sky temperature series").into());
        };
        assert_eq!(sky_temperature_series.values.len(), 2);

        let Some(horizontal_infrared_series) = simulation.results.find_series(
            "Environment",
            "Site Horizontal Infrared Radiation Rate per Area",
        ) else {
            return Err(std::io::Error::other("missing horizontal infrared series").into());
        };
        assert_eq!(horizontal_infrared_series.values.len(), 2);

        let Some(zone_air_capacity_series) = simulation
            .results
            .find_series("ZONE ONE", super::RUST_ZONE_AIR_HEAT_CAPACITY_VARIABLE)
        else {
            return Err(std::io::Error::other("missing zone-air debug series").into());
        };
        assert_eq!(zone_air_capacity_series.values.len(), 2);

        let Some(inside_convection_series) = simulation.results.find_series(
            "FLOOR",
            "Surface Inside Face Convection Heat Transfer Coefficient",
        ) else {
            return Err(std::io::Error::other("missing inside convection series").into());
        };
        assert_eq!(inside_convection_series.values.len(), 2);
        let Some(adjacent_air_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Adjacent Air Temperature")
        else {
            return Err(std::io::Error::other("missing adjacent air series").into());
        };
        assert_eq!(adjacent_air_series.values.len(), 2);
        let Some(iteration_count_series) = simulation.results.find_series(
            "Simulation",
            super::SURFACE_INSIDE_HEAT_BALANCE_ITERATION_COUNT_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing inside surface iteration count").into());
        };
        assert_eq!(iteration_count_series.values, vec![6.0, 6.0]);
        let Some(outside_balance_series) = simulation.results.find_series(
            "ROOF",
            super::SURFACE_OUTSIDE_BALANCE_REPORT_TEMPERATURE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing outside balance report temperature").into());
        };
        assert_eq!(outside_balance_series.values.len(), 2);

        let Some(weather_series) = simulation
            .results
            .find_series("Environment", "Site Outdoor Air Drybulb Temperature")
        else {
            return Err(std::io::Error::other("missing weather series").into());
        };
        assert_eq!(weather_series.values, vec![10.0, 12.0]);

        let Some(inside_surface_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing inside surface series").into());
        };
        assert_eq!(inside_surface_series.values.len(), 2);
        assert!(inside_surface_series.values[0].is_finite());
        assert_ne!(inside_surface_series.values[0], zone_series.values[0]);

        let Some(outside_surface_series) = simulation
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing outside surface series").into());
        };
        assert_eq!(outside_surface_series.values, vec![10.0, 12.0]);

        let Some(inside_conduction_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing inside conduction series").into());
        };
        assert_eq!(inside_conduction_series.values.len(), 2);
        assert!(inside_conduction_series.values[0] < 0.0);

        let Some(outside_conduction_series) = simulation.results.find_series(
            "FLOOR",
            "Surface Outside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing outside conduction series").into());
        };
        assert_eq!(
            outside_conduction_series.values[0],
            -inside_conduction_series.values[0]
        );
        let Some(inside_current_outside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside outside term").into());
        };
        let Some(inside_current_inside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside inside term").into());
        };
        let Some(inside_history_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside history term").into());
        };
        let Some(inside_history_temperature_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_HISTORY_TEMPERATURE_TERM_RATE_VARIABLE,
        ) else {
            return Err(
                std::io::Error::other("missing CTF inside history temperature term").into(),
            );
        };
        let Some(inside_history_flux_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_HISTORY_FLUX_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside history flux term").into());
        };
        let Some(outside_current_outside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF outside outside term").into());
        };
        let Some(outside_current_inside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF outside inside term").into());
        };
        let Some(outside_history_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF outside history term").into());
        };
        assert!(
            (inside_conduction_series.values[0]
                - inside_current_outside_term.values[0]
                - inside_current_inside_term.values[0]
                - inside_history_term.values[0])
                .abs()
                < 1.0e-9
        );
        assert!(
            (inside_history_term.values[0]
                - inside_history_temperature_term.values[0]
                - inside_history_flux_term.values[0])
                .abs()
                < 1.0e-9
        );
        assert!(
            (outside_conduction_series.values[0]
                - outside_current_outside_term.values[0]
                - outside_current_inside_term.values[0]
                - outside_history_term.values[0])
                .abs()
                < 1.0e-9
        );
        let Some(storage_series) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate")
        else {
            return Err(std::io::Error::other("missing surface heat storage series").into());
        };
        assert_eq!(storage_series.values.len(), 2);
        assert!(
            (storage_series.values[0]
                + inside_conduction_series.values[0]
                + outside_conduction_series.values[0])
                .abs()
                < 1.0e-9
        );
        let Some(storage_per_area_series) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate per Area")
        else {
            return Err(
                std::io::Error::other("missing surface heat storage per-area series").into(),
            );
        };
        assert_eq!(storage_per_area_series.values.len(), 2);
        assert!(
            (storage_per_area_series.values[0] - storage_series.values[0] / 100.0).abs() < 1.0e-9
        );

        let Some(zone_conduction_series) = simulation.results.find_series(
            "ZONE ONE",
            "Zone Opaque Surface Inside Faces Conduction Rate",
        ) else {
            return Err(std::io::Error::other("missing zone conduction series").into());
        };
        assert!(zone_conduction_series.values[0] < 0.0);

        let Some(zone_outside_conduction_series) = simulation.results.find_series(
            "ZONE ONE",
            "Zone Opaque Surface Outside Faces Conduction Rate",
        ) else {
            return Err(std::io::Error::other("missing zone outside conduction series").into());
        };
        assert_eq!(zone_outside_conduction_series.values.len(), 2);
        assert!(zone_outside_conduction_series.values[0].is_finite());

        let Some(surface_convection_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate")
        else {
            return Err(std::io::Error::other("missing zone air surface convection series").into());
        };
        assert_eq!(surface_convection_series.values.len(), 2);
        assert!(surface_convection_series.values[0].is_finite());

        Ok(())
    }

    #[test]
    fn compat_candidate_report_flags_follow_execution_variant() {
        let report_algorithm = super::heat_balance_zone_air_algorithm_execution_variant(
            HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate,
        );

        assert_eq!(
            report_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        );
        assert!(
            super::heat_balance_uses_surface_reference_air_surface_convection_report(
                report_algorithm
            )
        );
        assert!(
            !super::heat_balance_uses_surface_reference_air_surface_convection_report(
                HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
            )
        );
    }

    #[test]
    fn heat_balance_zone_air_rate_outputs_follow_report_sampling()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let options = HeatBalanceSimulationOptions::hourly_samples(1);
        let simulation = simulate_heat_balance_zone_air_temperatures(&model, &[10.0], options)?;
        assert_eq!(
            simulation.summary.zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::Average
        );
        let last_state_options = options
            .with_zone_air_report_sampling(HeatBalanceZoneAirReportSampling::LastSystemState);
        let last_state_simulation =
            simulate_heat_balance_zone_air_temperatures(&model, &[10.0], last_state_options)?;
        assert_eq!(
            last_state_simulation.summary.zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::LastSystemState
        );
        let steps = model.typed.timestep.number_of_timesteps_per_hour.max(1);
        let timestep_seconds = SECONDS_PER_HOUR / f64::from(steps);
        let mut state =
            initialize_heat_balance_state(&model, options.initial_zone_air_temperature_c)?;
        let mut surface_convection_sum = 0.0;
        let mut air_storage_sum = 0.0;
        let mut last_surface_convection = 0.0;
        let mut last_air_storage = 0.0;

        for _substep in 1..=steps {
            advance_heat_balance_state_one_timestep_internal(
                &model.typed,
                &mut state,
                HeatBalanceStepInput {
                    outdoor_dry_bulb_c: 10.0,
                    hour_ending: 1,
                    timestep_seconds,
                },
                None,
                options.zone_air_algorithm,
                options.surface_iteration_count,
                options.inside_hconv_reevaluation_interval,
                options.surface_loop_zone_air_correction,
            );
            let zone = &state.zones[0];
            last_surface_convection = zone_air_heat_balance_surface_convection_rate_w(zone);
            last_air_storage = zone_air_heat_balance_air_storage_rate_w(
                zone,
                timestep_seconds,
                options.zone_air_algorithm,
                None,
            );
            surface_convection_sum += last_surface_convection;
            air_storage_sum += last_air_storage;
        }

        let divisor = f64::from(steps);
        let surface_convection_series = simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate")
            .ok_or_else(|| std::io::Error::other("missing surface convection series"))?;
        assert!(
            (surface_convection_series.values[0] - surface_convection_sum / divisor).abs() < 1.0e-9
        );
        let last_surface_convection_series = last_state_simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate")
            .ok_or_else(|| std::io::Error::other("missing last surface convection series"))?;
        assert!(
            (last_surface_convection_series.values[0] - last_surface_convection).abs() < 1.0e-9
        );
        let air_storage_series = simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate")
            .ok_or_else(|| std::io::Error::other("missing air storage series"))?;
        assert!((air_storage_series.values[0] - air_storage_sum / divisor).abs() < 1.0e-9);
        let last_air_storage_series = last_state_simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate")
            .ok_or_else(|| std::io::Error::other("missing last air storage series"))?;
        assert!((last_air_storage_series.values[0] - last_air_storage).abs() < 1.0e-9);

        Ok(())
    }

    #[test]
    fn zone_surface_report_conduction_rates_sum_surface_report_terms()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone_id = state.zones[0].zone_id;
        for surface in &mut state.surfaces {
            surface.ctf = SurfaceCtfState {
                outside_0_w_per_m2_k: 0.0,
                cross_0_w_per_m2_k: 0.0,
                inside_0_w_per_m2_k: 0.0,
                const_in_part_w_per_m2: 0.0,
                const_out_part_w_per_m2: 0.0,
                outside_history_w_per_m2_k: Vec::new(),
                cross_history_w_per_m2_k: Vec::new(),
                inside_history_w_per_m2_k: Vec::new(),
                flux_history: Vec::new(),
                outside_temperature_history_c: Vec::new(),
                inside_temperature_history_c: Vec::new(),
                outside_flux_history_w_per_m2: Vec::new(),
                inside_flux_history_w_per_m2: Vec::new(),
            };
        }

        let [first, second, ..] = state.surfaces.as_mut_slice() else {
            return Err(std::io::Error::other("missing test surfaces").into());
        };
        first.area_m2 = 2.0;
        first.inside_face_temperature_c = 20.0;
        first.outside_face_temperature_c = 10.0;
        first.ctf.cross_0_w_per_m2_k = 1.0;
        first.ctf.outside_0_w_per_m2_k = 0.5;
        first.ctf.const_in_part_w_per_m2 = 3.0;
        first.ctf.const_out_part_w_per_m2 = 4.0;

        second.area_m2 = 3.0;
        second.inside_face_temperature_c = 18.0;
        second.outside_face_temperature_c = 12.0;
        second.ctf.cross_0_w_per_m2_k = 2.0;
        second.ctf.inside_0_w_per_m2_k = 1.0;
        second.ctf.outside_0_w_per_m2_k = 1.5;
        second.ctf.const_in_part_w_per_m2 = -1.0;
        second.ctf.const_out_part_w_per_m2 = 0.5;

        let (inside, outside) =
            zone_surface_report_conduction_rates_w(&state.surfaces, zone_id, false);
        assert!((inside - 41.0).abs() < 1.0e-12);
        assert!((outside - 74.5).abs() < 1.0e-12);

        Ok(())
    }
