    #[test]
    fn first_zone_simulation_writes_zone_temperature_results()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());

        let simulation = simulate_first_zone_uncontrolled(
            &model,
            &[20.0, 20.0],
            FirstZoneSimulationOptions::hourly_samples(2),
        )?;

        assert_eq!(simulation.summary.zone_name, "ZONE ONE");
        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.volume_m3, 1.0);
        assert_eq!(simulation.summary.exterior_area_m2, 6.0);
        assert_eq!(simulation.summary.conductance_w_per_k, 6.0);
        assert_eq!(simulation.summary.internal_gain_w, 12.0);
        assert_eq!(simulation.results.sample_count(), 2);
        assert_eq!(simulation.results.series.len(), 2);
        assert_eq!(simulation.state.timestep_index, 12);
        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert!(zone_series.values[0] > 20.0);
        assert!(zone_series.values[1] >= zone_series.values[0]);

        Ok(())
    }

    fn cube_model() -> TypedModel {
        let mut model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 6,
            },
            ..TypedModel::default()
        };
        model.materials.push(Material {
            id: MaterialId(0),
            name: NormalizedName::new("R1"),
            kind: MaterialKind::NoMass,
            roughness: Some(MaterialSurfaceRoughness::Rough),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.9),
            solar_absorptance: Some(0.75),
            visible_absorptance: Some(0.75),
        });
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0)],
        });
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("Always On"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });
        model.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        model.other_equipment.push(OtherEquipment {
            id: InternalGainId(0),
            name: NormalizedName::new("Plug Load"),
            fuel_type: NormalizedName::new("None"),
            zone: ZoneId(0),
            schedule: Some(ScheduleId(0)),
            design_level_calculation_method:
                OtherEquipmentDesignLevelCalculationMethod::EquipmentLevel,
            design_level_w: 12.0,
            power_per_floor_area_w_per_m2: 0.0,
            power_per_person_w: 0.0,
            fraction_latent: 0.0,
            fraction_radiant: 0.0,
            fraction_lost: 0.0,
            carbon_dioxide_generation_rate_m3_per_s_w: 0.0,
        });
        model.surfaces.extend(cube_surfaces());
        model
    }

    fn two_zone_interzone_model() -> TypedModel {
        let mut model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 1,
            },
            ..TypedModel::default()
        };
        model.materials.push(Material {
            id: MaterialId(0),
            name: NormalizedName::new("R1"),
            kind: MaterialKind::NoMass,
            roughness: Some(MaterialSurfaceRoughness::Rough),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.9),
            solar_absorptance: Some(0.75),
            visible_absorptance: Some(0.75),
        });
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0)],
        });
        model.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone A"),
            direction_of_relative_north_deg: 0.0,
            origin: point(0.0, 0.0, 0.0),
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::Value(1.0),
        });
        model.zones.push(Zone {
            id: ZoneId(1),
            name: NormalizedName::new("Zone B"),
            direction_of_relative_north_deg: 0.0,
            origin: point(1.0, 0.0, 0.0),
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::Value(1.0),
        });
        model.surfaces.push(interzone_surface(
            0,
            "A Wall",
            ZoneId(0),
            "B Wall",
            [
                point(1.0, 0.0, 0.0),
                point(1.0, 1.0, 0.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        ));
        model.surfaces.push(interzone_surface(
            1,
            "B Wall",
            ZoneId(1),
            "A Wall",
            [
                point(0.0, 0.0, 0.0),
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(0.0, 1.0, 0.0),
            ],
        ));
        model
    }

    fn cube_surfaces() -> Vec<Surface> {
        vec![
            surface(
                0,
                "Floor",
                SurfaceType::Floor,
                [
                    point(0.0, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(0.0, 1.0, 0.0),
                ],
            ),
            surface(
                1,
                "Roof",
                SurfaceType::Roof,
                [
                    point(0.0, 0.0, 1.0),
                    point(0.0, 1.0, 1.0),
                    point(1.0, 1.0, 1.0),
                    point(1.0, 0.0, 1.0),
                ],
            ),
            surface(
                2,
                "Wall X0",
                SurfaceType::Wall,
                [
                    point(0.0, 0.0, 0.0),
                    point(0.0, 1.0, 0.0),
                    point(0.0, 1.0, 1.0),
                    point(0.0, 0.0, 1.0),
                ],
            ),
            surface(
                3,
                "Wall X1",
                SurfaceType::Wall,
                [
                    point(1.0, 0.0, 0.0),
                    point(1.0, 0.0, 1.0),
                    point(1.0, 1.0, 1.0),
                    point(1.0, 1.0, 0.0),
                ],
            ),
            surface(
                4,
                "Wall Y0",
                SurfaceType::Wall,
                [
                    point(0.0, 0.0, 0.0),
                    point(0.0, 0.0, 1.0),
                    point(1.0, 0.0, 1.0),
                    point(1.0, 0.0, 0.0),
                ],
            ),
            surface(
                5,
                "Wall Y1",
                SurfaceType::Wall,
                [
                    point(0.0, 1.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(1.0, 1.0, 1.0),
                    point(0.0, 1.0, 1.0),
                ],
            ),
        ]
    }

    fn weather_record_with_precipitation(liquid_precipitation_depth_mm: f64) -> EpwRecord {
        EpwRecord {
            year: 2013,
            month: 9,
            day: 18,
            hour: 19,
            minute: 60,
            dry_bulb_c: 8.0,
            dew_point_c: 7.0,
            relative_humidity_percent: 93.0,
            atmospheric_pressure_pa: 81_800.0,
            horizontal_infrared_radiation_wh_per_m2: 330.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm,
        }
    }

    fn surface(id: u32, name: &str, surface_type: SurfaceType, vertices: [Point3; 4]) -> Surface {
        Surface {
            id: SurfaceId(id),
            name: NormalizedName::new(name),
            surface_type,
            construction: ConstructionId(0),
            zone: ZoneId(0),
            outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
            outside_boundary_condition_object: None,
            sun_exposure: ep_model::SunExposure::SunExposed,
            wind_exposure: ep_model::WindExposure::WindExposed,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: vertices.to_vec(),
        }
    }

    fn interzone_surface(
        id: u32,
        name: &str,
        zone: ZoneId,
        target_surface: &str,
        vertices: [Point3; 4],
    ) -> Surface {
        Surface {
            id: SurfaceId(id),
            name: NormalizedName::new(name),
            surface_type: SurfaceType::Wall,
            construction: ConstructionId(0),
            zone,
            outside_boundary_condition: OutsideBoundaryCondition::Surface,
            outside_boundary_condition_object: Some(NormalizedName::new(target_surface)),
            sun_exposure: ep_model::SunExposure::NoSun,
            wind_exposure: ep_model::WindExposure::NoWind,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: vertices.to_vec(),
        }
    }

    fn point(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
        Point3 { x_m, y_m, z_m }
    }
