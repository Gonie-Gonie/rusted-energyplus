use super::*;
use crate::{
    PartialRunPolicy, RunMode, RunOutputFormat, SupportStatus, TraceLevel, assess_support,
};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

fn input(scope: PortingScope) -> Value {
    let mut value = json!({
        "Version":{"Version":{"version_identifier":"26.1"}},
        "Building":{"Building":{}},
        "SimulationControl":{"Control":{"run_simulation_for_sizing_periods":"No"}},
        "GlobalGeometryRules":{"Geometry":{"starting_vertex_position":"UpperLeftCorner","vertex_entry_direction":"Counterclockwise","coordinate_system":"World"}},
        "RunPeriod":{"Period":{"begin_month":1,"begin_day_of_month":1,"begin_year":2013,"end_month":1,"end_day_of_month":1,"end_year":2013}},
        "Zone":{"Zone":{"volume":1,"floor_area":1}},
        "Material:NoMass":{"Opaque":{"roughness":"MediumRough","thermal_resistance":2}},
        "Construction":{"Opaque":{"outside_layer":"Opaque"}},
        "Schedule:Constant":{"On":{"hourly_value":1},"Type":{"hourly_value":4},"Heat":{"hourly_value":19},"Cool":{"hourly_value":25}},
    });
    let cube = [
        (
            "Floor",
            "Floor",
            [[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 0]],
        ),
        ("Roof", "Roof", [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]]),
        ("West", "Wall", [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0]]),
        ("East", "Wall", [[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]]),
        (
            "South",
            "Wall",
            [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]],
        ),
        (
            "North",
            "Wall",
            [[0, 1, 0], [0, 1, 1], [1, 1, 1], [1, 1, 0]],
        ),
    ];
    value["BuildingSurface:Detailed"] = json!(cube.into_iter().map(|(name, kind, points)| (name.to_string(),json!({
        "surface_type":kind,"construction_name":"Opaque","zone_name":"Zone",
        "outside_boundary_condition":"Outdoors","sun_exposure":"NoSun","wind_exposure":"NoWind",
        "vertices":points.into_iter().map(|point|json!({"vertex_x_coordinate":point[0],"vertex_y_coordinate":point[1],"vertex_z_coordinate":point[2]})).collect::<Vec<_>>(),
    }))).collect::<BTreeMap<_,_>>());
    if scope == PortingScope::B {
        value["ThermostatSetpoint:DualSetpoint"] = json!({"Dual":{"heating_setpoint_temperature_schedule_name":"Heat","cooling_setpoint_temperature_schedule_name":"Cool"}});
        value["ZoneControl:Thermostat"] = json!({"Thermostat":{"zone_or_zonelist_name":"Zone","control_type_schedule_name":"Type","control_1_object_type":"ThermostatSetpoint:DualSetpoint","control_1_name":"Dual"}});
        value["ZoneHVAC:IdealLoadsAirSystem"] = json!({"Ideal":{"zone_supply_air_node_name":"Supply","availability_schedule_name":"On","dehumidification_control_type":"None","humidification_control_type":"None","heating_limit":"NoLimit","cooling_limit":"NoLimit"}});
        value["ZoneHVAC:EquipmentList"] = json!({"Equipment":{"load_distribution_scheme":"SequentialLoad","equipment":[{"zone_equipment_object_type":"ZoneHVAC:IdealLoadsAirSystem","zone_equipment_name":"Ideal","zone_equipment_cooling_sequence":1,"zone_equipment_heating_or_no_load_sequence":1}]}});
        value["ZoneHVAC:EquipmentConnections"] = json!({"Connection":{"zone_name":"Zone","zone_conditioning_equipment_list_name":"Equipment","zone_air_inlet_node_or_nodelist_name":"Supply","zone_air_node_name":"Air","zone_return_air_node_or_nodelist_name":"Return"}});
    }
    value
}

fn single_day_weather_fixture() -> ep_runtime::weather::EpwWeatherFile {
    let mut weather_text = String::from(
        "LOCATION,Example\nDESIGN CONDITIONS\nTYPICAL/EXTREME PERIODS\nGROUND TEMPERATURES\nHOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\nCOMMENTS 1\nCOMMENTS 2\nDATA PERIODS,1,1,Data,Friday,1/1,1/1\n",
    );
    for hour in 1..=24 {
        weather_text.push_str(&format!(
            "1999,1,1,{hour},60,Source,1.0,0.0,50,101325,0,0,300,10,20,30,0,0,0,0,180,2.5\n"
        ));
    }
    ep_runtime::weather::parse_epw_weather_file(&weather_text).expect("weather fixture")
}

fn trace(value: &Value, scope: PortingScope) -> PortingScopeTrace {
    let raw = parse_epjson_str(&value.to_string()).expect("valid epJSON");
    let compiled = compile_raw_model(&raw);
    inspect_porting_scope(&raw, &compiled.report, compiled.model.as_ref(), scope)
}

