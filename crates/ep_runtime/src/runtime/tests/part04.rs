    #[test]
    fn ground_ctf_history_seeding_uses_energyplus_building_surface_default()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;
        state.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Ground;

        seed_initial_surface_ctf_boundary_histories(&mut state, 5.0);

        let surface = &state.surfaces[0];
        let expected_flux = surface_steady_u_value_w_per_m2_k(surface)
            * (ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C
                - ENERGYPLUS_ZONE_INITIAL_TEMP_C);
        assert_eq!(
            surface.outside_face_temperature_c,
            ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C
        );
        assert_eq!(
            surface.ctf.outside_temperature_history_c,
            vec![ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C]
        );
        assert!((surface.ctf.outside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert!((surface.ctf.inside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_initial_ctf_history_seeding_applies_boundary_reset_and_steady_flux()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;
        seed_initial_surface_ctf_boundary_histories(&mut state, 5.0);

        seed_energyplus_initial_surface_ctf_histories(
            &mut state,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            5.0,
        );

        let surface = &state.surfaces[0];
        let expected_flux =
            surface_steady_u_value_w_per_m2_k(surface) * (5.0 - ENERGYPLUS_ZONE_INITIAL_TEMP_C);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![5.0]);
        assert_eq!(
            surface.ctf.inside_temperature_history_c,
            vec![ENERGYPLUS_ZONE_INITIAL_TEMP_C]
        );
        assert!((surface.ctf.outside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert!((surface.ctf.inside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert_eq!(
            surface.inside_face_temperature_c,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C
        );
        assert_eq!(surface.outside_face_temperature_c, 5.0);

        Ok(())
    }

    #[test]
    fn heat_balance_options_track_initial_ctf_history_policy() {
        let options = HeatBalanceSimulationOptions::hourly_samples(24)
            .with_ctf_initial_history_policy(
                HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial,
            );

        assert_eq!(
            options.ctf_initial_history_policy,
            HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial
        );
    }

    #[test]
    fn energyplus_ctf_inside_face_balance_handles_standard_and_adiabatic()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 19.0;
        surface.ctf.inside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;

        let standard = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: 4.0,
            },
        );
        assert!((standard - 14.0).abs() < 1.0e-12);

        surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        let adiabatic = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: 4.0,
            },
        );
        assert!((adiabatic - (135.0 / 9.5)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn surface_balance_pass_can_freeze_outside_snapshot_for_inside_ctf_solve()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_id = state.surfaces[0].surface_id;
        let zone_id = state.surfaces[0].zone_id;
        let surface = &mut state.surfaces[0];
        surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;
        surface.outside_face_temperature_c = 30.0;
        surface.inside_face_temperature_c = 18.0;
        surface.inside_radiant_internal_gain_w_per_m2 = 0.0;
        surface.inside_shortwave_absorbed_w_per_m2 = 0.0;
        surface.inside_additional_heat_source_w_per_m2 = 0.0;
        surface.inside_radiant_hvac_w_per_m2 = 0.0;
        surface.inside_net_longwave_w_per_m2 = 0.0;
        surface.ctf = SurfaceCtfState {
            outside_0_w_per_m2_k: 4.0,
            cross_0_w_per_m2_k: 0.5,
            inside_0_w_per_m2_k: 3.0,
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

        let first_pass_inside_temperatures = BTreeMap::from([(surface_id, 18.0)]);
        let zone_temperatures = BTreeMap::from([(zone_id, 20.0)]);
        let inside_convection_coefficients = BTreeMap::from([(surface_id, 2.0)]);
        let outside_snapshots = BTreeMap::from([(
            surface_id,
            SurfaceBoundaryBalanceResult {
                temperature_c: 12.0,
                exterior_report_terms: SurfaceExteriorReportTerms {
                    convection_heat_gain_rate_w: 77.0,
                    ..SurfaceExteriorReportTerms::default()
                },
                outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
            },
        )]);

        run_surface_balance_passes(
            &model.typed,
            &mut state.surfaces,
            Some(&first_pass_inside_temperatures),
            None,
            None,
            &zone_temperatures,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: -20.0,
                hour_ending: 1,
                timestep_seconds: SECONDS_PER_HOUR,
            },
            None,
            1,
            false,
            false,
            false,
            None,
            false,
            InteriorLongwaveExchangeProbe::None,
            Some(&inside_convection_coefficients),
            None,
            Some(&outside_snapshots),
            None,
            false,
        );

        let surface = &state.surfaces[0];
        assert_eq!(surface.outside_face_temperature_c, 12.0);
        assert_eq!(
            surface.outside_report_terms.convection_heat_gain_rate_w,
            77.0
        );
        assert!((surface.inside_face_temperature_c - 13.6).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn surface_balance_pass_can_freeze_inside_ctf_outside_snapshot_without_mutating_report_state()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_id = state.surfaces[0].surface_id;
        let zone_id = state.surfaces[0].zone_id;
        let surface = &mut state.surfaces[0];
        surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;
        surface.outside_face_temperature_c = 30.0;
        surface.inside_face_temperature_c = 18.0;
        surface.inside_radiant_internal_gain_w_per_m2 = 0.0;
        surface.inside_shortwave_absorbed_w_per_m2 = 0.0;
        surface.inside_additional_heat_source_w_per_m2 = 0.0;
        surface.inside_radiant_hvac_w_per_m2 = 0.0;
        surface.inside_net_longwave_w_per_m2 = 0.0;
        surface.ctf = SurfaceCtfState {
            outside_0_w_per_m2_k: 4.0,
            cross_0_w_per_m2_k: 0.5,
            inside_0_w_per_m2_k: 3.0,
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

        let first_pass_inside_temperatures = BTreeMap::from([(surface_id, 18.0)]);
        let zone_temperatures = BTreeMap::from([(zone_id, 20.0)]);
        let inside_convection_coefficients = BTreeMap::from([(surface_id, 2.0)]);
        let inside_ctf_outside_temperature_snapshots = BTreeMap::from([(surface_id, 12.0)]);

        run_surface_balance_passes(
            &model.typed,
            &mut state.surfaces,
            Some(&first_pass_inside_temperatures),
            None,
            None,
            &zone_temperatures,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: -20.0,
                hour_ending: 1,
                timestep_seconds: SECONDS_PER_HOUR,
            },
            None,
            1,
            false,
            false,
            false,
            None,
            false,
            InteriorLongwaveExchangeProbe::None,
            Some(&inside_convection_coefficients),
            None,
            None,
            Some(&inside_ctf_outside_temperature_snapshots),
            false,
        );

        let surface = &state.surfaces[0];
        assert!((surface.outside_face_temperature_c - 12.0).abs() > 1.0e-6);
        assert!((surface.inside_ctf_outside_temperature_c - 12.0).abs() < 1.0e-12);
        assert_ne!(
            surface.outside_report_terms.convection_heat_gain_rate_w,
            77.0
        );
        assert!((surface.inside_face_temperature_c - 13.6).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn surface_inside_ctf_source_terms_follow_energyplus_temp_term_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 19.0;
        surface.ctf.inside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;
        surface.inside_radiant_internal_gain_w_per_m2 = 1.0;
        surface.inside_shortwave_absorbed_w_per_m2 = 2.0;
        surface.inside_additional_heat_source_w_per_m2 = 3.0;
        surface.inside_radiant_hvac_w_per_m2 = 4.0;
        surface.inside_net_longwave_w_per_m2 = 5.0;

        let source_terms = surface_inside_ctf_source_terms_w_per_m2(surface);
        assert!((source_terms - 15.0).abs() < 1.0e-12);

        let temperature = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: source_terms,
            },
        );
        assert!((temperature - 15.1).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn radiant_internal_gains_follow_energyplus_area_absorptance_distribution()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.other_equipment[0].fraction_radiant = 0.25;
        let model = SimulationModel::from_typed(typed);
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        let absorbed_radiant_gain_w = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_radiant_internal_gain_w_per_m2 * surface.area_m2)
            .sum::<f64>();
        assert!((absorbed_radiant_gain_w - 3.0).abs() < 1.0e-12);
        for surface in &state.surfaces {
            assert!((surface.inside_radiant_internal_gain_w_per_m2 - 0.5).abs() < 1.0e-12);
        }

        state.surfaces[0].inside_radiant_internal_gain_w_per_m2 = 10.0;
        update_surface_radiant_internal_gain_source_terms(&model.typed, &mut state.surfaces, 1);
        assert!((state.surfaces[0].inside_radiant_internal_gain_w_per_m2 - 0.5).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn interior_longwave_probe_is_zero_for_equal_surface_temperatures()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 21.0;
            surface.inside_net_longwave_w_per_m2 = 12.0;
        }

        update_surface_inside_longwave_exchange_probe(&mut state.surfaces, None);

        for surface in &state.surfaces {
            assert!(surface.inside_net_longwave_w_per_m2.abs() < 1.0e-12);
        }

        Ok(())
    }

    #[test]
    fn interior_longwave_probe_conserves_zone_exchange_signs()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 20.0;
        }
        state.surfaces[0].inside_face_temperature_c = 30.0;

        update_surface_inside_longwave_exchange_probe(&mut state.surfaces, None);

        assert!(state.surfaces[0].inside_net_longwave_w_per_m2 < 0.0);
        for surface in state.surfaces.iter().skip(1) {
            assert!(surface.inside_net_longwave_w_per_m2 > 0.0);
        }
        let zone_exchange_w = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_net_longwave_w_per_m2 * surface.area_m2)
            .sum::<f64>();
        assert!(zone_exchange_w.abs() < 1.0e-9);

        Ok(())
    }

    #[test]
    fn scriptf_interior_longwave_probe_is_zero_for_equal_surface_temperatures()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 21.0;
            surface.inside_net_longwave_w_per_m2 = 12.0;
        }

        update_surface_inside_scriptf_longwave_exchange_probe(&mut state.surfaces, None);

        for surface in &state.surfaces {
            assert!(surface.inside_net_longwave_w_per_m2.abs() < 1.0e-9);
        }

        Ok(())
    }

    #[test]
    fn scriptf_interior_longwave_probe_conserves_zone_exchange_signs()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 20.0;
        }
        state.surfaces[0].inside_face_temperature_c = 30.0;

        update_surface_inside_scriptf_longwave_exchange_probe(&mut state.surfaces, None);

        assert!(state.surfaces[0].inside_net_longwave_w_per_m2 < 0.0);
        let zone_exchange_w = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_net_longwave_w_per_m2 * surface.area_m2)
            .sum::<f64>();
        assert!(zone_exchange_w.abs() < 1.0e-8);

        Ok(())
    }

    #[test]
    fn scriptf_from_view_factors_matches_energyplus_1zone_eio_orientation() {
        let areas = [69.6773, 69.6773, 69.6773, 69.6773, 232.2576, 232.2576];
        let printed_final_view_factors = [
            [0.0000, 0.078565, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.0000, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.0000, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.078565, 0.0000, 0.3823, 0.3823],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.0000, 0.5410],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.5410, 0.0000],
        ];
        let surface_count = areas.len();
        let mut internal_view_factors = vec![0.0; surface_count * surface_count];
        for from_index in 0..surface_count {
            for to_index in 0..surface_count {
                internal_view_factors[to_index * surface_count + from_index] =
                    printed_final_view_factors[from_index][to_index];
            }
        }
        let mut emissivities = vec![0.9; surface_count];

        let script_f =
            energyplus_scriptf_from_view_factors(&areas, &internal_view_factors, &mut emissivities)
                .expect("script F matrix");
        let dimensionless = |sender_index: usize, receiver_index: usize| {
            script_f[sender_index * surface_count + receiver_index] / STEFAN_BOLTZMANN_W_PER_M2_K4
        };

        assert!((dimensionless(0, 4) - 0.3366).abs() < 5.0e-4);
        assert!((dimensionless(4, 0) - 0.1010).abs() < 5.0e-4);
        assert!((dimensionless(4, 5) - 0.4559).abs() < 5.0e-4);
        assert!((dimensionless(0, 0) - 0.0094307).abs() < 5.0e-5);
    }

    #[test]
    fn approximate_view_factors_match_energyplus_1zone_eio() {
        let areas = [69.6773, 69.6773, 69.6773, 69.6773, 232.2576, 232.2576];
        let surface_types = [
            SurfaceType::Wall,
            SurfaceType::Wall,
            SurfaceType::Wall,
            SurfaceType::Wall,
            SurfaceType::Floor,
            SurfaceType::Roof,
        ];
        let azimuths = [180.0, 90.0, 0.0, 270.0, 0.0, 0.0];
        let tilts = [90.0, 90.0, 90.0, 90.0, 180.0, 0.0];
        let snapshots = areas
            .iter()
            .copied()
            .zip(surface_types)
            .zip(azimuths)
            .zip(tilts)
            .map(|(((area_m2, surface_type), azimuth_deg), tilt_deg)| {
                InteriorLongwaveSurfaceSnapshot {
                    zone_id: ZoneId(0),
                    surface_type,
                    area_m2,
                    azimuth_deg,
                    tilt_deg,
                    temperature_k4: 293.15_f64.powi(4),
                    thermal_absorptance: 0.9,
                }
            })
            .collect::<Vec<_>>();
        let view_factors = fix_energyplus_approximate_view_factors(
            &areas,
            &energyplus_approximate_view_factors(&snapshots),
        );
        let printed_final_view_factors = [
            [0.0000, 0.078565, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.0000, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.0000, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.078565, 0.0000, 0.3823, 0.3823],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.0000, 0.5410],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.5410, 0.0000],
        ];
        let surface_count = areas.len();
        for from_index in 0..surface_count {
            for to_index in 0..surface_count {
                let actual = view_factors[to_index * surface_count + from_index];
                let expected = printed_final_view_factors[from_index][to_index];
                assert!(
                    (actual - expected).abs() < 5.0e-4,
                    "view factor {from_index}->{to_index}: actual {actual}, expected {expected}"
                );
            }
        }
    }

    #[test]
    fn energyplus_ctf_outside_face_balance_uses_ctf_zero_terms()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.ctf.outside_0_w_per_m2_k = 1.0;
        surface.ctf.cross_0_w_per_m2_k = 1.0;
        surface.ctf.const_out_part_w_per_m2 = 0.0;

        let temperature = energyplus_ctf_outside_face_temperature_c(
            surface,
            CtfOutsideFaceBalanceInput {
                outdoor_air_temperature_c: 10.0,
                radiant_temperature_c: 5.0,
                outside_convection_coefficient_w_per_m2_k: 3.0,
                outside_radiation_coefficient_w_per_m2_k: 2.0,
                absorbed_outside_source_w_per_m2: 7.0,
            },
        );

        assert!((temperature - (67.0 / 6.0)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_ctf_quick_outside_face_balance_uses_inside_balance_term()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.ctf.outside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 2.0;
        surface.ctf.inside_0_w_per_m2_k = 4.0;
        surface.ctf.const_out_part_w_per_m2 = 11.0;
        surface.ctf.const_in_part_w_per_m2 = 13.0;

        let temperature = energyplus_ctf_outside_face_temperature_quick_conduction_c(
            surface,
            CtfOutsideQuickConductionBalanceInput {
                environmental: CtfOutsideFaceBalanceInput {
                    outdoor_air_temperature_c: 10.0,
                    radiant_temperature_c: 5.0,
                    outside_convection_coefficient_w_per_m2_k: 3.0,
                    outside_radiation_coefficient_w_per_m2_k: 2.0,
                    absorbed_outside_source_w_per_m2: 7.0,
                },
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 6.0,
                net_inside_source_w_per_m2: 17.0,
            },
        );

        assert!((temperature - (66.0 / 7.6)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn heat_balance_timestep_advances_zone_air_state() -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 10.0,
                hour_ending: 1,
                timestep_seconds: 600.0,
            },
        );

        assert_eq!(state.timestep_index, 1);
        assert_eq!(state.zones[0].previous_mean_air_temperatures_c, [20.0; 3]);
        assert_eq!(state.zones[0].convective_internal_gain_w, 12.0);
        assert_eq!(state.zones[0].opaque_surface_conductance_w_per_k, 6.0);
        assert!(state.zones[0].mean_air_temperature_c > 12.0);
        assert!(state.zones[0].mean_air_temperature_c < 20.0);
        assert!(state.zones[0].opaque_surface_heat_gain_w < 0.0);
        let expected_outside_conduction = state
            .surfaces
            .iter()
            .map(surface_outside_conduction_rate_w)
            .sum::<f64>();
        assert!(
            (state.zones[0].opaque_surface_outside_conduction_w - expected_outside_conduction)
                .abs()
                < 1.0e-12
        );
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 10.0);
        assert!(
            state.surfaces[0].inside_face_temperature_c > state.zones[0].mean_air_temperature_c
        );
        assert!(state.surfaces[0].inside_face_temperature_c < 20.0);
        assert!(state.surfaces[0].heat_gain_to_zone_w < 0.0);
        let expected_sum_ha = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_convection_coefficient_w_per_m2_k * surface.area_m2)
            .sum::<f64>();
        let expected_sum_hat_surf = state
            .surfaces
            .iter()
            .map(|surface| {
                surface.inside_convection_coefficient_w_per_m2_k
                    * surface.area_m2
                    * surface.inside_face_temperature_c
            })
            .sum::<f64>();
        assert!((state.zones[0].sum_ha_w_per_k - expected_sum_ha).abs() < 1.0e-12);
        assert!((state.zones[0].sum_hat_surf_w - expected_sum_hat_surf).abs() < 1.0e-12);
        assert_eq!(state.zones[0].sum_hat_ref_w, 0.0);
        let coefficients = state.zones[0].zone_air_temperature_coefficients;
        assert!(
            (coefficients.temp_dependent_coefficient_w_per_k - expected_sum_ha).abs() < 1.0e-12
        );
        assert!(
            (coefficients.temp_independent_coefficient_w
                - (state.zones[0].convective_internal_gain_w + expected_sum_hat_surf))
                .abs()
                < 1.0e-12
        );
        assert!((coefficients.air_power_cap_w_per_k - (1207.2 / 600.0)).abs() < 1.0e-12);
        let expected_history = (1207.2 / 600.0) * (3.0 * 20.0 - 1.5 * 20.0 + 20.0 / 3.0);
        assert!((coefficients.third_order_history_term_w - expected_history).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn zone_air_heat_balance_storage_rate_uses_source_algorithm_branch()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone = &mut state.zones[0];
        zone.mean_air_temperature_c = 21.0;
        zone.previous_mean_air_temperatures_c = [20.0, 19.0, 18.0];
        zone.air_heat_capacity_j_per_k = 1200.0;
        zone.zone_air_temperature_coefficients
            .temp_dependent_coefficient_w_per_k = 5.0;
        zone.zone_air_temperature_coefficients
            .temp_independent_coefficient_w = 200.0;

        let analytical = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            None,
        );
        assert!((analytical - 95.0).abs() < 1.0e-12);

        let third_order = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
            None,
        );
        assert!((third_order - 20.0).abs() < 1.0e-12);

        let third_order_report_capacity = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
            Some(600.0),
        );
        assert!((third_order_report_capacity - 10.0).abs() < 1.0e-12);

        let invalid_timestep = zone_air_heat_balance_air_storage_rate_w(
            zone,
            0.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
            Some(600.0),
        );
        assert_eq!(invalid_timestep, 0.0);

        Ok(())
    }

    #[test]
    fn system_timestep_air_storage_report_uses_weather_proxy_capacity()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone = &mut state.zones[0];
        zone.mean_air_temperature_c = 21.0;
        zone.air_humidity_ratio = 0.012;
        zone.air_heat_capacity_j_per_k = 1200.0;
        let previous_system_temperature_c = 20.0;
        let system_timestep_seconds = 60.0;
        let records = [EpwRecord {
            year: 2026,
            month: 1,
            day: 1,
            hour: 1,
            minute: 60,
            dry_bulb_c: 5.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 50.0,
            atmospheric_pressure_pa: 82_000.0,
            horizontal_infrared_radiation_wh_per_m2: 300.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        }];
        let context = HeatBalanceWeatherContext {
            records: &records,
            record_index: 0,
            zone_steps_per_hour: 4,
            zone_timestep: Some(1),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        };
        let expected_capacity = energyplus_zone_air_heat_capacity_j_per_k(
            zone.volume_m3,
            82_000.0,
            zone.mean_air_temperature_c,
            zone.air_humidity_ratio,
        )
        .ok_or_else(|| std::io::Error::other("missing expected air capacity"))?;

        let storage_rate = zone_air_system_timestep_storage_report_rate_w(
            zone,
            previous_system_temperature_c,
            system_timestep_seconds,
            Some(context),
            records[0].dry_bulb_c,
        );
        let stale_capacity_rate = zone.air_heat_capacity_j_per_k
            * (zone.mean_air_temperature_c - previous_system_temperature_c)
            / system_timestep_seconds;
        let expected_rate = expected_capacity
            * (zone.mean_air_temperature_c - previous_system_temperature_c)
            / system_timestep_seconds;

        assert!((storage_rate - expected_rate).abs() < 1.0e-9);
        assert!((storage_rate - stale_capacity_rate).abs() > 1.0e-3);

        Ok(())
    }
