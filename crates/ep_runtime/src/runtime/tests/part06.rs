    #[test]
    fn heat_balance_zone_air_algorithm_option_defaults_to_simplified() {
        let options = HeatBalanceSimulationOptions::hourly_samples(2);

        assert_eq!(
            options.zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        );
        assert_eq!(options.surface_iteration_count, 1);
        assert_eq!(
            options.zone_conduction_report_source,
            HeatBalanceZoneConductionReportSource::ZoneState
        );
        assert_eq!(
            options.zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::Average
        );
        assert_eq!(
            options.surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration
        );
        assert_eq!(
            options
                .with_zone_conduction_report_source(
                    HeatBalanceZoneConductionReportSource::SurfaceReport
                )
                .zone_conduction_report_source,
            HeatBalanceZoneConductionReportSource::SurfaceReport
        );
        assert_eq!(
            options
                .with_zone_air_report_sampling(HeatBalanceZoneAirReportSampling::LastSystemState)
                .zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::LastSystemState
        );
        assert_eq!(
            options
                .with_surface_loop_zone_air_correction(
                    HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
                )
                .surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe)
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe,
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe)
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe
        );
        assert_eq!(
            options
                .with_surface_iteration_count(0)
                .surface_iteration_count,
            1
        );
        assert_eq!(
            options
                .with_surface_iteration_count(3)
                .surface_iteration_count,
            3
        );
    }

    #[test]
    fn heat_balance_surface_loop_zone_air_correction_runs_after_loop_probe()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[5.0, 35.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe,
                )
                .with_surface_iteration_count(3)
                .with_surface_loop_zone_air_correction(
                    HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop,
                ),
        )?;

        assert_eq!(
            simulation.summary.surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
        );
        let zone_temperature = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
            .ok_or_else(|| std::io::Error::other("missing zone temperature series"))?;
        assert_eq!(zone_temperature.values.len(), 2);

        Ok(())
    }

    #[test]
    fn heat_balance_uses_source_declared_doe2_outside_convection() {
        let mut model = TypedModel::default();

        assert!(!heat_balance_uses_doe2_outside_convection(
            &model,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        ));
        assert!(heat_balance_uses_doe2_outside_convection(
            &model,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
        ));

        model.surface_convection_algorithms.outside = Some(OutsideSurfaceConvectionAlgorithm::Doe2);

        assert!(heat_balance_uses_doe2_outside_convection(
            &model,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        ));
    }

    #[test]
    fn quick_outside_probe_reuses_cached_exterior_report_terms()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let Some(surface) = state.surfaces.iter_mut().find(|surface| {
            surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
        }) else {
            return Err(std::io::Error::other("missing outdoor surface").into());
        };
        surface.outside_report_terms = SurfaceExteriorReportTerms {
            convection_heat_gain_rate_w: 1.0,
            convection_heat_gain_rate_per_area_w_per_m2: 2.0,
            convection_coefficient_w_per_m2_k: 3.0,
            net_thermal_radiation_heat_gain_rate_w: 4.0,
            net_thermal_radiation_heat_gain_rate_per_area_w_per_m2: 5.0,
            thermal_radiation_to_air_coefficient_w_per_m2_k: 6.0,
            thermal_radiation_to_sky_coefficient_w_per_m2_k: 7.0,
            thermal_radiation_to_ground_coefficient_w_per_m2_k: 8.0,
            solar_radiation_heat_gain_rate_w: 9.0,
            solar_radiation_heat_gain_rate_per_area_w_per_m2: 10.0,
        };

        let cached_terms = surface_exterior_report_terms(
            &model.typed,
            surface,
            10.0,
            surface.outside_face_temperature_c,
            None,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
        );
        let fallback_terms = surface_exterior_report_terms(
            &model.typed,
            surface,
            10.0,
            surface.outside_face_temperature_c,
            None,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
        );

        assert_eq!(cached_terms, surface.outside_report_terms);
        assert_eq!(fallback_terms, SurfaceExteriorReportTerms::default());

        Ok(())
    }

    #[test]
    fn quick_outside_balance_freezes_exterior_coefficient_temperature()
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
        let record = weather_record_with_precipitation(0.0);

        let quick_context = QuickOutsideConductionContext {
            reference_air_temperature_c: 20.0,
            inside_convection_coefficient_w_per_m2_k: 3.0,
            net_inside_source_w_per_m2: 0.0,
            exterior_coefficient_surface_temperature_c: Some(20.0),
            use_doe2_outside_convection: true,
        };
        let frozen = exterior_surface_energy_balance(
            surface_state,
            typed_surface,
            &record,
            10.0,
            20.0,
            0.0,
            Terrain::Suburbs,
            0.0,
            0.0,
            300.0,
            Some(quick_context),
            true,
            10.0,
            0.0,
            quick_context.exterior_coefficient_surface_temperature_c,
        );
        let unfrozen = exterior_surface_energy_balance(
            surface_state,
            typed_surface,
            &record,
            10.0,
            20.0,
            0.0,
            Terrain::Suburbs,
            0.0,
            0.0,
            300.0,
            Some(QuickOutsideConductionContext {
                exterior_coefficient_surface_temperature_c: None,
                ..quick_context
            }),
            true,
            10.0,
            0.0,
            None,
        );
        let expected_coefficient = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            20.0,
            10.0,
            surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices)
                .to_radians()
                .cos(),
            surface_azimuth_deg(&typed_surface.vertices),
            0.0,
            0.0,
            surface_state.outside_layer_roughness,
        );

        assert!(
            (frozen
                .exterior_report_terms
                .convection_coefficient_w_per_m2_k
                - expected_coefficient)
                .abs()
                < 1.0e-12
        );
        assert!(
            unfrozen
                .exterior_report_terms
                .convection_coefficient_w_per_m2_k
                > frozen
                    .exterior_report_terms
                    .convection_coefficient_w_per_m2_k
                    + 1.0
        );

        Ok(())
    }

    #[test]
    fn energyplus_weather_record_is_rain_uses_hourly_threshold() {
        let mut record = weather_record_with_precipitation(0.799);
        assert!(!energyplus_weather_record_is_rain_at_timestep(
            &[record],
            0,
            1,
            1
        ));

        record.liquid_precipitation_depth_mm = 0.8;
        assert!(energyplus_weather_record_is_rain_at_timestep(
            &[record],
            0,
            1,
            1
        ));
    }

    #[test]
    fn energyplus_wet_timestep_fraction_uses_weather_interpolation() {
        let typed = cube_model();
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .expect("roof test surface");
        let records = [
            weather_record_with_precipitation(21.0),
            weather_record_with_precipitation(0.0),
        ];

        assert_eq!(
            energyplus_exterior_wet_timestep_fraction(&records, 1, 4, typed_surface),
            0.75
        );
    }

    #[test]
    fn energyplus_weather_context_uses_timestep_rain_and_dry_bulb_interpolation() {
        let typed = cube_model();
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .expect("roof test surface");
        let mut previous = weather_record_with_precipitation(0.0);
        previous.dry_bulb_c = 10.0;
        previous.relative_humidity_percent = 40.0;
        previous.atmospheric_pressure_pa = 80_000.0;
        previous.wind_speed_m_per_s = 2.0;
        previous.wind_direction_deg = 350.0;
        let mut current = weather_record_with_precipitation(1.0);
        current.dry_bulb_c = 22.0;
        current.relative_humidity_percent = 80.0;
        current.atmospheric_pressure_pa = 84_000.0;
        current.wind_speed_m_per_s = 10.0;
        current.wind_direction_deg = 10.0;
        previous.horizontal_infrared_radiation_wh_per_m2 = 200.0;
        current.horizontal_infrared_radiation_wh_per_m2 = 600.0;
        let records = [previous, current];

        assert!(
            (energyplus_weather_dry_bulb_at_timestep(Some(&records), 1, 22.0, 4, 2) - 16.0).abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_wind_speed_at_timestep(&records, 1, 10.0, 4, 2) - 6.0).abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_relative_humidity_at_timestep(&records, 1, 80.0, 4, 2) - 60.0)
                .abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_atmospheric_pressure_at_timestep(&records, 1, 84_000.0, 4, 2)
                - 82_000.0)
                .abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_wind_direction_at_timestep(&records, 1, 10.0, 4, 2) - 0.0).abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_horizontal_infrared_at_timestep(&records, 1, 600.0, 4, 2) - 400.0)
                .abs()
                < 1.0e-12
        );
        assert_eq!(
            energyplus_exterior_wet_context_fraction(
                HeatBalanceWeatherContext {
                    records: &records,
                    record_index: 1,
                    zone_steps_per_hour: 4,
                    zone_timestep: Some(3),
                    first_hour_interpolation_starting_values:
                        FirstHourInterpolationStartingValues::Hour24,
                },
                typed_surface,
            ),
            0.0
        );
        assert_eq!(
            energyplus_exterior_wet_context_fraction(
                HeatBalanceWeatherContext {
                    records: &records,
                    record_index: 1,
                    zone_steps_per_hour: 4,
                    zone_timestep: Some(4),
                    first_hour_interpolation_starting_values:
                        FirstHourInterpolationStartingValues::Hour24,
                },
                typed_surface,
            ),
            1.0
        );
        assert_eq!(
            energyplus_exterior_wet_context_fraction(
                HeatBalanceWeatherContext {
                    records: &records,
                    record_index: 1,
                    zone_steps_per_hour: 4,
                    zone_timestep: None,
                    first_hour_interpolation_starting_values:
                        FirstHourInterpolationStartingValues::Hour24,
                },
                typed_surface,
            ),
            0.25
        );
    }

    #[test]
    fn first_hour_weather_interpolation_uses_run_period_day_seed() {
        let mut records = vec![weather_record_with_precipitation(0.0); 25];
        records[0].dry_bulb_c = -3.0;
        records[23].dry_bulb_c = -11.0;
        records[24].dry_bulb_c = 4.0;

        let default_hour24 =
            energyplus_weather_dry_bulb_at_timestep(Some(&records), 0, records[0].dry_bulb_c, 4, 1);
        let explicit_hour1 = energyplus_weather_dry_bulb_at_timestep_with_starting_values(
            Some(&records),
            0,
            records[0].dry_bulb_c,
            4,
            1,
            FirstHourInterpolationStartingValues::Hour1,
        );

        assert!((default_hour24 - -9.0).abs() < 1.0e-12);
        assert!((explicit_hour1 - -3.0).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_zone_air_heat_capacity_uses_moist_air_psychrometrics() {
        let humidity_ratio = 0.0075;
        let density = energyplus_moist_air_density_kg_per_m3(82_000.0, 20.0, humidity_ratio)
            .expect("valid moist-air density");
        let expected_density =
            82_000.0 / (287.0 * (20.0 + KELVIN_OFFSET) * (1.0 + 1.607_768_7 * humidity_ratio));
        assert!((density - expected_density).abs() < 1.0e-12);

        let specific_heat = energyplus_moist_air_specific_heat_j_per_kg_k(humidity_ratio);
        let expected_specific_heat = 1.004_84e3 + humidity_ratio * 1.858_95e3;
        assert!((specific_heat - expected_specific_heat).abs() < 1.0e-12);

        let volume_m3 = 10.0;
        let heat_capacity =
            energyplus_zone_air_heat_capacity_j_per_k(volume_m3, 82_000.0, 20.0, humidity_ratio)
                .expect("valid zone air heat capacity");
        assert!(
            (heat_capacity - volume_m3 * expected_density * expected_specific_heat).abs() < 1.0e-9
        );
        assert!(heat_capacity < volume_m3 * 1.2 * 1006.0);
    }

    #[test]
    fn weather_context_updates_zone_air_heat_capacity_from_pressure_and_zone_humidity()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed);
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let initial_capacity = state.zones[0].air_heat_capacity_j_per_k;
        state.zones[0].air_humidity_ratio = 0.0025;

        let mut previous = weather_record_with_precipitation(0.0);
        previous.dry_bulb_c = 10.0;
        previous.relative_humidity_percent = 40.0;
        previous.atmospheric_pressure_pa = 80_000.0;
        let mut current = weather_record_with_precipitation(0.0);
        current.dry_bulb_c = 22.0;
        current.relative_humidity_percent = 80.0;
        current.atmospheric_pressure_pa = 84_000.0;
        let records = [previous, current];
        let context = HeatBalanceWeatherContext {
            records: &records,
            record_index: 1,
            zone_steps_per_hour: 4,
            zone_timestep: Some(2),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        };

        update_zone_air_heat_capacities_from_weather_context(
            &mut state.zones,
            Some(context),
            current.dry_bulb_c,
        );

        let expected_capacity = energyplus_zone_air_heat_capacity_j_per_k(
            state.zones[0].volume_m3,
            82_000.0,
            20.0,
            0.0025,
        )
        .expect("valid expected capacity");
        assert!((state.zones[0].air_heat_capacity_j_per_k - expected_capacity).abs() < 1.0e-9);
        assert!(state.zones[0].air_heat_capacity_j_per_k < initial_capacity);

        Ok(())
    }
