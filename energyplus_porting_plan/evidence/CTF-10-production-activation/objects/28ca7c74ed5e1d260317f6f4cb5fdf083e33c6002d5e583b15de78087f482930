    #[test]
    fn previous_boundary_probe_keeps_adiabatic_outside_face_history()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![10.0, 12.0];

        let coupled = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_boundary = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(coupled_floor_outside_temperature) = coupled
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled floor outside temperature").into());
        };
        let Some(coupled_floor_inside_temperature) = coupled
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled floor inside temperature").into());
        };
        let Some(previous_boundary_floor_outside_temperature) = previous_boundary
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other(
                "missing previous-boundary floor outside temperature",
            )
            .into());
        };

        assert_eq!(coupled_floor_outside_temperature.values.len(), 2);
        assert_eq!(previous_boundary_floor_outside_temperature.values.len(), 2);
        assert_eq!(
            coupled_floor_outside_temperature.values[0],
            coupled_floor_inside_temperature.values[0]
        );
        assert!(
            (coupled_floor_outside_temperature.values[0]
                - previous_boundary_floor_outside_temperature.values[0])
                .abs()
                > 1.0e-6
        );

        Ok(())
    }

    #[test]
    fn interleaved_longwave_probe_freezes_adiabatic_outside_ctf_report_state()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![10.0, 12.0];

        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(floor_inside_conduction) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing floor inside conduction").into());
        };
        let Some(floor_outside_conduction) = simulation.results.find_series(
            "FLOOR",
            "Surface Outside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing floor outside conduction").into());
        };
        let Some(floor_storage) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate")
        else {
            return Err(std::io::Error::other("missing floor heat storage").into());
        };
        let Some(floor_storage_per_area) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate per Area")
        else {
            return Err(std::io::Error::other("missing floor heat storage per-area").into());
        };

        assert_eq!(floor_inside_conduction.values.len(), 2);
        assert_eq!(floor_outside_conduction.values.len(), 2);
        assert!(
            (floor_inside_conduction.values[0] - floor_outside_conduction.values[0]).abs() > 1.0e-6
        );
        assert!(
            (floor_storage.values[0]
                + floor_inside_conduction.values[0]
                + floor_outside_conduction.values[0])
                .abs()
                < 1.0e-9
        );
        assert!(
            (floor_storage_per_area.values[0] - floor_storage.values[0] / 100.0).abs() < 1.0e-9
        );

        Ok(())
    }

    #[test]
    fn frozen_reference_air_probe_changes_interleaved_surface_reference_air()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![5.0, 35.0];

        let active = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let frozen_reference_air = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(active_floor_inside_temperature) = active
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing active floor inside temperature").into());
        };
        let Some(frozen_floor_inside_temperature) = frozen_reference_air
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing frozen floor inside temperature").into());
        };
        let Some(active_zone_temperature) = active
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing active zone temperature").into());
        };
        let Some(frozen_zone_temperature) = frozen_reference_air
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing frozen zone temperature").into());
        };

        assert_eq!(active_floor_inside_temperature.values.len(), 2);
        assert_eq!(frozen_floor_inside_temperature.values.len(), 2);
        assert!(
            active_floor_inside_temperature
                .values
                .iter()
                .zip(&frozen_floor_inside_temperature.values)
                .any(|(active, frozen)| (active - frozen).abs() > 1.0e-9)
        );
        assert!(
            active_zone_temperature
                .values
                .iter()
                .zip(&frozen_zone_temperature.values)
                .any(|(active, frozen)| (active - frozen).abs() > 1.0e-9)
        );

        Ok(())
    }

    #[test]
    fn converged_surface_probe_changes_fixed_iteration_cap()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![5.0, 35.0];

        let fixed_iterations = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe,
                )
                .with_surface_iteration_count(20),
        )?;
        let converged_iterations = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe,
                )
                .with_surface_iteration_count(20),
        )?;

        let Some(fixed_floor_temperature) = fixed_iterations
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing fixed floor temperature").into());
        };
        let Some(converged_floor_temperature) = converged_iterations
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing converged floor temperature").into());
        };

        assert_eq!(fixed_floor_temperature.values.len(), 2);
        assert_eq!(converged_floor_temperature.values.len(), 2);
        assert!(
            fixed_floor_temperature
                .values
                .iter()
                .zip(&converged_floor_temperature.values)
                .any(|(fixed, converged)| (fixed - converged).abs() > 1.0e-9)
        );

        Ok(())
    }

    #[test]
    fn current_adiabatic_history_probe_syncs_adiabatic_outside_face_after_solve()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![10.0, 12.0];

        let active = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let current_history = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(active_inside_temperature) = active
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing active floor inside temperature").into());
        };
        let Some(active_outside_temperature) = active
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing active floor outside temperature").into());
        };
        let Some(current_inside_temperature) = current_history
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing current floor inside temperature").into());
        };
        let Some(current_outside_temperature) = current_history
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing current floor outside temperature").into());
        };

        assert_eq!(current_inside_temperature.values.len(), 2);
        assert_eq!(current_outside_temperature.values.len(), 2);
        assert!(
            (active_inside_temperature.values[0] - active_outside_temperature.values[0]).abs()
                > 1.0e-6
        );
        assert!(
            (current_inside_temperature.values[0] - current_outside_temperature.values[0]).abs()
                < 1.0e-9
        );

        Ok(())
    }

    #[test]
    fn adiabatic_history_commit_override_preserves_report_face_and_uses_inside_for_ctf_history()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 20.0;
        surface.ctf = SurfaceCtfState {
            outside_0_w_per_m2_k: 4.0,
            cross_0_w_per_m2_k: 2.0,
            inside_0_w_per_m2_k: 3.0,
            flux_0: None,
            const_in_part_w_per_m2: 1.0,
            const_out_part_w_per_m2: 5.0,
            outside_history_w_per_m2_k: vec![0.1, 0.2],
            cross_history_w_per_m2_k: vec![0.3, 0.4],
            inside_history_w_per_m2_k: vec![0.5, 0.6],
            flux_history: vec![0.7, 0.8],
            outside_temperature_history_c: vec![7.0, 8.0],
            inside_temperature_history_c: vec![17.0, 18.0],
            outside_flux_history_w_per_m2: vec![70.0, 80.0],
            inside_flux_history_w_per_m2: vec![170.0, 180.0],
        };

        advance_surface_ctf_histories_with_outside_temperature_override(surface, Some(20.0));

        assert_eq!(surface.outside_face_temperature_c, 10.0);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![20.0, 7.0]);
        assert_eq!(surface.ctf.inside_temperature_history_c, vec![20.0, 17.0]);
        assert_eq!(surface.ctf.inside_flux_history_w_per_m2, vec![-19.0, 170.0]);
        assert_eq!(surface.ctf.outside_flux_history_w_per_m2, vec![45.0, 70.0]);

        Ok(())
    }

    #[test]
    fn inside_ctf_outside_history_commit_override_only_uses_outdoor_snapshots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        let surface_id = surface.surface_id;
        surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;

        let mut snapshots = BTreeMap::new();
        snapshots.insert(surface_id, 12.5);

        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                true,
                Some(&snapshots)
            ),
            Some(12.5)
        );
        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                false,
                Some(&snapshots)
            ),
            None
        );

        snapshots.clear();
        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                true,
                Some(&snapshots)
            ),
            None
        );

        surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        snapshots.insert(surface_id, 15.0);
        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                true,
                Some(&snapshots)
            ),
            None
        );

        Ok(())
    }

    #[test]
    fn result_store_finds_series_case_insensitively() {
        let mut store = ResultStore::new();
        store.write_output_by_handle(
            OutputHandle(0),
            "ZONE ONE".to_string(),
            "Zone Mean Air Temperature".to_string(),
            "C".to_string(),
            vec![20.0, 21.0],
        );

        assert_eq!(store.sample_count(), 2);
        assert!(
            store
                .find_series("zone one", "zone mean air temperature")
                .is_some()
        );
        assert_eq!(store.find_handle(OutputHandle(0)).unwrap().values[1], 21.0);
    }

    #[test]
    fn runtime_output_registry_resolves_declared_model_outputs() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        assert_eq!(registry.len(), 157);
        assert!(registry.meter_registry().is_empty());

        let resolution = registry.resolve_output_requests(&[
            RuntimeOutputRequest::hourly("zone one", "Zone Mean Air Temperature"),
            RuntimeOutputRequest::hourly("floor", "Surface Inside Face Temperature"),
            RuntimeOutputRequest::hourly("floor", "Surface Inside Face Adjacent Air Temperature"),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Inside Face Conduction Heat Transfer Rate",
            ),
            RuntimeOutputRequest::hourly(
                "zone one",
                "Zone Opaque Surface Inside Faces Conduction Rate",
            ),
            RuntimeOutputRequest::hourly(
                "zone one",
                "Zone Opaque Surface Outside Faces Conduction Rate",
            ),
            RuntimeOutputRequest::hourly("floor", "Surface Heat Storage Rate"),
            RuntimeOutputRequest::hourly("floor", "Surface Heat Storage Rate per Area"),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Outside Face Incident Solar Radiation Rate per Area",
            ),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Outside Face Convection Heat Transfer Coefficient",
            ),
            RuntimeOutputRequest::hourly("environment", "Site Outdoor Air Drybulb Temperature"),
            RuntimeOutputRequest::hourly("environment", "Site Outdoor Air Wetbulb Temperature"),
            RuntimeOutputRequest::hourly("environment", "Site Rain Status"),
        ]);

        assert!(resolution.diagnostics.is_empty());
        assert_eq!(resolution.resolved.len(), 13);
        assert_eq!(resolution.resolved[0].definition.handle, OutputHandle(0));
        assert_eq!(resolution.resolved[1].definition.key, "FLOOR");
    }

    #[test]
    fn runtime_output_registry_records_duplicate_registration_diagnostic() {
        let mut typed = cube_model();
        let mut duplicate_zone = typed.zones[0].clone();
        duplicate_zone.id = ZoneId(1);
        typed.zones.push(duplicate_zone);
        let model = SimulationModel::from_typed(typed);
        let registry = RuntimeOutputRegistry::from_model(&model);

        assert!(registry.diagnostics().has_errors());
        assert!(
            registry
                .diagnostics()
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code
                    == RuntimeDiagnosticCode::DuplicateOutputRegistration)
        );
    }

    #[test]
    fn runtime_diagnostic_codes_cover_conformance_blockers() {
        assert_eq!(
            RuntimeDiagnosticCode::UnsupportedHeatBalanceBranch.id(),
            "UnsupportedHeatBalanceBranch"
        );
        assert_eq!(
            RuntimeDiagnosticCode::UnsupportedSurfaceBoundary.id(),
            "UnsupportedSurfaceBoundary"
        );
        assert_eq!(
            RuntimeDiagnosticCode::NonFiniteHeatBalanceState.id(),
            "NonFiniteHeatBalanceState"
        );
        assert_eq!(
            RuntimeDiagnosticCode::OutputVariableUnavailable.id(),
            "OutputVariableUnavailable"
        );
        assert_eq!(
            RuntimeDiagnosticCode::TimestampMismatch.id(),
            "TimestampMismatch"
        );
        assert_eq!(
            RuntimeDiagnosticCode::ToleranceFailure.id(),
            "ToleranceFailure"
        );
    }

    #[test]
    fn runtime_output_registry_skips_no_sun_surface_solar_output() {
        let mut typed = cube_model();
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        let model = SimulationModel::from_typed(typed);
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "floor",
            "Surface Outside Face Incident Solar Radiation Rate per Area",
        )]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::OutputVariableUnavailable
        );
    }

    #[test]
    fn runtime_output_registry_diagnoses_unavailable_output() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "ZONE ONE",
            "Zone Lights Electricity Energy",
        )]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::OutputVariableUnavailable
        );
    }

    #[test]
    fn runtime_output_registry_resolves_system_node_setpoint_output() {
        let model = ideal_loads_node_state_model();
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "ZONE ONE INLET",
            NODE_STATE_SETPOINT_VARIABLE,
        )]);

        assert!(resolution.diagnostics.is_empty());
        assert_eq!(resolution.resolved.len(), 1);
        assert_eq!(resolution.resolved[0].definition.key, "ZONE ONE INLET");
        assert_eq!(
            resolution.resolved[0].definition.variable_name,
            NODE_STATE_SETPOINT_VARIABLE
        );
    }

    #[test]
    fn runtime_meter_registry_diagnoses_unavailable_meter() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry
            .meter_registry()
            .resolve_meter_requests(&[RuntimeMeterRequest::hourly("Electricity:Facility")]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::MeterUnavailable
        );
    }

    #[test]
    fn runtime_meter_registry_resolves_ideal_loads_facility_meters() {
        let model = ideal_loads_node_state_model();
        let registry = RuntimeOutputRegistry::from_model(&model);
        let heating_binding =
            crate::ideal_loads_facility_meter_binding(IdealLoadsFuelType::DistrictHeatingWater)
                .expect("district heating meter binding");
        let cooling_binding =
            crate::ideal_loads_facility_meter_binding(IdealLoadsFuelType::DistrictCooling)
                .expect("district cooling meter binding");

        let resolution = registry.meter_registry().resolve_meter_requests(&[
            RuntimeMeterRequest::hourly(heating_binding.meter_name),
            RuntimeMeterRequest::hourly(cooling_binding.meter_name),
            RuntimeMeterRequest::new(heating_binding.meter_name, RuntimeOutputFrequency::Monthly),
            RuntimeMeterRequest::new(heating_binding.meter_name, RuntimeOutputFrequency::Annual),
            RuntimeMeterRequest::new(
                cooling_binding.meter_name,
                RuntimeOutputFrequency::RunPeriod,
            ),
        ]);

        assert_eq!(registry.meter_registry().len(), 8);
        assert_eq!(resolution.resolved.len(), 5);
        assert!(resolution.diagnostics.is_empty());
        assert_eq!(
            resolution.resolved[0].definition.name,
            "DistrictHeatingWater:Facility"
        );
        assert_eq!(
            resolution.resolved[0].definition.aggregation_plan.kind,
            RuntimeMeterAggregationKind::HeatingEnergyTransfer
        );
        assert_eq!(
            resolution.resolved[0].definition.aggregation_plan.period,
            RuntimeMeterAggregationPeriod::Hourly
        );
        assert_eq!(
            resolution.resolved[0].definition.dependency_output_handles.len(),
            1
        );
        assert_eq!(
            resolution.resolved[0]
                .definition
                .aggregation_plan
                .dependency_output_handles,
            resolution.resolved[0].definition.dependency_output_handles
        );
        assert_eq!(
            heating_binding.fuel_energy_variable,
            crate::ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY
        );
        assert_eq!(resolution.resolved[0].definition.units, "J");
        assert_eq!(
            resolution.resolved[1].definition.name,
            "DistrictCooling:Facility"
        );
        assert_eq!(
            resolution.resolved[1].definition.aggregation_plan.kind,
            RuntimeMeterAggregationKind::CoolingEnergyTransfer
        );
        assert_eq!(
            resolution.resolved[1].definition.dependency_output_handles.len(),
            1
        );
        assert_eq!(
            cooling_binding.fuel_energy_variable,
            crate::ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY
        );
        assert_eq!(resolution.resolved[1].definition.units, "J");
        assert_eq!(
            resolution.resolved[2].definition.frequency,
            RuntimeOutputFrequency::Monthly
        );
        assert_eq!(
            resolution.resolved[2].definition.aggregation_plan.period,
            RuntimeMeterAggregationPeriod::Monthly
        );
        assert_eq!(
            resolution.resolved[3].definition.frequency,
            RuntimeOutputFrequency::Annual
        );
        assert_eq!(
            resolution.resolved[3].definition.aggregation_plan.period,
            RuntimeMeterAggregationPeriod::Annual
        );
        assert_eq!(
            resolution.resolved[4].definition.frequency,
            RuntimeOutputFrequency::RunPeriod
        );
        assert_eq!(
            resolution.resolved[4].definition.aggregation_plan.period,
            RuntimeMeterAggregationPeriod::RunPeriod
        );
        assert_eq!(meter_rate_to_energy_j(2.5, 3600.0), 9000.0);
        assert!(meter_value_is_zero_near_j(METER_ZERO_NEAR_TOLERANCE_J));
        assert!(!meter_value_is_zero_near_j(
            METER_ZERO_NEAR_TOLERANCE_J * 10.0
        ));
    }

    #[test]
    fn runtime_meter_aggregation_kind_resolves_facility_meter_families() {
        assert_eq!(
            RuntimeMeterAggregationKind::from_meter_name(ELECTRICITY_FACILITY_METER),
            RuntimeMeterAggregationKind::FacilityElectricity
        );
        assert_eq!(
            RuntimeMeterAggregationKind::from_meter_name(GAS_FACILITY_METER),
            RuntimeMeterAggregationKind::FacilityGas
        );
        assert_eq!(
            RuntimeMeterAggregationKind::from_meter_name(HEATING_ENERGY_TRANSFER_METER),
            RuntimeMeterAggregationKind::HeatingEnergyTransfer
        );
        assert_eq!(
            RuntimeMeterAggregationKind::from_meter_name(COOLING_ENERGY_TRANSFER_METER),
            RuntimeMeterAggregationKind::CoolingEnergyTransfer
        );

        let source_map = component_output_to_facility_meter_source_map(
            "Fan Electricity Rate",
            ELECTRICITY_FACILITY_METER,
        );
        assert_eq!(source_map.component_output_variable, "Fan Electricity Rate");
        assert_eq!(source_map.facility_meter_name, ELECTRICITY_FACILITY_METER);
        assert_eq!(
            source_map.aggregation_kind,
            RuntimeMeterAggregationKind::FacilityElectricity
        );
        assert_eq!(
            source_map.source_map,
            COMPONENT_OUTPUT_TO_FACILITY_METER_SOURCE_MAP
        );
    }

    #[test]
    fn result_store_diagnostics_report_duplicate_handles() {
        let mut store = ResultStore::new();
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "ZONE ONE".to_string(),
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values: vec![20.0],
        });
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "Environment".to_string(),
            variable_name: "Site Outdoor Air Drybulb Temperature".to_string(),
            units: "C".to_string(),
            values: vec![10.0],
        });

        let diagnostics = store.diagnostics();

        assert!(diagnostics.has_errors());
        assert_eq!(
            diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::DuplicateOutputHandle
        );
        assert_eq!(store.profile().series_count, 2);
    }

    #[test]
    fn result_store_diagnostics_report_duplicate_system_node_handles() {
        let mut store = ResultStore::new();
        store.add_series(OutputSeries {
            handle: OutputHandle(7),
            key: "ZONE ONE INLET".to_string(),
            variable_name: "System Node Temperature".to_string(),
            units: "C".to_string(),
            values: vec![50.0],
        });
        store.add_series(OutputSeries {
            handle: OutputHandle(7),
            key: "ZONE ONE INLET".to_string(),
            variable_name: "System Node Humidity Ratio".to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values: vec![0.0156],
        });

        let diagnostics = store.diagnostics();

        assert!(diagnostics.has_errors());
        let diagnostic = &diagnostics.diagnostics[0];
        assert_eq!(
            diagnostic.code,
            RuntimeDiagnosticCode::DuplicateOutputHandle
        );
        assert_eq!(diagnostic.key.as_deref(), Some("ZONE ONE INLET"));
        assert_eq!(
            diagnostic.variable_name.as_deref(),
            Some("System Node Humidity Ratio")
        );
        assert_eq!(diagnostic.handle, Some(OutputHandle(7)));
        assert_eq!(diagnostic.stage.as_deref(), Some("result-store"));
        assert_eq!(diagnostic.surface, None);
        assert_eq!(diagnostic.zone, None);
        assert_eq!(diagnostic.timestep, None);
    }