#[test]
fn closed_opaque_faces_reach_source_layout_and_view_factor_kernel() {
    use ep_runtime::heat_balance::inside_convection_layout::InsideConvectionLayout;
    use ep_runtime::heat_balance::radiation::{
        ApproximateViewFactorContext, ApproximateViewFactorSurface,
        calculate_selected_approximate_view_factors,
    };

    for count in [6, 10, 7] {
        let mut value = input(PortingScope::A);
        if count == 10 {
            let surfaces = value["BuildingSurface:Detailed"].as_object_mut().unwrap();
            let mut additions = Vec::new();
            for (name, lower) in surfaces.iter_mut() {
                if lower["surface_type"] != "Wall" {
                    continue;
                }
                let mut upper = lower.clone();
                for point in lower["vertices"].as_array_mut().unwrap() {
                    if point["vertex_z_coordinate"].as_f64() == Some(1.0) {
                        point["vertex_z_coordinate"] = json!(0.5);
                    }
                }
                for point in upper["vertices"].as_array_mut().unwrap() {
                    if point["vertex_z_coordinate"].as_f64() == Some(0.0) {
                        point["vertex_z_coordinate"] = json!(0.5);
                    }
                }
                additions.push((format!("{name}:Upper"), upper));
            }
            surfaces.extend(additions);
        }
        if count == 7 {
            let surfaces = value["BuildingSurface:Detailed"].as_object_mut().unwrap();
            let mut second = surfaces["Roof"].clone();
            let points = second["vertices"].as_array().unwrap().clone();
            surfaces.get_mut("Roof").unwrap()["vertices"] =
                json!([points[0], points[1], points[2]]);
            second["vertices"] = json!([points[0], points[2], points[3]]);
            surfaces.insert("Roof:Second".into(), second);
        }
        let raw = parse_epjson_str(&value.to_string()).expect("closed authored opaque faces");
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("compiled closed opaque faces");
        let scope = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(scope.admissible, "{count}: {:?}", scope.violations);
        let input = crate::ordinary_input_processor::resolve(&raw).expect("Source raw input");
        assert_eq!(
            input.get_num_objects_found("BuildingSurface:Detailed"),
            count
        );
        for name in value["BuildingSurface:Detailed"]
            .as_object()
            .unwrap()
            .keys()
        {
            assert!(input.has_object("BuildingSurface:Detailed", name));
        }

        let layout = InsideConvectionLayout::from_typed_model(model).expect("Source associations");
        assert_eq!(layout.groups().len(), 1);
        assert_eq!(layout.surfaces().len(), count);
        let group = &layout.groups()[0];
        assert_eq!(group.zone_id, model.zones[0].id);
        assert_eq!(group.surfaces.len(), count);
        assert_eq!(
            group
                .surfaces
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            count
        );
        // The entered volume lets this public geometry helper retain closure
        // diagnostics without claiming its separate six-face automatic route.
        let geometry = ep_runtime::geometry::zone_geometry_properties(model, &model.zones[0])
            .expect("shared closed-face geometry");
        assert_eq!(geometry.faces.len(), count);
        let triangle_count = if count == 7 { 2 } else { 0 };
        assert!(
            geometry
                .faces
                .iter()
                .all(|face| matches!(face.vertices.len(), 3 | 4))
        );
        assert_eq!(
            geometry
                .faces
                .iter()
                .filter(|face| face.vertices.len() == 3)
                .count(),
            triangle_count
        );
        assert_eq!(
            model
                .surfaces
                .iter()
                .filter(|surface| surface
                    .computed_geometry
                    .as_ref()
                    .is_some_and(|geometry| geometry.shape_category
                        == ep_model::SurfaceShapeCategory::Triangular))
                .count(),
            triangle_count
        );
        assert!(geometry.diagnostics.initially_closed);
        assert!(geometry.diagnostics.edges_winding_consistent);
        assert!(geometry.diagnostics.initial_edges_not_used_twice.is_empty());
        assert_eq!(
            geometry.diagnostics.initial_unique_vertex_count,
            if count == 10 { 12 } else { 8 }
        );
        assert!((geometry.diagnostics.signed_polyhedron_volume_m3.unwrap() - 1.0).abs() < 1e-12);

        let surfaces = group
            .surfaces
            .iter()
            .map(|id| {
                let surface = model
                    .surfaces
                    .iter()
                    .find(|surface| surface.id == *id)
                    .unwrap();
                let geometry = ep_runtime::geometry::surface_geometry_properties(&surface.vertices)
                    .expect("Source Newell geometry");
                ApproximateViewFactorSurface {
                    surface_id: surface.id,
                    surface_name: surface.name.0.clone(),
                    zone_id: surface.zone,
                    zone_name: model.zones[0].name.0.clone(),
                    surface_type: surface.surface_type,
                    area_m2: geometry.area_m2,
                    azimuth_deg: geometry.azimuth_deg,
                    tilt_deg: geometry.tilt_deg,
                }
            })
            .collect::<Vec<_>>();
        let walls = if count == 10 { 8 } else { 4 };
        assert!(
            surfaces[..walls]
                .iter()
                .all(|s| s.surface_type == ep_model::SurfaceType::Wall)
        );
        assert_eq!(surfaces[walls].surface_type, ep_model::SurfaceType::Floor);
        assert!(
            surfaces[walls + 1..]
                .iter()
                .all(|s| s.surface_type == ep_model::SurfaceType::Roof)
        );
        // This is the actual shared CalcApproximateViewFactors kernel, before
        // Fix/ScriptF and runtime activation; no initialized owner is fabricated.
        let factors = calculate_selected_approximate_view_factors(
            ApproximateViewFactorContext {
                enclosure_index: 1,
                zone_id: group.zone_id,
            },
            &surfaces,
        )
        .expect("dynamic square view-factor buffer");
        assert_eq!(factors.ordered_surface_ids, group.surfaces);
        assert_eq!(factors.view_factors.len(), count * count);
        assert!(factors.warnings.is_empty());
        assert!(
            factors
                .zone_area_seen_m2
                .iter()
                .all(|area| area.is_finite() && *area > 0.0)
        );
        assert!(
            factors
                .view_factors
                .iter()
                .all(|f| f.is_finite() && *f >= 0.0)
        );
        for from in 0..count {
            let sum: f64 = (0..count)
                .map(|to| factors.view_factors[to * count + from])
                .sum();
            assert!((sum - 1.0).abs() < 1e-12);
        }
    }
}

#[test]
fn timestep_admission_compilation_and_axis_agree_for_the_full_schema_domain() {
    for requested in 1..=60 {
        let mut value = input(PortingScope::A);
        value["Timestep"] = json!({"t":{"number_of_timesteps_per_hour":requested}});
        let raw = parse_epjson_str(&value.to_string()).expect("raw clock input");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("compiled valid clock");
        let trace = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(trace.admissible, "{requested}: {:?}", trace.violations);
        let actual = crate::ordinary_equipment_input::resolve(&raw).expect("Source raw admission");
        let steps = model.timestep.number_of_timesteps_per_hour;
        assert_eq!(actual, ep_runtime::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::from_family_inventory([], steps as usize).expect("clock inventory"));
        let axes = ep_runtime::time_axis::build_environment_time_axes(model)
            .expect("effective clock axis");
        assert_eq!(axes[0].zone_timestep.timesteps_per_hour, steps);
        assert_eq!(axes[0].sample_count(), 24 * steps as usize);
    }
}

