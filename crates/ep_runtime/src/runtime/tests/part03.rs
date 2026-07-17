    #[test]
    fn heat_balance_state_shell_initializes_cube_metrics() -> Result<(), Box<dyn std::error::Error>>
    {
        let model = SimulationModel::from_typed(cube_model());

        let state = initialize_heat_balance_state(&model, 20.0)?;

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.zones.len(), 1);
        assert_eq!(state.zones[0].zone_name, "ZONE ONE");
        assert_eq!(state.zones[0].mean_air_temperature_c, 20.0);
        assert_eq!(state.zones[0].zone_timestep_average_air_temperature_c, 20.0);
        assert_eq!(state.zones[0].previous_mean_air_temperatures_c, [20.0; 3]);
        assert_eq!(
            state.zones[0].previous_system_mean_air_temperatures_c,
            [20.0; 3]
        );
        assert_eq!(state.zones[0].previous_system_timestep_count, 1);
        assert_eq!(state.zones[0].volume_m3, 1.0);
        let expected_initial_air_heat_capacity =
            energyplus_standard_zone_air_heat_capacity_j_per_k(
                1.0,
                20.0,
                ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO,
            )
                .expect("valid standard initial air heat capacity");
        assert!(
            (state.zones[0].air_heat_capacity_j_per_k - expected_initial_air_heat_capacity).abs()
                < 1.0e-9
        );
        assert_eq!(state.zones[0].convective_internal_gain_w, 12.0);
        assert_eq!(state.zones[0].opaque_surface_conductance_w_per_k, 6.0);
        assert_eq!(state.zones[0].opaque_surface_heat_gain_w, 0.0);
        assert!((state.zones[0].sum_ha_w_per_k - 18.456).abs() < 1.0e-12);
        assert!((state.zones[0].sum_hat_surf_w - 369.12).abs() < 1.0e-12);
        assert_eq!(state.zones[0].sum_hat_ref_w, 0.0);
        assert!(
            (state.zones[0]
                .zone_air_temperature_coefficients
                .temp_dependent_coefficient_w_per_k
                - 18.456)
                .abs()
                < 1.0e-12
        );
        assert!(
            (state.zones[0]
                .zone_air_temperature_coefficients
                .temp_independent_coefficient_w
                - 381.12)
                .abs()
                < 1.0e-12
        );
        assert_eq!(
            state.zones[0]
                .zone_air_temperature_coefficients
                .air_power_cap_w_per_k,
            0.0
        );
        assert_eq!(state.surfaces.len(), 6);
        assert_ne!(state.construction_cache_hash, 0);
        assert!(state.construction_cache_build_wall_seconds >= 0.0);
        assert_eq!(state.construction_cache_entry_count, 1);
        assert_eq!(state.construction_cache_no_mass_count, 1);
        assert_eq!(state.construction_cache_massive_ctf_count, 0);
        assert_eq!(state.construction_cache_eio_seeded_count, 0);
        assert_eq!(state.construction_cache_rust_generated_count, 1);
        let construction_cache =
            crate::heat_balance::surface_manager::ConstructionThermalDataCache::build(
                &model.typed,
                &BTreeMap::new(),
            )?;
        let cache_token = construction_cache.invalidation_token();
        assert_eq!(
            cache_token.coefficient_cache_hash,
            construction_cache.coefficient_cache_hash
        );
        assert!(!construction_cache.is_invalidated_by(cache_token));
        let stale_token =
            crate::heat_balance::surface_manager::ConstructionCacheInvalidationToken::from_coefficient_cache_hash(
                cache_token.coefficient_cache_hash.wrapping_add(1),
            );
        assert!(construction_cache.is_invalidated_by(stale_token));
        let surface_slots = (0..state.surfaces.len()).collect::<Vec<_>>();
        assert_eq!(
            state.surface_indexes.surfaces_by_zone,
            vec![surface_slots.clone()]
        );
        assert_eq!(
            state.surface_indexes.surfaces_by_construction,
            vec![surface_slots.clone()]
        );
        assert_eq!(
            state.surface_indexes.opaque_surfaces_by_zone,
            vec![surface_slots.clone()]
        );
        assert_eq!(
            state.surface_indexes.opaque_surfaces,
            surface_slots.clone()
        );
        assert!(state.surface_indexes.fenestration_surfaces.is_empty());
        assert_eq!(
            state.surface_indexes.exterior_surfaces,
            surface_slots.clone()
        );
        assert!(state.surface_indexes.ground_surfaces.is_empty());
        assert!(state.surface_indexes.adiabatic_surfaces.is_empty());
        assert!(state.surface_indexes.interzone_surfaces.is_empty());
        assert_eq!(
            state.surface_indexes.output_requested_surfaces,
            surface_slots.clone()
        );
        assert_eq!(
            state.surface_indexes.ctf_surfaces,
            state.surface_indexes.opaque_surfaces
        );
        assert_eq!(
            state.surface_indexes.no_mass_surfaces,
            state.surface_indexes.opaque_surfaces
        );
        let floor = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        assert!((floor.tilt_deg - 180.0).abs() < 1.0e-9);
        let roof = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        assert!((roof.tilt_deg - 0.0).abs() < 1.0e-9);
        let wall = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall surface"))?;
        assert!((wall.tilt_deg - 90.0).abs() < 1.0e-9);
        assert_eq!(
            state.surfaces[0].outside_boundary_condition,
            OutsideBoundaryCondition::Outdoors
        );
        assert_eq!(state.surfaces[0].construction_name, "WALL");
        assert_eq!(state.surfaces[0].construction_thermal_data_index, 0);
        assert_eq!(state.surfaces[0].outside_layer_material_name, "R1");
        assert_eq!(
            state.surfaces[0].outside_layer_roughness,
            MaterialSurfaceRoughness::Rough
        );
        assert_eq!(state.surfaces[0].area_m2, 1.0);
        assert_eq!(state.surfaces[0].thermal_resistance_m2_k_per_w, 1.0);
        assert_eq!(state.surfaces[0].heat_capacity_j_per_m2_k, None);
        assert_eq!(state.surfaces[0].thermal_absorptance, 0.9);
        assert_eq!(state.surfaces[0].inside_thermal_absorptance, 0.9);
        assert_eq!(state.surfaces[0].conductance_w_per_k, 1.0);
        assert_eq!(
            state.surfaces[0].inside_convection_coefficient_w_per_m2_k,
            3.076
        );
        assert_eq!(state.surfaces[0].ctf.outside_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.cross_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.inside_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.const_in_part_w_per_m2, 0.0);
        assert_eq!(state.surfaces[0].ctf.const_out_part_w_per_m2, 0.0);
        assert_eq!(
            state.surfaces[0].ctf.outside_temperature_history_c,
            vec![20.0]
        );
        assert_eq!(state.surfaces[0].heat_gain_to_zone_w, 0.0);
        assert_eq!(state.surfaces[0].inside_face_temperature_c, 20.0);
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 20.0);

        Ok(())
    }

    #[test]
    fn same_model_heat_balance_states_reset_and_remain_independent()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());

        let mut first_state = initialize_heat_balance_state(&model, 20.0)?;
        let second_state = initialize_heat_balance_state(&model, 20.0)?;

        first_state.timestep_index = 12;
        first_state.zones[0].mean_air_temperature_c = 31.0;
        first_state.surfaces[0].inside_face_temperature_c = 28.0;

        assert_eq!(second_state.timestep_index, 0);
        assert_eq!(second_state.zones[0].mean_air_temperature_c, 20.0);
        assert_eq!(second_state.surfaces[0].inside_face_temperature_c, 20.0);

        let reset_state = initialize_heat_balance_state(&model, 20.0)?;
        assert_eq!(reset_state.timestep_index, 0);
        assert_eq!(reset_state.zones[0].mean_air_temperature_c, 20.0);
        assert_eq!(reset_state.surfaces[0].inside_face_temperature_c, 20.0);
        assert_eq!(reset_state.zones.len(), second_state.zones.len());
        assert_eq!(reset_state.surfaces.len(), second_state.surfaces.len());

        Ok(())
    }

    fn spectral_average_window_material(id: MaterialId, name: &str) -> Material {
        Material {
            id,
            name: NormalizedName::new(name),
            definition: ep_model::MaterialDefinition::WindowGlazingSpectralAverage(
                ep_model::WindowGlazingSpectralAverageMaterial {
                    thickness_m: 0.006,
                    solar_transmittance_at_normal_incidence: 0.775,
                    front_side_solar_reflectance_at_normal_incidence: 0.071,
                    back_side_solar_reflectance_at_normal_incidence: 0.071,
                    visible_transmittance_at_normal_incidence: 0.881,
                    front_side_visible_reflectance_at_normal_incidence: 0.08,
                    back_side_visible_reflectance_at_normal_incidence: 0.08,
                    infrared_transmittance_at_normal_incidence: 0.0,
                    front_side_infrared_hemispherical_emissivity: 0.84,
                    back_side_infrared_hemispherical_emissivity: 0.84,
                    conductivity_w_per_m_k: 0.9,
                    dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
                    solar_diffusing: false,
                    youngs_modulus_pa: 72.0e9,
                    poissons_ratio: 0.22,
                },
            ),
        }
    }

    #[test]
    fn opaque_runtime_filters_unreferenced_fenestration_construction()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed
            .materials
            .push(spectral_average_window_material(MaterialId(1), "Clear Glass"));
        typed.constructions.push(Construction {
            id: ConstructionId(1),
            name: NormalizedName::new("Window"),
            kind: ConstructionKind::Fenestration,
            outside_layer: Some(MaterialId(1)),
            layers: vec![MaterialId(1)],
            thermochromic_master: None,
            ground_factor: None,
            air_boundary: None,
            complex_fenestration: None,
            internal_heat_source: None,
        });
        let model = SimulationModel::from_typed(typed);

        let state = initialize_heat_balance_state(&model, 20.0)?;
        assert_eq!(state.construction_cache_entry_count, 1);

        let plan = build_execution_plan(&model);
        let calc_inside =
            stage_with_kind(&plan.stages, ExecutionStageKind::CalcHeatBalanceInsideSurf);
        assert_eq!(
            calc_inside.prebound.construction_ids,
            vec![ConstructionId(0)]
        );
        assert_eq!(
            calc_inside.prebound.surface_ids.len(),
            model.typed.surfaces.len()
        );

        Ok(())
    }

    #[test]
    fn construction_thermal_cache_indexes_sparse_reordered_and_filtered_ids()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.materials[0].id = MaterialId(10);
        typed
            .materials
            .push(spectral_average_window_material(MaterialId(99), "Clear Glass"));
        typed.constructions[0].id = ConstructionId(42);
        typed.constructions[0].outside_layer = Some(MaterialId(10));
        typed.constructions[0].layers = vec![MaterialId(10)];
        for surface in &mut typed.surfaces {
            surface.construction = ConstructionId(42);
        }
        typed.constructions.insert(
            0,
            Construction {
                id: ConstructionId(7),
                name: NormalizedName::new("Filtered Window"),
                kind: ConstructionKind::Fenestration,
                outside_layer: Some(MaterialId(99)),
                layers: vec![MaterialId(99)],
                thermochromic_master: None,
                ground_factor: None,
                air_boundary: None,
                complex_fenestration: None,
                internal_heat_source: None,
            },
        );

        let cache = crate::heat_balance::surface_manager::ConstructionThermalDataCache::build(
            &typed,
            &BTreeMap::new(),
        )?;
        assert_eq!(cache.len(), 1);
        let opaque_data = cache.data_for_surface(&typed.surfaces[0])?;
        assert_eq!(opaque_data.cache_index, 0);
        assert_eq!(opaque_data.construction_id, ConstructionId(42));

        let mut filtered_surface = typed.surfaces[0].clone();
        filtered_surface.construction = ConstructionId(7);
        assert!(matches!(
            cache.data_for_surface(&filtered_surface),
            Err(RuntimeError::MissingConstruction { .. })
        ));

        let mut missing_material = cube_model();
        missing_material.constructions[0].outside_layer = Some(MaterialId(99));
        missing_material.constructions[0].layers = vec![MaterialId(99)];
        assert!(matches!(
            crate::heat_balance::surface_manager::ConstructionThermalDataCache::build(
                &missing_material,
                &BTreeMap::new(),
            ),
            Err(RuntimeError::MissingMaterial { construction_name })
                if construction_name == "WALL"
        ));
        Ok(())
    }

    #[test]
    fn opaque_runtime_rejects_surface_using_fenestration_construction() {
        let mut typed = cube_model();
        typed.constructions[0].kind = ConstructionKind::Fenestration;
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);
        let calc_inside =
            stage_with_kind(&plan.stages, ExecutionStageKind::CalcHeatBalanceInsideSurf);
        assert!(calc_inside.prebound.construction_ids.is_empty());
        assert!(calc_inside.prebound.surface_ids.is_empty());

        let error = initialize_heat_balance_state(&model, 20.0)
            .expect_err("fenestration construction must not enter opaque heat balance");
        assert!(matches!(
            &error,
            RuntimeError::UnsupportedConstructionForOpaqueHeatBalance {
                construction_name,
                construction_kind: ConstructionKind::Fenestration,
                ..
            } if construction_name == "WALL"
        ));
        assert!(error.to_string().contains(
            "references fenestration construction WALL, which the opaque heat-balance runtime cannot consume"
        ));
    }

    #[test]
    fn opaque_runtime_filters_and_rejects_air_boundary_construction() {
        let mut typed = cube_model();
        typed.constructions[0].kind = ConstructionKind::AirBoundary;
        typed.constructions[0].outside_layer = None;
        typed.constructions[0].layers.clear();
        typed.constructions[0].air_boundary = Some(ConstructionAirBoundary {
            air_exchange: AirBoundaryAirExchange::None,
        });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);
        let calc_inside =
            stage_with_kind(&plan.stages, ExecutionStageKind::CalcHeatBalanceInsideSurf);
        assert!(calc_inside.prebound.construction_ids.is_empty());
        assert!(calc_inside.prebound.surface_ids.is_empty());

        let error = initialize_heat_balance_state(&model, 20.0)
            .expect_err("air boundary must not enter opaque heat balance");
        assert!(matches!(
            &error,
            RuntimeError::UnsupportedConstructionForOpaqueHeatBalance {
                construction_name,
                construction_kind: ConstructionKind::AirBoundary,
                ..
            } if construction_name == "WALL"
        ));
        assert!(error.to_string().contains(
            "references air_boundary construction WALL, which the opaque heat-balance runtime cannot consume"
        ));
    }

    #[test]
    fn opaque_runtime_filters_and_rejects_ground_factor_construction() {
        let mut typed = cube_model();
        typed.constructions[0].ground_factor =
            Some(ConstructionGroundFactor::FfactorGroundFloor {
                f_factor_w_per_m_k: 0.5,
                area_m2: 100.0,
                perimeter_exposed_m: 20.0,
                effective_thermal_resistance_m2_k_per_w: 9.835,
                insulation_thermal_resistance_m2_k_per_w: 9.758_076_923_076_923,
            });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);
        let calc_inside =
            stage_with_kind(&plan.stages, ExecutionStageKind::CalcHeatBalanceInsideSurf);
        assert!(calc_inside.prebound.construction_ids.is_empty());
        assert!(calc_inside.prebound.surface_ids.is_empty());

        let error = initialize_heat_balance_state(&model, 20.0)
            .expect_err("ground-factor construction must not enter opaque heat balance");
        assert!(matches!(
            &error,
            RuntimeError::UnsupportedConstructionForOpaqueHeatBalance {
                construction_name,
                construction_kind: ConstructionKind::Opaque,
                ..
            } if construction_name == "WALL"
        ));
    }

    #[test]
    fn opaque_runtime_rejects_fenestration_material_in_opaque_construction() {
        let mut typed = cube_model();
        typed.materials[0] =
            spectral_average_window_material(MaterialId(0), "Misclassified Glass");
        let model = SimulationModel::from_typed(typed);

        let error = initialize_heat_balance_state(&model, 20.0)
            .expect_err("fenestration material must not enter opaque heat balance");
        assert!(matches!(
            &error,
            RuntimeError::UnsupportedMaterialForOpaqueHeatBalance {
                construction_name,
                material_name,
                material_family: ep_model::MaterialFamily::Fenestration,
            } if construction_name == "WALL" && material_name == "MISCLASSIFIED GLASS"
        ));
        assert!(error.to_string().contains(
            "opaque construction WALL contains fenestration material MISCLASSIFIED GLASS"
        ));
    }

    #[test]
    fn heat_balance_state_uses_inside_layer_absorptance_for_interior_sources()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.materials.push(Material {
            id: MaterialId(1),
            name: NormalizedName::new("Inside Low Absorptance"),
            definition: ep_model::MaterialDefinition::NoMass(ep_model::NoMassMaterial {
                roughness: MaterialSurfaceRoughness::Smooth,
                thermal_resistance_m2_k_per_w: 1.0,
                surface: ep_model::OpaqueSurfaceProperties {
                    thermal_absorptance: 0.2,
                    solar_absorptance: 0.2,
                    visible_absorptance: 0.2,
                },
            }),
        });
        typed.materials.push(Material {
            id: MaterialId(2),
            name: NormalizedName::new("Inside High Absorptance"),
            definition: ep_model::MaterialDefinition::NoMass(ep_model::NoMassMaterial {
                roughness: MaterialSurfaceRoughness::Smooth,
                thermal_resistance_m2_k_per_w: 1.0,
                surface: ep_model::OpaqueSurfaceProperties {
                    thermal_absorptance: 0.8,
                    solar_absorptance: 0.8,
                    visible_absorptance: 0.8,
                },
            }),
        });
        typed.constructions[0].layers = vec![MaterialId(0), MaterialId(1)];
        typed.constructions.push(Construction {
            id: ConstructionId(1),
            name: NormalizedName::new("High Inside Wall"),
            kind: ConstructionKind::Opaque,
            outside_layer: Some(MaterialId(0)),
            layers: vec![MaterialId(0), MaterialId(2)],
            thermochromic_master: None,
            ground_factor: None,
            air_boundary: None,
            complex_fenestration: None,
            internal_heat_source: None,
        });
        typed.surfaces[0].construction = ConstructionId(1);
        typed.other_equipment[0].fraction_radiant = 0.25;
        let model = SimulationModel::from_typed(typed);
        let state = initialize_heat_balance_state(&model, 20.0)?;

        let high_inside = &state.surfaces[0];
        assert_eq!(high_inside.thermal_absorptance, 0.9);
        assert_eq!(high_inside.inside_thermal_absorptance, 0.8);
        let low_inside = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_id != high_inside.surface_id)
            .ok_or_else(|| std::io::Error::other("missing low-inside surface"))?;
        assert_eq!(low_inside.thermal_absorptance, 0.9);
        assert_eq!(low_inside.inside_thermal_absorptance, 0.2);

        let denominator = 0.8 + 5.0 * 0.2;
        let multiplier = 3.0 / denominator;
        assert!(
            (high_inside.inside_radiant_internal_gain_w_per_m2 - multiplier * 0.8).abs() < 1.0e-12
        );
        assert!(
            (low_inside.inside_radiant_internal_gain_w_per_m2 - multiplier * 0.2).abs() < 1.0e-12
        );

        Ok(())
    }

    #[test]
    fn energyplus_zone_air_temperature_coefficients_match_predictor_terms() {
        let coefficients = energyplus_zone_air_temperature_coefficients(
            18.456,
            369.12,
            2.0,
            12.0,
            3.0,
            45.0,
            1207.2,
            600.0,
            [20.0, 19.0, 18.0],
        );

        assert!((coefficients.temp_dependent_coefficient_w_per_k - 21.456).abs() < 1.0e-12);
        assert!((coefficients.temp_independent_coefficient_w - 424.12).abs() < 1.0e-12);
        assert!((coefficients.air_power_cap_w_per_k - 2.012).abs() < 1.0e-12);
        let expected_history = 2.012 * (3.0 * 20.0 - 1.5 * 19.0 + (1.0 / 3.0) * 18.0);
        assert!((coefficients.third_order_history_term_w - expected_history).abs() < 1.0e-12);
        assert!(
            (coefficients.third_order_temp_dependent_load_w_per_k
                - ((11.0 / 6.0) * 2.012 + 21.456))
                .abs()
                < 1.0e-12
        );
        assert!(
            (coefficients.third_order_temp_independent_load_w - (expected_history + 424.12)).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn energyplus_third_order_zone_air_temperature_matches_predictor_branch() {
        let temperature = energyplus_third_order_zone_air_temperature_c(
            20.0,
            424.12,
            21.456,
            1207.2,
            600.0,
            [20.0, 19.0, 18.0],
        );
        let air_power_cap = 1207.2 / 600.0;
        let history_term = air_power_cap * (3.0 * 20.0 - 1.5 * 19.0 + (1.0 / 3.0) * 18.0);
        let expected = (424.12 + history_term) / ((11.0 / 6.0) * air_power_cap + 21.456);
        assert!((temperature - expected).abs() < 1.0e-12);

        let fallback =
            energyplus_third_order_zone_air_temperature_c(20.0, 1.0, 0.0, 0.0, 600.0, [20.0; 3]);
        assert_eq!(fallback, 20.0);
    }

    #[test]
    fn energyplus_analytical_zone_air_temperature_matches_predictor_branch() {
        let zero_dependency =
            energyplus_analytical_zone_air_temperature_c(20.0, 12.0, 0.0, 1207.2, 600.0);
        assert!((zero_dependency - (20.0 + 12.0 * 600.0 / 1207.2)).abs() < 1.0e-12);

        let temperature =
            energyplus_analytical_zone_air_temperature_c(20.0, 72.0, 6.0, 1207.2, 600.0);
        let expected = 12.0 + (20.0 - 12.0) * (-6.0 * 600.0 / 1207.2_f64).exp();
        assert!((temperature - expected).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_tarp_natural_convection_matches_ashrae_branches() {
        let vertical = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(28.0, 20.0, 0.0);
        assert!((vertical - 2.62).abs() < 1.0e-12);

        let unstable_delta = 2.0_f64.powf(1.0 / 3.0);
        let unstable = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(22.0, 20.0, 1.0);
        let expected_unstable = 9.482 * unstable_delta / (7.238 - 1.0);
        assert!((unstable - expected_unstable).abs() < 1.0e-12);
        assert_eq!(
            energyplus_ashrae_tarp_natural_convection_branch(22.0, 20.0, 1.0).id(),
            "unstable-horizontal-or-tilt"
        );

        let stable = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(22.0, 20.0, -1.0);
        let expected_stable = 1.810 * unstable_delta / (1.382 + 1.0);
        assert!((stable - expected_stable).abs() < 1.0e-12);
        assert_eq!(
            energyplus_ashrae_tarp_natural_convection_branch(22.0, 20.0, -1.0).id(),
            "stable-horizontal-or-tilt"
        );

        let zero_delta = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(20.0, 20.0, 1.0);
        assert_eq!(zero_delta, 0.0);
        assert_eq!(
            energyplus_ashrae_tarp_natural_convection_branch(28.0, 20.0, 0.0).id(),
            "vertical-wall"
        );
    }

    #[test]
    fn energyplus_tarp_inside_convection_uses_surface_orientation_and_limits()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state(&model, 20.0)?;
        let floor = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        let roof = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        let wall = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall surface"))?;

        let delta_term = 2.0_f64.powf(1.0 / 3.0);
        let floor_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(floor, 22.0, 20.0);
        let expected_floor = 9.482 * delta_term / (7.238 - 1.0);
        assert!((floor_coefficient - expected_floor).abs() < 1.0e-12);
        assert_eq!(
            energyplus_tarp_inside_convection_branch_id(floor, 22.0, 20.0),
            "unstable-horizontal-or-tilt"
        );

        let roof_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(roof, 22.0, 20.0);
        let expected_roof = 1.810 * delta_term / (1.382 + 1.0);
        assert!((roof_coefficient - expected_roof).abs() < 1.0e-12);
        assert_eq!(
            energyplus_tarp_inside_convection_branch_id(roof, 22.0, 20.0),
            "stable-horizontal-or-tilt"
        );

        let wall_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(wall, 22.0, 20.0);
        let expected_wall = 1.31 * delta_term;
        assert!((wall_coefficient - expected_wall).abs() < 1.0e-12);
        assert_eq!(
            energyplus_tarp_inside_convection_branch_id(wall, 22.0, 20.0),
            "vertical-wall"
        );

        let zero_delta_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(floor, 20.0, 20.0);
        assert_eq!(zero_delta_coefficient, 0.1);

        Ok(())
    }

    #[test]
    fn energyplus_doe2_outside_convection_uses_wind_side_and_roughness()
    -> Result<(), Box<dyn std::error::Error>> {
        let windward = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            180.0,
            4.0,
            MaterialSurfaceRoughness::MediumRough,
        );
        let leeward = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            0.0,
            4.0,
            MaterialSurfaceRoughness::MediumRough,
        );
        let smoother = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            180.0,
            4.0,
            MaterialSurfaceRoughness::VerySmooth,
        );

        assert!((windward - 16.031846262998357).abs() < 1.0e-12);
        assert!((leeward - 11.929263692153699).abs() < 1.0e-12);
        assert!(windward > leeward);
        assert!(smoother < windward);

        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let state = initialize_heat_balance_state(&model, 20.0)?;
        let wall_state = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "WALL Y1")
            .ok_or_else(|| std::io::Error::other("missing wall surface state"))?;
        let wall = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "WALL Y1")
            .ok_or_else(|| std::io::Error::other("missing wall surface"))?;
        assert_eq!(
            energyplus_outside_convection_branch_id(wall_state, Some(wall), 180.0, true),
            "doe2-windward"
        );
        assert_eq!(
            energyplus_outside_convection_branch_id(wall_state, Some(wall), 0.0, true),
            "doe2-leeward"
        );
        assert_eq!(
            energyplus_outside_convection_branch_id(wall_state, Some(wall), 180.0, false),
            "simple-combined"
        );

        Ok(())
    }

    #[test]
    fn energyplus_surface_wind_speed_uses_terrain_and_centroid_height() {
        let typed = cube_model();
        let roof = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .expect("roof test surface");
        let expected_weather_mod = (270.0_f64 / 10.0).powf(0.14);
        let roof_height_m =
            roof.vertices.iter().map(|vertex| vertex.z_m).sum::<f64>() / roof.vertices.len() as f64;
        let expected_roof_wind = 4.0 * expected_weather_mod * (roof_height_m / 370.0).powf(0.22);

        let expected_roof_air_temperature = 20.0
            - 0.0065
                * (roof_height_m - ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M);
        assert!(
            (energyplus_surface_outdoor_air_temperature_c(roof, 20.0)
                - expected_roof_air_temperature)
                .abs()
                < 1.0e-12
        );

        assert!(
            (energyplus_surface_outside_wind_speed_m_per_s(roof, Terrain::Suburbs, 4.0)
                - expected_roof_wind)
                .abs()
                < 1.0e-12
        );

        let mut no_wind_roof = roof.clone();
        no_wind_roof.wind_exposure = WindExposure::NoWind;
        assert_eq!(
            energyplus_surface_outside_wind_speed_m_per_s(&no_wind_roof, Terrain::Suburbs, 4.0),
            0.0
        );
    }

    #[test]
    fn surface_ctf_history_terms_update_flux_constants() -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.outside_face_temperature_c = 10.0;
        surface.ctf.cross_history_w_per_m2_k = vec![0.2, 0.1];
        surface.ctf.inside_history_w_per_m2_k = vec![0.3, 0.05];
        surface.ctf.outside_history_w_per_m2_k = vec![0.4, 0.2];
        surface.ctf.flux_history = vec![0.5, 0.25];
        surface.ctf.outside_temperature_history_c = vec![8.0, 7.0];
        surface.ctf.inside_temperature_history_c = vec![18.0, 17.0];
        surface.ctf.inside_flux_history_w_per_m2 = vec![1.2, 0.8];
        surface.ctf.outside_flux_history_w_per_m2 = vec![-0.4, -0.2];

        update_surface_ctf_history_constants(surface);

        assert!((surface.ctf.const_in_part_w_per_m2 - (-3.15)).abs() < 1.0e-12);
        assert!((surface.ctf.const_out_part_w_per_m2 - (-0.95)).abs() < 1.0e-12);

        let slot_samples = surface_ctf_history_slot_samples(surface);
        assert_eq!(slot_samples.len(), 2);
        let slot = &slot_samples[0];
        assert_eq!(slot.slot_index, 1);
        assert_eq!(slot_samples[1].slot_index, 2);
        let inside_slot_sum = slot_samples
            .iter()
            .map(|sample| sample.inside_total_term_w)
            .sum::<f64>();
        let outside_slot_sum = slot_samples
            .iter()
            .map(|sample| sample.outside_total_term_w)
            .sum::<f64>();
        assert!(
            (inside_slot_sum - surface.area_m2 * surface.ctf.const_in_part_w_per_m2).abs()
                < 1.0e-12
        );
        assert!(
            (outside_slot_sum + surface.area_m2 * surface.ctf.const_out_part_w_per_m2)
                .abs()
                < 1.0e-12
        );

        let inside_flux = surface_inside_conduction_flux_w_per_m2(surface);
        let outside_flux = surface_outside_conduction_flux_w_per_m2(surface);
        advance_surface_ctf_histories(surface);

        assert_eq!(surface.ctf.outside_temperature_history_c, vec![10.0, 8.0]);
        assert_eq!(surface.ctf.inside_temperature_history_c, vec![20.0, 18.0]);
        assert_eq!(
            surface.ctf.inside_flux_history_w_per_m2,
            vec![inside_flux, 1.2]
        );
        assert_eq!(
            surface.ctf.outside_flux_history_w_per_m2,
            vec![outside_flux, -0.4]
        );

        Ok(())
    }

    #[test]
    fn surface_ctf_conduction_report_signs_match_energyplus_storage_convention()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.outside_face_temperature_c = 10.0;
        surface.ctf.outside_0_w_per_m2_k = 0.7;
        surface.ctf.cross_0_w_per_m2_k = 0.2;
        surface.ctf.inside_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;
        surface.ctf.const_out_part_w_per_m2 = -0.3;

        let inside_flux = surface_inside_conduction_flux_w_per_m2(surface);
        let outside_ctf_flux = surface_outside_conduction_flux_w_per_m2(surface);
        let inside_rate = surface_inside_conduction_rate_w(surface);
        let outside_report_rate = surface_outside_conduction_rate_w(surface);
        let storage_rate = surface_heat_storage_rate_w(inside_rate, outside_report_rate);

        assert!((inside_rate - surface.area_m2 * inside_flux).abs() < 1.0e-12);
        assert!(
            (outside_report_rate + surface.area_m2 * outside_ctf_flux).abs() < 1.0e-12,
            "EnergyPlus flips Qout to SurfOpaqOutFaceCondFlux before reporting"
        );
        assert!((storage_rate + inside_rate + outside_report_rate).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_down_interpolate_three_history_values_matches_source_ratios() {
        assert_eq!(
            super::energyplus_down_interpolate_three_history_values(
                3600.0,
                1800.0,
                [12.0, 9.0, 3.0]
            ),
            [12.0, 10.5, 9.0]
        );
        assert_eq!(
            super::energyplus_down_interpolate_three_history_values(
                3600.0,
                1200.0,
                [12.0, 9.0, 3.0]
            ),
            [12.0, 11.0, 10.0]
        );
        assert_eq!(
            super::energyplus_down_interpolate_three_history_values(
                3600.0,
                900.0,
                [12.0, 8.0, 2.0]
            ),
            [12.0, 11.0, 10.0]
        );
    }

    #[test]
    fn heat_balance_state_applies_construction_ctf_coefficients()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            20.0,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 2,
                    outside_w_per_m2_k: -0.4,
                    cross_w_per_m2_k: 0.2,
                    inside_w_per_m2_k: -0.3,
                    flux: Some(-0.5),
                },
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

        let ctf = &state.surfaces[0].ctf;
        assert_eq!(ctf.outside_0_w_per_m2_k, 2.0);
        assert_eq!(ctf.cross_0_w_per_m2_k, 0.5);
        assert_eq!(ctf.inside_0_w_per_m2_k, 3.0);
        assert_eq!(ctf.flux_0, None);
        assert_eq!(ctf.outside_history_w_per_m2_k, vec![0.4, -0.4]);
        assert_eq!(ctf.cross_history_w_per_m2_k, vec![0.1, 0.2]);
        assert_eq!(ctf.inside_history_w_per_m2_k, vec![0.3, -0.3]);
        assert_eq!(ctf.flux_history, vec![0.5, -0.5]);
        assert_eq!(ctf.outside_temperature_history_c, vec![20.0, 20.0]);
        assert_eq!(ctf.inside_temperature_history_c, vec![20.0, 20.0]);
        assert_eq!(ctf.outside_flux_history_w_per_m2, vec![0.0, 0.0]);
        assert_eq!(ctf.inside_flux_history_w_per_m2, vec![0.0, 0.0]);

        Ok(())
    }

    #[test]
    fn heat_balance_state_orders_energyplus_ctf_history_indices_for_runtime_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            20.0,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 5,
                    outside_w_per_m2_k: -4.1142049e-08,
                    cross_w_per_m2_k: 1.5543709e-08,
                    inside_w_per_m2_k: -4.1142049e-08,
                    flux: Some(1.2297289e-11),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 4,
                    outside_w_per_m2_k: 0.00057884701,
                    cross_w_per_m2_k: 0.00022976293,
                    inside_w_per_m2_k: 0.00057884701,
                    flux: Some(-4.0580373e-07),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 3,
                    outside_w_per_m2_k: -0.33051123,
                    cross_w_per_m2_k: 0.091914804,
                    inside_w_per_m2_k: -0.33051123,
                    flux: Some(0.0006592243),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 2,
                    outside_w_per_m2_k: 12.566595,
                    cross_w_per_m2_k: 2.1743923,
                    inside_w_per_m2_k: 12.566595,
                    flux: Some(-0.058066613),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: -62.622544,
                    cross_w_per_m2_k: 4.7096437,
                    inside_w_per_m2_k: -62.622544,
                    flux: Some(0.60555731),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 58.08561,
                    cross_w_per_m2_k: 0.72354869,
                    inside_w_per_m2_k: 58.08561,
                    flux: None,
                },
            ],
        )?;

        let ctf = &state.surfaces[0].ctf;
        assert_eq!(ctf.outside_0_w_per_m2_k, 58.08561);
        assert_eq!(ctf.cross_0_w_per_m2_k, 0.72354869);
        assert_eq!(ctf.inside_0_w_per_m2_k, 58.08561);
        assert_eq!(ctf.flux_0, None);
        assert_eq!(
            ctf.outside_history_w_per_m2_k,
            vec![
                -62.622544,
                12.566595,
                -0.33051123,
                0.00057884701,
                -4.1142049e-08
            ]
        );
        assert_eq!(
            ctf.cross_history_w_per_m2_k,
            vec![
                4.7096437,
                2.1743923,
                0.091914804,
                0.00022976293,
                1.5543709e-08
            ]
        );
        assert_eq!(
            ctf.inside_history_w_per_m2_k,
            vec![
                -62.622544,
                12.566595,
                -0.33051123,
                0.00057884701,
                -4.1142049e-08
            ]
        );
        assert_eq!(
            ctf.flux_history,
            vec![
                0.60555731,
                -0.058066613,
                0.0006592243,
                -4.0580373e-07,
                1.2297289e-11
            ]
        );

        Ok(())
    }

    #[test]
    fn heat_balance_summary_captures_run_period_initial_ctf_history_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let simulation = simulate_heat_balance_zone_air_temperatures_internal(
            &model,
            &[5.0],
            None,
            None,
            HeatBalanceSimulationOptions::hourly_samples(1),
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

        let floor_initial_slots = simulation
            .summary
            .run_period_initial_ctf_history_slots
            .iter()
            .filter(|sample| sample.surface_name == "FLOOR")
            .collect::<Vec<_>>();
        assert_eq!(floor_initial_slots.len(), 1);
        assert_eq!(floor_initial_slots[0].slot_index, 1);
        assert!(floor_initial_slots[0].inside_total_term_w.is_finite());
        assert!(floor_initial_slots[0].outside_total_term_w.is_finite());

        let floor_first_sample_slots = simulation
            .summary
            .first_sample_ctf_history_slots
            .iter()
            .filter(|sample| sample.surface_name == "FLOOR")
            .collect::<Vec<_>>();
        assert_eq!(floor_first_sample_slots.len(), 1);
        assert_eq!(floor_first_sample_slots[0].slot_index, 1);
        assert!(floor_first_sample_slots[0].timestep_count > 0);

        let floor_hourly_slots = simulation
            .summary
            .hourly_ctf_history_slots
            .iter()
            .filter(|sample| sample.surface_name == "FLOOR")
            .collect::<Vec<_>>();
        assert_eq!(floor_hourly_slots.len(), 1);
        assert_eq!(floor_hourly_slots[0].sample_index, 0);
        assert_eq!(floor_hourly_slots[0].slot_index, 1);
        assert_eq!(
            floor_hourly_slots[0].inside_total_term_w,
            floor_first_sample_slots[0].inside_total_term_w
        );

        Ok(())
    }

    #[test]
    fn initial_ctf_history_seeding_uses_boundary_temperature_and_u_value()
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

        let surface = &state.surfaces[0];
        let expected_u_value = 1.0 / surface.thermal_resistance_m2_k_per_w;
        let expected_flux = expected_u_value * (5.0 - ENERGYPLUS_ZONE_INITIAL_TEMP_C);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![5.0]);
        assert_eq!(
            surface.ctf.inside_temperature_history_c,
            vec![ENERGYPLUS_ZONE_INITIAL_TEMP_C]
        );
        assert!((surface.ctf.outside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert!((surface.ctf.inside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);

        Ok(())
    }
