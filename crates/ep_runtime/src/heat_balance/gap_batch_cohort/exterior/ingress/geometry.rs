//! Primary-only typed identities and the real full GetHeatBalanceInput route.
//! No Native expected state is read by this owner factory.
use super::*;
use ep_model::*;

fn required_number(row: &Value, key: &str) -> Result<f64> {
    numeric(row, key)?.ok_or_else(|| invalid(key).into())
}
fn required_alpha(row: &Value, key: &str) -> Result<String> {
    alpha(row, key)?.ok_or_else(|| invalid(key).into())
}
fn original_name(row: &Value, key: &str) -> Result<String> {
    Ok(text(member(row, key)?)?.to_owned())
}
fn one<'a>(factory: &'a Value, family: &str) -> Result<(&'a str, &'a Value)> {
    let rows = raw_family(factory, family)?;
    if rows.len() != 1 {
        return Err(invalid("one actual primary object").into());
    }
    Ok(rows[0])
}
fn primary_surface(raw: &Value) -> Result<OpaqueSurfaceProperties> {
    Ok(OpaqueSurfaceProperties {
        thermal_absorptance: numeric(raw, "thermal_absorptance")?.unwrap_or(0.9),
        solar_absorptance: numeric(raw, "solar_absorptance")?.unwrap_or(0.7),
        visible_absorptance: numeric(raw, "visible_absorptance")?.unwrap_or(0.7),
    })
}
fn primary_inside_convection(value: &str) -> Result<InsideSurfaceConvectionAlgorithm> {
    Ok(match value {
        "SIMPLE" => InsideSurfaceConvectionAlgorithm::Simple,
        "TARP" => InsideSurfaceConvectionAlgorithm::Tarp,
        "CEILINGDIFFUSER" => InsideSurfaceConvectionAlgorithm::CeilingDiffuser,
        "TROMBEWALL" => InsideSurfaceConvectionAlgorithm::TrombeWall,
        "ADAPTIVECONVECTIONALGORITHM" => {
            InsideSurfaceConvectionAlgorithm::AdaptiveConvectionAlgorithm
        }
        "ASTMC1340" => InsideSurfaceConvectionAlgorithm::AstmC1340,
        _ => return Err(invalid("primary inside convection enum").into()),
    })
}
fn primary_outside_convection(value: &str) -> Result<OutsideSurfaceConvectionAlgorithm> {
    Ok(match value {
        "SIMPLECOMBINED" => OutsideSurfaceConvectionAlgorithm::SimpleCombined,
        "TARP" => OutsideSurfaceConvectionAlgorithm::Tarp,
        "MOWITT" => OutsideSurfaceConvectionAlgorithm::MoWitt,
        "DOE-2" => OutsideSurfaceConvectionAlgorithm::Doe2,
        "ADAPTIVECONVECTIONALGORITHM" => {
            OutsideSurfaceConvectionAlgorithm::AdaptiveConvectionAlgorithm
        }
        _ => return Err(invalid("primary outside convection enum").into()),
    })
}
fn primary_model(
    factory: &Value,
    control: &SelectedHeatBalanceInputState,
    steps: i32,
) -> Result<TypedModel> {
    let mut model = identity_model(factory, control, steps)?;
    // Typed identity metadata must describe the same raw project/Zone operands
    // that the later real GetProjectControlData/GetZoneData calls consume.
    // The cold control constructor is not that project's authored selection.
    let inside_input = raw_family(factory, "SurfaceConvectionAlgorithm:Inside")?;
    let outside_input = raw_family(factory, "SurfaceConvectionAlgorithm:Outside")?;
    let inside_primary = inside_input
        .first()
        .map(|(_, row)| alpha(row, "algorithm"))
        .transpose()?
        .flatten();
    let outside_primary = outside_input
        .first()
        .map(|(_, row)| alpha(row, "algorithm"))
        .transpose()?
        .flatten();
    let inside_default = primary_inside_convection(inside_primary.as_deref().unwrap_or("TARP"))?;
    let outside_default =
        primary_outside_convection(outside_primary.as_deref().unwrap_or("DOE-2"))?;
    model.surface_convection_algorithms.inside = inside_input.first().map(|_| inside_default);
    model.surface_convection_algorithms.outside = outside_input.first().map(|_| outside_default);
    for (name, raw) in raw_family(factory, "Zone")? {
        let zone = model
            .zones
            .iter_mut()
            .find(|zone| zone.name == NormalizedName::new(name))
            .ok_or("primary Zone convection identity")?;
        zone.inside_convection_algorithm = match alpha(raw, "zone_inside_convection_algorithm")? {
            Some(value) => ZoneConvectionAlgorithm::Override(primary_inside_convection(&value)?),
            None => ZoneConvectionAlgorithm::Inherited(inside_default),
        };
        zone.outside_convection_algorithm = match alpha(raw, "zone_outside_convection_algorithm")? {
            Some(value) => ZoneConvectionAlgorithm::Override(primary_outside_convection(&value)?),
            None => ZoneConvectionAlgorithm::Inherited(outside_default),
        };
    }
    let (name, raw) = one(factory, "Building")?;
    model.building = Some(Building {
        name: NormalizedName::new(name),
        north_axis_deg: numeric(raw, "north_axis")?.unwrap_or(0.0),
        terrain: match required_alpha(raw, "terrain")?.as_str() {
            "CITY" => Terrain::City,
            "COUNTRY" => Terrain::Country,
            "OCEAN" => Terrain::Ocean,
            "SUBURBS" => Terrain::Suburbs,
            "URBAN" => Terrain::Urban,
            _ => return Err(invalid("primary terrain").into()),
        },
        loads_convergence_tolerance_w: numeric(raw, "loads_convergence_tolerance_value")?
            .unwrap_or(0.04),
        temperature_convergence_tolerance_delta_c: numeric(
            raw,
            "temperature_convergence_tolerance_value",
        )?
        .unwrap_or(0.4),
        solar_distribution: match required_alpha(raw, "solar_distribution")?.as_str() {
            "MINIMALSHADOWING" => SolarDistribution::MinimalShadowing,
            "FULLEXTERIOR" => SolarDistribution::FullExterior,
            "FULLINTERIORANDEXTERIOR" => SolarDistribution::FullInteriorAndExterior,
            _ => return Err(invalid("unrepresented primary solar distribution").into()),
        },
        maximum_number_of_warmup_days: required_number(raw, "maximum_number_of_warmup_days")?
            as u32,
        minimum_number_of_warmup_days: required_number(raw, "minimum_number_of_warmup_days")?
            as u32,
    });
    if let Some((name, raw)) = raw_family(factory, "Site:Location")?.first().copied() {
        model.site = Some(SiteLocation {
            name: NormalizedName::new(name),
            latitude_deg: required_number(raw, "latitude")?,
            longitude_deg: required_number(raw, "longitude")?,
            time_zone_hours: required_number(raw, "time_zone")?,
            elevation_m: required_number(raw, "elevation")?,
        });
    }
    let (_, ggr) = one(factory, "GlobalGeometryRules")?;
    if required_alpha(ggr, "starting_vertex_position")? != "UPPERLEFTCORNER"
        || required_alpha(ggr, "vertex_entry_direction")? != "COUNTERCLOCKWISE"
        || required_alpha(ggr, "coordinate_system")? != "WORLD"
    {
        return Err(invalid("actual supported raw World geometry primary").into());
    }
    model.global_geometry_rules = Some(GlobalGeometryRules::default());
    // Family-local ordering follows the actual material input getters. These
    // typed IDs are identity metadata, never assumed equal to Native ordinals.
    for family in ["Material", "Material:NoMass"] {
        for (name, raw) in raw_family(factory, family)? {
            let roughness =
                MaterialSurfaceRoughness::from_energyplus_name(&required_alpha(raw, "roughness")?)
                    .ok_or("primary roughness")?;
            let surface = primary_surface(raw)?;
            let definition = if family == "Material" {
                let thickness = required_number(raw, "thickness")?;
                let conductivity = required_number(raw, "conductivity")?;
                MaterialDefinition::Regular(RegularMaterial {
                    roughness,
                    thickness_m: thickness,
                    conductivity_w_per_m_k: conductivity,
                    density_kg_per_m3: required_number(raw, "density")?,
                    specific_heat_j_per_kg_k: required_number(raw, "specific_heat")?,
                    thermal_resistance_m2_k_per_w: thickness / conductivity,
                    surface,
                })
            } else {
                MaterialDefinition::NoMass(NoMassMaterial {
                    roughness,
                    thermal_resistance_m2_k_per_w: required_number(raw, "thermal_resistance")?,
                    surface,
                })
            };
            let id = MaterialId(u32::try_from(model.materials.len())?);
            model.ctf_report_names.materials.push((id, name.to_owned()));
            model.materials.push(Material {
                id,
                name: NormalizedName::new(name),
                definition,
            });
        }
    }
    for (name, raw) in raw_family(factory, "Construction")? {
        let mut layers = Vec::new();
        for i in 1..=10 {
            let key = if i == 1 {
                "outside_layer".to_owned()
            } else {
                format!("layer_{i}")
            };
            let Some(name) = alpha(raw, &key)? else {
                break;
            };
            layers.push(
                model
                    .materials
                    .iter()
                    .find(|m| m.name == NormalizedName::new(&name))
                    .ok_or("primary material reference")?
                    .id,
            );
        }
        let id = ConstructionId(u32::try_from(model.constructions.len())?);
        model
            .ctf_report_names
            .constructions
            .push((id, name.to_owned()));
        model.constructions.push(Construction {
            id,
            name: NormalizedName::new(name),
            kind: ConstructionKind::Opaque,
            outside_layer: layers.first().copied(),
            layers,
            thermochromic_master: None,
            ground_factor: None,
            air_boundary: None,
            complex_fenestration: None,
            window_equivalent_layer: None,
            internal_heat_source: None,
        });
    }
    for (name, raw) in raw_family(factory, "BuildingSurface:Detailed")? {
        let zone_name = required_alpha(raw, "zone_name")?;
        let zone = model
            .zones
            .iter()
            .find(|z| z.name == NormalizedName::new(&zone_name))
            .ok_or("primary Surface Zone reference")?;
        let construction_name = required_alpha(raw, "construction_name")?;
        let construction = model
            .constructions
            .iter()
            .find(|c| c.name == NormalizedName::new(&construction_name))
            .ok_or("primary Surface Construction reference")?
            .id;
        let vertices = array(member(raw, "vertices")?)?
            .iter()
            .map(|v| {
                Ok(Point3 {
                    x_m: required_number(v, "vertex_x_coordinate")?,
                    y_m: required_number(v, "vertex_y_coordinate")?,
                    z_m: required_number(v, "vertex_z_coordinate")?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let id = SurfaceId(u32::try_from(model.surfaces.len())?);
        model.surfaces.push(Surface {
            id,
            name: NormalizedName::new(name),
            surface_type: match required_alpha(raw, "surface_type")?.as_str() {
                "WALL" => SurfaceType::Wall,
                "FLOOR" => SurfaceType::Floor,
                "ROOF" => SurfaceType::Roof,
                "CEILING" => SurfaceType::Ceiling,
                _ => return Err(invalid("primary opaque type").into()),
            },
            construction,
            zone: zone.id,
            space: *zone
                .spaces
                .first()
                .ok_or("genuine default Space identity")?,
            outside_boundary_condition: match required_alpha(raw, "outside_boundary_condition")?
                .as_str()
            {
                "OUTDOORS" => OutsideBoundaryCondition::Outdoors,
                "ADIABATIC" => OutsideBoundaryCondition::Adiabatic,
                "GROUND" => OutsideBoundaryCondition::Ground,
                _ => return Err(invalid("primary opaque boundary").into()),
            },
            outside_boundary_condition_object: alpha(raw, "outside_boundary_condition_object")?
                .as_deref()
                .map(NormalizedName::new),
            sun_exposure: match required_alpha(raw, "sun_exposure")?.as_str() {
                "SUNEXPOSED" => SunExposure::SunExposed,
                "NOSUN" => SunExposure::NoSun,
                _ => return Err(invalid("primary sun exposure").into()),
            },
            wind_exposure: match required_alpha(raw, "wind_exposure")?.as_str() {
                "WINDEXPOSED" => WindExposure::WindExposed,
                "NOWIND" => WindExposure::NoWind,
                _ => return Err(invalid("primary wind exposure").into()),
            },
            view_factor_to_ground: match raw.get("view_factor_to_ground") {
                Some(Value::String(s)) if s.eq_ignore_ascii_case("Autocalculate") => {
                    AutoOrNumber::AutoCalculate
                }
                _ => AutoOrNumber::Value(required_number(raw, "view_factor_to_ground")?),
            },
            vertices,
            computed_geometry: None,
        });
    }
    Ok(model)
}

fn surface_inputs(
    factory: &Value,
) -> Result<hb::selected_heat_balance_input::surface::SelectedSurfaceInput> {
    use hb::selected_heat_balance_input::surface::{OpaqueSurfaceInput, SelectedSurfaceInput};
    let geometry_rules = project_objects(factory, &["GlobalGeometryRules"])?
        .remove("GlobalGeometryRules")
        .unwrap_or_default();
    let mut building_surfaces = Vec::new();
    for (name, raw) in raw_family(factory, "BuildingSurface:Detailed")? {
        let mut primary = raw.clone();
        object(&primary)?;
        let vertices = array(member(&primary, "vertices")?)?
            .iter()
            .map(|v| {
                Ok([
                    required_number(v, "vertex_x_coordinate")?,
                    required_number(v, "vertex_y_coordinate")?,
                    required_number(v, "vertex_z_coordinate")?,
                ])
            })
            .collect::<Result<_>>()?;
        primary
            .as_object_mut()
            .ok_or("primary object")?
            .remove("vertices");
        let object = ProjectInputObject {
            object_name: name.into(),
            idf_max_fields: Some(count(member(&primary, "idf_max_fields")?)?),
            fields: object(&primary)?
                .iter()
                .filter(|(k, _)| !k.starts_with("idf_"))
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        match v {
                            Value::String(s) => ProjectInputValue::String(s.clone()),
                            Value::Number(n) => {
                                ProjectInputValue::Number(n.as_f64().ok_or("primary number")?)
                            }
                            _ => return Err(invalid("primary scalar Surface input").into()),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
        };
        building_surfaces.push(OpaqueSurfaceInput {
            object,
            idf_max_extensible_fields: Some(count(member(&primary, "idf_max_extensible_fields")?)?),
            vertices,
        });
    }
    Ok(SelectedSurfaceInput {
        geometry_rules,
        building_surfaces,
    })
}

impl GainFactory {
    pub(crate) fn new_geometry(
        ingress: &Value,
        name: &str,
        mtd_path: &std::path::Path,
        read_location: bool,
    ) -> Result<Self> {
        if ingress["schema"] != "src06-Exterior-primary-Source-ingress.v1" {
            return Err(invalid("primary-only ingress schema").into());
        }
        let factory = member(member(ingress, "factories")?, name)?;
        // Exclude the exploratory windows/shading input by primary scope, before
        // any caller/getter is run. This is not an output-based filtering rule.
        for family in [
            "FenestrationSurface:Detailed",
            "Shading:Zone:Detailed",
            "Shading:Building:Detailed",
            "Shading:Site:Detailed",
            "Shading:Overhang",
            "Shading:Overhang:Projection",
            "Output:Variable",
            "Output:Meter",
        ] {
            if !raw_family(factory, family)?.is_empty() {
                return Err(
                    invalid("outside finite no-window/no-attached-shading unit scope").into(),
                );
            }
        }
        let mut input = make_input(member(ingress, "definitions")?, factory)?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input)?;
        let mut globals = SelectedHeatBalanceGlobals::default();
        let mut weather = crate::weather::day::WeatherDayState::default();
        let mut project = SelectedProjectData::default();
        let mut startup = hb::selected_simulation_startup::SelectedSimulationStartup::default();
        let mut prescan = SourceOutputPreScan::default();
        let mut errors = Vec::new();
        let mut console = Vec::new();
        let mut error_state = error_state();
        let mut eio = Vec::new();
        {
            let mut backend = error_backend(&mut error_state, &mut errors, &mut console);
            hb::source_output_prescan::post_input_processing(
                &input,
                &mut shortcut,
                &SourceReportingPreScanInput::default(),
                &mut prescan,
                false,
                &mut backend,
            )?;
            let raw = SelectedProjectDataInput {
                source_is_epjson: false,
                objects: project_objects(
                    factory,
                    &[
                        "Version",
                        "HeatBalanceAlgorithm",
                        "Timestep",
                        "SimulationControl",
                    ],
                )?,
                diagnostics: Vec::new(),
            };
            project.get_project_data(
                &raw,
                &mut ProjectDataOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    globals: &mut globals,
                    weather: &mut weather,
                    hvac: &mut startup.hvac_globals,
                    heat_balance_current_module_object: &mut String::new(),
                },
                &mut backend,
                &mut eio,
            )?;
        }
        startup
            .construction_bounds
            .set_preconstruction_input_parameters(&input)?;
        let mut files = Files::default();
        let mut schedules = SelectedScheduleManager::default();
        schedules.init_constant_state(0)?;
        let steps = usize::try_from(weather.global.time_steps_in_hour)?;
        {
            let mut backend = error_backend(&mut error_state, &mut errors, &mut console);
            schedules.process_schedule_input_with_backend(
                &mut input,
                &schedule_inputs(factory)?,
                steps,
                &mut files,
                &mut backend,
            )?;
        }
        let model = primary_model(
            factory,
            &SelectedHeatBalanceInputState::default(),
            weather.global.time_steps_in_hour,
        )?;
        let inventory = object(member(factory, "raw")?)?
            .iter()
            .map(|(k, v)| Ok((k.as_str(), object(v)?.len())))
            .collect::<Result<Vec<_>>>()?;
        let radiation_input =
            OrdinaryRadiationInput::from_admitted_inventory(inventory.clone(), false)?;
        let equipment_input =
            OrdinaryEquipmentInput::from_source_family_inventory(inventory, steps)?;
        let mut reports = CtfInitializationReports::default();
        reports
            .registry
            .set_predefined_tables(globals.do_zone_sizing, globals.do_system_sizing)?;
        let families = object(member(factory, "raw")?)?
            .keys()
            .filter(|k| k.as_str() != "BuildingSurface:Detailed" && !k.starts_with("Schedule:"))
            .map(String::as_str)
            .collect::<Vec<_>>();
        reports.heat_balance_input = Some(SelectedHeatBalanceInput {
            source_is_epjson: false,
            surface: surface_inputs(factory)?,
            objects: project_objects(factory, &families)?,
        });
        reports.selected_material_input = Some(hb::selected_material_data::SelectedMaterialInput {
            source_is_epjson: false,
            regular: project_objects(factory, &["Material"])?
                .remove("Material")
                .unwrap_or_default(),
            no_mass: project_objects(factory, &["Material:NoMass"])?
                .remove("Material:NoMass")
                .unwrap_or_default(),
            air_gaps: project_objects(factory, &["Material:AirGap"])?
                .remove("Material:AirGap")
                .unwrap_or_default(),
        });
        {
            let mut backend = error_backend(&mut error_state, &mut errors, &mut console);
            for kind in [
                hb::source_output_registration::TimeStepKind::Zone,
                hb::source_output_registration::TimeStepKind::System,
            ] {
                reports
                    .output_registration
                    .time_pointers
                    .setup_time_pointer(kind, &mut backend)?;
            }
        }
        reports.selected_input_processor = Some(input);
        reports.input_processor_shortcut = shortcut;
        reports.selected_heat_balance_globals = Some(globals);
        reports.selected_project_data = Some(project);
        reports.selected_simulation_startup = Some(startup);
        reports.selected_schedule_manager = Some(schedules);
        reports.output_prescan = prescan;
        reports.facility_electric_service =
            Some(hb::facility_electric_service::FacilityElectricService::new());
        let mut branch = hb::startup_integrity::SourceBranchNodeConnections::default();
        let mut mtd = hb::source_output_registration::SourceMtdFileHost::new(mtd_path);
        // Literal actual Native caller phase. These are call inputs, not
        // successful getter/readiness witnesses or expected exit fields.
        weather.global.begin_sim_flag = true;
        weather.global.begin_envrn_flag = true;
        if read_location {
            let mut error = false;
            let mut backend = error_backend(&mut error_state, &mut errors, &mut console);
            weather.get_source_location_info(
                reports
                    .heat_balance_input
                    .as_ref()
                    .ok_or("actual HB raw Site input")?,
                reports
                    .selected_input_processor
                    .as_mut()
                    .ok_or("same Site IP")?,
                &mut reports.input_processor_shortcut,
                &mut backend,
                &mut error,
            )?;
            if error {
                return Err(invalid("real Location getter reported errors").into());
            }
            weather.check_source_location_validity(&mut backend)?;
        }
        {
            let mut context = CtfInitializationContext {
                reports: &mut reports,
                eio: &mut eio,
                backend: error_backend(&mut error_state, &mut errors, &mut console),
            };
            let result = context.get_source_heat_balance_input(
                &model,
                &mut weather.environment,
                weather.global.minutes_in_time_step,
                &mut mtd,
                equipment_input,
                radiation_input,
                &mut branch,
            );
            if let Err(error) = result {
                // Passive failure-only shapes/flags. No uninitialized numeric
                // array values, expected data, or scientific writes are read.
                let geometry = &context.reports.surface_geometry_input;
                let classes = geometry.final_surfaces.as_deref().map(|rows| {
                    rows.iter()
                        .map(|row| (row.row.name.as_str(), row.row.class))
                        .collect::<Vec<_>>()
                });
                let solar = context.reports.ordinary_solar_radiation.as_ref();
                let solar_f_lengths = solar
                    .and_then(|owner| owner.enclosure_storage())
                    .map(|rows| rows.iter().map(|row| row.f.len()).collect::<Vec<_>>());
                let radiant_f_lengths = solar.map(|owner| {
                    owner
                        .radiant_enclosures()
                        .iter()
                        .map(|row| row.f.as_ref().map(Vec::len))
                        .collect::<Vec<_>>()
                });
                let user_f = context
                    .reports
                    .selected_input_processor
                    .as_ref()
                    .map(|input| {
                        input.get_num_objects_found("ZoneProperty:UserViewFactors:BySurfaceName")
                    });
                let shelf = context
                    .reports
                    .selected_heat_balance_globals
                    .as_ref()
                    .map(|globals| globals.any_inside_shelf);
                return Err(invalid(&format!(
                    "GeometryFactory case={name}, phase=GetHeatBalanceInput: {error:?}; actual UseRepresentativeSurfaceCalculations={}, userF={user_f:?}, anyInsideShelf={shelf:?}, solar_initialized={:?}, Solar_F_lengths={solar_f_lengths:?}, Radiant_F_lengths={radiant_f_lengths:?}, actual_Surface_classes={classes:?}",
                    geometry.use_representative_surface_calculations,
                    solar.map(|owner| owner.is_initialized()),
                )).into());
            }
        }
        let globals = reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or("same real globals")?;
        let mut air = SourceAirHeatBalance::default();
        air.allocate_source_arrays(globals)?;
        let mut radiation = reports
            .ordinary_solar_radiation
            .take()
            .ok_or("genuine Solar getter owner")?;
        radiation.allocate_source_solar_enclosure_arrays()?;
        let geometry = std::mem::take(&mut reports.surface_geometry_input);
        Ok(Self {
            reports,
            weather,
            air,
            modules: SourceGainModules::default(),
            radiation,
            geometry,
            arrays: SourceSurfaceHeatBalanceArrays::default(),
            history: None,
            public_ctf: hb::ctf_public_initialization::OrderedCtfPublicInitialization::begin(
                true, 0,
            ),
            shading_stream: Default::default(),
            daylighting: Default::default(),
            eso: Vec::new(),
            error_state,
            errors,
            console,
            eio,
            files,
            model,
        })
    }
}