#[test]
fn run_period_year_normalization_reaches_calendar_and_axis_before_source_weather() {
    for (begin, end, weekday, year) in [
        (json!(0), json!(0), Some("Tuesday"), 2013),
        (json!(0.9), json!(-0.9), Some("Tuesday"), 2013),
        (json!(2013.9), json!(2013.9), None, 2013),
        (json!(2013), json!(0), None, 2013),
    ] {
        let mut value = input(PortingScope::A);
        value["RunPeriod"]["Period"]["begin_year"] = begin;
        value["RunPeriod"]["Period"]["end_year"] = end;
        if let Some(weekday) = weekday {
            value["RunPeriod"]["Period"]["day_of_week_for_start_day"] = json!(weekday);
        }
        let raw = parse_epjson_str(&value.to_string()).expect("raw RunPeriod year variants");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("normalized typed RunPeriod");
        let trace = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(trace.admissible, "{value}: {:?}", trace.violations);
        assert!(
            crate::ordinary_input_processor::resolve(&raw).is_ok(),
            "raw Source admission"
        );
        let axes = ep_runtime::time_axis::build_environment_time_axes(model)
            .expect("normalized date calendar");
        assert_eq!(axes[0].calendar.start_year, year);
        assert_eq!(axes[0].calendar.end_year, year);
        assert_eq!(axes[0].calendar.total_days, 1);
        assert_eq!(
            axes[0].sample_count(),
            24 * model.timestep.number_of_timesteps_per_hour as usize
        );
    }
    for (name, period, begin_year, end_year, days) in [
        (
            "December rollover with fractional start and zero end year",
            json!({"begin_month":12,"begin_day_of_month":31,"begin_year":2013.9,"end_month":1,"end_day_of_month":1,"end_year":0}),
            2013,
            2014,
            2,
        ),
        (
            "February29 zero years select the Original leap default",
            json!({"begin_month":2,"begin_day_of_month":29,"begin_year":0,"end_month":2,"end_day_of_month":29,"end_year":0}),
            2012,
            2012,
            1,
        ),
    ] {
        let mut value = input(PortingScope::A);
        value["RunPeriod"]["Period"] = period;
        let raw = parse_epjson_str(&value.to_string()).expect("raw year/date normalization");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("normalized year/date model");
        let trace = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(trace.admissible, "{name}: {:?}", trace.violations);
        assert!(
            crate::ordinary_input_processor::resolve(&raw).is_ok(),
            "{name}"
        );
        let axes =
            ep_runtime::time_axis::build_environment_time_axes(model).expect("year/date calendar");
        assert_eq!(axes[0].calendar.start_year, begin_year, "{name}");
        assert_eq!(axes[0].calendar.end_year, end_year, "{name}");
        assert_eq!(axes[0].calendar.total_days, days, "{name}");
        assert_eq!(
            axes[0].sample_count(),
            days * 24 * model.timestep.number_of_timesteps_per_hour as usize,
            "{name}"
        );
    }
    for (begin, end) in [(1582, 1582), (0, 2013), (2013, 2012)] {
        let mut value = input(PortingScope::A);
        value["RunPeriod"]["Period"]["begin_year"] = json!(begin);
        value["RunPeriod"]["Period"]["end_year"] = json!(end);
        let raw =
            parse_epjson_str(&value.to_string()).expect("raw Source-invalid year relationship");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("typed numeric fields");
        assert!(
            ep_runtime::time_axis::build_environment_time_axes(model).is_err(),
            "Source-invalid calendar must stay blocked"
        );
    }
}

#[test]
fn heat_balance_numeric_controls_admit_schema_numbers_without_changing_algorithms() {
    for (field, valid, invalid) in [
        (
            "surface_temperature_upper_limit",
            vec![200.0, 250.5],
            vec![199.999, 0.0],
        ),
        (
            "minimum_surface_convection_heat_transfer_coefficient_value",
            vec![0.1, 4.25, 1e-9],
            vec![0.0, -0.1],
        ),
        (
            "maximum_surface_convection_heat_transfer_coefficient_value",
            vec![1.0, 250.5],
            vec![0.999, -1.0],
        ),
    ] {
        let mut cases: Vec<_> = valid
            .into_iter()
            .map(|value| (json!(value), true))
            .collect();
        cases.extend(invalid.into_iter().map(|value| (json!(value), false)));
        cases.extend(
            [
                Value::Null,
                json!(""),
                json!(" "),
                json!("200"),
                json!(true),
            ]
            .into_iter()
            .map(|value| (value, false)),
        );
        for (value, accepted) in cases {
            let mut authored = input(PortingScope::A);
            authored["HeatBalanceAlgorithm"] =
                json!({"h":{"algorithm":"ConductionTransferFunction"}});
            authored["HeatBalanceAlgorithm"]["h"][field] = value.clone();
            let raw = parse_epjson_str(&authored.to_string()).expect("raw numeric controls");
            let compiled = compile_raw_model(&raw);
            let model = compiled.model.as_ref().expect("unrelated typed input");
            let scope = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
            assert_eq!(
                scope.admissible, accepted,
                "{field}={value}: {:?}",
                scope.violations
            );
            assert_eq!(
                crate::ordinary_equipment_input::resolve(&raw).is_ok(),
                accepted,
                "{field}={value}"
            );
            if accepted {
                assert_eq!(
                    scope.settings["heat_balance_controls"][field].as_f64(),
                    value.as_f64()
                );
            }
        }
    }
    let mut authored = input(PortingScope::A);
    authored["HeatBalanceAlgorithm"] = json!({"h":{}});
    let defaults = trace(&authored, PortingScope::A);
    assert!(defaults.admissible, "{:?}", defaults.violations);
    assert_eq!(
        defaults.settings["heat_balance_controls"],
        json!({
            "surface_temperature_upper_limit":200.0,
            "minimum_surface_convection_heat_transfer_coefficient_value":0.1,
            "maximum_surface_convection_heat_transfer_coefficient_value":1000.0,
        })
    );
    // Native has no global lower<=upper condition without user convection.
    authored["HeatBalanceAlgorithm"]["h"] = json!({
        "surface_temperature_upper_limit":250,
        "minimum_surface_convection_heat_transfer_coefficient_value":4,
        "maximum_surface_convection_heat_transfer_coefficient_value":1,
    });
    let crossed = trace(&authored, PortingScope::A);
    assert!(crossed.admissible, "{:?}", crossed.violations);
    let raw = parse_epjson_str(&authored.to_string()).expect("raw independent limits");
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_ok());
}

#[test]
fn default_controls_and_source_adjustments_are_observable() {
    let mut value = input(PortingScope::A);
    value["Building"]["Building"]["north_axis"] = json!(-450);
    value["Building"]["Building"]["minimum_number_of_warmup_days"] = json!(30);
    let actual = trace(&value, PortingScope::A);
    assert!(actual.admissible, "{:?}", actual.violations);
    assert_eq!(actual.settings["building"]["north_axis_deg"], -450.0);
    assert_eq!(
        actual.settings["building"]["effective_north_axis_deg"],
        -90.0
    );
    assert_eq!(actual.settings["building"]["maximum_warmup_days"], 30);
    assert_eq!(actual.settings["building"]["minimum_warmup_days"], 30);
    assert_eq!(
        actual.settings["heat_balance_algorithm"],
        "ConductionTransferFunction"
    );
    assert_eq!(
        actual.settings["zone_air_heat_balance_algorithm"],
        "ThirdOrderBackwardDifference"
    );
    assert_eq!(actual.settings["room_air_model_type"], "Mixing");
    assert_eq!(actual.settings["timesteps_per_hour"], 4);
    assert_eq!(actual.production["conformance_claim"], false);
    value["Building"]["Building"]["maximum_number_of_warmup_days"] = json!(0);
    assert!(!trace(&value, PortingScope::A).admissible);

    // This tests the input boundary only, not Source activation or window
    // transport. Actual opaque consumers already branch on the real enum;
    // reflection and positive window/TDD children remain separately guarded.
    for scope in [PortingScope::A, PortingScope::B] {
        for (distribution, accepted) in [
            ("MinimalShadowing", true),
            ("FullExterior", true),
            ("FullInteriorAndExterior", true),
            ("FullExteriorWithReflections", false),
            ("FullInteriorAndExteriorWithReflections", false),
        ] {
            let mut authored = input(scope);
            authored["Building"]["Building"]["solar_distribution"] = json!(distribution);
            let actual = trace(&authored, scope);
            assert_eq!(
                actual.admissible, accepted,
                "{scope:?}/{distribution}: {:?}",
                actual.violations
            );
            assert_eq!(
                actual.settings["building"]["solar_distribution"],
                distribution
            );
            assert_eq!(actual.production["conformance_claim"], false);
            if !accepted {
                assert!(
                    actual
                        .violations
                        .iter()
                        .any(|message| message.contains("solar reflection"))
                );
            }
            let raw = parse_epjson_str(&authored.to_string()).expect("schema-valid solar enum");
            let ordinary = match scope {
                PortingScope::A => crate::ordinary_equipment_input::resolve(&raw),
                PortingScope::B => crate::ordinary_equipment_input::resolve_source_b(&raw),
            };
            assert!(
                ordinary.is_ok(),
                "raw field ingress differs for {distribution}: {ordinary:?}"
            );
        }
    }
}

