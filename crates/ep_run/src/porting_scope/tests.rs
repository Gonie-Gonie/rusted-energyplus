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

    let weather = single_day_weather_fixture();
    for (scope, count, roof_type) in
        [PortingScope::A, PortingScope::B]
            .into_iter()
            .flat_map(|scope| {
                [
                    (6, "Roof"),
                    (10, "Roof"),
                    (7, "Roof"),
                    (6, "Ceiling"),
                    (10, "Ceiling"),
                    (7, "Ceiling"),
                ]
                .map(move |(count, roof_type)| (scope, count, roof_type))
            })
    {
        let mut value = input(scope);
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
        // Keep the fourth schema enum in the authored input and typed owner.
        // Only the genuine runtime getter may replace its local alpha by ROOF.
        for surface in value["BuildingSurface:Detailed"]
            .as_object_mut()
            .unwrap()
            .values_mut()
        {
            if surface["surface_type"] == "Roof" {
                surface["surface_type"] = json!(roof_type);
            }
        }
        let raw = parse_epjson_str(&value.to_string()).expect("closed authored opaque faces");
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("compiled closed opaque faces");
        let mut admitted = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
        assert!(
            admitted.admissible,
            "{scope:?}/{count}/{roof_type}: {:?}",
            admitted.violations
        );
        let input = match scope {
            PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&raw),
            PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
        }
        .expect("Source raw input");
        if scope == PortingScope::A {
            assert_eq!(
                crate::ordinary_equipment_input::resolve(&raw).is_ok(),
                roof_type == "Roof",
                "unscoped legacy admission retains its original three-class gate"
            );
        }
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
        let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &input)
            .expect("actual raw surface getter operands");
        assert_eq!(projected.surface.building_surfaces.len(), count);
        for row in &projected.surface.building_surfaces {
            let name = &row.object.object_name;
            let authored = value["BuildingSurface:Detailed"][name]["surface_type"]
                .as_str()
                .unwrap();
            assert_eq!(
                row.object.fields["surface_type"],
                ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                    authored.into()
                ),
                "the raw IP projection must retain {authored} for {name}"
            );
            let typed = model
                .surfaces
                .iter()
                .find(|surface| surface.name.0.eq_ignore_ascii_case(name))
                .unwrap();
            assert_eq!(
                Some(typed.surface_type),
                match authored {
                    "Wall" => Some(ep_model::SurfaceType::Wall),
                    "Floor" => Some(ep_model::SurfaceType::Floor),
                    "Roof" => Some(ep_model::SurfaceType::Roof),
                    "Ceiling" => Some(ep_model::SurfaceType::Ceiling),
                    _ => None,
                }
            );
        }
        let environment = super::environment::environment_from_weather(
            model,
            &weather,
            "embedded opaque enum weather",
        )
        .expect("actual Source environment");
        assert!(validate_porting_environment(&environment).is_empty());
        admitted.environment = Some(environment);
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                .expect("actual validated opaque enum Source admission");
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
        assert!(
            support.allows_rust_runtime(),
            "{scope:?}/{count}/{roof_type}: {:?}",
            support.diagnostics
        );

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
        assert!(surfaces[walls + 1..].iter().all(|s| s.surface_type
            == if roof_type == "Ceiling" {
                ep_model::SurfaceType::Ceiling
            } else {
                ep_model::SurfaceType::Roof
            }));
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
    // Two legal disconnected opaque Zones take an explicit Source-only lane.
    // No typed IDs/vectors are edited to manufacture an order discrepancy.
    let mut value = input(PortingScope::A);
    let zone = value["Zone"]["Zone"].clone();
    value["Zone"] = json!({"Zulu":zone.clone(),"Alpha":zone});
    value["Zone"]["Alpha"]["volume"] = json!(2.0);
    value["Zone"]["Alpha"]["floor_area"] = json!(2.0);
    let faces = value["BuildingSurface:Detailed"]
        .as_object()
        .unwrap()
        .clone();
    let mut surfaces = serde_json::Map::new();
    for (zone, shift, scale) in [("Zulu", 0.0, 1.0), ("Alpha", 10.0, 2.0)] {
        for (name, face) in &faces {
            let mut face = face.clone();
            face["zone_name"] = json!(zone);
            for point in face["vertices"].as_array_mut().unwrap() {
                point["vertex_x_coordinate"] =
                    json!(shift + scale * point["vertex_x_coordinate"].as_f64().unwrap());
            }
            surfaces.insert(format!("{zone}:{name}"), face);
        }
    }
    value["BuildingSurface:Detailed"] = Value::Object(surfaces);
    let raw = parse_epjson_str(&value.to_string()).expect("two disconnected authored cubes");
    let compiled = compile_raw_model(&raw);
    let model = compiled.model.as_ref().expect("two real compiled Zones");
    let mut selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
    assert!(selected.admissible, "{:?}", selected.violations);
    assert!(
        !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B).admissible
    );
    let input =
        crate::ordinary_input_processor::resolve_source_a(&raw).expect("multi-Zone Source IP");
    let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &input)
        .expect("actual Zone/Surface operands");
    assert_eq!(projected.objects["Zone"].len(), 2);
    assert_eq!(projected.surface.building_surfaces.len(), 12);
    selected.environment = Some(
        super::environment::environment_from_weather(
            model,
            &weather,
            "authored disconnected Zones",
        )
        .expect("actual multi-Zone environment"),
    );
    let admission = crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
        .expect("narrow SourceA topology admission");
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
    assert_eq!(
        support.runtime_class,
        crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
    );
    assert_eq!(
        support.runtime_class.id(),
        "source-multizone-heat-balance-compatibility"
    );
    assert_eq!(
        serde_json::to_value(support.runtime_class).expect("Source class wire"),
        json!("source-multizone-heat-balance-compatibility")
    );
    assert!(support.matched_capability_ids.is_empty());
    assert!(!support.runtime_class.conformance_promotion_allowed());
    let legacy = assess_support(
        &raw,
        &compiled.report,
        Some(model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );
    assert!(!legacy.allows_rust_runtime());
    assert!(
        legacy
            .diagnostics
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "UnsupportedTopology")
    );

    // Unit94 admits real ordered ZoneLists only on the explicit Source A
    // lane. A one-Zone list also keeps empty capabilities/nonpromotion.
    for one_zone in [false, true] {
        let mut lists = if one_zone {
            self::input(PortingScope::A)
        } else {
            value.clone()
        };
        let member_names = if one_zone {
            vec!["Zone"]
        } else {
            vec!["Zulu", "Alpha"]
        };
        lists["ZoneList"] = json!({"Members":{"zones":member_names.iter().map(|name|json!({"zone_name":name})).collect::<Vec<_>>()}});
        lists["OtherEquipment"] = json!({"List gain":{"zone_or_zonelist_or_space_or_spacelist_name":"Members",
            "schedule_name":"On","fuel_type":"Electricity","design_level":23}});
        lists["People"] = json!({"List occupants":{"zone_or_zonelist_or_space_or_spacelist_name":"Members",
            "number_of_people_schedule_name":"On","activity_level_schedule_name":"On",
            "number_of_people_calculation_method":"Area/Person","number_of_people":7,
            "people_per_floor_area":0.75,"floor_area_per_person":8}});
        let raw = parse_epjson_str(&lists.to_string()).expect("actual ordered list input");
        assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_ok());
        assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
        assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled.model.as_ref().expect("real list instances");
        assert_eq!(model.zone_lists.len(), 1);
        assert_eq!(model.other_equipment.len(), member_names.len());
        assert_eq!(model.source_other_equipment_inputs.len(), 1);
        assert_eq!(
            model.source_other_equipment_inputs[0].instances,
            0..member_names.len()
        );
        for (row, zone_name) in model.other_equipment.iter().zip(&member_names) {
            assert_eq!(Some(row.zone), model.zone_names.resolve(zone_name));
            assert_eq!(
                row.name.0,
                format!("{} LIST GAIN", zone_name.to_ascii_uppercase())
            );
        }
        assert_eq!(model.people.len(), member_names.len());
        assert_eq!(model.source_people_inputs.len(), 1);
        assert_eq!(
            model.source_people_inputs[0].instances,
            0..member_names.len()
        );
        for (row, zone_name) in model.people.iter().zip(&member_names) {
            assert_eq!(Some(row.zone), model.zone_names.resolve(zone_name));
            assert_eq!(
                row.name.0,
                format!("{} LIST OCCUPANTS", zone_name.to_ascii_uppercase())
            );
            assert_eq!(row.number_of_people, 7.0);
            assert_eq!(row.people_per_floor_area, 0.75);
            assert_eq!(row.floor_area_per_person, 8.0);
        }
        let mut reordered = model.clone();
        reordered.source_people_inputs[0].instances = 0..0;
        assert!(
            !inspect_porting_scope(&raw, &compiled.report, Some(&reordered), PortingScope::A)
                .admissible
        );
        let mut selected =
            inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(selected.admissible, "{:?}", selected.violations);
        assert!(
            !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B).admissible
        );
        selected.environment = Some(
            super::environment::environment_from_weather(
                model,
                &weather,
                "actual ordered ZoneList",
            )
            .expect("list environment"),
        );
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                .expect("Source list admission");
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
        assert_eq!(
            support.runtime_class,
            crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
        );
        assert!(support.matched_capability_ids.is_empty());
        assert!(!support.runtime_class.conformance_promotion_allowed());
    }

    // Unit97 singleton authored Spaces use the same Source-only class as
    // bounded Zones/ZoneLists. No ZoneList can hide the singleton predicate.
    // The no-Space control retains its existing one-Zone capability.
    for (authored, explicit_surfaces, list_target) in [
        (false, false, false),
        (true, false, false),
        (true, true, false),
        (true, false, true),
        (true, true, true),
    ] {
        let mut single = self::input(PortingScope::A);
        if authored {
            single["Space"] = json!({"Room97":{"zone_name":"Zone","ceiling_height":0,"volume":0}});
            if explicit_surfaces {
                for surface in single["BuildingSurface:Detailed"]
                    .as_object_mut()
                    .expect("actual singleton surfaces")
                    .values_mut()
                {
                    surface["space_name"] = json!("Room97");
                }
            }
        }
        if list_target {
            single["SpaceList"] = json!({"Only98":{"spaces":[{"space_name":"Room97"}]}});
            single["People"] = json!({"P98":{"zone_or_zonelist_or_space_or_spacelist_name":"Only98","number_of_people_schedule_name":"On","activity_level_schedule_name":"On","number_of_people":1}});
            single["OtherEquipment"] = json!({"G98":{"zone_or_zonelist_or_space_or_spacelist_name":"Only98","schedule_name":"On","design_level":7}});
        }
        let raw = parse_epjson_str(&single.to_string()).expect("actual singleton Space input");
        assert!(
            !raw.objects
                .contains_key(&ep_raw_model::ObjectType("ZoneList".into()))
        );
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled
            .model
            .as_ref()
            .expect("real singleton Source model");
        assert_eq!(model.zones.len(), 1);
        assert_eq!(model.spaces.len(), 1);
        assert_eq!(
            model.spaces[0].origin,
            if authored {
                ep_model::SpaceOrigin::Authored
            } else {
                ep_model::SpaceOrigin::AutoZoneDefault
            }
        );
        if list_target {
            assert_eq!(model.space_lists.len(), 1);
            assert_eq!(model.people[0].name.0, "ROOM97 P98");
            assert_eq!(model.other_equipment[0].name.0, "ROOM97 G98");
            assert!(
                matches!(model.source_people_inputs[0].target,ep_model::SourcePeopleInputTarget::SpaceList(id) if id==model.space_lists[0].id)
            );
            assert!(
                matches!(model.source_other_equipment_inputs[0].target,ep_model::SourceOtherEquipmentInputTarget::SpaceList(id) if id==model.space_lists[0].id)
            );
        }
        let mut selected =
            inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(selected.admissible, "{:?}", selected.violations);
        selected.environment = Some(
            super::environment::environment_from_weather(
                model,
                &weather,
                "actual singleton whole-Zone Space",
            )
            .expect("same genuine singleton environment preparation"),
        );
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                .expect("validated singleton Source A scope");
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
        assert_eq!(support.status, SupportStatus::SupportedCompatibility);
        assert!(support.failed_capability_ids.is_empty());
        if authored {
            assert_eq!(
                support.runtime_class,
                crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
            );
            assert!(support.matched_capability_ids.is_empty());
            assert!(support.matched_capabilities.is_empty());
            assert!(!support.runtime_class.conformance_promotion_allowed());
            assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
            assert!(
                !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B)
                    .admissible
            );
            let legacy = assess_support(
                &raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
            );
            assert!(!legacy.allows_rust_runtime());
            assert!(
                legacy
                    .diagnostics
                    .diagnostics
                    .iter()
                    .any(|diagnostic| { diagnostic.code == "UnsupportedSpacePartitioning" })
            );
        } else {
            assert_eq!(
                support.runtime_class,
                crate::RuntimeClass::OneZoneHeatBalanceCompatibility
            );
            assert_eq!(
                support.matched_capability_ids,
                ["official_1zone_uncontrolled_declared_heat_balance"]
            );
            assert!(support.runtime_class.conformance_promotion_allowed());
        }
    }

    // Unit99: positive raw SpaceList objects select the finite Source class
    // even when both lists have no members and every Space is implicit.
    for mixed in [false, true] {
        let mut empty_case = self::input(PortingScope::A);
        empty_case["SpaceList"] = json!({"Absent99":{},"Empty99":{"spaces":[]}});
        let schedule = if mixed { "On" } else { "Missing" };
        empty_case["People"] = json!({
            "A empty":{"zone_or_zonelist_or_space_or_spacelist_name":"Absent99","number_of_people_schedule_name":schedule,"activity_level_schedule_name":"Missing","number_of_people":2},
            "C empty":{"zone_or_zonelist_or_space_or_spacelist_name":"Empty99","number_of_people_schedule_name":schedule,"activity_level_schedule_name":"","number_of_people_calculation_method":"Area/Person","floor_area_per_person":0}
        });
        empty_case["OtherEquipment"] = json!({
            "A empty":{"zone_or_zonelist_or_space_or_spacelist_name":"Absent99","schedule_name":schedule,"design_level_calculation_method":"Watts/Area","power_per_floor_area":-2,"fraction_latent":0.75,"fraction_radiant":0.75},
            "C empty":{"zone_or_zonelist_or_space_or_spacelist_name":"Empty99","schedule_name":schedule,"design_level":-3}
        });
        if mixed {
            empty_case["People"]["B actual"] = json!({"zone_or_zonelist_or_space_or_spacelist_name":"Zone","number_of_people_schedule_name":"On","activity_level_schedule_name":"On","number_of_people":1});
            empty_case["OtherEquipment"]["B actual"] = json!({"zone_or_zonelist_or_space_or_spacelist_name":"Zone","schedule_name":"On","design_level":5});
        }
        let raw = parse_epjson_str(&empty_case.to_string()).expect("real empty SpaceLists");
        assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_ok());
        assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
        assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled.model.as_ref().expect("real empty input bindings");
        assert!(model.authored_space_names.is_empty());
        assert_eq!(model.spaces.len(), 1);
        assert_eq!(
            model.spaces[0].origin,
            ep_model::SpaceOrigin::AutoZoneDefault
        );
        assert_eq!(model.people.len(), usize::from(mixed));
        assert_eq!(model.other_equipment.len(), usize::from(mixed));
        let expected = if mixed {
            vec![0..0, 0..1, 1..1]
        } else {
            vec![0..0, 0..0]
        };
        assert_eq!(
            model
                .source_people_inputs
                .iter()
                .map(|row| row.instances.clone())
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            model
                .source_other_equipment_inputs
                .iter()
                .map(|row| row.instances.clone())
                .collect::<Vec<_>>(),
            expected
        );
        for mutation in 0..6 {
            let mut invalid = model.clone();
            match mutation {
                0 => {
                    invalid.source_people_inputs.remove(0);
                }
                1 => {
                    invalid.source_other_equipment_inputs[0].target =
                        ep_model::SourceGainInputTarget::Zone(model.zones[0].id);
                }
                2 => {
                    invalid.source_people_inputs[0].instances = 1..1;
                }
                3 => {
                    invalid.source_people_inputs[0].target =
                        ep_model::SourceGainInputTarget::SpaceList(
                            model
                                .space_list_names
                                .resolve("Empty99")
                                .expect("other real empty list"),
                        );
                }
                4 => {
                    invalid.space_lists[0].spaces.push(model.spaces[0].id);
                }
                _ => {
                    invalid.source_people_inputs[0].instances =
                        std::ops::Range { start: 1, end: 0 };
                }
            }
            assert!(
                !inspect_porting_scope(&raw, &compiled.report, Some(&invalid), PortingScope::A)
                    .admissible,
                "empty range association/coverage mutation {mutation}"
            );
        }
        let mut selected =
            inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(selected.admissible, "{:?}", selected.violations);
        assert!(
            !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B).admissible
        );
        selected.environment = Some(
            super::environment::environment_from_weather(
                model,
                &weather,
                "real empty SpaceList scope",
            )
            .expect("same one-Zone weather preparation"),
        );
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                .expect("validated empty SpaceList input");
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
        assert_eq!(
            support.runtime_class,
            crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
        );
        assert!(support.matched_capability_ids.is_empty());
        assert!(!support.runtime_class.conformance_promotion_allowed());
        assert!(
            !assess_support(
                &raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal
            )
            .allows_rust_runtime()
        );
    }

    // Unit100: only a genuine Source A group can supply a list factor. The
    // unused earlier list proves Zone.ListGroup is a List ID, not a Group ID.
    for one_zone in [false, true] {
        for multiplier in [None, Some(2_u32), Some(i32::MAX as u32)] {
            let mut group_case = if one_zone {
                self::input(PortingScope::A)
            } else {
                value.clone()
            };
            let members = if one_zone {
                vec!["Zone"]
            } else {
                vec!["Zulu", "Alpha"]
            };
            group_case["ZoneList"] = json!({
                "A unused":{"zones":[{"zone_name":members[0]}]},
                "Both":{"zones":members.iter().map(|name| json!({"zone_name":name})).collect::<Vec<_>>()}
            });
            group_case["ZoneGroup"] =
                json!({" G ":{"zone_list_name":"Both","metadata":[null,true]}});
            if let Some(multiplier) = multiplier {
                group_case["ZoneGroup"][" G "]["zone_list_multiplier"] = json!(multiplier);
            }
            group_case["People"] = json!({"P100":{"zone_or_zonelist_or_space_or_spacelist_name":"Both","number_of_people_schedule_name":"On","activity_level_schedule_name":"On","number_of_people":2}});
            group_case["OtherEquipment"] = json!({"G100":{"zone_or_zonelist_or_space_or_spacelist_name":"Both","schedule_name":"On","fuel_type":"Electricity","design_level":23}});
            let raw = parse_epjson_str(&group_case.to_string()).expect("actual ZoneGroup input");
            assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_ok());
            assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
            let compiled = ep_compiler::compile_raw_model_for_source(&raw);
            assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
            let model = compiled
                .model
                .as_ref()
                .expect("actual typed group membership");
            assert_eq!(model.zone_groups.len(), 1);
            let group = &model.zone_groups[0];
            assert_eq!(group.id.0, 0);
            assert_eq!(group.zone_list.0, 1);
            assert_ne!(group.id.0, group.zone_list.0);
            assert_eq!(group.name.0, " G ");
            assert_eq!(group.multiplier, multiplier.unwrap_or(1));
            assert!(model.zone_group_names.resolve("G").is_none());
            assert!(model.authored_space_names.is_empty());
            for zone in &model.zones {
                assert_eq!(zone.list_group, Some(group.zone_list));
                assert_eq!(zone.list_multiplier, multiplier.unwrap_or(1));
            }
            let mut selected =
                inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
            assert!(selected.admissible, "{:?}", selected.violations);
            assert!(
                !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B)
                    .admissible
            );
            for mutation in 0..4 {
                let mut invalid = model.clone();
                match mutation {
                    0 => invalid.zones[0].list_group = Some(ep_model::ZoneListId(group.id.0)),
                    1 => {
                        invalid.zones[0].list_multiplier = if group.multiplier == 1 { 2 } else { 1 }
                    }
                    2 => invalid.zone_groups[0].zone_list = ep_model::ZoneListId(0),
                    _ => {
                        invalid.zone_groups.clear();
                    }
                }
                assert!(
                    !inspect_porting_scope(&raw, &compiled.report, Some(&invalid), PortingScope::A)
                        .admissible,
                    "Group/List identity and factor mutation {mutation}"
                );
            }
            selected.environment = Some(
                super::environment::environment_from_weather(
                    model,
                    &weather,
                    "actual ZoneGroup Source input",
                )
                .expect("unchanged genuine weather preparation"),
            );
            let admission =
                crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                    .expect("validated Source A grouped input");
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
            assert_eq!(
                support.runtime_class,
                crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
            );
            assert!(support.matched_capability_ids.is_empty());
            assert!(!support.runtime_class.conformance_promotion_allowed());
            assert!(
                !assess_support(
                    &raw,
                    &compiled.report,
                    Some(model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal
                )
                .allows_rust_runtime()
            );
            let mut overflow = group_case.clone();
            overflow["Zone"][members[0]]["multiplier"] = json!(2);
            overflow["ZoneGroup"][" G "]["zone_list_multiplier"] = json!(i32::MAX);
            let overflow_raw =
                parse_epjson_str(&overflow.to_string()).expect("defined JSON integer factors");
            assert!(crate::ordinary_equipment_input::resolve_source_a(&overflow_raw).is_err());
            let overflow = ep_compiler::compile_raw_model_for_source(&overflow_raw);
            assert!(
                !inspect_porting_scope(
                    &overflow_raw,
                    &overflow.report,
                    overflow.model.as_ref(),
                    PortingScope::A
                )
                .admissible,
                "a product beyond signed Native int stays outside Source scope"
            );
        }
    }

    // Unit88 admits only direct-Zone Electricity OtherEquipment on this
    // Source topology. Actual compiler/IP names link opposite object/Zone order;
    // no typed IDs, rates or thermal outcomes are synthesized for this test.
    let mut gain_case = value.clone();
    gain_case["OtherEquipment"] = json!({
        "A gain":{"zone_or_zonelist_or_space_or_spacelist_name":"Zulu","schedule_name":"On",
            "fuel_type":"Electricity","design_level_calculation_method":"EquipmentLevel",
            "design_level":100,"fraction_radiant":0.25},
        "Z gain":{"zone_or_zonelist_or_space_or_spacelist_name":"Alpha","schedule_name":"On",
            "fuel_type":"Electricity","design_level_calculation_method":"EquipmentLevel",
            "design_level":300,"fraction_radiant":0.25}
    });
    let gain_raw =
        parse_epjson_str(&gain_case.to_string()).expect("multi-Zone electric gain raw input");
    let gain_ip = crate::ordinary_input_processor::resolve_source_a(&gain_raw)
        .expect("actual multi-Zone electric gain input processor");
    let gain_input = crate::ordinary_heat_balance_input::resolve_source(&gain_raw, &gain_ip)
        .expect("actual gain getter operands");
    assert_eq!(gain_input.objects["OtherEquipment"].len(), 2);
    let gain_compiled = compile_raw_model(&gain_raw);
    let gain_model = gain_compiled
        .model
        .as_ref()
        .expect("actual typed multi-Zone electric gains");
    assert_eq!(gain_model.other_equipment.len(), 2);
    for (name, target, power) in [
        ("A gain", "Zulu", 100.0_f64),
        ("Z gain", "Alpha", 300.0_f64),
    ] {
        let raw = gain_input.objects["OtherEquipment"]
            .iter()
            .find(|row| row.object_name == name)
            .expect("actual raw gain row");
        assert_eq!(
            raw.fields["zone_or_zonelist_or_space_or_spacelist_name"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                target.into()
            )
        );
        let typed = gain_model
            .other_equipment
            .iter()
            .find(|row| row.name == ep_model::NormalizedName::new(name))
            .expect("actual typed gain row");
        let zone = gain_model
            .zones
            .iter()
            .find(|row| row.name == ep_model::NormalizedName::new(target))
            .expect("actual target Zone");
        assert_eq!(typed.zone, zone.id);
        assert_eq!(
            typed.fuel_type,
            ep_model::NormalizedName::new("Electricity")
        );
        assert_eq!(typed.design_level_w.to_bits(), power.to_bits());
        assert_eq!(typed.fraction_radiant.to_bits(), 0.25_f64.to_bits());
    }
    let mut gain_scope = inspect_porting_scope(
        &gain_raw,
        &gain_compiled.report,
        Some(gain_model),
        PortingScope::A,
    );
    assert!(gain_scope.admissible, "{:?}", gain_scope.violations);
    gain_scope.environment = Some(
        super::environment::environment_from_weather(
            gain_model,
            &weather,
            "authored multi-Zone electric gains",
        )
        .expect("genuine gain environment"),
    );
    let gain_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&gain_raw, &gain_scope)
            .expect("validated Source gain admission");
    let gain_support = crate::support::assess_support_with_source_admission(
        &gain_raw,
        &gain_compiled.report,
        Some(gain_model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&gain_admission),
    );
    assert!(
        gain_support.allows_rust_runtime(),
        "{:?}",
        gain_support.diagnostics
    );
    assert_eq!(
        gain_support.runtime_class,
        crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
    );
    assert!(gain_support.matched_capability_ids.is_empty());
    assert!(!gain_support.runtime_class.conformance_promotion_allowed());
    assert!(crate::ordinary_equipment_input::resolve_source_b(&gain_raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve(&gain_raw).is_err());
    assert!(
        !inspect_porting_scope(
            &gain_raw,
            &gain_compiled.report,
            Some(gain_model),
            PortingScope::B
        )
        .admissible
    );
    // Unit91: the three existing People sizing methods retain actual direct
    // Zone bindings on the same Source-only topology. This is raw/IP/typed
    // admission evidence; the runtime getter owns computed design occupants.
    for (method, field, typed_method) in [
        (
            "People",
            "number_of_people",
            ep_model::PeopleNumberCalculationMethod::People,
        ),
        (
            "People/Area",
            "people_per_floor_area",
            ep_model::PeopleNumberCalculationMethod::PeoplePerArea,
        ),
        (
            "Area/Person",
            "floor_area_per_person",
            ep_model::PeopleNumberCalculationMethod::AreaPerPerson,
        ),
    ] {
        let mut people_case = value.clone();
        people_case["Schedule:Constant"]["Activity"] = json!({"hourly_value":120});
        people_case["People"] = json!({
            "A occupants":{"zone_or_zonelist_or_space_or_spacelist_name":"Zulu",
                "number_of_people_schedule_name":"On","activity_level_schedule_name":"Activity",
                "number_of_people_calculation_method":method,field:1.5,
                "fraction_radiant":0.3,"sensible_heat_fraction":0.6},
            "Z occupants":{"zone_or_zonelist_or_space_or_spacelist_name":"Alpha",
                "number_of_people_schedule_name":"On","activity_level_schedule_name":"Activity",
                "number_of_people_calculation_method":method,field:3.0,
                "fraction_radiant":0.3,"sensible_heat_fraction":0.6}
        });
        let people_raw = parse_epjson_str(&people_case.to_string())
            .expect("actual two-Zone direct People raw input");
        let people_ip = crate::ordinary_input_processor::resolve_source_a(&people_raw)
            .expect("actual two-Zone People input processor");
        let people_input =
            crate::ordinary_heat_balance_input::resolve_source(&people_raw, &people_ip)
                .expect("unchanged actual People getter operands");
        let people_compiled = compile_raw_model(&people_raw);
        let people_model = people_compiled
            .model
            .as_ref()
            .expect("typed two-Zone People");
        assert_eq!(people_model.people.len(), 2);
        assert_eq!(people_input.objects["People"].len(), 2);
        for (name, target) in [("A occupants", "Zulu"), ("Z occupants", "Alpha")] {
            let row = people_model
                .people
                .iter()
                .find(|row| row.name == ep_model::NormalizedName::new(name))
                .expect("actual typed People identity");
            let zone = people_model
                .zones
                .iter()
                .find(|row| row.name == ep_model::NormalizedName::new(target))
                .expect("actual typed target Zone");
            assert_eq!(row.zone, zone.id);
            assert_eq!(row.number_of_people_calculation_method, typed_method);
            let input = people_input.objects["People"]
                .iter()
                .find(|row| row.object_name == name)
                .expect("actual People raw projection");
            assert!(!input.fields.contains_key("carbon_dioxide_generation_rate"));
        }
        let mut people_scope = inspect_porting_scope(
            &people_raw,
            &people_compiled.report,
            Some(people_model),
            PortingScope::A,
        );
        assert!(people_scope.admissible, "{:?}", people_scope.violations);
        people_scope.environment = Some(
            super::environment::environment_from_weather(
                people_model,
                &weather,
                "authored two-Zone People",
            )
            .expect("real People environment"),
        );
        let admission = crate::support::SourceSupportAdmission::from_validated_scope(
            &people_raw,
            &people_scope,
        )
        .expect("validated People Source scope");
        let supported = crate::support::assess_support_with_source_admission(
            &people_raw,
            &people_compiled.report,
            Some(people_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&admission),
        );
        assert!(
            supported.allows_rust_runtime(),
            "{:?}",
            supported.diagnostics
        );
        assert_eq!(
            supported.runtime_class,
            crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
        );
        assert!(supported.matched_capability_ids.is_empty());
        assert!(!supported.runtime_class.conformance_promotion_allowed());
        assert!(crate::ordinary_equipment_input::resolve_source_b(&people_raw).is_err());
        assert!(crate::ordinary_equipment_input::resolve(&people_raw).is_err());
        assert!(
            !inspect_porting_scope(
                &people_raw,
                &people_compiled.report,
                Some(people_model),
                PortingScope::B
            )
            .admissible
        );
    }
    // Actual compiler and raw-to-IP projection already share each format's Source
    // order. Legal reverse-lexical IDF declarations prove that fact, without
    // renumbering IDs or reordering the compiled vectors for this test.
    let converted = r#"{"Zone":{"Alpha":{},"Zulu":{}}}"#;
    for staged in [false, true] {
        let raw = if staged {
            ep_raw_model::parse_epjson_str_with_idf_order(converted, "Zone,Zulu;Zone,Alpha;")
        } else {
            parse_epjson_str(converted)
        }
        .expect("actual direct/staged Zone input");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("actual compiler Zone order");
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)
            .expect("actual ordered Source IP");
        let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
            .expect("actual ordered Zone projection");
        let expected = if staged {
            ["Zulu", "Alpha"]
        } else {
            ["Alpha", "Zulu"]
        };
        assert_eq!(
            projected.objects["Zone"]
                .iter()
                .map(|row| row.object_name.as_str())
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            model
                .zones
                .iter()
                .map(|row| row.name.clone())
                .collect::<Vec<_>>(),
            expected.map(ep_model::NormalizedName::new)
        );
        for zone in &model.zones {
            let spaces: Vec<_> = model
                .spaces
                .iter()
                .filter(|space| space.zone == zone.id)
                .collect();
            assert_eq!(spaces.len(), 1);
            assert_eq!(spaces[0].origin, ep_model::SpaceOrigin::AutoZoneDefault);
            assert_eq!(zone.spaces.as_slice(), [spaces[0].id]);
        }
    }
    // Existing two-cube inputs, now touching at x=1. Only the named Wall pair
    // changes boundary: actual compiler operands remain the source of this
    // admission check, never a producer of runtime geometry or coupling state.
    let mut paired = value.clone();
    for face in paired["BuildingSurface:Detailed"]
        .as_object_mut()
        .unwrap()
        .values_mut()
    {
        if face["zone_name"] == "Alpha" {
            for point in face["vertices"].as_array_mut().unwrap() {
                point["vertex_x_coordinate"] =
                    json!(point["vertex_x_coordinate"].as_f64().unwrap() - 9.0);
            }
        }
    }
    for (name, other) in [("Zulu:East", "Alpha:West"), ("Alpha:West", "Zulu:East")] {
        paired["BuildingSurface:Detailed"][name]["outside_boundary_condition"] = json!("Surface");
        paired["BuildingSurface:Detailed"][name]["outside_boundary_condition_object"] =
            json!(other);
    }
    let raw =
        parse_epjson_str(&paired.to_string()).expect("explicit reciprocal paired Wall inputs");
    let ip =
        crate::ordinary_input_processor::resolve_source_a(&raw).expect("real paired Source IP");
    assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
        .expect("retained paired raw alpha operands");
    for (name, target) in [("Zulu:East", "Alpha:West"), ("Alpha:West", "Zulu:East")] {
        let row = projected
            .surface
            .building_surfaces
            .iter()
            .find(|row| row.object.object_name == name)
            .expect("real raw paired row");
        assert_eq!(
            row.object.fields["outside_boundary_condition"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                "Surface".into()
            )
        );
        assert_eq!(
            row.object.fields["outside_boundary_condition_object"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                target.into()
            )
        );
    }
    let compiled = compile_raw_model(&raw);
    let model = compiled.model.as_ref().expect("actual typed paired Walls");
    let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
    assert!(selected.admissible, "{:?}", selected.violations);
    assert_eq!(
        model
            .surfaces
            .iter()
            .filter(|s| s.outside_boundary_condition == ep_model::OutsideBoundaryCondition::Surface)
            .count(),
        2
    );
    // The existing reciprocal-pair guard compares independent Zone and list
    // factors. Grouping both Zones equally is legal; unequal group factors are not.
    for same_factor in [false, true] {
        let mut grouped_pair = paired.clone();
        grouped_pair["ZoneList"] = json!({
            "Alpha list":{"zones":[{"zone_name":"Alpha"}]},
            "Zulu list":{"zones":[{"zone_name":"Zulu"}]}
        });
        grouped_pair["ZoneGroup"] = json!({
            "Alpha group":{"zone_list_name":"Alpha list","zone_list_multiplier":2},
            "Zulu group":{"zone_list_name":"Zulu list","zone_list_multiplier":if same_factor {2} else {3}}
        });
        let raw =
            parse_epjson_str(&grouped_pair.to_string()).expect("genuine paired group factors");
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let grouped = inspect_porting_scope(
            &raw,
            &compiled.report,
            compiled.model.as_ref(),
            PortingScope::A,
        );
        assert_eq!(grouped.admissible, same_factor, "{:?}", grouped.violations);
        if !same_factor {
            assert!(
                grouped
                    .violations
                    .iter()
                    .any(|message| message.contains("equal Zone factors"))
            );
        }
    }
    for (field, invalid) in [
        ("outside_boundary_condition_object", json!("Missing")),
        ("zone_name", json!("Zulu")),
        ("construction_name", json!("Other")),
        ("surface_type", json!("Window")),
        ("sun_exposure", json!(null)),
        ("wind_exposure", json!(false)),
        ("sun_exposure", json!(" ")),
        ("wind_exposure", json!("Unknown")),
    ] {
        let mut bad = paired.clone();
        bad["BuildingSurface:Detailed"]["Alpha:West"][field] = invalid;
        let raw = parse_epjson_str(&bad.to_string()).expect("authored invalid paired subset");
        assert!(
            crate::ordinary_equipment_input::resolve_source_a(&raw).is_err(),
            "paired {field} escaped"
        );
    }
    // Unit87 adds normal horizontal opaque pairs without a class-equality
    // rule. The typed model keeps authored Ceiling; only the genuine Source
    // getter may alias that alpha to ROOF. These are admission operands, not
    // reconstructed runtime Surface owners or physical result expectations.
    let mut horizontal = paired.clone();
    let zone = horizontal["Zone"]["Zulu"].clone();
    horizontal["Zone"] = json!({"Zulu":zone.clone(),"Alpha":zone});
    horizontal["Zone"]["Alpha"]["volume"] = json!(2.0);
    let mut horizontal_surfaces = serde_json::Map::new();
    for (zone, offset, height) in [("Zulu", 0.0, 1.0), ("Alpha", 1.0, 2.0)] {
        for (name, original) in &faces {
            let mut face = original.clone();
            face["zone_name"] = json!(zone);
            for point in face["vertices"].as_array_mut().unwrap() {
                point["vertex_z_coordinate"] =
                    json!(offset + height * point["vertex_z_coordinate"].as_f64().unwrap());
            }
            horizontal_surfaces.insert(format!("{zone}:{name}"), face);
        }
    }
    horizontal["BuildingSurface:Detailed"] = Value::Object(horizontal_surfaces);
    for (name, target) in [("Zulu:Roof", "Alpha:Floor"), ("Alpha:Floor", "Zulu:Roof")] {
        horizontal["BuildingSurface:Detailed"][name]["outside_boundary_condition"] =
            json!("Surface");
        horizontal["BuildingSurface:Detailed"][name]["outside_boundary_condition_object"] =
            json!(target);
    }
    for (lower, upper) in [
        ("Roof", "Floor"),
        ("Floor", "Roof"),
        ("Ceiling", "Floor"),
        ("Floor", "Ceiling"),
    ] {
        let mut classes = horizontal.clone();
        classes["BuildingSurface:Detailed"]["Zulu:Roof"]["surface_type"] = json!(lower);
        classes["BuildingSurface:Detailed"]["Alpha:Floor"]["surface_type"] = json!(upper);
        let raw = parse_epjson_str(&classes.to_string()).expect("normal horizontal pair classes");
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)
            .expect("normal opaque class pair Source input");
        let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
            .expect("authored horizontal class operands");
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("actual typed horizontal pair");
        for (name, kind) in [("Zulu:Roof", lower), ("Alpha:Floor", upper)] {
            let row = projected
                .surface
                .building_surfaces
                .iter()
                .find(|row| row.object.object_name == name)
                .expect("raw horizontal class row");
            assert_eq!(
                row.object.fields["surface_type"],
                ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                    kind.into()
                )
            );
            let typed = model
                .surfaces
                .iter()
                .find(|row| row.name == ep_model::NormalizedName::new(name))
                .expect("typed authored class");
            let expected = match kind {
                "Floor" => ep_model::SurfaceType::Floor,
                "Roof" => ep_model::SurfaceType::Roof,
                _ => {
                    assert_eq!(kind, "Ceiling");
                    ep_model::SurfaceType::Ceiling
                }
            };
            assert_eq!(typed.surface_type, expected);
            assert_eq!(
                typed.outside_boundary_condition,
                ep_model::OutsideBoundaryCondition::Surface
            );
        }
        let mut typed_violations = Vec::new();
        super::surface_pairs::validate(model, PortingScope::A, &mut typed_violations);
        assert!(
            typed_violations.is_empty(),
            "{lower}/{upper}: {typed_violations:?}"
        );
        let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(
            selected.admissible,
            "{lower}/{upper}: {:?}",
            selected.violations
        );
        assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
        assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    }
    // Other Native-valid class combinations have their own literal warning
    // branches. They remain separate from this warning-free Roof/Floor lane.
    for (lower, upper) in [
        ("Roof", "Roof"),
        ("Floor", "Floor"),
        ("Wall", "Roof"),
        ("Floor", "Wall"),
    ] {
        let mut invalid = horizontal.clone();
        invalid["BuildingSurface:Detailed"]["Zulu:Roof"]["surface_type"] = json!(lower);
        invalid["BuildingSurface:Detailed"]["Alpha:Floor"]["surface_type"] = json!(upper);
        let raw = parse_epjson_str(&invalid.to_string()).expect("held class-warning input");
        assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("typed class-warning operands");
        let mut typed_violations = Vec::new();
        super::surface_pairs::validate(model, PortingScope::A, &mut typed_violations);
        assert!(
            !typed_violations.is_empty(),
            "{lower}/{upper} class-warning lane escaped"
        );
    }
    // The paired and ordinary Outdoors faces retain authored absence/empty
    // strings through raw/IP projection. Only the actual getter supplies the
    // schema defaults; typed input remains the authored exposure selection.
    for sun in [None, Some(""), Some("NoSun"), Some("SunExposed")] {
        for wind in [None, Some(""), Some("NoWind"), Some("WindExposed")] {
            let mut exposures = paired.clone();
            for name in ["Alpha:West", "Zulu:West"] {
                let face = exposures["BuildingSurface:Detailed"][name]
                    .as_object_mut()
                    .unwrap();
                for (field, value) in [("sun_exposure", sun), ("wind_exposure", wind)] {
                    if let Some(value) = value {
                        face.insert(field.into(), json!(value));
                    } else {
                        face.remove(field);
                    }
                }
            }
            let raw = parse_epjson_str(&exposures.to_string())
                .expect("schema-valid exposure presence/default matrix");
            let ip = crate::ordinary_input_processor::resolve_source_a(&raw)
                .expect("paired exposure schema values reach actual Source IP");
            let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
                .expect("unaltered actual raw exposure operands");
            let compiled = compile_raw_model(&raw);
            let model = compiled.model.as_ref().expect("typed exposure defaults");
            for name in ["Alpha:West", "Zulu:West"] {
                let row = projected
                    .surface
                    .building_surfaces
                    .iter()
                    .find(|row| row.object.object_name == name)
                    .unwrap();
                for (field, value) in [("sun_exposure", sun), ("wind_exposure", wind)] {
                    assert_eq!(
                        row.object.fields.get(field),
                        value.map(|value| ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(value.into())).as_ref(),
                        "raw/IP authored presence for {name}.{field}"
                    );
                }
                let typed = model
                    .surfaces
                    .iter()
                    .find(|row| row.name.0.eq_ignore_ascii_case(name))
                    .unwrap();
                assert_eq!(
                    typed.sun_exposure == ep_model::SunExposure::SunExposed,
                    sun != Some("NoSun")
                );
                assert_eq!(
                    typed.wind_exposure == ep_model::WindExposure::WindExposed,
                    wind != Some("NoWind")
                );
            }
            let selected =
                inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
            assert!(selected.admissible, "{:?}", selected.violations);
            assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        }
    }
    // Exact reverse identities keep their existing fast path. Normal mismatches
    // below are structural candidates only; real Source owners decide the U gate.
    // The compiler creates all natural Material/Construction IDs; no vectors or
    // IDs are rearranged to manufacture a Source-order mismatch.
    let material_names = [
        "STACK A", "STACK B", "STACK C", "STACK D", "STACK E", "STACK F", "STACK G", "STACK H",
        "STACK I", "STACK J",
    ];
    let construction_value = |names: &[&str]| {
        Value::Object(
            names
                .iter()
                .enumerate()
                .map(|(index, name)| {
                    (
                        if index == 0 {
                            "outside_layer".into()
                        } else {
                            format!("layer_{}", index + 1)
                        },
                        json!(name),
                    )
                })
                .collect(),
        )
    };
    let mut stacks = paired.clone();
    let material = stacks["Material:NoMass"]["Opaque"].clone();
    for name in material_names {
        // Identical properties do not merge actual typed Material identities;
        // a differing normal identity reaches only the later Source U/warning lane.
        stacks["Material:NoMass"][name] = material.clone();
    }
    for (count, same_construction) in [(1, false), (2, false), (3, false), (10, false), (3, true)] {
        let names = if same_construction {
            vec![material_names[0], material_names[1], material_names[0]]
        } else {
            material_names[..count].to_vec()
        };
        let reversed = names.iter().rev().copied().collect::<Vec<_>>();
        let mut value = stacks.clone();
        value["Construction"]["STACK FORWARD"] = construction_value(&names);
        value["Construction"]["STACK REVERSE"] = construction_value(&reversed);
        value["BuildingSurface:Detailed"]["Zulu:East"]["construction_name"] =
            json!("STACK FORWARD");
        value["BuildingSurface:Detailed"]["Alpha:West"]["construction_name"] =
            json!(if same_construction {
                "STACK FORWARD"
            } else {
                "STACK REVERSE"
            });
        let raw =
            parse_epjson_str(&value.to_string()).expect("actual reversed opaque input layers");
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)
            .expect("exact reversed stacks reach real Source IP");
        assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
        assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
            .expect("authored Construction layer operands remain unchanged");
        for (name, expected) in [("STACK FORWARD", &names), ("STACK REVERSE", &reversed)] {
            let row = projected.objects["Construction"]
                .iter()
                .find(|row| row.object_name == name)
                .unwrap();
            for (index, material) in expected.iter().enumerate() {
                let field = if index == 0 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", index + 1)
                };
                assert_eq!(
                    row.fields[&field],
                    ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                        (*material).into()
                    )
                );
            }
        }
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("real typed opaque reverse stack owners");
        let faces = ["Zulu:East", "Alpha:West"].map(|name| {
            model
                .surfaces
                .iter()
                .find(|surface| surface.name.0.eq_ignore_ascii_case(name))
                .unwrap()
        });
        let constructors = faces.map(|face| {
            model
                .constructions
                .iter()
                .find(|construction| construction.id == face.construction)
                .unwrap()
        });
        assert_eq!(constructors[0].effective_layers().len(), count);
        assert!(
            constructors[0]
                .effective_layers()
                .iter()
                .eq(constructors[1].effective_layers().iter().rev())
        );
        assert_eq!(constructors[0].id == constructors[1].id, same_construction);
        let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(selected.admissible, "{:?}", selected.violations);
    }
    let mut invalid_stacks = stacks;
    invalid_stacks["Construction"]["STACK FORWARD"] = construction_value(&material_names[..2]);
    invalid_stacks["BuildingSurface:Detailed"]["Zulu:East"]["construction_name"] =
        json!("STACK FORWARD");
    invalid_stacks["BuildingSurface:Detailed"]["Alpha:West"]["construction_name"] =
        json!("STACK REVERSE");
    for (layers, structural_candidate) in [
        (construction_value(&material_names[..2]), true), // same order, equal nominal U
        (construction_value(&material_names[..3]), true), // unequal count
        (
            construction_value(&[material_names[1], material_names[2]]),
            true,
        ), // equal properties, different opaque ID
        (
            json!({"outside_layer":material_names[0],"layer_3":material_names[1]}),
            false,
        ), // actual gap
    ] {
        let mut value = invalid_stacks.clone();
        value["Construction"]["STACK REVERSE"] = layers;
        let raw =
            parse_epjson_str(&value.to_string()).expect("preserved opaque mismatch and gap inputs");
        let compiled = compile_raw_model(&raw);
        if structural_candidate {
            // These normal stacks are raw/IP/typed candidates, not a claim
            // about missing runtime NominalU or future Warning/Continue effects.
            let model = compiled
                .model
                .as_ref()
                .expect("valid normal stack candidate");
            for name in ["STACK FORWARD", "STACK REVERSE"] {
                let construction = model
                    .constructions
                    .iter()
                    .find(|row| row.name.0.eq_ignore_ascii_case(name))
                    .expect("actual normal Construction identity");
                assert!(!construction.effective_layers().is_empty());
            }
            assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_ok());
            let selected =
                inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
            assert!(selected.admissible, "{:?}", selected.violations);
            assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        } else {
            assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
            assert!(!trace(&value, PortingScope::A).admissible);
        }
    }
    let mut nonpalindrome = invalid_stacks;
    nonpalindrome["BuildingSurface:Detailed"]["Alpha:West"]["construction_name"] =
        json!("STACK FORWARD");
    let raw = parse_epjson_str(&nonpalindrome.to_string())
        .expect("same nonpalindromic constructor remains an authored mismatch");
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_ok());
    assert!(trace(&nonpalindrome, PortingScope::A).admissible);
    assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    // A separately named, sole-layer normal alias proves actual raw/IP/typed
    // identity custody. Its equal properties are authored input only: no Source
    // MaterialGroup, NominalU, diagnostic or physical result is fabricated here.
    let mut similar = paired.clone();
    similar["Material:NoMass"]["OPAQUE ALIAS86"] = similar["Material:NoMass"]["Opaque"].clone();
    similar["Construction"]["ALIAS LINK86"] = json!({"outside_layer":"OPAQUE ALIAS86"});
    similar["BuildingSurface:Detailed"]["Alpha:West"]["construction_name"] = json!("ALIAS LINK86");
    let raw = parse_epjson_str(&similar.to_string()).expect("authored distinct normal alias input");
    let ip = crate::ordinary_input_processor::resolve_source_a(&raw)
        .expect("normal differing-identity structural Source IP candidate");
    let materials = crate::ordinary_material_input::resolve_source(&raw, &ip)
        .expect("genuine raw/IP normal Material custody");
    let inputs = ["Opaque", "OPAQUE ALIAS86"].map(|name| {
        materials
            .no_mass
            .iter()
            .find(|row| row.object_name == name)
            .expect("actual distinct named raw Material object")
    });
    assert_ne!(inputs[0].object_name, inputs[1].object_name);
    assert_eq!(inputs[0].fields, inputs[1].fields);
    let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
        .expect("actual alias Construction projection");
    assert_eq!(
        projected.objects["Construction"]
            .iter()
            .find(|row| row.object_name == "ALIAS LINK86")
            .unwrap()
            .fields["outside_layer"],
        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
            "OPAQUE ALIAS86".into()
        )
    );
    let compiled = compile_raw_model(&raw);
    let model = compiled
        .model
        .as_ref()
        .expect("actual normal alias typed identities");
    let aliases = ["Opaque", "OPAQUE ALIAS86"].map(|name| {
        model
            .materials
            .iter()
            .find(|row| row.name == ep_model::NormalizedName::new(name))
            .expect("actual distinct typed Material")
    });
    assert_ne!(aliases[0].id, aliases[1].id);
    assert_eq!(aliases[0].definition, aliases[1].definition);
    let constructors = ["Opaque", "ALIAS LINK86"].map(|name| {
        model
            .constructions
            .iter()
            .find(|row| row.name == ep_model::NormalizedName::new(name))
            .expect("actual distinct typed Construction")
    });
    assert_ne!(constructors[0].id, constructors[1].id);
    assert_eq!(constructors[0].effective_layers(), &[aliases[0].id]);
    assert_eq!(constructors[1].effective_layers(), &[aliases[1].id]);
    let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
    assert!(selected.admissible, "{:?}", selected.violations);
    assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    // Input-definition algebra only: splitting the stated NoMass resistance
    // gives a1vs2 normal stack candidate without computing U in admission.
    let mut unequal = paired.clone();
    unequal["Material:NoMass"]["HALF OPAQUE85"] = unequal["Material:NoMass"]["Opaque"].clone();
    unequal["Material:NoMass"]["HALF OPAQUE85"]["thermal_resistance"] = json!(1.0);
    unequal["Construction"]["UNEQUAL LEFT85"] = json!({"outside_layer":"Opaque"});
    unequal["Construction"]["UNEQUAL RIGHT85"] =
        json!({"outside_layer":"HALF OPAQUE85","layer_2":"HALF OPAQUE85"});
    unequal["BuildingSurface:Detailed"]["Zulu:East"]["construction_name"] = json!("UNEQUAL LEFT85");
    unequal["BuildingSurface:Detailed"]["Alpha:West"]["construction_name"] =
        json!("UNEQUAL RIGHT85");
    let raw = parse_epjson_str(&unequal.to_string()).expect("declared split resistance input");
    let ip = crate::ordinary_input_processor::resolve_source_a(&raw)
        .expect("unequal normal count structural admission");
    let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
        .expect("unchanged raw Construction operands");
    assert_eq!(
        projected.objects["Construction"]
            .iter()
            .find(|row| row.object_name == "UNEQUAL RIGHT85")
            .unwrap()
            .fields["layer_2"],
        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
            "HALF OPAQUE85".into()
        )
    );
    let compiled = compile_raw_model(&raw);
    let model = compiled
        .model
        .as_ref()
        .expect("actual normal opaque Constructor IDs");
    let counts = ["UNEQUAL LEFT85", "UNEQUAL RIGHT85"].map(|name| {
        model
            .constructions
            .iter()
            .find(|row| row.name.0.eq_ignore_ascii_case(name))
            .unwrap()
            .effective_layers()
            .len()
    });
    assert_eq!(counts, [1, 2]);
    let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
    assert!(selected.admissible, "{:?}", selected.violations);
    assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    let mut wrong_order = paired.clone();
    wrong_order["BuildingSurface:Detailed"]["Alpha:West"]["vertices"]
        .as_array_mut()
        .unwrap()
        .swap(1, 3);
    let raw = parse_epjson_str(&wrong_order.to_string())
        .expect("authored same-direction paired vertices");
    let compiled = compile_raw_model(&raw);
    let selected = inspect_porting_scope(
        &raw,
        &compiled.report,
        compiled.model.as_ref(),
        PortingScope::A,
    );
    assert!(
        !selected.admissible,
        "same-direction World vertices must stay fenced"
    );
    // Proper IRT use changes only the paired Construction/material operands.
    // These assertions inspect genuine raw/IP and typed properties; no full
    // Source getter, physical outcome or unexposed ROnly member is simulated.
    let mut irt_pair = paired.clone();
    irt_pair
        .as_object_mut()
        .unwrap()
        .remove("Output:Constructions");
    irt_pair["Material:InfraredTransparent"] = json!({"WALL IRT":{}});
    irt_pair["Construction"]["IRT LINK"] = json!({"outside_layer":"WALL IRT"});
    for name in ["Zulu:East", "Alpha:West"] {
        irt_pair["BuildingSurface:Detailed"][name]["construction_name"] = json!("IRT LINK");
    }
    let raw = parse_epjson_str(&irt_pair.to_string()).expect("actual name-only paired IRT input");
    let ip = crate::ordinary_input_processor::resolve_source_a(&raw)
        .expect("defined proper IRT pair Source IP");
    assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    let material_input = crate::ordinary_material_input::resolve_source(&raw, &ip)
        .expect("actual IRT raw/IP custody");
    assert!(material_input.source_is_epjson);
    assert_eq!(material_input.infrared_transparent.len(), 1);
    assert_eq!(
        material_input.infrared_transparent[0].object_name,
        "WALL IRT"
    );
    assert!(material_input.infrared_transparent[0].fields.is_empty());
    assert!(
        material_input.infrared_transparent[0]
            .idf_max_fields
            .is_none()
    );
    let compiled = compile_raw_model(&raw);
    let model = compiled
        .model
        .as_ref()
        .expect("actual one-layer paired IRT typed owners");
    let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
    assert!(selected.admissible, "{:?}", selected.violations);
    let material = model
        .materials
        .iter()
        .find(|m| m.kind() == ep_model::MaterialKind::InfraredTransparent)
        .expect("actual IRT material");
    assert_eq!(
        material.thermal_resistance().unwrap().to_bits(),
        0.01_f64.to_bits()
    );
    assert!(material.roughness().is_none());
    let properties = material
        .surface_properties()
        .expect("fixed IRT absorptances");
    assert_eq!(
        properties.thermal_absorptance.to_bits(),
        0.9999_f64.to_bits()
    );
    assert_eq!(properties.solar_absorptance.to_bits(), 1.0_f64.to_bits());
    assert_eq!(properties.visible_absorptance.to_bits(), 1.0_f64.to_bits());
    let construction = model
        .constructions
        .iter()
        .find(|c| c.name == ep_model::NormalizedName::new("IRT LINK"))
        .expect("actual IRT Construction");
    assert_eq!(construction.layers, [material.id]);
    assert_eq!(
        model
            .surfaces
            .iter()
            .filter(|s| s.construction == construction.id)
            .count(),
        2
    );
    // Horizontal normal classes must not leak through the established IRT
    // Surface-use guard. Independently exercise both raw and typed pair fences.
    for (left, right) in [("Roof", "Floor"), ("Ceiling", "Floor"), ("Wall", "Roof")] {
        let mut invalid = irt_pair.clone();
        invalid["BuildingSurface:Detailed"]["Zulu:East"]["surface_type"] = json!(left);
        invalid["BuildingSurface:Detailed"]["Alpha:West"]["surface_type"] = json!(right);
        let raw = parse_epjson_str(&invalid.to_string()).expect("IRT non-Wall pair operands");
        assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("typed IRT class fence operands");
        let mut typed_violations = Vec::new();
        super::surface_pairs::validate(model, PortingScope::A, &mut typed_violations);
        assert!(
            !typed_violations.is_empty(),
            "{left}/{right} IRT escaped typed Wall fence"
        );
        assert!(
            !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A).admissible
        );
        assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
        assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    }
    // The normal differing-ID lane must not admit distinct IRT identities,
    // even though the existing name-only definitions have the same fixed fields.
    let mut irt_alias = irt_pair.clone();
    irt_alias["Material:InfraredTransparent"]["WALL IRT ALIAS86"] = json!({});
    irt_alias["Construction"]["IRT ALIAS LINK86"] = json!({"outside_layer":"WALL IRT ALIAS86"});
    irt_alias["BuildingSurface:Detailed"]["Alpha:West"]["construction_name"] =
        json!("IRT ALIAS LINK86");
    let raw = parse_epjson_str(&irt_alias.to_string()).expect("distinct sole-layer IRT identities");
    let compiled = compile_raw_model(&raw);
    let model = compiled
        .model
        .as_ref()
        .expect("actual typed IRT alias records");
    let materials = ["WALL IRT", "WALL IRT ALIAS86"].map(|name| {
        model
            .materials
            .iter()
            .find(|row| row.name == ep_model::NormalizedName::new(name))
            .expect("actual distinct IRT Material")
    });
    assert_ne!(materials[0].id, materials[1].id);
    assert_eq!(materials[0].definition, materials[1].definition);
    assert!(
        materials
            .iter()
            .all(|row| row.kind() == ep_model::MaterialKind::InfraredTransparent)
    );
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
    let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
    assert!(
        !selected.admissible,
        "different IRT IDs cannot use the normal U lane"
    );
    assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    let mut unequal_irt = irt_pair.clone();
    unequal_irt["Construction"]["NORMAL TWO85"] =
        json!({"outside_layer":"Opaque","layer_2":"Opaque"});
    unequal_irt["BuildingSurface:Detailed"]["Alpha:West"]["construction_name"] =
        json!("NORMAL TWO85");
    let raw = parse_epjson_str(&unequal_irt.to_string())
        .expect("new unequal IRT/normal slice must stay excluded");
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
    assert!(!trace(&unequal_irt, PortingScope::A).admissible);
    let mut multilayer_irt = irt_pair.clone();
    multilayer_irt["Construction"]["IRT LINK"] =
        json!({"outside_layer":"WALL IRT", "layer_2":"Opaque", "layer_3":"WALL IRT"});
    let raw =
        parse_epjson_str(&multilayer_irt.to_string()).expect("authored multi-layer IRT input");
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
    assert!(
        !trace(&multilayer_irt, PortingScope::A).admissible,
        "a palindromic stack must not broaden the genuine IRT sole-layer rule"
    );
    for report in ["Materials", "Constructions"] {
        let mut invalid = irt_pair.clone();
        invalid["Output:Constructions"] = json!({"Report":{"details_type_1":report}});
        let raw = parse_epjson_str(&invalid.to_string()).expect("IRT undefined text-report branch");
        assert!(
            crate::ordinary_equipment_input::resolve_source_a(&raw).is_err(),
            "{report} must stay fenced"
        );
    }
    let mut outdoor = irt_pair.clone();
    outdoor["BuildingSurface:Detailed"]["Zulu:West"]["construction_name"] = json!("IRT LINK");
    let raw = parse_epjson_str(&outdoor.to_string()).expect("IRT additional Outdoors use");
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
    let mut extra_adiabatic = irt_pair.clone();
    extra_adiabatic["BuildingSurface:Detailed"]["Zulu:Floor"]["outside_boundary_condition"] =
        json!("Adiabatic");
    extra_adiabatic["BuildingSurface:Detailed"]["Zulu:Floor"]["construction_name"] =
        json!("IRT LINK");
    let raw = parse_epjson_str(&extra_adiabatic.to_string())
        .expect("IRT additional multi-Zone Adiabatic use");
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
    let mut invalid_pair = irt_pair;
    invalid_pair["BuildingSurface:Detailed"]["Alpha:West"]["outside_boundary_condition_object"] =
        json!("Missing");
    let raw = parse_epjson_str(&invalid_pair.to_string()).expect("IRT invalid actual pair target");
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());

    // Unit102: ElectricEquipment uses a separate arena and remains a Source A
    // compatibility admission, including the ordinary one-Zone default Space.
    for authored_lists in [false, true] {
        let mut value = self::input(PortingScope::A);
        value["OtherEquipment"] = json!({"Same":{"zone_or_zonelist_or_space_or_spacelist_name":"Zone","schedule_name":"On","design_level":2}});
        value["ElectricEquipment"] = json!({"Same":{"zone_or_zonelist_or_space_or_spacelist_name":"Zone","schedule_name":"Constant-1.0","design_level":3,"end_use_subcategory":" Case  kept "}});
        if authored_lists {
            value["Space"] = json!({"Room":{"zone_name":"Zone"}});
            value["ZoneList"] = json!({"Both":{"zones":[{"zone_name":"Zone"}]}});
            value["SpaceList"] =
                json!({"Both":{},"Only":{"spaces":[{"space_name":"Room"}]},"Empty":{}});
            value["ElectricEquipment"]["A empty"] =
                json!({"zone_or_zonelist_or_space_or_spacelist_name":"Empty","schedule_name":"On"});
            value["ElectricEquipment"]["B list"] = json!({"zone_or_zonelist_or_space_or_spacelist_name":"Both","schedule_name":"On","design_level_calculation_method":"Watts/Area","watts_per_floor_area":4});
            value["ElectricEquipment"]["C space list"] = json!({"zone_or_zonelist_or_space_or_spacelist_name":"Only","schedule_name":"On","design_level_calculation_method":"Watts/Person","watts_per_person":5});
            value["ElectricEquipment"]["D space"] = json!({"zone_or_zonelist_or_space_or_spacelist_name":"Room","schedule_name":"On","design_level":6});
        }
        let raw = parse_epjson_str(&value.to_string()).expect("actual electric Source input");
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled
            .model
            .as_ref()
            .expect("distinct electric typed inputs");
        assert_eq!(model.other_equipment.len(), 1);
        assert_eq!(model.source_other_equipment_inputs.len(), 1);
        assert_eq!(
            model.electric_equipment.len(),
            if authored_lists { 4 } else { 1 }
        );
        assert_eq!(
            model.source_electric_equipment_inputs.len(),
            if authored_lists { 5 } else { 1 }
        );
        let selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(selected.admissible, "{:?}", selected.violations);
        assert!(
            !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B).admissible
        );
        let mut selected = selected;
        selected.environment = Some(
            super::environment::environment_from_weather(model, &weather, "electric Source A")
                .expect("actual selected environment"),
        );
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                .expect("electric Source admission");
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
        assert_eq!(
            support.runtime_class,
            crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
        );
        assert!(support.matched_capability_ids.is_empty());
        assert!(!support.runtime_class.conformance_promotion_allowed());
        assert!(
            !assess_support(
                &raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal
            )
            .allows_rust_runtime()
        );
        let legacy = compile_raw_model(&raw);
        let legacy_model = legacy
            .model
            .as_ref()
            .expect("legacy preserves raw electric family");
        assert!(legacy_model.electric_equipment.is_empty());
        assert!(
            !inspect_porting_scope(&raw, &legacy.report, Some(legacy_model), PortingScope::A)
                .admissible
        );
        for changed in 0..5 {
            let mut corrupt = model.clone();
            match changed {
                0 => corrupt.electric_equipment[0].id = ep_model::ElectricEquipmentId(99),
                1 => corrupt.electric_equipment[0].name.0.push('X'),
                2 => corrupt.electric_equipment[0].design_level_w += 1.0,
                3 => corrupt.electric_equipment[0].schedule = None,
                _ => {
                    corrupt
                        .source_electric_equipment_inputs
                        .last_mut()
                        .expect("raw Electric range")
                        .instances
                        .end = 0
                }
            }
            assert!(
                !inspect_porting_scope(&raw, &compiled.report, Some(&corrupt), PortingScope::A)
                    .admissible,
                "Electric binding corruption {changed}"
            );
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
    // Actual Source intake/effective settings only; physical solver parity is
    // established separately on complete same-input primary runs.
    for scope in [PortingScope::A, PortingScope::B] {
        for (authored, effective) in [
            (
                "ThirdOrderBackwardDifference",
                "ThirdOrderBackwardDifference",
            ),
            ("analyticalsolution", "AnalyticalSolution"),
            ("eulermethod", "EulerMethod"),
        ] {
            let mut selected = input(scope);
            selected["ZoneAirHeatBalanceAlgorithm"] = json!({"Control":{
                "algorithm":authored,
                "do_space_heat_balance_for_sizing":"No",
                "do_space_heat_balance_for_simulation":"No",
            }});
            let raw = parse_epjson_str(&selected.to_string()).expect("actual Source air algorithm");
            let source = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            };
            assert!(
                source.is_ok(),
                "{scope:?}/{authored}: {:?}",
                source.as_ref().err()
            );
            let selected_scope = trace(&selected, scope);
            assert!(selected_scope.admissible, "{:?}", selected_scope.violations);
            assert_eq!(
                selected_scope.settings["zone_air_heat_balance_algorithm"],
                effective
            );
            if effective != "ThirdOrderBackwardDifference" {
                assert!(crate::ordinary_input_processor::resolve(&raw).is_err());
            }
        }
    }
    assert_eq!(actual.settings["timesteps_per_hour"], 4);
    assert_eq!(actual.production["conformance_claim"], false);
    value["Building"]["Building"]["maximum_number_of_warmup_days"] = json!(0);
    assert!(!trace(&value, PortingScope::A).admissible);

    // This tests the input boundary only, not Source activation or window
    // transport. All five choices reach the genuine opaque solar branches;
    // positive window/TDD and specular shading children remain guarded.
    for scope in [PortingScope::A, PortingScope::B] {
        for distribution in [
            "MinimalShadowing",
            "FullExterior",
            "FullInteriorAndExterior",
            "FullExteriorWithReflections",
            "FullInteriorAndExteriorWithReflections",
        ] {
            let mut authored = input(scope);
            authored["Building"]["Building"]["solar_distribution"] = json!(distribution);
            let actual = trace(&authored, scope);
            assert!(
                actual.admissible,
                "{scope:?}/{distribution}: {:?}",
                actual.violations
            );
            assert_eq!(
                actual.settings["building"]["solar_distribution"],
                distribution
            );
            assert_eq!(actual.production["conformance_claim"], false);
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
    // This field is consumed by the same real raw/IP/getter owners. Admission
    // covers its Native-defined integer range, rather than one example scalar.
    let weather = single_day_weather_fixture();
    for scope in [PortingScope::A, PortingScope::B] {
        for multiplier in [1_u32, 2, 3, 65_535, i32::MAX as u32] {
            let mut authored = input(scope);
            authored["Zone"]["Zone"]["multiplier"] = json!(multiplier);
            let raw = parse_epjson_str(&authored.to_string()).expect("actual Zone multiplier");
            let compiled = compile_raw_model(&raw);
            let model = compiled
                .model
                .as_ref()
                .expect("Native-defined typed multiplier");
            assert_eq!(model.zones[0].multiplier, multiplier);
            assert_eq!(model.zones[0].list_multiplier, 1);
            let ip = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .expect("actual multiplier input owner");
            let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
                .expect("real retained Zone numeric operand");
            assert_eq!(
                projected.objects["Zone"][0].fields.get("multiplier"),
                Some(
                    &ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                        f64::from(multiplier)
                    )
                ),
            );
            let mut admitted = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
            assert!(
                admitted.admissible,
                "{scope:?}/{multiplier}: {:?}",
                admitted.violations
            );
            assert_eq!(admitted.production["conformance_claim"], false);
            admitted.environment = Some(
                super::environment::environment_from_weather(
                    model,
                    &weather,
                    "embedded Zone multiplier weather",
                )
                .expect("same genuine EPW/calendar operands"),
            );
            let admission =
                crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                    .expect("all real Source admission guards");
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
            assert!(
                actual.allows_rust_runtime(),
                "{scope:?}/{multiplier}: {:?}",
                actual.diagnostics
            );
            // The independent group restriction is not broadened by a positive
            // Zone multiplier, including the largest representable scalar.
            let mut grouped = model.clone();
            grouped.zones[0].list_multiplier = 2;
            let group_trace = inspect_porting_scope(&raw, &compiled.report, Some(&grouped), scope);
            assert!(!group_trace.admissible);
            assert!(
                group_trace
                    .violations
                    .iter()
                    .any(|v| v.contains("ZoneGroup/list"))
            );
        }
        for multiplier in [json!(0), json!(-1), json!(1.5), json!(2147483648_u64)] {
            let mut authored = input(scope);
            authored["Zone"]["Zone"]["multiplier"] = multiplier;
            let raw = parse_epjson_str(&authored.to_string()).expect("numeric raw multiplier");
            let compiled = compile_raw_model(&raw);
            assert!(compiled.has_errors());
            assert!(compiled.model.is_none());
            assert!(!inspect_porting_scope(&raw, &compiled.report, None, scope).admissible);
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
            "RoomAirModelType",
            "room_air_modeling_type",
            "UserDefined",
            "room_air_model_type",
        ),
        (
            "SurfaceConvectionAlgorithm:Inside",
            "algorithm",
            "AdaptiveConvectionAlgorithm",
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
    value["SizingPeriod:DesignDay"] = json!({"Day":{
        "month":7,"day_of_month":21,"day_type":"SummerDesignDay",
        "maximum_dry_bulb_temperature":32,"daily_dry_bulb_temperature_range":12,
        "humidity_condition_type":"WetBulb","wetbulb_or_dewpoint_at_maximum_dry_bulb":15,
        "wind_speed":2,"wind_direction":0,"sky_clearness":1
    }});
    assert!(trace(&value, PortingScope::A).admissible);
    value["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] = json!("Yes");
    let actual = trace(&value, PortingScope::A);
    assert!(actual.admissible, "{:?}", actual.violations);
    assert!(actual.inactive_branches.is_empty());
    let mut bounded_b = input(PortingScope::B);
    bounded_b["SizingPeriod:DesignDay"] = value["SizingPeriod:DesignDay"].clone();
    bounded_b["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] = json!("Yes");
    let selected_b = trace(&bounded_b, PortingScope::B);
    assert!(selected_b.admissible, "{:?}", selected_b.violations);
    assert!(selected_b.inactive_branches.is_empty());
    for field in [
        "do_zone_sizing_calculation",
        "do_system_sizing_calculation",
        "do_plant_sizing_calculation",
    ] {
        let mut sizing = bounded_b.clone();
        sizing["SimulationControl"]["Control"][field] = json!("Yes");
        assert!(!trace(&sizing, PortingScope::B).admissible, "{field}");
    }

    // Adaptive comfort uses actual DD initialization only. A zero design count
    // still creates its owner, whereas an empty SpaceList creates none.
    for scope in [PortingScope::A, PortingScope::B] {
        for (model, also_ash) in [
            ("AdaptiveASH55", false),
            ("AdaptiveCEN15251", false),
            ("AdaptiveCEN15251", true),
        ] {
            for count in [0.0, 1.5] {
                let mut adaptive = input(scope);
                adaptive["SizingPeriod:DesignDay"] = value["SizingPeriod:DesignDay"].clone();
                adaptive["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] =
                    json!("Yes");
                adaptive["Schedule:Constant"]["Activity"] = json!({"hourly_value":120});
                adaptive["People"] = json!({"Adaptive":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Zone",
                    "number_of_people_schedule_name":"On",
                    "activity_level_schedule_name":"Activity",
                    "number_of_people":count,
                    "sensible_heat_fraction":0.6,
                    "thermal_comfort_model_7_type":model
                }});
                if also_ash {
                    adaptive["People"]["Adaptive"]["thermal_comfort_model_1_type"] =
                        json!("AdaptiveASH55");
                }
                let actual = trace(&adaptive, scope);
                assert!(!actual.admissible);
                assert!(actual.violations.iter().any(|message| {
                    message.contains("adaptive comfort requires DesignDay-only")
                }));
                adaptive["SimulationControl"]["Control"]["run_simulation_for_weather_file_run_periods"] =
                    json!("No");
                let run_period = adaptive
                    .as_object_mut()
                    .expect("adaptive object")
                    .remove("RunPeriod")
                    .expect("weather RunPeriod");
                let actual = trace(&adaptive, scope);
                assert!(actual.admissible, "{:?}", actual.violations);
                let raw = parse_epjson_str(&adaptive.to_string()).expect("adaptive DD input");
                match scope {
                    PortingScope::A => crate::ordinary_equipment_input::resolve_source_a(&raw),
                    PortingScope::B => crate::ordinary_equipment_input::resolve_source_b(&raw),
                }
                .expect("adaptive DD Source admission");
                assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
                for report in ["AdaptiveComfortSummary", "adaptivecomfortsummary"] {
                    let mut reporting = adaptive.clone();
                    reporting["Output:Table:SummaryReports"] =
                        json!({"Reports":{"reports":[{"report_name":report}]}});
                    let actual = trace(&reporting, scope);
                    assert!(actual.admissible, "{:?}", actual.violations);
                    let resolve_report = |value: &Value| {
                        let raw = parse_epjson_str(&value.to_string()).expect("summary request");
                        match scope {
                            PortingScope::A => {
                                crate::ordinary_equipment_input::resolve_source_a(&raw)
                            }
                            PortingScope::B => {
                                crate::ordinary_equipment_input::resolve_source_b(&raw)
                            }
                        }
                    };
                    resolve_report(&reporting).expect("connected adaptive summary request");
                    reporting["Output:Table:SummaryReports"]["Reports"]["reports"][0]["report_name"] =
                        json!("UnsupportedSummary");
                    assert!(resolve_report(&reporting).is_err());
                }
                adaptive["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] =
                    json!("No");
                assert!(!trace(&adaptive, scope).admissible);
                if scope == PortingScope::A {
                    adaptive["SimulationControl"]["Control"]["run_simulation_for_weather_file_run_periods"] =
                        json!("Yes");
                    adaptive["RunPeriod"] = run_period;
                    adaptive["SpaceList"] = json!({"Empty":{"spaces":[]}});
                    adaptive["People"]["Adaptive"]["zone_or_zonelist_or_space_or_spacelist_name"] =
                        json!("Empty");
                    let raw =
                        parse_epjson_str(&adaptive.to_string()).expect("empty adaptive target");
                    let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                    let actual = inspect_porting_scope(
                        &raw,
                        &compiled.report,
                        compiled.model.as_ref(),
                        scope,
                    );
                    assert!(actual.admissible, "{:?}", actual.violations);
                }
            }
        }
    }

    // Explicit SourceA consumes scalar/default-profile DesignDays
    // without inheriting the frozen one-Zone capability or ignored-DD warning.
    let weather = single_day_weather_fixture();
    for (mode, operand, amount) in [
        ("WetBulb", "wetbulb_or_dewpoint_at_maximum_dry_bulb", 15.0),
        ("DewPoint", "wetbulb_or_dewpoint_at_maximum_dry_bulb", 15.0),
        ("HumidityRatio", "humidity_ratio_at_maximum_dry_bulb", 0.009),
        ("Enthalpy", "enthalpy_at_maximum_dry_bulb", 50000.0),
        (
            "WetBulbProfileDefaultMultipliers",
            "wetbulb_or_dewpoint_at_maximum_dry_bulb",
            15.0,
        ),
    ] {
        let mut value = self::input(PortingScope::A);
        value["SizingPeriod:DesignDay"] = json!({" DD ":{
            "month":7,"day_of_month":21,"day_type":"SummerDesignDay",
            "maximum_dry_bulb_temperature":32,"daily_dry_bulb_temperature_range":12,
            "humidity_condition_type":mode,"wind_speed":2,"wind_direction":0,
            "daily_wet_bulb_temperature_range":6,"sky_clearness":1
        }});
        value["SizingPeriod:DesignDay"][" DD "][operand] = amount.into();
        let raw = parse_epjson_str(&value.to_string()).expect("scalar DesignDay SourceA raw");
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled.model.as_ref().expect("single Zone typed model");
        let mut selected =
            inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(selected.admissible, "{:?}", selected.violations);
        selected.environment = Some(
            super::environment::environment_from_weather(
                model,
                &weather,
                "inactive scalar DesignDay SourceA",
            )
            .expect("actual coverage"),
        );
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                .expect("explicit DesignDay SourceA admission");
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
        assert_eq!(
            support.runtime_class,
            crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
        );
        assert!(
            support.matched_capability_ids.is_empty() && support.matched_capabilities.is_empty()
        );
        assert!(!support.runtime_class.conformance_promotion_allowed());
        assert!(
            support
                .source_consumed_objects
                .iter()
                .any(|row| row.object_type == "SizingPeriod:DesignDay")
        );
        assert!(
            !support
                .diagnostics
                .diagnostics
                .iter()
                .any(|row| row.code == "UnsupportedAlgorithmIgnored"
                    && row.object_type.as_deref() == Some("SizingPeriod:DesignDay"))
        );
        value["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] = json!("Yes");
        let active = trace(&value, PortingScope::A);
        assert!(active.admissible, "{:?}", active.violations);
        assert!(active.inactive_branches.is_empty());
        if !matches!(mode, "WetBulb" | "DewPoint") {
            crate::ordinary_equipment_input::resolve_source_b(&raw)
                .expect("same SourceB scalar DD admission");
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        }
    }

    // Named-day profiles can omit max DB and retain the official reset key
    // while entering the same nonpromotable inactive Source class.
    let mut value = self::input(PortingScope::A);
    value["SizingPeriod:DesignDay"] = json!({"Scheduled DD":{
        "month":7,"day_of_month":21,"day_type":"SummerDesignDay",
        "dry_bulb_temperature_range_modifier_type":"TemperatureProfileSchedule",
        "dry_bulb_temperature_range_modifier_day_schedule_name":" actual day ",
        "humidity_condition_type":"RelativeHumiditySchedule",
        "humidity_condition_day_schedule_name":" actual day ",
        "solar_model_indicator":"Schedule", "beam_solar_day_schedule_name":" actual day ",
        "diffuse_solar_day_schedule_name":" actual day ",
        "begin_environment_reset_mode":"SuppressAllBeginEnvironmentResets",
        "wind_speed":2,"wind_direction":0
    }});
    let mut day = serde_json::Map::new();
    for hour in 1..=24 {
        day.insert(format!("hour_{hour}"), 0.5.into());
    }
    value["Schedule:Day:Hourly"] = json!({});
    value["Schedule:Day:Hourly"][" actual day "] = day.into();
    let raw = parse_epjson_str(&value.to_string()).expect("scheduled inactive DD raw");
    crate::ordinary_equipment_input::resolve_source_a(&raw).expect("scheduled SourceA admission");
    crate::ordinary_equipment_input::resolve_source_b(&raw)
        .expect("same SourceB scheduled DD raw admission");
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    let compiled = ep_compiler::compile_raw_model_for_source(&raw);
    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
    let model = compiled.model.as_ref().expect("typed weather Zone");
    let mut selected = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
    assert!(selected.admissible, "{:?}", selected.violations);
    selected.environment = Some(
        super::environment::environment_from_weather(
            model,
            &weather,
            "scheduled inactive DD SourceA",
        )
        .expect("weather coverage"),
    );
    let admission = crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
        .expect("scheduled inactive SourceA support");
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
    assert_eq!(
        support.runtime_class,
        crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility
    );
    assert!(support.matched_capability_ids.is_empty() && support.matched_capabilities.is_empty());
    assert!(!support.runtime_class.conformance_promotion_allowed());
    assert!(
        support
            .source_consumed_objects
            .iter()
            .any(|row| row.object_type == "SizingPeriod:DesignDay" && row.count == 1)
    );
    assert!(
        !support
            .diagnostics
            .diagnostics
            .iter()
            .any(|row| row.code == "UnsupportedAlgorithmIgnored"
                && row.object_type.as_deref() == Some("SizingPeriod:DesignDay"))
    );
    let mut active = value.clone();
    active["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] = json!("Yes");
    let selected = trace(&active, PortingScope::A);
    assert!(selected.admissible, "{:?}", selected.violations);
    for field in [
        "do_zone_sizing_calculation",
        "do_system_sizing_calculation",
        "do_plant_sizing_calculation",
    ] {
        let mut sizing = active.clone();
        sizing["SimulationControl"]["Control"][field] = json!("Yes");
        assert!(!trace(&sizing, PortingScope::A).admissible, "{field}");
    }
    let mut no_weather = active.clone();
    no_weather["SimulationControl"]["Control"]["run_simulation_for_weather_file_run_periods"] =
        json!("No");
    assert!(!trace(&no_weather, PortingScope::A).admissible);
    active.as_object_mut().unwrap().remove("RunPeriod");
    assert!(!trace(&active, PortingScope::A).admissible);

    // A genuine DD-only input has no weather RunPeriod or prepared clock.
    let mut dd_only = no_weather;
    dd_only.as_object_mut().unwrap().remove("RunPeriod");
    dd_only["Site:Location"] = json!({"Actual DD Site":{
        "latitude":39.74,"longitude":-105.18,"time_zone":-7,"elevation":1829
    }});
    for selected_scope in [PortingScope::A, PortingScope::B] {
        let mut candidate = if selected_scope == PortingScope::A {
            dd_only.clone()
        } else {
            let mut b = input(PortingScope::B);
            for family in [
                "SimulationControl",
                "SizingPeriod:DesignDay",
                "Schedule:Day:Hourly",
                "Site:Location",
            ] {
                b[family] = dd_only[family].clone();
            }
            b.as_object_mut().unwrap().remove("RunPeriod");
            b
        };
        let mut selected = trace(&candidate, selected_scope);
        assert!(selected.admissible, "{:?}", selected.violations);
        assert!(selected.source_design_day_only());
        let raw = parse_epjson_str(&candidate.to_string()).expect("DD-only raw input");
        let (environment, calendar_admission) =
            super::environment::porting_source_design_day_trace(
                &raw,
                selected_scope,
                std::path::Path::new("in.epw"),
                true,
            )
            .expect("input-only DD admission");
        assert!(
            super::environment::validate_source_design_day_environment(
                &environment,
                &calendar_admission
            )
            .is_empty()
        );
        assert_eq!(environment["weather_coverage"]["status"], "absent");
        assert!(environment.get("first").is_none());
        assert!(environment.get("calendar_year").is_none());
        selected.environment = Some(environment);
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled.model.as_ref().expect("Source DD-only typed model");
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                .expect("Source DD-only support admission");
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
        assert_eq!(
            support.runtime_class,
            match selected_scope {
                PortingScope::A => crate::RuntimeClass::SourceMultiZoneHeatBalanceCompatibility,
                PortingScope::B => crate::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility,
            }
        );
        if selected_scope == PortingScope::A {
            assert!(
                support.matched_capability_ids.is_empty()
                    && support.matched_capabilities.is_empty()
            );
            assert!(!support.runtime_class.conformance_promotion_allowed());
        } else {
            // Preserve the existing SourceB class/capability policy; DD input
            // consumption is separately validated and grants no new capability.
            assert!(!support.matched_capability_ids.is_empty());
            assert!(support.runtime_class.conformance_promotion_allowed());
        }
        assert!(
            support
                .source_consumed_objects
                .iter()
                .any(|row| row.object_type == "SizingPeriod:DesignDay" && row.count == 1)
        );
        candidate["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] =
            json!("No");
        assert!(!trace(&candidate, selected_scope).admissible);
    }
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
    // The first positive-OA lane traverses real Raw -> Compiler -> Scope ->
    // environment-bound Support -> IP/statistics, without a runtime shortcut.
    {
        use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as OaRaw;
        for method in [
            "Flow/Person",
            "Flow/Area",
            "Flow/Zone",
            "AirChanges/Hour",
            "Sum",
            "Maximum",
        ] {
            let mut oa = input(PortingScope::B);
            oa["DesignSpecification:OutdoorAir"] = json!({"Actual OA":{
                "outdoor_air_method":method, "outdoor_air_flow_per_person":0.003,
                "outdoor_air_flow_per_zone_floor_area":0.0002, "outdoor_air_flow_per_zone":0.01,
                "outdoor_air_flow_air_changes_per_hour":0.25,
                "outdoor_air_schedule_name":"On",
                "proportional_control_minimum_outdoor_air_flow_rate_schedule_name":"On"
            }});
            oa["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["design_specification_outdoor_air_object_name"] =
                json!("Actual OA");
            oa["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_inlet_node_name"] =
                json!("Outside 각");
            let raw = parse_epjson_str(&oa.to_string()).expect("authored basic OA raw");
            let retained = raw.clone();
            let compiled = ep_compiler::compile_raw_model_for_source(&raw);
            assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
            let model = compiled
                .model
                .as_ref()
                .expect("actual basic OA typed model");
            let simulation = ep_model::SimulationModel::from_typed(model.clone());
            ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                .expect("actual six-method Source OA validation");
            assert!(ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_err());
            let mut selected =
                inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B);
            assert!(selected.admissible, "{:?}", selected.violations);
            selected.environment = Some(
                super::environment::environment_from_weather(
                    model,
                    &single_day_weather_fixture(),
                    "actual basic OA admission weather",
                )
                .expect("real Source OA environment"),
            );
            let token =
                crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                    .expect("same actual raw basic OA scope token");
            let support = crate::support::assess_support_with_source_admission(
                &raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&token),
            );
            assert!(support.allows_rust_runtime(), "{:?}", support.diagnostics);
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
            assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
            assert!(
                !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A)
                    .admissible
            );
            let ip =
                crate::ordinary_input_processor::resolve_source_b(&raw).expect("real basic OA IP");
            let mut statistics =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            ip.report_source_idf_records_stats(&mut statistics)
                .expect("actual DSOA statistics owner");
            let fields = ip
                .source_raw_object_fields("DesignSpecification:OutdoorAir", "Actual OA")
                .expect("retained actual DSOA raw fields");
            assert_eq!(fields["outdoor_air_method"], OaRaw::String(method.into()));
            assert_eq!(
                fields["outdoor_air_flow_per_zone"],
                OaRaw::Number("0.01".into())
            );
            assert_eq!(
                fields["outdoor_air_schedule_name"],
                OaRaw::String("On".into())
            );
            assert_eq!(
                fields["proportional_control_minimum_outdoor_air_flow_rate_schedule_name"],
                OaRaw::String("On".into())
            );
            assert_eq!(raw, retained);
            // Same real Raw/Compiler/Scope/Support/IP chain, now with the
            // four owned limit names and actual positive/zero numeric operands.
            for limit in [
                "NoLimit",
                "LimitFlowRate",
                "LimitCapacity",
                "LimitFlowRateAndCapacity",
            ] {
                for (flow, capacity) in [(0.00005, 1.0), (0.0, 0.0)] {
                    let mut hard = oa.clone();
                    let unit = &mut hard["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"];
                    unit["heating_limit"] = json!(limit);
                    unit["cooling_limit"] = json!(limit);
                    if method == "Flow/Person" && limit == "NoLimit" && flow > 0.0 {
                        unit["demand_controlled_ventilation_type"] = json!("OccupancySchedule");
                    }
                    if matches!(limit, "LimitFlowRate" | "LimitFlowRateAndCapacity") {
                        unit["maximum_heating_air_flow_rate"] = json!(flow);
                        unit["maximum_cooling_air_flow_rate"] = json!(flow);
                    }
                    if matches!(limit, "LimitCapacity" | "LimitFlowRateAndCapacity") {
                        unit["maximum_sensible_heating_capacity"] = json!(capacity);
                        unit["maximum_total_cooling_capacity"] = json!(capacity);
                    }
                    // Reuse one genuine SUM/NoLimit positive context: no extra
                    // getter/physics fixture or RawModel normalization is added.
                    let wide_priority = method == "Sum" && limit == "NoLimit" && flow > 0.0;
                    if wide_priority {
                        for field in [
                            "zone_equipment_cooling_sequence",
                            "zone_equipment_heating_or_no_load_sequence",
                        ] {
                            hard["ZoneHVAC:EquipmentList"]["Equipment"]["equipment"][0][field] =
                                json!(4_294_967_297_u64);
                        }
                    }
                    let hard_raw = parse_epjson_str(&hard.to_string())
                        .expect("actual OA hard-limit raw input");
                    let retained_hard = hard_raw.clone();
                    let compiled = ep_compiler::compile_raw_model_for_source(&hard_raw);
                    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                    let model = compiled
                        .model
                        .as_ref()
                        .expect("real typed OA hard-limit model");
                    if wide_priority {
                        assert!(ep_compiler::compile_raw_model(&hard_raw).has_errors());
                        let entry = &model.zone_equipment_lists[0].equipment[0];
                        assert_eq!(
                            (entry.cooling_sequence, entry.heating_or_no_load_sequence),
                            (1, 1)
                        );
                    }
                    let simulation = ep_model::SimulationModel::from_typed(model.clone());
                    ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                        .expect("actual Source OA hard-limit eligibility");
                    assert!(ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_err());
                    let mut selected = inspect_porting_scope(
                        &hard_raw,
                        &compiled.report,
                        Some(model),
                        PortingScope::B,
                    );
                    assert!(selected.admissible, "{:?}", selected.violations);
                    selected.environment = Some(
                        super::environment::environment_from_weather(
                            model,
                            &single_day_weather_fixture(),
                            "authored OA hard-limit admission weather",
                        )
                        .expect("real hard-limit environment"),
                    );
                    let admission = crate::support::SourceSupportAdmission::from_validated_scope(
                        &hard_raw, &selected,
                    )
                    .expect("same real hard-limit raw/scope token");
                    let support = crate::support::assess_support_with_source_admission(
                        &hard_raw,
                        &compiled.report,
                        Some(model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&admission),
                    );
                    assert!(support.allows_rust_runtime(), "{:?}", support.diagnostics);
                    assert!(crate::ordinary_equipment_input::resolve(&hard_raw).is_err());
                    assert!(crate::ordinary_equipment_input::resolve_source_a(&hard_raw).is_err());
                    let hard_ip = crate::ordinary_input_processor::resolve_source_b(&hard_raw)
                        .expect("real hard-limit IP");
                    let mut statistics = ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                    hard_ip
                        .report_source_idf_records_stats(&mut statistics)
                        .expect("actual hard-limit statistics owner");
                    let actual = hard_ip
                        .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                        .expect("retained hard-limit raw alpha/numeric fields");
                    assert_eq!(actual["heating_limit"], OaRaw::String(limit.into()));
                    assert_eq!(actual["cooling_limit"], OaRaw::String(limit.into()));
                    if method == "Flow/Person" && limit == "NoLimit" && flow > 0.0 {
                        assert_eq!(
                            actual["demand_controlled_ventilation_type"],
                            OaRaw::String("OccupancySchedule".into())
                        );
                    }
                    if wide_priority {
                        let list = hard_ip
                            .source_raw_object_fields("ZoneHVAC:EquipmentList", "Equipment")
                            .expect("actual retained EquipmentList");
                        let entries = match list.get("equipment") {
                            Some(OaRaw::Array(entries)) => Some(entries),
                            _ => None,
                        }
                        .expect("actual retained entries");
                        let entry = match entries.first() {
                            Some(OaRaw::Object(entry)) => Some(entry),
                            _ => None,
                        }
                        .expect("actual retained entry");
                        for field in [
                            "zone_equipment_cooling_sequence",
                            "zone_equipment_heating_or_no_load_sequence",
                        ] {
                            assert_eq!(entry[field], OaRaw::Number("4294967297".into()));
                        }
                    }
                    assert_eq!(hard_raw, retained_hard);
                    for missing in [
                        "maximum_heating_air_flow_rate",
                        "maximum_cooling_air_flow_rate",
                        "maximum_sensible_heating_capacity",
                        "maximum_total_cooling_capacity",
                    ] {
                        if !hard["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]
                            .as_object()
                            .expect("unit")
                            .contains_key(missing)
                        {
                            continue;
                        }
                        for invalid in [Value::Null, json!("Autosize"), json!(-1.0)] {
                            let mut bad = hard.clone();
                            bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][missing] = invalid;
                            let bad_raw = parse_epjson_str(&bad.to_string())
                                .expect("actual invalid active numeric");
                            assert!(
                                crate::ordinary_equipment_input::resolve_source_b(&bad_raw)
                                    .is_err()
                            );
                            let bad_compiled = ep_compiler::compile_raw_model_for_source(&bad_raw);
                            assert!(
                                !inspect_porting_scope(
                                    &bad_raw,
                                    &bad_compiled.report,
                                    bad_compiled.model.as_ref(),
                                    PortingScope::B
                                )
                                .admissible
                            );
                        }
                    }
                }
            }
            // Each negative is a genuine authored raw candidate. Scope and IP
            // must both hold it; typed defaulting cannot grant permission.
            for invalid in 0..22 {
                let mut bad = oa.clone();
                match invalid {
                    0 => {
                        // Other five existing methods retain the empty-selector
                        // rejection; this per-person candidate keeps CO2 held.
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["demand_controlled_ventilation_type"] =
                            if method == "Flow/Person" {
                                json!("CO2Setpoint")
                            } else {
                                json!("")
                            }
                    }
                    1 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_economizer_type"] =
                            json!("")
                    }
                    2 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heat_recovery_type"] =
                            json!("")
                    }
                    3 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]
                            .as_object_mut()
                            .expect("unit")
                            .remove("dehumidification_control_type");
                    }
                    4 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["humidification_control_type"] =
                            json!("")
                    }
                    5 => bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["cooling_limit"] = json!(""),
                    6 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heating_limit"] =
                            json!("LimitCapacity");
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_sensible_heating_capacity"] =
                            json!("Autosize");
                    }
                    7 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["design_specification_outdoor_air_object_name"] =
                            json!("Missing")
                    }
                    8 => {
                        bad.as_object_mut()
                            .expect("root")
                            .remove("DesignSpecification:OutdoorAir");
                    }
                    9 => {
                        bad["DesignSpecification:OutdoorAir"]["Other"] =
                            bad["DesignSpecification:OutdoorAir"]["Actual OA"].clone()
                    }
                    10 => {
                        bad["DesignSpecification:OutdoorAir"]["Actual OA"]["outdoor_air_method"] =
                            json!("IndoorAirQualityProcedure")
                    }
                    11 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_inlet_node_name"] =
                            json!("Supply")
                    }
                    12 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_inlet_node_name"] =
                            json!("Air")
                    }
                    13 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_inlet_node_name"] =
                            json!("Return")
                    }
                    14 => bad["NodeList"] = json!({"Outside 각":{"nodes":[{"node_name":"Other"}]}}),
                    15 => {
                        bad["NodeList"] =
                            json!({"Real Returns":{"nodes":[{"node_name":"Outside 각"}]}});
                        bad["ZoneHVAC:EquipmentConnections"]["Connection"]["zone_return_air_node_or_nodelist_name"] =
                            json!("Real Returns");
                    }
                    16 => bad["OutdoorAir:Node"] = json!({"Authored OA":{"height_above_ground":0}}),
                    17 => {
                        bad["OutdoorAir:NodeList"] = json!({"Authored OA":{"nodes":[{"node_or_nodelist_name":"Outside 각"}]}})
                    }
                    18 => {
                        bad["ZoneControl:Humidistat"] = json!({"Uncomposed control":{"zone_name":"Zone","humidifying_relative_humidity_setpoint_schedule_name":"On"}})
                    }
                    19 => {
                        bad["DesignSpecification:OutdoorAir"]["Actual OA"]["outdoor_air_schedule_name"] =
                            json!("Missing")
                    }
                    20 => {
                        bad["DesignSpecification:OutdoorAir"]["Actual OA"]["proportional_control_minimum_outdoor_air_flow_rate_schedule_name"] =
                            json!("Missing")
                    }
                    21 => bad["AirLoopHVAC"] = json!({"Uncomposed airloop":{}}),
                    _ => unreachable!(),
                }
                let bad_raw =
                    parse_epjson_str(&bad.to_string()).expect("negative OA raw candidate");
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(&bad_raw).is_err(),
                    "method={method} invalid={invalid}"
                );
                let bad_compiled = ep_compiler::compile_raw_model_for_source(&bad_raw);
                assert!(
                    !inspect_porting_scope(
                        &bad_raw,
                        &bad_compiled.report,
                        bad_compiled.model.as_ref(),
                        PortingScope::B
                    )
                    .admissible,
                    "method={method} invalid={invalid}"
                );
            }
            for defaults in 0..3 {
                let mut actual = oa.clone();
                let requirement = actual["DesignSpecification:OutdoorAir"]["Actual OA"]
                    .as_object_mut()
                    .expect("DSOA");
                if defaults == 0 {
                    requirement.remove("outdoor_air_method");
                }
                if defaults == 1 {
                    requirement.insert("outdoor_air_method".into(), json!(""));
                }
                requirement.remove("outdoor_air_schedule_name");
                requirement
                    .remove("proportional_control_minimum_outdoor_air_flow_rate_schedule_name");
                if defaults == 2 {
                    requirement.insert("outdoor_air_schedule_name".into(), json!(""));
                    requirement.insert(
                        "proportional_control_minimum_outdoor_air_flow_rate_schedule_name".into(),
                        json!(""),
                    );
                }
                let actual_raw =
                    parse_epjson_str(&actual.to_string()).expect("actual DSOA defaults");
                crate::ordinary_equipment_input::resolve_source_b(&actual_raw)
                    .expect("retained blank/omitted DSOA fields");
                let actual_compiled = ep_compiler::compile_raw_model_for_source(&actual_raw);
                assert!(
                    !actual_compiled.has_errors(),
                    "{:?}",
                    actual_compiled.report.diagnostics
                );
                assert!(
                    inspect_porting_scope(
                        &actual_raw,
                        &actual_compiled.report,
                        actual_compiled.model.as_ref(),
                        PortingScope::B
                    )
                    .admissible
                );
            }
        }
    }

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
    // Both Source scopes use the genuine direct People/comfort chain. Other
    // sizing methods and positive optional comfort models retain their bounds.
    for scope in [PortingScope::A, PortingScope::B] {
        let mut people = input(scope);
        people["Schedule:Constant"]["Activity"] = json!({"hourly_value":120});
        people["People"] = json!({"Occupants":{"zone_or_zonelist_or_space_or_spacelist_name":"Zone","number_of_people_schedule_name":"On","number_of_people":1.5,"activity_level_schedule_name":"Activity","sensible_heat_fraction":0.6}});
        if scope == PortingScope::B {
            // The selected manager owns the existing supply Node; this extends
            // the actual support-assessment caller, not just raw/IP projection.
            people["SetpointManager:Scheduled"] = json!({"Supply setpoint":{
                "control_variable":"Temperature", "schedule_name":"Heat",
                "setpoint_node_or_nodelist_name":"Supply"
            }});
        }
        let mut actual = trace(&people, scope);
        assert!(actual.admissible, "{:?}", actual.violations);
        let raw = parse_epjson_str(&people.to_string()).expect("authored People scope");
        match scope {
            PortingScope::A => crate::ordinary_equipment_input::resolve_source_a(&raw),
            PortingScope::B => crate::ordinary_equipment_input::resolve_source_b(&raw),
        }
        .expect("actual People Source admission");
        assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        let compiled = compile_raw_model(&raw);
        let model = compiled
            .model
            .as_ref()
            .expect("actual compiled People model");
        actual.environment = Some(
            super::environment::environment_from_weather(
                model,
                &single_day_weather_fixture(),
                "authored People admission weather",
            )
            .expect("real People environment preparation"),
        );
        let admission = crate::support::SourceSupportAdmission::from_validated_scope(&raw, &actual)
            .expect("same validated People raw/IP scope");
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
        assert!(
            support
                .typed_objects
                .iter()
                .any(|row| row.object_type == "People")
        );
        if scope == PortingScope::A {
            let mut unused = people.clone();
            unused["NodeList"] = json!({
                "  A unused  ":{"nodes":[{"node_name":"  real unused node  ","ignored":"metadata"}],"ignored":"metadata"},
                "Z deferred":{"nodes":[{"node_name":"Tail A"},{"node_name":""},{"node_name":"Tail A"}]}
            });
            let raw =
                parse_epjson_str(&unused.to_string()).expect("actual Source A unused NodeLists");
            let retained = raw.clone();
            let compiled = ep_compiler::compile_raw_model_for_source(&raw);
            assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
            let model = compiled
                .model
                .as_ref()
                .expect("actual unused Source A model");
            assert_eq!(model.node_lists.len(), 2);
            assert!(model.node_lists.iter().any(|list| matches!(
                list.nodes,
                ep_model::NodeListMembers::DeferredSourceOriginalPrefix { .. }
            )));
            let mut actual = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
            assert!(actual.admissible, "{:?}", actual.violations);
            actual.environment = Some(
                super::environment::environment_from_weather(
                    model,
                    &single_day_weather_fixture(),
                    "authored unused NodeList Source A weather",
                )
                .expect("real A environment"),
            );
            let admission =
                crate::support::SourceSupportAdmission::from_validated_scope(&raw, &actual)
                    .expect("actual Source A unused-list custody");
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
            let ip =
                crate::ordinary_input_processor::resolve_source_a(&raw).expect("actual A raw IP");
            assert_eq!(
                ip.source_raw_object_names("NodeList")
                    .expect("actual A list order"),
                ["  A unused  ", "Z deferred"]
            );
            assert_eq!(raw, retained);
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
            for (family, objects) in [
                (
                    "SetpointManager:Scheduled",
                    json!({"manager":{"control_variable":"Temperature","schedule_name":"Heat","setpoint_node_or_nodelist_name":"unused"}}),
                ),
                (
                    "ZoneHVAC:EquipmentConnections",
                    json!({"connection":{"zone_name":"Zone","zone_conditioning_equipment_list_name":"list","zone_air_node_name":"air"}}),
                ),
            ] {
                let mut invalid = unused.clone();
                invalid[family] = objects;
                assert!(
                    !trace(&invalid, scope).admissible,
                    "Source A keeps {family} outside this lane"
                );
                assert!(
                    crate::ordinary_equipment_input::resolve_source_a(
                        &parse_epjson_str(&invalid.to_string()).expect("changed Source A family")
                    )
                    .is_err()
                );
            }
        }
        if scope == PortingScope::B {
            assert_eq!(model.setpoint_managers.len(), 1);
            assert!(
                !support
                    .unsupported_objects
                    .iter()
                    .any(|row| row.object_type == "SetpointManager:Scheduled")
            );
            let legacy = assess_support(
                &raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
            );
            assert_eq!(legacy.status, SupportStatus::Unsupported);
            assert!(
                legacy
                    .unsupported_objects
                    .iter()
                    .any(|row| row.object_type == "SetpointManager:Scheduled")
            );
            // Exercise the real compile/scope/support caller before runtime:
            // explicit trailing empty strings are advisory graph omissions,
            // while the original raw/IP cells remain the Node getter's input.
            let mut warning = people.clone();
            warning["NodeList"] = json!({"Supply List":{"nodes":[
                {"node_name":"Supply"},{"node_name":""},{"node_name":""}]}});
            warning["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["zone_supply_air_node_name"] =
                json!("Supply List");
            warning["ZoneHVAC:EquipmentConnections"]["Connection"]["zone_air_inlet_node_or_nodelist_name"] =
                json!("Supply List");
            warning["SetpointManager:Scheduled"]["Supply setpoint"]["setpoint_node_or_nodelist_name"] =
                json!("Supply List");
            let warning_raw =
                parse_epjson_str(&warning.to_string()).expect("actual trailing-empty Source raw");
            let original_warning_raw = warning_raw.clone();
            let old_compile = compile_raw_model(&warning_raw);
            assert!(old_compile.has_errors() && old_compile.model.is_none());
            assert!(
                old_compile
                    .report
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "MissingRequiredField"
                        && d.object_type == "NodeList"
                        && d.field.as_deref() == Some("node_name"))
            );
            let warning_compile = ep_compiler::compile_raw_model_for_source(&warning_raw);
            assert!(
                !warning_compile.has_errors(),
                "{:?}",
                warning_compile.report.diagnostics
            );
            let warning_model = warning_compile
                .model
                .as_ref()
                .expect("actual Source compiled warning model");
            assert_eq!(warning_model.node_lists.len(), 1);
            assert_eq!(
                warning_model.node_lists[0]
                    .nodes
                    .proven_nodes()
                    .expect("trailing blank leaves a proven prefix")
                    .len(),
                1
            );
            let simulation = ep_model::SimulationModel::from_typed(warning_model.clone());
            assert_eq!(simulation.graph.node_list_members.len(), 1);
            assert!(ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_ok());
            let mut warning_scope = inspect_porting_scope(
                &warning_raw,
                &warning_compile.report,
                Some(warning_model),
                scope,
            );
            assert!(warning_scope.admissible, "{:?}", warning_scope.violations);
            warning_scope.environment = Some(
                super::environment::environment_from_weather(
                    warning_model,
                    &single_day_weather_fixture(),
                    "authored trailing-empty Source admission weather",
                )
                .expect("actual warning-scope environment"),
            );
            let warning_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &warning_raw,
                &warning_scope,
            )
            .expect("actual raw/compiled warning Source admission");
            let warning_support = crate::support::assess_support_with_source_admission(
                &warning_raw,
                &warning_compile.report,
                Some(warning_model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&warning_admission),
            );
            assert!(
                warning_support.allows_rust_runtime(),
                "{:?}",
                warning_support.diagnostics
            );
            let warning_ip = crate::ordinary_input_processor::resolve_source_b(&warning_raw)
                .expect("actual Source IP transfer");
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as SourceRaw;
            let original_members = match &warning_ip
                .source_raw_object_fields("NodeList", "Supply List")
                .expect("actual raw NodeList fields")["nodes"]
            {
                SourceRaw::Array(members) => Some(members),
                _ => None,
            }
            .expect("actual retained member array");
            assert_eq!(original_members.len(), 3);
            for member in &original_members[1..] {
                let fields = match member {
                    SourceRaw::Object(fields) => Some(fields),
                    _ => None,
                }
                .expect("actual retained member object");
                assert_eq!(fields["node_name"], SourceRaw::String(String::new()));
            }
            assert_eq!(warning_raw, original_warning_raw);
            // Original-prefix holes remain explicit advisory prerequisites. An unused
            // declaration must not reject independent Source roles or compact a late duplicate.
            for members in [
                json!([{"node_name":"Supply"},{"node_name":""},{"node_name":"Supply"}]),
                json!([{"node_name":""},{"node_name":"Supply"}]),
                json!([{"node_name":"Supply"},{"node_name":""},{"node_name":"Other List"}]),
            ] {
                let mut unused = people.clone();
                unused["NodeList"] = json!({
                    "Unused original prefix":{"nodes":members},
                    "Other List":{"nodes":[{"node_name":"Supply"}]}
                });
                let raw =
                    parse_epjson_str(&unused.to_string()).expect("actual unused-hole Source raw");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("unused deferred Source model");
                let id = model
                    .node_list_names
                    .resolve("Unused original prefix")
                    .expect("real list");
                let list = model
                    .node_lists
                    .iter()
                    .find(|list| list.id == id)
                    .expect("real declaration");
                assert!(matches!(
                    list.nodes,
                    ep_model::NodeListMembers::DeferredSourceOriginalPrefix { .. }
                ));
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                assert_eq!(simulation.graph.deferred_source_node_lists.len(), 1);
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("unused deferred input does not obstruct proven Source roles");
                let observed = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(observed.admissible, "{:?}", observed.violations);
                crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("original raw arrays retain Source IP custody");
                assert_eq!(raw, retained);
            }
            for (name, valid) in [
                ("", false),
                (" \t\r\u{c}", false),
                ("X\nY", false),
                ("\u{b}", true),
                ("\u{a0}", true),
                (" Supply List ", true),
            ] {
                let mut exact = people.clone();
                exact["NodeList"] = json!({name:{"nodes":[{"node_name":"Supply"}]}});
                let raw =
                    parse_epjson_str(&exact.to_string()).expect("exact authored NodeList name");
                assert_eq!(
                    crate::ordinary_equipment_input::resolve_source_b(&raw).is_ok(),
                    valid,
                    "official NodeList object key {name:?}"
                );
                if valid {
                    let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                    let model = compiled.model.as_ref().expect("exact-key Source model");
                    let id = model
                        .node_list_names
                        .resolve(name)
                        .expect("exact Source identity");
                    assert_eq!(
                        model
                            .node_lists
                            .iter()
                            .find(|list| list.id == id)
                            .expect("real list")
                            .name
                            .0,
                        name.to_ascii_uppercase()
                    );
                    if name.starts_with(' ') {
                        assert!(model.node_list_names.resolve(name.trim()).is_none());
                    }
                }
            }
            for bad_member in [json!({"node_name":3}), json!({"node_name":"Supply"})] {
                let mut invalid = warning.clone();
                // The duplicate must lie inside the original shortened prefix.
                // A late duplicate after an internal hole remains Deferred.
                invalid["NodeList"]["Supply List"]["nodes"][1] = bad_member;
                let invalid_raw =
                    parse_epjson_str(&invalid.to_string()).expect("changed actual NodeList input");
                assert!(ep_compiler::compile_raw_model_for_source(&invalid_raw).has_errors());
            }
            // Exercise actual Source compilation, scope, support classification
            // and raw IP custody for return controls, without simulating flow or
            // reconstructing any private Config/Node arrays from typed fields.
            for (fraction, basis) in [(true, false), (false, true), (true, true)] {
                let mut returns = people.clone();
                returns["Schedule:Constant"]["Return Fraction"] = json!({"hourly_value":0.375});
                returns["NodeList"] = json!({"Return Basis":{"nodes":[
                    {"node_name":"Supply"},{"node_name":"Air"}
                ]}});
                if fraction {
                    returns["ZoneHVAC:EquipmentConnections"]["Connection"]["zone_return_air_node_1_flow_rate_fraction_schedule_name"] =
                        json!("Return Fraction");
                }
                if basis {
                    returns["ZoneHVAC:EquipmentConnections"]["Connection"]["zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name"] =
                        json!("Return Basis");
                }
                let raw = parse_epjson_str(&returns.to_string())
                    .expect("actual Source return controls raw");
                let unchanged = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("actual return controls typed model");
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("Source eligibility uses genuine existing return-control owners");
                assert!(ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_err());
                let mut actual_scope =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(actual_scope.admissible, "{:?}", actual_scope.violations);
                actual_scope.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "authored Source return-control admission weather",
                    )
                    .expect("actual Source return-control environment"),
                );
                let admission = crate::support::SourceSupportAdmission::from_validated_scope(
                    &raw,
                    &actual_scope,
                )
                .expect("real return controls Source admission token");
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
                assert_eq!(
                    support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                assert!(support.matched_capability_ids.is_empty());
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("actual return controls raw IP custody");
                let fields = ip
                    .source_raw_object_fields("ZoneHVAC:EquipmentConnections", "Connection")
                    .expect("actual return controls Connection fields");
                if fraction {
                    assert_eq!(
                        fields["zone_return_air_node_1_flow_rate_fraction_schedule_name"],
                        SourceRaw::String("Return Fraction".into())
                    );
                }
                if basis {
                    assert_eq!(
                        fields["zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name"],
                        SourceRaw::String("Return Basis".into())
                    );
                    let mut deferred = returns.clone();
                    deferred["NodeList"]["Return Basis"]["nodes"] = json!([
                        {"node_name":"Supply"},{"node_name":""},{"node_name":"Supply"}
                    ]);
                    let deferred_raw = parse_epjson_str(&deferred.to_string())
                        .expect("original unresolved return basis raw");
                    let deferred_compile = ep_compiler::compile_raw_model_for_source(&deferred_raw);
                    assert!(
                        !deferred_compile.has_errors(),
                        "{:?}",
                        deferred_compile.report.diagnostics
                    );
                    let deferred_model = deferred_compile
                        .model
                        .as_ref()
                        .expect("retained Deferred basis declaration");
                    assert!(
                        !inspect_porting_scope(
                            &deferred_raw,
                            &deferred_compile.report,
                            Some(deferred_model),
                            scope
                        )
                        .admissible
                    );
                }
                assert_eq!(raw, unchanged);
            }
            // A genuine optional Source exhaust remains outside the unchanged
            // legacy direct binding. Source eligibility returns no physical binding.
            let mut exhaust = people.clone();
            exhaust["ZoneHVAC:EquipmentConnections"]["Connection"]["zone_air_exhaust_node_or_nodelist_name"] =
                json!("Exhaust");
            exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["zone_exhaust_air_node_name"] =
                json!("Exhaust");
            exhaust["NodeList"] = json!({"Manager Targets":{"nodes":[{"node_name":"Supply"},{"node_name":"Exhaust"}]}});
            exhaust["SetpointManager:Scheduled"]["Supply setpoint"]["setpoint_node_or_nodelist_name"] =
                json!("Manager Targets");
            let exhaust_raw =
                parse_epjson_str(&exhaust.to_string()).expect("real Source exhaust raw");
            let original_exhaust_raw = exhaust_raw.clone();
            let exhaust_compile = ep_compiler::compile_raw_model_for_source(&exhaust_raw);
            assert!(
                !exhaust_compile.has_errors(),
                "{:?}",
                exhaust_compile.report.diagnostics
            );
            let exhaust_model = exhaust_compile
                .model
                .as_ref()
                .expect("actual Source exhaust typed model");
            let exhaust_simulation = ep_model::SimulationModel::from_typed(exhaust_model.clone());
            assert!(ep_runtime::bind_direct_zone_purchased_air_model(&exhaust_simulation).is_err());
            ep_runtime::validate_source_direct_zone_purchased_air_model(&exhaust_simulation)
                .expect("matched real Source exhaust eligibility");
            let mut exhaust_scope = inspect_porting_scope(
                &exhaust_raw,
                &exhaust_compile.report,
                Some(exhaust_model),
                scope,
            );
            assert!(exhaust_scope.admissible, "{:?}", exhaust_scope.violations);
            exhaust_scope.environment = Some(
                super::environment::environment_from_weather(
                    exhaust_model,
                    &single_day_weather_fixture(),
                    "authored real exhaust admission weather",
                )
                .expect("actual exhaust environment"),
            );
            let exhaust_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &exhaust_raw,
                &exhaust_scope,
            )
            .expect("same actual Source B/raw exhaust custody");
            let exhaust_support = crate::support::assess_support_with_source_admission(
                &exhaust_raw,
                &exhaust_compile.report,
                Some(exhaust_model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&exhaust_admission),
            );
            assert!(
                exhaust_support.allows_rust_runtime(),
                "{:?}",
                exhaust_support.diagnostics
            );
            assert_eq!(
                exhaust_support.runtime_class,
                crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
            );
            assert!(exhaust_support.matched_capability_ids.is_empty());
            assert!(
                !exhaust_support
                    .diagnostics
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "IdealLoadsFixtureDemandDiagnosticOnly")
            );
            let legacy_exhaust = assess_support(
                &exhaust_raw,
                &exhaust_compile.report,
                Some(exhaust_model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
            );
            assert!(!legacy_exhaust.allows_rust_runtime());
            let exhaust_ip = crate::ordinary_input_processor::resolve_source_b(&exhaust_raw)
                .expect("real Source exhaust IP custody");
            assert_eq!(
                exhaust_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("real exhaust object")["zone_exhaust_air_node_name"],
                SourceRaw::String("Exhaust".into())
            );
            assert_eq!(exhaust_raw, original_exhaust_raw);

            // Explicit JSON empty fuel remains authored through the real
            // compiler/scope/token/support/IP path. Typed defaults are advisory;
            // the separate real runtime getter owns Invalid and actual meters.
            for (heating, cooling) in [
                (Some(""), None),
                (None, Some("")),
                (Some(""), Some("")),
                (None, None),
            ] {
                let mut fuel_input = exhaust.clone();
                for (field, authored) in [
                    ("heating_fuel_type", heating),
                    ("cooling_fuel_type", cooling),
                ] {
                    if let Some(value) = authored {
                        fuel_input["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][field] = json!(value);
                    }
                }
                let raw = parse_epjson_str(&fuel_input.to_string()).expect("real empty-fuel JSON");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("real empty-fuel advisory model");
                assert_eq!(
                    model.ideal_loads_air_systems[0].heating_fuel_type,
                    ep_model::IdealLoadsFuelType::DistrictHeatingWater
                );
                assert_eq!(
                    model.ideal_loads_air_systems[0].cooling_fuel_type,
                    ep_model::IdealLoadsFuelType::DistrictCooling
                );
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("real raw-owner Source eligibility");
                let mut selected =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(selected.admissible, "{:?}", selected.violations);
                selected.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "authored empty-fuel custody weather",
                    )
                    .expect("real empty-fuel environment"),
                );
                let admission =
                    crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                        .expect("actual empty-fuel Source admission");
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
                assert!(support.matched_capability_ids.is_empty());
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("real fuel raw IP");
                let fields = ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained real fuel fields");
                for (field, authored) in [
                    ("heating_fuel_type", heating),
                    ("cooling_fuel_type", cooling),
                ] {
                    assert_eq!(
                        fields.get(field),
                        authored
                            .map(|value| SourceRaw::String(value.into()))
                            .as_ref()
                    );
                }
                assert_eq!(raw, retained);
                for field in ["heating_fuel_type", "cooling_fuel_type"] {
                    for value in [
                        json!(null),
                        json!(" "),
                        json!("Water"),
                        json!("None"),
                        json!(7),
                        json!(true),
                    ] {
                        let mut invalid = fuel_input.clone();
                        invalid["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][field] = value;
                        let invalid_raw = parse_epjson_str(&invalid.to_string())
                            .expect("malformed fuel retained raw");
                        assert!(
                            crate::ordinary_equipment_input::resolve_source_b(&invalid_raw)
                                .is_err()
                        );
                        assert!(
                            crate::support::SourceSupportAdmission::from_validated_scope(
                                &invalid_raw,
                                &selected
                            )
                            .is_none()
                        );
                    }
                }
            }

            // Unit126: actual compile -> scope -> support -> retained raw IP
            // custody for selected mixed-ASCII/UTF-8 identities. The runtime
            // owner probe remains separate; this is not a simulation result.
            {
                let alias = "ZONE one inlets 가";
                let physical = "Zone one inlet 가";
                let unit_name = "ZONE one ideal loads 각";
                let mut utf8 = exhaust.clone();
                utf8["NodeList"][alias] = json!({"nodes":[{"node_name":physical}]});
                utf8["NodeList"]["Manager Targets"]["nodes"][0]["node_name"] = json!(physical);
                utf8["ZoneHVAC:EquipmentConnections"]["Connection"]["zone_air_inlet_node_or_nodelist_name"] =
                    json!(alias);
                let mut actual_unit = utf8["ZoneHVAC:IdealLoadsAirSystem"]
                    .as_object_mut()
                    .expect("real unit object map")
                    .remove("Ideal")
                    .expect("real selected unit");
                actual_unit["zone_supply_air_node_name"] = json!(alias);
                utf8["ZoneHVAC:IdealLoadsAirSystem"][unit_name] = actual_unit;
                utf8["ZoneHVAC:EquipmentList"]["Equipment"]["equipment"][0]["zone_equipment_name"] =
                    json!(unit_name);
                utf8["Output:Variable"] = json!({
                    "UTF8 unit":{"key_value":unit_name,"variable_name":"Zone Ideal Loads Supply Air Sensible Heating Rate","reporting_frequency":"Timestep"},
                    "UTF8 node":{"key_value":physical,"variable_name":"System Node Mass Flow Rate","reporting_frequency":"Timestep"}
                });
                let raw = parse_epjson_str(&utf8.to_string()).expect("real UTF-8 Source input");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled.model.as_ref().expect("real UTF-8 typed model");
                let resolved = model
                    .resolve_advisory_node_reference(alias)
                    .require_proven_nodes()
                    .expect("real selected singleton alias");
                assert_eq!(resolved.len(), 1);
                let actual_node = model
                    .nodes
                    .iter()
                    .find(|node| node.id == resolved[0])
                    .expect("real physical node arena member");
                assert_eq!(
                    actual_node.name.0.as_bytes(),
                    "ZONE ONE INLET 가".as_bytes()
                );
                assert_eq!(
                    model.ideal_loads_air_systems[0].name.0.as_bytes(),
                    "ZONE ONE IDEAL LOADS 각".as_bytes()
                );
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("actual Source role eligibility");
                let mut actual_scope =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(actual_scope.admissible, "{:?}", actual_scope.violations);
                actual_scope.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "authored UTF-8 identity weather",
                    )
                    .expect("real UTF-8 scope environment"),
                );
                let admission = crate::support::SourceSupportAdmission::from_validated_scope(
                    &raw,
                    &actual_scope,
                )
                .expect("actual UTF-8 raw/IP Source token");
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
                assert_eq!(
                    support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                assert!(support.matched_capability_ids.is_empty());
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("real UTF-8 raw IP custody");
                let actual_parent = ip
                    .source_raw_object_fields("ZoneHVAC:EquipmentConnections", "Connection")
                    .expect("real parent alpha");
                assert_eq!(
                    actual_parent["zone_air_inlet_node_or_nodelist_name"],
                    SourceRaw::String(alias.into())
                );
                let actual_unit = ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", unit_name)
                    .expect("exact original unit key");
                assert_eq!(
                    actual_unit["zone_supply_air_node_name"],
                    SourceRaw::String(alias.into())
                );
                assert_eq!(
                    ip.source_raw_object_names("ZoneHVAC:IdealLoadsAirSystem")
                        .expect("actual unit names"),
                    &[unit_name.to_owned()]
                );
                let actual_list = ip
                    .source_raw_object_fields("NodeList", alias)
                    .expect("exact original alias key");
                let members = match &actual_list["nodes"] {
                    SourceRaw::Array(members) => Some(members),
                    _ => None,
                }
                .expect("actual members");
                let member = match &members[0] {
                    SourceRaw::Object(member) => Some(member),
                    _ => None,
                }
                .expect("actual first member");
                assert_eq!(member["node_name"], SourceRaw::String(physical.into()));
                for (request, key) in [("UTF8 unit", unit_name), ("UTF8 node", physical)] {
                    assert_eq!(
                        ip.source_raw_object_fields("Output:Variable", request)
                            .expect("actual retaincase output key")["key_value"],
                        SourceRaw::String(key.into())
                    );
                }
                assert_eq!(raw, retained);
            }

            // The same real compiler/scope/token/IP caller also covers heating-only
            // and both constant capacity shapes, without physical HVAC owner seeds.
            for (limit, dehumidification, humidification) in [
                ("NoLimit", "SCHEMA_DEFAULT", "None"),
                ("NoLimit", "ConstantSensibleHeatRatio", "None"),
                ("LimitFlowRate", "ConstantSensibleHeatRatio", "None"),
                ("LimitCapacity", "ConstantSensibleHeatRatio", "None"),
                (
                    "LimitFlowRateAndCapacity",
                    "ConstantSensibleHeatRatio",
                    "None",
                ),
                (
                    "NoLimit",
                    "ConstantSensibleHeatRatio",
                    "ConstantSupplyHumidityRatio",
                ),
                (
                    "LimitFlowRate",
                    "ConstantSensibleHeatRatio",
                    "ConstantSupplyHumidityRatio",
                ),
                (
                    "LimitCapacity",
                    "ConstantSensibleHeatRatio",
                    "ConstantSupplyHumidityRatio",
                ),
                (
                    "LimitFlowRateAndCapacity",
                    "ConstantSensibleHeatRatio",
                    "ConstantSupplyHumidityRatio",
                ),
                (
                    "LimitCapacity",
                    "ConstantSupplyHumidityRatio",
                    "ConstantSupplyHumidityRatio",
                ),
                (
                    "LimitFlowRateAndCapacity",
                    "ConstantSupplyHumidityRatio",
                    "ConstantSupplyHumidityRatio",
                ),
                ("LimitCapacity", "None", "ConstantSupplyHumidityRatio"),
                (
                    "LimitFlowRateAndCapacity",
                    "None",
                    "ConstantSupplyHumidityRatio",
                ),
                ("LimitCapacity", "None", "None"),
                ("LimitFlowRateAndCapacity", "None", "None"),
                ("LimitCapacity", "ConstantSupplyHumidityRatio", "None"),
                (
                    "LimitFlowRateAndCapacity",
                    "ConstantSupplyHumidityRatio",
                    "None",
                ),
                ("NoLimit", "ConstantSupplyHumidityRatio", "None"),
                ("LimitFlowRate", "ConstantSupplyHumidityRatio", "None"),
                ("NoLimit", "None", "ConstantSupplyHumidityRatio"),
                ("LimitFlowRate", "None", "ConstantSupplyHumidityRatio"),
                (
                    "NoLimit",
                    "ConstantSupplyHumidityRatio",
                    "ConstantSupplyHumidityRatio",
                ),
                (
                    "LimitFlowRate",
                    "ConstantSupplyHumidityRatio",
                    "ConstantSupplyHumidityRatio",
                ),
            ] {
                let humidity_ratio = 0.006_123_456_789_f64;
                let omitted_selector = dehumidification == "SCHEMA_DEFAULT";
                let selected_dehumidification = if omitted_selector {
                    "ConstantSensibleHeatRatio"
                } else {
                    dehumidification
                };
                let shr = if humidification == "None" {
                    0.35_f64
                } else {
                    1.0_f64
                };
                let mut humidity = people.clone();
                if omitted_selector {
                    humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]
                        .as_object_mut()
                        .expect("real default SHR object")
                        .remove("dehumidification_control_type");
                } else {
                    humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["dehumidification_control_type"] =
                        json!(dehumidification);
                }
                if selected_dehumidification == "ConstantSensibleHeatRatio" && !omitted_selector {
                    humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["cooling_sensible_heat_ratio"] =
                        json!(shr);
                }
                humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["humidification_control_type"] =
                    json!(humidification);
                humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["cooling_limit"] = json!(limit);
                humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_cooling_air_flow_rate"] =
                    json!(0.2);
                if matches!(limit, "LimitCapacity" | "LimitFlowRateAndCapacity") {
                    humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_total_cooling_capacity"] =
                        json!(900.0);
                }
                humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["minimum_cooling_supply_air_humidity_ratio"] =
                    json!(humidity_ratio);
                let raw =
                    parse_epjson_str(&humidity.to_string()).expect("constant cooling raw input");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("real constant cooling typed model");
                let unit = &model.ideal_loads_air_systems[0];
                assert_eq!(
                    unit.dehumidification_control_type,
                    match selected_dehumidification {
                        "None" => DehumidificationControlType::None,
                        "ConstantSensibleHeatRatio" =>
                            DehumidificationControlType::ConstantSensibleHeatRatio,
                        _ => DehumidificationControlType::ConstantSupplyHumidityRatio,
                    }
                );
                if selected_dehumidification == "ConstantSensibleHeatRatio" {
                    let expected = if omitted_selector { 0.7_f64 } else { shr };
                    assert_eq!(
                        unit.cooling_sensible_heat_ratio.to_bits(),
                        expected.to_bits()
                    );
                }
                assert_eq!(
                    unit.humidification_control_type,
                    if humidification == "None" {
                        HumidificationControlType::None
                    } else {
                        HumidificationControlType::ConstantSupplyHumidityRatio
                    }
                );
                assert_eq!(
                    unit.minimum_cooling_supply_air_humidity_ratio.to_bits(),
                    humidity_ratio.to_bits()
                );
                if matches!(limit, "LimitCapacity" | "LimitFlowRateAndCapacity") {
                    assert!(matches!(unit.maximum_total_cooling_capacity_w,
                        Some(ep_model::AutosizeOrNumber::Value(value)) if value.to_bits() == 900.0_f64.to_bits()));
                }
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("Source constant cooling eligibility");
                if dehumidification == "None" && humidification == "None" {
                    ep_runtime::bind_direct_zone_purchased_air_model(&simulation)
                        .expect("unchanged Legacy sensible numeric-capacity binding");
                } else {
                    assert!(ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_err());
                }
                let mut actual_scope =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(actual_scope.admissible, "{:?}", actual_scope.violations);
                actual_scope.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "constant cooling admission weather",
                    )
                    .expect("real constant cooling environment"),
                );
                let admission = crate::support::SourceSupportAdmission::from_validated_scope(
                    &raw,
                    &actual_scope,
                )
                .expect("same raw constant cooling Source token");
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
                assert_eq!(
                    support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                if dehumidification == "None" && humidification == "None" {
                    assert!(!support.matched_capability_ids.is_empty());
                } else {
                    assert!(support.matched_capability_ids.is_empty());
                }
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("real constant cooling raw IP");
                let fields = ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained constant cooling alpha/numeric fields");
                if omitted_selector {
                    assert!(!fields.contains_key("dehumidification_control_type"));
                    assert!(!fields.contains_key("cooling_sensible_heat_ratio"));
                } else {
                    assert_eq!(
                        fields["dehumidification_control_type"],
                        SourceRaw::String(dehumidification.into())
                    );
                    if selected_dehumidification == "ConstantSensibleHeatRatio" {
                        let actual = match &fields["cooling_sensible_heat_ratio"] {
                            SourceRaw::Number(token) => Some(token),
                            _ => None,
                        }
                        .expect("real explicit SHR numeric token");
                        assert_eq!(
                            actual.parse::<f64>().expect("finite SHR").to_bits(),
                            shr.to_bits()
                        );
                    }
                }
                assert_eq!(
                    fields["humidification_control_type"],
                    SourceRaw::String(humidification.into())
                );
                assert_eq!(fields["cooling_limit"], SourceRaw::String(limit.into()));
                if matches!(limit, "LimitCapacity" | "LimitFlowRateAndCapacity") {
                    let token = match &fields["maximum_total_cooling_capacity"] {
                        SourceRaw::Number(token) => Some(token),
                        _ => None,
                    }
                    .expect("actual hard-sized capacity token");
                    assert_eq!(
                        token
                            .parse::<f64>()
                            .expect("actual capacity scalar")
                            .to_bits(),
                        900.0_f64.to_bits()
                    );
                }
                let token = match &fields["minimum_cooling_supply_air_humidity_ratio"] {
                    SourceRaw::Number(token) => Some(token),
                    _ => None,
                }
                .expect("real retained humidity-ratio numeric token");
                assert_eq!(
                    token
                        .parse::<f64>()
                        .expect("actual humidity scalar")
                        .to_bits(),
                    humidity_ratio.to_bits()
                );
                assert_eq!(raw, retained);
            }
            let mut empty_dehumidification = people.clone();
            empty_dehumidification["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["dehumidification_control_type"] =
                json!("");
            let empty_raw = parse_epjson_str(&empty_dehumidification.to_string())
                .expect("direct epJSON empty alpha");
            assert!(empty_raw.is_epjson());
            let empty_compile = ep_compiler::compile_raw_model_for_source(&empty_raw);
            assert!(
                !empty_compile.has_errors(),
                "{:?}",
                empty_compile.report.diagnostics
            );
            let empty_model = empty_compile
                .model
                .as_ref()
                .expect("existing typed default projection");
            assert_eq!(
                empty_model.ideal_loads_air_systems[0].dehumidification_control_type,
                DehumidificationControlType::ConstantSensibleHeatRatio
            );
            let empty_error = crate::ordinary_input_processor::resolve_source_b(&empty_raw)
                .expect_err("Source must not confuse a direct empty enum with omitted SHR");
            assert!(
                empty_error.contains("explicit-empty dehumidification"),
                "{empty_error}"
            );
            assert!(
                !inspect_porting_scope(&empty_raw, &empty_compile.report, Some(empty_model), scope)
                    .admissible
            );

            let mut invalid_shr = people.clone();
            invalid_shr["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["dehumidification_control_type"] =
                json!("ConstantSensibleHeatRatio");
            invalid_shr["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["cooling_sensible_heat_ratio"] =
                json!(0.0);
            let invalid_raw =
                parse_epjson_str(&invalid_shr.to_string()).expect("real zero SHR token");
            let invalid_compile = ep_compiler::compile_raw_model_for_source(&invalid_raw);
            assert!(
                !invalid_compile.has_errors(),
                "{:?}",
                invalid_compile.report.diagnostics
            );
            let invalid_model = invalid_compile
                .model
                .as_ref()
                .expect("typed zero SHR token");
            assert!(
                !inspect_porting_scope(
                    &invalid_raw,
                    &invalid_compile.report,
                    Some(invalid_model),
                    scope
                )
                .admissible
            );

            // The real Humidistat producer and its consumer retain original raw
            // schedule pointers. This test validates admission and custody only.
            for high in [None, Some(""), Some("Cool")] {
                let mut humidity = people.clone();
                humidity["ZoneControl:Humidistat"] = json!({"Humidity owner":{
                    "zone_name":"Zone",
                    "humidifying_relative_humidity_setpoint_schedule_name":"Heat"
                }});
                if let Some(high) = high {
                    humidity["ZoneControl:Humidistat"]["Humidity owner"]["dehumidifying_relative_humidity_setpoint_schedule_name"] =
                        json!(high);
                }
                let raw =
                    parse_epjson_str(&humidity.to_string()).expect("real Humidistat raw operands");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("real Source Humidistat typed model");
                let control = &model.zone_humidistats[0];
                if high.is_none() || high == Some("") {
                    assert_eq!(
                        control.humidifying_relative_humidity_setpoint_schedule,
                        control.dehumidifying_relative_humidity_setpoint_schedule
                    );
                    assert!(ep_compiler::compile_raw_model(&raw).has_errors());
                } else {
                    assert_ne!(
                        control.humidifying_relative_humidity_setpoint_schedule,
                        control.dehumidifying_relative_humidity_setpoint_schedule
                    );
                    assert!(!ep_compiler::compile_raw_model(&raw).has_errors());
                }
                let scope_trace = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(scope_trace.admissible, "{:?}", scope_trace.violations);
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("real four-alpha Humidistat IP");
                assert_eq!(ip.get_num_objects_found("ZoneControl:Humidistat"), 1);
                let mut statistics =
                    ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                ip.report_source_idf_records_stats(&mut statistics)
                    .expect("admitted Humidistat reaches actual post-allocation input statistics");
                let actual = ip
                    .source_raw_object_fields("ZoneControl:Humidistat", "Humidity owner")
                    .expect("actual Humidistat raw fields");
                assert_eq!(
                    actual["humidifying_relative_humidity_setpoint_schedule_name"],
                    SourceRaw::String("Heat".into())
                );
                if let Some(high) = high {
                    assert_eq!(
                        actual["dehumidifying_relative_humidity_setpoint_schedule_name"],
                        SourceRaw::String(high.into())
                    );
                } else {
                    assert!(
                        !actual
                            .contains_key("dehumidifying_relative_humidity_setpoint_schedule_name")
                    );
                }
                humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["dehumidification_control_type"] =
                    json!("Humidistat");
                let selected = parse_epjson_str(&humidity.to_string())
                    .expect("actual connected HVAC Humidistat selector");
                let selected_compile = ep_compiler::compile_raw_model_for_source(&selected);
                assert!(!selected_compile.has_errors());
                assert!(
                    inspect_porting_scope(
                        &selected,
                        &selected_compile.report,
                        selected_compile.model.as_ref(),
                        scope
                    )
                    .admissible
                );
                crate::ordinary_input_processor::resolve_source_b(&selected)
                    .expect("same actual connected Humidistat Source IP");
                assert_eq!(raw, retained);
            }
            // A missing ZoneControl object is an actual Native zero-demand path,
            // not a fabricated moisture load. Exercise the real raw/typed token.
            for (dehumidification, humidification) in [
                ("Humidistat", "None"),
                ("ConstantSupplyHumidityRatio", "Humidistat"),
                ("None", "Humidistat"),
                ("Humidistat", "Humidistat"),
            ] {
                let mut humidity = people.clone();
                humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["dehumidification_control_type"] =
                    json!(dehumidification);
                humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["humidification_control_type"] =
                    json!(humidification);
                let raw =
                    parse_epjson_str(&humidity.to_string()).expect("actual absent-control input");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("actual absent-control typed model");
                assert!(model.zone_humidistats.is_empty());
                let mut selected =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(selected.admissible, "{:?}", selected.violations);
                selected.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "actual absent Humidistat control weather",
                    )
                    .expect("actual Source scope environment"),
                );
                let admission =
                    crate::support::SourceSupportAdmission::from_validated_scope(&raw, &selected)
                        .expect("actual absence-path Source token");
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
                assert!(support.matched_capability_ids.is_empty());
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("actual absent-control Source IP");
                assert_eq!(ip.get_num_objects_found("ZoneControl:Humidistat"), 0);
                assert_eq!(raw, retained);
                if dehumidification == "Humidistat" {
                    humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["humidification_control_type"] =
                        json!("");
                    let unresolved = parse_epjson_str(&humidity.to_string())
                        .expect("actual explicit-empty humidity selector");
                    let unresolved_compile = ep_compiler::compile_raw_model_for_source(&unresolved);
                    assert!(!unresolved_compile.has_errors());
                    assert!(
                        crate::ordinary_equipment_input::resolve_source_b(&unresolved).is_err()
                    );
                    assert!(
                        !inspect_porting_scope(
                            &unresolved,
                            &unresolved_compile.report,
                            unresolved_compile.model.as_ref(),
                            scope
                        )
                        .admissible
                    );
                }
            }

            // Unit127: a real cooling schedule reaches Source eligibility,
            // scope/support and raw IP; this does not simulate any HVAC flow.
            for value in [0.0_f64, 1.0] {
                let name = "127 Cooling Availability";
                let mut availability = people.clone();
                availability["Schedule:Constant"][name] = json!({"hourly_value":value});
                availability["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["cooling_availability_schedule_name"] =
                    json!(name);
                let raw = parse_epjson_str(&availability.to_string())
                    .expect("real cooling availability input");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("real cooling availability typed model");
                let unit = &model.ideal_loads_air_systems[0];
                let selected = unit
                    .cooling_availability_schedule
                    .expect("actual selected cooling schedule");
                let schedule = model
                    .schedules
                    .iter()
                    .find(|row| row.id == selected)
                    .expect("real schedule arena member");
                assert_eq!(schedule.hourly_value.to_bits(), value.to_bits());
                assert!(unit.heating_availability_schedule.is_none());
                assert_ne!(unit.availability_schedule, Some(selected));
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("actual Source cooling schedule eligibility");
                assert_eq!(
                    ep_runtime::bind_direct_zone_purchased_air_model(&simulation).expect_err("Legacy cooling fence"),
                    ep_runtime::DirectZonePurchasedAirBindingError::UnsupportedFeature {
                        feature: ep_runtime::DirectZonePurchasedAirBindingFeature::NoCoolingAvailabilitySchedule,
                    }
                );
                let mut actual_scope =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(actual_scope.admissible, "{:?}", actual_scope.violations);
                actual_scope.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "authored cooling availability weather",
                    )
                    .expect("real cooling availability environment"),
                );
                let admission = crate::support::SourceSupportAdmission::from_validated_scope(
                    &raw,
                    &actual_scope,
                )
                .expect("same raw cooling schedule admission");
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
                assert_eq!(
                    support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                assert!(support.matched_capability_ids.is_empty());
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("actual cooling schedule raw IP");
                let fields = ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("real unit alphas");
                assert_eq!(
                    fields["cooling_availability_schedule_name"],
                    SourceRaw::String(name.into())
                );
                assert!(!fields.contains_key("heating_availability_schedule_name"));
                let fields = ip
                    .source_raw_object_fields("Schedule:Constant", name)
                    .expect("actual schedule token");
                let token = match &fields["hourly_value"] {
                    SourceRaw::Number(token) => Some(token),
                    _ => None,
                }
                .expect("real retained schedule number");
                assert_eq!(
                    token.parse::<f64>().expect("actual finite token").to_bits(),
                    value.to_bits()
                );
                assert_eq!(raw, retained);
                let mut heating = availability;
                heating["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heating_availability_schedule_name"] =
                    json!(name);
                let heating_raw = parse_epjson_str(&heating.to_string())
                    .expect("real still-fenced heating declaration");
                let heating_compile = ep_compiler::compile_raw_model_for_source(&heating_raw);
                assert!(
                    !heating_compile.has_errors(),
                    "{:?}",
                    heating_compile.report.diagnostics
                );
                let heating_model = heating_compile
                    .model
                    .as_ref()
                    .expect("actual heating typed model");
                ep_runtime::validate_source_direct_zone_purchased_air_model(
                    &ep_model::SimulationModel::from_typed(heating_model.clone()),
                )
                .expect("Source admits actual heating schedule declaration");
                let heating_schedule = heating_model.ideal_loads_air_systems[0]
                    .heating_availability_schedule
                    .expect("actual typed heating schedule ID");
                assert_eq!(
                    heating_model
                        .schedules
                        .iter()
                        .find(|row| row.id == heating_schedule)
                        .expect("actual heating schedule arena")
                        .hourly_value
                        .to_bits(),
                    value.to_bits()
                );
                let mut heating_scope = inspect_porting_scope(
                    &heating_raw,
                    &heating_compile.report,
                    Some(heating_model),
                    scope,
                );
                assert!(heating_scope.admissible, "{:?}", heating_scope.violations);
                heating_scope.environment = Some(
                    super::environment::environment_from_weather(
                        heating_model,
                        &single_day_weather_fixture(),
                        "authored heating role weather",
                    )
                    .expect("actual heating scope environment"),
                );
                let heating_admission =
                    crate::support::SourceSupportAdmission::from_validated_scope(
                        &heating_raw,
                        &heating_scope,
                    )
                    .expect("real heating Source token");
                let heating_support = crate::support::assess_support_with_source_admission(
                    &heating_raw,
                    &heating_compile.report,
                    Some(heating_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&heating_admission),
                );
                assert!(
                    heating_support.allows_rust_runtime(),
                    "{:?}",
                    heating_support.diagnostics
                );
                assert_eq!(
                    heating_support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                assert!(heating_support.matched_capability_ids.is_empty());
                let heating_ip = crate::ordinary_input_processor::resolve_source_b(&heating_raw)
                    .expect("real heating raw IP");
                let heating_fields = heating_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained heating unit alphas");
                assert_eq!(
                    heating_fields["heating_availability_schedule_name"],
                    SourceRaw::String(name.into())
                );
                assert_eq!(ep_runtime::bind_direct_zone_purchased_air_model(&ep_model::SimulationModel::from_typed(heating_model.clone())).expect_err("Legacy heating role remains fenced"), ep_runtime::DirectZonePurchasedAirBindingError::UnsupportedFeature { feature: ep_runtime::DirectZonePurchasedAirBindingFeature::NoHeatingAvailabilitySchedule });
            }

            // Unit129: a real cooling-fuel schedule reaches Source eligibility,
            // scope/support and raw IP; this does not simulate any HVAC flow.
            for value in [0.0_f64, 0.5, 2.0] {
                let name = "129 Cooling Fuel Efficiency";
                let mut efficiency = people.clone();
                efficiency["Schedule:Constant"][name] = json!({"hourly_value":value});
                efficiency["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["cooling_fuel_efficiency_schedule_name"] =
                    json!(name);
                let raw = parse_epjson_str(&efficiency.to_string())
                    .expect("real cooling efficiency input");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("real cooling efficiency typed model");
                let unit = &model.ideal_loads_air_systems[0];
                let selected = unit
                    .cooling_fuel_efficiency_schedule
                    .expect("actual selected cooling schedule");
                let schedule = model
                    .schedules
                    .iter()
                    .find(|row| row.id == selected)
                    .expect("real schedule arena member");
                assert_eq!(schedule.hourly_value.to_bits(), value.to_bits());
                assert!(unit.heating_fuel_efficiency_schedule.is_none());
                assert!(unit.heating_availability_schedule.is_none());
                assert!(unit.cooling_availability_schedule.is_none());
                assert_ne!(unit.availability_schedule, Some(selected));
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("actual Source cooling schedule eligibility");
                assert_eq!(
                    ep_runtime::bind_direct_zone_purchased_air_model(&simulation).expect_err("Legacy cooling fuel fence"),
                    ep_runtime::DirectZonePurchasedAirBindingError::UnsupportedFeature {
                        feature: ep_runtime::DirectZonePurchasedAirBindingFeature::NoFuelEfficiencySchedules,
                    }
                );
                let mut actual_scope =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(actual_scope.admissible, "{:?}", actual_scope.violations);
                actual_scope.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "authored cooling efficiency weather",
                    )
                    .expect("real cooling efficiency environment"),
                );
                let admission = crate::support::SourceSupportAdmission::from_validated_scope(
                    &raw,
                    &actual_scope,
                )
                .expect("same raw cooling schedule admission");
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
                assert_eq!(
                    support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                assert!(support.matched_capability_ids.is_empty());
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("actual cooling schedule raw IP");
                let fields = ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("real unit alphas");
                assert_eq!(
                    fields["cooling_fuel_efficiency_schedule_name"],
                    SourceRaw::String(name.into())
                );
                assert!(!fields.contains_key("heating_fuel_efficiency_schedule_name"));
                let fields = ip
                    .source_raw_object_fields("Schedule:Constant", name)
                    .expect("actual schedule token");
                let token = match &fields["hourly_value"] {
                    SourceRaw::Number(token) => Some(token),
                    _ => None,
                }
                .expect("real retained schedule number");
                assert_eq!(
                    token.parse::<f64>().expect("actual finite token").to_bits(),
                    value.to_bits()
                );
                assert_eq!(raw, retained);
                let mut heating = efficiency;
                heating["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heating_fuel_efficiency_schedule_name"] =
                    json!(name);
                let heating_raw = parse_epjson_str(&heating.to_string())
                    .expect("real still-fenced heating declaration");
                let heating_compile = ep_compiler::compile_raw_model_for_source(&heating_raw);
                assert!(
                    !heating_compile.has_errors(),
                    "{:?}",
                    heating_compile.report.diagnostics
                );
                let heating_model = heating_compile
                    .model
                    .as_ref()
                    .expect("actual heating typed model");
                ep_runtime::validate_source_direct_zone_purchased_air_model(
                    &ep_model::SimulationModel::from_typed(heating_model.clone()),
                )
                .expect("Source admits actual heating schedule declaration");
                let heating_schedule = heating_model.ideal_loads_air_systems[0]
                    .heating_fuel_efficiency_schedule
                    .expect("actual typed heating schedule ID");
                assert_eq!(
                    heating_model
                        .schedules
                        .iter()
                        .find(|row| row.id == heating_schedule)
                        .expect("actual heating schedule arena")
                        .hourly_value
                        .to_bits(),
                    value.to_bits()
                );
                let mut heating_scope = inspect_porting_scope(
                    &heating_raw,
                    &heating_compile.report,
                    Some(heating_model),
                    scope,
                );
                assert!(heating_scope.admissible, "{:?}", heating_scope.violations);
                heating_scope.environment = Some(
                    super::environment::environment_from_weather(
                        heating_model,
                        &single_day_weather_fixture(),
                        "authored heating role weather",
                    )
                    .expect("actual heating scope environment"),
                );
                let heating_admission =
                    crate::support::SourceSupportAdmission::from_validated_scope(
                        &heating_raw,
                        &heating_scope,
                    )
                    .expect("real heating Source token");
                let heating_support = crate::support::assess_support_with_source_admission(
                    &heating_raw,
                    &heating_compile.report,
                    Some(heating_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&heating_admission),
                );
                assert!(
                    heating_support.allows_rust_runtime(),
                    "{:?}",
                    heating_support.diagnostics
                );
                assert_eq!(
                    heating_support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                assert!(heating_support.matched_capability_ids.is_empty());
                let heating_ip = crate::ordinary_input_processor::resolve_source_b(&heating_raw)
                    .expect("real heating raw IP");
                let heating_fields = heating_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained heating unit alphas");
                assert_eq!(
                    heating_fields["heating_fuel_efficiency_schedule_name"],
                    SourceRaw::String(name.into())
                );
                assert_eq!(ep_runtime::bind_direct_zone_purchased_air_model(&ep_model::SimulationModel::from_typed(heating_model.clone())).expect_err("Legacy heating role remains fenced"), ep_runtime::DirectZonePurchasedAirBindingError::UnsupportedFeature { feature: ep_runtime::DirectZonePurchasedAirBindingFeature::NoFuelEfficiencySchedules });
            }

            // Full actual raw compiler -> scope -> Source token -> runtime class
            // -> IP custody for ordered return counts, without simulating flow.
            for count in [0_usize, 1, 3, 5] {
                let mut returns = exhaust.clone();
                let names = (0..count)
                    .map(|number| {
                        if number == 0 {
                            "Return".to_owned()
                        } else {
                            format!("Return {}", number + 1)
                        }
                    })
                    .collect::<Vec<_>>();
                if count == 0 {
                    returns["ZoneHVAC:EquipmentConnections"]["Connection"]
                        .as_object_mut()
                        .expect("real Connection object")
                        .remove("zone_return_air_node_or_nodelist_name");
                } else {
                    returns["NodeList"]["Return Vector"] = json!({"nodes":names.iter().map(|name|json!({"node_name":name})).collect::<Vec<_>>()});
                    returns["ZoneHVAC:EquipmentConnections"]["Connection"]["zone_return_air_node_or_nodelist_name"] =
                        json!("Return Vector");
                }
                let raw =
                    parse_epjson_str(&returns.to_string()).expect("real ordered return vector raw");
                let retained = raw.clone();
                let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                let model = compiled
                    .model
                    .as_ref()
                    .expect("actual Source return vector typed model");
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("matched exhaust Source vector eligibility");
                assert!(ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_err());
                if let Some(name) = model.zone_equipment_connections[0]
                    .zone_return_air_node_or_nodelist_name
                    .as_ref()
                {
                    let nodes = model
                        .resolve_advisory_node_reference(&name.0)
                        .require_proven_nodes()
                        .expect("authentic ordered Proven return identities");
                    let actual_names = nodes
                        .iter()
                        .map(|id| {
                            model
                                .nodes
                                .iter()
                                .find(|node| node.id == *id)
                                .expect("real arena Node")
                                .name
                                .0
                                .clone()
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(
                        actual_names,
                        names
                            .iter()
                            .map(|name| name.to_ascii_uppercase())
                            .collect::<Vec<_>>()
                    );
                } else {
                    assert_eq!(count, 0);
                }
                let mut actual_scope =
                    inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
                assert!(actual_scope.admissible, "{:?}", actual_scope.violations);
                actual_scope.environment = Some(
                    super::environment::environment_from_weather(
                        model,
                        &single_day_weather_fixture(),
                        "authored Source return-vector admission weather",
                    )
                    .expect("actual return vector environment"),
                );
                let admission = crate::support::SourceSupportAdmission::from_validated_scope(
                    &raw,
                    &actual_scope,
                )
                .expect("actual raw return vector token");
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
                assert_eq!(
                    support.runtime_class,
                    crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
                );
                assert!(support.matched_capability_ids.is_empty());
                let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
                    .expect("actual vector IP custody");
                let fields = ip
                    .source_raw_object_fields("ZoneHVAC:EquipmentConnections", "Connection")
                    .expect("original Connection alpha fields");
                if count == 0 {
                    assert!(!fields.contains_key("zone_return_air_node_or_nodelist_name"));
                } else {
                    assert_eq!(
                        fields["zone_return_air_node_or_nodelist_name"],
                        SourceRaw::String("Return Vector".into())
                    );
                    let fields = ip
                        .source_raw_object_fields("NodeList", "Return Vector")
                        .expect("original raw return members");
                    let members = match &fields["nodes"] {
                        SourceRaw::Array(members) => Some(members),
                        _ => None,
                    }
                    .expect("original nodes array");
                    assert_eq!(members.len(), count);
                    for (member, name) in members.iter().zip(&names) {
                        let fields = match member {
                            SourceRaw::Object(fields) => Some(fields),
                            _ => None,
                        }
                        .expect("original member object");
                        assert_eq!(fields["node_name"], SourceRaw::String(name.clone()));
                    }
                }
                assert_eq!(raw, retained);
                let mut no_exhaust = returns.clone();
                no_exhaust["ZoneHVAC:EquipmentConnections"]["Connection"]
                    .as_object_mut()
                    .expect("Connection")
                    .remove("zone_air_exhaust_node_or_nodelist_name");
                no_exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]
                    .as_object_mut()
                    .expect("Ideal")
                    .remove("zone_exhaust_air_node_name");
                no_exhaust["SetpointManager:Scheduled"]["Supply setpoint"]["setpoint_node_or_nodelist_name"] =
                    json!("Supply");
                no_exhaust["NodeList"]
                    .as_object_mut()
                    .expect("NodeList family")
                    .remove("Manager Targets");
                let no_exhaust_raw =
                    parse_epjson_str(&no_exhaust.to_string()).expect("real no-exhaust raw vector");
                let no_exhaust_compile = ep_compiler::compile_raw_model_for_source(&no_exhaust_raw);
                assert!(
                    !no_exhaust_compile.has_errors(),
                    "{:?}",
                    no_exhaust_compile.report.diagnostics
                );
                let no_exhaust_model = no_exhaust_compile
                    .model
                    .as_ref()
                    .expect("real no-exhaust typed model");
                let no_exhaust_simulation =
                    ep_model::SimulationModel::from_typed(no_exhaust_model.clone());
                assert_eq!(
                    ep_runtime::validate_source_direct_zone_purchased_air_model(
                        &no_exhaust_simulation
                    )
                    .is_ok(),
                    count == 1
                );
                assert_eq!(
                    ep_runtime::bind_direct_zone_purchased_air_model(&no_exhaust_simulation)
                        .is_ok(),
                    count == 1
                );
                assert_eq!(
                    inspect_porting_scope(
                        &no_exhaust_raw,
                        &no_exhaust_compile.report,
                        Some(no_exhaust_model),
                        scope
                    )
                    .admissible,
                    count == 1
                );
                if count == 3 {
                    for position in 0..count {
                        for collision in ["Supply", "Air", "Exhaust"] {
                            let mut invalid = returns.clone();
                            invalid["NodeList"]["Return Vector"]["nodes"][position]["node_name"] =
                                json!(collision);
                            let raw = parse_epjson_str(&invalid.to_string())
                                .expect("real colliding late return member");
                            let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                            assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                            let model = compiled.model.as_ref().expect("compiled colliding roles");
                            assert!(
                                ep_runtime::validate_source_direct_zone_purchased_air_model(
                                    &ep_model::SimulationModel::from_typed(model.clone())
                                )
                                .is_err()
                            );
                            assert!(
                                !inspect_porting_scope(&raw, &compiled.report, Some(model), scope)
                                    .admissible
                            );
                        }
                    }
                    let mut deferred = returns.clone();
                    deferred["NodeList"]["Return Vector"]["nodes"] = json!([
                        {"node_name":"Return"},{"node_name":""},{"node_name":"Return"}
                    ]);
                    let raw = parse_epjson_str(&deferred.to_string())
                        .expect("actual unresolved selected return prefix");
                    let compiled = ep_compiler::compile_raw_model_for_source(&raw);
                    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
                    let model = compiled
                        .model
                        .as_ref()
                        .expect("retained Deferred return list");
                    assert!(matches!(
                        model.resolve_advisory_node_reference("Return Vector"),
                        ep_model::AdvisoryNodeReference::DeferredList { .. }
                    ));
                    assert!(
                        ep_runtime::validate_source_direct_zone_purchased_air_model(
                            &ep_model::SimulationModel::from_typed(model.clone())
                        )
                        .is_err()
                    );
                    assert!(
                        !inspect_porting_scope(&raw, &compiled.report, Some(model), scope)
                            .admissible
                    );
                }
            }

            for (family, field, value) in [
                (
                    "ZoneHVAC:IdealLoadsAirSystem",
                    "zone_exhaust_air_node_name",
                    json!("Different"),
                ),
                (
                    "ZoneHVAC:IdealLoadsAirSystem",
                    "zone_exhaust_air_node_name",
                    json!("Supply"),
                ),
                (
                    "ZoneHVAC:IdealLoadsAirSystem",
                    "zone_exhaust_air_node_name",
                    json!(null),
                ),
                (
                    "ZoneHVAC:IdealLoadsAirSystem",
                    "system_inlet_air_node_name",
                    json!("Plenum"),
                ),
            ] {
                let mut invalid = exhaust.clone();
                invalid[family][if family == "ZoneHVAC:IdealLoadsAirSystem" {
                    "Ideal"
                } else {
                    "Connection"
                }][field] = value;
                let invalid_raw =
                    parse_epjson_str(&invalid.to_string()).expect("changed exhaust fields");
                let invalid_compile = ep_compiler::compile_raw_model_for_source(&invalid_raw);
                if let Some(model) = invalid_compile.model.as_ref() {
                    assert!(
                        ep_runtime::validate_source_direct_zone_purchased_air_model(
                            &ep_model::SimulationModel::from_typed(model.clone())
                        )
                        .is_err()
                    );
                } else {
                    assert!(invalid_compile.has_errors());
                }
            }
            // An admissible trace cannot grant a token to changed raw operands.
            for (field, bad_value) in [
                ("control_variable", "Unknown"),
                ("schedule_name", "Missing"),
            ] {
                let mut bad = people.clone();
                bad["SetpointManager:Scheduled"]["Supply setpoint"][field] = json!(bad_value);
                let bad_raw =
                    parse_epjson_str(&bad.to_string()).expect("changed Scheduled raw input");
                assert!(
                    crate::support::SourceSupportAdmission::from_validated_scope(&bad_raw, &actual)
                        .is_none()
                );
            }
        }
    }
}

#[test]
fn latent_sources_reach_both_scope_moisture_correctors() {
    for scope in [PortingScope::A, PortingScope::B] {
        let mut value = input(scope);
        value["OtherEquipment"] = json!({"Gain":{
            "zone_or_zonelist_or_space_or_spacelist_name":"Zone", "schedule_name":"On",
            "design_level_calculation_method":"EquipmentLevel", "design_level":100,
            "fraction_latent":0.0, "fraction_radiant":0.0, "fraction_lost":0.0,
        }});
        assert!(trace(&value, scope).admissible);
        value["OtherEquipment"]["Gain"]["fraction_latent"] = json!(0.25);
        let actual = trace(&value, scope);
        assert!(actual.admissible, "{:?}", actual.violations);
        assert!(
            !actual
                .violations
                .iter()
                .any(|message| message.contains("latent gains"))
        );
    }
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
    // Same real input/weather path, but only the Source lane can defer actual
    // calendar controls to its connected getters/owners. No preview schedule is
    // supplied to Source, and legacy raw/admission guards stay in force.
    for scope in [PortingScope::A, PortingScope::B] {
        for (start, end) in [
            ("1/1", "1/1"),
            ("2nd Sunday in March", "Last Sunday in October"),
            ("1/1", "5th Monday in February"),
            ("365", "1"),
        ] {
            let mut value = input(scope);
            value["RunPeriodControl:SpecialDays"] = json!({"Holiday":{"start_date":"1/1","duration":1.9,"special_day_type":"CustomDay1"}});
            value["RunPeriodControl:DaylightSavingTime"] =
                json!({" ":{"start_date":start,"end_date":end}});
            let raw = parse_epjson_str(&value.to_string()).unwrap();
            let compiled = compile_raw_model(&raw);
            let model = compiled
                .model
                .as_ref()
                .expect("schema-number duration projects Native int");
            assert_eq!(model.run_period_special_days[0].duration_days, 1);
            if start == "365" {
                let dst = model.run_period_daylight_saving_time.unwrap();
                assert_eq!(
                    dst.start_date,
                    ep_model::CalendarDateRule::IdfDstOrdinalInput { ordinal: 365 }
                );
                assert_eq!(
                    dst.end_date,
                    ep_model::CalendarDateRule::IdfDstOrdinalInput { ordinal: 1 }
                );
                assert!(ep_model::parse_calendar_date_rule(start).is_none());
                assert!(
                    super::environment::environment_from_weather(
                        model,
                        &weather,
                        "legacy ordinal preview"
                    )
                    .unwrap_err()
                    .contains("actual Source retained weekday")
                );
            }
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
            let proof =
                super::environment::SourceCalendarInputAdmission::from_raw(&raw, scope).unwrap();
            let source = super::environment::source_environment_from_weather(
                model,
                &weather,
                "embedded calendar weather",
                &proof,
            )
            .unwrap();
            assert!(validate_source_environment(&source, &proof).is_empty());
            assert!(source.get("daylight_saving").is_none());
            assert!(source.get("schedule_values").is_none());
            assert_eq!(
                source["source_calendar_controls"]["status"],
                "deferred_to_execution"
            );
            let ip = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .unwrap();
            let operands = crate::ordinary_heat_balance_input::resolve(&raw, &ip).unwrap();
            assert_eq!(operands.objects["RunPeriodControl:SpecialDays"].len(), 1);
            assert_eq!(
                operands.objects["RunPeriodControl:DaylightSavingTime"][0].object_name,
                " "
            );
            let mut admitted = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
            assert!(admitted.admissible, "{:?}", admitted.violations);
            admitted.environment = Some(source);
            let admission =
                crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                    .unwrap();
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
            for family in [
                "RunPeriodControl:SpecialDays",
                "RunPeriodControl:DaylightSavingTime",
            ] {
                assert!(
                    support
                        .typed_objects
                        .iter()
                        .any(|row| row.object_type == family)
                );
            }
        }
    }
    for bad in [Value::Null, json!(0), json!(false), json!([]), json!({})] {
        let mut value = input(PortingScope::A);
        value["RunPeriodControl:DaylightSavingTime"] =
            json!({" ":{"start_date":bad,"end_date":"1/1"}});
        let raw = parse_epjson_str(&value.to_string()).unwrap();
        assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
    }
    for unsupported in ["0", "366", "1.9"] {
        let mut value = input(PortingScope::A);
        value["RunPeriodControl:DaylightSavingTime"] =
            json!({" ":{"start_date":unsupported,"end_date":"1/1"}});
        let raw = parse_epjson_str(&value.to_string()).unwrap();
        assert!(compile_raw_model(&raw).has_errors());
    }
    let mut value = input(PortingScope::A);
    value["RunPeriodControl:SpecialDays"] = json!({"Holiday":{"start_date":"365"}});
    let raw = parse_epjson_str(&value.to_string()).unwrap();
    assert!(compile_raw_model(&raw).has_errors()); // Unwritten local weekday is not the IDFDST ctor0.
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
    // The same real opaque input-owner chain now retains name-only IRT.
    // This is admission/custody evidence, not a physical run or seeded result.
    for scope in [PortingScope::A, PortingScope::B] {
        for boundary in ["Ground", "Adiabatic"] {
            let mut value = input(scope);
            value["Material:InfraredTransparent"] =
                json!({"MiXeD IRT":{"unconsumed_metadata":[{"value":true}]}});
            value["Construction"]["IRT CON"] = json!({"outside_layer":"MiXeD IRT"});
            value["BuildingSurface:Detailed"]["Floor"]["construction_name"] = json!("IRT CON");
            value["BuildingSurface:Detailed"]["Floor"]["outside_boundary_condition"] =
                json!(boundary);
            let raw = parse_epjson_str(&value.to_string())
                .expect("name-only IRT with genuine raw metadata");
            let compiled = compile_raw_model(&raw);
            let model = compiled.model.as_ref().expect("one-layer typed IRT");
            let material = model
                .materials
                .iter()
                .find(|row| row.kind() == ep_model::MaterialKind::InfraredTransparent)
                .unwrap();
            assert_eq!(material.roughness(), None);
            assert_eq!(
                model
                    .ctf_report_names
                    .materials
                    .iter()
                    .find(|(id, _)| *id == material.id)
                    .unwrap()
                    .1,
                "MIXED IRT"
            );
            let ip = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .expect("defined Source IRT use");
            assert!(ip.has_object("Material:InfraredTransparent", "MiXeD IRT"));
            let projected = crate::ordinary_material_input::resolve_source(&raw, &ip)
                .expect("real IRT raw operands");
            assert_eq!(projected.infrared_transparent.len(), 1);
            assert_eq!(projected.infrared_transparent[0].object_name, "MiXeD IRT");
            assert!(
                projected.infrared_transparent[0].fields.is_empty(),
                "unknown raw metadata must not become a physical operand"
            );
            let mut admitted = inspect_porting_scope(&raw, &compiled.report, Some(model), scope);
            assert!(admitted.admissible, "{:?}", admitted.violations);
            let environment = super::environment::environment_from_weather(
                model,
                &weather,
                "embedded IRT weather",
            )
            .unwrap();
            assert!(validate_porting_environment(&environment).is_empty());
            admitted.environment = Some(environment);
            let admission =
                crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
                    .expect("all actual Source custody checks");
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
            if scope == PortingScope::A {
                assert!(
                    crate::ordinary_equipment_input::resolve(&raw).is_err(),
                    "legacy IRT remains fenced"
                );
            }
            for report in ["Materials", "Constructions"] {
                let mut rejected = value.clone();
                rejected["Output:Constructions"] = json!({"Report":{"details_type_1":report}});
                let rejected = parse_epjson_str(&rejected.to_string()).unwrap();
                assert!(
                    match scope {
                        PortingScope::A =>
                            crate::ordinary_input_processor::resolve_source_a(&rejected),
                        PortingScope::B =>
                            crate::ordinary_input_processor::resolve_source_b(&rejected),
                    }
                    .is_err(),
                    "reached Native Invalid roughness report must stay explicit"
                );
            }
            let mut outdoors = value.clone();
            outdoors["BuildingSurface:Detailed"]["Floor"]["outside_boundary_condition"] =
                json!("Outdoors");
            let outdoors = parse_epjson_str(&outdoors.to_string()).unwrap();
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&outdoors),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&outdoors),
                }
                .is_err()
            );
            let mut unused = value.clone();
            unused["BuildingSurface:Detailed"]["Floor"]["construction_name"] = json!("Opaque");
            unused["Output:Constructions"] = json!({"Report":{"details_type_1":"Constructions"}});
            let unused = parse_epjson_str(&unused.to_string()).unwrap();
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&unused),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&unused),
                }
                .is_ok(),
                "unused IRT does not enter reportTransferFunction"
            );
        }
    }
    // Actual staged-IDF recovery must reach the same Source IP/material
    // projector; direct epJSON deliberately has no recovered IDF metadata.
    let epjson = r#"{"Zone":{"z":{}},"Material:InfraredTransparent":{"A":{},"Z":{}}}"#;
    let idf = "Zone,z;Material:InfraredTransparent,Z;Material:InfraredTransparent,A;";
    for staged in [false, true] {
        let raw = if staged {
            ep_raw_model::parse_epjson_str_with_idf_order(epjson, idf)
        } else {
            parse_epjson_str(epjson)
        }
        .expect("actual direct/staged IRT input");
        assert_eq!(
            raw.has_idf_declaration_order("Material:InfraredTransparent"),
            staged
        );
        for scope in [PortingScope::A, PortingScope::B] {
            let ip = match scope {
                PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
            }
            .expect("real Source IRT ingress");
            let projected = crate::ordinary_material_input::resolve_source(&raw, &ip)
                .expect("retained genuine IRT IDF order and field count");
            assert_eq!(projected.source_is_epjson, !staged);
            let rows = &projected.infrared_transparent;
            assert_eq!(
                rows.iter()
                    .map(|row| row.object_name.as_str())
                    .collect::<Vec<_>>(),
                if staged { ["Z", "A"] } else { ["A", "Z"] }
            );
            assert!(
                rows.iter()
                    .all(|row| row.idf_max_fields == staged.then_some(1) && row.fields.is_empty())
            );
        }
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
        for (inside, expected_inside) in [
            ("TARP", I::Tarp),
            ("Simple", I::Simple),
            ("CeilingDiffuser", I::CeilingDiffuser),
            ("ASTMC1340", I::AstmC1340),
        ] {
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
                    PortingScope::A => crate::ordinary_input_processor::resolve_source_a(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .expect("actual IP admission");
                assert!(ip.has_object(INSIDE, " ") && ip.has_object(OUTSIDE, ""));
                let projected = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)
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
                // The same enum table now reaches the genuine local fields.
                // Give the globals different values so inherited substitution
                // cannot satisfy the raw/typed/Source admission assertions.
                value[INSIDE] = json!({" ":{"algorithm":if expected_inside == I::Tarp {"Simple"} else {"TARP"}}});
                value[OUTSIDE] = json!({"":{"algorithm":if expected_outside == O::Doe2 {"MoWiTT"} else {"DOE-2"}}});
                value["Zone"]["Zone"]["zone_inside_convection_algorithm"] = json!(inside);
                value["Zone"]["Zone"]["zone_outside_convection_algorithm"] = json!(outside);
                let local_raw =
                    parse_epjson_str(&value.to_string()).expect("actual local enum strings");
                let local_compiled = compile_raw_model(&local_raw);
                let local_model = local_compiled
                    .model
                    .as_ref()
                    .expect("typed authored local selections");
                let local_zone = &local_model.zones[0];
                assert_eq!(
                    local_zone.inside_convection_algorithm,
                    ep_model::ZoneConvectionAlgorithm::Override(expected_inside)
                );
                assert_eq!(
                    local_zone.outside_convection_algorithm,
                    ep_model::ZoneConvectionAlgorithm::Override(expected_outside)
                );
                assert_ne!(
                    local_model.surface_convection_algorithms.inside,
                    Some(expected_inside)
                );
                assert_ne!(
                    local_model.surface_convection_algorithms.outside,
                    Some(expected_outside)
                );
                let local_ip = match scope {
                    PortingScope::A => {
                        crate::ordinary_input_processor::resolve_source_a(&local_raw)
                    }
                    PortingScope::B => {
                        crate::ordinary_input_processor::resolve_source_b(&local_raw)
                    }
                }
                .expect("actual local raw/IP admission");
                let local_projection =
                    crate::ordinary_heat_balance_input::resolve_source(&local_raw, &local_ip)
                        .expect("same Source GetZoneData raw alpha operands");
                let actual_zone = &local_projection.objects["Zone"][0];
                for (field, expected) in [
                    ("zone_inside_convection_algorithm", inside),
                    ("zone_outside_convection_algorithm", outside),
                ] {
                    assert_eq!(
                        actual_zone.fields[field],
                        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                            expected.into()
                        )
                    );
                }
                let mut local_trace = inspect_porting_scope(
                    &local_raw,
                    &local_compiled.report,
                    Some(local_model),
                    scope,
                );
                assert!(local_trace.admissible, "{:?}", local_trace.violations);
                assert!(
                    crate::support::SourceSupportAdmission::from_validated_scope(
                        &local_raw,
                        &local_trace
                    )
                    .is_none()
                );
                local_trace.environment = Some(
                    super::environment::environment_from_weather(
                        local_model,
                        &weather,
                        "embedded Zone-local convection weather",
                    )
                    .expect("same real EPW/calendar preparation"),
                );
                assert!(
                    validate_porting_environment(
                        local_trace.environment.as_ref().expect("environment")
                    )
                    .is_empty()
                );
                let local_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                    &local_raw,
                    &local_trace,
                )
                .expect("bounded actual Source local-alpha admission");
                let support = crate::support::assess_support_with_source_admission(
                    &local_raw,
                    &local_compiled.report,
                    Some(local_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&local_admission),
                );
                assert!(support.allows_rust_runtime(), "{:?}", support.diagnostics);
                assert!(
                    local_model.zones[0]
                        .inside_convection_algorithm
                        .is_override()
                        && local_model.zones[0]
                            .outside_convection_algorithm
                            .is_override()
                );
                let legacy = crate::support::assess_support(
                    &local_raw,
                    &local_compiled.report,
                    Some(local_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                );
                assert!(
                    legacy
                        .diagnostics
                        .diagnostics
                        .iter()
                        .any(|d| d.code == "UnsupportedZoneConvectionOverride"),
                    "Source local-alpha fact must never exempt Legacy"
                );
            }
        }
        for (field, unsupported) in [
            ("zone_inside_convection_algorithm", "TrombeWall"),
            (
                "zone_inside_convection_algorithm",
                "AdaptiveConvectionAlgorithm",
            ),
            (
                "zone_outside_convection_algorithm",
                "AdaptiveConvectionAlgorithm",
            ),
        ] {
            let mut value = input(scope);
            value["Zone"]["Zone"][field] = json!(unsupported);
            let raw =
                parse_epjson_str(&value.to_string()).expect("official unsupported local enum");
            assert!(
                match scope {
                    PortingScope::A => crate::ordinary_input_processor::resolve(&raw),
                    PortingScope::B => crate::ordinary_input_processor::resolve_source_b(&raw),
                }
                .is_err()
            );
            assert!(!trace(&value, scope).admissible);
        }
        for family in [INSIDE, OUTSIDE] {
            for contents in [json!({}), json!({"":{}}), json!({" ":{"algorithm":""}})] {
                let mut value = input(scope);
                value[family] = contents;
                value["Zone"]["Zone"]["zone_inside_convection_algorithm"] = json!("");
                value["Zone"]["Zone"]["zone_outside_convection_algorithm"] = json!("");
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
                let local_field = if family == INSIDE {
                    "zone_inside_convection_algorithm"
                } else {
                    "zone_outside_convection_algorithm"
                };
                let mut local = input(scope);
                local["Zone"]["Zone"][local_field] = invalid.clone();
                let local_raw =
                    parse_epjson_str(&local.to_string()).expect("invalid local enum stays raw");
                assert!(
                    match scope {
                        PortingScope::A => crate::ordinary_input_processor::resolve(&local_raw),
                        PortingScope::B =>
                            crate::ordinary_input_processor::resolve_source_b(&local_raw),
                    }
                    .is_err(),
                    "local alpha must have actual schema string/enum shape"
                );
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
