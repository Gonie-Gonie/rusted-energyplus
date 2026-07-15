    #[test]
    fn cached_heat_balance_step_keeps_precomputed_schedule_while_public_step_reads_live_model()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.other_equipment[0].fraction_radiant = 0.2;
        let mut model = SimulationModel::from_typed(typed);
        let schedule_cache =
            crate::schedules::precompute_hour_only_internal_gain_schedule_cache(&model.typed)?;
        let cached_initial_state =
            super::initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache(
                &model,
                20.0,
                &[],
                &schedule_cache,
            )?;
        let mut cached_state = cached_initial_state.clone();
        let mut fallback_state = cached_initial_state;

        model.typed.schedules[0].hourly_value = 0.25;
        let input = HeatBalanceStepInput {
            outdoor_dry_bulb_c: 20.0,
            hour_ending: 7,
            timestep_seconds: SECONDS_PER_HOUR / 6.0,
        };
        super::advance_heat_balance_state_one_timestep_internal_with_schedule_cache(
            &model.typed,
            &schedule_cache,
            &mut cached_state,
            input,
            None,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.runtime_config(),
            1,
            None,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
        );
        advance_heat_balance_state_one_timestep(&model.typed, &mut fallback_state, input);

        let full_schedule_convective_w = 12.0 * (1.0 - 0.2);
        assert_eq!(
            cached_state.zones[0].convective_internal_gain_w,
            full_schedule_convective_w
        );
        assert_eq!(
            fallback_state.zones[0].convective_internal_gain_w,
            full_schedule_convective_w * 0.25
        );
        assert!(cached_state
            .surfaces
            .iter()
            .zip(&fallback_state.surfaces)
            .all(|(cached, fallback)| {
                cached.inside_radiant_internal_gain_w_per_m2
                    > fallback.inside_radiant_internal_gain_w_per_m2
            }));
        assert!(cached_state
            .surfaces
            .iter()
            .any(|surface| surface.inside_radiant_internal_gain_w_per_m2 > 0.0));
        Ok(())
    }

    #[test]
    fn heat_balance_cache_reuses_hour_samples_across_substeps_warmup_and_two_days()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.schedules.clear();
        typed.compact_schedules.push(ScheduleCompact {
            id: ScheduleId(0),
            name: NormalizedName::new("Daily Gain Pattern"),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![ScheduleCompactDayProfile {
                    day_types: all_schedule_day_types(),
                    interpolation: ScheduleInterpolation::No,
                    segments: vec![
                        ScheduleCompactSegment {
                            until_minute_of_day: 60,
                            value: 0.25,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 24 * 60,
                            value: 0.75,
                        },
                    ],
                }],
            }],
        });
        let model = SimulationModel::from_typed(typed);
        let mut options = HeatBalanceSimulationOptions::hourly_samples(48);
        options.warmup = HeatBalanceWarmupOptions {
            enabled: true,
            minimum_days: 1,
            maximum_days: 1,
            temperature_convergence_tolerance_delta_c: 0.0,
            loads_convergence_tolerance_w: 0.0,
        };

        let simulation =
            simulate_heat_balance_zone_air_temperatures(&model, &[20.0; 48], options)?;
        let gain_series = simulation
            .results
            .find_series(
                "ZONE ONE",
                "Zone Air Heat Balance Internal Convective Heat Gain Rate",
            )
            .expect("internal convective gain output should exist");

        assert_eq!(gain_series.values.len(), 48);
        for day_start in [0, 24] {
            assert_eq!(gain_series.values[day_start], 3.0);
            assert!(gain_series.values[day_start + 1..day_start + 24]
                .iter()
                .all(|value| *value == 9.0));
        }
        assert_eq!(simulation.summary.warmup.day_count, 1);
        assert_eq!(simulation.summary.warmup.timestep_count, 24 * 6);
        assert_eq!(simulation.state.timestep_index, 72 * 6);
        let profile = simulation.internal_gain_schedule_cache_profile;
        assert_eq!(profile.referenced_only_cache_build_count, 1);
        assert_eq!(profile.cache_entry_count, 1);
        assert_eq!(profile.cache_logical_sample_count, 24);
        assert_eq!(profile.cache_build_compact_value_evaluation_count, 24);
        assert_eq!(profile.initialization.cached_value_lookup_count, 2);
        assert_eq!(profile.warmup.cached_value_lookup_count, 288);
        assert_eq!(profile.run_period.cached_value_lookup_count, 576);
        assert_eq!(profile.total_cached_value_lookup_count(), 866);
        assert_eq!(profile.total_live_fallback_lookup_count(), 0);
        assert_eq!(profile.total_live_schedule_family_chain_scan_count(), 0);
        assert_eq!(profile.total_compact_profile_resolution_count(), 0);
        assert_eq!(profile.total_compact_value_evaluation_count(), 0);
        Ok(())
    }

    #[test]
    fn cached_and_live_internal_gain_timestep_paths_are_bit_equal_with_nonvacuous_operation_counts()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.other_equipment[0].fraction_radiant = 0.2;
        typed.schedules.clear();
        typed.compact_schedules.push(ScheduleCompact {
            id: ScheduleId(0),
            name: NormalizedName::new("Profiled Compact Gain"),
            schedule_type_limits: None,
            periods: vec![ScheduleCompactPeriod {
                through_schedule_day_of_year: 366,
                day_profiles: vec![ScheduleCompactDayProfile {
                    day_types: all_schedule_day_types(),
                    interpolation: ScheduleInterpolation::No,
                    segments: vec![
                        ScheduleCompactSegment {
                            until_minute_of_day: 8 * 60,
                            value: 0.25,
                        },
                        ScheduleCompactSegment {
                            until_minute_of_day: 24 * 60,
                            value: 0.75,
                        },
                    ],
                }],
            }],
        });
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(1),
            name: NormalizedName::new("Profiled Constant Gain"),
            schedule_type_limits: None,
            hourly_value: 0.5,
        });
        let mut constant_equipment = typed.other_equipment[0].clone();
        constant_equipment.id = InternalGainId(1);
        constant_equipment.name = NormalizedName::new("Constant Profile Equipment");
        constant_equipment.schedule = Some(ScheduleId(1));
        constant_equipment.design_level_w = 4.0;
        typed.other_equipment.push(constant_equipment);
        let model = SimulationModel::from_typed(typed);
        let (schedule_cache, build_profile) =
            crate::schedules::precompute_hour_only_internal_gain_schedule_cache_profiled(
                &model.typed,
            )?;
        // Share one cached initialization so this A/B isolates repeated timestep access.
        let initial_state =
            super::initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache(
                &model,
                20.0,
                &[],
                &schedule_cache,
            )?;
        let mut cached_state = initial_state.clone();
        let mut live_state = initial_state;
        let mut cached_operations =
            crate::schedules::InternalGainSchedulePhaseOperations::default();
        let mut live_operations =
            crate::schedules::InternalGainSchedulePhaseOperations::default();

        for hour_ending in 1..=24 {
            let input = HeatBalanceStepInput {
                outdoor_dry_bulb_c: 20.0,
                hour_ending,
                timestep_seconds: SECONDS_PER_HOUR,
            };
            super::advance_heat_balance_state_one_timestep_internal_with_schedule_cache_profiled(
                &model.typed,
                &schedule_cache,
                &mut cached_operations,
                &mut cached_state,
                input,
                None,
                HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.runtime_config(),
                1,
                None,
                HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
            );
            super::advance_heat_balance_state_one_timestep_internal_with_live_schedule_profiled(
                &model.typed,
                &mut live_operations,
                &mut live_state,
                input,
                None,
                HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.runtime_config(),
                1,
                None,
                HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
            );

            assert_eq!(
                cached_state.zones[0].convective_internal_gain_w.to_bits(),
                live_state.zones[0].convective_internal_gain_w.to_bits()
            );
            assert_eq!(
                cached_state.zones[0].mean_air_temperature_c.to_bits(),
                live_state.zones[0].mean_air_temperature_c.to_bits()
            );
            for (cached, live) in cached_state.surfaces.iter().zip(&live_state.surfaces) {
                assert_eq!(
                    cached.inside_radiant_internal_gain_w_per_m2.to_bits(),
                    live.inside_radiant_internal_gain_w_per_m2.to_bits()
                );
                assert_eq!(
                    cached.inside_face_temperature_c.to_bits(),
                    live.inside_face_temperature_c.to_bits()
                );
                assert_eq!(
                    cached.outside_face_temperature_c.to_bits(),
                    live.outside_face_temperature_c.to_bits()
                );
            }
        }

        assert_eq!(build_profile.referenced_only_cache_build_count, 1);
        assert_eq!(build_profile.cache_entry_count, 2);
        assert_eq!(build_profile.cache_logical_sample_count, 48);
        assert_eq!(build_profile.cache_build_compact_value_evaluation_count, 24);
        assert_eq!(cached_operations.cached_value_lookup_count, 96);
        assert_eq!(cached_operations.live_fallback_lookup_count, 0);
        assert_eq!(cached_operations.live_schedule_family_chain_scan_count, 0);
        assert_eq!(cached_operations.compact_profile_resolution_count, 0);
        assert_eq!(cached_operations.compact_value_evaluation_count, 0);
        assert_eq!(live_operations.cached_value_lookup_count, 0);
        assert_eq!(live_operations.live_fallback_lookup_count, 96);
        assert_eq!(live_operations.live_schedule_family_chain_scan_count, 96);
        assert_eq!(live_operations.compact_profile_resolution_count, 48);
        assert_eq!(live_operations.compact_value_evaluation_count, 48);
        Ok(())
    }

    #[test]
    fn heat_balance_cache_ignores_unreferenced_invalid_compact_schedule()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.compact_schedules.push(ScheduleCompact {
            id: ScheduleId(99),
            name: NormalizedName::new("Unreferenced Invalid Calendar"),
            schedule_type_limits: None,
            periods: Vec::new(),
        });
        let model = SimulationModel::from_typed(typed);

        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[20.0],
            HeatBalanceSimulationOptions::hourly_samples(1),
        )?;

        assert_eq!(simulation.state.zones[0].convective_internal_gain_w, 12.0);
        Ok(())
    }