#[test]
fn effective_site_atmosphere_uses_the_compiled_terrain_and_no_override() {
    // Pinned GetProjectControlData terrain branches and the absent-object
    // GetSiteAtmosphereData default; these are settings, not height physics.
    for (terrain, exponent, height) in [
        ("Country", 0.14, 270.0),
        ("Suburbs", 0.22, 370.0),
        ("Urban", 0.22, 370.0),
        ("City", 0.33, 460.0),
        ("Ocean", 0.10, 210.0),
    ] {
        let mut value = input(PortingScope::A);
        value["Building"]["Building"]["terrain"] = json!(terrain);
        let actual = trace(&value, PortingScope::A);
        assert!(actual.admissible, "{terrain}: {:?}", actual.violations);
        assert_eq!(
            actual.settings["site_atmosphere"],
            json!({
                "wind_speed_profile_exponent": exponent,
                "wind_speed_profile_boundary_layer_thickness_m": height,
                "air_temperature_gradient_k_per_m": 0.0065,
            })
        );
    }
    let value = input(PortingScope::A);
    assert_eq!(
        trace(&value, PortingScope::A).settings["site_atmosphere"],
        json!({
            "wind_speed_profile_exponent": 0.22,
            "wind_speed_profile_boundary_layer_thickness_m": 370.0,
            "air_temperature_gradient_k_per_m": 0.0065,
        })
    );
    let mut value = value;
    value["Site:HeightVariation"] = json!({"Override":{
        "wind_speed_profile_exponent":0.22,
        "wind_speed_profile_boundary_layer_thickness":370.0,
        "air_temperature_gradient_coefficient":0.0065,
    }});
    let actual = trace(&value, PortingScope::A);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|v| v.contains("Site:HeightVariation"))
    );
}

#[test]
fn scientific_algorithms_are_rejected_before_bounded_execution() {
    for (kind, field, setting, expected) in [
        (
            "HeatBalanceAlgorithm",
            "algorithm",
            "ConductionFiniteDifference",
            "heat_balance_algorithm",
        ),
        (
            "ZoneAirHeatBalanceAlgorithm",
            "algorithm",
            "EulerMethod",
            "zone_air_heat_balance_algorithm",
        ),
        (
            "RoomAirModelType",
            "room_air_modeling_type",
            "UserDefined",
            "room_air_model_type",
        ),
        (
            "SurfaceConvectionAlgorithm:Inside",
            "algorithm",
            "CeilingDiffuser",
            "convection",
        ),
        (
            "SurfaceConvectionAlgorithm:Outside",
            "algorithm",
            "AdaptiveConvectionAlgorithm",
            "convection",
        ),
    ] {
        let mut value = input(PortingScope::A);
        value[kind] = json!({"Control":{field:setting}});
        let actual = trace(&value, PortingScope::A);
        assert!(!actual.admissible, "{kind} incorrectly admitted");
        assert!(
            actual
                .violations
                .iter()
                .any(|message| message.contains(expected)),
            "{:?}",
            actual.violations
        );
    }
}

#[test]
fn new_features_and_active_design_days_are_not_silently_ignored() {
    let mut value = input(PortingScope::A);
    value["ZoneInfiltration:DesignFlowRate"] = json!({"Infiltration":{"design_flow_rate":0.01}});
    assert!(!trace(&value, PortingScope::A).admissible);
    let mut value = input(PortingScope::A);
    value["SizingPeriod:DesignDay"] = json!({"Day":{}});
    assert!(trace(&value, PortingScope::A).admissible);
    value["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] = json!("Yes");
    let actual = trace(&value, PortingScope::A);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|message| message.contains("active sizing-period"))
    );
}

#[test]
fn direct_sensible_limits_are_explicit_and_scope_a_rejects_conditioning() {
    let mut value = input(PortingScope::B);
    assert!(
        trace(&value, PortingScope::B).admissible,
        "{:?}",
        trace(&value, PortingScope::B).violations
    );
    assert!(!trace(&value, PortingScope::A).admissible);
    for side in ["heating", "cooling"] {
        value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][format!("{side}_limit")] =
            json!("LimitFlowRate");
        value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][format!("maximum_{side}_air_flow_rate")] =
            json!(0.02);
    }
    let actual = trace(&value, PortingScope::B);
    assert!(actual.admissible, "{:?}", actual.violations);
    assert_eq!(
        actual.settings["ideal_loads"][0]["maximum_heating_air_flow_rate_m3_per_s"],
        0.02
    );
    value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_heating_air_flow_rate"] =
        json!("Autosize");
    let actual = trace(&value, PortingScope::B);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|message| message.contains("Autosize"))
    );
}

#[test]
fn bounded_controls_do_not_change_ordinary_support_assessment() {
    let mut value = input(PortingScope::A);
    value["ZoneAirHeatBalanceAlgorithm"] =
        json!({"Algorithm":{"algorithm":"ThirdOrderBackwardDifference"}});
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    assert!(
        inspect_porting_scope(
            &raw,
            &compiled.report,
            compiled.model.as_ref(),
            PortingScope::A
        )
        .admissible
    );
    let ordinary = assess_support(
        &raw,
        &compiled.report,
        compiled.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Summary,
    );
    assert_eq!(ordinary.status, SupportStatus::Unsupported);
    assert!(
        ordinary
            .unsupported_objects
            .iter()
            .any(|entry| entry.object_type == "ZoneAirHeatBalanceAlgorithm")
    );
}

#[test]
fn latent_sources_are_blocked_inside_the_sensible_only_contract() {
    let mut value = input(PortingScope::A);
    value["OtherEquipment"] = json!({"Gain":{
        "zone_or_zonelist_or_space_or_spacelist_name":"Zone", "schedule_name":"On",
        "design_level_calculation_method":"EquipmentLevel", "design_level":100,
        "fraction_latent":0.0, "fraction_radiant":0.0, "fraction_lost":0.0,
    }});
    assert!(trace(&value, PortingScope::A).admissible);
    value["OtherEquipment"]["Gain"]["fraction_latent"] = json!(0.25);
    let actual = trace(&value, PortingScope::A);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|message| message.contains("latent gains"))
    );
}

