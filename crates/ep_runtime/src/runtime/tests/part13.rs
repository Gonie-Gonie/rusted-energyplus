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