#[test]
fn effective_calendar_branches_are_checked_after_weather_resolution() {
    let mut value = input(PortingScope::A);
    value["RunPeriodControl:SpecialDays"] = json!({"Holiday":{
        "start_date":"1/1", "duration":1, "special_day_type":"Holiday",
    }});
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    let weather = single_day_weather_fixture();
    let actual = super::environment::environment_from_weather(
        compiled.model.as_ref().expect("compiled model"),
        &weather,
        "embedded unit fixture",
    )
    .expect("weather inputs");
    assert_eq!(actual["active_special_day_count"], 1);
    assert!(
        validate_porting_environment(&actual)
            .iter()
            .any(|message| message.contains("special-day"))
    );
    let mut value = input(PortingScope::A);
    value["RunPeriodControl:DaylightSavingTime"] =
        json!({"DST":{"start_date":"1/1","end_date":"1/1"}});
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    let actual = super::environment::environment_from_weather(
        compiled.model.as_ref().expect("compiled model"),
        &weather,
        "embedded unit fixture",
    )
    .expect("weather inputs");
    assert_eq!(actual["daylight_saving"]["active"], true);
    assert!(
        validate_porting_environment(&actual)
            .iter()
            .any(|message| message.contains("daylight-saving"))
    );
}

#[test]
fn weather_trace_uses_real_calendar_selection_and_zone_schedule_values() {
    let value = input(PortingScope::B);
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    let weather = single_day_weather_fixture();
    let actual = super::environment::environment_from_weather(
        compiled.model.as_ref().expect("compiled model"),
        &weather,
        "embedded unit fixture",
    )
    .expect("weather input trace");
    assert_eq!(actual["resolved_start_year"], 2013);
    assert_eq!(actual["resolved_start_day_of_week"], "Tuesday");
    assert_eq!(actual["calendar_year"], 2013);
    assert_eq!(actual["weather_record_year"], 1999);
    assert_eq!(actual["hourly_samples"], 24);
    assert_eq!(actual["zone_timestep_samples"], 96);
    assert_eq!(actual["daylight_saving"]["active"], false);
    let schedules = actual["schedule_values"].as_array().expect("schedules");
    let thermostat = schedules
        .iter()
        .find(|schedule| schedule["name"] == "TYPE")
        .expect("thermostat schedule");
    assert!(
        thermostat["values"]
            .as_array()
            .expect("values")
            .iter()
            .all(|value| value == &json!(4.0))
    );
}

#[test]
fn simulation_control_inactive_hvac_request_preserves_raw_getter_and_sizing_boundary() {
    for scope in [PortingScope::A, PortingScope::B] {
        let mut value = input(scope);
        value["SimulationControl"]["Control"]["do_plant_sizing_calculation"] = json!("No");
        value["SimulationControl"]["Control"]["do_hvac_sizing_simulation_for_sizing_periods"] =
            json!("Yes");
        value["SimulationControl"]["Control"]["maximum_number_of_hvac_sizing_simulation_passes"] =
            json!(3);
        let raw = parse_epjson_str(&value.to_string()).expect("raw cross-flag input");
        let compiled = compile_raw_model(&raw);
        let actual = inspect_porting_scope(&raw, &compiled.report, compiled.model.as_ref(), scope);
        assert!(actual.admissible, "{:?}", actual.violations);
        // The admission trace must not silently rewrite the requested Yes.
        assert_eq!(
            actual.settings["simulation_control"]["do_hvac_sizing_simulation_for_sizing_periods"],
            true
        );
        let input = match scope {
            PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
            PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
        }
        .expect("actual shared InputProcessor ingress");
        let project = crate::ordinary_project_data::resolve(&raw, true, &input)
            .expect("same raw operands for the real GetProjectData");
        let control = &project.objects["SimulationControl"][0];
        use ep_runtime::heat_balance::selected_project_data::ProjectInputValue;
        assert_eq!(
            control.fields["do_hvac_sizing_simulation_for_sizing_periods"],
            ProjectInputValue::String("Yes".into())
        );
        assert_eq!(
            control.fields["do_plant_sizing_calculation"],
            ProjectInputValue::String("No".into())
        );
        assert_eq!(
            control.fields["maximum_number_of_hvac_sizing_simulation_passes"],
            ProjectInputValue::Number(3.0)
        );
        for field in [
            "do_zone_sizing_calculation",
            "do_system_sizing_calculation",
            "do_plant_sizing_calculation",
        ] {
            let mut active = value.clone();
            active["SimulationControl"]["Control"][field] = json!("Yes");
            assert!(
                !trace(&active, scope).admissible,
                "active {field} must remain blocked"
            );
        }
    }
}

#[test]
fn building_surface_ground_months_preserve_numbers_defaults_and_type_boundary() {
    const FAMILY: &str = "Site:GroundTemperature:BuildingSurface";
    const MONTHS: [&str; 12] = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    let weather = single_day_weather_fixture();
    for scope in [PortingScope::A, PortingScope::B] {
        let mut value = input(scope);
        value[FAMILY] = json!({});
        assert!(trace(&value, scope).admissible);
        let raw = parse_epjson_str(&value.to_string()).expect("empty optional family map");
        let ip = match scope {
            PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
            PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
        }
        .expect("zero objects use the real absent-model default path");
        assert_eq!(ip.get_num_objects_found(FAMILY), 0);
        value[FAMILY] = json!({"Ground":{"january_ground_temperature":26.5}});
        let raw = parse_epjson_str(&value.to_string()).expect("actual monthly Source input");
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("actual compiled monthly model");
        let mut admitted = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
        assert!(admitted.admissible, "{:?}", admitted.violations);
        assert!(
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted).is_none()
        );
        let environment = super::environment::environment_from_weather(
            model,
            &weather,
            "embedded ground-admission weather fixture",
        )
        .expect("real environment preparation");
        assert!(validate_porting_environment(&environment).is_empty());
        admitted.environment = Some(environment);
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                .expect("same real IP validation after bounded scope and environment");
        let assessment = crate::support::assess_support_with_source_admission(
            &raw,
            &compiled.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&admission),
        );
        assert!(
            assessment.allows_rust_runtime(),
            "{:?}",
            assessment.diagnostics
        );
        assert!(
            assessment
                .source_consumed_objects
                .iter()
                .any(|entry| entry.object_type == FAMILY)
        );
        assert!(
            !assessment
                .unsupported_objects
                .iter()
                .any(|entry| entry.object_type == FAMILY)
        );
        let unscoped = assess_support(
            &raw,
            &compiled.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
        );
        assert!(!unscoped.allows_rust_runtime());
        assert!(
            unscoped
                .unsupported_objects
                .iter()
                .any(|entry| entry.object_type == FAMILY)
        );
        for sibling in [
            "Site:GroundTemperature:FCfactorMethod",
            "Site:GroundTemperature:Shallow",
            "Site:GroundTemperature:Deep",
        ] {
            let mut blocked = value.clone();
            blocked[sibling] = json!({"Other":{}});
            let raw = parse_epjson_str(&blocked.to_string()).expect("other ground family");
            let blocked_trace = trace(&blocked, scope);
            assert!(!blocked_trace.admissible);
            assert!(
                crate::support::SourceSupportAdmission::from_validated_scope(&raw, &blocked_trace)
                    .is_none()
            );
        }
        let mut invalid = value.clone();
        invalid[FAMILY]["Ground"]["january_ground_temperature"] = json!(null);
        let raw = parse_epjson_str(&invalid.to_string()).expect("wrong direct JSON numeric type");
        assert!(
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted).is_none()
        );
        value[FAMILY] = json!({"Ground":{}});
        for supplied in [None, Some(-10.5), Some(18.0), Some(32.25)] {
            value[FAMILY]["Ground"] = json!({});
            for (index, month) in MONTHS.iter().enumerate() {
                if let Some(supplied) = supplied {
                    value[FAMILY]["Ground"][format!("{month}_ground_temperature")] =
                        json!(supplied + index as f64);
                }
            }
            assert!(trace(&value, scope).admissible);
            let raw = parse_epjson_str(&value.to_string()).expect("numeric monthly input");
            let ip = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .expect("same actual raw IP admission");
            let projected = crate::ordinary_heat_balance_input::resolve(&raw, &ip)
                .expect("same raw monthly operands for actual Source factory");
            assert_eq!(projected.objects[FAMILY].len(), 1);
            for (index, month) in MONTHS.iter().enumerate() {
                let key = format!("{month}_ground_temperature");
                let field = projected.objects[FAMILY][0].fields.get(&key);
                if let Some(supplied) = supplied {
                    assert_eq!(field, Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(supplied + index as f64)));
                } else {
                    // Absence stays absent until the actual getter's18C default.
                    assert!(field.is_none());
                }
            }
        }
        for key in ["", " ", "Arbitrary key"] {
            let mut keyed = input(scope);
            keyed[FAMILY] = json!({key:{"january_ground_temperature":26.5}});
            assert!(trace(&keyed, scope).admissible);
            let raw = parse_epjson_str(&keyed.to_string()).expect("unnamed numeric singleton key");
            let ip = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .expect("actual raw IP preserves arbitrary singleton key");
            assert!(ip.has_object(FAMILY, key));
            let projected = crate::ordinary_heat_balance_input::resolve(&raw, &ip)
                .expect("same Source identity");
            assert_eq!(projected.objects[FAMILY][0].object_name, key);
        }
        for invalid in [
            json!(null),
            json!(""),
            json!("18"),
            json!(false),
            json!([]),
            json!({}),
        ] {
            value[FAMILY]["Ground"]["january_ground_temperature"] = invalid;
            let raw = parse_epjson_str(&value.to_string()).expect("wrong direct JSON field type");
            let result = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            };
            assert!(result.is_err());
        }
        value[FAMILY] = json!({"Ground":{}, "Duplicate":{}});
        let raw = parse_epjson_str(&value.to_string()).expect("duplicate singleton");
        let result = match scope {
            PortingScope::A => crate::ordinary_equipment_input::resolve(&raw),
            PortingScope::B => crate::ordinary_equipment_input::resolve_source_b(&raw),
        };
        assert!(result.err().is_some_and(|error| {
            error.contains("requires unique Site:GroundTemperature:BuildingSurface")
        }));
    }
    let converted = r#"{"Zone":{"z":{}},"Site:GroundTemperature:BuildingSurface":{"Site:GroundTemperature:BuildingSurface 1":{}}}"#;
    let raw = ep_raw_model::parse_epjson_str_with_idf_order(
        converted,
        "Zone,z;Site:GroundTemperature:BuildingSurface;",
    )
    .expect("actual IDF omission overlay");
    let ip =
        crate::ordinary_input_processor::resolve(&raw).expect("IDF default-only ground object");
    let projected =
        crate::ordinary_heat_balance_input::resolve(&raw, &ip).expect("IDF raw factory operands");
    assert_eq!(projected.objects[FAMILY][0].idf_max_fields, Some(12));
    assert!(projected.objects[FAMILY][0].fields.is_empty());
}

#[test]
fn interior_air_gap_preserves_raw_ip_typed_identity_and_source_support() {
    const FAMILY: &str = "Material:AirGap";
    let weather = single_day_weather_fixture();
    for scope in [PortingScope::A, PortingScope::B] {
        let mut value = input(scope);
        value["Material"] = json!({"Solid":{"roughness":"MediumRough","thickness":0.1,"conductivity":1.7,"density":2200,"specific_heat":840}});
        value[FAMILY] =
            json!({"MiXeD Gap":{"thermal_resistance":0.15},"Spare":{"thermal_resistance":2.0}});
        value["Construction"]["Opaque"] =
            json!({"outside_layer":"Solid","layer_2":"MiXeD Gap","layer_3":"Solid"});
        let raw = parse_epjson_str(&value.to_string()).expect("named numeric-only AirGap");
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("actual interior AirGap model");
        let ip = match scope {
            PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
            PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
        }
        .expect("same actual raw/IP admission");
        assert_eq!(ip.get_num_objects_found(FAMILY), 2);
        assert!(ip.has_object(FAMILY, "MiXeD Gap"));
        let material_input =
            crate::ordinary_material_input::resolve(&raw, &ip).expect("same material raw custody");
        assert!(material_input.source_is_epjson);
        assert_eq!(material_input.air_gaps.len(), 2);
        assert_eq!(material_input.air_gaps[0].object_name, "MiXeD Gap");
        assert_eq!(material_input.air_gaps[0].idf_max_fields, None);
        assert_eq!(
            material_input.air_gaps[0].fields["thermal_resistance"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(0.15)
        );
        let construction = &model.constructions[0];
        let gap_id = construction.effective_layers()[1];
        let gap = model
            .materials
            .iter()
            .find(|m| m.id == gap_id)
            .expect("same actual middle layer");
        assert!(
            matches!(gap.definition, ep_model::MaterialDefinition::AirGap(v) if v.thermal_resistance_m2_k_per_w == 0.15)
        );
        let mut admitted = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
        assert!(admitted.admissible, "{:?}", admitted.violations);
        assert!(
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted).is_none()
        );
        let environment = super::environment::environment_from_weather(
            model,
            &weather,
            "embedded interior-AirGap weather",
        )
        .expect("actual calendar preparation");
        assert!(validate_porting_environment(&environment).is_empty());
        admitted.environment = Some(environment);
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                .expect("all actual raw/IP/scope/weather checks");
        let assessment = crate::support::assess_support_with_source_admission(
            &raw,
            &compiled.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&admission),
        );
        assert!(
            assessment.allows_rust_runtime(),
            "{:?}",
            assessment.diagnostics
        );
        assert!(
            assessment
                .typed_objects
                .iter()
                .any(|entry| entry.object_type == FAMILY && entry.count == 2)
        );
        assert!(
            !assessment
                .source_consumed_objects
                .iter()
                .any(|entry| entry.object_type == FAMILY),
            "typed AirGap stays typed, never rebranded RawOnly"
        );
        for accepted in [0.0005, 0.001, 0.15, 1.0e100] {
            let mut boundary = value.clone();
            boundary[FAMILY]["MiXeD Gap"]["thermal_resistance"] = json!(accepted);
            let raw =
                parse_epjson_str(&boundary.to_string()).expect("finite positive schema number");
            assert!(
                crate::ordinary_material_input::resolve(
                    &raw,
                    &match scope {
                        PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                        PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                    }
                    .expect("R has no invented admission upper/min0.001 bound")
                )
                .is_ok()
            );
        }
        for invalid in [
            json!(null),
            json!(""),
            json!("0.15"),
            json!(true),
            json!([]),
            json!({}),
            json!(0),
            json!(-0.15),
        ] {
            let mut boundary = value.clone();
            boundary[FAMILY]["MiXeD Gap"]["thermal_resistance"] = invalid;
            let raw =
                parse_epjson_str(&boundary.to_string()).expect("invalid scalar preserved raw");
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .is_err()
            );
        }
        for fields in [
            json!({}),
            json!({"thermal_resistance":0.15,"roughness":"Rough"}),
        ] {
            let mut boundary = value.clone();
            boundary[FAMILY]["MiXeD Gap"] = fields;
            let raw = parse_epjson_str(&boundary.to_string()).expect("missing/unknown field raw");
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .is_err()
            );
        }
        for key in ["", " "] {
            let mut boundary = value.clone();
            boundary[FAMILY] = json!({(key):{"thermal_resistance":0.15}});
            let raw = parse_epjson_str(&boundary.to_string()).expect("invalid named instance raw");
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .is_err()
            );
        }
        for construction in [
            json!({"outside_layer":"MiXeD Gap","layer_2":"Solid"}),
            json!({"outside_layer":"Solid","layer_2":"MiXeD Gap"}),
        ] {
            let mut boundary = value.clone();
            boundary["Construction"]["Opaque"] = construction;
            let raw =
                parse_epjson_str(&boundary.to_string()).expect("invalid endpoint retained raw");
            let compiled = compile_raw_model(&raw);
            assert!(compiled.model.is_none());
            assert!(
                compiled
                    .report
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "InvalidAirGapLayerPosition")
            );
        }
        let mut duplicate = value.clone();
        duplicate[FAMILY] = json!({"Solid":{"thermal_resistance":0.15}});
        let duplicate = compile_raw_model(&parse_epjson_str(&duplicate.to_string()).unwrap());
        assert!(duplicate.model.is_none());
        assert!(
            duplicate
                .report
                .diagnostics
                .iter()
                .any(|d| d.code == "DuplicateName" && d.object_type == FAMILY)
        );
    }
}

#[test]
fn unnamed_geometry_rule_keys_preserve_raw_ip_identity_and_strict_alpha_scalars() {
    const FAMILY: &str = "GlobalGeometryRules";
    for scope in [PortingScope::A, PortingScope::B] {
        for key in ["", " ", "Arbitrary Geometry Identity"] {
            let mut value = input(scope);
            value[FAMILY] = json!({(key):{"starting_vertex_position":"UpperLeftCorner","vertex_entry_direction":"Counterclockwise","coordinate_system":"World","daylighting_reference_point_coordinate_system":"","rectangular_surface_coordinate_system":"World"}});
            let raw = parse_epjson_str(&value.to_string()).expect("schema unnamed singleton key");
            let compiled = compile_raw_model(&raw);
            assert!(
                compiled.model.is_some(),
                "{:?}",
                compiled.report.diagnostics
            );
            let ip = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .expect("same GGR raw/IP scalar admission");
            assert_eq!(ip.get_num_objects_found(FAMILY), 1);
            assert!(ip.has_object(FAMILY, key));
            assert!(
                ip.is_object_unused(FAMILY, key),
                "projection is not getter usage"
            );
            let actual = crate::ordinary_heat_balance_input::resolve(&raw, &ip)
                .expect("same raw GGR custody");
            assert_eq!(actual.surface.geometry_rules.len(), 1);
            let row = &actual.surface.geometry_rules[0];
            assert_eq!(row.object_name, key);
            assert_eq!(
                row.fields["coordinate_system"],
                ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                    "World".into()
                )
            );
            assert!(trace(&value, scope).admissible);
        }
        for field in [
            "starting_vertex_position",
            "vertex_entry_direction",
            "coordinate_system",
            "daylighting_reference_point_coordinate_system",
            "rectangular_surface_coordinate_system",
        ] {
            for scalar in [
                json!(null),
                json!(1),
                json!(true),
                json!([]),
                json!({}),
                json!(" "),
            ] {
                let mut value = input(scope);
                value[FAMILY]["Geometry"][field] = scalar;
                let raw = parse_epjson_str(&value.to_string()).expect("invalid alpha remains raw");
                assert!(
                    match scope {
                        PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                        PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                    }
                    .is_err(),
                    "{scope:?}/{field}"
                );
            }
        }
        let mut duplicate = input(scope);
        duplicate[FAMILY] = json!({"":duplicate[FAMILY]["Geometry"].clone()," ":duplicate[FAMILY]["Geometry"].clone()});
        let raw = parse_epjson_str(&duplicate.to_string())
            .expect("duplicate singleton objects preserved raw");
        assert!(
            match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .is_err()
        );
    }
}

#[test]
fn ground_boundaries_reach_real_raw_ip_scope_and_source_support_without_sun_wind_aliases() {
    let weather = single_day_weather_fixture();
    for scope in [PortingScope::A, PortingScope::B] {
        for target in ["West", "Floor", "Roof"] {
            for (sun, wind) in [
                ("NoSun", "NoWind"),
                ("SunExposed", "NoWind"),
                ("NoSun", "WindExposed"),
                ("SunExposed", "WindExposed"),
            ] {
                let mut value = input(scope);
                value["Site:GroundTemperature:BuildingSurface"] =
                    json!({"Ground":{"january_ground_temperature":18.0}});
                let surface = &mut value["BuildingSurface:Detailed"][target];
                surface["outside_boundary_condition"] = json!("Ground");
                surface["outside_boundary_condition_object"] = json!("unused Ground operand");
                surface["sun_exposure"] = json!(sun);
                surface["wind_exposure"] = json!(wind);
                let raw = parse_epjson_str(&value.to_string()).expect("actual Ground scalar input");
                let compiled = compile_raw_model(&raw);
                let model = compiled.model.as_ref().expect("schema-valid Ground model");
                let ip = match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .expect("same real raw/IP admission");
                let projected = crate::ordinary_heat_balance_input::resolve(&raw, &ip)
                    .expect("same actual surface getter operands");
                let row = projected
                    .surface
                    .building_surfaces
                    .iter()
                    .find(|row| row.object.object_name == target)
                    .unwrap();
                assert_eq!(
                    row.object.fields["outside_boundary_condition_object"],
                    ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                        "unused Ground operand".into()
                    )
                );
                let typed = model
                    .surfaces
                    .iter()
                    .find(|row| row.name.0.eq_ignore_ascii_case(target))
                    .unwrap();
                assert_eq!(
                    typed.outside_boundary_condition,
                    ep_model::OutsideBoundaryCondition::Ground
                );
                assert_eq!(
                    typed.sun_exposure == ep_model::SunExposure::SunExposed,
                    sun == "SunExposed"
                );
                assert_eq!(
                    typed.wind_exposure == ep_model::WindExposure::WindExposed,
                    wind == "WindExposed"
                );
                let mut admitted =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(admitted.admissible, "{:?}", admitted.violations);
                assert!(
                    crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                        .is_none()
                );
                let environment = super::environment::environment_from_weather(
                    model,
                    &weather,
                    "embedded Ground boundary weather",
                )
                .unwrap();
                assert!(validate_porting_environment(&environment).is_empty());
                admitted.environment = Some(environment);
                let admission =
                    crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                        .expect("actual validated Source scope/environment");
                let support = crate::support::assess_support_with_source_admission(
                    &raw,
                    &compiled.report,
                    Some(model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&admission),
                );
                assert!(support.allows_rust_runtime(), "{:?}", support.diagnostics);
            }
        }
        for boundary in [
            "GroundFCfactorMethod",
            "Foundation",
            "OtherSideCoefficients",
            "Surface",
        ] {
            let mut value = input(scope);
            value["BuildingSurface:Detailed"]["Floor"]["outside_boundary_condition"] =
                json!(boundary);
            let raw = parse_epjson_str(&value.to_string()).unwrap();
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .is_err()
            );
        }
    }
}

#[test]
fn global_convection_enums_reach_validated_source_support_and_inherited_zone() {
    use ep_model::{InsideSurfaceConvectionAlgorithm as I, OutsideSurfaceConvectionAlgorithm as O};
    const INSIDE: &str = "SurfaceConvectionAlgorithm:Inside";
    const OUTSIDE: &str = "SurfaceConvectionAlgorithm:Outside";
    let weather = single_day_weather_fixture();
    for scope in [PortingScope::A, PortingScope::B] {
        for (inside, expected_inside) in [("TARP", I::Tarp), ("Simple", I::Simple)] {
            for (outside, expected_outside) in [
                ("DOE-2", O::Doe2),
                ("SimpleCombined", O::SimpleCombined),
                ("TARP", O::Tarp),
                ("MoWiTT", O::MoWitt),
            ] {
                let mut value = input(scope);
                value[INSIDE] = json!({" ":{"algorithm":inside}});
                value[OUTSIDE] = json!({"":{"algorithm":outside}});
                let raw = parse_epjson_str(&value.to_string()).expect("actual global selections");
                let compiled = compile_raw_model(&raw);
                let model = compiled.model.as_ref().expect("valid typed enum choices");
                let zone = &model.zones[0];
                assert_eq!(
                    zone.inside_convection_algorithm.effective(),
                    expected_inside
                );
                assert_eq!(
                    zone.outside_convection_algorithm.effective(),
                    expected_outside
                );
                assert!(
                    !zone.inside_convection_algorithm.is_override()
                        && !zone.outside_convection_algorithm.is_override()
                );
                let ip = match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .expect("actual IP admission");
                assert!(ip.has_object(INSIDE, " ") && ip.has_object(OUTSIDE, ""));
                let projected = crate::ordinary_heat_balance_input::resolve(&raw, &ip)
                    .expect("actual getter operands");
                assert_eq!(projected.objects[INSIDE][0].object_name, " ");
                assert_eq!(projected.objects[OUTSIDE][0].object_name, "");
                let mut admitted =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(admitted.admissible, "{:?}", admitted.violations);
                assert!(
                    crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                        .is_none()
                );
                let environment = super::environment::environment_from_weather(
                    model,
                    &weather,
                    "embedded convection weather",
                )
                .expect("real EPW/calendar preparation");
                assert!(validate_porting_environment(&environment).is_empty());
                admitted.environment = Some(environment);
                let admission =
                    crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                        .expect("all real raw/IP/reporting guards");
                let actual = crate::support::assess_support_with_source_admission(
                    &raw,
                    &compiled.report,
                    Some(model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&admission),
                );
                assert!(actual.allows_rust_runtime(), "{:?}", actual.diagnostics);
                let legacy = assess_support(
                    &raw,
                    &compiled.report,
                    Some(model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                );
                for (code, nondefault) in [
                    (
                        "UnsupportedInsideConvectionAlgorithm",
                        expected_inside != I::Tarp,
                    ),
                    (
                        "UnsupportedOutsideConvectionAlgorithm",
                        expected_outside != O::Doe2,
                    ),
                ] {
                    assert_eq!(
                        legacy
                            .diagnostics
                            .diagnostics
                            .iter()
                            .any(|d| d.code == code),
                        nondefault
                    );
                }
                value["Zone"]["Zone"]["zone_outside_convection_algorithm"] = json!(outside);
                assert!(
                    !trace(&value, scope).admissible,
                    "authored Zone overrides stay excluded"
                );
            }
        }
        for family in [INSIDE, OUTSIDE] {
            for contents in [json!({}), json!({"":{}}), json!({" ":{"algorithm":""}})] {
                let mut value = input(scope);
                value[family] = contents;
                let raw =
                    parse_epjson_str(&value.to_string()).expect("absent/empty/default singleton");
                let compiled = compile_raw_model(&raw);
                let model = compiled.model.as_ref().expect("default model");
                assert!(trace(&value, scope).admissible);
                assert_eq!(
                    model.zones[0].inside_convection_algorithm.effective(),
                    I::Tarp
                );
                assert_eq!(
                    model.zones[0].outside_convection_algorithm.effective(),
                    O::Doe2
                );
                assert!(
                    match scope {
                        PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                        PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                    }
                    .is_ok()
                );
            }
            for invalid in [
                json!(null),
                json!(1),
                json!(false),
                json!([]),
                json!({}),
                json!(" "),
            ] {
                let mut value = input(scope);
                value[family] = json!({"":{"algorithm":invalid}});
                let raw = parse_epjson_str(&value.to_string()).expect("wrong JSON enum type/value");
                assert!(
                    match scope {
                        PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                        PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                    }
                    .is_err()
                );
            }
            let mut value = input(scope);
            value[family] = json!({"a":{},"b":{}});
            let raw = parse_epjson_str(&value.to_string()).expect("duplicate singleton");
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .is_err()
            );
            value[family] = json!({"":{"algorithm":"AdaptiveConvectionAlgorithm"}});
            assert!(!trace(&value, scope).admissible);
        }
    }
}
