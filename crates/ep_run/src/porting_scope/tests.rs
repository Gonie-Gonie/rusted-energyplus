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
                    } else if method == "Sum" && limit == "NoLimit" && flow > 0.0 {
                        unit["demand_controlled_ventilation_type"] = json!("CO2Setpoint");
                    }
                    if method == "AirChanges/Hour" && limit == "NoLimit" && flow > 0.0 {
                        unit["demand_controlled_ventilation_type"] = json!("");
                    } else if method == "Flow/Area" && limit == "LimitFlowRate" && flow > 0.0 {
                        unit["demand_controlled_ventilation_type"] = json!("");
                        unit["outdoor_air_economizer_type"] = json!("");
                    }
                    if method == "Flow/Person" && limit == "LimitFlowRate" && flow > 0.0 {
                        unit["outdoor_air_economizer_type"] = json!("DifferentialDryBulb");
                    } else if method == "Sum" && limit == "LimitFlowRateAndCapacity" && flow > 0.0 {
                        unit["outdoor_air_economizer_type"] = json!("DifferentialEnthalpy");
                    }
                    if method == "Flow/Zone" && limit == "NoLimit" && flow > 0.0 {
                        unit["heat_recovery_type"] = json!("Sensible");
                        unit["sensible_heat_recovery_effectiveness"] = json!(0.0);
                        unit["latent_heat_recovery_effectiveness"] = json!(1.0);
                    } else if method == "Maximum" && limit == "NoLimit" && flow > 0.0 {
                        unit["heat_recovery_type"] = json!("Enthalpy");
                        unit["sensible_heat_recovery_effectiveness"] = json!(1.0);
                        unit["latent_heat_recovery_effectiveness"] = json!(0.0);
                    } else if method == "Flow/Area" && limit == "NoLimit" {
                        if flow == 0.0 {
                            unit["heat_recovery_type"] = json!("");
                            unit["sensible_heat_recovery_effectiveness"] = json!(-0.0);
                        } else {
                            unit.as_object_mut()
                                .expect("actual OA unit")
                                .remove("heat_recovery_type");
                        }
                    }
                    // Reuse genuine existing rows: no extra context or loop.
                    if flow > 0.0 {
                        if method == "Flow/Zone" && limit == "LimitFlowRate" {
                            unit["dehumidification_control_type"] =
                                json!("ConstantSupplyHumidityRatio");
                            unit["humidification_control_type"] =
                                json!("ConstantSupplyHumidityRatio");
                            unit["maximum_heating_supply_air_humidity_ratio"] = json!(1.1);
                            unit["minimum_cooling_supply_air_humidity_ratio"] = json!(1.2);
                        } else if method == "Flow/Zone" && limit == "LimitCapacity" {
                            unit["dehumidification_control_type"] =
                                json!("ConstantSupplyHumidityRatio");
                        } else if method == "Maximum" && limit == "LimitCapacity" {
                            unit["dehumidification_control_type"] =
                                json!("ConstantSensibleHeatRatio");
                        } else if method == "Maximum" && limit == "LimitFlowRateAndCapacity" {
                            unit["dehumidification_control_type"] =
                                json!("ConstantSensibleHeatRatio");
                            unit["humidification_control_type"] =
                                json!("ConstantSupplyHumidityRatio");
                        } else if method == "Flow/Area" && limit == "LimitFlowRateAndCapacity" {
                            unit["humidification_control_type"] =
                                json!("ConstantSupplyHumidityRatio");
                        } else if method == "Flow/Area" && limit == "LimitCapacity" {
                            unit["humidification_control_type"] = json!("");
                        } else if method == "AirChanges/Hour" && limit == "LimitCapacity" {
                            unit.as_object_mut()
                                .expect("unit")
                                .remove("dehumidification_control_type");
                        } else if method == "AirChanges/Hour" && limit == "LimitFlowRate" {
                            unit.as_object_mut()
                                .expect("unit")
                                .remove("humidification_control_type");
                        } else if method == "Sum" && limit == "LimitCapacity" {
                            unit["dehumidification_control_type"] = json!("");
                            unit["humidification_control_type"] = json!("");
                        }
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
                    if flow > 0.0 && method == "Flow/Person" && limit == "LimitCapacity" {
                        hard["OutdoorAir:Node"] = json!({"Outside 각": {"drybulb_temperature_schedule_name":"On", "wind_pressure_coefficient_curve_name":"", "symmetric_wind_pressure_coefficient_curve":"", "wind_angle_type":""}});
                    } else if flow > 0.0 && method == "Sum" && limit == "LimitFlowRate" {
                        hard["OutdoorAir:Node"] =
                            json!({"Outside 각": {"height_above_ground": -1.25}});
                    } else if flow > 0.0 && method == "Flow/Person" && limit == "LimitFlowRate" {
                        hard["OutdoorAir:Node"] = json!({"Outside 각": {"height_above_ground": 0.0, "wind_speed_schedule_name":"On", "wind_direction_schedule_name":"", "wetbulb_temperature_schedule_name":"On"}});
                    } else if flow > 0.0 && method == "Flow/Zone" && limit == "LimitCapacity" {
                        hard["OutdoorAir:Node"] = json!({"Outside 각": {"height_above_ground": -0.0, "wind_speed_schedule_name":"", "wind_direction_schedule_name":"", "wetbulb_temperature_schedule_name":"", "drybulb_temperature_schedule_name":"", "wind_pressure_coefficient_curve_name":"  ", "symmetric_wind_pressure_coefficient_curve":"No", "wind_angle_type":"Absolute"}});
                    } else if flow > 0.0 && method == "Flow/Area" && limit == "LimitCapacity" {
                        hard["OutdoorAir:Node"] = json!({"Outside 각": {"height_above_ground": 10.0, "wind_direction_schedule_name":"OA wind compact", "wetbulb_temperature_schedule_name":"OA wind compact", "drybulb_temperature_schedule_name":"On", "wind_pressure_coefficient_curve_name":"Unused no-AFN curve 각", "symmetric_wind_pressure_coefficient_curve":"Yes", "wind_angle_type":"Relative"}});
                    }
                    if flow > 0.0 && method == "Flow/Area" && limit == "LimitCapacity" {
                        // One necessary schedule extension of this same admission cell.
                        hard["Schedule:Compact"] = json!({"OA wind compact":{"data":[
                            {"field":"Through: 12/31"},{"field":"For: AllDays"},
                            {"field":"Until: 24:00"},{"field":1.0}
                        ]}});
                        // Same one positive cell, all known OA operands held.
                        // Unknown JSON members do not become IDF fields or A1.
                        hard["OutdoorAir:Node"]["Outside 각"]["metadata_extra"] =
                            json!({"retained":[null,true,1.25,"value"]});
                        hard["OutdoorAir:Node"]["Outside 각"]["extra_number"] = json!(-1.25);
                        hard["OutdoorAir:Node"]["Outside 각"]["idf_order"] =
                            json!("ignored direct JSON metadata");
                        hard["OutdoorAir:Node"]["Outside 각"]["idf_max_fields"] =
                            json!([null, "ignored"]);
                        hard["OutdoorAir:Node"]["Outside 각"]["idf_max_extensible_fields"] =
                            json!(null);
                        hard["OutdoorAir:Node"]["Outside 각"]["name"] = json!(false);
                    }
                    // Same existing cells: genuine mixed topology and list-only
                    // admission, without another parse/context or a Node-state seed.
                    if flow > 0.0 && method == "Sum" && limit == "LimitFlowRate" {
                        hard["OutdoorAir:Node"]["Additional authored OA"] =
                            json!({"wind_speed_schedule_name":"Constant-1.0"});
                        hard["NodeList"]["OA expansion"] =
                            json!({"nodes":[{"node_name":"List A"},{"node_name":"List B"}]});
                        hard["OutdoorAir:NodeList"] = json!({
                            "":{}, "\t ":{"nodes":[]},
                            "Mixed":{"nodes":[{"node_or_nodelist_name":""},{"node_or_nodelist_name":"OA expansion"},{"node_or_nodelist_name":"Direct OA"},{"node_or_nodelist_name":"OA expansion"},{"node_or_nodelist_name":" "}],
                                "idf_order":"ignored direct JSON metadata", "idf_max_extensible_fields":0}
                        });
                    } else if flow > 0.0 && method == "Maximum" && limit == "NoLimit" {
                        hard["OutdoorAir:NodeList"] = json!({
                            "":{}, "\t ":{"nodes":[]},
                            "Mixed":{"nodes":[{"node_or_nodelist_name":""},{"node_or_nodelist_name":"List-only extra"},{"node_or_nodelist_name":"Outside 각"},{"node_or_nodelist_name":"Outside 각"},{"node_or_nodelist_name":" "}],
                                "idf_order":"ignored direct JSON metadata", "idf_max_extensible_fields":0}
                        });
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
                    let mut hard_ip = crate::ordinary_input_processor::resolve_source_b(&hard_raw)
                        .expect("real hard-limit IP");
                    let mut statistics = ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                    hard_ip
                        .report_source_idf_records_stats(&mut statistics)
                        .expect("actual hard-limit statistics owner");
                    if hard.get("OutdoorAir:NodeList").is_some() {
                        assert!(
                            support
                                .source_consumed_objects
                                .iter()
                                .any(|row| row.object_type == "OutdoorAir:NodeList"
                                    && row.count == 3)
                        );
                        assert_eq!(
                            hard_ip
                                .get_object_def_max_args("OutdoorAir:NodeList")
                                .expect("real nameless extensible shape")
                                .alphas,
                            5
                        );
                        assert!(
                            hard_ip
                                .source_raw_object_fields("OutdoorAir:NodeList", "")
                                .expect("empty nameless IP identity")
                                .is_empty()
                        );
                        assert_eq!(
                            hard_ip
                                .source_raw_object_fields("OutdoorAir:NodeList", "\t ")
                                .expect("whitespace nameless IP identity")
                                .get("nodes"),
                            Some(&OaRaw::Array(Vec::new()))
                        );
                        let mixed = hard_ip
                            .source_raw_object_fields("OutdoorAir:NodeList", "Mixed")
                            .expect("real retained mixed list");
                        assert_eq!(
                            mixed.get("idf_order"),
                            Some(&OaRaw::String("ignored direct JSON metadata".into()))
                        );
                        assert_eq!(
                            mixed.get("idf_max_extensible_fields"),
                            Some(&OaRaw::Number("0".into()))
                        );
                        assert!(
                            matches!(mixed.get("nodes"), Some(OaRaw::Array(members)) if members.len() == 5)
                        );
                    }
                    if hard.get("OutdoorAir:Node").is_some() {
                        assert!(support.source_consumed_objects.iter().any(|row| {
                            row.object_type == "OutdoorAir:Node"
                                && row.count
                                    == hard["OutdoorAir:Node"]
                                        .as_object()
                                        .expect("actual authored singles")
                                        .len()
                        }));
                        let oa = hard_ip
                            .source_raw_object_fields("OutdoorAir:Node", "Outside 각")
                            .expect("actual authored OA raw custody");
                        if method == "Flow/Person" && limit == "LimitFlowRate" {
                            assert_eq!(
                                oa.get("wind_speed_schedule_name"),
                                Some(&OaRaw::String("On".into()))
                            );
                            assert_eq!(
                                oa.get("wind_direction_schedule_name"),
                                Some(&OaRaw::String(String::new()))
                            );
                        } else if method == "Flow/Zone" {
                            assert_eq!(
                                oa.get("wind_speed_schedule_name"),
                                Some(&OaRaw::String(String::new()))
                            );
                            assert_eq!(
                                oa.get("wind_direction_schedule_name"),
                                Some(&OaRaw::String(String::new()))
                            );
                        } else if method == "Flow/Area" {
                            assert_eq!(
                                oa.get("drybulb_temperature_schedule_name"),
                                Some(&OaRaw::String("On".into()))
                            );
                            assert!(!oa.contains_key("wind_speed_schedule_name"));
                            assert_eq!(
                                oa.get("wind_direction_schedule_name"),
                                Some(&OaRaw::String("OA wind compact".into()))
                            );
                            assert!(
                                hard_ip
                                    .source_raw_object_fields("Schedule:Compact", "OA wind compact")
                                    .is_ok()
                            );
                            assert_eq!(
                                oa.get("idf_order"),
                                Some(&OaRaw::String("ignored direct JSON metadata".into()))
                            );
                            assert_eq!(
                                oa.get("idf_max_fields"),
                                Some(&OaRaw::Array(vec![
                                    OaRaw::Null,
                                    OaRaw::String("ignored".into())
                                ]))
                            );
                            assert_eq!(oa.get("idf_max_extensible_fields"), Some(&OaRaw::Null));
                            assert_eq!(oa.get("name"), Some(&OaRaw::Bool(false)));
                            assert_eq!(
                                oa.get("extra_number"),
                                Some(&OaRaw::Number("-1.25".into()))
                            );
                            assert!(
                                matches!(oa.get("metadata_extra"), Some(OaRaw::Object(fields))
                                if fields.get("retained") == Some(&OaRaw::Array(vec![
                                    OaRaw::Null, OaRaw::Bool(true), OaRaw::Number("1.25".into()),
                                    OaRaw::String("value".into()),
                                ])))
                            );
                        }
                        if method == "Flow/Person" && limit == "LimitFlowRate" {
                            assert_eq!(
                                oa.get("wetbulb_temperature_schedule_name"),
                                Some(&OaRaw::String("On".into()))
                            );
                        } else if method == "Flow/Zone" {
                            assert_eq!(
                                oa.get("wetbulb_temperature_schedule_name"),
                                Some(&OaRaw::String(String::new()))
                            );
                        } else if method == "Flow/Area" {
                            assert_eq!(
                                oa.get("wetbulb_temperature_schedule_name"),
                                Some(&OaRaw::String("OA wind compact".into()))
                            );
                        }
                        if method == "Flow/Person" && limit == "LimitCapacity" {
                            assert_eq!(
                                oa.get("drybulb_temperature_schedule_name"),
                                Some(&OaRaw::String("On".into()))
                            );
                            assert!(!oa.contains_key("wetbulb_temperature_schedule_name"));
                        } else if method == "Flow/Zone" {
                            assert_eq!(
                                oa.get("drybulb_temperature_schedule_name"),
                                Some(&OaRaw::String(String::new()))
                            );
                        }
                        let (curve_name, symmetric, angle) =
                            if method == "Flow/Person" && limit == "LimitCapacity" {
                                ("", "", "")
                            } else if method == "Flow/Zone" {
                                ("  ", "No", "Absolute")
                            } else if method == "Flow/Area" {
                                ("Unused no-AFN curve 각", "Yes", "Relative")
                            } else {
                                ("", "", "")
                            };
                        if method == "Flow/Area"
                            || method == "Flow/Zone"
                            || (method == "Flow/Person" && limit == "LimitCapacity")
                        {
                            assert_eq!(
                                oa.get("wind_pressure_coefficient_curve_name"),
                                Some(&OaRaw::String(curve_name.into()))
                            );
                            assert_eq!(
                                oa.get("symmetric_wind_pressure_coefficient_curve"),
                                Some(&OaRaw::String(symmetric.into()))
                            );
                            assert_eq!(
                                oa.get("wind_angle_type"),
                                Some(&OaRaw::String(angle.into()))
                            );
                        }
                        let height_token = if method == "Sum" {
                            Some("-1.25")
                        } else if method == "Flow/Person" && limit == "LimitCapacity" {
                            None
                        } else if method == "Flow/Person" {
                            Some("0.0")
                        } else if method == "Flow/Zone" {
                            Some("-0.0")
                        } else {
                            Some("10.0")
                        };
                        if let Some(token) = height_token {
                            assert_eq!(
                                oa.get("height_above_ground"),
                                Some(&OaRaw::Number(token.into()))
                            );
                        } else {
                            assert!(!oa.contains_key("height_above_ground"));
                        }
                    }
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
                    } else if method == "Sum" && limit == "NoLimit" && flow > 0.0 {
                        assert_eq!(
                            actual["demand_controlled_ventilation_type"],
                            OaRaw::String("CO2Setpoint".into())
                        );
                    }
                    if (method == "AirChanges/Hour" && limit == "NoLimit"
                        || method == "Flow/Area" && limit == "LimitFlowRate")
                        && flow > 0.0
                    {
                        assert_eq!(
                            actual["demand_controlled_ventilation_type"],
                            OaRaw::String(String::new())
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0].demand_controlled_ventilation_type,
                            ep_model::DemandControlledVentilationType::None
                        );
                    }
                    if method == "Flow/Area" && limit == "LimitFlowRate" && flow > 0.0 {
                        assert_eq!(
                            actual["outdoor_air_economizer_type"],
                            OaRaw::String(String::new())
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0].outdoor_air_economizer_type,
                            ep_model::OutdoorAirEconomizerType::NoEconomizer
                        );
                    }
                    if method == "Flow/Person" && limit == "LimitFlowRate" && flow > 0.0 {
                        assert_eq!(
                            actual["outdoor_air_economizer_type"],
                            OaRaw::String("DifferentialDryBulb".into())
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0].outdoor_air_economizer_type,
                            ep_model::OutdoorAirEconomizerType::DifferentialDryBulb
                        );
                    } else if method == "Sum" && limit == "LimitFlowRateAndCapacity" && flow > 0.0 {
                        assert_eq!(
                            actual["outdoor_air_economizer_type"],
                            OaRaw::String("DifferentialEnthalpy".into())
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0].outdoor_air_economizer_type,
                            ep_model::OutdoorAirEconomizerType::DifferentialEnthalpy
                        );
                    }
                    if method == "Flow/Zone" && limit == "NoLimit" && flow > 0.0 {
                        assert_eq!(
                            actual["heat_recovery_type"],
                            OaRaw::String("Sensible".into())
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0].heat_recovery_type,
                            ep_model::HeatRecoveryType::Sensible
                        );
                    } else if method == "Maximum" && limit == "NoLimit" && flow > 0.0 {
                        assert_eq!(
                            actual["heat_recovery_type"],
                            OaRaw::String("Enthalpy".into())
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0].heat_recovery_type,
                            ep_model::HeatRecoveryType::Enthalpy
                        );
                    } else if method == "Flow/Area" && limit == "NoLimit" {
                        assert_eq!(
                            model.ideal_loads_air_systems[0].heat_recovery_type,
                            ep_model::HeatRecoveryType::None
                        );
                        if flow == 0.0 {
                            assert_eq!(actual["heat_recovery_type"], OaRaw::String(String::new()));
                            assert_eq!(
                                actual["sensible_heat_recovery_effectiveness"],
                                OaRaw::Number("-0.0".into())
                            );
                            assert_eq!(
                                model.ideal_loads_air_systems[0]
                                    .sensible_heat_recovery_effectiveness
                                    .to_bits(),
                                (-0.0_f64).to_bits()
                            );
                        } else {
                            assert!(!actual.contains_key("heat_recovery_type"));
                        }
                    }
                    let authored_unit = &hard["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"];
                    match authored_unit.get("dehumidification_control_type") {
                        Some(value) => {
                            let name = value.as_str().expect("authored humidity selector");
                            assert_eq!(
                                actual["dehumidification_control_type"],
                                OaRaw::String(name.into())
                            );
                            let expected = match name {
                                "None" => ep_model::DehumidificationControlType::None,
                                "ConstantSupplyHumidityRatio" => ep_model::DehumidificationControlType::ConstantSupplyHumidityRatio,
                                // Advisory typed default remains SHR; the actual getter owns Invalid.
                                "" | "ConstantSensibleHeatRatio" => ep_model::DehumidificationControlType::ConstantSensibleHeatRatio,
                                _ => unreachable!(),
                            };
                            assert_eq!(
                                model.ideal_loads_air_systems[0].dehumidification_control_type,
                                expected
                            );
                        }
                        None => {
                            assert!(!actual.contains_key("dehumidification_control_type"));
                            assert_eq!(
                                model.ideal_loads_air_systems[0].dehumidification_control_type,
                                ep_model::DehumidificationControlType::ConstantSensibleHeatRatio
                            );
                        }
                    }
                    match authored_unit.get("humidification_control_type") {
                        Some(value) => {
                            let name = value.as_str().expect("authored humidity selector");
                            assert_eq!(
                                actual["humidification_control_type"],
                                OaRaw::String(name.into())
                            );
                            let expected = match name {
                                // Advisory typed default remains None; the actual getter owns Invalid.
                                "" | "None" => ep_model::HumidificationControlType::None,
                                "ConstantSupplyHumidityRatio" => {
                                    ep_model::HumidificationControlType::ConstantSupplyHumidityRatio
                                }
                                _ => unreachable!(),
                            };
                            assert_eq!(
                                model.ideal_loads_air_systems[0].humidification_control_type,
                                expected
                            );
                        }
                        None => {
                            assert!(!actual.contains_key("humidification_control_type"));
                            assert_eq!(
                                model.ideal_loads_air_systems[0].humidification_control_type,
                                ep_model::HumidificationControlType::None
                            );
                        }
                    }
                    if method == "Flow/Zone" && limit == "LimitFlowRate" && flow > 0.0 {
                        assert_eq!(
                            actual["maximum_heating_supply_air_humidity_ratio"],
                            OaRaw::Number("1.1".into())
                        );
                        assert_eq!(
                            actual["minimum_cooling_supply_air_humidity_ratio"],
                            OaRaw::Number("1.2".into())
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0]
                                .maximum_heating_supply_air_humidity_ratio
                                .to_bits(),
                            1.1_f64.to_bits()
                        );
                        assert_eq!(
                            model.ideal_loads_air_systems[0]
                                .minimum_cooling_supply_air_humidity_ratio
                                .to_bits(),
                            1.2_f64.to_bits()
                        );
                        let legacy = ep_compiler::compile_raw_model(&hard_raw);
                        assert!(legacy.report.diagnostics.iter().any(|row| row.code
                            == "InvalidNumericRange"
                            && row.object_type == "ZoneHVAC:IdealLoadsAirSystem"
                            && row.field.as_deref()
                                == Some("maximum_heating_supply_air_humidity_ratio")));
                        assert!(legacy.report.diagnostics.iter().any(|row| row.code
                            == "InvalidNumericRange"
                            && row.object_type == "ZoneHVAC:IdealLoadsAirSystem"
                            && row.field.as_deref()
                                == Some("minimum_cooling_supply_air_humidity_ratio")));
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
                            // Reuse the two NoOA high-W negatives to retain
                            // Humidistat ownership; other size negatives stay literal.
                            let no_oa_high_w = method == "Flow/Zone"
                                && limit == "LimitFlowRate"
                                && flow > 0.0
                                && matches!(
                                    missing,
                                    "maximum_heating_air_flow_rate"
                                        | "maximum_cooling_air_flow_rate"
                                )
                                && bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][missing].is_null();
                            if no_oa_high_w {
                                bad.as_object_mut()
                                    .expect("root")
                                    .remove("DesignSpecification:OutdoorAir");
                                let unit = &mut bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"];
                                unit.as_object_mut()
                                    .expect("unit")
                                    .remove("design_specification_outdoor_air_object_name");
                                unit.as_object_mut()
                                    .expect("unit")
                                    .remove("outdoor_air_inlet_node_name");
                                unit[missing] = json!(flow);
                                unit["maximum_heating_supply_air_humidity_ratio"] = json!(0.0156);
                                unit["minimum_cooling_supply_air_humidity_ratio"] = json!(0.0077);
                                if missing == "maximum_heating_air_flow_rate" {
                                    unit["maximum_heating_supply_air_humidity_ratio"] = json!(1.1);
                                    unit["humidification_control_type"] = json!("Humidistat");
                                } else {
                                    unit["minimum_cooling_supply_air_humidity_ratio"] = json!(1.2);
                                    unit["dehumidification_control_type"] = json!("Humidistat");
                                }
                            }
                            let bad_raw = parse_epjson_str(&bad.to_string())
                                .expect("actual invalid active numeric");
                            assert!(
                                crate::ordinary_equipment_input::resolve_source_b(&bad_raw)
                                    .is_err()
                            );
                            let bad_compiled = ep_compiler::compile_raw_model_for_source(&bad_raw);
                            if no_oa_high_w {
                                assert!(
                                    !bad_compiled.has_errors(),
                                    "{:?}",
                                    bad_compiled.report.diagnostics
                                );
                                let model = bad_compiled
                                    .model
                                    .as_ref()
                                    .expect("actual NoOA high-W advisory model");
                                assert!(
                                    ep_runtime::validate_source_direct_zone_purchased_air_model(
                                        &ep_model::SimulationModel::from_typed(model.clone())
                                    )
                                    .is_err()
                                );
                            }
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
                        // Named CO2 still cannot enable contaminant simulation.
                        // Exact empty is owned; whitespace/type/unknown are not.
                        if method == "Flow/Person" {
                            bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["demand_controlled_ventilation_type"] =
                                json!("CO2Setpoint");
                            bad["ZoneAirContaminantBalance"] = json!({
                                "Uncomposed CO2": {"carbon_dioxide_concentration": "Yes"}
                            });
                        } else {
                            bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["demand_controlled_ventilation_type"] =
                                match method {
                                    "Flow/Zone" => json!(" "),
                                    "Flow/Area" => json!(null),
                                    "AirChanges/Hour" => json!(0),
                                    "Sum" => json!("UNKNOWN"),
                                    "Maximum" => json!(" invalid "),
                                    _ => unreachable!(),
                                };
                        }
                    }
                    1 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_economizer_type"] =
                            json!("");
                        if method == "Sum" {
                            bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["cooling_limit"] =
                                json!("LimitCapacity");
                            bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_total_cooling_capacity"] =
                                json!(1.0);
                        }
                    }
                    2 => {
                        let unit = &mut bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"];
                        unit["heat_recovery_type"] = json!("Sensible");
                        match method {
                            "Flow/Person" => {
                                unit["sensible_heat_recovery_effectiveness"] = json!(-0.1)
                            }
                            "Flow/Zone" => {
                                unit["sensible_heat_recovery_effectiveness"] = json!(1.1)
                            }
                            "Flow/Area" => unit["latent_heat_recovery_effectiveness"] = json!(-0.1),
                            "AirChanges/Hour" => {
                                unit["latent_heat_recovery_effectiveness"] = json!(1.1)
                            }
                            "Sum" => unit["heat_recovery_type"] = json!("UNKNOWN"),
                            "Maximum" => unit["sensible_heat_recovery_effectiveness"] = json!(null),
                            _ => unreachable!(),
                        }
                    }
                    3 => {
                        let unit = &mut bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"];
                        unit["dehumidification_control_type"] = json!("ConstantSensibleHeatRatio");
                        match method {
                            "Flow/Person" => {
                                unit["dehumidification_control_type"] = json!("Humidistat")
                            }
                            "Flow/Zone" => unit["cooling_sensible_heat_ratio"] = json!(0.0),
                            "Flow/Area" => unit["cooling_sensible_heat_ratio"] = json!(-0.0),
                            "AirChanges/Hour" => unit["cooling_sensible_heat_ratio"] = json!(1.1),
                            "Sum" => unit["cooling_sensible_heat_ratio"] = json!("0.7"),
                            "Maximum" => unit["cooling_sensible_heat_ratio"] = json!(null),
                            _ => unreachable!(),
                        }
                    }
                    4 => {
                        let unit = &mut bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"];
                        match method {
                            "Flow/Person" => unit["humidification_control_type"] = json!(" "),
                            "Flow/Zone" => {
                                unit["maximum_heating_supply_air_humidity_ratio"] = json!(0.0)
                            }
                            "Flow/Area" => {
                                unit["maximum_heating_supply_air_humidity_ratio"] = json!(-0.0)
                            }
                            "AirChanges/Hour" => {
                                unit["minimum_cooling_supply_air_humidity_ratio"] = json!(0.0)
                            }
                            "Sum" => {
                                unit["minimum_cooling_supply_air_humidity_ratio"] = json!(-0.0)
                            }
                            "Maximum" => unit["humidification_control_type"] = json!("Humidistat"),
                            _ => unreachable!(),
                        }
                    }
                    5 => {
                        let unit = &mut bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"];
                        if method == "Flow/Person" {
                            unit["outdoor_air_economizer_type"] = json!("DifferentialDryBulb");
                            unit["cooling_limit"] = json!("NoLimit");
                        } else if method == "Sum" {
                            unit["outdoor_air_economizer_type"] = json!("DifferentialEnthalpy");
                            unit["cooling_limit"] = json!("LimitCapacity");
                            unit["maximum_total_cooling_capacity"] = json!(1.0);
                        } else {
                            unit["cooling_limit"] = json!("");
                        }
                    }
                    6 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heating_limit"] =
                            json!("LimitCapacity");
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_sensible_heating_capacity"] =
                            json!("Autosize");
                    }
                    7 => {
                        bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["design_specification_outdoor_air_object_name"] =
                            json!("Missing");
                        if method == "Sum" {
                            // Both refs are nonblank, but the same deep Raw prerequisite must fail.
                            bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["dehumidification_control_type"] =
                                json!("");
                            bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["humidification_control_type"] =
                                json!("");
                        }
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
                    16 => {
                        bad["OutdoorAir:Node"] = match method {
                            "Flow/Person" => json!({"Outside 각":{"height_above_ground":"0"}}),
                            "Flow/Zone" => json!({"Outside 각":{"height_above_ground":null}}),
                            "Flow/Area" => {
                                json!({"Outside 각":{"drybulb_temperature_schedule_name":"On","wetbulb_temperature_schedule_name":"Missing"}})
                            }
                            "AirChanges/Hour" => {
                                json!({"Outside 각":{"symmetric_wind_pressure_coefficient_curve":"no"}})
                            }
                            "Sum" => json!({"Outside 각":{}}),
                            "Maximum" => json!({"Mismatched OA":{"height_above_ground":null}}),
                            _ => unreachable!(),
                        };
                        if method == "Sum" {
                            bad.as_object_mut()
                                .expect("root")
                                .remove("DesignSpecification:OutdoorAir");
                            let unit = bad["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]
                                .as_object_mut()
                                .expect("unit");
                            unit.remove("design_specification_outdoor_air_object_name");
                            unit.remove("outdoor_air_inlet_node_name");
                        }
                    }
                    17 => {
                        bad["OutdoorAir:NodeList"] = match method {
                            "Flow/Person" => json!({"Authored OA":{"nodes":null}}),
                            "Flow/Zone" => json!({"Authored OA":{"nodes":"Outside 각"}}),
                            "Flow/Area" => json!({"Authored OA":{"nodes":[{}]}}),
                            "AirChanges/Hour" => {
                                json!({"Authored OA":{"nodes":[{"node_or_nodelist_name":null}]}})
                            }
                            "Sum" => json!({"Authored OA":{"nodes":[{"node_or_nodelist_name":1}]}}),
                            "Maximum" => json!({"Authored OA":{"nodes":["Outside 각"]}}),
                            _ => unreachable!(),
                        }
                    }
                    18 => {
                        bad["ZoneControl:Humidistat"] = json!({"Uncomposed control":{"zone_name":"Zone","humidifying_relative_humidity_setpoint_schedule_name":"On"}})
                    }
                    19 => {
                        if method == "Flow/Person" {
                            bad["OutdoorAir:Node"] =
                                json!({"Outside 각":{"wind_speed_schedule_name":"Missing"}});
                        } else if method == "Flow/Zone" {
                            bad["OutdoorAir:Node"] = json!({"Outside 각":{"wetbulb_temperature_schedule_name":"Missing"}});
                        } else if method == "Flow/Area" {
                            bad["OutdoorAir:Node"] =
                                json!({"Outside 각":{"wetbulb_temperature_schedule_name":null}});
                        } else if method == "AirChanges/Hour" {
                            bad["OutdoorAir:Node"] =
                                json!({"Outside 각":{"wetbulb_temperature_schedule_name":1.0}});
                        } else if method == "Sum" {
                            bad["OutdoorAir:Node"] = json!({"Outside 각":{"wetbulb_temperature_schedule_name":" Constant-1.0 "}});
                        } else {
                            bad["DesignSpecification:OutdoorAir"]["Actual OA"]["outdoor_air_schedule_name"] =
                                json!("Missing");
                        }
                    }
                    20 => {
                        if method == "Flow/Person" {
                            bad["OutdoorAir:Node"] = json!({"Outside 각":{"drybulb_temperature_schedule_name":"Missing"}});
                        } else if method == "Sum" {
                            bad["OutdoorAir:Node"] =
                                json!({"Outside 각":{"drybulb_temperature_schedule_name":null}});
                        } else if method == "Flow/Zone" {
                            bad["OutdoorAir:Node"] =
                                json!({"Outside 각":{"wind_direction_schedule_name":null}});
                        } else if method == "Flow/Area" {
                            bad["OutdoorAir:Node"] =
                                json!({"Outside 각":{"wind_speed_schedule_name":1.0}});
                        } else if method == "AirChanges/Hour" {
                            bad["OutdoorAir:Node"] = json!({"Outside 각":{"wind_direction_schedule_name":" Constant-1.0 "}});
                        } else {
                            bad["DesignSpecification:OutdoorAir"]["Actual OA"]["proportional_control_minimum_outdoor_air_flow_rate_schedule_name"] =
                                json!("Missing");
                        }
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
            warning["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["demand_controlled_ventilation_type"] =
                json!("OccupancySchedule");
            warning["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_economizer_type"] =
                json!("DifferentialEnthalpy");
            warning["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heat_recovery_type"] =
                json!("Enthalpy");
            warning["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["sensible_heat_recovery_effectiveness"] =
                json!(1.0);
            warning["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["latent_heat_recovery_effectiveness"] =
                json!(0.0);
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
            assert!(ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_err());
            ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                .expect("Source retains the named inactive occupancy selector without OA");
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
            assert_eq!(
                warning_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("actual inactive DCV raw fields")["demand_controlled_ventilation_type"],
                SourceRaw::String("OccupancySchedule".into())
            );
            assert_eq!(
                warning_model.ideal_loads_air_systems[0].outdoor_air_economizer_type,
                ep_model::OutdoorAirEconomizerType::DifferentialEnthalpy
            );
            assert_eq!(
                warning_model.ideal_loads_air_systems[0].heat_recovery_type,
                ep_model::HeatRecoveryType::Enthalpy
            );
            assert_eq!(
                warning_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained inactive ventilation")["outdoor_air_economizer_type"],
                SourceRaw::String("DifferentialEnthalpy".into())
            );
            assert_eq!(
                warning_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained inactive ventilation")["heat_recovery_type"],
                SourceRaw::String("Enthalpy".into())
            );
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
            exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["demand_controlled_ventilation_type"] =
                json!("CO2Setpoint");
            exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_economizer_type"] =
                json!("DifferentialDryBulb");
            exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heat_recovery_type"] =
                json!("Sensible");
            exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["sensible_heat_recovery_effectiveness"] =
                json!(0.0);
            exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["latent_heat_recovery_effectiveness"] =
                json!(1.0);
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
            assert_eq!(
                exhaust_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("actual inactive CO2 raw fields")["demand_controlled_ventilation_type"],
                SourceRaw::String("CO2Setpoint".into())
            );
            assert_eq!(
                exhaust_model.ideal_loads_air_systems[0].outdoor_air_economizer_type,
                ep_model::OutdoorAirEconomizerType::DifferentialDryBulb
            );
            assert_eq!(
                exhaust_model.ideal_loads_air_systems[0].heat_recovery_type,
                ep_model::HeatRecoveryType::Sensible
            );
            assert_eq!(
                exhaust_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained inactive ventilation")["outdoor_air_economizer_type"],
                SourceRaw::String("DifferentialDryBulb".into())
            );
            assert_eq!(
                exhaust_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("retained inactive ventilation")["heat_recovery_type"],
                SourceRaw::String("Sensible".into())
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
                    ("demand_controlled_ventilation_type", heating),
                    ("outdoor_air_economizer_type", heating),
                    ("heat_recovery_type", heating),
                ] {
                    if let Some(value) = authored {
                        fuel_input["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][field] = json!(value);
                    } else {
                        fuel_input["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]
                            .as_object_mut()
                            .expect("actual retained empty-or-omitted unit")
                            .remove(field);
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
                assert_eq!(
                    (
                        model.ideal_loads_air_systems[0].demand_controlled_ventilation_type,
                        model.ideal_loads_air_systems[0].outdoor_air_economizer_type,
                        model.ideal_loads_air_systems[0].heat_recovery_type,
                    ),
                    (
                        ep_model::DemandControlledVentilationType::None,
                        ep_model::OutdoorAirEconomizerType::NoEconomizer,
                        ep_model::HeatRecoveryType::None,
                    )
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
                    ("demand_controlled_ventilation_type", heating),
                    ("outdoor_air_economizer_type", heating),
                    ("heat_recovery_type", heating),
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
                // Reuse existing rows: omitted/explicit SHR, both Constant,
                // and inactive None/None reach the actual Source lane.
                let high_w = (limit == "NoLimit"
                    && ((matches!(
                        dehumidification,
                        "SCHEMA_DEFAULT" | "ConstantSensibleHeatRatio"
                    ) && humidification == "None")
                        || (dehumidification == "ConstantSupplyHumidityRatio"
                            && humidification == "ConstantSupplyHumidityRatio")))
                    || (limit == "LimitCapacity"
                        && dehumidification == "None"
                        && humidification == "None");
                let humidity_ratio = if high_w {
                    1.2_f64
                } else {
                    0.006_123_456_789_f64
                };
                let high_heating_w = high_w && humidification == "ConstantSupplyHumidityRatio";
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
                if high_heating_w {
                    humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_heating_supply_air_humidity_ratio"] =
                        json!(1.1);
                }
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
                if high_w {
                    let legacy = ep_compiler::compile_raw_model(&raw);
                    assert!(legacy.report.diagnostics.iter().any(|row| row.code
                        == "InvalidNumericRange"
                        && row.object_type == "ZoneHVAC:IdealLoadsAirSystem"
                        && row.field.as_deref()
                            == Some("minimum_cooling_supply_air_humidity_ratio")));
                    if high_heating_w {
                        assert_eq!(
                            unit.maximum_heating_supply_air_humidity_ratio.to_bits(),
                            1.1_f64.to_bits()
                        );
                        assert!(legacy.report.diagnostics.iter().any(|row| row.code
                            == "InvalidNumericRange"
                            && row.object_type == "ZoneHVAC:IdealLoadsAirSystem"
                            && row.field.as_deref()
                                == Some("maximum_heating_supply_air_humidity_ratio")));
                    }
                }
                if matches!(limit, "LimitCapacity" | "LimitFlowRateAndCapacity") {
                    assert!(matches!(unit.maximum_total_cooling_capacity_w,
                        Some(ep_model::AutosizeOrNumber::Value(value)) if value.to_bits() == 900.0_f64.to_bits()));
                }
                let simulation = ep_model::SimulationModel::from_typed(model.clone());
                ep_runtime::validate_source_direct_zone_purchased_air_model(&simulation)
                    .expect("Source constant cooling eligibility");
                if !high_w {
                    if dehumidification == "None" && humidification == "None" {
                        ep_runtime::bind_direct_zone_purchased_air_model(&simulation)
                            .expect("unchanged Legacy sensible numeric-capacity binding");
                    } else {
                        assert!(
                            ep_runtime::bind_direct_zone_purchased_air_model(&simulation).is_err()
                        );
                    }
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
                if high_heating_w {
                    assert_eq!(
                        fields["maximum_heating_supply_air_humidity_ratio"],
                        SourceRaw::Number("1.1".into())
                    );
                }
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
            // Advisory typed SHR stays separate from the actual getter's Invalid.
            let empty_ip = crate::ordinary_input_processor::resolve_source_b(&empty_raw)
                .expect("owned NoOA exact-empty Source IP");
            assert_eq!(
                empty_ip
                    .source_raw_object_fields("ZoneHVAC:IdealLoadsAirSystem", "Ideal")
                    .expect("actual empty raw fields")
                    .get("dehumidification_control_type"),
                Some(&SourceRaw::String(String::new()))
            );
            let mut empty_scope =
                inspect_porting_scope(&empty_raw, &empty_compile.report, Some(empty_model), scope);
            assert!(empty_scope.admissible, "{:?}", empty_scope.violations);
            empty_scope.environment = Some(
                super::environment::environment_from_weather(
                    empty_model,
                    &single_day_weather_fixture(),
                    "actual empty NoOA admission weather",
                )
                .expect("real empty NoOA environment"),
            );
            let empty_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &empty_raw,
                &empty_scope,
            )
            .expect("same exact-empty Source token");
            let empty_support = crate::support::assess_support_with_source_admission(
                &empty_raw,
                &empty_compile.report,
                Some(empty_model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&empty_admission),
            );
            assert!(
                empty_support.allows_rust_runtime(),
                "{:?}",
                empty_support.diagnostics
            );
            assert_eq!(
                empty_support.runtime_class,
                crate::support::RuntimeClass::IdealLoadsDirectZoneCoupledCompatibility
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
                    // Reuse the two existing negative contexts for distinct crossed holds.
                    if humidification == "Humidistat" {
                        humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["dehumidification_control_type"] =
                            json!("");
                    } else {
                        humidity["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["humidification_control_type"] =
                            json!("");
                    }
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
                no_exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["demand_controlled_ventilation_type"] =
                    json!("None");
                no_exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_economizer_type"] =
                    json!("NoEconomizer");
                no_exhaust["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["heat_recovery_type"] =
                    json!("None");
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
    // Same real Source B material/IP/support owner chain. SGS is unused by EVERY
    // Construction, including declarations that no current Surface consumes.
    let mut value = input(PortingScope::B);
    value["WindowMaterial:SimpleGlazingSystem"] =
        json!({"MiXeD Simple":{"u_factor":4.2,"solar_heat_gain_coefficient":0.57}});
    let raw = parse_epjson_str(&value.to_string()).expect("one unused Source SGS");
    let compiled = ep_compiler::compile_raw_model_for_source(&raw);
    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
    let model = compiled.model.as_ref().expect("actual Source SGS material");
    let ip = crate::ordinary_input_processor::resolve_source_b(&raw)
        .expect("real Source B/raw SGS admission");
    assert_eq!(
        ip.get_num_objects_found("WindowMaterial:SimpleGlazingSystem"),
        1
    );
    let projected =
        crate::ordinary_material_input::resolve_source(&raw, &ip).expect("same raw material owner");
    assert_eq!(projected.simple_glazing.len(), 1);
    assert_eq!(projected.simple_glazing[0].object_name, "MiXeD Simple");
    assert!(
        !projected.simple_glazing[0]
            .fields
            .contains_key("visible_transmittance")
    );
    assert_eq!(
        projected.simple_glazing[0].fields["u_factor"],
        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(4.2)
    );
    let mut admitted = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B);
    assert!(admitted.admissible, "{:?}", admitted.violations);
    admitted.environment = Some(
        super::environment::environment_from_weather(
            model,
            &weather,
            "existing material-owner weather",
        )
        .expect("actual calendar/environment"),
    );
    let admission = crate::support::SourceSupportAdmission::from_validated_scope(&raw, &admitted)
        .expect("complete unused-SGS Source prerequisites");
    let selected = crate::support::assess_support_with_source_admission(
        &raw,
        &compiled.report,
        Some(model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&admission),
    );
    assert!(selected.allows_rust_runtime(), "{:?}", selected.diagnostics);
    let mut without_raw = raw.clone();
    without_raw.objects.remove(&ep_raw_model::ObjectType(
        "WindowMaterial:SimpleGlazingSystem".into(),
    ));
    let no_raw_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&without_raw, &admitted)
            .expect("remaining B raw still validates, without an SGS token");
    let no_raw = crate::support::assess_support_with_source_admission(
        &without_raw,
        &compiled.report,
        Some(model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&no_raw_admission),
    );
    assert!(
        !no_raw.allows_rust_runtime(),
        "absent Raw SGS cannot waive an independently supplied typed SGS"
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
    assert!(
        !legacy.allows_rust_runtime(),
        "no unscoped SGS compatibility waiver"
    );
    assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve_source_a(&raw).is_err());
    assert!(
        !inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A).admissible
    );
    value["Construction"]["Unused Window"] = json!({"outside_layer":"MiXeD Simple"});
    let consumed =
        parse_epjson_str(&value.to_string()).expect("unused construction still references SGS");
    assert!(crate::ordinary_equipment_input::resolve_source_b(&consumed).is_err());
    let consumed_model = ep_compiler::compile_raw_model_for_source(&consumed);
    assert!(
        !consumed_model.has_errors(),
        "the typed graph still retains the held window construction"
    );
    assert!(
        !inspect_porting_scope(
            &consumed,
            &consumed_model.report,
            consumed_model.model.as_ref(),
            PortingScope::B
        )
        .admissible
    );
    value["Construction"]
        .as_object_mut()
        .expect("same construction map")
        .remove("Unused Window");
    value["WindowMaterial:SimpleGlazingSystem"]["MiXeD Simple"]["visible_transmittance"] =
        json!("");
    let blank = parse_epjson_str(&value.to_string()).expect("explicit invalid VT stays authored");
    assert!(crate::ordinary_equipment_input::resolve_source_b(&blank).is_err());
    value["WindowMaterial:SimpleGlazingSystem"]["MiXeD Simple"]["visible_transmittance"] =
        json!(0.0);
    let zero = parse_epjson_str(&value.to_string()).expect("zero is not omission");
    assert!(crate::ordinary_equipment_input::resolve_source_b(&zero).is_err());
    // Source compilation, retained raw/IP operands and actual support share the
    // same named-unused Gas input. Every pre-existing material case above stays.
    const GAS: &str = "WindowMaterial:Gas";
    let mut gas_value = input(PortingScope::B);
    gas_value[GAS] = json!({
        " Air ": {"gas_type":"Air", "thickness":0.012,
            "conductivity_coefficient_a":-1.0,"conductivity_coefficient_b":-2.0,
            "conductivity_coefficient_c":0.0,"viscosity_coefficient_a":1.0,
            "viscosity_coefficient_b":-3.0,"viscosity_coefficient_c":0.0,
            "specific_heat_coefficient_a":1.0,"specific_heat_coefficient_b":-4.0,
            "specific_heat_coefficient_c":0.0,"molecular_weight":20.0,
            "specific_heat_ratio":1.5,"name":"ignored body identity",
            "idf_order":{"nested":[null,true]},"idf_max_fields":[],
            "metadata":null},
        "Air": {"gas_type":"Argon", "thickness":0.013},
        "\u{a0}": {"gas_type":"Krypton", "thickness":0.014},
        "\u{b}": {"gas_type":"Xenon", "thickness":0.015}
    });
    let gas_raw = parse_epjson_str(&gas_value.to_string()).expect("same legal named Gas Raw");
    // This public Source entry sets source_file_declarations; a Legacy compile
    // cannot establish the newly required actual loader-name/ID association.
    let gas_compiled = ep_compiler::compile_raw_model_for_source(&gas_raw);
    assert!(
        !gas_compiled.has_errors(),
        "{:?}",
        gas_compiled.report.diagnostics
    );
    let gas_model = gas_compiled
        .model
        .as_ref()
        .expect("actual Source Gas identities");
    assert!(gas_model.material_names.is_source_exact_namespace());
    assert_ne!(
        gas_model.material_names.resolve(" Air "),
        gas_model.material_names.resolve("Air")
    );
    for name in [" Air ", "Air", "\u{a0}", "\u{b}"] {
        let id = gas_model
            .material_names
            .resolve(name)
            .expect("same named Gas ID");
        assert!(
            gas_model
                .ctf_report_names
                .materials
                .contains(&(id, name.to_ascii_uppercase()))
        );
    }
    let mut gas_ip = crate::ordinary_input_processor::resolve_source_b(&gas_raw)
        .expect("all known fields validated on original Raw");
    assert_eq!(gas_ip.get_num_objects_found(GAS), 4);
    let gas_args = gas_ip
        .get_object_def_max_args(GAS)
        .expect("real Gas schema definition");
    assert_eq!(
        (
            gas_args.alphas,
            gas_args.numerics,
            gas_args.total().unwrap()
        ),
        (2, 12, 14)
    );
    let original = gas_ip.source_raw_object_fields(GAS, " Air ").unwrap();
    assert!(
        matches!(original.get("name"), Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::String(value)) if value == "ignored body identity")
    );
    assert!(matches!(
        original.get("idf_order"),
        Some(
            ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Object(
                _
            )
        )
    ));
    assert!(
        matches!(original.get("idf_max_fields"), Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Array(rows)) if rows.is_empty())
    );
    assert!(matches!(
        original.get("metadata"),
        Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Null)
    ));
    let gas_input = crate::ordinary_material_input::resolve_source(&gas_raw, &gas_ip)
        .expect("same real Gas operand projection");
    assert_eq!(gas_input.window_gas.len(), 4);
    let gas_row = gas_input
        .window_gas
        .iter()
        .find(|row| row.object_name == " Air ")
        .unwrap();
    assert_eq!(gas_row.fields.len(), 13);
    assert!(gas_row.idf_max_fields.is_none());
    assert!(gas_row.fields.contains_key("specific_heat_ratio"));
    for ignored in ["name", "idf_order", "idf_max_fields", "metadata"] {
        assert!(!gas_row.fields.contains_key(ignored));
    }
    let mut stats = ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
    gas_ip
        .report_source_idf_records_stats(&mut stats)
        .expect("genuine complete input statistics");
    let all_records: usize = gas_raw.objects.values().map(|objects| objects.len()).sum();
    assert_eq!(
        usize::try_from(stats.number_input_records).unwrap(),
        all_records
    );
    assert!(
        gas_ip.is_object_unused(GAS, " Air "),
        "projection/statistics do not run GetObjectItem"
    );
    let mut gas_trace = inspect_porting_scope(
        &gas_raw,
        &gas_compiled.report,
        Some(gas_model),
        PortingScope::B,
    );
    assert!(gas_trace.admissible, "{:?}", gas_trace.violations);
    let gas_environment = super::environment::environment_from_weather(
        gas_model,
        &weather,
        "existing Gas caller weather",
    )
    .unwrap();
    assert!(validate_porting_environment(&gas_environment).is_empty());
    gas_trace.environment = Some(gas_environment);
    let gas_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&gas_raw, &gas_trace).unwrap();
    let gas_support = crate::support::assess_support_with_source_admission(
        &gas_raw,
        &gas_compiled.report,
        Some(gas_model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&gas_admission),
    );
    assert!(
        gas_support.allows_rust_runtime(),
        "{:?}",
        gas_support.diagnostics
    );
    assert!(
        !assess_support(
            &gas_raw,
            &gas_compiled.report,
            Some(gas_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal
        )
        .allows_rust_runtime()
    );
    assert!(crate::ordinary_equipment_input::resolve(&gas_raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve_source_a(&gas_raw).is_err());
    let mut no_gas_raw = gas_raw.clone();
    no_gas_raw
        .objects
        .remove(&ep_raw_model::ObjectType(GAS.into()));
    let no_gas_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&no_gas_raw, &gas_trace)
            .unwrap();
    assert!(
        !crate::support::assess_support_with_source_admission(
            &no_gas_raw,
            &gas_compiled.report,
            Some(gas_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&no_gas_admission)
        )
        .allows_rust_runtime()
    );
    let gas_id = gas_model.material_names.resolve("Air").unwrap();
    let mut custom_typed = gas_model.clone();
    let custom_row = custom_typed
        .materials
        .iter_mut()
        .find(|row| row.id == gas_id)
        .unwrap();
    assert!(
        matches!(
            &custom_row.definition,
            ep_model::MaterialDefinition::WindowGas(_)
        ),
        "same actual Gas kind"
    );
    if let ep_model::MaterialDefinition::WindowGas(custom) = &mut custom_row.definition {
        custom.gas_type = ep_model::WindowGasType::Custom;
    }
    assert!(
        !crate::support::assess_support_with_source_admission(
            &gas_raw,
            &gas_compiled.report,
            Some(&custom_typed),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&gas_admission)
        )
        .allows_rust_runtime()
    );
    let mut used_typed = gas_model.clone();
    used_typed.constructions[0].layers.push(gas_id);
    assert!(
        !crate::support::assess_support_with_source_admission(
            &gas_raw,
            &gas_compiled.report,
            Some(&used_typed),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&gas_admission)
        )
        .allows_rust_runtime()
    );
    // Original schema bounds precede the signed-i64-effective Native operand.
    let mut wrapped_value = gas_value.clone();
    wrapped_value[GAS]["Air"]["thickness"] = json!(u64::MAX);
    let wrapped_raw = parse_epjson_str(&wrapped_value.to_string()).unwrap();
    let wrapped_compiled = ep_compiler::compile_raw_model_for_source(&wrapped_raw);
    assert!(!wrapped_compiled.has_errors());
    let wrapped_model = wrapped_compiled.model.as_ref().unwrap();
    let wrapped_id = wrapped_model.material_names.resolve("Air").unwrap();
    assert!(
        wrapped_model
            .materials
            .iter()
            .find(|row| row.id == wrapped_id)
            .unwrap()
            .as_window_gas()
            .unwrap()
            .thickness_m
            > 0.0
    );
    let wrapped_ip = crate::ordinary_input_processor::resolve_source_b(&wrapped_raw).unwrap();
    let wrapped_input =
        crate::ordinary_material_input::resolve_source(&wrapped_raw, &wrapped_ip).unwrap();
    assert_eq!(
        wrapped_input
            .window_gas
            .iter()
            .find(|row| row.object_name == "Air")
            .unwrap()
            .fields["thickness"],
        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(-1.0)
    );
    for (field, invalid) in [
        ("gas_type", json!("Steam")),
        ("gas_type", json!("air")),
        ("gas_type", json!(null)),
        ("thickness", json!("0.012")),
        ("thickness", json!(0.0)),
        ("conductivity_coefficient_a", json!([])),
        ("viscosity_coefficient_a", json!(0.0)),
        ("specific_heat_coefficient_a", json!(0.0)),
        ("molecular_weight", json!(19.0)),
        ("molecular_weight", json!(201.0)),
        ("specific_heat_ratio", json!(1.0)),
    ] {
        let mut bad = gas_value.clone();
        bad[GAS]["Air"][field] = invalid;
        let raw = parse_epjson_str(&bad.to_string()).unwrap();
        assert!(
            crate::ordinary_input_processor::resolve_source_b(&raw).is_err(),
            "{field}"
        );
    }
    let mut missing = gas_value.clone();
    missing[GAS]["Air"]
        .as_object_mut()
        .unwrap()
        .remove("thickness");
    assert!(
        crate::ordinary_input_processor::resolve_source_b(
            &parse_epjson_str(&missing.to_string()).unwrap()
        )
        .is_err()
    );
    for field in ["outside_layer", "layer_2"] {
        let mut used = gas_value.clone();
        used["Construction"]["Unused Gas Window"] = if field == "outside_layer" {
            json!({"outside_layer":"Air"})
        } else {
            json!({"outside_layer":"Opaque", "layer_2":"Air"})
        };
        let raw = parse_epjson_str(&used.to_string()).unwrap();
        assert!(crate::ordinary_equipment_input::resolve_source_b(&raw).is_err());
    }
    // Authentic Custom-only Source declaration/IP/admission, after old Named
    // cases. Missing optional values and wrapped operands stay raw-owned.
    let mut custom_value = gas_value.clone();
    custom_value[GAS] = json!({
        "C Custom":{"gas_type":"Custom","thickness":0.012,"conductivity_coefficient_a":0.02,
            "viscosity_coefficient_a":0.00001,"specific_heat_coefficient_a":1000.0,"molecular_weight":28.0,
            "name":{"ignored":[true,null]},"idf_order":false},
        "F Effective":{"gas_type":"Custom","thickness":u64::MAX,"conductivity_coefficient_a":0.03,
            "viscosity_coefficient_a":0.00001,"specific_heat_coefficient_a":900.0,"molecular_weight":30.0,
            "specific_heat_ratio":u64::MAX}
    });
    let custom_raw = parse_epjson_str(&custom_value.to_string()).unwrap();
    let custom_compiled = ep_compiler::compile_raw_model_for_source(&custom_raw);
    assert!(
        !custom_compiled.has_errors(),
        "{:?}",
        custom_compiled.report.diagnostics
    );
    let custom_model = custom_compiled.model.as_ref().unwrap();
    assert!(custom_model.material_names.is_source_exact_namespace());
    let custom_id = custom_model.material_names.resolve("C Custom").unwrap();
    assert!(
        custom_model
            .ctf_report_names
            .materials
            .contains(&(custom_id, "C CUSTOM".into()))
    );
    let custom_ip = crate::ordinary_input_processor::resolve_source_b(&custom_raw).unwrap();
    let custom_input =
        crate::ordinary_material_input::resolve_source(&custom_raw, &custom_ip).unwrap();
    let omitted = custom_input
        .window_gas
        .iter()
        .find(|row| row.object_name == "C Custom")
        .unwrap();
    assert!(!omitted.fields.contains_key("specific_heat_ratio"));
    assert!(!omitted.fields.contains_key("name"));
    assert!(!omitted.fields.contains_key("idf_order"));
    let effective = custom_input
        .window_gas
        .iter()
        .find(|row| row.object_name == "F Effective")
        .unwrap();
    assert_eq!(
        effective.fields["thickness"],
        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(-1.0)
    );
    assert_eq!(
        effective.fields["specific_heat_ratio"],
        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(-1.0)
    );
    let mut custom_trace = inspect_porting_scope(
        &custom_raw,
        &custom_compiled.report,
        Some(custom_model),
        PortingScope::B,
    );
    assert!(custom_trace.admissible, "{:?}", custom_trace.violations);
    custom_trace.environment = Some(
        super::environment::environment_from_weather(
            custom_model,
            &weather,
            "existing Custom material weather",
        )
        .unwrap(),
    );
    let custom_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&custom_raw, &custom_trace)
            .unwrap();
    let actual = crate::support::assess_support_with_source_admission(
        &custom_raw,
        &custom_compiled.report,
        Some(custom_model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&custom_admission),
    );
    assert!(actual.allows_rust_runtime(), "{:?}", actual.diagnostics);
    assert!(crate::ordinary_equipment_input::resolve(&custom_raw).is_err());
    assert!(crate::ordinary_equipment_input::resolve_source_a(&custom_raw).is_err());
    assert!(
        !assess_support(
            &custom_raw,
            &custom_compiled.report,
            Some(custom_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal
        )
        .allows_rust_runtime()
    );
    // Mutate genuine compiled models only to test classification closure.
    let mut bad_models = Vec::new();
    let mut alias = custom_model.clone();
    assert!(
        alias
            .material_names
            .insert("EXTRA ALIAS", custom_id)
            .is_none()
    );
    bad_models.push(alias);
    let mut duplicate_alias = custom_model.clone();
    duplicate_alias
        .material_names
        .insert_instance("C CUSTOM", custom_id);
    bad_models.push(duplicate_alias);
    let mut loader_alias = custom_model.clone();
    loader_alias
        .ctf_report_names
        .materials
        .push((custom_id, "EXTRA LOADER".into()));
    bad_models.push(loader_alias);
    let mut missing_loader = custom_model.clone();
    missing_loader
        .ctf_report_names
        .materials
        .retain(|(id, _)| *id != custom_id);
    bad_models.push(missing_loader);
    let mut wrong_type = custom_model.clone();
    let row = wrong_type
        .materials
        .iter_mut()
        .find(|row| row.id == custom_id)
        .unwrap();
    assert!(matches!(
        &row.definition,
        ep_model::MaterialDefinition::WindowGas(_)
    ));
    if let ep_model::MaterialDefinition::WindowGas(gas) = &mut row.definition {
        gas.gas_type = ep_model::WindowGasType::Air;
    }
    bad_models.push(wrong_type);
    let mut wrong_name = custom_model.clone();
    wrong_name
        .materials
        .iter_mut()
        .find(|row| row.id == custom_id)
        .unwrap()
        .name
        .0 = "WRONG ARENA NAME".into();
    bad_models.push(wrong_name);
    let mut extra_typed = custom_model.clone();
    let mut extra_row = extra_typed
        .materials
        .iter()
        .find(|row| row.id == custom_id)
        .unwrap()
        .clone();
    extra_row.id = ep_model::MaterialId(u32::try_from(extra_typed.materials.len()).unwrap());
    extra_row.name.0 = "EXTRA TYPED".into();
    extra_typed.materials.push(extra_row);
    bad_models.push(extra_typed);
    let mut used_fallback = custom_model.clone();
    used_fallback.constructions[0].layers.clear();
    used_fallback.constructions[0].outside_layer = Some(custom_id);
    bad_models.push(used_fallback);
    for bad_model in &bad_models {
        assert!(
            !crate::support::assess_support_with_source_admission(
                &custom_raw,
                &custom_compiled.report,
                Some(bad_model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&custom_admission)
            )
            .allows_rust_runtime()
        );
    }
    let mut extra_raw = custom_raw.clone();
    let row = extra_raw.objects[&ep_raw_model::ObjectType(GAS.into())]
        [&ep_raw_model::ObjectName("C Custom".into())]
        .clone();
    extra_raw
        .objects
        .get_mut(&ep_raw_model::ObjectType(GAS.into()))
        .unwrap()
        .insert(ep_raw_model::ObjectName("Z EXTRA".into()), row);
    let extra_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&extra_raw, &custom_trace)
            .unwrap();
    assert!(
        !crate::support::assess_support_with_source_admission(
            &extra_raw,
            &custom_compiled.report,
            Some(custom_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&extra_admission)
        )
        .allows_rust_runtime()
    );
    let mut without_custom = custom_raw.clone();
    without_custom
        .objects
        .remove(&ep_raw_model::ObjectType(GAS.into()));
    let absent_admission = crate::support::SourceSupportAdmission::from_validated_scope(
        &without_custom,
        &custom_trace,
    )
    .unwrap();
    assert!(
        !crate::support::assess_support_with_source_admission(
            &without_custom,
            &custom_compiled.report,
            Some(custom_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&absent_admission)
        )
        .allows_rust_runtime()
    );
    // Original-schema-valid missing/derived failures declare Source IDs so the
    // actual runtime owner can diagnose; legacy validation stays distinct.
    let mut missing_value = custom_value.clone();
    for field in [
        "viscosity_coefficient_a",
        "specific_heat_coefficient_a",
        "molecular_weight",
    ] {
        missing_value[GAS]["C Custom"]
            .as_object_mut()
            .unwrap()
            .remove(field);
    }
    let missing_raw = parse_epjson_str(&missing_value.to_string()).unwrap();
    assert!(crate::ordinary_input_processor::resolve_source_b(&missing_raw).is_ok());
    let source_missing = ep_compiler::compile_raw_model_for_source(&missing_raw);
    assert!(
        !source_missing.has_errors(),
        "{:?}",
        source_missing.report.diagnostics
    );
    assert!(
        compile_raw_model(&missing_raw)
            .report
            .diagnostics
            .iter()
            .any(|row| row.code == "MissingCustomWindowGasProperty")
    );
    let mut zero_value = custom_value.clone();
    zero_value[GAS]["C Custom"]["conductivity_coefficient_a"] = json!(0.0);
    let zero_raw = parse_epjson_str(&zero_value.to_string()).unwrap();
    assert!(!ep_compiler::compile_raw_model_for_source(&zero_raw).has_errors());
    assert!(
        compile_raw_model(&zero_raw)
            .report
            .diagnostics
            .iter()
            .any(|row| row.code == "InvalidWindowGasConductivityAt300K")
    );
    for (field, invalid) in [
        ("gas_type", json!("custom")),
        ("viscosity_coefficient_a", json!(0.0)),
        ("specific_heat_coefficient_a", json!(null)),
        ("molecular_weight", json!(201.0)),
        ("specific_heat_ratio", json!(1.0)),
    ] {
        let mut bad = custom_value.clone();
        bad[GAS]["C Custom"][field] = invalid;
        assert!(
            crate::ordinary_input_processor::resolve_source_b(
                &parse_epjson_str(&bad.to_string()).unwrap()
            )
            .is_err()
        );
    }
    custom_value["Construction"]["Unused Custom Window"] = json!({"outside_layer":"C Custom"});
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &parse_epjson_str(&custom_value.to_string()).unwrap()
        )
        .is_err()
    );
    // The same real Source compile/IP/environment/support chain now owns a
    // Mixture together with genuine Named and Custom single-Gas neighbours.
    const MIXTURE: &str = "WindowMaterial:GasMixture";
    let mut mixture_value = input(PortingScope::B);
    mixture_value[GAS] = json!({
        "Named Neighbour":{"gas_type":"Air","thickness":0.012},
        "Custom Neighbour":{"gas_type":"Custom","thickness":0.012,
            "conductivity_coefficient_a":0.02,"viscosity_coefficient_a":0.00001,
            "specific_heat_coefficient_a":1000.0,"molecular_weight":28.0}
    });
    mixture_value[MIXTURE] = json!({" Mix \u{a0}":{
        "thickness":0.012,"number_of_gases_in_mixture":3,
        "gas_1_type":"Air","gas_1_fraction":0.4,
        "gas_2_type":"Air","gas_2_fraction":0.8,
        "gas_3_type":"Krypton","gas_4_type":"Xenon","gas_4_fraction":0.7,
        "name":false,"idf_order":[],"metadata":{"retained":[null,1.25]}
    }});
    let mixture_raw = parse_epjson_str(&mixture_value.to_string()).unwrap();
    let mixture_compiled = ep_compiler::compile_raw_model_for_source(&mixture_raw);
    assert!(
        !mixture_compiled.has_errors(),
        "{:?}",
        mixture_compiled.report.diagnostics
    );
    let mixture_model = mixture_compiled.model.as_ref().unwrap();
    let mixture_id = mixture_model.material_names.resolve(" Mix \u{a0}").unwrap();
    assert!(
        mixture_model
            .ctf_report_names
            .materials
            .contains(&(mixture_id, " MIX \u{a0}".into()))
    );
    let mut mixture_ip = crate::ordinary_input_processor::resolve_source_b(&mixture_raw).unwrap();
    let args = mixture_ip.get_object_def_max_args(MIXTURE).unwrap();
    assert_eq!(
        (args.alphas, args.numerics, args.total().unwrap()),
        (5, 6, 11)
    );
    let original = mixture_ip
        .source_raw_object_fields(MIXTURE, " Mix \u{a0}")
        .unwrap();
    assert!(
        matches!(original.get("number_of_gases_in_mixture"), Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Number(token)) if token == "3")
    );
    assert!(matches!(
        original.get("metadata"),
        Some(
            ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Object(
                _
            )
        )
    ));
    let projected =
        crate::ordinary_material_input::resolve_source(&mixture_raw, &mixture_ip).unwrap();
    assert_eq!(
        (
            projected.window_gas.len(),
            projected.window_gas_mixture.len()
        ),
        (2, 1)
    );
    assert_eq!(projected.window_gas_mixture[0].fields.len(), 9);
    let mut stats = ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
    mixture_ip
        .report_source_idf_records_stats(&mut stats)
        .unwrap();
    assert_eq!(
        usize::try_from(stats.number_input_records).unwrap(),
        mixture_raw
            .objects
            .values()
            .map(|rows| rows.len())
            .sum::<usize>()
    );
    assert!(mixture_ip.is_object_unused(MIXTURE, " Mix \u{a0}"));
    let mut mixture_trace = inspect_porting_scope(
        &mixture_raw,
        &mixture_compiled.report,
        Some(mixture_model),
        PortingScope::B,
    );
    assert!(mixture_trace.admissible, "{:?}", mixture_trace.violations);
    mixture_trace.environment = Some(
        super::environment::environment_from_weather(
            mixture_model,
            &weather,
            "same existing Mixture caller weather",
        )
        .unwrap(),
    );
    let mixture_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&mixture_raw, &mixture_trace)
            .unwrap();
    let support = crate::support::assess_support_with_source_admission(
        &mixture_raw,
        &mixture_compiled.report,
        Some(mixture_model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&mixture_admission),
    );
    assert!(support.allows_rust_runtime(), "{:?}", support.diagnostics);
    assert!(
        crate::ordinary_equipment_input::resolve(&mixture_raw).is_err()
            && crate::ordinary_equipment_input::resolve_source_a(&mixture_raw).is_err()
    );
    let mut models = Vec::new();
    let mut alias = mixture_model.clone();
    alias
        .material_names
        .insert("EXTRA MIXTURE ALIAS", mixture_id);
    models.push(alias);
    let mut wrong_count = mixture_model.clone();
    let row = wrong_count
        .materials
        .iter_mut()
        .find(|row| row.id == mixture_id)
        .unwrap();
    if let ep_model::MaterialDefinition::WindowGasMixture(value) = &mut row.definition {
        value.gases = ep_model::WindowGasMixture::One([value.gases.components()[0]]);
    }
    models.push(wrong_count);
    let mut wrong_species = mixture_model.clone();
    let row = wrong_species
        .materials
        .iter_mut()
        .find(|row| row.id == mixture_id)
        .unwrap();
    if let ep_model::MaterialDefinition::WindowGasMixture(value) = &mut row.definition {
        let active = value.gases.components();
        value.gases = ep_model::WindowGasMixture::Three([active[0], active[1], active[0]]);
    }
    models.push(wrong_species);
    let mut wrong_kind = mixture_model.clone();
    let named = wrong_kind
        .materials
        .iter()
        .find(|row| row.kind() == ep_model::MaterialKind::WindowGas)
        .unwrap()
        .definition;
    wrong_kind
        .materials
        .iter_mut()
        .find(|row| row.id == mixture_id)
        .unwrap()
        .definition = named;
    models.push(wrong_kind);
    let mut extra_row = mixture_model.clone();
    let mut extra = extra_row
        .materials
        .iter()
        .find(|row| row.id == mixture_id)
        .unwrap()
        .clone();
    extra.id = ep_model::MaterialId(u32::try_from(extra_row.materials.len()).unwrap());
    extra_row.materials.push(extra);
    models.push(extra_row);
    let mut missing_loader = mixture_model.clone();
    missing_loader
        .ctf_report_names
        .materials
        .retain(|(id, _)| *id != mixture_id);
    models.push(missing_loader);
    let mut used = mixture_model.clone();
    used.constructions[0].layers.clear();
    used.constructions[0].outside_layer = Some(mixture_id);
    models.push(used);
    for model in models {
        assert!(
            !crate::support::assess_support_with_source_admission(
                &mixture_raw,
                &mixture_compiled.report,
                Some(&model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&mixture_admission)
            )
            .allows_rust_runtime()
        );
    }
    for (field, value) in [
        ("number_of_gases_in_mixture", json!(1.0)),
        ("number_of_gases_in_mixture", json!(5)),
        ("gas_1_type", json!("air")),
        ("gas_2_type", json!("Custom")),
        ("gas_2_fraction", json!(0.0)),
        ("gas_4_fraction", json!(1.1)),
        ("gas_4_type", json!(null)),
        ("thickness", json!("0.012")),
    ] {
        let mut bad = mixture_value.clone();
        bad[MIXTURE][" Mix \u{a0}"][field] = value;
        assert!(
            crate::ordinary_input_processor::resolve_source_b(
                &parse_epjson_str(&bad.to_string()).unwrap()
            )
            .is_err()
        );
    }
    let mut missing_active = mixture_value.clone();
    missing_active[MIXTURE][" Mix \u{a0}"]
        .as_object_mut()
        .unwrap()
        .remove("gas_3_type");
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &parse_epjson_str(&missing_active.to_string()).unwrap()
        )
        .is_err()
    );
    let mut missing_inactive = mixture_value.clone();
    missing_inactive[MIXTURE][" Mix \u{a0}"]["number_of_gases_in_mixture"] = json!(1);
    missing_inactive[MIXTURE][" Mix \u{a0}"]
        .as_object_mut()
        .unwrap()
        .remove("gas_2_type");
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &parse_epjson_str(&missing_inactive.to_string()).unwrap()
        )
        .is_err()
    );
    mixture_value["Construction"]["Mixture Used"] = json!({"outside_layer":" Mix \u{a0}"});
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &parse_epjson_str(&mixture_value.to_string()).unwrap()
        )
        .is_err()
    );

    // Actual Source Raw/IP/compile/weather/support, independent of the private
    // runtime property tests. No new output or physical fixture is created.
    const GLASS: &str = "WindowMaterial:Glazing";
    let mut glass_value = input(PortingScope::B);
    glass_value[GLASS] = json!({
        " DEFAULT ":{"optical_data_type":"SpectralAverage","thickness":0.003,"solar_diffusing":""},
        " WRAPPED ":{"optical_data_type":"SpectralAverage","thickness":u64::MAX,"conductivity":u64::MAX,"young_s_modulus":u64::MAX,
            "window_glass_spectral_data_set_name":"unknown spectrum",
            "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name":"",
            "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name":" ",
            "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name":"unused table 한",
            "name":false,"idf_order":[],"idf_max_fields":null,"extra":{"values":[null,true,1.25]}}
    });
    let glass_raw = parse_epjson_str(&glass_value.to_string()).unwrap();
    let glass_compiled = ep_compiler::compile_raw_model_for_source(&glass_raw);
    assert!(
        !glass_compiled.has_errors(),
        "{:?}",
        glass_compiled.report.diagnostics
    );
    let glass_model = glass_compiled.model.as_ref().unwrap();
    let glass_id = glass_model.material_names.resolve(" WRAPPED ").unwrap();
    assert!(
        glass_model.material_names.is_source_exact_namespace()
            && glass_model
                .ctf_report_names
                .materials
                .contains(&(glass_id, " WRAPPED ".into()))
    );
    let mut glass_ip = crate::ordinary_input_processor::resolve_source_b(&glass_raw).unwrap();
    let args = glass_ip.get_object_def_max_args(GLASS).unwrap();
    assert_eq!(
        (args.alphas, args.numerics, args.total().unwrap()),
        (7, 14, 21)
    );
    let original = glass_ip
        .source_raw_object_fields(GLASS, " WRAPPED ")
        .unwrap();
    assert!(
        matches!(original.get("thickness"), Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Number(token)) if token == "18446744073709551615")
            && original.contains_key("extra")
            && original.contains_key("idf_max_fields")
    );
    let projected = crate::ordinary_material_input::resolve_source(&glass_raw, &glass_ip).unwrap();
    assert_eq!(
        (
            projected.glazing.len(),
            projected.glazing[0].fields.len(),
            projected.glazing[1].fields.len()
        ),
        (2, 3, 8)
    );
    assert_eq!(
        ["thickness", "conductivity", "young_s_modulus"]
            .map(|field| projected.glazing[1].fields[field].clone()),
        [-1.0; 3].map(ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number)
    );
    assert!(
        matches!(&projected.glazing[1].fields["window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name"], ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(value) if value == " ")
            && !projected.glazing[1].fields.contains_key("extra")
    );
    let mut stats = ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
    glass_ip
        .report_source_idf_records_stats(&mut stats)
        .unwrap();
    assert_eq!(
        usize::try_from(stats.number_input_records).unwrap(),
        glass_raw
            .objects
            .values()
            .map(|rows| rows.len())
            .sum::<usize>()
    );
    assert!(
        glass_ip.is_object_unused(GLASS, " DEFAULT ")
            && glass_ip.is_object_unused(GLASS, " WRAPPED ")
    );
    let mut glass_trace = inspect_porting_scope(
        &glass_raw,
        &glass_compiled.report,
        Some(glass_model),
        PortingScope::B,
    );
    assert!(glass_trace.admissible, "{:?}", glass_trace.violations);
    glass_trace.environment = Some(
        super::environment::environment_from_weather(
            glass_model,
            &weather,
            "same existing Glass caller weather",
        )
        .unwrap(),
    );
    let glass_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&glass_raw, &glass_trace)
            .unwrap();
    let support = crate::support::assess_support_with_source_admission(
        &glass_raw,
        &glass_compiled.report,
        Some(glass_model),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&glass_admission),
    );
    assert!(support.allows_rust_runtime(), "{:?}", support.diagnostics);
    assert!(
        crate::ordinary_equipment_input::resolve(&glass_raw).is_err()
            && crate::ordinary_equipment_input::resolve_source_a(&glass_raw).is_err()
            && !assess_support(
                &glass_raw,
                &glass_compiled.report,
                Some(glass_model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal
            )
            .allows_rust_runtime()
    );
    let mut models = Vec::new();
    let mut all_missing = glass_model.clone();
    all_missing
        .materials
        .retain(|row| row.kind() != ep_model::MaterialKind::WindowGlazing);
    models.push(all_missing);
    let mut alias = glass_model.clone();
    alias.material_names.insert("EXTRA GLASS ALIAS", glass_id);
    models.push(alias);
    let mut duplicate_alias = glass_model.clone();
    duplicate_alias
        .material_names
        .insert_instance(" WRAPPED ", glass_id);
    models.push(duplicate_alias);
    let mut loader_alias = glass_model.clone();
    loader_alias
        .ctf_report_names
        .materials
        .push((glass_id, "EXTRA LOADER".into()));
    models.push(loader_alias);
    let mut missing_loader = glass_model.clone();
    missing_loader
        .ctf_report_names
        .materials
        .retain(|(id, _)| *id != glass_id);
    models.push(missing_loader);
    let mut wrong_id = glass_model.clone();
    wrong_id
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .id = ep_model::MaterialId(u32::MAX);
    models.push(wrong_id);
    let mut wrong_name = glass_model.clone();
    wrong_name
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .name = ep_model::NormalizedName::new("WRONG");
    models.push(wrong_name);
    let mut wrong_kind = glass_model.clone();
    let other = wrong_kind
        .materials
        .iter()
        .find(|row| row.kind() != ep_model::MaterialKind::WindowGlazing)
        .unwrap()
        .definition;
    wrong_kind
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .definition = other;
    models.push(wrong_kind);
    let mut extra = glass_model.clone();
    let mut row = extra
        .materials
        .iter()
        .find(|row| row.id == glass_id)
        .unwrap()
        .clone();
    row.id = ep_model::MaterialId(u32::try_from(extra.materials.len()).unwrap());
    extra.materials.push(row);
    models.push(extra);
    let mut used = glass_model.clone();
    used.constructions[0].layers.clear();
    used.constructions[0].outside_layer = Some(glass_id);
    models.push(used);
    for model in models {
        assert!(
            !crate::support::assess_support_with_source_admission(
                &glass_raw,
                &glass_compiled.report,
                Some(&model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&glass_admission)
            )
            .allows_rust_runtime()
        );
    }
    // A genuine no-Glass Raw/compile/trace token cannot cover an extra typed Glass.
    let mut empty_value = glass_value.clone();
    empty_value.as_object_mut().unwrap().remove(GLASS);
    let empty_raw = parse_epjson_str(&empty_value.to_string()).unwrap();
    let empty_compiled = ep_compiler::compile_raw_model_for_source(&empty_raw);
    let empty_model = empty_compiled.model.as_ref().unwrap();
    let mut empty_trace = inspect_porting_scope(
        &empty_raw,
        &empty_compiled.report,
        Some(empty_model),
        PortingScope::B,
    );
    empty_trace.environment = Some(
        super::environment::environment_from_weather(
            empty_model,
            &weather,
            "same existing no-Glass caller weather",
        )
        .unwrap(),
    );
    let empty_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&empty_raw, &empty_trace)
            .unwrap();
    assert!(
        !crate::support::assess_support_with_source_admission(
            &empty_raw,
            &empty_compiled.report,
            Some(glass_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&empty_admission)
        )
        .allows_rust_runtime()
    );
    for (field, value) in [
        ("optical_data_type", json!("BSDF")),
        ("optical_data_type", json!("spectralaverage")),
        ("solar_diffusing", json!("no")),
        ("solar_diffusing", json!(" ")),
        ("thickness", json!(0.0)),
        ("conductivity", json!(0.0)),
        ("young_s_modulus", json!(-1.0)),
        ("poisson_s_ratio", json!(1.0)),
        ("solar_transmittance_at_normal_incidence", json!(1.1)),
        ("window_glass_spectral_data_set_name", json!(null)),
        (
            "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name",
            json!(false),
        ),
        (
            "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name",
            json!([]),
        ),
        (
            "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name",
            json!({}),
        ),
    ] {
        let mut invalid = glass_value.clone();
        invalid[GLASS][" DEFAULT "][field] = value;
        assert!(
            crate::ordinary_input_processor::resolve_source_b(
                &parse_epjson_str(&invalid.to_string()).unwrap()
            )
            .is_err()
        );
    }
    glass_value["Construction"]["Glass Used"] = json!({"outside_layer":" WRAPPED "});
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &parse_epjson_str(&glass_value.to_string()).unwrap()
        )
        .is_err()
    );

    // Actual Source compiler/IP/statistics/weather/support. Unknown body
    // optical tags are metadata here, not a prerequisite for this family.
    const ALTERNATIVE: &str = "WindowMaterial:Glazing:RefractionExtinctionMethod";
    let mut alternative_value = input(PortingScope::B);
    alternative_value["WindowMaterial:Glazing"] =
        json!({"Average Neighbour":{"optical_data_type":"SpectralAverage","thickness":0.003}});
    alternative_value[ALTERNATIVE] = json!({
        " DEFAULT ":{"thickness":0.003,"solar_index_of_refraction":1.5,"solar_extinction_coefficient":20.0,"visible_index_of_refraction":1.7,"visible_extinction_coefficient":100.0,"solar_diffusing":""},
        " WRAPPED ":{"thickness":u64::MAX,"solar_index_of_refraction":u64::MAX,"solar_extinction_coefficient":u64::MAX,"visible_index_of_refraction":u64::MAX,"visible_extinction_coefficient":u64::MAX,"conductivity":u64::MAX,"solar_diffusing":"Yes",
            "optical_data_type":{"ignored":true},"young_s_modulus":false,"poisson_s_ratio":[],"name":null,"idf_order":{},"idf_max_fields":false,"metadata":[null,1.25]}
    });
    let alternative_raw = parse_epjson_str(&alternative_value.to_string()).unwrap();
    let compiled = ep_compiler::compile_raw_model_for_source(&alternative_raw);
    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
    let model = compiled.model.as_ref().unwrap();
    let id = model.material_names.resolve(" WRAPPED ").unwrap();
    assert!(
        model.material_names.is_source_exact_namespace()
            && model
                .ctf_report_names
                .materials
                .contains(&(id, " WRAPPED ".into()))
    );
    let mut ip = crate::ordinary_input_processor::resolve_source_b(&alternative_raw).unwrap();
    let args = ip.get_object_def_max_args(ALTERNATIVE).unwrap();
    assert_eq!(
        (args.alphas, args.numerics, args.total().unwrap()),
        (2, 9, 11)
    );
    let original = ip
        .source_raw_object_fields(ALTERNATIVE, " WRAPPED ")
        .unwrap();
    assert!(
        matches!(original.get("solar_index_of_refraction"),Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Number(token)) if token=="18446744073709551615")
            && original.contains_key("optical_data_type")
            && original.contains_key("idf_max_fields")
    );
    let projected = crate::ordinary_material_input::resolve_source(&alternative_raw, &ip).unwrap();
    assert_eq!(
        (
            projected.refraction.len(),
            projected.refraction[0].fields.len(),
            projected.refraction[1].fields.len()
        ),
        (2, 6, 7)
    );
    assert_eq!(
        [
            "thickness",
            "solar_index_of_refraction",
            "solar_extinction_coefficient",
            "visible_index_of_refraction",
            "visible_extinction_coefficient",
            "conductivity"
        ]
        .map(|field| projected.refraction[1].fields[field].clone()),
        [-1.0; 6].map(ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number)
    );
    assert!(
        !projected.refraction[1]
            .fields
            .contains_key("optical_data_type")
            && !projected.refraction[1]
                .fields
                .contains_key("young_s_modulus")
    );
    let mut stats = ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
    ip.report_source_idf_records_stats(&mut stats).unwrap();
    assert_eq!(
        usize::try_from(stats.number_input_records).unwrap(),
        alternative_raw
            .objects
            .values()
            .map(|rows| rows.len())
            .sum::<usize>()
    );
    assert!(
        ip.is_object_unused(ALTERNATIVE, " DEFAULT ")
            && ip.is_object_unused(ALTERNATIVE, " WRAPPED ")
    );
    let mut trace = inspect_porting_scope(
        &alternative_raw,
        &compiled.report,
        Some(model),
        PortingScope::B,
    );
    assert!(trace.admissible, "{:?}", trace.violations);
    trace.environment = Some(
        super::environment::environment_from_weather(
            model,
            &weather,
            "existing Refraction caller weather",
        )
        .unwrap(),
    );
    let admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&alternative_raw, &trace)
            .unwrap();
    assert!(
        crate::support::assess_support_with_source_admission(
            &alternative_raw,
            &compiled.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&admission)
        )
        .allows_rust_runtime()
    );
    assert!(
        crate::ordinary_equipment_input::resolve(&alternative_raw).is_err()
            && crate::ordinary_equipment_input::resolve_source_a(&alternative_raw).is_err()
            && !assess_support(
                &alternative_raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal
            )
            .allows_rust_runtime()
    );
    let mut models = Vec::new();
    let mut absent = model.clone();
    absent
        .materials
        .retain(|row| row.kind() != ep_model::MaterialKind::WindowGlazingRefractionExtinction);
    models.push(absent);
    let mut alias = model.clone();
    alias.material_names.insert("EXTRA ALTERNATIVE ALIAS", id);
    models.push(alias);
    let mut duplicate = model.clone();
    duplicate.material_names.insert_instance(" WRAPPED ", id);
    models.push(duplicate);
    let mut loader = model.clone();
    loader
        .ctf_report_names
        .materials
        .push((id, "EXTRA LOADER".into()));
    models.push(loader);
    let mut missing = model.clone();
    missing
        .ctf_report_names
        .materials
        .retain(|(key, _)| *key != id);
    models.push(missing);
    let mut wrong_id = model.clone();
    wrong_id
        .materials
        .iter_mut()
        .find(|row| row.id == id)
        .unwrap()
        .id = ep_model::MaterialId(u32::MAX);
    models.push(wrong_id);
    let mut wrong_name = model.clone();
    wrong_name
        .materials
        .iter_mut()
        .find(|row| row.id == id)
        .unwrap()
        .name = ep_model::NormalizedName::new("WRONG");
    models.push(wrong_name);
    let mut wrong_kind = model.clone();
    let average = wrong_kind
        .materials
        .iter()
        .find(|row| row.kind() == ep_model::MaterialKind::WindowGlazing)
        .unwrap()
        .definition;
    wrong_kind
        .materials
        .iter_mut()
        .find(|row| row.id == id)
        .unwrap()
        .definition = average;
    models.push(wrong_kind);
    let mut extra = model.clone();
    let mut row = extra
        .materials
        .iter()
        .find(|row| row.id == id)
        .unwrap()
        .clone();
    row.id = ep_model::MaterialId(u32::try_from(extra.materials.len()).unwrap());
    extra.materials.push(row);
    models.push(extra);
    let mut used = model.clone();
    used.constructions[0].layers.clear();
    used.constructions[0].outside_layer = Some(id);
    models.push(used);
    let mut layered = model.clone();
    let opaque = layered.constructions[0].outside_layer.unwrap();
    layered.constructions[0].layers = vec![opaque, id];
    models.push(layered);
    for model in models {
        assert!(
            !crate::support::assess_support_with_source_admission(
                &alternative_raw,
                &compiled.report,
                Some(&model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&admission)
            )
            .allows_rust_runtime()
        );
    }
    let mut empty_value = alternative_value.clone();
    empty_value.as_object_mut().unwrap().remove(ALTERNATIVE);
    let empty_raw = parse_epjson_str(&empty_value.to_string()).unwrap();
    let empty_compiled = ep_compiler::compile_raw_model_for_source(&empty_raw);
    let empty_model = empty_compiled.model.as_ref().unwrap();
    let mut empty_trace = inspect_porting_scope(
        &empty_raw,
        &empty_compiled.report,
        Some(empty_model),
        PortingScope::B,
    );
    empty_trace.environment = Some(
        super::environment::environment_from_weather(
            empty_model,
            &weather,
            "same noRefraction weather",
        )
        .unwrap(),
    );
    let empty_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&empty_raw, &empty_trace)
            .unwrap();
    assert!(
        !crate::support::assess_support_with_source_admission(
            &empty_raw,
            &empty_compiled.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&empty_admission)
        )
        .allows_rust_runtime()
    );
    for (field, value) in [
        ("thickness", json!(0.0)),
        ("solar_index_of_refraction", json!(1.0)),
        ("solar_extinction_coefficient", json!(null)),
        ("visible_index_of_refraction", json!("1.7")),
        ("visible_extinction_coefficient", json!(false)),
        ("infrared_transmittance_at_normal_incidence", json!(1.0)),
        ("infrared_hemispherical_emissivity", json!(0.0)),
        ("conductivity", json!(0.0)),
        (
            "dirt_correction_factor_for_solar_and_visible_transmittance",
            json!(1.1),
        ),
        ("solar_diffusing", json!("yes")),
        ("solar_diffusing", json!(" ")),
    ] {
        let mut bad = alternative_value.clone();
        bad[ALTERNATIVE][" DEFAULT "][field] = value;
        assert!(
            crate::ordinary_input_processor::resolve_source_b(
                &parse_epjson_str(&bad.to_string()).unwrap()
            )
            .is_err()
        );
    }
    let mut missing = alternative_value.clone();
    missing[ALTERNATIVE][" DEFAULT "]
        .as_object_mut()
        .unwrap()
        .remove("visible_extinction_coefficient");
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &parse_epjson_str(&missing.to_string()).unwrap()
        )
        .is_err()
    );
    alternative_value["Construction"]["Alternative Used"] = json!({"outside_layer":" WRAPPED "});
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &parse_epjson_str(&alternative_value.to_string()).unwrap()
        )
        .is_err()
    );
    // Coherent Source-only Dataset + unused Spectral Glass. The dataset names
    // are ordered instances: case-distinct map keys can share one uppercase Name.
    const DATASET: &str = "MaterialProperty:GlazingSpectralData";
    const SPECTRAL_GLASS: &str = "WindowMaterial:Glazing";
    let mut spectral_value = input(PortingScope::B);
    spectral_value[DATASET] = json!({
        "A Shared":{"wavelength_1":0.5,"transmittance_1":u64::MAX,"front_reflectance_1":0.2,"back_reflectance_1":0.3,
            "name":null,"idf_max_fields":false,"metadata":[null,1.25,{"x":true}]},
        "a shared":{"wavelength_1":0.6,"transmittance_1":0.7,"front_reflectance_1":0.1,"back_reflectance_1":0.1},
        "B EMPTY":{"extensions":[]},
        "C EXT":{"extensions":[{},{"unknown":[null,{"retained":true}]},{"wavelength":0.7,"transmittance":0.4,"front_reflectance":0.2,"back_reflectance":0.3}]}
    });
    spectral_value[SPECTRAL_GLASS] = json!({
        " LINK ":{"optical_data_type":"Spectral","thickness":0.003,"window_glass_spectral_data_set_name":"a ShArEd",
            "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name":"unresolved transmittance",
            "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name":"unresolved front",
            "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name":"unresolved back",
            "metadata":{"all":[null,true,[],{}]},"idf_order":{},"name":false},
        "AVERAGE NEIGHBOUR":{"optical_data_type":"SpectralAverage","thickness":0.003},
        "ZERO LINK":{"optical_data_type":"Spectral","thickness":0.003,"window_glass_spectral_data_set_name":"b empty"}
    });
    let spectral_raw =
        ep_raw_model::parse_epjson_str_for_source(&spectral_value.to_string()).unwrap();
    let compiled = ep_compiler::compile_raw_model_for_source(&spectral_raw);
    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
    let model = compiled.model.as_ref().unwrap();
    let first = model
        .glazing_spectral_data_names
        .resolve("a ShArEd")
        .unwrap();
    let glass_id = model.material_names.resolve(" LINK ").unwrap();
    let average_id = model.material_names.resolve("AVERAGE NEIGHBOUR").unwrap();
    let ordered = spectral_raw.source_ordered_instances(DATASET).unwrap();
    assert!(
        model
            .glazing_spectral_data_names
            .is_source_exact_namespace()
            && model
                .glazing_spectral_data_names
                .names()
                .iter()
                .map(|name| name.0.as_str())
                .collect::<Vec<_>>()
                == ["A SHARED", "B EMPTY", "C EXT", "A SHARED"]
            && ordered
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>()
                == ["A Shared", "B EMPTY", "C EXT", "a shared"]
            && first == model.glazing_spectral_data[0].id
            && model.glazing_spectral_data[3].id != first
    );
    assert!(
        matches!(model.materials.iter().find(|row|row.id==glass_id).unwrap().definition,
        ep_model::MaterialDefinition::WindowGlazingSpectral(value) if value.spectral_data==Some(first))
            && model
                .ctf_report_names
                .materials
                .contains(&(glass_id, " LINK ".into()))
    );
    let mut ip = crate::ordinary_input_processor::resolve_source_b(&spectral_raw).unwrap();
    let args = ip.get_object_def_max_args(DATASET).unwrap();
    assert_eq!(
        (args.alphas, args.numerics, args.total().unwrap()),
        (1, 28, 29)
    );
    let original = ip.source_raw_object_fields(DATASET, "A Shared").unwrap();
    assert!(
        matches!(original.get("transmittance_1"),Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Number(token)) if token=="18446744073709551615")
            && original.contains_key("name")
            && original.contains_key("idf_max_fields")
            && original.contains_key("metadata")
    );
    let projected = crate::ordinary_material_input::resolve_source(&spectral_raw, &ip).unwrap();
    let projected_first = projected
        .spectral_data
        .iter()
        .find(|row| row.object.object_name == "A Shared")
        .unwrap();
    let projected_empty = projected
        .spectral_data
        .iter()
        .find(|row| row.object.object_name == "B EMPTY")
        .unwrap();
    let projected_extensions = projected
        .spectral_data
        .iter()
        .find(|row| row.object.object_name == "C EXT")
        .unwrap();
    assert_eq!(
        (
            projected.spectral_data.len(),
            projected.glazing.len(),
            projected_extensions.extensions.len()
        ),
        (4, 3, 3)
    );
    assert!(
        projected_first.object.fields["transmittance_1"]
            == ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(-1.0)
            && !projected_first.object.fields.contains_key("metadata")
            && projected_empty.extensions.is_empty()
            && projected_extensions.extensions[0].is_empty()
            && projected_extensions.extensions[1].is_empty()
            && projected_extensions.extensions[2].len() == 4
            && projected.glazing[0].fields.contains_key(
                "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name"
            )
    );
    let mut stats = ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
    ip.report_source_idf_records_stats(&mut stats).unwrap();
    assert_eq!(
        usize::try_from(stats.number_input_records).unwrap(),
        spectral_raw
            .objects
            .values()
            .map(|rows| rows.len())
            .sum::<usize>()
    );
    assert!(
        ip.is_object_unused(DATASET, "A Shared")
            && ip.is_object_unused(DATASET, "a shared")
            && ip.is_object_unused(SPECTRAL_GLASS, " LINK ")
    );
    let mut trace = inspect_porting_scope(
        &spectral_raw,
        &compiled.report,
        Some(model),
        PortingScope::B,
    );
    assert!(trace.admissible, "{:?}", trace.violations);
    trace.environment = Some(
        super::environment::environment_from_weather(
            model,
            &weather,
            "same Spectral caller weather",
        )
        .unwrap(),
    );
    let admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&spectral_raw, &trace)
            .unwrap();
    assert!(
        crate::support::assess_support_with_source_admission(
            &spectral_raw,
            &compiled.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&admission)
        )
        .allows_rust_runtime()
    );
    assert!(
        crate::ordinary_equipment_input::resolve(&spectral_raw).is_err()
            && crate::ordinary_equipment_input::resolve_source_a(&spectral_raw).is_err()
            && !assess_support(
                &spectral_raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal
            )
            .allows_rust_runtime()
    );

    let mut models = Vec::new();
    let mut absent = model.clone();
    absent.glazing_spectral_data.clear();
    models.push(absent);
    let mut absent_glass = model.clone();
    absent_glass
        .materials
        .retain(|row| row.kind() != ep_model::MaterialKind::WindowGlazing);
    models.push(absent_glass);
    let mut extra = model.clone();
    extra
        .glazing_spectral_data
        .push(extra.glazing_spectral_data[0].clone());
    models.push(extra);
    let mut wrong_id = model.clone();
    wrong_id.glazing_spectral_data[0].id = ep_model::GlazingSpectralDataId(u32::MAX);
    models.push(wrong_id);
    let mut wrong_name = model.clone();
    wrong_name.glazing_spectral_data[0].name = ep_model::NormalizedName::new("WRONG");
    models.push(wrong_name);
    let mut wrong_order = model.clone();
    wrong_order.glazing_spectral_data.swap(0, 1);
    models.push(wrong_order);
    let mut dataset_alias = model.clone();
    dataset_alias
        .glazing_spectral_data_names
        .insert("EXTRA DATA ALIAS", first);
    models.push(dataset_alias);
    let mut wrong_first = model.clone();
    wrong_first.glazing_spectral_data_names = ep_model::NameMap::source_exact_namespace();
    wrong_first
        .glazing_spectral_data
        .iter()
        .enumerate()
        .for_each(|(index, row)| {
            wrong_first.glazing_spectral_data_names.insert_instance(
                &row.name.0,
                if index == 0 {
                    ep_model::GlazingSpectralDataId(1)
                } else {
                    row.id
                },
            );
        });
    models.push(wrong_first);
    let mut wrong_link = model.clone();
    if let ep_model::MaterialDefinition::WindowGlazingSpectral(ref mut value) = wrong_link
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .definition
    {
        value.spectral_data = Some(ep_model::GlazingSpectralDataId(1));
    }
    models.push(wrong_link);
    let average = model
        .materials
        .iter()
        .find(|row| row.id == average_id)
        .unwrap()
        .definition;
    let spectral = model
        .materials
        .iter()
        .find(|row| row.id == glass_id)
        .unwrap()
        .definition;
    let mut wrong_mode = model.clone();
    wrong_mode
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .definition = average;
    models.push(wrong_mode);
    let mut wrong_average = model.clone();
    wrong_average
        .materials
        .iter_mut()
        .find(|row| row.id == average_id)
        .unwrap()
        .definition = spectral;
    models.push(wrong_average);
    let mut wrong_kind = model.clone();
    let opaque_definition = wrong_kind
        .materials
        .iter()
        .find(|row| row.kind() != ep_model::MaterialKind::WindowGlazing)
        .unwrap()
        .definition;
    wrong_kind
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .definition = opaque_definition;
    models.push(wrong_kind);
    let mut glass_alias = model.clone();
    glass_alias
        .material_names
        .insert("EXTRA GLASS ALIAS", glass_id);
    models.push(glass_alias);
    let mut duplicate_alias = model.clone();
    duplicate_alias
        .material_names
        .insert_instance(" LINK ", glass_id);
    models.push(duplicate_alias);
    let mut loader = model.clone();
    loader
        .ctf_report_names
        .materials
        .push((glass_id, "EXTRA LOADER".into()));
    models.push(loader);
    let mut missing_loader = model.clone();
    missing_loader
        .ctf_report_names
        .materials
        .retain(|(id, _)| *id != glass_id);
    models.push(missing_loader);
    let mut bad_material_id = model.clone();
    bad_material_id
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .id = ep_model::MaterialId(u32::MAX);
    models.push(bad_material_id);
    let mut bad_material_name = model.clone();
    bad_material_name
        .materials
        .iter_mut()
        .find(|row| row.id == glass_id)
        .unwrap()
        .name = ep_model::NormalizedName::new("WRONG GLASS");
    models.push(bad_material_name);
    let mut extra_glass = model.clone();
    let mut row = extra_glass
        .materials
        .iter()
        .find(|row| row.id == glass_id)
        .unwrap()
        .clone();
    row.id = ep_model::MaterialId(u32::try_from(extra_glass.materials.len()).unwrap());
    extra_glass.materials.push(row);
    models.push(extra_glass);
    let mut outside = model.clone();
    outside.constructions[0].layers.clear();
    outside.constructions[0].outside_layer = Some(glass_id);
    models.push(outside);
    let mut nonfirst = model.clone();
    let opaque = nonfirst.constructions[0].outside_layer.unwrap();
    nonfirst.constructions[0].layers = vec![opaque, glass_id];
    models.push(nonfirst);
    for model in models {
        assert!(
            !crate::support::assess_support_with_source_admission(
                &spectral_raw,
                &compiled.report,
                Some(&model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&admission)
            )
            .allows_rust_runtime()
        );
    }
    let mut empty_value = spectral_value.clone();
    empty_value.as_object_mut().unwrap().remove(DATASET);
    empty_value[SPECTRAL_GLASS]
        .as_object_mut()
        .unwrap()
        .remove(" LINK ");
    empty_value[SPECTRAL_GLASS]
        .as_object_mut()
        .unwrap()
        .remove("ZERO LINK");
    let empty_raw = ep_raw_model::parse_epjson_str_for_source(&empty_value.to_string()).unwrap();
    let empty_compiled = ep_compiler::compile_raw_model_for_source(&empty_raw);
    let empty_model = empty_compiled.model.as_ref().unwrap();
    let mut empty_trace = inspect_porting_scope(
        &empty_raw,
        &empty_compiled.report,
        Some(empty_model),
        PortingScope::B,
    );
    empty_trace.environment = Some(
        super::environment::environment_from_weather(
            empty_model,
            &weather,
            "same noDataset weather",
        )
        .unwrap(),
    );
    let empty_admission =
        crate::support::SourceSupportAdmission::from_validated_scope(&empty_raw, &empty_trace)
            .unwrap();
    assert!(
        !crate::support::assess_support_with_source_admission(
            &empty_raw,
            &empty_compiled.report,
            Some(model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&empty_admission)
        )
        .allows_rust_runtime()
    );

    // Dataset custody must stand on its own: no Glass registry blocker may mask it.
    let mut dataset_only_value = spectral_value.clone();
    dataset_only_value
        .as_object_mut()
        .unwrap()
        .remove(SPECTRAL_GLASS);
    let dataset_only_raw =
        ep_raw_model::parse_epjson_str_for_source(&dataset_only_value.to_string()).unwrap();
    let dataset_only_compiled = ep_compiler::compile_raw_model_for_source(&dataset_only_raw);
    let dataset_only_model = dataset_only_compiled.model.as_ref().unwrap();
    let mut dataset_only_trace = inspect_porting_scope(
        &dataset_only_raw,
        &dataset_only_compiled.report,
        Some(dataset_only_model),
        PortingScope::B,
    );
    dataset_only_trace.environment = Some(
        super::environment::environment_from_weather(
            dataset_only_model,
            &weather,
            "same dataset-only caller weather",
        )
        .unwrap(),
    );
    let dataset_only_admission = crate::support::SourceSupportAdmission::from_validated_scope(
        &dataset_only_raw,
        &dataset_only_trace,
    )
    .unwrap();
    assert!(
        crate::support::assess_support_with_source_admission(
            &dataset_only_raw,
            &dataset_only_compiled.report,
            Some(dataset_only_model),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&dataset_only_admission),
        )
        .allows_rust_runtime()
    );
    let mut dataset_only_models = Vec::new();
    let mut missing = dataset_only_model.clone();
    missing.glazing_spectral_data.clear();
    dataset_only_models.push(missing);
    let mut extra = dataset_only_model.clone();
    extra
        .glazing_spectral_data
        .push(extra.glazing_spectral_data[0].clone());
    dataset_only_models.push(extra);
    let mut wrong_id = dataset_only_model.clone();
    wrong_id.glazing_spectral_data[0].id = ep_model::GlazingSpectralDataId(u32::MAX);
    dataset_only_models.push(wrong_id);
    let mut wrong_name = dataset_only_model.clone();
    wrong_name.glazing_spectral_data[0].name = ep_model::NormalizedName::new("WRONG DATASET");
    dataset_only_models.push(wrong_name);
    let mut wrong_order = dataset_only_model.clone();
    wrong_order.glazing_spectral_data.swap(0, 1);
    dataset_only_models.push(wrong_order);
    let mut alias = dataset_only_model.clone();
    alias
        .glazing_spectral_data_names
        .insert("EXTRA DATASET ALIAS", first);
    dataset_only_models.push(alias);
    let mut wrong_first = dataset_only_model.clone();
    wrong_first.glazing_spectral_data_names = ep_model::NameMap::source_exact_namespace();
    wrong_first
        .glazing_spectral_data
        .iter()
        .enumerate()
        .for_each(|(index, row)| {
            wrong_first.glazing_spectral_data_names.insert_instance(
                &row.name.0,
                if index == 0 {
                    ep_model::GlazingSpectralDataId(1)
                } else {
                    row.id
                },
            );
        });
    dataset_only_models.push(wrong_first);
    for malformed in dataset_only_models {
        let support = crate::support::assess_support_with_source_admission(
            &dataset_only_raw,
            &dataset_only_compiled.report,
            Some(&malformed),
            RunMode::Compatibility,
            PartialRunPolicy::Deny,
            RunOutputFormat::RustNative,
            TraceLevel::Normal,
            Some(&dataset_only_admission),
        );
        assert!(
            !support.allows_rust_runtime()
                && support
                    .unsupported_objects
                    .iter()
                    .any(|row| row.object_type == DATASET)
                && support.diagnostics.diagnostics.iter().any(|row| row.code
                    == "UnsupportedGlazingSpectralDataSourceAdmission"
                    && row.blocking)
        );
    }
    // With no Raw or typed dataset rows, an extra namespace entry still breaks custody.
    let mut namespace_only = empty_model.clone();
    namespace_only
        .glazing_spectral_data_names
        .insert("UNBOUND DATASET", first);
    let support = crate::support::assess_support_with_source_admission(
        &empty_raw,
        &empty_compiled.report,
        Some(&namespace_only),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
        Some(&empty_admission),
    );
    assert!(
        !support.allows_rust_runtime()
            && support
                .unsupported_objects
                .iter()
                .any(|row| row.object_type == DATASET && row.count == 1)
            && support.diagnostics.diagnostics.iter().any(|row| row.code
                == "UnsupportedGlazingSpectralDataSourceAdmission"
                && row.blocking)
    );

    // Each invalid row changes only one originally valid Raw field/shape.
    for (family, name, field, value) in [
        (DATASET, "A Shared", "wavelength_1", json!(null)),
        (DATASET, "A Shared", "transmittance_1", json!("0.4")),
        (DATASET, "A Shared", "front_reflectance_1", json!(true)),
        (DATASET, "A Shared", "back_reflectance_1", json!([])),
        (DATASET, "C EXT", "extensions", json!({})),
        (DATASET, "C EXT", "extensions", json!(["wrong item"])),
        (DATASET, "C EXT", "extensions", json!([{"wavelength":null}])),
        (
            DATASET,
            "C EXT",
            "extensions",
            json!([{"transmittance":false}]),
        ),
        (
            DATASET,
            "C EXT",
            "extensions",
            json!([{"front_reflectance":[]}]),
        ),
        (
            DATASET,
            "C EXT",
            "extensions",
            json!([{"back_reflectance":{}}]),
        ),
        (
            SPECTRAL_GLASS,
            " LINK ",
            "optical_data_type",
            json!("spectral"),
        ),
        (SPECTRAL_GLASS, " LINK ", "optical_data_type", json!("BSDF")),
        (SPECTRAL_GLASS, " LINK ", "thickness", json!(0.0)),
        (
            SPECTRAL_GLASS,
            " LINK ",
            "window_glass_spectral_data_set_name",
            json!(false),
        ),
        (
            SPECTRAL_GLASS,
            " LINK ",
            "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name",
            json!({}),
        ),
    ] {
        let mut invalid = spectral_value.clone();
        invalid[family][name][field] = value;
        assert!(
            crate::ordinary_input_processor::resolve_source_b(
                &ep_raw_model::parse_epjson_str_for_source(&invalid.to_string()).unwrap()
            )
            .is_err()
        );
    }
    let mut missing = spectral_value.clone();
    missing[SPECTRAL_GLASS][" LINK "]
        .as_object_mut()
        .unwrap()
        .remove("thickness");
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &ep_raw_model::parse_epjson_str_for_source(&missing.to_string()).unwrap()
        )
        .is_err()
    );
    let mut used = spectral_value.clone();
    used["Construction"]["Spectral Used"] = json!({"outside_layer":" LINK "});
    assert!(
        crate::ordinary_equipment_input::resolve_source_b(
            &ep_raw_model::parse_epjson_str_for_source(&used.to_string()).unwrap()
        )
        .is_err()
    );
    let converted = json!({"Zone":{"Zone":{}},
        "MaterialProperty:GlazingSpectralData":spectral_value[DATASET].clone(),
        "WindowMaterial:Glazing":spectral_value[SPECTRAL_GLASS].clone()});
    let staged =
        ep_raw_model::parse_epjson_str_with_idf_order(&converted.to_string(), "Zone,Zone;")
            .unwrap();
    assert!(
        !staged.is_epjson() && crate::ordinary_equipment_input::resolve_source_b(&staged).is_err()
    );
    // Real Raw -> Source compiler/IP -> scope/weather -> source support. Curves
    // have no typed arena; no advisory numeric projection or duplicate filter.
    {
        use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
        let curve_objects: serde_json::Value = serde_json::from_str(
            r###"{
  "Curve:Biquadratic": {
    "CURVE UNUSED BIQUADRATIC": {
      "coefficient1_constant": 18446744073709551615,
      "coefficient2_x": 0.2,
      "coefficient3_x_2": 0.3,
      "coefficient4_y": 0.4,
      "coefficient5_y_2": 0.5,
      "coefficient6_x_y": 0.6,
      "minimum_value_of_x": -10.0,
      "maximum_value_of_x": 50.0,
      "minimum_value_of_y": -5.0,
      "maximum_value_of_y": 25.0
    }
  },
  "Curve:QuadraticLinear": {
    "CURVE UNUSED QUADRATIC LINEAR": {
      "coefficient1_constant": 0.1,
      "coefficient2_x": 0.2,
      "coefficient3_x_2": 0.3,
      "coefficient4_y": 0.4,
      "coefficient5_x_y": 0.5,
      "coefficient6_x_2_y": 0.6,
      "minimum_value_of_x": -10.0,
      "maximum_value_of_x": 50.0,
      "minimum_value_of_y": -5.0,
      "maximum_value_of_y": 25.0
    }
  },
  "Curve:CubicLinear": {
    "CURVE UNUSED CUBIC LINEAR": {
      "coefficient1_constant": 0.1,
      "coefficient2_x": 0.2,
      "coefficient3_x_2": 0.3,
      "coefficient4_x_3": 0.4,
      "coefficient5_y": 0.5,
      "coefficient6_x_y": 0.6,
      "minimum_value_of_x": -10.0,
      "maximum_value_of_x": 50.0,
      "minimum_value_of_y": -5.0,
      "maximum_value_of_y": 25.0
    }
  },
  "Curve:Bicubic": {
    "CURVE UNUSED BICUBIC": {
      "coefficient1_constant": 0.1,
      "coefficient2_x": 0.2,
      "coefficient3_x_2": 0.3,
      "coefficient4_y": 0.4,
      "coefficient5_y_2": 0.5,
      "coefficient6_x_y": 0.6,
      "coefficient7_x_3": 0.7,
      "coefficient8_y_3": 0.8,
      "coefficient9_x_2_y": 0.9,
      "coefficient10_x_y_2": 1.0,
      "minimum_value_of_x": -10.0,
      "maximum_value_of_x": 50.0,
      "minimum_value_of_y": -5.0,
      "maximum_value_of_y": 25.0
    }
  },
  "Curve:FanPressureRise": {
    "CURVE UNUSED FAN PRESSURE RISE": {
      "coefficient1_c1": 0.1,
      "coefficient2_c2": 0.2,
      "coefficient3_c3": 0.3,
      "coefficient4_c4": 0.4,
      "minimum_value_of_qfan": 0.1,
      "maximum_value_of_qfan": 2.0,
      "minimum_value_of_psm": 50.0,
      "maximum_value_of_psm": 500.0
    }
  }
}"###,
        )
        .unwrap();
        let mut curve_value = input(PortingScope::B);
        for (family, objects) in curve_objects.as_object().unwrap() {
            curve_value[family] = objects.clone();
        }
        let standard = curve_value["Curve:Biquadratic"]["CURVE UNUSED BIQUADRATIC"].clone();
        curve_value["Curve:Biquadratic"] = json!({"B SECOND":standard.clone(),"a first":standard.clone(),"\u{000b}":standard.clone(),"\u{00a0}":standard.clone()});
        curve_value["Curve:Biquadratic"]["a first"]["coefficient1_constant"] = json!(0.1);
        curve_value["Curve:Biquadratic"]["B SECOND"]["name"] = json!(false);
        curve_value["Curve:Biquadratic"]["B SECOND"]["idf_order"] = json!([null, "ignored"]);
        curve_value["Curve:Biquadratic"]["B SECOND"]["idf_max_fields"] = json!({"ignored":true});
        curve_value["Curve:Biquadratic"]["B SECOND"]["idf_max_extensible_fields"] = json!(false);
        curve_value["Curve:Biquadratic"]["B SECOND"]["extra"] =
            json!({"retained":[null,true,1.25,-1.25,{"large":u64::MAX}]});
        curve_value["DesignSpecification:OutdoorAir"] = json!({"CURVE DSOA":{
            "outdoor_air_method":"Flow/Zone", "outdoor_air_flow_per_zone":0.01
        }});
        curve_value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["design_specification_outdoor_air_object_name"] =
            json!("CURVE DSOA");
        curve_value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["outdoor_air_inlet_node_name"] =
            json!("CURVE OA");
        curve_value["OutdoorAir:Node"] = json!({"CURVE OA":{"wind_pressure_coefficient_curve_name":"B SECOND","symmetric_wind_pressure_coefficient_curve":"Yes","wind_angle_type":"Relative"}});
        let raw = ep_raw_model::parse_epjson_str_for_source(&curve_value.to_string()).unwrap();
        let compiled = ep_compiler::compile_raw_model_for_source(&raw);
        let model = compiled.model.as_ref().unwrap();
        let mut ip = crate::ordinary_input_processor::resolve_source_b(&raw).unwrap();
        assert_eq!(
            ip.source_raw_object_names("Curve:Biquadratic")
                .unwrap()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["\u{00a0}", "\u{000b}", "B SECOND", "a first"]
        );
        assert!(
            matches!(ip.source_raw_object_fields("Curve:Biquadratic","B SECOND").unwrap().get("coefficient1_constant"),Some(S::Number(token))if token=="18446744073709551615")
                && matches!(
                    ip.source_raw_object_fields("Curve:Biquadratic", "B SECOND")
                        .unwrap()
                        .get("name"),
                    Some(S::Bool(false))
                )
                && matches!(
                    ip.source_raw_object_fields("Curve:Biquadratic", "B SECOND")
                        .unwrap()
                        .get("idf_order"),
                    Some(S::Array(_))
                )
                && matches!(
                    ip.source_raw_object_fields("Curve:Biquadratic", "B SECOND")
                        .unwrap()
                        .get("extra"),
                    Some(S::Object(_))
                )
        );
        assert_eq!(ip.get_num_objects_found("Curve:Biquadratic"), 4);
        assert_eq!(
            (
                ip.get_object_def_max_args("Curve:Bicubic").unwrap().alphas,
                ip.get_object_def_max_args("Curve:Bicubic")
                    .unwrap()
                    .numerics
            ),
            (4, 16)
        );
        let mut trace = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::B);
        assert!(trace.admissible, "{:?}", trace.violations);
        trace.environment = Some(
            super::environment::environment_from_weather(
                model,
                &weather,
                "same unused five-Curve weather",
            )
            .unwrap(),
        );
        let admission =
            crate::support::SourceSupportAdmission::from_validated_scope(&raw, &trace).unwrap();
        assert!(
            crate::support::assess_support_with_source_admission(
                &raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                Some(&admission)
            )
            .allows_rust_runtime()
        );
        assert!(
            !crate::support::assess_support_with_source_admission(
                &raw,
                &compiled.report,
                Some(model),
                RunMode::Compatibility,
                PartialRunPolicy::Deny,
                RunOutputFormat::RustNative,
                TraceLevel::Normal,
                None
            )
            .allows_rust_runtime()
        );
        let mut diagnostic = curve_value.clone();
        diagnostic["Curve:Biquadratic"]["B SECOND"]["minimum_value_of_x"] = json!(2.0);
        diagnostic["Curve:Biquadratic"]["B SECOND"]["maximum_value_of_x"] = json!(1.0);
        diagnostic["Curve:QuadraticLinear"]["b second"] =
            diagnostic["Curve:QuadraticLinear"]["CURVE UNUSED QUADRATIC LINEAR"].clone();
        assert!(
            crate::ordinary_input_processor::resolve_source_b(
                &ep_raw_model::parse_epjson_str_for_source(&diagnostic.to_string()).unwrap()
            )
            .is_ok(),
            "Native getter owns duplicate/range diagnostics"
        );
        assert!(
            crate::ordinary_equipment_input::resolve_source_a(&raw).is_err()
                && crate::ordinary_equipment_input::resolve(&raw).is_err()
        );
        let staged_value = json!({"Zone":{"Zone":{}},"Curve:Biquadratic":curve_value["Curve:Biquadratic"].clone()});
        let staged =
            ep_raw_model::parse_epjson_str_with_idf_order(&staged_value.to_string(), "Zone,Zone;")
                .unwrap();
        assert!(
            !staged.is_epjson()
                && crate::ordinary_equipment_input::resolve_source_b(&staged).is_err()
        );
        for (family, name, field, bad) in [
            (
                "Curve:Biquadratic",
                "B SECOND",
                "coefficient1_constant",
                json!("0.1"),
            ),
            (
                "Curve:QuadraticLinear",
                "CURVE UNUSED QUADRATIC LINEAR",
                "coefficient5_x_y",
                json!(null),
            ),
            (
                "Curve:CubicLinear",
                "CURVE UNUSED CUBIC LINEAR",
                "coefficient4_x_3",
                json!(false),
            ),
            (
                "Curve:Bicubic",
                "CURVE UNUSED BICUBIC",
                "coefficient10_x_y_2",
                json!([]),
            ),
            (
                "Curve:FanPressureRise",
                "CURVE UNUSED FAN PRESSURE RISE",
                "minimum_value_of_qfan",
                json!({}),
            ),
            (
                "Curve:Biquadratic",
                "B SECOND",
                "input_unit_type_for_x",
                json!("temperature"),
            ),
            (
                "Curve:CubicLinear",
                "CURVE UNUSED CUBIC LINEAR",
                "input_unit_type_for_y",
                json!("Pressure"),
            ),
            (
                "Curve:Bicubic",
                "CURVE UNUSED BICUBIC",
                "output_unit_type",
                json!(null),
            ),
        ] {
            let mut bad_value = curve_value.clone();
            bad_value[family][name][field] = bad;
            assert!(
                crate::ordinary_input_processor::resolve_source_b(
                    &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string()).unwrap()
                )
                .is_err()
            );
        }
        let mut missing = curve_value.clone();
        missing["Curve:Bicubic"]["CURVE UNUSED BICUBIC"]
            .as_object_mut()
            .unwrap()
            .remove("coefficient10_x_y_2");
        assert!(
            crate::ordinary_equipment_input::resolve_source_b(
                &ep_raw_model::parse_epjson_str_for_source(&missing.to_string()).unwrap()
            )
            .is_err()
        );
        let mut blank = curve_value.clone();
        blank["Curve:Biquadratic"][" \t\r\u{000c}"] = standard;
        assert!(
            crate::ordinary_equipment_input::resolve_source_b(
                &ep_raw_model::parse_epjson_str_for_source(&blank.to_string()).unwrap()
            )
            .is_err()
        );
        for (family, objects) in [
            ("AirflowNetwork:SimulationControl", json!({"held":{}})),
            ("Duct:Loss:Conduction", json!({"held":{}})),
            ("EnergyManagementSystem:Program", json!({"held":{}})),
            ("Table:Lookup", json!({"held":{}})),
            ("Curve:Linear", json!({"held":{}})),
            ("Curve:Functional:PressureDrop", json!({"held":{}})),
            (
                "WindowMaterial:Glazing",
                json!({"held":{"optical_data_type":"BSDF","thickness":0.003,"window_glass_spectral_and_incident_angle_transmittance_data_set_table_name":"B SECOND","window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name":"B SECOND","window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name":"B SECOND"}}),
            ),
        ] {
            let mut held = curve_value.clone();
            held[family] = objects;
            assert!(
                crate::ordinary_equipment_input::resolve_source_b(
                    &ep_raw_model::parse_epjson_str_for_source(&held.to_string()).unwrap()
                )
                .is_err()
            );
        }
        // New181 boundary: actual unused Angle is admitted; the old held matrix now names BSDF.
        {
            let mut angle_value = curve_value.clone();
            angle_value["Curve:Biquadratic"]["ANGLE LOOKUP"] =
                angle_value["Curve:Biquadratic"]["a first"].clone();
            angle_value["Curve:Biquadratic"]["ANGLE LOOKUP"]["minimum_value_of_x"] = json!(0.0);
            angle_value["Curve:Biquadratic"]["ANGLE LOOKUP"]["maximum_value_of_x"] = json!(90.0);
            angle_value["Curve:Biquadratic"]["ANGLE LOOKUP"]["minimum_value_of_y"] = json!(0.1);
            angle_value["Curve:Biquadratic"]["ANGLE LOOKUP"]["maximum_value_of_y"] = json!(4.0);
            angle_value["WindowMaterial:Glazing"] = json!({" ANGLE GLASS ":{
                "optical_data_type":"SpectralAndAngle","thickness":0.003,
                "solar_transmittance_at_normal_incidence":0.75,"front_side_solar_reflectance_at_normal_incidence":0.75,
                "visible_transmittance_at_normal_incidence":0.75,"infrared_transmittance_at_normal_incidence":0.75,
                "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name":"angle lookup",
                "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name":"ANGLE LOOKUP",
                "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name":"ANGLE LOOKUP",
                "name":false,"idf_max_fields":[{"retained":true}]
            }});
            let angle_raw =
                ep_raw_model::parse_epjson_str_for_source(&angle_value.to_string()).unwrap();
            let angle_compiled = ep_compiler::compile_raw_model_for_source(&angle_raw);
            assert!(
                !angle_compiled.has_errors(),
                "{:?}",
                angle_compiled.report.diagnostics
            );
            let angle_model = angle_compiled.model.as_ref().unwrap();
            let id = angle_model.material_names.resolve(" ANGLE GLASS ").unwrap();
            assert!(
                angle_model.material_names.is_source_exact_namespace()
                    && angle_model.material_names.resolve("ANGLE GLASS").is_none()
                    && angle_model.materials.iter().any(|row| row.id == id
                        && matches!(
                            row.definition,
                            ep_model::MaterialDefinition::WindowGlazingSpectralAndAngle(_)
                        ))
                    && angle_model
                        .ctf_report_names
                        .materials
                        .iter()
                        .any(|row| row == &(id, " ANGLE GLASS ".into()))
            );
            let angle_ip = crate::ordinary_input_processor::resolve_source_b(&angle_raw).unwrap();
            assert!(
                matches!(angle_ip.source_raw_object_fields("WindowMaterial:Glazing"," ANGLE GLASS ").unwrap().get("window_glass_spectral_and_incident_angle_transmittance_data_set_table_name"),Some(S::String(v))if v=="angle lookup")
            );
            assert!(
                matches!(
                    angle_ip
                        .source_raw_object_fields("WindowMaterial:Glazing", " ANGLE GLASS ")
                        .unwrap()
                        .get("name"),
                    Some(S::Bool(false))
                ) && matches!(
                    angle_ip
                        .source_raw_object_fields("WindowMaterial:Glazing", " ANGLE GLASS ")
                        .unwrap()
                        .get("idf_max_fields"),
                    Some(S::Array(_))
                )
            );
            let mut angle_trace = inspect_porting_scope(
                &angle_raw,
                &angle_compiled.report,
                Some(angle_model),
                PortingScope::B,
            );
            assert!(angle_trace.admissible, "{:?}", angle_trace.violations);
            angle_trace.environment = Some(
                super::environment::environment_from_weather(
                    angle_model,
                    &weather,
                    "same Angle Source weather",
                )
                .unwrap(),
            );
            let angle_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &angle_raw,
                &angle_trace,
            )
            .unwrap();
            assert!(
                crate::support::assess_support_with_source_admission(
                    &angle_raw,
                    &angle_compiled.report,
                    Some(angle_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&angle_admission)
                )
                .allows_rust_runtime()
            );
            assert!(
                !crate::support::assess_support_with_source_admission(
                    &angle_raw,
                    &angle_compiled.report,
                    Some(angle_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    None
                )
                .allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&angle_raw).is_err()
                    && crate::ordinary_equipment_input::resolve(&angle_raw).is_err()
            );
            for (field, bad) in [
                ("window_glass_spectral_data_set_name", json!(0)),
                (
                    "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name",
                    json!(false),
                ),
                (
                    "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name",
                    json!(null),
                ),
                (
                    "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name",
                    json!([]),
                ),
                ("thickness", json!("0.003")),
            ] {
                let mut bad_value = angle_value.clone();
                bad_value["WindowMaterial:Glazing"][" ANGLE GLASS "][field] = bad;
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string()).unwrap()
                    )
                    .is_err()
                );
            }
            let mut wrong_tag = angle_model.clone();
            let position = wrong_tag
                .materials
                .iter()
                .position(|row| row.id == id)
                .unwrap();
            wrong_tag.materials[position].definition = angle_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowGlazing)
                .unwrap()
                .definition;
            let mut missing_all = angle_model.clone();
            missing_all
                .materials
                .retain(|row| row.kind() != ep_model::MaterialKind::WindowGlazing);
            let mut wrong_id = angle_model.clone();
            wrong_id.materials[position].id = ep_model::MaterialId(u32::MAX);
            let mut loader_alias = angle_model.clone();
            loader_alias
                .ctf_report_names
                .materials
                .push((id, "ANGLE ALIAS".into()));
            assert!(
                [wrong_tag, missing_all, wrong_id, loader_alias]
                    .iter()
                    .all(
                        |model| !crate::support::assess_support_with_source_admission(
                            &angle_raw,
                            &angle_compiled.report,
                            Some(model),
                            RunMode::Compatibility,
                            PartialRunPolicy::Deny,
                            RunOutputFormat::RustNative,
                            TraceLevel::Normal,
                            Some(&angle_admission)
                        )
                        .allows_rust_runtime()
                    )
            );
            let mut used = angle_value.clone();
            used["Construction"]["ANGLE USED"] = json!({"outside_layer":" ANGLE GLASS "});
            assert!(
                crate::ordinary_equipment_input::resolve_source_b(
                    &ep_raw_model::parse_epjson_str_for_source(&used.to_string()).unwrap()
                )
                .is_err()
            );
            let mut staged = angle_value.clone();
            staged["WindowMaterial:Glazing"][" ANGLE GLASS "]["optical_data_type"] = json!("BSDF");
            assert!(
                crate::ordinary_equipment_input::resolve_source_b(
                    &ep_raw_model::parse_epjson_str_for_source(&staged.to_string()).unwrap()
                )
                .is_err()
            );
        }
        // Unused Shade follows the same real DSOA/Ideal/OA SourceB route.
        // Raw/IP retain unknown members; only the fourteen getter operands project.
        {
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            let mut shade_value = curve_value.clone();
            shade_value["WindowMaterial:Shade"] = json!({
                " A SHADE ": {
                    "solar_transmittance":0.15,"solar_reflectance":0.35,
                    "visible_transmittance":0.1,"visible_reflectance":0.4,
                    "infrared_hemispherical_emissivity":0.8,"infrared_transmittance":0.1,
                    "thickness":0.002,"conductivity":0.2,
                    "name":false,"idf_order":[],"metadata":null
                },
                " B SHADE ": {
                    "solar_transmittance":0.15,"solar_reflectance":0.35,
                    "visible_transmittance":0.1,"visible_reflectance":0.4,
                    "infrared_hemispherical_emissivity":0.8,"infrared_transmittance":0.1,
                    "thickness":0.004,"conductivity":u64::MAX,
                    "shade_to_glass_distance":0.001,"top_opening_multiplier":0.0,
                    "bottom_opening_multiplier":1.0,"left_side_opening_multiplier":0.25,
                    "right_side_opening_multiplier":0.75,"airflow_permeability":0.8,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":null,"extra_string":"unconsumed",
                    "metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            let shade_raw =
                ep_raw_model::parse_epjson_str_for_source(&shade_value.to_string()).unwrap();
            let shade_compiled = ep_compiler::compile_raw_model_for_source(&shade_raw);
            assert!(
                !shade_compiled.has_errors(),
                "{:?}",
                shade_compiled.report.diagnostics
            );
            let shade_model = shade_compiled.model.as_ref().unwrap();
            let shade_a = shade_model.material_names.resolve(" A SHADE ").unwrap();
            let shade_b = shade_model.material_names.resolve(" B SHADE ").unwrap();
            assert!(
                shade_a != shade_b
                    && shade_model.material_names.is_source_exact_namespace()
                    && shade_model.material_names.resolve("A SHADE").is_none()
                    && shade_model
                        .materials
                        .iter()
                        .filter(|row| row.kind() == ep_model::MaterialKind::WindowShade)
                        .count()
                        == 2
                    && shade_model.materials.iter().any(|row| row.id == shade_a
                        && row.name.0 == " A SHADE "
                        && matches!(row.definition, ep_model::MaterialDefinition::WindowShade(_)))
                    && shade_model.materials.iter().any(|row| row.id == shade_b
                        && row.name.0 == " B SHADE "
                        && matches!(row.definition, ep_model::MaterialDefinition::WindowShade(_)))
                    && shade_model
                        .ctf_report_names
                        .materials
                        .contains(&(shade_a, " A SHADE ".into()))
                    && shade_model
                        .ctf_report_names
                        .materials
                        .contains(&(shade_b, " B SHADE ".into()))
            );
            let mut shade_ip =
                crate::ordinary_input_processor::resolve_source_b(&shade_raw).unwrap();
            let shade_args = shade_ip
                .get_object_def_max_args("WindowMaterial:Shade")
                .unwrap();
            let mut shade_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            shade_ip
                .report_source_idf_records_stats(&mut shade_stats)
                .unwrap();
            assert!(
                (
                    shade_args.alphas,
                    shade_args.numerics,
                    shade_args.total().unwrap()
                ) == (1, 14, 15)
                    && shade_ip.get_num_objects_found("WindowMaterial:Shade") == 2
                    && shade_ip
                        .source_raw_object_names("WindowMaterial:Shade")
                        .unwrap()
                        .iter()
                        .map(String::as_str)
                        .eq([" A SHADE ", " B SHADE "])
                    && shade_ip.is_object_unused("WindowMaterial:Shade", " A SHADE ")
                    && shade_ip.is_object_unused("WindowMaterial:Shade", " B SHADE ")
                    && usize::try_from(shade_stats.number_input_records).unwrap()
                        == shade_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let shade_original = shade_ip
                .source_raw_object_fields("WindowMaterial:Shade", " B SHADE ")
                .unwrap();
            let shade_raw_fields = &shade_raw.objects
                [&ep_raw_model::ObjectType("WindowMaterial:Shade".into())]
                [&ep_raw_model::ObjectName(" B SHADE ".into())]
                .fields;
            assert!(
                matches!(shade_raw_fields.get(&ep_raw_model::FieldName("conductivity".into())),
                    Some(ep_raw_model::RawValue::Number(token)) if token == "18446744073709551615")
                    && matches!(shade_original.get("conductivity"),
                        Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(
                        ep_model::classify_source_json_number("18446744073709551615"),
                        Ok(ep_model::SourceJsonNumber::Integer(-1))
                    )
                    && matches!(shade_original.get("name"), Some(S::Bool(false)))
                    && matches!(shade_original.get("idf_order"), Some(S::Number(token))
                        if token == "18446744073709551615")
                    && matches!(shade_original.get("idf_max_fields"), Some(S::Object(_)))
                    && matches!(
                        shade_original.get("idf_max_extensible_fields"),
                        Some(S::Null)
                    )
                    && matches!(shade_original.get("extra_string"), Some(S::String(value))
                        if value == "unconsumed")
                    && matches!(shade_original.get("metadata"), Some(S::Object(fields))
                        if matches!(fields.get("retained"), Some(S::Array(rows)) if rows.len() == 5))
            );
            let shade_projection =
                crate::ordinary_material_input::resolve_source(&shade_raw, &shade_ip).unwrap();
            assert!(
                shade_projection.window_shade.len() == 2
                    && shade_projection.window_shade[0].object_name == " A SHADE "
                    && shade_projection.window_shade[0].fields.len() == 8
                    && shade_projection.window_shade[1].object_name == " B SHADE "
                    && shade_projection.window_shade[1].fields.len() == 14
                    && shade_projection
                        .window_shade
                        .iter()
                        .all(|row| row.idf_max_fields.is_none()
                            && row.fields.keys().all(|field| {
                                crate::ordinary_equipment_input::source_b_shade_known_field(field)
                            }))
                    && matches!(shade_projection.window_shade[1].fields.get("conductivity"),
                        Some(P::Number(value)) if value.to_bits() == (-1.0_f64).to_bits())
            );
            let mut shade_trace = inspect_porting_scope(
                &shade_raw,
                &shade_compiled.report,
                Some(shade_model),
                PortingScope::B,
            );
            assert!(shade_trace.admissible, "{:?}", shade_trace.violations);
            shade_trace.environment = Some(
                super::environment::environment_from_weather(
                    shade_model,
                    &weather,
                    "same Shade Source weather",
                )
                .unwrap(),
            );
            let shade_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &shade_raw,
                &shade_trace,
            )
            .unwrap();
            assert!(
                crate::support::assess_support_with_source_admission(
                    &shade_raw,
                    &shade_compiled.report,
                    Some(shade_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&shade_admission),
                )
                .allows_rust_runtime()
            );
            let shade_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                &json!({
                    "Zone":{"Zone":{}},
                    "WindowMaterial:Shade":{"IDF SHADE":{
                        "solar_transmittance":0.15,"solar_reflectance":0.35,
                        "visible_transmittance":0.1,"visible_reflectance":0.4,
                        "infrared_hemispherical_emissivity":0.8,"infrared_transmittance":0.1,
                        "thickness":0.002,"conductivity":0.2
                    }}
                })
                .to_string(),
                "Zone,Zone;WindowMaterial:Shade,IDF SHADE,0.15,0.35,0.1,0.4,0.8,0.1,0.002,0.2;",
            )
            .unwrap();
            let mut controlled_shade = shade_value.clone();
            controlled_shade["WindowShadingControl"] = json!({"SHADE CONTROL":{
                "shading_type":"InteriorShade","shading_device_material_name":" A SHADE "
            }});
            assert!(
                !crate::support::assess_support_with_source_admission(
                    &shade_raw,
                    &shade_compiled.report,
                    Some(shade_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    None,
                )
                .allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&shade_raw).is_err()
                    && crate::ordinary_equipment_input::resolve(&shade_raw).is_err()
                    && !shade_idf.is_epjson()
                    && crate::ordinary_equipment_input::resolve_source_b(&shade_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&controlled_shade.to_string())
                            .unwrap(),
                    )
                    .is_err()
            );
            for field in [
                "solar_transmittance",
                "solar_reflectance",
                "visible_transmittance",
                "visible_reflectance",
                "infrared_hemispherical_emissivity",
                "infrared_transmittance",
                "thickness",
                "conductivity",
                "shade_to_glass_distance",
                "top_opening_multiplier",
                "bottom_opening_multiplier",
                "left_side_opening_multiplier",
                "right_side_opening_multiplier",
                "airflow_permeability",
            ] {
                let mut bad_value = shade_value.clone();
                bad_value["WindowMaterial:Shade"][" B SHADE "][field] = json!(false);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string()).unwrap(),
                    )
                    .is_err(),
                    "wrong JSON kind for {field}"
                );
            }
            let shade_position = shade_model
                .materials
                .iter()
                .position(|row| row.id == shade_a)
                .unwrap();
            let opaque_material = shade_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowShade)
                .unwrap();
            for case in 0..12 {
                let mut bad_model = shade_model.clone();
                match case {
                    0 => bad_model
                        .materials
                        .retain(|row| row.kind() != ep_model::MaterialKind::WindowShade),
                    1 => {
                        let mut extra = bad_model.materials[shade_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA SHADE".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[shade_position].id = opaque_material.id,
                    3 => {
                        bad_model.materials[shade_position].definition = opaque_material.definition
                    }
                    4 => {
                        bad_model.materials[shade_position].name =
                            ep_model::NormalizedName("WRONG SHADE".into())
                    }
                    5 => bad_model.materials[shade_position].id = ep_model::MaterialId(u32::MAX),
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != shade_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((shade_a, " A SHADE ".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA SHADE ALIAS ", shade_a);
                    }
                    9 => {
                        // Break name-to-ID association, not a valid arena/tuple permutation.
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == shade_a)
                            .unwrap()
                            .1 = " B SHADE ".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == shade_b)
                            .unwrap()
                            .1 = " A SHADE ".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => bad_model
                        .material_names
                        .insert_instance(" A SHADE ", shade_a),
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &shade_raw,
                        &shade_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&shade_admission),
                    )
                    .allows_rust_runtime(),
                    "Shade custody case {case}"
                );
            }
            let mut used_shade_value = shade_value.clone();
            used_shade_value["Construction"]["SHADE USED"] =
                json!({"outside_layer":"Opaque","layer_2":" A SHADE "});
            assert!(
                crate::ordinary_equipment_input::resolve_source_b(
                    &ep_raw_model::parse_epjson_str_for_source(&used_shade_value.to_string())
                        .unwrap(),
                )
                .is_err()
            );
            let shade_construction = shade_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .unwrap();
            for case in 0..3 {
                let mut used_model = shade_model.clone();
                let construction = &mut used_model.constructions[shade_construction];
                match case {
                    // These two keep the real opaque outside layer and use a nonfirst Shade.
                    0 => construction.layers.push(shade_a),
                    1 => construction.layers.push(shade_b),
                    // Empty layers exercise the real outside-only effective_layers fallback.
                    _ => {
                        construction.layers.clear();
                        construction.outside_layer = Some(shade_a);
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &shade_raw,
                        &shade_compiled.report,
                        Some(&used_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&shade_admission),
                    )
                    .allows_rust_runtime(),
                    "active Shade layer case {case}"
                );
            }
        }
        // Unused Screen follows the same real DSOA/Ideal/OA SourceB route.
        // Raw/IP retain unknown members; only the thirteen consumed body keys project.
        {
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            let mut screen_value = curve_value.clone();
            screen_value["WindowMaterial:Screen"] = json!({
                " A SCREEN ": {
                    "diffuse_solar_reflectance":0.25,"diffuse_visible_reflectance":0.5,
                    "screen_material_spacing":0.004,"screen_material_diameter":0.001,
                    "name":false,"idf_order":[],"metadata":null
                },
                " B SCREEN ": {
                    "reflected_beam_transmittance_accounting_method":"ModelAsDirectBeam",
                    "diffuse_solar_reflectance":0.25,"diffuse_visible_reflectance":0.5,
                    "thermal_hemispherical_emissivity":0.75,"conductivity":u64::MAX,
                    "screen_material_spacing":0.004,"screen_material_diameter":0.001,
                    "screen_to_glass_distance":0.04,"top_opening_multiplier":0.1,
                    "bottom_opening_multiplier":0.2,"left_side_opening_multiplier":0.3,
                    "right_side_opening_multiplier":0.4,
                    "angle_of_resolution_for_screen_transmittance_output_map":5.0,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":null,"extra_string":"unconsumed",
                    "metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            let screen_raw =
                ep_raw_model::parse_epjson_str_for_source(&screen_value.to_string()).unwrap();
            let screen_compiled = ep_compiler::compile_raw_model_for_source(&screen_raw);
            assert!(
                !screen_compiled.has_errors(),
                "{:?}",
                screen_compiled.report.diagnostics
            );
            let screen_model = screen_compiled.model.as_ref().unwrap();
            let screen_a = screen_model.material_names.resolve(" A SCREEN ").unwrap();
            let screen_b = screen_model.material_names.resolve(" B SCREEN ").unwrap();
            assert!(
                screen_a != screen_b
                    && screen_model.material_names.is_source_exact_namespace()
                    && screen_model.material_names.resolve("A SCREEN").is_none()
                    && screen_model
                        .materials
                        .iter()
                        .filter(|row| row.kind() == ep_model::MaterialKind::WindowScreen)
                        .count()
                        == 2
                    && screen_model.materials.iter().any(|row| row.id == screen_a
                        && row.name.0 == " A SCREEN "
                        && matches!(
                            row.definition,
                            ep_model::MaterialDefinition::WindowScreen(_)
                        ))
                    && screen_model.materials.iter().any(|row| row.id == screen_b
                        && row.name.0 == " B SCREEN "
                        && matches!(
                            row.definition,
                            ep_model::MaterialDefinition::WindowScreen(_)
                        ))
                    && screen_model
                        .ctf_report_names
                        .materials
                        .contains(&(screen_a, " A SCREEN ".into()))
                    && screen_model
                        .ctf_report_names
                        .materials
                        .contains(&(screen_b, " B SCREEN ".into()))
            );
            let mut screen_ip =
                crate::ordinary_input_processor::resolve_source_b(&screen_raw).unwrap();
            let screen_args = screen_ip
                .get_object_def_max_args("WindowMaterial:Screen")
                .unwrap();
            let mut screen_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            screen_ip
                .report_source_idf_records_stats(&mut screen_stats)
                .unwrap();
            assert!(
                (
                    screen_args.alphas,
                    screen_args.numerics,
                    screen_args.total().unwrap()
                ) == (2, 12, 14)
                    && screen_ip.get_num_objects_found("WindowMaterial:Screen") == 2
                    && screen_ip
                        .source_raw_object_names("WindowMaterial:Screen")
                        .unwrap()
                        .iter()
                        .map(String::as_str)
                        .eq([" A SCREEN ", " B SCREEN "])
                    && screen_ip.is_object_unused("WindowMaterial:Screen", " A SCREEN ")
                    && screen_ip.is_object_unused("WindowMaterial:Screen", " B SCREEN ")
                    && usize::try_from(screen_stats.number_input_records).unwrap()
                        == screen_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let screen_original = screen_ip
                .source_raw_object_fields("WindowMaterial:Screen", " B SCREEN ")
                .unwrap();
            let screen_raw_fields = &screen_raw.objects
                [&ep_raw_model::ObjectType("WindowMaterial:Screen".into())]
                [&ep_raw_model::ObjectName(" B SCREEN ".into())]
                .fields;
            assert!(
                matches!(screen_raw_fields.get(&ep_raw_model::FieldName("conductivity".into())),
                    Some(ep_raw_model::RawValue::Number(token)) if token == "18446744073709551615")
                    && matches!(screen_original.get("conductivity"),
                        Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(
                        ep_model::classify_source_json_number("18446744073709551615"),
                        Ok(ep_model::SourceJsonNumber::Integer(-1))
                    )
                    && matches!(screen_original.get("name"), Some(S::Bool(false)))
                    && matches!(screen_original.get("idf_order"), Some(S::Number(token))
                        if token == "18446744073709551615")
                    && matches!(screen_original.get("idf_max_fields"), Some(S::Object(_)))
                    && matches!(
                        screen_original.get("idf_max_extensible_fields"),
                        Some(S::Null)
                    )
                    && matches!(screen_original.get("extra_string"), Some(S::String(value))
                        if value == "unconsumed")
                    && matches!(screen_original.get("metadata"), Some(S::Object(fields))
                        if matches!(fields.get("retained"), Some(S::Array(rows)) if rows.len() == 5))
            );
            let screen_projection =
                crate::ordinary_material_input::resolve_source(&screen_raw, &screen_ip).unwrap();
            assert!(
                screen_projection.window_screen.len() == 2
                    && screen_projection.window_screen[0].object_name == " A SCREEN "
                    && screen_projection.window_screen[0].fields.len() == 4
                    && screen_projection.window_screen[1].object_name == " B SCREEN "
                    && screen_projection.window_screen[1].fields.len() == 13
                    && screen_projection.window_screen.iter().all(|row| row
                        .idf_max_fields
                        .is_none()
                        && row.fields.keys().all(|field| {
                            crate::ordinary_equipment_input::source_b_screen_known_field(field)
                        }))
                    && matches!(screen_projection.window_screen[1].fields.get("conductivity"),
                        Some(P::Number(value)) if value.to_bits() == (-1.0_f64).to_bits())
            );
            let mut screen_trace = inspect_porting_scope(
                &screen_raw,
                &screen_compiled.report,
                Some(screen_model),
                PortingScope::B,
            );
            assert!(screen_trace.admissible, "{:?}", screen_trace.violations);
            screen_trace.environment = Some(
                super::environment::environment_from_weather(
                    screen_model,
                    &weather,
                    "same Screen Source weather",
                )
                .unwrap(),
            );
            let screen_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &screen_raw,
                &screen_trace,
            )
            .unwrap();
            assert!(
                crate::support::assess_support_with_source_admission(
                    &screen_raw,
                    &screen_compiled.report,
                    Some(screen_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&screen_admission),
                )
                .allows_rust_runtime()
            );
            let screen_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                &json!({
                    "Zone":{"Zone":{}},
                    "WindowMaterial:Screen":{"IDF SCREEN":{
                        "reflected_beam_transmittance_accounting_method":"ModelAsDiffuse",
                        "diffuse_solar_reflectance":0.25,"diffuse_visible_reflectance":0.5,
                        "thermal_hemispherical_emissivity":0.75,"conductivity":0.25,
                        "screen_material_spacing":0.004,"screen_material_diameter":0.001,
                        "screen_to_glass_distance":0.04
                    }}
                }).to_string(),
                "Zone,Zone;WindowMaterial:Screen,IDF SCREEN,ModelAsDiffuse,0.25,0.5,0.75,0.25,0.004,0.001,0.04;",
            ).unwrap();
            let mut controlled_screen = screen_value.clone();
            controlled_screen["WindowShadingControl"] = json!({"SCREEN CONTROL":{
                "shading_type":"ExteriorScreen","shading_device_material_name":" A SCREEN "
            }});
            assert!(
                !crate::support::assess_support_with_source_admission(
                    &screen_raw,
                    &screen_compiled.report,
                    Some(screen_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    None,
                )
                .allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&screen_raw).is_err()
                    && crate::ordinary_equipment_input::resolve(&screen_raw).is_err()
                    && !screen_idf.is_epjson()
                    && crate::ordinary_equipment_input::resolve_source_b(&screen_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&controlled_screen.to_string())
                            .unwrap(),
                    )
                    .is_err()
            );
            for field in [
                "reflected_beam_transmittance_accounting_method",
                "diffuse_solar_reflectance",
                "diffuse_visible_reflectance",
                "thermal_hemispherical_emissivity",
                "conductivity",
                "screen_material_spacing",
                "screen_material_diameter",
                "screen_to_glass_distance",
                "top_opening_multiplier",
                "bottom_opening_multiplier",
                "left_side_opening_multiplier",
                "right_side_opening_multiplier",
                "angle_of_resolution_for_screen_transmittance_output_map",
            ] {
                let mut bad_value = screen_value.clone();
                bad_value["WindowMaterial:Screen"][" B SCREEN "][field] = json!(false);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string()).unwrap(),
                    )
                    .is_err(),
                    "wrong JSON kind for {field}"
                );
            }
            let screen_position = screen_model
                .materials
                .iter()
                .position(|row| row.id == screen_a)
                .unwrap();
            let opaque_material = screen_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowScreen)
                .unwrap();
            for case in 0..12 {
                let mut bad_model = screen_model.clone();
                match case {
                    0 => bad_model
                        .materials
                        .retain(|row| row.kind() != ep_model::MaterialKind::WindowScreen),
                    1 => {
                        let mut extra = bad_model.materials[screen_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA SCREEN".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[screen_position].id = opaque_material.id,
                    3 => {
                        bad_model.materials[screen_position].definition = opaque_material.definition
                    }
                    4 => {
                        bad_model.materials[screen_position].name =
                            ep_model::NormalizedName("WRONG SCREEN".into())
                    }
                    5 => bad_model.materials[screen_position].id = ep_model::MaterialId(u32::MAX),
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != screen_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((screen_a, " A SCREEN ".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA SCREEN ALIAS ", screen_a);
                    }
                    9 => {
                        // Break name-to-ID association, not a valid arena/tuple permutation.
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == screen_a)
                            .unwrap()
                            .1 = " B SCREEN ".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == screen_b)
                            .unwrap()
                            .1 = " A SCREEN ".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => bad_model
                        .material_names
                        .insert_instance(" A SCREEN ", screen_a),
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &screen_raw,
                        &screen_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&screen_admission),
                    )
                    .allows_rust_runtime(),
                    "Screen custody case {case}"
                );
            }
            let mut used_screen_value = screen_value.clone();
            used_screen_value["Construction"]["SCREEN USED"] =
                json!({"outside_layer":"Opaque","layer_2":" A SCREEN "});
            assert!(
                crate::ordinary_equipment_input::resolve_source_b(
                    &ep_raw_model::parse_epjson_str_for_source(&used_screen_value.to_string())
                        .unwrap(),
                )
                .is_err()
            );
            let screen_construction = screen_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .unwrap();
            for case in 0..3 {
                let mut used_model = screen_model.clone();
                let construction = &mut used_model.constructions[screen_construction];
                match case {
                    // These two keep the real opaque outside layer and use a nonfirst Screen.
                    0 => construction.layers.push(screen_a),
                    1 => construction.layers.push(screen_b),
                    // Empty layers exercise the real outside-only effective_layers fallback.
                    _ => {
                        construction.layers.clear();
                        construction.outside_layer = Some(screen_a);
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &screen_raw,
                        &screen_compiled.report,
                        Some(&used_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&screen_admission),
                    )
                    .allows_rust_runtime(),
                    "active Screen layer case {case}"
                );
            }
        }
        // Unused Blind follows the same real DSOA/Ideal/OA SourceB route.
        // Raw/IP retain unknown members; only the twenty-eight consumed body keys project.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            let mut blind_value = curve_value.clone();
            blind_value["WindowMaterial:Blind"] = json!({
                " A BLIND ": {
                    "slat_width":0.02,"slat_separation":0.02,
                    "front_side_slat_beam_solar_reflectance":0.2,
                    "back_side_slat_beam_solar_reflectance":0.3,
                    "front_side_slat_diffuse_solar_reflectance":0.2,
                    "back_side_slat_diffuse_solar_reflectance":0.3,
                    "slat_beam_visible_transmittance":0.0,
                    "name":false,"idf_order":[],"metadata":null
                },
                " B BLIND ": {
                    "slat_orientation":"Vertical","slat_width":0.02,"slat_separation":0.02,
                    "slat_thickness":0.00025,"slat_angle":45.0,"slat_conductivity":u64::MAX,
                    "slat_beam_solar_transmittance":0.1,
                    "front_side_slat_beam_solar_reflectance":0.2,
                    "back_side_slat_beam_solar_reflectance":0.3,
                    "slat_diffuse_solar_transmittance":0.1,
                    "front_side_slat_diffuse_solar_reflectance":0.2,
                    "back_side_slat_diffuse_solar_reflectance":0.3,
                    "slat_beam_visible_transmittance":0.1,
                    "front_side_slat_beam_visible_reflectance":0.2,
                    "back_side_slat_beam_visible_reflectance":0.3,
                    "slat_diffuse_visible_transmittance":0.1,
                    "front_side_slat_diffuse_visible_reflectance":0.2,
                    "back_side_slat_diffuse_visible_reflectance":0.3,
                    "slat_infrared_hemispherical_transmittance":0.1,
                    "front_side_slat_infrared_hemispherical_emissivity":0.7,
                    "back_side_slat_infrared_hemispherical_emissivity":0.6,
                    "blind_to_glass_distance":0.05,"blind_top_opening_multiplier":0.5,
                    "blind_bottom_opening_multiplier":0.0,"blind_left_side_opening_multiplier":0.5,
                    "blind_right_side_opening_multiplier":0.5,"minimum_slat_angle":0.0,"maximum_slat_angle":180.0,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":null,"extra_string":"unconsumed",
                    "metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            let blind_raw =
                ep_raw_model::parse_epjson_str_for_source(&blind_value.to_string()).unwrap();
            let blind_compiled = ep_compiler::compile_raw_model_for_source(&blind_raw);
            assert!(
                !blind_compiled.has_errors(),
                "{:?}",
                blind_compiled.report.diagnostics
            );
            let blind_model = blind_compiled.model.as_ref().unwrap();
            let blind_a = blind_model.material_names.resolve(" A BLIND ").unwrap();
            let blind_b = blind_model.material_names.resolve(" B BLIND ").unwrap();
            assert!(
                blind_a != blind_b
                    && blind_model.material_names.is_source_exact_namespace()
                    && blind_model.material_names.resolve("A BLIND").is_none()
                    && blind_model
                        .materials
                        .iter()
                        .filter(|row| row.kind() == ep_model::MaterialKind::WindowBlind)
                        .count()
                        == 2
                    && blind_model.materials.iter().any(|row| row.id == blind_a
                        && row.name.0 == " A BLIND "
                        && matches!(row.definition, ep_model::MaterialDefinition::WindowBlind(_)))
                    && blind_model.materials.iter().any(|row| row.id == blind_b
                        && row.name.0 == " B BLIND "
                        && matches!(row.definition, ep_model::MaterialDefinition::WindowBlind(_)))
                    && blind_model
                        .ctf_report_names
                        .materials
                        .contains(&(blind_a, " A BLIND ".into()))
                    && blind_model
                        .ctf_report_names
                        .materials
                        .contains(&(blind_b, " B BLIND ".into()))
            );
            let mut blind_ip =
                crate::ordinary_input_processor::resolve_source_b(&blind_raw).unwrap();
            let blind_args = blind_ip
                .get_object_def_max_args("WindowMaterial:Blind")
                .unwrap();
            let mut blind_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            blind_ip
                .report_source_idf_records_stats(&mut blind_stats)
                .unwrap();
            assert!(
                (
                    blind_args.alphas,
                    blind_args.numerics,
                    blind_args.total().unwrap()
                ) == (2, 27, 29)
                    && blind_ip.get_num_objects_found("WindowMaterial:Blind") == 2
                    && blind_ip
                        .source_raw_object_names("WindowMaterial:Blind")
                        .unwrap()
                        .iter()
                        .map(String::as_str)
                        .eq([" A BLIND ", " B BLIND "])
                    && blind_ip.is_object_unused("WindowMaterial:Blind", " A BLIND ")
                    && blind_ip.is_object_unused("WindowMaterial:Blind", " B BLIND ")
                    && usize::try_from(blind_stats.number_input_records).unwrap()
                        == blind_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let blind_original = blind_ip
                .source_raw_object_fields("WindowMaterial:Blind", " B BLIND ")
                .unwrap();
            let blind_raw_fields = &blind_raw.objects
                [&ep_raw_model::ObjectType("WindowMaterial:Blind".into())]
                [&ep_raw_model::ObjectName(" B BLIND ".into())]
                .fields;
            assert!(
                matches!(blind_raw_fields.get(&ep_raw_model::FieldName("slat_conductivity".into())),
                    Some(ep_raw_model::RawValue::Number(token)) if token == "18446744073709551615")
                    && matches!(blind_original.get("slat_conductivity"),
                        Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(
                        ep_model::classify_source_json_number("18446744073709551615"),
                        Ok(ep_model::SourceJsonNumber::Integer(-1))
                    )
                    && matches!(blind_original.get("name"), Some(S::Bool(false)))
                    && matches!(blind_original.get("idf_order"), Some(S::Number(token))
                        if token == "18446744073709551615")
                    && matches!(blind_original.get("idf_max_fields"), Some(S::Object(_)))
                    && matches!(
                        blind_original.get("idf_max_extensible_fields"),
                        Some(S::Null)
                    )
                    && matches!(blind_original.get("extra_string"), Some(S::String(value))
                        if value == "unconsumed")
                    && matches!(blind_original.get("metadata"), Some(S::Object(fields))
                        if matches!(fields.get("retained"), Some(S::Array(rows)) if rows.len() == 5))
            );
            let blind_projection =
                crate::ordinary_material_input::resolve_source(&blind_raw, &blind_ip).unwrap();
            assert!(
                blind_projection.window_blind.len() == 2
                    && blind_projection.window_blind[0].object_name == " A BLIND "
                    && blind_projection.window_blind[0].fields.len() == 7
                    && blind_projection.window_blind[1].object_name == " B BLIND "
                    && blind_projection.window_blind[1].fields.len() == 28
                    && blind_projection
                        .window_blind
                        .iter()
                        .all(|row| row.idf_max_fields.is_none()
                            && row.fields.keys().all(|field| {
                                crate::ordinary_equipment_input::source_b_blind_known_field(field)
                            }))
                    && matches!(blind_projection.window_blind[1].fields.get("slat_conductivity"),
                        Some(P::Number(value)) if value.to_bits() == (-1.0_f64).to_bits())
            );
            let mut blind_trace = inspect_porting_scope(
                &blind_raw,
                &blind_compiled.report,
                Some(blind_model),
                PortingScope::B,
            );
            assert!(blind_trace.admissible, "{:?}", blind_trace.violations);
            blind_trace.environment = Some(
                super::environment::environment_from_weather(
                    blind_model,
                    &weather,
                    "same Blind Source weather",
                )
                .unwrap(),
            );
            let blind_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &blind_raw,
                &blind_trace,
            )
            .unwrap();
            let mut permuted_blind_model = blind_model.clone();
            let a_position = permuted_blind_model
                .materials
                .iter()
                .position(|row| row.id == blind_a)
                .unwrap();
            let b_position = permuted_blind_model
                .materials
                .iter()
                .position(|row| row.id == blind_b)
                .unwrap();
            permuted_blind_model.materials.swap(a_position, b_position);
            assert!(
                crate::support::assess_support_with_source_admission(
                    &blind_raw,
                    &blind_compiled.report,
                    Some(blind_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&blind_admission),
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &blind_raw,
                        &blind_compiled.report,
                        Some(&permuted_blind_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&blind_admission)
                    )
                    .allows_rust_runtime()
            );
            let blind_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                &json!({"Zone":{"Zone":{}},"WindowMaterial:Blind":{"IDF BLIND":{
                    "slat_orientation":"Horizontal","slat_width":0.02,"slat_separation":0.02,
                    "slat_thickness":0.00025,"slat_angle":45.0,"slat_conductivity":221.0,
                    "slat_beam_solar_transmittance":0.1,
                    "front_side_slat_beam_solar_reflectance":0.2,"back_side_slat_beam_solar_reflectance":0.3,
                    "slat_diffuse_solar_transmittance":0.1,
                    "front_side_slat_diffuse_solar_reflectance":0.2,"back_side_slat_diffuse_solar_reflectance":0.3,
                    "slat_beam_visible_transmittance":0.1,
                    "front_side_slat_beam_visible_reflectance":0.2,"back_side_slat_beam_visible_reflectance":0.3,
                    "slat_diffuse_visible_transmittance":0.1,
                    "front_side_slat_diffuse_visible_reflectance":0.2,"back_side_slat_diffuse_visible_reflectance":0.3,
                    "slat_infrared_hemispherical_transmittance":0.1,
                    "front_side_slat_infrared_hemispherical_emissivity":0.7,"back_side_slat_infrared_hemispherical_emissivity":0.6,
                    "blind_to_glass_distance":0.05,"blind_top_opening_multiplier":0.5,
                    "blind_bottom_opening_multiplier":0.0,"blind_left_side_opening_multiplier":0.5,
                    "blind_right_side_opening_multiplier":0.5,"minimum_slat_angle":0.0,"maximum_slat_angle":180.0
                }}}).to_string(),
                "Zone,Zone;WindowMaterial:Blind,IDF BLIND,Horizontal,0.02,0.02,0.00025,45.0,221.0,0.1,0.2,0.3,0.1,0.2,0.3,0.1,0.2,0.3,0.1,0.2,0.3,0.1,0.7,0.6,0.05,0.5,0.0,0.5,0.5,0.0,180.0;",
            ).unwrap();
            let mut controlled_blind = blind_value.clone();
            controlled_blind["WindowShadingControl"] = json!({"BLIND CONTROL":{
                "shading_type":"ExteriorBlind","shading_device_material_name":" A BLIND "
            }});
            assert!(
                !crate::support::assess_support_with_source_admission(
                    &blind_raw,
                    &blind_compiled.report,
                    Some(blind_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    None,
                )
                .allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&blind_raw).is_err()
                    && crate::ordinary_equipment_input::resolve(&blind_raw).is_err()
                    && !blind_idf.is_epjson()
                    && crate::ordinary_equipment_input::resolve_source_b(&blind_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&controlled_blind.to_string())
                            .unwrap(),
                    )
                    .is_err()
            );
            for field in [
                "slat_orientation",
                "slat_width",
                "slat_separation",
                "slat_thickness",
                "slat_angle",
                "slat_conductivity",
                "slat_beam_solar_transmittance",
                "front_side_slat_beam_solar_reflectance",
                "back_side_slat_beam_solar_reflectance",
                "slat_diffuse_solar_transmittance",
                "front_side_slat_diffuse_solar_reflectance",
                "back_side_slat_diffuse_solar_reflectance",
                "slat_beam_visible_transmittance",
                "front_side_slat_beam_visible_reflectance",
                "back_side_slat_beam_visible_reflectance",
                "slat_diffuse_visible_transmittance",
                "front_side_slat_diffuse_visible_reflectance",
                "back_side_slat_diffuse_visible_reflectance",
                "slat_infrared_hemispherical_transmittance",
                "front_side_slat_infrared_hemispherical_emissivity",
                "back_side_slat_infrared_hemispherical_emissivity",
                "blind_to_glass_distance",
                "blind_top_opening_multiplier",
                "blind_bottom_opening_multiplier",
                "blind_left_side_opening_multiplier",
                "blind_right_side_opening_multiplier",
                "minimum_slat_angle",
                "maximum_slat_angle",
            ] {
                let mut bad_value = blind_value.clone();
                bad_value["WindowMaterial:Blind"][" B BLIND "][field] = json!(false);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string()).unwrap(),
                    )
                    .is_err(),
                    "wrong JSON kind for {field}"
                );
            }
            let blind_position = blind_model
                .materials
                .iter()
                .position(|row| row.id == blind_a)
                .unwrap();
            let opaque_material = blind_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowBlind)
                .unwrap();
            for case in 0..12 {
                let mut bad_model = blind_model.clone();
                match case {
                    0 => bad_model
                        .materials
                        .retain(|row| row.kind() != ep_model::MaterialKind::WindowBlind),
                    1 => {
                        let mut extra = bad_model.materials[blind_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA BLIND".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[blind_position].id = opaque_material.id,
                    3 => {
                        bad_model.materials[blind_position].definition = opaque_material.definition
                    }
                    4 => {
                        bad_model.materials[blind_position].name =
                            ep_model::NormalizedName("WRONG BLIND".into())
                    }
                    5 => bad_model.materials[blind_position].id = ep_model::MaterialId(u32::MAX),
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != blind_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((blind_a, " A BLIND ".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA BLIND ALIAS ", blind_a);
                    }
                    9 => {
                        // Break name-to-ID association, not a valid arena/tuple permutation.
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == blind_a)
                            .unwrap()
                            .1 = " B BLIND ".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == blind_b)
                            .unwrap()
                            .1 = " A BLIND ".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => bad_model
                        .material_names
                        .insert_instance(" A BLIND ", blind_a),
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &blind_raw,
                        &blind_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&blind_admission),
                    )
                    .allows_rust_runtime(),
                    "Blind custody case {case}"
                );
            }
            let blind_construction = blind_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .unwrap();
            for case in 0_usize..11 {
                let mut used_blind_value = blind_value.clone();
                let mut fields =
                    serde_json::Map::from_iter([("outside_layer".into(), json!("Opaque"))]);
                let layer = if case == 0 || case == 10 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", case + 1)
                };
                fields.insert(
                    layer,
                    json!(if case.is_multiple_of(2) {
                        " A BLIND "
                    } else {
                        " B BLIND "
                    }),
                );
                used_blind_value["Construction"]["BLIND USED"] = serde_json::Value::Object(fields);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&used_blind_value.to_string())
                            .unwrap()
                    )
                    .is_err(),
                    "Raw Blind layer case {case}"
                );
                let mut used_model = blind_model.clone();
                let construction = &mut used_model.constructions[blind_construction];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(blind_a);
                } else {
                    construction.layers.resize(case + 1, opaque_material.id);
                    construction.layers[case] = if case.is_multiple_of(2) {
                        blind_a
                    } else {
                        blind_b
                    };
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &blind_raw,
                        &blind_compiled.report,
                        Some(&used_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&blind_admission)
                    )
                    .allows_rust_runtime(),
                    "typed/effective Blind layer case {case}"
                );
            }
        }

        // Unused Shade:EQL uses the existing positive DSOA/Ideal/OA fixture, not a substitute scene.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            const EQL: &str = "WindowMaterial:Shade:EquivalentLayer";
            let mut eql_value = curve_value.clone();
            eql_value[EQL] = json!({
                " A SHADE EQL ":{
                    "front_side_shade_beam_diffuse_solar_transmittance":0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.15,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.2,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.25,
                    "name":false,"idf_order":[],"metadata":null
                },
                " B SHADE EQL ":{
                    "shade_beam_beam_solar_transmittance":0,
                    "front_side_shade_beam_diffuse_solar_transmittance":0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.15,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.2,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.25,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence":0.1,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence":0.2,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence":0.3,
                    "shade_material_infrared_transmittance":0.04,
                    "front_side_shade_material_infrared_emissivity":0.8,
                    "back_side_shade_material_infrared_emissivity":0.75,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":null,"extra_string":"unconsumed",
                    "metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            let eql_raw = ep_raw_model::parse_epjson_str_for_source(&eql_value.to_string())
                .expect("Source EQL epJSON");
            let eql_compiled = ep_compiler::compile_raw_model_for_source(&eql_raw);
            assert!(
                !eql_compiled.has_errors(),
                "{:?}",
                eql_compiled.report.diagnostics
            );
            let eql_model = eql_compiled
                .model
                .as_ref()
                .expect("two real Source EQL materials");
            let eql_a = eql_model
                .material_names
                .resolve(" A SHADE EQL ")
                .expect("untrimmed A");
            let eql_b = eql_model
                .material_names
                .resolve(" B SHADE EQL ")
                .expect("untrimmed B");
            assert!(
                eql_a != eql_b
                    && eql_model.material_names.is_source_exact_namespace()
                    && eql_model.material_names.resolve("A SHADE EQL").is_none()
                    && eql_model
                        .materials
                        .iter()
                        .filter(
                            |row| row.kind() == ep_model::MaterialKind::WindowShadeEquivalentLayer
                        )
                        .count()
                        == 2
                    && eql_model.materials.iter().any(|row| row.id == eql_a
                        && row.name.0 == " A SHADE EQL "
                        && matches!(
                            row.definition,
                            ep_model::MaterialDefinition::WindowShadeEquivalentLayer(_)
                        ))
                    && eql_model.materials.iter().any(|row| row.id == eql_b
                        && row.name.0 == " B SHADE EQL "
                        && matches!(
                            row.definition,
                            ep_model::MaterialDefinition::WindowShadeEquivalentLayer(_)
                        ))
                    && eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(eql_a, " A SHADE EQL ".into()))
                    && eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(eql_b, " B SHADE EQL ".into()))
            );
            let mut eql_ip = crate::ordinary_input_processor::resolve_source_b(&eql_raw)
                .expect("same real Raw/IP");
            let eql_args = eql_ip.get_object_def_max_args(EQL).expect("actual1A11N");
            let mut eql_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            eql_ip
                .report_source_idf_records_stats(&mut eql_stats)
                .expect("actual record statistics");
            assert!(
                (
                    eql_args.alphas,
                    eql_args.numerics,
                    eql_args.total().expect("bounded args")
                ) == (1, 11, 12)
                    && eql_ip.get_num_objects_found(EQL) == 2
                    && eql_ip
                        .source_raw_object_names(EQL)
                        .expect("actual ordered names")
                        .iter()
                        .map(String::as_str)
                        .eq([" A SHADE EQL ", " B SHADE EQL "])
                    && eql_ip.is_object_unused(EQL, " A SHADE EQL ")
                    && eql_ip.is_object_unused(EQL, " B SHADE EQL ")
                    && usize::try_from(eql_stats.number_input_records)
                        .expect("nonnegative records")
                        == eql_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let eql_original = eql_ip
                .source_raw_object_fields(EQL, " B SHADE EQL ")
                .expect("original field custody");
            let eql_raw_fields = &eql_raw.objects[&ep_raw_model::ObjectType(EQL.into())]
                [&ep_raw_model::ObjectName(" B SHADE EQL ".into())]
                .fields;
            assert!(
                matches!(eql_raw_fields.get(&ep_raw_model::FieldName("shade_beam_beam_solar_transmittance".into())),Some(ep_raw_model::RawValue::Number(token)) if token == "0")
                    && matches!(eql_original.get("shade_beam_beam_solar_transmittance"),Some(S::Number(token)) if token == "0")
                    && matches!(
                        ep_model::classify_source_json_number("0"),
                        Ok(ep_model::SourceJsonNumber::Integer(0))
                    )
                    && matches!(eql_original.get("name"), Some(S::Bool(false)))
                    && matches!(eql_original.get("idf_order"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(eql_original.get("idf_max_fields"), Some(S::Object(_)))
                    && matches!(eql_original.get("idf_max_extensible_fields"), Some(S::Null))
                    && matches!(eql_original.get("extra_string"),Some(S::String(value)) if value == "unconsumed")
                    && matches!(eql_original.get("metadata"),Some(S::Object(fields)) if matches!(fields.get("retained"),Some(S::Array(rows)) if rows.len() == 5))
            );
            let eql_projection = crate::ordinary_material_input::resolve_source(&eql_raw, &eql_ip)
                .expect("same Raw/IP material projection");
            let eql_keys = [
                "shade_beam_beam_solar_transmittance",
                "front_side_shade_beam_diffuse_solar_transmittance",
                "back_side_shade_beam_diffuse_solar_transmittance",
                "front_side_shade_beam_diffuse_solar_reflectance",
                "back_side_shade_beam_diffuse_solar_reflectance",
                "shade_beam_beam_visible_transmittance_at_normal_incidence",
                "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
                "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
                "shade_material_infrared_transmittance",
                "front_side_shade_material_infrared_emissivity",
                "back_side_shade_material_infrared_emissivity",
            ];
            let projected = &eql_projection.window_shade_equivalent_layer;
            assert!(
                eql_projection.source_is_epjson && projected.len() == 2
                    && projected[0].object_name == " A SHADE EQL " && projected[0].fields.len() == 4
                    && projected[1].object_name == " B SHADE EQL " && projected[1].fields.len() == 11
                    && projected.iter().all(|row| row.idf_max_fields.is_none()
                        && row.fields.keys().all(|field| crate::ordinary_equipment_input::source_b_shade_equivalent_layer_known_field(field)))
                    && eql_keys.iter().zip([0.0_f64,0.1,0.15,0.2,0.25,0.1,0.2,0.3,0.04,0.8,0.75])
                        .all(|(field,value)|matches!(projected[1].fields.get(*field),Some(P::Number(actual)) if actual.to_bits() == value.to_bits()))
            );
            let mut eql_trace = inspect_porting_scope(
                &eql_raw,
                &eql_compiled.report,
                Some(eql_model),
                PortingScope::B,
            );
            assert!(eql_trace.admissible, "{:?}", eql_trace.violations);
            eql_trace.environment = Some(
                super::environment::environment_from_weather(
                    eql_model,
                    &weather,
                    "same ShadeEQL Source weather",
                )
                .expect("actual Source environment"),
            );
            let eql_admission =
                crate::support::SourceSupportAdmission::from_validated_scope(&eql_raw, &eql_trace)
                    .expect("validated Source EQL scope");
            let mut permuted_eql_model = eql_model.clone();
            let eql_a_position = eql_model
                .materials
                .iter()
                .position(|row| row.id == eql_a)
                .expect("A arena row");
            let eql_b_position = eql_model
                .materials
                .iter()
                .position(|row| row.id == eql_b)
                .expect("B arena row");
            permuted_eql_model
                .materials
                .swap(eql_a_position, eql_b_position);
            assert!(
                crate::support::assess_support_with_source_admission(
                    &eql_raw,
                    &eql_compiled.report,
                    Some(eql_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&eql_admission)
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &eql_raw,
                        &eql_compiled.report,
                        Some(&permuted_eql_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&eql_admission)
                    )
                    .allows_rust_runtime()
            );
            let eql_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                &json!({"Zone":{"Zone":{}},"WindowMaterial:Shade:EquivalentLayer":{"IDF SHADE EQL":{
                    "shade_beam_beam_solar_transmittance":0.1,
                    "front_side_shade_beam_diffuse_solar_transmittance":0.1,
                    "back_side_shade_beam_diffuse_solar_transmittance":0.15,
                    "front_side_shade_beam_diffuse_solar_reflectance":0.2,
                    "back_side_shade_beam_diffuse_solar_reflectance":0.25,
                    "shade_beam_beam_visible_transmittance_at_normal_incidence":0.1,
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence":0.2,
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence":0.3,
                    "shade_material_infrared_transmittance":0.04,
                    "front_side_shade_material_infrared_emissivity":0.8,
                    "back_side_shade_material_infrared_emissivity":0.75
                }}}).to_string(),
                "Zone,Zone;WindowMaterial:Shade:EquivalentLayer,IDF SHADE EQL,0.1,0.1,0.15,0.2,0.25,0.1,0.2,0.3,0.04,0.8,0.75;",
            ).expect("genuinely matched twelve-operand IDF");
            let mut controlled_eql = eql_value.clone();
            controlled_eql["WindowShadingControl"] = json!({"EQL CONTROL":{"shading_type":"ExteriorShade","shading_device_material_name":" A SHADE EQL "}});
            let mut active_eql = eql_value.clone();
            active_eql["Construction:WindowEquivalentLayer"] =
                json!({"ACTIVE EQL":{"outside_layer":" A SHADE EQL "}});
            assert!(
                !crate::support::assess_support_with_source_admission(
                    &eql_raw,
                    &eql_compiled.report,
                    Some(eql_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    None
                )
                .allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&eql_raw).is_err()
                    && crate::ordinary_equipment_input::resolve(&eql_raw).is_err()
                    && !eql_idf.is_epjson()
                    && crate::ordinary_equipment_input::resolve_source_b(&eql_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&controlled_eql.to_string())
                            .expect("controlled EQL Raw")
                    )
                    .is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&active_eql.to_string())
                            .expect("active EQL Raw")
                    )
                    .is_err()
            );
            for field in eql_keys {
                let mut bad_value = eql_value.clone();
                bad_value[EQL][" B SHADE EQL "][field] = json!(false);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string())
                            .expect("wrong-kind Raw")
                    )
                    .is_err(),
                    "ShadeEQL known field {field}"
                );
            }
            let opaque_eql = eql_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowShadeEquivalentLayer)
                .expect("held opaque material");
            for case in 0..12 {
                let mut bad_model = eql_model.clone();
                match case {
                    0 => bad_model.materials.retain(|row| {
                        row.kind() != ep_model::MaterialKind::WindowShadeEquivalentLayer
                    }),
                    1 => {
                        let mut extra = bad_model.materials[eql_a_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA SHADE EQL".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[eql_a_position].id = opaque_eql.id,
                    3 => bad_model.materials[eql_a_position].definition = opaque_eql.definition,
                    4 => {
                        bad_model.materials[eql_a_position].name =
                            ep_model::NormalizedName("WRONG SHADE EQL".into())
                    }
                    5 => bad_model.materials[eql_a_position].id = ep_model::MaterialId(u32::MAX),
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != eql_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((eql_a, " A SHADE EQL ".into())),
                    8 => {
                        let _ = bad_model.material_names.insert(" EXTRA EQL ALIAS ", eql_a);
                    }
                    9 => {
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == eql_a)
                            .expect("A report tuple")
                            .1 = " B SHADE EQL ".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == eql_b)
                            .expect("B report tuple")
                            .1 = " A SHADE EQL ".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => bad_model
                        .material_names
                        .insert_instance(" A SHADE EQL ", eql_a),
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &eql_raw,
                        &eql_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&eql_admission)
                    )
                    .allows_rust_runtime(),
                    "ShadeEQL custody case {case}"
                );
            }
            let eql_construction = eql_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .expect("real opaque Construction");
            for case in 0_usize..11 {
                let mut used_value = eql_value.clone();
                let mut fields =
                    serde_json::Map::from_iter([("outside_layer".into(), json!("Opaque"))]);
                let key = if case == 0 || case == 10 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", case + 1)
                };
                fields.insert(
                    key,
                    json!(if case.is_multiple_of(2) {
                        " A SHADE EQL "
                    } else {
                        " B SHADE EQL "
                    }),
                );
                used_value["Construction"]["EQL USED"] = serde_json::Value::Object(fields);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&used_value.to_string())
                            .expect("original active EQL layer")
                    )
                    .is_err(),
                    "Raw EQL layer {case}"
                );
                let mut used_model = eql_model.clone();
                let construction = &mut used_model.constructions[eql_construction];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(eql_a);
                } else {
                    construction.layers.resize(case + 1, opaque_eql.id);
                    construction.layers[case] = if case.is_multiple_of(2) { eql_a } else { eql_b };
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &eql_raw,
                        &eql_compiled.report,
                        Some(&used_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&eql_admission)
                    )
                    .allows_rust_runtime(),
                    "typed/effective EQL layer {case}"
                );
            }
        }

        // Unused Drape:EQL uses the existing real positive DSOA/Ideal/OA fixture.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            const DRAPE: &str = "WindowMaterial:Drape:EquivalentLayer";
            let mut drape_value = curve_value.clone();
            drape_value[DRAPE] = json!({
                "\u{b}A DRAPE EQL\u{a0}":{
                    "front_side_drape_beam_diffuse_solar_transmittance":0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance":0.15,
                    "front_side_drape_beam_diffuse_solar_reflectance":0.2,
                    "back_side_drape_beam_diffuse_solar_reflectance":0.25,
                    "name":false,"idf_order":[],"metadata":null
                },
                " B DRAPE EQL ":{
                    "drape_beam_beam_solar_transmittance_at_normal_incidence":0,
                    "front_side_drape_beam_diffuse_solar_transmittance":0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance":0.15,
                    "front_side_drape_beam_diffuse_solar_reflectance":0.2,
                    "back_side_drape_beam_diffuse_solar_reflectance":0.25,
                    "drape_beam_beam_visible_transmittance":0.1,
                    "drape_beam_diffuse_visible_transmittance":0.2,
                    "drape_beam_diffuse_visible_reflectance":0.3,
                    "drape_material_infrared_transmittance":0.04,
                    "front_side_drape_material_infrared_emissivity":0.8,
                    "back_side_drape_material_infrared_emissivity":0.75,
                    "width_of_pleated_fabric":u64::MAX,"length_of_pleated_fabric":0.8,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":null,"extra_string":"unconsumed",
                    "metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            let drape_raw = ep_raw_model::parse_epjson_str_for_source(&drape_value.to_string())
                .expect("Source DRAPE epJSON");
            let drape_compiled = ep_compiler::compile_raw_model_for_source(&drape_raw);
            assert!(
                !drape_compiled.has_errors(),
                "{:?}",
                drape_compiled.report.diagnostics
            );
            let drape_model = drape_compiled
                .model
                .as_ref()
                .expect("two real Source DRAPE materials");
            let drape_a = drape_model
                .material_names
                .resolve("\u{b}A DRAPE EQL\u{a0}")
                .expect("untrimmed A");
            let drape_b = drape_model
                .material_names
                .resolve(" B DRAPE EQL ")
                .expect("untrimmed B");
            assert!(
                drape_a != drape_b
                    && drape_model.material_names.is_source_exact_namespace()
                    && drape_model.material_names.resolve("A DRAPE EQL").is_none()
                    && drape_model
                        .materials
                        .iter()
                        .filter(
                            |row| row.kind() == ep_model::MaterialKind::WindowDrapeEquivalentLayer
                        )
                        .count()
                        == 2
                    && drape_model.materials.iter().any(|row| row.id == drape_a
                        && row.name.0 == "\u{b}A DRAPE EQL\u{a0}"
                        && matches!(
                            row.definition,
                            ep_model::MaterialDefinition::WindowDrapeEquivalentLayer(_)
                        ))
                    && drape_model.materials.iter().any(|row| row.id == drape_b
                        && row.name.0 == " B DRAPE EQL "
                        && matches!(
                            row.definition,
                            ep_model::MaterialDefinition::WindowDrapeEquivalentLayer(_)
                        ))
                    && drape_model
                        .ctf_report_names
                        .materials
                        .contains(&(drape_a, "\u{b}A DRAPE EQL\u{a0}".into()))
                    && drape_model
                        .ctf_report_names
                        .materials
                        .contains(&(drape_b, " B DRAPE EQL ".into()))
            );
            let mut drape_ip = crate::ordinary_input_processor::resolve_source_b(&drape_raw)
                .expect("same real Raw/IP");
            let drape_args = drape_ip
                .get_object_def_max_args(DRAPE)
                .expect("actual1A13N");
            let mut drape_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            drape_ip
                .report_source_idf_records_stats(&mut drape_stats)
                .expect("actual record statistics");
            assert!(
                (
                    drape_args.alphas,
                    drape_args.numerics,
                    drape_args.total().expect("bounded args")
                ) == (1, 13, 14)
                    && drape_ip.get_num_objects_found(DRAPE) == 2
                    && drape_ip
                        .source_raw_object_names(DRAPE)
                        .expect("actual ordered names")
                        .iter()
                        .map(String::as_str)
                        .eq(["\u{b}A DRAPE EQL\u{a0}", " B DRAPE EQL "])
                    && drape_ip.is_object_unused(DRAPE, "\u{b}A DRAPE EQL\u{a0}")
                    && drape_ip.is_object_unused(DRAPE, " B DRAPE EQL ")
                    && usize::try_from(drape_stats.number_input_records)
                        .expect("nonnegative records")
                        == drape_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let drape_original = drape_ip
                .source_raw_object_fields(DRAPE, " B DRAPE EQL ")
                .expect("original field custody");
            let drape_raw_fields = &drape_raw.objects[&ep_raw_model::ObjectType(DRAPE.into())]
                [&ep_raw_model::ObjectName(" B DRAPE EQL ".into())]
                .fields;
            assert!(
                matches!(drape_raw_fields.get(&ep_raw_model::FieldName("drape_beam_beam_solar_transmittance_at_normal_incidence".into())),Some(ep_raw_model::RawValue::Number(token)) if token == "0")
                    && matches!(drape_original.get("drape_beam_beam_solar_transmittance_at_normal_incidence"),Some(S::Number(token)) if token == "0")
                    && matches!(
                        ep_model::classify_source_json_number("0"),
                        Ok(ep_model::SourceJsonNumber::Integer(0))
                    )
                    && matches!(drape_raw_fields.get(&ep_raw_model::FieldName("width_of_pleated_fabric".into())),Some(ep_raw_model::RawValue::Number(token)) if token == "18446744073709551615")
                    && matches!(drape_original.get("width_of_pleated_fabric"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(
                        ep_model::classify_source_json_number("18446744073709551615"),
                        Ok(ep_model::SourceJsonNumber::Integer(-1))
                    )
                    && matches!(drape_original.get("name"), Some(S::Bool(false)))
                    && matches!(drape_original.get("idf_order"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(drape_original.get("idf_max_fields"), Some(S::Object(_)))
                    && matches!(
                        drape_original.get("idf_max_extensible_fields"),
                        Some(S::Null)
                    )
                    && matches!(drape_original.get("extra_string"),Some(S::String(value)) if value == "unconsumed")
                    && matches!(drape_original.get("metadata"),Some(S::Object(fields)) if matches!(fields.get("retained"),Some(S::Array(rows)) if rows.len() == 5))
            );
            let drape_projection =
                crate::ordinary_material_input::resolve_source(&drape_raw, &drape_ip)
                    .expect("same Raw/IP material projection");
            let drape_keys = [
                "drape_beam_beam_solar_transmittance_at_normal_incidence",
                "front_side_drape_beam_diffuse_solar_transmittance",
                "back_side_drape_beam_diffuse_solar_transmittance",
                "front_side_drape_beam_diffuse_solar_reflectance",
                "back_side_drape_beam_diffuse_solar_reflectance",
                "drape_beam_beam_visible_transmittance",
                "drape_beam_diffuse_visible_transmittance",
                "drape_beam_diffuse_visible_reflectance",
                "drape_material_infrared_transmittance",
                "front_side_drape_material_infrared_emissivity",
                "back_side_drape_material_infrared_emissivity",
                "width_of_pleated_fabric",
                "length_of_pleated_fabric",
            ];
            let projected = &drape_projection.window_drape_equivalent_layer;
            let mut count_mismatch_raw = drape_raw.clone();
            count_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(DRAPE.into()))
                .expect("Drape family")
                .remove(&ep_raw_model::ObjectName("\u{b}A DRAPE EQL\u{a0}".into()));
            let mut identity_mismatch_raw = drape_raw.clone();
            let identity_rows = identity_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(DRAPE.into()))
                .expect("Drape family");
            let identity_row = identity_rows
                .remove(&ep_raw_model::ObjectName("\u{b}A DRAPE EQL\u{a0}".into()))
                .expect("existing Drape row");
            identity_rows.insert(
                ep_raw_model::ObjectName("WRONG DRAPE IP NAME".into()),
                identity_row,
            );
            assert!(
                drape_projection.source_is_epjson && projected.len() == 2
                    && crate::ordinary_material_input::resolve_source(&count_mismatch_raw,&drape_ip).is_err()
                    && crate::ordinary_material_input::resolve_source(&identity_mismatch_raw,&drape_ip).is_err()
                    && projected[0].object_name == "\u{b}A DRAPE EQL\u{a0}" && projected[0].fields.len() == 4
                    && projected[1].object_name == " B DRAPE EQL " && projected[1].fields.len() == 13
                    && projected.iter().all(|row| row.idf_max_fields.is_none()
                        && row.fields.keys().all(|field| crate::ordinary_equipment_input::source_b_drape_equivalent_layer_known_field(field)))
                    && drape_keys.iter().zip([0.0_f64,0.1,0.15,0.2,0.25,0.1,0.2,0.3,0.04,0.8,0.75,-1.0,0.8])
                        .all(|(field,value)|matches!(projected[1].fields.get(*field),Some(P::Number(actual)) if actual.to_bits() == value.to_bits()))
            );
            let mut drape_trace = inspect_porting_scope(
                &drape_raw,
                &drape_compiled.report,
                Some(drape_model),
                PortingScope::B,
            );
            assert!(drape_trace.admissible, "{:?}", drape_trace.violations);
            drape_trace.environment = Some(
                super::environment::environment_from_weather(
                    drape_model,
                    &weather,
                    "same DrapeEQL Source weather",
                )
                .expect("actual Source environment"),
            );
            let drape_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &drape_raw,
                &drape_trace,
            )
            .expect("validated Source DRAPE scope");
            let mut permuted_drape_model = drape_model.clone();
            let drape_a_position = drape_model
                .materials
                .iter()
                .position(|row| row.id == drape_a)
                .expect("A arena row");
            let drape_b_position = drape_model
                .materials
                .iter()
                .position(|row| row.id == drape_b)
                .expect("B arena row");
            permuted_drape_model
                .materials
                .swap(drape_a_position, drape_b_position);
            assert!(
                crate::support::assess_support_with_source_admission(
                    &drape_raw,
                    &drape_compiled.report,
                    Some(drape_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&drape_admission)
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &drape_raw,
                        &drape_compiled.report,
                        Some(&permuted_drape_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&drape_admission)
                    )
                    .allows_rust_runtime()
            );
            let drape_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                &json!({"Zone":{"Zone":{}},"WindowMaterial:Drape:EquivalentLayer":{"IDF DRAPE":{
                    "front_side_drape_beam_diffuse_solar_transmittance":0.1,
                    "back_side_drape_beam_diffuse_solar_transmittance":0.2,
                    "front_side_drape_beam_diffuse_solar_reflectance":0.3,
                    "back_side_drape_beam_diffuse_solar_reflectance":0.4
                }}})
                .to_string(),
                "Zone,Zone;WindowMaterial:Drape:EquivalentLayer,IDF DRAPE,,0.1,0.2,0.3,0.4;",
            )
            .expect("matched Zone+Drape IDF with blankN1 and four required numbers");
            let mut controlled_drape = drape_value.clone();
            controlled_drape["WindowShadingControl"] = json!({"DRAPE CONTROL":{"shading_type":"ExteriorShade","shading_device_material_name":"\u{b}A DRAPE EQL\u{a0}"}});
            let mut active_drape = drape_value.clone();
            active_drape["Construction:WindowEquivalentLayer"] =
                json!({"ACTIVE DRAPE":{"outside_layer":"\u{b}A DRAPE EQL\u{a0}"}});
            assert!(
                !crate::support::assess_support_with_source_admission(
                    &drape_raw,
                    &drape_compiled.report,
                    Some(drape_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    None
                )
                .allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&drape_raw).is_err()
                    && crate::ordinary_equipment_input::resolve(&drape_raw).is_err()
                    && !drape_idf.is_epjson()
                    && crate::ordinary_equipment_input::resolve_source_b(&drape_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&controlled_drape.to_string())
                            .expect("controlled DRAPE Raw")
                    )
                    .is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&active_drape.to_string())
                            .expect("active DRAPE Raw")
                    )
                    .is_err()
            );
            for field in drape_keys {
                let mut bad_value = drape_value.clone();
                bad_value[DRAPE][" B DRAPE EQL "][field] = json!(false);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string())
                            .expect("wrong-kind Raw")
                    )
                    .is_err(),
                    "DrapeEQL known field {field}"
                );
            }
            let opaque_drape = drape_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowDrapeEquivalentLayer)
                .expect("held opaque material");
            for case in 0_usize..13 {
                let mut bad_model = drape_model.clone();
                match case {
                    0 => bad_model.materials.retain(|row| {
                        row.kind() != ep_model::MaterialKind::WindowDrapeEquivalentLayer
                    }),
                    1 => {
                        let mut extra = bad_model.materials[drape_a_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA DRAPE EQL".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[drape_a_position].id = opaque_drape.id,
                    3 => bad_model.materials[drape_a_position].definition = opaque_drape.definition,
                    4 => {
                        bad_model.materials[drape_a_position].name =
                            ep_model::NormalizedName("WRONG DRAPE EQL".into())
                    }
                    5 => bad_model.materials[drape_a_position].id = ep_model::MaterialId(u32::MAX),
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != drape_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((drape_a, "\u{b}A DRAPE EQL\u{a0}".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA DRAPE ALIAS ", drape_a);
                    }
                    9 => {
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == drape_a)
                            .expect("A report tuple")
                            .1 = " B DRAPE EQL ".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == drape_b)
                            .expect("B report tuple")
                            .1 = "\u{b}A DRAPE EQL\u{a0}".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    11 => bad_model
                        .material_names
                        .insert_instance("\u{b}A DRAPE EQL\u{a0}", drape_a),
                    _ => {
                        bad_model.material_names = ep_model::NameMap::source_exact_namespace();
                        bad_model.materials.iter().for_each(|row| {
                            let id = if row.id == drape_a {
                                drape_b
                            } else if row.id == drape_b {
                                drape_a
                            } else {
                                row.id
                            };
                            let _ = bad_model.material_names.insert(&row.name.0, id);
                        });
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &drape_raw,
                        &drape_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&drape_admission)
                    )
                    .allows_rust_runtime(),
                    "DrapeEQL custody case {case}"
                );
            }
            let drape_construction = drape_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .expect("real opaque Construction");
            for case in 0_usize..11 {
                let mut used_value = drape_value.clone();
                let mut fields =
                    serde_json::Map::from_iter([("outside_layer".into(), json!("Opaque"))]);
                let key = if case == 0 || case == 10 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", case + 1)
                };
                fields.insert(
                    key,
                    json!(if case.is_multiple_of(2) {
                        "\u{b}A DRAPE EQL\u{a0}"
                    } else {
                        " b drape eql "
                    }),
                );
                used_value["Construction"]["DRAPE USED"] = serde_json::Value::Object(fields);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&used_value.to_string())
                            .expect("original active DRAPE layer")
                    )
                    .is_err(),
                    "Raw DRAPE layer {case}"
                );
                let mut used_model = drape_model.clone();
                let construction = &mut used_model.constructions[drape_construction];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(drape_a);
                } else {
                    construction.layers.resize(case + 1, opaque_drape.id);
                    construction.layers[case] = if case.is_multiple_of(2) {
                        drape_a
                    } else {
                        drape_b
                    };
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &drape_raw,
                        &drape_compiled.report,
                        Some(&used_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&drape_admission)
                    )
                    .allows_rust_runtime(),
                    "typed/effective DRAPE layer {case}"
                );
            }
        }

        // Unused Screen:EQL extends the existing real positive DSOA/Ideal/OA scene.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            const SCREEN_EQL: &str = "WindowMaterial:Screen:EquivalentLayer";
            let mut screen_eql_value = curve_value.clone();
            screen_eql_value[SCREEN_EQL] = json!({
                "\u{b}A SCREEN EQL\u{a0}":{
                    "screen_beam_beam_solar_transmittance":"",
                    "screen_beam_diffuse_solar_transmittance":0.2,
                    "screen_beam_diffuse_solar_reflectance":0.3,
                    "screen_beam_beam_visible_transmittance":0.05,
                    "screen_beam_diffuse_visible_transmittance":0.1,
                    "screen_beam_diffuse_visible_reflectance":0.2,
                    "name":false,"idf_order":[],"metadata":null
                },
                " B SCREEN EQL ":{
                    "screen_beam_beam_solar_transmittance":0,
                    "screen_beam_diffuse_solar_transmittance":0.2,
                    "screen_beam_diffuse_solar_reflectance":0.3,
                    "screen_beam_beam_visible_transmittance":0.05,
                    "screen_beam_diffuse_visible_transmittance":0.1,
                    "screen_beam_diffuse_visible_reflectance":0.2,
                    "screen_infrared_transmittance":0.9,
                    "screen_infrared_emissivity":0.9,
                    "screen_wire_spacing":u64::MAX,"screen_wire_diameter":u64::MAX,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":null,"extra_string":"unconsumed",
                    "metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            let screen_eql_raw =
                ep_raw_model::parse_epjson_str_for_source(&screen_eql_value.to_string())
                    .expect("Source ScreenEQL epJSON");
            let screen_eql_compiled = ep_compiler::compile_raw_model_for_source(&screen_eql_raw);
            assert!(
                !screen_eql_compiled.has_errors(),
                "{:?}",
                screen_eql_compiled.report.diagnostics
            );
            let screen_eql_model = screen_eql_compiled
                .model
                .as_ref()
                .expect("two Source ScreenEQL materials");
            let screen_eql_a = screen_eql_model
                .material_names
                .resolve("\u{b}A SCREEN EQL\u{a0}")
                .expect("untrimmed A");
            let screen_eql_b = screen_eql_model
                .material_names
                .resolve(" B SCREEN EQL ")
                .expect("untrimmed B");
            assert!(
                screen_eql_a != screen_eql_b
                    && screen_eql_model.material_names.is_source_exact_namespace()
                    && screen_eql_model
                        .material_names
                        .resolve("A SCREEN EQL")
                        .is_none()
                    && screen_eql_model
                        .materials
                        .iter()
                        .filter(
                            |row| row.kind() == ep_model::MaterialKind::WindowScreenEquivalentLayer
                        )
                        .count()
                        == 2
                    && screen_eql_model
                        .materials
                        .iter()
                        .any(|row| row.id == screen_eql_a
                            && row.name.0 == "\u{b}A SCREEN EQL\u{a0}"
                            && matches!(
                                row.definition,
                                ep_model::MaterialDefinition::WindowScreenEquivalentLayer(_)
                            ))
                    && screen_eql_model
                        .materials
                        .iter()
                        .any(|row| row.id == screen_eql_b
                            && row.name.0 == " B SCREEN EQL "
                            && matches!(
                                row.definition,
                                ep_model::MaterialDefinition::WindowScreenEquivalentLayer(_)
                            ))
                    && screen_eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(screen_eql_a, "\u{b}A SCREEN EQL\u{a0}".into()))
                    && screen_eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(screen_eql_b, " B SCREEN EQL ".into()))
            );
            let mut screen_eql_ip =
                crate::ordinary_input_processor::resolve_source_b(&screen_eql_raw)
                    .expect("real Raw/IP");
            let screen_eql_args = screen_eql_ip
                .get_object_def_max_args(SCREEN_EQL)
                .expect("actual1A10N");
            let mut screen_eql_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            screen_eql_ip
                .report_source_idf_records_stats(&mut screen_eql_stats)
                .expect("real record statistics");
            assert!(
                (
                    screen_eql_args.alphas,
                    screen_eql_args.numerics,
                    screen_eql_args.total().expect("bounded args")
                ) == (1, 10, 11)
                    && screen_eql_ip.get_num_objects_found(SCREEN_EQL) == 2
                    && screen_eql_ip
                        .source_raw_object_names(SCREEN_EQL)
                        .expect("ordered names")
                        .iter()
                        .map(String::as_str)
                        .eq(["\u{b}A SCREEN EQL\u{a0}", " B SCREEN EQL "])
                    && screen_eql_ip.is_object_unused(SCREEN_EQL, "\u{b}A SCREEN EQL\u{a0}")
                    && screen_eql_ip.is_object_unused(SCREEN_EQL, " B SCREEN EQL ")
                    && usize::try_from(screen_eql_stats.number_input_records)
                        .expect("nonnegative records")
                        == screen_eql_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let screen_eql_original = screen_eql_ip
                .source_raw_object_fields(SCREEN_EQL, " B SCREEN EQL ")
                .expect("original custody");
            let screen_eql_raw_fields = &screen_eql_raw.objects
                [&ep_raw_model::ObjectType(SCREEN_EQL.into())]
                [&ep_raw_model::ObjectName(" B SCREEN EQL ".into())]
                .fields;
            assert!(
                matches!(screen_eql_raw_fields.get(&ep_raw_model::FieldName("screen_beam_beam_solar_transmittance".into())),Some(ep_raw_model::RawValue::Number(token)) if token == "0")
                    && matches!(screen_eql_original.get("screen_beam_beam_solar_transmittance"),Some(S::Number(token)) if token == "0")
                    && matches!(ep_model::classify_source_json_number("0"),Ok(ep_model::SourceJsonNumber::Integer(0)))
                    && ["screen_wire_spacing","screen_wire_diameter"].iter().all(|field|
                        matches!(screen_eql_raw_fields.get(&ep_raw_model::FieldName((*field).into())),Some(ep_raw_model::RawValue::Number(token)) if token == "18446744073709551615")
                            && matches!(screen_eql_original.get(*field),Some(S::Number(token)) if token == "18446744073709551615"))
                    && matches!(ep_model::classify_source_json_number("18446744073709551615"),Ok(ep_model::SourceJsonNumber::Integer(-1)))
                    && matches!(screen_eql_original.get("name"),Some(S::Bool(false)))
                    && matches!(screen_eql_original.get("idf_order"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(screen_eql_original.get("idf_max_fields"),Some(S::Object(_)))
                    && matches!(screen_eql_original.get("idf_max_extensible_fields"),Some(S::Null))
                    && matches!(screen_eql_original.get("extra_string"),Some(S::String(text)) if text == "unconsumed")
                    && matches!(screen_eql_original.get("metadata"),Some(S::Object(fields)) if matches!(fields.get("retained"),Some(S::Array(rows)) if rows.len() == 5))
            );
            let screen_eql_projection =
                crate::ordinary_material_input::resolve_source(&screen_eql_raw, &screen_eql_ip)
                    .expect("same Raw/IP projection");
            let screen_eql_keys = [
                "screen_beam_beam_solar_transmittance",
                "screen_beam_diffuse_solar_transmittance",
                "screen_beam_diffuse_solar_reflectance",
                "screen_beam_beam_visible_transmittance",
                "screen_beam_diffuse_visible_transmittance",
                "screen_beam_diffuse_visible_reflectance",
                "screen_infrared_transmittance",
                "screen_infrared_emissivity",
                "screen_wire_spacing",
                "screen_wire_diameter",
            ];
            let projected = &screen_eql_projection.window_screen_equivalent_layer;
            let mut count_mismatch_raw = screen_eql_raw.clone();
            count_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(SCREEN_EQL.into()))
                .expect("EQL family")
                .remove(&ep_raw_model::ObjectName("\u{b}A SCREEN EQL\u{a0}".into()));
            let mut identity_mismatch_raw = screen_eql_raw.clone();
            let identity_rows = identity_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(SCREEN_EQL.into()))
                .expect("EQL family");
            let identity_row = identity_rows
                .remove(&ep_raw_model::ObjectName("\u{b}A SCREEN EQL\u{a0}".into()))
                .expect("existing row");
            identity_rows.insert(
                ep_raw_model::ObjectName("WRONG SCREEN EQL IP NAME".into()),
                identity_row,
            );
            let mut exact_auto_scene = screen_eql_value.clone();
            exact_auto_scene[SCREEN_EQL]["\u{b}A SCREEN EQL\u{a0}"]["screen_beam_beam_solar_transmittance"] =
                json!("Autocalculate");
            let exact_auto_raw =
                ep_raw_model::parse_epjson_str_for_source(&exact_auto_scene.to_string())
                    .expect("exact Auto Raw");
            let exact_auto_ip = crate::ordinary_input_processor::resolve_source_b(&exact_auto_raw)
                .expect("legal exact Auto admission");
            let exact_auto_projection =
                crate::ordinary_material_input::resolve_source(&exact_auto_raw, &exact_auto_ip)
                    .expect("exact Auto projection");
            assert!(
                screen_eql_projection.source_is_epjson && projected.len() == 2
                    && crate::ordinary_material_input::resolve_source(&count_mismatch_raw,&screen_eql_ip).is_err()
                    && crate::ordinary_material_input::resolve_source(&identity_mismatch_raw,&screen_eql_ip).is_err()
                    && projected[0].object_name == "\u{b}A SCREEN EQL\u{a0}" && projected[0].fields.len() == 6
                    && matches!(projected[0].fields.get("screen_beam_beam_solar_transmittance"),Some(P::String(text)) if text.is_empty())
                    && projected[1].object_name == " B SCREEN EQL " && projected[1].fields.len() == 10
                    && projected.iter().all(|row|row.idf_max_fields.is_none() && row.fields.keys().all(|field|
                        crate::ordinary_equipment_input::source_b_screen_equivalent_layer_known_field(field)))
                    && screen_eql_keys.iter().zip([0.0_f64,0.2,0.3,0.05,0.1,0.2,0.9,0.9,-1.0,-1.0])
                        .all(|(field,value)|matches!(projected[1].fields.get(*field),Some(P::Number(actual)) if actual.to_bits() == value.to_bits()))
                    && matches!(exact_auto_projection.window_screen_equivalent_layer[0].fields.get("screen_beam_beam_solar_transmittance"),Some(P::String(text)) if text == "Autocalculate")
            );
            let mut screen_eql_trace = inspect_porting_scope(
                &screen_eql_raw,
                &screen_eql_compiled.report,
                Some(screen_eql_model),
                PortingScope::B,
            );
            assert!(
                screen_eql_trace.admissible,
                "{:?}",
                screen_eql_trace.violations
            );
            screen_eql_trace.environment = Some(
                super::environment::environment_from_weather(
                    screen_eql_model,
                    &weather,
                    "same ScreenEQLEQL Source weather",
                )
                .expect("actual Source environment"),
            );
            let screen_eql_admission =
                crate::support::SourceSupportAdmission::from_validated_scope(
                    &screen_eql_raw,
                    &screen_eql_trace,
                )
                .expect("validated Source SCREEN_EQL scope");
            let mut permuted_screen_eql_model = screen_eql_model.clone();
            let screen_eql_a_position = screen_eql_model
                .materials
                .iter()
                .position(|row| row.id == screen_eql_a)
                .expect("A arena row");
            let screen_eql_b_position = screen_eql_model
                .materials
                .iter()
                .position(|row| row.id == screen_eql_b)
                .expect("B arena row");
            permuted_screen_eql_model
                .materials
                .swap(screen_eql_a_position, screen_eql_b_position);
            assert!(
                crate::support::assess_support_with_source_admission(
                    &screen_eql_raw,
                    &screen_eql_compiled.report,
                    Some(screen_eql_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&screen_eql_admission)
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &screen_eql_raw,
                        &screen_eql_compiled.report,
                        Some(&permuted_screen_eql_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&screen_eql_admission)
                    )
                    .allows_rust_runtime()
            );
            let screen_eql_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                &json!({"Zone":{"Zone":{}},"WindowMaterial:Screen:EquivalentLayer":{"IDF EQL SCREEN":{
                    "screen_beam_beam_solar_transmittance":"Autocalculate",
                    "screen_beam_diffuse_solar_transmittance":0.1,"screen_beam_diffuse_solar_reflectance":0.2,
                    "screen_beam_beam_visible_transmittance":0.3,"screen_beam_diffuse_visible_transmittance":0.4,
                    "screen_beam_diffuse_visible_reflectance":0.2,"screen_infrared_transmittance":0.02,
                    "screen_infrared_emissivity":0.93,"screen_wire_spacing":0.025,"screen_wire_diameter":0.005
                }}}).to_string(),
                "Zone,Zone;WindowMaterial:Screen:EquivalentLayer,IDF EQL SCREEN,Autocalculate,0.1,0.2,0.3,0.4,0.2,0.02,0.93,0.025,0.005;",
            ).expect("matched Zone+ScreenEQL IDF11operands");
            let mut controlled_screen_eql = screen_eql_value.clone();
            controlled_screen_eql["WindowShadingControl"] = json!({"EQL SCREEN CONTROL":{"shading_type":"ExteriorScreen","shading_device_material_name":"\u{b}A SCREEN EQL\u{a0}"}});
            let mut active_screen_eql = screen_eql_value.clone();
            active_screen_eql["Construction:WindowEquivalentLayer"] =
                json!({"ACTIVE EQL SCREEN":{"outside_layer":"\u{b}A SCREEN EQL\u{a0}"}});
            let mut ems_screen_eql = screen_eql_value.clone();
            ems_screen_eql["EnergyManagementSystem:ConstructionIndexVariable"] =
                json!({"HELD_EQL_EMS":{"construction_object_name":"Opaque"}});
            assert!(
                !crate::support::assess_support_with_source_admission(&screen_eql_raw,&screen_eql_compiled.report,Some(screen_eql_model),RunMode::Compatibility,PartialRunPolicy::Deny,RunOutputFormat::RustNative,TraceLevel::Normal,None).allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&screen_eql_raw).is_err()
                    && crate::ordinary_equipment_input::resolve(&screen_eql_raw).is_err()
                    && !screen_eql_idf.is_epjson() && crate::ordinary_equipment_input::resolve_source_b(&screen_eql_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&controlled_screen_eql.to_string()).expect("control Raw")).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&active_screen_eql.to_string()).expect("active EQL Raw")).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&ems_screen_eql.to_string()).expect("held EMS Raw")).is_err()
                    && ["autocalculate"," Autocalculate "," "].iter().all(|text| {
                        let mut bad_auto = screen_eql_value.clone();
                        bad_auto[SCREEN_EQL]["\u{b}A SCREEN EQL\u{a0}"]["screen_beam_beam_solar_transmittance"] = json!(text);
                        crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&bad_auto.to_string()).expect("nonexact Auto Raw")).is_err()
                    })
            );
            for field in screen_eql_keys {
                let mut bad_value = screen_eql_value.clone();
                bad_value[SCREEN_EQL][" B SCREEN EQL "][field] = json!(false);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string())
                            .expect("wrong-kind Raw")
                    )
                    .is_err(),
                    "ScreenEQLEQL known field {field}"
                );
            }
            let opaque_screen_eql = screen_eql_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowScreenEquivalentLayer)
                .expect("held opaque material");
            for case in 0_usize..12 {
                let mut bad_model = screen_eql_model.clone();
                match case {
                    0 => bad_model.materials.retain(|row| {
                        row.kind() != ep_model::MaterialKind::WindowScreenEquivalentLayer
                    }),
                    1 => {
                        let mut extra = bad_model.materials[screen_eql_a_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA SCREEN EQL".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[screen_eql_a_position].id = opaque_screen_eql.id,
                    3 => {
                        bad_model.materials[screen_eql_a_position].definition =
                            opaque_screen_eql.definition
                    }
                    4 => {
                        bad_model.materials[screen_eql_a_position].name =
                            ep_model::NormalizedName("WRONG SCREEN EQL".into())
                    }
                    5 => {
                        bad_model.materials[screen_eql_a_position].id =
                            ep_model::MaterialId(u32::MAX)
                    }
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != screen_eql_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((screen_eql_a, "\u{b}A SCREEN EQL\u{a0}".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA SCREEN_EQL ALIAS ", screen_eql_a);
                    }
                    9 => {
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == screen_eql_a)
                            .expect("A report tuple")
                            .1 = " B SCREEN EQL ".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == screen_eql_b)
                            .expect("B report tuple")
                            .1 = "\u{b}A SCREEN EQL\u{a0}".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => {
                        bad_model.material_names = ep_model::NameMap::source_exact_namespace();
                        bad_model.materials.iter().for_each(|row| {
                            let id = if row.id == screen_eql_a {
                                screen_eql_b
                            } else if row.id == screen_eql_b {
                                screen_eql_a
                            } else {
                                row.id
                            };
                            let _ = bad_model.material_names.insert(&row.name.0, id);
                        });
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &screen_eql_raw,
                        &screen_eql_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&screen_eql_admission)
                    )
                    .allows_rust_runtime(),
                    "ScreenEQLEQL custody case {case}"
                );
            }
            let screen_eql_construction = screen_eql_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .expect("real opaque Construction");
            for case in 0_usize..11 {
                let mut used_value = screen_eql_value.clone();
                let mut fields =
                    serde_json::Map::from_iter([("outside_layer".into(), json!("Opaque"))]);
                let key = if case == 0 || case == 10 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", case + 1)
                };
                fields.insert(
                    key,
                    json!(if case.is_multiple_of(2) {
                        "\u{b}A SCREEN EQL\u{a0}"
                    } else {
                        " b screen eql "
                    }),
                );
                used_value["Construction"]["SCREEN_EQL USED"] = serde_json::Value::Object(fields);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&used_value.to_string())
                            .expect("original active SCREEN_EQL layer")
                    )
                    .is_err(),
                    "Raw SCREEN_EQL layer {case}"
                );
                let mut used_model = screen_eql_model.clone();
                let construction = &mut used_model.constructions[screen_eql_construction];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(screen_eql_a);
                } else {
                    construction.layers.resize(case + 1, opaque_screen_eql.id);
                    construction.layers[case] = if case.is_multiple_of(2) {
                        screen_eql_a
                    } else {
                        screen_eql_b
                    };
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &screen_eql_raw,
                        &screen_eql_compiled.report,
                        Some(&used_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&screen_eql_admission)
                    )
                    .allows_rust_runtime(),
                    "typed/effective SCREEN_EQL layer {case}"
                );
            }
        }

        // Unused Blind:EQL extends the existing real positive DSOA/Ideal/OA scene.
        // Caller observes public definition/Raw/IP/projection custody; private item counts/blanks belong to runtime B1.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            const BLIND_EQL: &str = "WindowMaterial:Blind:EquivalentLayer";
            let mut blind_eql_value = curve_value.clone();
            blind_eql_value[BLIND_EQL] = json!({
                "\u{b}":{
                    "slat_width":0.02,"slat_separation":0.015,"slat_angle":45,
                    "front_side_slat_beam_diffuse_solar_reflectance":0.3,"back_side_slat_beam_diffuse_solar_reflectance":0.35,
                    "front_side_slat_diffuse_diffuse_solar_reflectance":0.2,"back_side_slat_diffuse_diffuse_solar_reflectance":0.3,
                    "name":false,"idf_order":[],"metadata":null
                },
                "\u{a0}":{
                    "slat_orientation":"Horizontal","slat_width":0.02,"slat_separation":0.015,"slat_crown":-0.0,"slat_angle":45.0,
                    "front_side_slat_beam_diffuse_solar_transmittance":0.1,"back_side_slat_beam_diffuse_solar_transmittance":0.15,
                    "front_side_slat_beam_diffuse_solar_reflectance":0.3,"back_side_slat_beam_diffuse_solar_reflectance":0.35,
                    "front_side_slat_beam_diffuse_visible_transmittance":0.02,"back_side_slat_beam_diffuse_visible_transmittance":0.03,
                    "front_side_slat_beam_diffuse_visible_reflectance":0.25,"back_side_slat_beam_diffuse_visible_reflectance":0.28,
                    "slat_diffuse_diffuse_solar_transmittance":0.1,"front_side_slat_diffuse_diffuse_solar_reflectance":0.2,
                    "back_side_slat_diffuse_diffuse_solar_reflectance":0.3,"slat_diffuse_diffuse_visible_transmittance":0.11,
                    "front_side_slat_diffuse_diffuse_visible_reflectance":0.22,"back_side_slat_diffuse_diffuse_visible_reflectance":0.33,
                    "slat_infrared_transmittance":0.05,"front_side_slat_infrared_emissivity":0.84,"back_side_slat_infrared_emissivity":0.76,
                    "slat_angle_control":"FixedSlatAngle","name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},
                    "idf_max_extensible_fields":null,"extra_string":"unconsumed",
                    "metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            let blind_eql_raw =
                ep_raw_model::parse_epjson_str_for_source(&blind_eql_value.to_string())
                    .expect("Source BlindEQL epJSON");
            let blind_eql_compiled = ep_compiler::compile_raw_model_for_source(&blind_eql_raw);
            assert!(
                !blind_eql_compiled.has_errors(),
                "{:?}",
                blind_eql_compiled.report.diagnostics
            );
            let blind_eql_model = blind_eql_compiled
                .model
                .as_ref()
                .expect("two Source BlindEQL materials");
            let blind_eql_a = blind_eql_model
                .material_names
                .resolve("\u{b}")
                .expect("untrimmed A");
            let blind_eql_b = blind_eql_model
                .material_names
                .resolve("\u{a0}")
                .expect("untrimmed B");
            assert!(
                blind_eql_a != blind_eql_b
                    && blind_eql_model.material_names.is_source_exact_namespace()
                    && blind_eql_model.material_names.resolve("").is_none()
                    && blind_eql_model
                        .materials
                        .iter()
                        .filter(
                            |row| row.kind() == ep_model::MaterialKind::WindowBlindEquivalentLayer
                        )
                        .count()
                        == 2
                    && blind_eql_model
                        .materials
                        .iter()
                        .any(|row| row.id == blind_eql_a
                            && row.name.0 == "\u{b}"
                            && matches!(
                                row.definition,
                                ep_model::MaterialDefinition::WindowBlindEquivalentLayer(_)
                            ))
                    && blind_eql_model
                        .materials
                        .iter()
                        .any(|row| row.id == blind_eql_b
                            && row.name.0 == "\u{a0}"
                            && matches!(
                                row.definition,
                                ep_model::MaterialDefinition::WindowBlindEquivalentLayer(_)
                            ))
                    && blind_eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(blind_eql_a, "\u{b}".into()))
                    && blind_eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(blind_eql_b, "\u{a0}".into()))
            );
            let mut blind_eql_ip =
                crate::ordinary_input_processor::resolve_source_b(&blind_eql_raw)
                    .expect("real Raw/IP");
            let blind_eql_args = blind_eql_ip
                .get_object_def_max_args(BLIND_EQL)
                .expect("public3A21N definition, not private item counts");
            let mut blind_eql_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            blind_eql_ip
                .report_source_idf_records_stats(&mut blind_eql_stats)
                .expect("real record statistics");
            assert!(
                (
                    blind_eql_args.alphas,
                    blind_eql_args.numerics,
                    blind_eql_args.total().expect("bounded args")
                ) == (3, 21, 24)
                    && blind_eql_ip.get_num_objects_found(BLIND_EQL) == 2
                    && blind_eql_ip
                        .source_raw_object_names(BLIND_EQL)
                        .expect("ordered names")
                        .iter()
                        .map(String::as_str)
                        .eq(["\u{a0}", "\u{b}"])
                    && blind_eql_ip.is_object_unused(BLIND_EQL, "\u{b}")
                    && blind_eql_ip.is_object_unused(BLIND_EQL, "\u{a0}")
                    && usize::try_from(blind_eql_stats.number_input_records)
                        .expect("nonnegative records")
                        == blind_eql_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let blind_eql_original = blind_eql_ip
                .source_raw_object_fields(BLIND_EQL, "\u{a0}")
                .expect("full original custody");
            let blind_eql_minimum = blind_eql_ip
                .source_raw_object_fields(BLIND_EQL, "\u{b}")
                .expect("minimum original custody");
            let blind_eql_raw_fields = &blind_eql_raw.objects
                [&ep_raw_model::ObjectType(BLIND_EQL.into())]
                [&ep_raw_model::ObjectName("\u{a0}".into())]
                .fields;
            assert!(
                matches!(blind_eql_minimum.get("slat_angle"),Some(S::Number(token)) if token == "45")
                    && matches!(
                        ep_model::classify_source_json_number("45"),
                        Ok(ep_model::SourceJsonNumber::Integer(45))
                    )
                    && matches!(blind_eql_raw_fields.get(&ep_raw_model::FieldName("slat_angle".into())),Some(ep_raw_model::RawValue::Number(token)) if token == "45.0")
                    && matches!(blind_eql_original.get("slat_angle"),Some(S::Number(token)) if token == "45.0")
                    && matches!(ep_model::classify_source_json_number("45.0"),Ok(ep_model::SourceJsonNumber::Float(value)) if value.to_bits() == 45.0_f64.to_bits())
                    && matches!(blind_eql_original.get("slat_crown"),Some(S::Number(token)) if token == "-0.0")
                    && matches!(ep_model::classify_source_json_number("-0.0"),Ok(ep_model::SourceJsonNumber::Float(value)) if value.to_bits() == (-0.0_f64).to_bits())
                    && matches!(blind_eql_original.get("name"), Some(S::Bool(false)))
                    && matches!(blind_eql_original.get("idf_order"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(blind_eql_original.get("idf_max_fields"), Some(S::Object(_)))
                    && matches!(
                        blind_eql_original.get("idf_max_extensible_fields"),
                        Some(S::Null)
                    )
                    && matches!(blind_eql_original.get("extra_string"),Some(S::String(text)) if text == "unconsumed")
                    && matches!(blind_eql_original.get("metadata"),Some(S::Object(fields)) if matches!(fields.get("retained"),Some(S::Array(rows)) if rows.len() == 5))
            );
            let blind_eql_projection =
                crate::ordinary_material_input::resolve_source(&blind_eql_raw, &blind_eql_ip)
                    .expect("same Raw/IP projection");
            let blind_eql_numeric_keys = [
                "slat_width",
                "slat_separation",
                "slat_crown",
                "slat_angle",
                "front_side_slat_beam_diffuse_solar_transmittance",
                "back_side_slat_beam_diffuse_solar_transmittance",
                "front_side_slat_beam_diffuse_solar_reflectance",
                "back_side_slat_beam_diffuse_solar_reflectance",
                "front_side_slat_beam_diffuse_visible_transmittance",
                "back_side_slat_beam_diffuse_visible_transmittance",
                "front_side_slat_beam_diffuse_visible_reflectance",
                "back_side_slat_beam_diffuse_visible_reflectance",
                "slat_diffuse_diffuse_solar_transmittance",
                "front_side_slat_diffuse_diffuse_solar_reflectance",
                "back_side_slat_diffuse_diffuse_solar_reflectance",
                "slat_diffuse_diffuse_visible_transmittance",
                "front_side_slat_diffuse_diffuse_visible_reflectance",
                "back_side_slat_diffuse_diffuse_visible_reflectance",
                "slat_infrared_transmittance",
                "front_side_slat_infrared_emissivity",
                "back_side_slat_infrared_emissivity",
            ];
            let projected = &blind_eql_projection.window_blind_equivalent_layer;
            let projected_minimum = projected
                .iter()
                .find(|row| row.object_name == "\u{b}")
                .expect("minimum projection");
            let projected_full = projected
                .iter()
                .find(|row| row.object_name == "\u{a0}")
                .expect("full projection");
            let mut count_mismatch_raw = blind_eql_raw.clone();
            count_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(BLIND_EQL.into()))
                .expect("EQL family")
                .remove(&ep_raw_model::ObjectName("\u{b}".into()));
            let mut identity_mismatch_raw = blind_eql_raw.clone();
            let identity_rows = identity_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(BLIND_EQL.into()))
                .expect("EQL family");
            let identity_row = identity_rows
                .remove(&ep_raw_model::ObjectName("\u{b}".into()))
                .expect("existing row");
            identity_rows.insert(
                ep_raw_model::ObjectName("WRONG BLIND EQL IP NAME".into()),
                identity_row,
            );
            assert!(
                blind_eql_projection.source_is_epjson && projected.len() == 2
                    && crate::ordinary_material_input::resolve_source(&count_mismatch_raw,&blind_eql_ip).is_err()
                    && crate::ordinary_material_input::resolve_source(&identity_mismatch_raw,&blind_eql_ip).is_err()
                    && projected_minimum.fields.len() == 7 && projected_full.fields.len() == 23
                    && projected.iter().all(|row|row.idf_max_fields.is_none() && row.fields.keys().all(|field|
                        crate::ordinary_equipment_input::source_b_blind_equivalent_layer_known_field(field)))
                    && blind_eql_numeric_keys.iter().zip([0.02_f64,0.015,-0.0,45.0,0.1,0.15,0.3,0.35,0.02,0.03,0.25,0.28,0.1,0.2,0.3,0.11,0.22,0.33,0.05,0.84,0.76])
                        .all(|(field,value)|matches!(projected_full.fields.get(*field),Some(P::Number(actual)) if actual.to_bits() == value.to_bits()))
                    && matches!(projected_minimum.fields.get("slat_angle"),Some(P::Number(value)) if value.to_bits() == 45.0_f64.to_bits())
                    && matches!(projected_full.fields.get("slat_orientation"),Some(P::String(text)) if text == "Horizontal")
                    && matches!(projected_full.fields.get("slat_angle_control"),Some(P::String(text)) if text == "FixedSlatAngle")
                    && ["\n"," \t\r\u{c}"].iter().all(|name|{
                        let mut bad_name = blind_eql_value.clone();
                        let row = bad_name[BLIND_EQL].as_object_mut().expect("family map").remove("\u{b}").expect("minimum row");
                        bad_name[BLIND_EQL].as_object_mut().expect("family map").insert((*name).into(),row);
                        crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&bad_name.to_string()).expect("bad original name Raw"))
                            .is_err_and(|error|error.contains("Source BlindEQL requires original RE2 material name"))
                    })
            );
            let mut blind_eql_trace = inspect_porting_scope(
                &blind_eql_raw,
                &blind_eql_compiled.report,
                Some(blind_eql_model),
                PortingScope::B,
            );
            assert!(
                blind_eql_trace.admissible,
                "{:?}",
                blind_eql_trace.violations
            );
            blind_eql_trace.environment = Some(
                super::environment::environment_from_weather(
                    blind_eql_model,
                    &weather,
                    "same BlindEQL Source weather",
                )
                .expect("actual Source environment"),
            );
            let blind_eql_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &blind_eql_raw,
                &blind_eql_trace,
            )
            .expect("validated Source BLIND_EQL scope");
            let mut permuted_blind_eql_model = blind_eql_model.clone();
            let blind_eql_a_position = blind_eql_model
                .materials
                .iter()
                .position(|row| row.id == blind_eql_a)
                .expect("A arena row");
            let blind_eql_b_position = blind_eql_model
                .materials
                .iter()
                .position(|row| row.id == blind_eql_b)
                .expect("B arena row");
            permuted_blind_eql_model
                .materials
                .swap(blind_eql_a_position, blind_eql_b_position);
            assert!(
                crate::support::assess_support_with_source_admission(
                    &blind_eql_raw,
                    &blind_eql_compiled.report,
                    Some(blind_eql_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&blind_eql_admission)
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &blind_eql_raw,
                        &blind_eql_compiled.report,
                        Some(&permuted_blind_eql_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&blind_eql_admission)
                    )
                    .allows_rust_runtime()
            );
            let blind_eql_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                r#"{"Zone":{"Zone":{}},"WindowMaterial:Blind:EquivalentLayer":{"IDF BLIND EQL":{"slat_width":0.02,"slat_separation":0.015,"slat_crown":0.0015,"slat_angle":45.0,"front_side_slat_beam_diffuse_solar_transmittance":0.1,"back_side_slat_beam_diffuse_solar_transmittance":0.15,"front_side_slat_beam_diffuse_solar_reflectance":0.3,"back_side_slat_beam_diffuse_solar_reflectance":0.35,"front_side_slat_beam_diffuse_visible_transmittance":0.02,"back_side_slat_beam_diffuse_visible_transmittance":0.03,"front_side_slat_beam_diffuse_visible_reflectance":0.25,"back_side_slat_beam_diffuse_visible_reflectance":0.28,"slat_diffuse_diffuse_solar_transmittance":0.1,"front_side_slat_diffuse_diffuse_solar_reflectance":0.2,"back_side_slat_diffuse_diffuse_solar_reflectance":0.3,"slat_diffuse_diffuse_visible_transmittance":0.11,"front_side_slat_diffuse_diffuse_visible_reflectance":0.22,"back_side_slat_diffuse_diffuse_visible_reflectance":0.33,"slat_infrared_transmittance":0.05,"front_side_slat_infrared_emissivity":0.84,"back_side_slat_infrared_emissivity":0.76,"slat_orientation":"Horizontal","slat_angle_control":"FixedSlatAngle"}}}"#,
                "Zone,Zone;WindowMaterial:Blind:EquivalentLayer,IDF BLIND EQL,Horizontal,0.02,0.015,0.0015,45.0,0.1,0.15,0.3,0.35,0.02,0.03,0.25,0.28,0.1,0.2,0.3,0.11,0.22,0.33,0.05,0.84,0.76,FixedSlatAngle;",
            ).expect("matched Zone+BlindEQL IDF24materialoperands");
            let blind_eql_idf_compiled = ep_compiler::compile_raw_model(&blind_eql_idf);
            let source_a_base =
                ep_raw_model::parse_epjson_str_for_source(&input(PortingScope::A).to_string())
                    .expect("unchanged valid SourceA baseline");
            let mut source_a_with_blind_eql = source_a_base.clone();
            source_a_with_blind_eql.objects.insert(
                ep_raw_model::ObjectType(BLIND_EQL.into()),
                blind_eql_raw
                    .objects
                    .get(&ep_raw_model::ObjectType(BLIND_EQL.into()))
                    .expect("same genuine BlindEQL family only")
                    .clone(),
            );
            let mut controlled_blind_eql = blind_eql_value.clone();
            controlled_blind_eql["WindowShadingControl"] = json!({"EQL BLIND CONTROL":{"shading_type":"ExteriorBlind","shading_device_material_name":"\u{b}"}});
            let mut active_blind_eql = blind_eql_value.clone();
            active_blind_eql["Construction:WindowEquivalentLayer"] =
                json!({"ACTIVE EQL BLIND":{"outside_layer":"\u{b}"}});
            let mut ems_blind_eql = blind_eql_value.clone();
            ems_blind_eql["EnergyManagementSystem:ConstructionIndexVariable"] =
                json!({"HELD_EQL_EMS":{"construction_object_name":"Opaque"}});
            assert!(
                !crate::support::assess_support_with_source_admission(&blind_eql_raw,&blind_eql_compiled.report,Some(blind_eql_model),RunMode::Compatibility,PartialRunPolicy::Deny,RunOutputFormat::RustNative,TraceLevel::Normal,None).allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&source_a_base).is_ok()
                    && crate::ordinary_equipment_input::resolve_source_a(&source_a_with_blind_eql)
                        .is_err_and(|error|error == "ordinary equipment excludes positive family WindowMaterial:Blind:EquivalentLayer")
                    && crate::ordinary_equipment_input::resolve(&blind_eql_raw).is_err()
                    && !blind_eql_idf.is_epjson() && !blind_eql_idf_compiled.has_errors()
                    && blind_eql_idf_compiled.model.as_ref().is_some_and(|model|model.materials.len() == 1
                        && model.materials[0].kind() == ep_model::MaterialKind::WindowBlindEquivalentLayer)
                    && crate::ordinary_equipment_input::resolve_source_b(&blind_eql_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_a(&blind_eql_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&controlled_blind_eql.to_string()).expect("control Raw")).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&active_blind_eql.to_string()).expect("active EQL Raw")).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&ems_blind_eql.to_string()).expect("held EMS Raw")).is_err()
            );
            for (field, kind, probe) in [
                ("slat_width", "kind", 0_usize),
                ("slat_separation", "kind", 0_usize),
                ("slat_crown", "kind", 0_usize),
                ("slat_angle", "kind", 0_usize),
                (
                    "front_side_slat_beam_diffuse_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_slat_beam_diffuse_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_slat_beam_diffuse_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_slat_beam_diffuse_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_slat_beam_diffuse_visible_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_slat_beam_diffuse_visible_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_slat_beam_diffuse_visible_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_slat_beam_diffuse_visible_reflectance",
                    "kind",
                    0_usize,
                ),
                ("slat_diffuse_diffuse_solar_transmittance", "kind", 0_usize),
                (
                    "front_side_slat_diffuse_diffuse_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_slat_diffuse_diffuse_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "slat_diffuse_diffuse_visible_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_slat_diffuse_diffuse_visible_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_slat_diffuse_diffuse_visible_reflectance",
                    "kind",
                    0_usize,
                ),
                ("slat_infrared_transmittance", "kind", 0_usize),
                ("front_side_slat_infrared_emissivity", "kind", 0_usize),
                ("back_side_slat_infrared_emissivity", "kind", 0_usize),
                ("slat_orientation", "kind", 0_usize),
                ("slat_angle_control", "kind", 0_usize),
                ("", "alpha", 0_usize),
                ("", "alpha", 1_usize),
                ("", "alpha", 2_usize),
                ("", "alpha", 3_usize),
                ("", "alpha", 4_usize),
                ("", "alpha", 5_usize),
            ] {
                let mut bad_value = blind_eql_value.clone();
                if kind == "kind" {
                    bad_value[BLIND_EQL]["\u{a0}"][field] = json!(false);
                } else {
                    let (key, text) = match probe {
                        0 => ("slat_orientation", "horizontal"),
                        1 => ("slat_orientation", " Horizontal "),
                        2 => ("slat_orientation", "UnknownOrientation"),
                        3 => ("slat_angle_control", "fixedslatangle"),
                        4 => ("slat_angle_control", " FixedSlatAngle "),
                        _ => ("slat_angle_control", "UnknownControl"),
                    };
                    bad_value[BLIND_EQL]["\u{a0}"][key] = json!(text);
                }
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string())
                            .expect("wrong-kind/strict-alpha Raw")
                    )
                    .is_err_and(|error| error.contains("Source BlindEQL requires")),
                    "BlindEQL original boundary {field}/{kind}/{probe}"
                );
            }
            let opaque_blind_eql = blind_eql_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowBlindEquivalentLayer)
                .expect("held opaque material");
            for case in 0_usize..12 {
                let mut bad_model = blind_eql_model.clone();
                match case {
                    0 => bad_model.materials.retain(|row| {
                        row.kind() != ep_model::MaterialKind::WindowBlindEquivalentLayer
                    }),
                    1 => {
                        let mut extra = bad_model.materials[blind_eql_a_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA BLIND EQL".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[blind_eql_a_position].id = opaque_blind_eql.id,
                    3 => {
                        bad_model.materials[blind_eql_a_position].definition =
                            opaque_blind_eql.definition
                    }
                    4 => {
                        bad_model.materials[blind_eql_a_position].name =
                            ep_model::NormalizedName("WRONG BLIND EQL".into())
                    }
                    5 => {
                        bad_model.materials[blind_eql_a_position].id =
                            ep_model::MaterialId(u32::MAX)
                    }
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != blind_eql_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((blind_eql_a, "\u{b}".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA BLIND_EQL ALIAS ", blind_eql_a);
                    }
                    9 => {
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == blind_eql_a)
                            .expect("A report tuple")
                            .1 = "\u{a0}".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == blind_eql_b)
                            .expect("B report tuple")
                            .1 = "\u{b}".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => {
                        bad_model.material_names = ep_model::NameMap::source_exact_namespace();
                        bad_model.materials.iter().for_each(|row| {
                            let id = if row.id == blind_eql_a {
                                blind_eql_b
                            } else if row.id == blind_eql_b {
                                blind_eql_a
                            } else {
                                row.id
                            };
                            let _ = bad_model.material_names.insert(&row.name.0, id);
                        });
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &blind_eql_raw,
                        &blind_eql_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&blind_eql_admission)
                    )
                    .allows_rust_runtime(),
                    "BlindEQL custody case {case}"
                );
            }
            let blind_eql_construction = blind_eql_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .expect("real opaque Construction");
            for case in 0_usize..11 {
                let mut used_value = blind_eql_value.clone();
                let mut fields =
                    serde_json::Map::from_iter([("outside_layer".into(), json!("Opaque"))]);
                let key = if case == 0 || case == 10 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", case + 1)
                };
                fields.insert(
                    key,
                    json!(if case.is_multiple_of(2) {
                        "\u{b}"
                    } else {
                        "\u{a0}"
                    }),
                );
                used_value["Construction"]["BLIND_EQL USED"] = serde_json::Value::Object(fields);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&used_value.to_string())
                            .expect("original active BLIND_EQL layer")
                    )
                    .is_err_and(|error| error.contains(
                        "Source unused BlindEQL excludes every Construction layer reference"
                    )),
                    "Raw BLIND_EQL layer {case}"
                );
                let mut used_model = blind_eql_model.clone();
                let construction = &mut used_model.constructions[blind_eql_construction];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(blind_eql_a);
                } else {
                    construction.layers.resize(case + 1, opaque_blind_eql.id);
                    construction.layers[case] = if case.is_multiple_of(2) {
                        blind_eql_a
                    } else {
                        blind_eql_b
                    };
                }
                let used_assessment = crate::support::assess_support_with_source_admission(
                    &blind_eql_raw,
                    &blind_eql_compiled.report,
                    Some(&used_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&blind_eql_admission),
                );
                assert!(
                    !used_assessment.allows_rust_runtime()
                        && used_assessment
                            .unsupported_objects
                            .iter()
                            .any(|entry| entry.object_type == BLIND_EQL),
                    "typed/effective BLIND_EQL layer {case}"
                );
            }
        }

        // Unused Glazing:EQL extends the existing real positive DSOA/Ideal/OA scene.
        // Caller observes public definition/Raw/IP/projection custody; private item counts/blanks belong to runtime B1.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            const GLAZING_EQL: &str = "WindowMaterial:Glazing:EquivalentLayer";
            let mut glazing_eql_value = curve_value.clone();
            glazing_eql_value[GLAZING_EQL] = json!({
                "\u{b}":{
                    "front_side_beam_beam_solar_transmittance":0,"back_side_beam_beam_solar_transmittance":0.62,
                    "front_side_beam_beam_solar_reflectance":0.21,"back_side_beam_beam_solar_reflectance":0.22,
                    "thermal_resistance":u64::MAX,"name":false,"idf_order":[],"metadata":null
                },
                "\u{a0}":{
                    "optical_data_type":"SpectralAverage","window_glass_spectral_data_set_name":"Unresolved but ignored set",
                    "front_side_beam_beam_solar_transmittance":-0.0,"back_side_beam_beam_solar_transmittance":0.62,
                    "front_side_beam_beam_solar_reflectance":0.21,"back_side_beam_beam_solar_reflectance":0.22,
                    "front_side_beam_beam_visible_solar_transmittance":0.71,"back_side_beam_beam_visible_solar_transmittance":0.72,
                    "front_side_beam_beam_visible_solar_reflectance":0.11,"back_side_beam_beam_visible_solar_reflectance":0.12,
                    "front_side_beam_diffuse_solar_transmittance":0.31,"back_side_beam_diffuse_solar_transmittance":0.32,
                    "front_side_beam_diffuse_solar_reflectance":0.23,"back_side_beam_diffuse_solar_reflectance":0.24,
                    "front_side_beam_diffuse_visible_solar_transmittance":0.41,"back_side_beam_diffuse_visible_solar_transmittance":0.42,
                    "front_side_beam_diffuse_visible_solar_reflectance":0.13,"back_side_beam_diffuse_visible_solar_reflectance":0.14,
                    "diffuse_diffuse_solar_transmittance":"Autocalculate","front_side_diffuse_diffuse_solar_reflectance":"",
                    "back_side_diffuse_diffuse_solar_reflectance":0.26,"diffuse_diffuse_visible_solar_transmittance":"",
                    "front_side_diffuse_diffuse_visible_solar_reflectance":"Autocalculate","back_side_diffuse_diffuse_visible_solar_reflectance":0.16,
                    "infrared_transmittance_applies_to_front_and_back_":0.03,"front_side_infrared_emissivity":0.81,
                    "back_side_infrared_emissivity":0.82,"thermal_resistance":1.8446744073709552e19,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},"idf_max_extensible_fields":null,
                    "extra_string":"unconsumed","metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}
                }
            });
            // Preserve the actual Integer -0 token; serde_json::Value alone would print integer0.
            let glazing_eql_text = glazing_eql_value.to_string().replacen(
                "\"front_side_beam_beam_solar_transmittance\":0,",
                "\"front_side_beam_beam_solar_transmittance\":-0,",
                1,
            );
            let glazing_eql_raw = ep_raw_model::parse_epjson_str_for_source(&glazing_eql_text)
                .expect("Source GlazingEQL original tokens");
            let glazing_eql_compiled = ep_compiler::compile_raw_model_for_source(&glazing_eql_raw);
            assert!(
                !glazing_eql_compiled.has_errors(),
                "{:?}",
                glazing_eql_compiled.report.diagnostics
            );
            let glazing_eql_model = glazing_eql_compiled
                .model
                .as_ref()
                .expect("two Source GlazingEQL materials");
            let glazing_eql_a = glazing_eql_model
                .material_names
                .resolve("\u{b}")
                .expect("untrimmed A");
            let glazing_eql_b = glazing_eql_model
                .material_names
                .resolve("\u{a0}")
                .expect("untrimmed B");
            assert!(
                glazing_eql_a != glazing_eql_b
                    && glazing_eql_model.material_names.is_source_exact_namespace()
                    && glazing_eql_model.material_names.resolve("").is_none()
                    && glazing_eql_model
                        .materials
                        .iter()
                        .filter(|row| row.kind()
                            == ep_model::MaterialKind::WindowGlazingEquivalentLayer)
                        .count()
                        == 2
                    && glazing_eql_model
                        .materials
                        .iter()
                        .any(|row| row.id == glazing_eql_a
                            && row.name.0 == "\u{b}"
                            && matches!(
                                row.definition,
                                ep_model::MaterialDefinition::WindowGlazingEquivalentLayer(_)
                            ))
                    && glazing_eql_model
                        .materials
                        .iter()
                        .any(|row| row.id == glazing_eql_b
                            && row.name.0 == "\u{a0}"
                            && matches!(
                                row.definition,
                                ep_model::MaterialDefinition::WindowGlazingEquivalentLayer(_)
                            ))
                    && glazing_eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(glazing_eql_a, "\u{b}".into()))
                    && glazing_eql_model
                        .ctf_report_names
                        .materials
                        .contains(&(glazing_eql_b, "\u{a0}".into()))
            );
            let mut glazing_eql_ip =
                crate::ordinary_input_processor::resolve_source_b(&glazing_eql_raw)
                    .expect("real Raw/IP");
            let glazing_eql_args = glazing_eql_ip
                .get_object_def_max_args(GLAZING_EQL)
                .expect("public3A26N definition only, not private item counts");
            let mut glazing_eql_stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            glazing_eql_ip
                .report_source_idf_records_stats(&mut glazing_eql_stats)
                .expect("real record statistics");
            assert!(
                (
                    glazing_eql_args.alphas,
                    glazing_eql_args.numerics,
                    glazing_eql_args.total().expect("bounded args")
                ) == (3, 26, 29)
                    && glazing_eql_ip.get_num_objects_found(GLAZING_EQL) == 2
                    && glazing_eql_ip
                        .source_raw_object_names(GLAZING_EQL)
                        .expect("ordered names")
                        .iter()
                        .map(String::as_str)
                        .eq(["\u{a0}", "\u{b}"])
                    && glazing_eql_ip.is_object_unused(GLAZING_EQL, "\u{b}")
                    && glazing_eql_ip.is_object_unused(GLAZING_EQL, "\u{a0}")
                    && usize::try_from(glazing_eql_stats.number_input_records)
                        .expect("nonnegative records")
                        == glazing_eql_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let glazing_eql_original = glazing_eql_ip
                .source_raw_object_fields(GLAZING_EQL, "\u{a0}")
                .expect("full original custody");
            let glazing_eql_minimum = glazing_eql_ip
                .source_raw_object_fields(GLAZING_EQL, "\u{b}")
                .expect("minimum original custody");
            let glazing_eql_min_raw = &glazing_eql_raw.objects
                [&ep_raw_model::ObjectType(GLAZING_EQL.into())]
                [&ep_raw_model::ObjectName("\u{b}".into())]
                .fields;
            let glazing_eql_full_raw = &glazing_eql_raw.objects
                [&ep_raw_model::ObjectType(GLAZING_EQL.into())]
                [&ep_raw_model::ObjectName("\u{a0}".into())]
                .fields;
            assert!(
                matches!(glazing_eql_min_raw.get(&ep_raw_model::FieldName("front_side_beam_beam_solar_transmittance".into())),Some(ep_raw_model::RawValue::Number(token)) if token == "-0")
                    && matches!(glazing_eql_minimum.get("front_side_beam_beam_solar_transmittance"),Some(S::Number(token)) if token == "-0")
                    && matches!(
                        ep_model::classify_source_json_number("-0"),
                        Ok(ep_model::SourceJsonNumber::Integer(0))
                    )
                    && matches!(glazing_eql_original.get("front_side_beam_beam_solar_transmittance"),Some(S::Number(token)) if token == "-0.0")
                    && matches!(ep_model::classify_source_json_number("-0.0"),Ok(ep_model::SourceJsonNumber::Float(value)) if value.to_bits() == (-0.0_f64).to_bits())
                    && matches!(glazing_eql_minimum.get("thermal_resistance"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(
                        ep_model::classify_source_json_number("18446744073709551615"),
                        Ok(ep_model::SourceJsonNumber::Integer(-1))
                    )
                    && matches!(glazing_eql_full_raw.get(&ep_raw_model::FieldName("thermal_resistance".into())),Some(ep_raw_model::RawValue::Number(token))
                        if matches!(ep_model::classify_source_json_number(token),Ok(ep_model::SourceJsonNumber::Float(value)) if value.to_bits() == 18446744073709551616.0_f64.to_bits()))
                    && matches!(glazing_eql_original.get("diffuse_diffuse_solar_transmittance"),Some(S::String(text)) if text == "Autocalculate")
                    && matches!(glazing_eql_original.get("front_side_diffuse_diffuse_solar_reflectance"),Some(S::String(text)) if text.is_empty())
                    && matches!(glazing_eql_original.get("name"), Some(S::Bool(false)))
                    && matches!(glazing_eql_original.get("idf_order"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(
                        glazing_eql_original.get("idf_max_fields"),
                        Some(S::Object(_))
                    )
                    && matches!(
                        glazing_eql_original.get("idf_max_extensible_fields"),
                        Some(S::Null)
                    )
                    && matches!(glazing_eql_original.get("extra_string"),Some(S::String(text)) if text == "unconsumed")
                    && matches!(glazing_eql_original.get("metadata"),Some(S::Object(fields)) if matches!(fields.get("retained"),Some(S::Array(rows)) if rows.len() == 5))
            );
            let glazing_eql_projection =
                crate::ordinary_material_input::resolve_source(&glazing_eql_raw, &glazing_eql_ip)
                    .expect("same Raw/IP projection");
            let glazing_eql_numeric_keys = [
                "front_side_beam_beam_solar_transmittance",
                "back_side_beam_beam_solar_transmittance",
                "front_side_beam_beam_solar_reflectance",
                "back_side_beam_beam_solar_reflectance",
                "front_side_beam_beam_visible_solar_transmittance",
                "back_side_beam_beam_visible_solar_transmittance",
                "front_side_beam_beam_visible_solar_reflectance",
                "back_side_beam_beam_visible_solar_reflectance",
                "front_side_beam_diffuse_solar_transmittance",
                "back_side_beam_diffuse_solar_transmittance",
                "front_side_beam_diffuse_solar_reflectance",
                "back_side_beam_diffuse_solar_reflectance",
                "front_side_beam_diffuse_visible_solar_transmittance",
                "back_side_beam_diffuse_visible_solar_transmittance",
                "front_side_beam_diffuse_visible_solar_reflectance",
                "back_side_beam_diffuse_visible_solar_reflectance",
                "diffuse_diffuse_solar_transmittance",
                "front_side_diffuse_diffuse_solar_reflectance",
                "back_side_diffuse_diffuse_solar_reflectance",
                "diffuse_diffuse_visible_solar_transmittance",
                "front_side_diffuse_diffuse_visible_solar_reflectance",
                "back_side_diffuse_diffuse_visible_solar_reflectance",
                "infrared_transmittance_applies_to_front_and_back_",
                "front_side_infrared_emissivity",
                "back_side_infrared_emissivity",
                "thermal_resistance",
            ];
            let projected = &glazing_eql_projection.window_glazing_equivalent_layer;
            let projected_minimum = projected
                .iter()
                .find(|row| row.object_name == "\u{b}")
                .expect("minimum projection");
            let projected_full = projected
                .iter()
                .find(|row| row.object_name == "\u{a0}")
                .expect("full projection");
            let mut count_mismatch_raw = glazing_eql_raw.clone();
            count_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(GLAZING_EQL.into()))
                .expect("EQL family")
                .remove(&ep_raw_model::ObjectName("\u{b}".into()));
            let mut identity_mismatch_raw = glazing_eql_raw.clone();
            let identity_rows = identity_mismatch_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(GLAZING_EQL.into()))
                .expect("EQL family");
            let identity_row = identity_rows
                .remove(&ep_raw_model::ObjectName("\u{b}".into()))
                .expect("existing row");
            identity_rows.insert(
                ep_raw_model::ObjectName("WRONG GLAZING EQL IP NAME".into()),
                identity_row,
            );
            assert!(
                glazing_eql_projection.source_is_epjson && projected.len() == 2
                    && crate::ordinary_material_input::resolve_source(&count_mismatch_raw,&glazing_eql_ip).is_err()
                    && crate::ordinary_material_input::resolve_source(&identity_mismatch_raw,&glazing_eql_ip).is_err()
                    && projected_minimum.fields.len() == 5 && projected_full.fields.len() == 28
                    && projected.iter().all(|row|row.idf_max_fields.is_none() && row.fields.keys().all(|field|
                        crate::ordinary_equipment_input::source_b_glazing_equivalent_layer_known_field(field)))
                    && glazing_eql_numeric_keys.iter().zip([Some(-0.0_f64),Some(0.62),Some(0.21),Some(0.22),Some(0.71),Some(0.72),Some(0.11),Some(0.12),
                        Some(0.31),Some(0.32),Some(0.23),Some(0.24),Some(0.41),Some(0.42),Some(0.13),Some(0.14),None,None,Some(0.26),None,None,Some(0.16),
                        Some(0.03),Some(0.81),Some(0.82),Some(18446744073709551616.0)])
                        .all(|(field,value)|match value {Some(expected)=>matches!(projected_full.fields.get(*field),Some(P::Number(actual)) if actual.to_bits() == expected.to_bits()),
                            None=>matches!(projected_full.fields.get(*field),Some(P::String(text)) if matches!(text.as_str(),""|"Autocalculate"))})
                    && matches!(projected_minimum.fields.get("front_side_beam_beam_solar_transmittance"),Some(P::Number(value)) if value.to_bits() == 0.0_f64.to_bits())
                    && matches!(projected_minimum.fields.get("thermal_resistance"),Some(P::Number(value)) if value.to_bits() == (-1.0_f64).to_bits())
                    && matches!(projected_full.fields.get("diffuse_diffuse_solar_transmittance"),Some(P::String(text)) if text == "Autocalculate")
                    && matches!(projected_full.fields.get("front_side_diffuse_diffuse_solar_reflectance"),Some(P::String(text)) if text.is_empty())
                    && matches!(projected_full.fields.get("optical_data_type"),Some(P::String(text)) if text == "SpectralAverage")
                    && matches!(projected_full.fields.get("window_glass_spectral_data_set_name"),Some(P::String(text)) if text == "Unresolved but ignored set")
                    && glazing_eql_model.materials.iter().find(|row|row.id == glazing_eql_a).expect("same minimum advisory row")
                        .as_window_glazing_equivalent_layer().is_some_and(|row|row.thermal_resistance_m2_k_per_w.to_bits() == 18446744073709551616.0_f64.to_bits())
                    && ["","\n"," \t\r\u{c}"].iter().all(|name|{
                        let mut bad_name = glazing_eql_value.clone();
                        let row = bad_name[GLAZING_EQL].as_object_mut().expect("family map").remove("\u{b}").expect("minimum row");
                        bad_name[GLAZING_EQL].as_object_mut().expect("family map").insert((*name).into(),row);
                        crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&bad_name.to_string()).expect("bad original name Raw"))
                            .is_err_and(|error|error.contains("Source GlazingEQL requires original RE2 material name"))
                    })
            );
            let mut glazing_eql_trace = inspect_porting_scope(
                &glazing_eql_raw,
                &glazing_eql_compiled.report,
                Some(glazing_eql_model),
                PortingScope::B,
            );
            assert!(
                glazing_eql_trace.admissible,
                "{:?}",
                glazing_eql_trace.violations
            );
            glazing_eql_trace.environment = Some(
                super::environment::environment_from_weather(
                    glazing_eql_model,
                    &weather,
                    "same GlazingEQL Source weather",
                )
                .expect("actual Source environment"),
            );
            let glazing_eql_admission =
                crate::support::SourceSupportAdmission::from_validated_scope(
                    &glazing_eql_raw,
                    &glazing_eql_trace,
                )
                .expect("validated Source GLAZING_EQL scope");
            let mut permuted_glazing_eql_model = glazing_eql_model.clone();
            let glazing_eql_a_position = glazing_eql_model
                .materials
                .iter()
                .position(|row| row.id == glazing_eql_a)
                .expect("A arena row");
            let glazing_eql_b_position = glazing_eql_model
                .materials
                .iter()
                .position(|row| row.id == glazing_eql_b)
                .expect("B arena row");
            permuted_glazing_eql_model
                .materials
                .swap(glazing_eql_a_position, glazing_eql_b_position);
            assert!(
                crate::support::assess_support_with_source_admission(
                    &glazing_eql_raw,
                    &glazing_eql_compiled.report,
                    Some(glazing_eql_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&glazing_eql_admission)
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &glazing_eql_raw,
                        &glazing_eql_compiled.report,
                        Some(&permuted_glazing_eql_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&glazing_eql_admission)
                    )
                    .allows_rust_runtime()
            );
            let glazing_eql_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                r#"{"Zone":{"Zone":{}},"WindowMaterial:Glazing:EquivalentLayer":{"IDF GLASS EQL":{"front_side_beam_beam_solar_transmittance":0.61,"back_side_beam_beam_solar_transmittance":0.62,"front_side_beam_beam_solar_reflectance":0.21,"back_side_beam_beam_solar_reflectance":0.22,"front_side_beam_beam_visible_solar_transmittance":0.71,"back_side_beam_beam_visible_solar_transmittance":0.72,"front_side_beam_beam_visible_solar_reflectance":0.11,"back_side_beam_beam_visible_solar_reflectance":0.12,"front_side_beam_diffuse_solar_transmittance":0.31,"back_side_beam_diffuse_solar_transmittance":0.32,"front_side_beam_diffuse_solar_reflectance":0.23,"back_side_beam_diffuse_solar_reflectance":0.24,"front_side_beam_diffuse_visible_solar_transmittance":0.41,"back_side_beam_diffuse_visible_solar_transmittance":0.42,"front_side_beam_diffuse_visible_solar_reflectance":0.13,"back_side_beam_diffuse_visible_solar_reflectance":0.14,"diffuse_diffuse_solar_transmittance":0.51,"front_side_diffuse_diffuse_solar_reflectance":0.25,"back_side_diffuse_diffuse_solar_reflectance":0.26,"diffuse_diffuse_visible_solar_transmittance":0.52,"front_side_diffuse_diffuse_visible_solar_reflectance":0.15,"back_side_diffuse_diffuse_visible_solar_reflectance":0.16,"infrared_transmittance_applies_to_front_and_back_":0.03,"front_side_infrared_emissivity":0.81,"back_side_infrared_emissivity":0.82,"thermal_resistance":0.17,"optical_data_type":"SpectralAverage","window_glass_spectral_data_set_name":"UNRESOLVED SET"}}}"#,
                "Zone,Zone;WindowMaterial:Glazing:EquivalentLayer,IDF GLASS EQL,SpectralAverage,UNRESOLVED SET,0.61,0.62,0.21,0.22,0.71,0.72,0.11,0.12,0.31,0.32,0.23,0.24,0.41,0.42,0.13,0.14,0.51,0.25,0.26,0.52,0.15,0.16,0.03,0.81,0.82,0.17;",
            ).expect("matched Zone+GlazingEQL IDF29materialoperands");
            let glazing_eql_idf_compiled = ep_compiler::compile_raw_model(&glazing_eql_idf);
            let source_a_base =
                ep_raw_model::parse_epjson_str_for_source(&input(PortingScope::A).to_string())
                    .expect("unchanged valid SourceA baseline");
            let mut source_a_with_glazing_eql = source_a_base.clone();
            source_a_with_glazing_eql.objects.insert(
                ep_raw_model::ObjectType(GLAZING_EQL.into()),
                glazing_eql_raw
                    .objects
                    .get(&ep_raw_model::ObjectType(GLAZING_EQL.into()))
                    .expect("same genuine GlazingEQL family only")
                    .clone(),
            );
            let mut controlled_glazing_eql = glazing_eql_value.clone();
            controlled_glazing_eql["WindowShadingControl"] = json!({"EQL GLASS CONTROL":{"shading_type":"ExteriorShade","shading_device_material_name":"\u{b}"}});
            let mut active_glazing_eql = glazing_eql_value.clone();
            active_glazing_eql["Construction:WindowEquivalentLayer"] =
                json!({"ACTIVE EQL GLASS":{"outside_layer":"\u{b}"}});
            let mut ems_glazing_eql = glazing_eql_value.clone();
            ems_glazing_eql["EnergyManagementSystem:ConstructionIndexVariable"] =
                json!({"HELD_EQL_EMS":{"construction_object_name":"Opaque"}});
            assert!(
                !crate::support::assess_support_with_source_admission(&glazing_eql_raw,&glazing_eql_compiled.report,Some(glazing_eql_model),RunMode::Compatibility,PartialRunPolicy::Deny,RunOutputFormat::RustNative,TraceLevel::Normal,None).allows_rust_runtime()
                    && crate::ordinary_equipment_input::resolve_source_a(&source_a_base).is_ok()
                    && crate::ordinary_equipment_input::resolve_source_a(&source_a_with_glazing_eql)
                        .is_err_and(|error|error == "ordinary equipment excludes positive family WindowMaterial:Glazing:EquivalentLayer")
                    && crate::ordinary_equipment_input::resolve(&glazing_eql_raw).is_err()
                    && !glazing_eql_idf.is_epjson() && !glazing_eql_idf_compiled.has_errors()
                    && glazing_eql_idf_compiled.model.as_ref().is_some_and(|model|model.materials.len() == 1
                        && model.materials[0].kind() == ep_model::MaterialKind::WindowGlazingEquivalentLayer)
                    && crate::ordinary_equipment_input::resolve_source_b(&glazing_eql_idf)
                        .is_err_and(|error|error == "Source unused GlazingEQL requires direct epJSON")
                    && crate::ordinary_equipment_input::resolve_source_a(&glazing_eql_idf).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&controlled_glazing_eql.to_string()).expect("control Raw")).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&active_glazing_eql.to_string()).expect("active EQL Raw")).is_err()
                    && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&ems_glazing_eql.to_string()).expect("held EMS Raw")).is_err()
            );
            for (field, kind, probe) in [
                ("front_side_beam_beam_solar_transmittance", "kind", 0_usize),
                ("back_side_beam_beam_solar_transmittance", "kind", 0_usize),
                ("front_side_beam_beam_solar_reflectance", "kind", 0_usize),
                ("back_side_beam_beam_solar_reflectance", "kind", 0_usize),
                (
                    "front_side_beam_beam_visible_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_beam_beam_visible_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_beam_beam_visible_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_beam_beam_visible_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_beam_diffuse_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_beam_diffuse_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                ("front_side_beam_diffuse_solar_reflectance", "kind", 0_usize),
                ("back_side_beam_diffuse_solar_reflectance", "kind", 0_usize),
                (
                    "front_side_beam_diffuse_visible_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_beam_diffuse_visible_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_beam_diffuse_visible_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_beam_diffuse_visible_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                ("diffuse_diffuse_solar_transmittance", "kind", 0_usize),
                (
                    "front_side_diffuse_diffuse_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_diffuse_diffuse_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "diffuse_diffuse_visible_solar_transmittance",
                    "kind",
                    0_usize,
                ),
                (
                    "front_side_diffuse_diffuse_visible_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "back_side_diffuse_diffuse_visible_solar_reflectance",
                    "kind",
                    0_usize,
                ),
                (
                    "infrared_transmittance_applies_to_front_and_back_",
                    "kind",
                    0_usize,
                ),
                ("front_side_infrared_emissivity", "kind", 0_usize),
                ("back_side_infrared_emissivity", "kind", 0_usize),
                ("thermal_resistance", "kind", 0_usize),
                ("optical_data_type", "kind", 0_usize),
                ("window_glass_spectral_data_set_name", "kind", 0_usize),
                (
                    "front_side_beam_beam_solar_transmittance",
                    "missing",
                    0_usize,
                ),
                (
                    "back_side_beam_beam_solar_transmittance",
                    "missing",
                    0_usize,
                ),
                ("front_side_beam_beam_solar_reflectance", "missing", 0_usize),
                ("back_side_beam_beam_solar_reflectance", "missing", 0_usize),
                ("diffuse_diffuse_solar_transmittance", "auto", 0_usize),
                ("diffuse_diffuse_solar_transmittance", "auto", 1_usize),
                ("diffuse_diffuse_solar_transmittance", "auto", 2_usize),
                (
                    "front_side_diffuse_diffuse_solar_reflectance",
                    "auto",
                    0_usize,
                ),
                (
                    "front_side_diffuse_diffuse_solar_reflectance",
                    "auto",
                    1_usize,
                ),
                (
                    "front_side_diffuse_diffuse_solar_reflectance",
                    "auto",
                    2_usize,
                ),
                (
                    "back_side_diffuse_diffuse_solar_reflectance",
                    "auto",
                    0_usize,
                ),
                (
                    "back_side_diffuse_diffuse_solar_reflectance",
                    "auto",
                    1_usize,
                ),
                (
                    "back_side_diffuse_diffuse_solar_reflectance",
                    "auto",
                    2_usize,
                ),
                (
                    "diffuse_diffuse_visible_solar_transmittance",
                    "auto",
                    0_usize,
                ),
                (
                    "diffuse_diffuse_visible_solar_transmittance",
                    "auto",
                    1_usize,
                ),
                (
                    "diffuse_diffuse_visible_solar_transmittance",
                    "auto",
                    2_usize,
                ),
                (
                    "front_side_diffuse_diffuse_visible_solar_reflectance",
                    "auto",
                    0_usize,
                ),
                (
                    "front_side_diffuse_diffuse_visible_solar_reflectance",
                    "auto",
                    1_usize,
                ),
                (
                    "front_side_diffuse_diffuse_visible_solar_reflectance",
                    "auto",
                    2_usize,
                ),
                (
                    "back_side_diffuse_diffuse_visible_solar_reflectance",
                    "auto",
                    0_usize,
                ),
                (
                    "back_side_diffuse_diffuse_visible_solar_reflectance",
                    "auto",
                    1_usize,
                ),
                (
                    "back_side_diffuse_diffuse_visible_solar_reflectance",
                    "auto",
                    2_usize,
                ),
                ("optical_data_type", "mode", 0_usize),
                ("optical_data_type", "mode", 1_usize),
                ("optical_data_type", "mode", 2_usize),
                ("optical_data_type", "spectral", 0_usize),
            ] {
                let mut bad_value = glazing_eql_value.clone();
                match kind {
                    "kind" => {
                        bad_value[GLAZING_EQL]["\u{a0}"][field] = json!(false);
                    }
                    "missing" => {
                        bad_value[GLAZING_EQL]["\u{a0}"]
                            .as_object_mut()
                            .expect("full object")
                            .remove(field);
                    }
                    "auto" => {
                        bad_value[GLAZING_EQL]["\u{a0}"][field] = json!(match probe {
                            0 => "autocalculate",
                            1 => " Autocalculate ",
                            _ => " ",
                        });
                    }
                    "mode" => {
                        bad_value[GLAZING_EQL]["\u{a0}"][field] = json!(match probe {
                            0 => "spectralaverage",
                            1 => " SpectralAverage ",
                            _ => "UnknownMode",
                        });
                    }
                    _ => {
                        bad_value[GLAZING_EQL]["\u{a0}"][field] = json!("Spectral");
                    }
                }
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&bad_value.to_string())
                            .expect("original-invalid Raw")
                    )
                    .is_err_and(|error| error.contains("Source GlazingEQL requires")),
                    "GlazingEQL original boundary {field}/{kind}/{probe}"
                );
            }
            let opaque_glazing_eql = glazing_eql_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowGlazingEquivalentLayer)
                .expect("held opaque material");
            for case in 0_usize..12 {
                let mut bad_model = glazing_eql_model.clone();
                match case {
                    0 => bad_model.materials.retain(|row| {
                        row.kind() != ep_model::MaterialKind::WindowGlazingEquivalentLayer
                    }),
                    1 => {
                        let mut extra = bad_model.materials[glazing_eql_a_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA GLAZING EQL".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[glazing_eql_a_position].id = opaque_glazing_eql.id,
                    3 => {
                        bad_model.materials[glazing_eql_a_position].definition =
                            opaque_glazing_eql.definition
                    }
                    4 => {
                        bad_model.materials[glazing_eql_a_position].name =
                            ep_model::NormalizedName("WRONG GLAZING EQL".into())
                    }
                    5 => {
                        bad_model.materials[glazing_eql_a_position].id =
                            ep_model::MaterialId(u32::MAX)
                    }
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != glazing_eql_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((glazing_eql_a, "\u{b}".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA GLAZING_EQL ALIAS ", glazing_eql_a);
                    }
                    9 => {
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == glazing_eql_a)
                            .expect("A report tuple")
                            .1 = "\u{a0}".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == glazing_eql_b)
                            .expect("B report tuple")
                            .1 = "\u{b}".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => {
                        bad_model.material_names = ep_model::NameMap::source_exact_namespace();
                        bad_model.materials.iter().for_each(|row| {
                            let id = if row.id == glazing_eql_a {
                                glazing_eql_b
                            } else if row.id == glazing_eql_b {
                                glazing_eql_a
                            } else {
                                row.id
                            };
                            let _ = bad_model.material_names.insert(&row.name.0, id);
                        });
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &glazing_eql_raw,
                        &glazing_eql_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&glazing_eql_admission)
                    )
                    .allows_rust_runtime(),
                    "GlazingEQL custody case {case}"
                );
            }
            let glazing_eql_construction = glazing_eql_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .expect("real opaque Construction");
            for case in 0_usize..11 {
                let mut used_value = glazing_eql_value.clone();
                let mut fields =
                    serde_json::Map::from_iter([("outside_layer".into(), json!("Opaque"))]);
                let key = if case == 0 || case == 10 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", case + 1)
                };
                fields.insert(
                    key,
                    json!(if case.is_multiple_of(2) {
                        "\u{b}"
                    } else {
                        "\u{a0}"
                    }),
                );
                used_value["Construction"]["GLAZING_EQL USED"] = serde_json::Value::Object(fields);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&used_value.to_string())
                            .expect("original active GLAZING_EQL layer")
                    )
                    .is_err_and(|error| error.contains(
                        "Source unused GlazingEQL excludes every Construction layer reference"
                    )),
                    "Raw GLAZING_EQL layer {case}"
                );
                let mut used_model = glazing_eql_model.clone();
                let construction = &mut used_model.constructions[glazing_eql_construction];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(glazing_eql_a);
                } else {
                    construction.layers.resize(case + 1, opaque_glazing_eql.id);
                    construction.layers[case] = if case.is_multiple_of(2) {
                        glazing_eql_a
                    } else {
                        glazing_eql_b
                    };
                }
                let used_assessment = crate::support::assess_support_with_source_admission(
                    &glazing_eql_raw,
                    &glazing_eql_compiled.report,
                    Some(&used_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&glazing_eql_admission),
                );
                assert!(
                    !used_assessment.allows_rust_runtime()
                        && used_assessment
                            .unsupported_objects
                            .iter()
                            .any(|entry| entry.object_type == GLAZING_EQL),
                    "typed/effective GLAZING_EQL layer {case}"
                );
            }
        }

        // Unused Gap:EQL uses the existing genuine DSOA/Ideal/OA scene; no Material or Window owner here.
        // Public3A12N definition, original Raw/IP tokens and plain-f64 projection are distinct observations.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            const GAP_EQL: &str = "WindowMaterial:Gap:EquivalentLayer";
            let mut gap_eql_value = curve_value.clone();
            gap_eql_value[GAP_EQL] = json!({
                "\u{b}":{"gas_type":"CUSTOM","thickness":u64::MAX,"gap_vent_type":"VentedIndoor",
                    "conductivity_coefficient_a":0.5,"viscosity_coefficient_a":0.125,"specific_heat_coefficient_a":2.0,
                    "molecular_weight":20.0,"specific_heat_ratio":u64::MAX,"name":false,"idf_order":[],"metadata":null},
                "\u{a0}":{"gas_type":"CUSTOM","thickness":0.125,"gap_vent_type":"VentedOutdoor",
                    "conductivity_coefficient_a":0.25,"conductivity_coefficient_b":-0.0,"conductivity_coefficient_c":0,
                    "viscosity_coefficient_a":0.125,"viscosity_coefficient_b":-0.25,"viscosity_coefficient_c":0.0625,
                    "specific_heat_coefficient_a":2.0,"specific_heat_coefficient_b":-0.5,"specific_heat_coefficient_c":0.25,
                    "molecular_weight":200.0,"specific_heat_ratio":1.8446744073709552e19,
                    "name":false,"idf_order":u64::MAX,"idf_max_fields":{"ignored":true},"idf_max_extensible_fields":null,
                    "extra_string":"unconsumed","metadata":{"retained":[null,true,1.25,"mixed",{"nested":u64::MAX}]}}
            });
            let gap_eql_text = gap_eql_value.to_string().replacen(
                "\"conductivity_coefficient_c\":0,",
                "\"conductivity_coefficient_c\":-0,",
                1,
            );
            let gap_eql_raw = ep_raw_model::parse_epjson_str_for_source(&gap_eql_text)
                .expect("original GapEQL Number tokens");
            let gap_eql_compiled = ep_compiler::compile_raw_model_for_source(&gap_eql_raw);
            assert!(
                !gap_eql_compiled.has_errors(),
                "{:?}",
                gap_eql_compiled.report.diagnostics
            );
            let gap_eql_model = gap_eql_compiled
                .model
                .as_ref()
                .expect("two genuine GapEQL typed identities");
            let gap_eql_a = gap_eql_model
                .material_names
                .resolve("\u{b}")
                .expect("untrimmed VT identity");
            let gap_eql_b = gap_eql_model
                .material_names
                .resolve("\u{a0}")
                .expect("untrimmed NBSP identity");
            assert!(gap_eql_a != gap_eql_b && gap_eql_model.material_names.is_source_exact_namespace()
                && gap_eql_model.material_names.resolve("").is_none()
                && gap_eql_model.materials.iter().filter(|row|row.kind() == ep_model::MaterialKind::WindowGapEquivalentLayer).count() == 2
                && gap_eql_model.materials.iter().any(|row|row.id == gap_eql_a && row.name.0 == "\u{b}" && matches!(row.definition,ep_model::MaterialDefinition::WindowGapEquivalentLayer(_)))
                && gap_eql_model.materials.iter().any(|row|row.id == gap_eql_b && row.name.0 == "\u{a0}" && matches!(row.definition,ep_model::MaterialDefinition::WindowGapEquivalentLayer(_)))
                && gap_eql_model.ctf_report_names.materials.contains(&(gap_eql_a,"\u{b}".into())) && gap_eql_model.ctf_report_names.materials.contains(&(gap_eql_b,"\u{a0}".into())));
            let mut gap_eql_ip = crate::ordinary_input_processor::resolve_source_b(&gap_eql_raw)
                .expect("actual Raw/IP");
            let args = gap_eql_ip
                .get_object_def_max_args(GAP_EQL)
                .expect("public3A12N definition, not private item counts");
            let mut stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            gap_eql_ip
                .report_source_idf_records_stats(&mut stats)
                .expect("actual object census");
            assert!(
                (
                    args.alphas,
                    args.numerics,
                    args.total().expect("fixed max args")
                ) == (3, 12, 15)
                    && gap_eql_ip.get_num_objects_found(GAP_EQL) == 2
                    && gap_eql_ip
                        .source_raw_object_names(GAP_EQL)
                        .expect("actual signed-i8 name order")
                        .iter()
                        .map(String::as_str)
                        .eq(["\u{a0}", "\u{b}"])
                    && gap_eql_ip.is_object_unused(GAP_EQL, "\u{b}")
                    && gap_eql_ip.is_object_unused(GAP_EQL, "\u{a0}")
                    && usize::try_from(stats.number_input_records).expect("nonnegative census")
                        == gap_eql_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let original = gap_eql_ip
                .source_raw_object_fields(GAP_EQL, "\u{a0}")
                .expect("full original custody");
            let minimum = gap_eql_ip
                .source_raw_object_fields(GAP_EQL, "\u{b}")
                .expect("partial original custody");
            let full_raw = &gap_eql_raw.objects[&ep_raw_model::ObjectType(GAP_EQL.into())]
                [&ep_raw_model::ObjectName("\u{a0}".into())]
                .fields;
            let min_raw = &gap_eql_raw.objects[&ep_raw_model::ObjectType(GAP_EQL.into())]
                [&ep_raw_model::ObjectName("\u{b}".into())]
                .fields;
            assert!(
                matches!(min_raw.get(&ep_raw_model::FieldName("thickness".into())),Some(ep_raw_model::RawValue::Number(token)) if token == "18446744073709551615")
                    && matches!(minimum.get("thickness"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(minimum.get("specific_heat_ratio"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(
                        ep_model::classify_source_json_number("18446744073709551615"),
                        Ok(ep_model::SourceJsonNumber::Integer(-1))
                    )
                    && matches!(original.get("conductivity_coefficient_b"),Some(S::Number(token)) if token == "-0.0")
                    && matches!(original.get("conductivity_coefficient_c"),Some(S::Number(token)) if token == "-0")
                    && matches!(ep_model::classify_source_json_number("-0.0"),Ok(ep_model::SourceJsonNumber::Float(v)) if v.to_bits() == (-0.0_f64).to_bits())
                    && matches!(
                        ep_model::classify_source_json_number("-0"),
                        Ok(ep_model::SourceJsonNumber::Integer(0))
                    )
                    && full_raw.iter().all(|(field, value)| match value {
                        ep_raw_model::RawValue::Number(token) =>
                            matches!(original.get(&field.0),Some(S::Number(got)) if got == token),
                        ep_raw_model::RawValue::String(text) =>
                            matches!(original.get(&field.0),Some(S::String(got)) if got == text),
                        _ => true,
                    })
                    && matches!(original.get("name"), Some(S::Bool(false)))
                    && matches!(original.get("idf_order"),Some(S::Number(token)) if token == "18446744073709551615")
                    && matches!(original.get("idf_max_fields"), Some(S::Object(_)))
                    && matches!(original.get("idf_max_extensible_fields"), Some(S::Null))
                    && matches!(original.get("extra_string"),Some(S::String(v)) if v == "unconsumed")
                    && matches!(original.get("metadata"),Some(S::Object(fields)) if matches!(fields.get("retained"),Some(S::Array(rows)) if rows.len() == 5))
            );
            let projection =
                crate::ordinary_material_input::resolve_source(&gap_eql_raw, &gap_eql_ip)
                    .expect("real GapEQL Source projection");
            let full = projection
                .window_gap_equivalent_layer
                .iter()
                .find(|row| row.object_name == "\u{a0}")
                .expect("full projection");
            let minimum_projection = projection
                .window_gap_equivalent_layer
                .iter()
                .find(|row| row.object_name == "\u{b}")
                .expect("partial projection");
            let numeric_fields = [
                "thickness",
                "conductivity_coefficient_a",
                "conductivity_coefficient_b",
                "conductivity_coefficient_c",
                "viscosity_coefficient_a",
                "viscosity_coefficient_b",
                "viscosity_coefficient_c",
                "specific_heat_coefficient_a",
                "specific_heat_coefficient_b",
                "specific_heat_coefficient_c",
                "molecular_weight",
                "specific_heat_ratio",
            ];
            let mut wrong_count = gap_eql_raw.clone();
            wrong_count
                .objects
                .get_mut(&ep_raw_model::ObjectType(GAP_EQL.into()))
                .expect("family")
                .remove(&ep_raw_model::ObjectName("\u{b}".into()));
            let mut wrong_name = gap_eql_raw.clone();
            let wrong_rows = wrong_name
                .objects
                .get_mut(&ep_raw_model::ObjectType(GAP_EQL.into()))
                .expect("family");
            let row = wrong_rows
                .remove(&ep_raw_model::ObjectName("\u{b}".into()))
                .expect("same row");
            wrong_rows.insert(
                ep_raw_model::ObjectName("WRONG GAP EQL IP NAME".into()),
                row,
            );
            assert!(projection.source_is_epjson && projection.window_gap_equivalent_layer.len() == 2 && full.fields.len() == 14 && minimum_projection.fields.len() == 8
                && crate::ordinary_material_input::resolve_source(&wrong_count,&gap_eql_ip).is_err() && crate::ordinary_material_input::resolve_source(&wrong_name,&gap_eql_ip).is_err()
                && projection.window_gap_equivalent_layer.iter().all(|row|row.idf_max_fields.is_none() && row.fields.keys().all(|f|crate::ordinary_equipment_input::source_b_gap_equivalent_layer_known_field(f)))
                && numeric_fields.iter().zip([0.125_f64,0.25,-0.0,0.0,0.125,-0.25,0.0625,2.0,-0.5,0.25,200.0,18446744073709551616.0])
                    .all(|(field,expected)|matches!(full.fields.get(*field),Some(P::Number(actual)) if actual.to_bits() == expected.to_bits()))
                && matches!(minimum_projection.fields.get("thickness"),Some(P::Number(value)) if value.to_bits() == (-1.0_f64).to_bits())
                && matches!(minimum_projection.fields.get("specific_heat_ratio"),Some(P::Number(value)) if value.to_bits() == (-1.0_f64).to_bits())
                && matches!(full.fields.get("gas_type"),Some(P::String(s)) if s == "CUSTOM") && matches!(full.fields.get("gap_vent_type"),Some(P::String(s)) if s == "VentedOutdoor")
                && gap_eql_model.materials.iter().find(|r|r.id == gap_eql_a).expect("same advisory A").as_window_gap_equivalent_layer().is_some_and(|r|
                    r.thickness_m.to_bits() == 18446744073709551616.0_f64.to_bits() && r.properties.specific_heat_ratio.to_bits() == 18446744073709551616.0_f64.to_bits())
                && ["","\n"," \t\r\u{c}"].iter().all(|name| {
                    let mut value = gap_eql_value.clone();let row = value[GAP_EQL].as_object_mut().expect("family").remove("\u{b}").expect("row");
                    value[GAP_EQL].as_object_mut().expect("family").insert((*name).into(),row);
                    crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&value.to_string()).expect("bad Raw name"))
                        .is_err_and(|error|error.contains("Source GapEQL requires original RE2 material name"))
                }));
            let mut gap_eql_trace = inspect_porting_scope(
                &gap_eql_raw,
                &gap_eql_compiled.report,
                Some(gap_eql_model),
                PortingScope::B,
            );
            assert!(gap_eql_trace.admissible, "{:?}", gap_eql_trace.violations);
            gap_eql_trace.environment = Some(
                super::environment::environment_from_weather(
                    gap_eql_model,
                    &weather,
                    "same GapEQL Source weather",
                )
                .expect("actual Source environment"),
            );
            let gap_eql_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &gap_eql_raw,
                &gap_eql_trace,
            )
            .expect("validated Source GAP_EQL scope");
            let mut permuted_gap_eql_model = gap_eql_model.clone();
            let gap_eql_a_position = gap_eql_model
                .materials
                .iter()
                .position(|row| row.id == gap_eql_a)
                .expect("A arena row");
            let gap_eql_b_position = gap_eql_model
                .materials
                .iter()
                .position(|row| row.id == gap_eql_b)
                .expect("B arena row");
            permuted_gap_eql_model
                .materials
                .swap(gap_eql_a_position, gap_eql_b_position);
            assert!(
                crate::support::assess_support_with_source_admission(
                    &gap_eql_raw,
                    &gap_eql_compiled.report,
                    Some(gap_eql_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&gap_eql_admission)
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &gap_eql_raw,
                        &gap_eql_compiled.report,
                        Some(&permuted_gap_eql_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&gap_eql_admission)
                    )
                    .allows_rust_runtime()
            );
            let gap_eql_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                r#"{"Zone":{"Zone":{}},"WindowMaterial:Gap:EquivalentLayer":{"IDF GAP EQL":{"gas_type":"AIR","thickness":0.125,"gap_vent_type":"Sealed"}}}"#,
                "Zone,Zone;WindowMaterial:Gap:EquivalentLayer,IDF GAP EQL,AIR,0.125,Sealed;",
            ).expect("matched Zone+GapEQL IDF four material operands");
            let gap_eql_idf_compiled = ep_compiler::compile_raw_model(&gap_eql_idf);
            let source_a_base =
                ep_raw_model::parse_epjson_str_for_source(&input(PortingScope::A).to_string())
                    .expect("valid SourceA control");
            let mut a_with_gap = source_a_base.clone();
            a_with_gap.objects.insert(
                ep_raw_model::ObjectType(GAP_EQL.into()),
                gap_eql_raw
                    .objects
                    .get(&ep_raw_model::ObjectType(GAP_EQL.into()))
                    .expect("same Gap family only")
                    .clone(),
            );
            let mut controlled = gap_eql_value.clone();
            controlled["WindowShadingControl"] = json!({"GAP CONTROL":{"shading_type":"ExteriorShade","shading_device_material_name":"\u{b}"}});
            let mut active = gap_eql_value.clone();
            active["Construction:WindowEquivalentLayer"] =
                json!({"ACTIVE GAP EQL":{"outside_layer":"\u{b}"}});
            let mut ems = gap_eql_value.clone();
            ems["EnergyManagementSystem:ConstructionIndexVariable"] =
                json!({"HELD_GAP_EMS":{"construction_object_name":"Opaque"}});
            assert!(!crate::support::assess_support_with_source_admission(&gap_eql_raw,&gap_eql_compiled.report,Some(gap_eql_model),RunMode::Compatibility,PartialRunPolicy::Deny,RunOutputFormat::RustNative,TraceLevel::Normal,None).allows_rust_runtime()
                && crate::ordinary_equipment_input::resolve_source_a(&source_a_base).is_ok()
                && crate::ordinary_equipment_input::resolve_source_a(&a_with_gap).is_err_and(|e|e == "ordinary equipment excludes positive family WindowMaterial:Gap:EquivalentLayer")
                && crate::ordinary_equipment_input::resolve(&gap_eql_raw).is_err()
                && !gap_eql_idf.is_epjson() && !gap_eql_idf_compiled.has_errors()
                && gap_eql_idf_compiled.model.as_ref().is_some_and(|model|model.materials.len() == 1 && model.materials[0].kind() == ep_model::MaterialKind::WindowGapEquivalentLayer)
                && crate::ordinary_equipment_input::resolve_source_b(&gap_eql_idf).is_err_and(|e|e == "Source unused GapEQL requires direct epJSON")
                && crate::ordinary_equipment_input::resolve_source_a(&gap_eql_idf).is_err()
                && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&controlled.to_string()).expect("control Raw")).is_err()
                && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&active.to_string()).expect("active Raw")).is_err()
                && crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&ems.to_string()).expect("EMS Raw")).is_err());
            for (field, kind, probe) in [
                ("thickness", "kind", 0_usize),
                ("thickness", "kind", 1_usize),
                ("thickness", "kind", 2_usize),
                ("thickness", "kind", 3_usize),
                ("thickness", "kind", 4_usize),
                ("conductivity_coefficient_a", "kind", 0_usize),
                ("conductivity_coefficient_a", "kind", 1_usize),
                ("conductivity_coefficient_a", "kind", 2_usize),
                ("conductivity_coefficient_a", "kind", 3_usize),
                ("conductivity_coefficient_a", "kind", 4_usize),
                ("conductivity_coefficient_b", "kind", 0_usize),
                ("conductivity_coefficient_b", "kind", 1_usize),
                ("conductivity_coefficient_b", "kind", 2_usize),
                ("conductivity_coefficient_b", "kind", 3_usize),
                ("conductivity_coefficient_b", "kind", 4_usize),
                ("conductivity_coefficient_c", "kind", 0_usize),
                ("conductivity_coefficient_c", "kind", 1_usize),
                ("conductivity_coefficient_c", "kind", 2_usize),
                ("conductivity_coefficient_c", "kind", 3_usize),
                ("conductivity_coefficient_c", "kind", 4_usize),
                ("viscosity_coefficient_a", "kind", 0_usize),
                ("viscosity_coefficient_a", "kind", 1_usize),
                ("viscosity_coefficient_a", "kind", 2_usize),
                ("viscosity_coefficient_a", "kind", 3_usize),
                ("viscosity_coefficient_a", "kind", 4_usize),
                ("viscosity_coefficient_b", "kind", 0_usize),
                ("viscosity_coefficient_b", "kind", 1_usize),
                ("viscosity_coefficient_b", "kind", 2_usize),
                ("viscosity_coefficient_b", "kind", 3_usize),
                ("viscosity_coefficient_b", "kind", 4_usize),
                ("viscosity_coefficient_c", "kind", 0_usize),
                ("viscosity_coefficient_c", "kind", 1_usize),
                ("viscosity_coefficient_c", "kind", 2_usize),
                ("viscosity_coefficient_c", "kind", 3_usize),
                ("viscosity_coefficient_c", "kind", 4_usize),
                ("specific_heat_coefficient_a", "kind", 0_usize),
                ("specific_heat_coefficient_a", "kind", 1_usize),
                ("specific_heat_coefficient_a", "kind", 2_usize),
                ("specific_heat_coefficient_a", "kind", 3_usize),
                ("specific_heat_coefficient_a", "kind", 4_usize),
                ("specific_heat_coefficient_b", "kind", 0_usize),
                ("specific_heat_coefficient_b", "kind", 1_usize),
                ("specific_heat_coefficient_b", "kind", 2_usize),
                ("specific_heat_coefficient_b", "kind", 3_usize),
                ("specific_heat_coefficient_b", "kind", 4_usize),
                ("specific_heat_coefficient_c", "kind", 0_usize),
                ("specific_heat_coefficient_c", "kind", 1_usize),
                ("specific_heat_coefficient_c", "kind", 2_usize),
                ("specific_heat_coefficient_c", "kind", 3_usize),
                ("specific_heat_coefficient_c", "kind", 4_usize),
                ("molecular_weight", "kind", 0_usize),
                ("molecular_weight", "kind", 1_usize),
                ("molecular_weight", "kind", 2_usize),
                ("molecular_weight", "kind", 3_usize),
                ("molecular_weight", "kind", 4_usize),
                ("specific_heat_ratio", "kind", 0_usize),
                ("specific_heat_ratio", "kind", 1_usize),
                ("specific_heat_ratio", "kind", 2_usize),
                ("specific_heat_ratio", "kind", 3_usize),
                ("specific_heat_ratio", "kind", 4_usize),
                ("gas_type", "alpha_kind", 0_usize),
                ("gas_type", "alpha_kind", 1_usize),
                ("gas_type", "alpha_kind", 2_usize),
                ("gas_type", "alpha_kind", 3_usize),
                ("gas_type", "alpha_kind", 4_usize),
                ("gap_vent_type", "alpha_kind", 0_usize),
                ("gap_vent_type", "alpha_kind", 1_usize),
                ("gap_vent_type", "alpha_kind", 2_usize),
                ("gap_vent_type", "alpha_kind", 3_usize),
                ("gap_vent_type", "alpha_kind", 4_usize),
                ("gas_type", "missing", 0_usize),
                ("thickness", "missing", 0_usize),
                ("gap_vent_type", "missing", 0_usize),
                ("thickness", "positive", 0_usize),
                ("thickness", "positive", 1_usize),
                ("thickness", "positive", 2_usize),
                ("viscosity_coefficient_a", "positive", 0_usize),
                ("viscosity_coefficient_a", "positive", 1_usize),
                ("viscosity_coefficient_a", "positive", 2_usize),
                ("specific_heat_coefficient_a", "positive", 0_usize),
                ("specific_heat_coefficient_a", "positive", 1_usize),
                ("specific_heat_coefficient_a", "positive", 2_usize),
                ("specific_heat_ratio", "ratio", 0_usize),
                ("specific_heat_ratio", "ratio", 1_usize),
                ("specific_heat_ratio", "ratio", 2_usize),
                ("molecular_weight", "weight", 0_usize),
                ("molecular_weight", "weight", 1_usize),
                ("thickness", "auto", 0_usize),
                ("thickness", "auto", 1_usize),
                ("thickness", "auto", 2_usize),
                ("conductivity_coefficient_a", "auto", 0_usize),
                ("conductivity_coefficient_a", "auto", 1_usize),
                ("conductivity_coefficient_a", "auto", 2_usize),
                ("conductivity_coefficient_b", "auto", 0_usize),
                ("conductivity_coefficient_b", "auto", 1_usize),
                ("conductivity_coefficient_b", "auto", 2_usize),
                ("conductivity_coefficient_c", "auto", 0_usize),
                ("conductivity_coefficient_c", "auto", 1_usize),
                ("conductivity_coefficient_c", "auto", 2_usize),
                ("viscosity_coefficient_a", "auto", 0_usize),
                ("viscosity_coefficient_a", "auto", 1_usize),
                ("viscosity_coefficient_a", "auto", 2_usize),
                ("viscosity_coefficient_b", "auto", 0_usize),
                ("viscosity_coefficient_b", "auto", 1_usize),
                ("viscosity_coefficient_b", "auto", 2_usize),
                ("viscosity_coefficient_c", "auto", 0_usize),
                ("viscosity_coefficient_c", "auto", 1_usize),
                ("viscosity_coefficient_c", "auto", 2_usize),
                ("specific_heat_coefficient_a", "auto", 0_usize),
                ("specific_heat_coefficient_a", "auto", 1_usize),
                ("specific_heat_coefficient_a", "auto", 2_usize),
                ("specific_heat_coefficient_b", "auto", 0_usize),
                ("specific_heat_coefficient_b", "auto", 1_usize),
                ("specific_heat_coefficient_b", "auto", 2_usize),
                ("specific_heat_coefficient_c", "auto", 0_usize),
                ("specific_heat_coefficient_c", "auto", 1_usize),
                ("specific_heat_coefficient_c", "auto", 2_usize),
                ("molecular_weight", "auto", 0_usize),
                ("molecular_weight", "auto", 1_usize),
                ("molecular_weight", "auto", 2_usize),
                ("specific_heat_ratio", "auto", 0_usize),
                ("specific_heat_ratio", "auto", 1_usize),
                ("specific_heat_ratio", "auto", 2_usize),
                ("gas_type", "enum", 0_usize),
                ("gas_type", "enum", 1_usize),
                ("gas_type", "enum", 2_usize),
                ("gas_type", "enum", 3_usize),
                ("gas_type", "enum", 4_usize),
                ("gap_vent_type", "enum", 0_usize),
                ("gap_vent_type", "enum", 1_usize),
                ("gap_vent_type", "enum", 2_usize),
                ("gap_vent_type", "enum", 3_usize),
                ("gap_vent_type", "enum", 4_usize),
            ] {
                let mut value = gap_eql_value.clone();
                match kind {
                    "kind" | "alpha_kind" => {
                        value[GAP_EQL]["\u{a0}"][field] = match probe {
                            0 => json!(null),
                            1 => json!(false),
                            2 => {
                                if kind == "kind" {
                                    json!("0.125")
                                } else {
                                    json!(0.125)
                                }
                            }
                            3 => json!([]),
                            _ => json!({}),
                        };
                    }
                    "missing" => {
                        value[GAP_EQL]["\u{a0}"]
                            .as_object_mut()
                            .expect("row")
                            .remove(field);
                    }
                    "positive" => {
                        value[GAP_EQL]["\u{a0}"][field] = match probe {
                            0 => json!(0),
                            1 => json!(-0.0),
                            _ => json!(-1),
                        };
                    }
                    "ratio" => {
                        value[GAP_EQL]["\u{a0}"][field] = match probe {
                            0 => json!(1),
                            1 => json!(0),
                            _ => json!(-1),
                        };
                    }
                    "weight" => {
                        value[GAP_EQL]["\u{a0}"][field] = if probe == 0 {
                            json!(19.999)
                        } else {
                            json!(200.001)
                        };
                    }
                    "auto" => {
                        value[GAP_EQL]["\u{a0}"][field] = json!(match probe {
                            0 => "Autocalculate",
                            1 => "Autosize",
                            _ => "",
                        });
                    }
                    _ => {
                        value[GAP_EQL]["\u{a0}"][field] = json!(if field == "gas_type" {
                            match probe {
                                0 => "air",
                                1 => "Air",
                                2 => "",
                                3 => " ",
                                _ => "STEAM",
                            }
                        } else {
                            match probe {
                                0 => "sealed",
                                1 => "VENTEDINDOOR",
                                2 => "",
                                3 => " ",
                                _ => "UnknownVent",
                            }
                        });
                    }
                }
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&value.to_string())
                            .expect("original-invalid GapEQL Raw")
                    )
                    .is_err_and(|e| e.contains("Source GapEQL requires")),
                    "GapEQL original {field}/{kind}/{probe}"
                );
            }
            let opaque_gap_eql = gap_eql_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::WindowGapEquivalentLayer)
                .expect("held opaque material");
            for case in 0_usize..12 {
                let mut bad_model = gap_eql_model.clone();
                match case {
                    0 => bad_model.materials.retain(|row| {
                        row.kind() != ep_model::MaterialKind::WindowGapEquivalentLayer
                    }),
                    1 => {
                        let mut extra = bad_model.materials[gap_eql_a_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA GAP EQL".into());
                        bad_model.materials.push(extra);
                    }
                    2 => bad_model.materials[gap_eql_a_position].id = opaque_gap_eql.id,
                    3 => {
                        bad_model.materials[gap_eql_a_position].definition =
                            opaque_gap_eql.definition
                    }
                    4 => {
                        bad_model.materials[gap_eql_a_position].name =
                            ep_model::NormalizedName("WRONG GAP EQL".into())
                    }
                    5 => {
                        bad_model.materials[gap_eql_a_position].id = ep_model::MaterialId(u32::MAX)
                    }
                    6 => bad_model
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != gap_eql_a),
                    7 => bad_model
                        .ctf_report_names
                        .materials
                        .push((gap_eql_a, "\u{b}".into())),
                    8 => {
                        let _ = bad_model
                            .material_names
                            .insert(" EXTRA GAP_EQL ALIAS ", gap_eql_a);
                    }
                    9 => {
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == gap_eql_a)
                            .expect("A report tuple")
                            .1 = "\u{a0}".into();
                        bad_model
                            .ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == gap_eql_b)
                            .expect("B report tuple")
                            .1 = "\u{b}".into();
                    }
                    10 => {
                        bad_model.material_names = ep_model::NameMap::default();
                        bad_model.materials.iter().for_each(|row| {
                            let _ = bad_model.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => {
                        bad_model.material_names = ep_model::NameMap::source_exact_namespace();
                        bad_model.materials.iter().for_each(|row| {
                            let id = if row.id == gap_eql_a {
                                gap_eql_b
                            } else if row.id == gap_eql_b {
                                gap_eql_a
                            } else {
                                row.id
                            };
                            let _ = bad_model.material_names.insert(&row.name.0, id);
                        });
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &gap_eql_raw,
                        &gap_eql_compiled.report,
                        Some(&bad_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&gap_eql_admission)
                    )
                    .allows_rust_runtime(),
                    "GapEQL custody case {case}"
                );
            }
            let gap_eql_construction = gap_eql_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .expect("real opaque Construction");
            for case in 0_usize..11 {
                let mut used_value = gap_eql_value.clone();
                let mut fields =
                    serde_json::Map::from_iter([("outside_layer".into(), json!("Opaque"))]);
                let key = if case == 0 || case == 10 {
                    "outside_layer".into()
                } else {
                    format!("layer_{}", case + 1)
                };
                fields.insert(
                    key,
                    json!(if case.is_multiple_of(2) {
                        "\u{b}"
                    } else {
                        "\u{a0}"
                    }),
                );
                used_value["Construction"]["GAP_EQL USED"] = serde_json::Value::Object(fields);
                assert!(
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&used_value.to_string())
                            .expect("original active GAP_EQL layer")
                    )
                    .is_err_and(|error| error.contains(
                        "Source unused GapEQL excludes every Construction layer reference"
                    )),
                    "Raw GAP_EQL layer {case}"
                );
                let mut used_model = gap_eql_model.clone();
                let construction = &mut used_model.constructions[gap_eql_construction];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(gap_eql_a);
                } else {
                    construction.layers.resize(case + 1, opaque_gap_eql.id);
                    construction.layers[case] = if case.is_multiple_of(2) {
                        gap_eql_a
                    } else {
                        gap_eql_b
                    };
                }
                let used_assessment = crate::support::assess_support_with_source_admission(
                    &gap_eql_raw,
                    &gap_eql_compiled.report,
                    Some(&used_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&gap_eql_admission),
                );
                assert!(
                    !used_assessment.allows_rust_runtime()
                        && used_assessment
                            .unsupported_objects
                            .iter()
                            .any(|entry| entry.object_type == GAP_EQL),
                    "typed/effective GAP_EQL layer {case}"
                );
            }
        }

        // RoofVegetation uses the existing real DSOA/Ideal/OA fixture; no Material or Window context.
        {
            use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as S;
            use ep_runtime::heat_balance::selected_project_data::ProjectInputValue as P;
            const ROOF: &str = "Material:RoofVegetation";
            let mut roof_value = curve_value.clone();
            roof_value[ROOF] = json!({
                "\u{b}":{},
                "\u{a0}":{
                    "height_of_plants":0.45,"leaf_area_index":2,"leaf_reflectivity":0.24,"leaf_emissivity":0.96,
                    "minimum_stomatal_resistance":190.0,"soil_layer_name":" \t ","roughness":"MediumSmooth",
                    "thickness":0.13,"conductivity_of_dry_soil":0.37,"density_of_dry_soil":1130.0,
                    "specific_heat_of_dry_soil":1230,"thermal_absorptance":0.93,"solar_absorptance":0.73,
                    "visible_absorptance":0.77,"saturation_volumetric_moisture_content_of_the_soil_layer":0.33,
                    "residual_volumetric_moisture_content_of_the_soil_layer":0.03,"initial_volumetric_moisture_content_of_the_soil_layer":0.4,
                    "moisture_diffusion_calculation_method":"Simple","name":false,"idf_order":u64::MAX,
                    "idf_max_fields":[],"idf_max_extensible_fields":null,"metadata":{"held":[null,true,1.25,"text",{"nested":u64::MAX}]}
                }
            });
            let roof_raw = ep_raw_model::parse_epjson_str_for_source(&roof_value.to_string())
                .expect("two original Roof rows");
            let roof_compiled = ep_compiler::compile_raw_model_for_source(&roof_raw);
            let roof_model = roof_compiled
                .model
                .as_ref()
                .expect("genuine Source Roof typed identities");
            let roof_a = roof_model
                .material_names
                .resolve("\u{b}")
                .expect("untrimmed VT Roof");
            let roof_b = roof_model
                .material_names
                .resolve("\u{a0}")
                .expect("untrimmed NBSP Roof");
            // U01: the real Source compiler/loader namespace and report identity are global, not arena indices.
            assert!(
                !roof_compiled.has_errors()
                    && roof_a != roof_b
                    && roof_model.material_names.is_source_exact_namespace()
                    && roof_model
                        .materials
                        .iter()
                        .filter(|row| row.kind() == ep_model::MaterialKind::RoofVegetation)
                        .count()
                        == 2
                    && roof_model.material_names.resolve("").is_none()
                    && roof_model.materials.iter().any(|row| row.id == roof_a
                        && row.name.0 == "\u{b}"
                        && row.kind() == ep_model::MaterialKind::RoofVegetation)
                    && roof_model.materials.iter().any(|row| row.id == roof_b
                        && row.name.0 == "\u{a0}"
                        && row.kind() == ep_model::MaterialKind::RoofVegetation)
                    && roof_model
                        .ctf_report_names
                        .materials
                        .contains(&(roof_a, "\u{b}".into()))
                    && roof_model
                        .ctf_report_names
                        .materials
                        .contains(&(roof_b, "\u{a0}".into()))
            );
            let mut roof_ip = crate::ordinary_input_processor::resolve_source_b(&roof_raw)
                .expect("same original Roof Raw/IP");
            let args = roof_ip
                .get_object_def_max_args(ROOF)
                .expect("public4A15N definition");
            let mut stats =
                ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
            roof_ip
                .report_source_idf_records_stats(&mut stats)
                .expect("real object census");
            // U02: public definition and original unused census only; private item blanks/counts belong to Runtime B1.
            assert!(
                (
                    args.alphas,
                    args.numerics,
                    args.total().expect("fixed definition")
                ) == (4, 15, 19)
                    && roof_ip.get_num_objects_found(ROOF) == 2
                    && roof_ip
                        .source_raw_object_names(ROOF)
                        .expect("signed-i8 Source order")
                        .iter()
                        .map(String::as_str)
                        .eq(["\u{a0}", "\u{b}"])
                    && roof_ip.is_object_unused(ROOF, "\u{b}")
                    && roof_ip.is_object_unused(ROOF, "\u{a0}")
                    && usize::try_from(stats.number_input_records).expect("nonnegative records")
                        == roof_raw
                            .objects
                            .values()
                            .map(|rows| rows.len())
                            .sum::<usize>()
            );
            let original = roof_ip
                .source_raw_object_fields(ROOF, "\u{a0}")
                .expect("original full row");
            let default_original = roof_ip
                .source_raw_object_fields(ROOF, "\u{b}")
                .expect("original empty row");
            let original_raw = &roof_raw.objects[&ep_raw_model::ObjectType(ROOF.into())]
                [&ep_raw_model::ObjectName("\u{a0}".into())]
                .fields;
            let projected = crate::ordinary_material_input::resolve_source(&roof_raw, &roof_ip)
                .expect("real Source material projection");
            let full = projected
                .roof_vegetation
                .iter()
                .find(|row| row.object_name == "\u{a0}")
                .expect("full Roof projection");
            let empty = projected
                .roof_vegetation
                .iter()
                .find(|row| row.object_name == "\u{b}")
                .expect("empty Roof projection");
            let roof_numeric = [
                "height_of_plants",
                "leaf_area_index",
                "leaf_reflectivity",
                "leaf_emissivity",
                "minimum_stomatal_resistance",
                "thickness",
                "conductivity_of_dry_soil",
                "density_of_dry_soil",
                "specific_heat_of_dry_soil",
                "thermal_absorptance",
                "solar_absorptance",
                "visible_absorptance",
                "saturation_volumetric_moisture_content_of_the_soil_layer",
                "residual_volumetric_moisture_content_of_the_soil_layer",
                "initial_volumetric_moisture_content_of_the_soil_layer",
            ];
            let mut wrong_count = roof_raw.clone();
            wrong_count
                .objects
                .get_mut(&ep_raw_model::ObjectType(ROOF.into()))
                .expect("family")
                .remove(&ep_raw_model::ObjectName("\u{b}".into()));
            let mut wrong_name = roof_raw.clone();
            let rows = wrong_name
                .objects
                .get_mut(&ep_raw_model::ObjectType(ROOF.into()))
                .expect("family");
            let row = rows
                .remove(&ep_raw_model::ObjectName("\u{b}".into()))
                .expect("row");
            rows.insert(ep_raw_model::ObjectName("WRONG ROOF IP NAME".into()), row);
            // U03: Raw/IP preserve original number tokens and recursive metadata; projection is only plain f64.
            assert!(default_original.is_empty() && empty.fields.is_empty() && projected.roof_vegetation.len()==2 && projected.source_is_epjson
                && full.fields.len()==18 && full.idf_max_fields.is_none() && empty.idf_max_fields.is_none()
                && matches!(original_raw.get(&ep_raw_model::FieldName("leaf_area_index".into())),Some(ep_raw_model::RawValue::Number(token)) if token=="2")
                && matches!(original.get("leaf_area_index"),Some(S::Number(token)) if token=="2")
                && matches!(ep_model::classify_source_json_number("2"),Ok(ep_model::SourceJsonNumber::Integer(2)))
                && matches!(original.get("minimum_stomatal_resistance"),Some(S::Number(token)) if token=="190.0")
                && matches!(ep_model::classify_source_json_number("190.0"),Ok(ep_model::SourceJsonNumber::Float(v)) if v.to_bits()==190.0_f64.to_bits())
                && matches!(original.get("soil_layer_name"),Some(S::String(s)) if s==" \t ")
                && matches!(original.get("name"),Some(S::Bool(false))) && matches!(original.get("idf_order"),Some(S::Number(t)) if t=="18446744073709551615")
                && matches!(original.get("idf_max_fields"),Some(S::Array(v)) if v.is_empty()) && matches!(original.get("idf_max_extensible_fields"),Some(S::Null))
                && matches!(original.get("metadata"),Some(S::Object(v)) if matches!(v.get("held"),Some(S::Array(items)) if items.len()==5))
                && roof_numeric.iter().zip([0.45_f64,2.0,0.24,0.96,190.0,0.13,0.37,1130.0,1230.0,0.93,0.73,0.77,0.33,0.03,0.4])
                    .all(|(field,want)|matches!(full.fields.get(*field),Some(P::Number(got)) if got.to_bits()==want.to_bits()))
                && full.fields.keys().all(|field|crate::ordinary_equipment_input::source_b_roof_vegetation_known_field(field))
                && crate::ordinary_material_input::resolve_source(&wrong_count,&roof_ip).is_err() && crate::ordinary_material_input::resolve_source(&wrong_name,&roof_ip).is_err());
            let mut legacy_joint_raw = ep_raw_model::parse_epjson_str_for_source(
                r#"{"Material:RoofVegetation":{"LEGACY JOINT ROOF":{}}}"#,
            )
            .expect("standalone legal Legacy joint control");
            legacy_joint_raw
                .objects
                .get_mut(&ep_raw_model::ObjectType(ROOF.into()))
                .expect("Legacy family")
                .get_mut(&ep_raw_model::ObjectName("LEGACY JOINT ROOF".into()))
                .expect("Legacy row")
                .fields = original_raw.clone();
            let legacy = ep_compiler::compile_raw_model(&legacy_joint_raw);
            // U04: Source advisory retains legal excessive initial moisture, Legacy still clamps/warns; metadata remains inert.
            assert!(
                !roof_compiled
                    .report
                    .diagnostics
                    .iter()
                    .any(|row| row.code == "RoofVegetationInitialMoistureReset")
                    && !roof_compiled
                        .report
                        .defaults_applied
                        .iter()
                        .any(|row| row.object_type == ROOF
                            && row.object_name == "\u{a0}"
                            && row.field == "soil_layer_name")
                    && roof_model
                        .materials
                        .iter()
                        .find(|row| row.id == roof_b)
                        .expect("same Roof B")
                        .as_roof_vegetation()
                        .is_some_and(|row| row.initial_volumetric_moisture_content.to_bits()
                            == 0.4_f64.to_bits())
                    && legacy
                        .report
                        .diagnostics
                        .iter()
                        .any(|row| row.code == "RoofVegetationInitialMoistureReset")
                    && legacy.model.as_ref().is_some_and(|model| model
                        .materials
                        .iter()
                        .filter_map(|row| row.as_roof_vegetation())
                        .any(|row| row.initial_volumetric_moisture_content.to_bits()
                            == 0.33_f64.to_bits()))
                    && [
                        json!(null),
                        json!(false),
                        json!(u64::MAX),
                        json!("unconsumed"),
                        json!([]),
                        json!({"nested":[true,null]})
                    ]
                    .iter()
                    .all(|metadata| {
                        let mut value = roof_value.clone();
                        value[ROOF]["\u{a0}"]["metadata"] = metadata.clone();
                        crate::ordinary_equipment_input::resolve_source_b(
                            &ep_raw_model::parse_epjson_str_for_source(&value.to_string())
                                .expect("unknown metadata kinds"),
                        )
                        .is_ok()
                    })
            );
            let mut roof_trace = inspect_porting_scope(
                &roof_raw,
                &roof_compiled.report,
                Some(roof_model),
                PortingScope::B,
            );
            // U05: genuine full scene admission precedes its real weather environment.
            assert!(roof_trace.admissible, "{:?}", roof_trace.violations);
            roof_trace.environment = Some(
                super::environment::environment_from_weather(
                    roof_model,
                    &weather,
                    "same unused Roof Source weather",
                )
                .expect("actual Source environment"),
            );
            let roof_admission = crate::support::SourceSupportAdmission::from_validated_scope(
                &roof_raw,
                &roof_trace,
            )
            .expect("genuine Roof Source scope");
            let roof_a_position = roof_model
                .materials
                .iter()
                .position(|row| row.id == roof_a)
                .expect("Roof A arena row");
            let roof_b_position = roof_model
                .materials
                .iter()
                .position(|row| row.id == roof_b)
                .expect("Roof B arena row");
            let mut permuted = roof_model.clone();
            permuted.materials.swap(roof_a_position, roof_b_position);
            // U06: valid arena permutation keeps global identity, while absence of actual admission remains closed.
            assert!(
                crate::support::assess_support_with_source_admission(
                    &roof_raw,
                    &roof_compiled.report,
                    Some(roof_model),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&roof_admission)
                )
                .allows_rust_runtime()
                    && crate::support::assess_support_with_source_admission(
                        &roof_raw,
                        &roof_compiled.report,
                        Some(&permuted),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&roof_admission)
                    )
                    .allows_rust_runtime()
                    && !crate::support::assess_support_with_source_admission(
                        &roof_raw,
                        &roof_compiled.report,
                        Some(roof_model),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        None
                    )
                    .allows_rust_runtime()
            );
            let source_a_base =
                ep_raw_model::parse_epjson_str_for_source(&input(PortingScope::A).to_string())
                    .expect("valid SourceA control");
            let mut a_with_roof = source_a_base.clone();
            a_with_roof.objects.insert(
                ep_raw_model::ObjectType(ROOF.into()),
                roof_raw.objects[&ep_raw_model::ObjectType(ROOF.into())].clone(),
            );
            let roof_idf = ep_raw_model::parse_epjson_str_with_idf_order(
                r#"{"Zone":{"Zone":{}},"Material:RoofVegetation":{"IDF ROOF":{}}}"#,
                "Zone,Zone;Material:RoofVegetation,IDF ROOF,,,,,,,,,,,,,,,,,,;",
            )
            .expect("matched Zone and Name plus18 blank Roof operands");
            let idf_legacy = ep_compiler::compile_raw_model(&roof_idf);
            // U07: matched SourceA differs only by this family; matched IDF is Legacy-positive before its format fence.
            assert!(crate::ordinary_equipment_input::resolve_source_a(&source_a_base).is_ok()
                && crate::ordinary_equipment_input::resolve_source_a(&a_with_roof).is_err_and(|e|e=="ordinary equipment excludes positive family Material:RoofVegetation")
                && !roof_idf.is_epjson() && !idf_legacy.has_errors()
                && idf_legacy.model.as_ref().is_some_and(|model|model.materials.len()==1 && model.materials[0].kind()==ep_model::MaterialKind::RoofVegetation)
                && crate::ordinary_equipment_input::resolve_source_b(&roof_idf).is_err_and(|e|e=="Source unused RoofVegetation requires direct epJSON"));
            // U08: invalid original names cannot fall through either Source loader; bounded values do not license UIntMAX.
            assert!(
                ["", "\n", " \t\r\u{c}", "BAD\nNAME"].iter().all(|name| {
                    let mut value = roof_value.clone();
                    let row = value[ROOF]
                        .as_object_mut()
                        .expect("family")
                        .remove("\u{b}")
                        .expect("row");
                    value[ROOF]
                        .as_object_mut()
                        .expect("family")
                        .insert((*name).into(), row);
                    crate::ordinary_equipment_input::resolve_source_b(
                        &ep_raw_model::parse_epjson_str_for_source(&value.to_string())
                            .expect("bad original Roof name"),
                    )
                    .is_err_and(|e| {
                        e == "Source RoofVegetation requires original RE2 material name"
                    })
                }) && roof_model
                    .materials
                    .iter()
                    .any(|row| row.id == ep_model::MaterialId(0)
                        && row.kind() != ep_model::MaterialKind::RoofVegetation)
            );
            let mut roof_cases: Vec<(&str, Option<serde_json::Value>, bool)> = roof_numeric
                .iter()
                .flat_map(|&field| {
                    [
                        (field, Some(json!(null)), false),
                        (field, Some(json!(false)), false),
                        (field, Some(json!("0.2")), false),
                        (field, Some(json!([])), false),
                        (field, Some(json!({})), false),
                    ]
                })
                .collect();
            roof_cases.extend(
                [
                    "soil_layer_name",
                    "roughness",
                    "moisture_diffusion_calculation_method",
                ]
                .into_iter()
                .flat_map(|field| {
                    [
                        (field, Some(json!(null)), false),
                        (field, Some(json!(false)), false),
                        (field, Some(json!(0.2)), false),
                        (field, Some(json!([])), false),
                        (field, Some(json!({})), false),
                    ]
                }),
            );
            roof_cases.extend(
                [
                    ("height_of_plants", "0.0049", false),
                    ("height_of_plants", "0.005", false),
                    ("height_of_plants", "0.2", true),
                    ("height_of_plants", "1.0", true),
                    ("height_of_plants", "1.0001", false),
                    ("leaf_area_index", "0.0009", false),
                    ("leaf_area_index", "0.001", false),
                    ("leaf_area_index", "1.0", true),
                    ("leaf_area_index", "5.0", true),
                    ("leaf_area_index", "5.0001", false),
                    ("leaf_reflectivity", "0.0499", false),
                    ("leaf_reflectivity", "0.05", true),
                    ("leaf_reflectivity", "0.22", true),
                    ("leaf_reflectivity", "0.5", true),
                    ("leaf_reflectivity", "0.5001", false),
                    ("leaf_emissivity", "0.7999", false),
                    ("leaf_emissivity", "0.8", true),
                    ("leaf_emissivity", "0.95", true),
                    ("leaf_emissivity", "1.0", true),
                    ("leaf_emissivity", "1.0001", false),
                    ("minimum_stomatal_resistance", "49.999", false),
                    ("minimum_stomatal_resistance", "50.0", true),
                    ("minimum_stomatal_resistance", "180.0", true),
                    ("minimum_stomatal_resistance", "300.0", true),
                    ("minimum_stomatal_resistance", "300.001", false),
                    ("thickness", "0.0499", false),
                    ("thickness", "0.05", false),
                    ("thickness", "0.1", true),
                    ("thickness", "0.7", true),
                    ("thickness", "0.7001", false),
                    ("conductivity_of_dry_soil", "0.1999", false),
                    ("conductivity_of_dry_soil", "0.2", true),
                    ("conductivity_of_dry_soil", "0.35", true),
                    ("conductivity_of_dry_soil", "1.5", true),
                    ("conductivity_of_dry_soil", "1.5001", false),
                    ("density_of_dry_soil", "299.999", false),
                    ("density_of_dry_soil", "300.0", true),
                    ("density_of_dry_soil", "1100.0", true),
                    ("density_of_dry_soil", "2000.0", true),
                    ("density_of_dry_soil", "2000.001", false),
                    ("specific_heat_of_dry_soil", "499.999", false),
                    ("specific_heat_of_dry_soil", "500.0", false),
                    ("specific_heat_of_dry_soil", "1200.0", true),
                    ("specific_heat_of_dry_soil", "2000.0", true),
                    ("specific_heat_of_dry_soil", "2000.001", false),
                    ("thermal_absorptance", "0.7999", false),
                    ("thermal_absorptance", "0.8", false),
                    ("thermal_absorptance", "0.9", true),
                    ("thermal_absorptance", "1.0", true),
                    ("thermal_absorptance", "1.0001", false),
                    ("solar_absorptance", "0.3999", false),
                    ("solar_absorptance", "0.4", true),
                    ("solar_absorptance", "0.7", true),
                    ("solar_absorptance", "0.9", true),
                    ("solar_absorptance", "0.9001", false),
                    ("visible_absorptance", "0.4999", false),
                    ("visible_absorptance", "0.5", false),
                    ("visible_absorptance", "0.75", true),
                    ("visible_absorptance", "1.0", true),
                    ("visible_absorptance", "1.0001", false),
                    (
                        "saturation_volumetric_moisture_content_of_the_soil_layer",
                        "0.0999",
                        false,
                    ),
                    (
                        "saturation_volumetric_moisture_content_of_the_soil_layer",
                        "0.1",
                        false,
                    ),
                    (
                        "saturation_volumetric_moisture_content_of_the_soil_layer",
                        "0.3",
                        true,
                    ),
                    (
                        "saturation_volumetric_moisture_content_of_the_soil_layer",
                        "0.5",
                        true,
                    ),
                    (
                        "saturation_volumetric_moisture_content_of_the_soil_layer",
                        "0.5001",
                        false,
                    ),
                    (
                        "residual_volumetric_moisture_content_of_the_soil_layer",
                        "0.0099",
                        false,
                    ),
                    (
                        "residual_volumetric_moisture_content_of_the_soil_layer",
                        "0.01",
                        true,
                    ),
                    (
                        "residual_volumetric_moisture_content_of_the_soil_layer",
                        "0.03",
                        true,
                    ),
                    (
                        "residual_volumetric_moisture_content_of_the_soil_layer",
                        "0.1",
                        true,
                    ),
                    (
                        "residual_volumetric_moisture_content_of_the_soil_layer",
                        "0.1001",
                        false,
                    ),
                    (
                        "initial_volumetric_moisture_content_of_the_soil_layer",
                        "0.0499",
                        false,
                    ),
                    (
                        "initial_volumetric_moisture_content_of_the_soil_layer",
                        "0.05",
                        false,
                    ),
                    (
                        "initial_volumetric_moisture_content_of_the_soil_layer",
                        "0.1",
                        true,
                    ),
                    (
                        "initial_volumetric_moisture_content_of_the_soil_layer",
                        "0.5",
                        true,
                    ),
                    (
                        "initial_volumetric_moisture_content_of_the_soil_layer",
                        "0.5001",
                        false,
                    ),
                ]
                .into_iter()
                .map(|(field, token, accepted)| {
                    (
                        field,
                        Some(
                            serde_json::from_str::<serde_json::Value>(token)
                                .expect("literal numeric boundary"),
                        ),
                        accepted,
                    )
                }),
            );
            roof_cases.extend(roof_numeric.iter().flat_map(|&field| {
                ["", "Autocalculate", "Autosize"]
                    .into_iter()
                    .map(move |token| (field, Some(json!(token)), false))
            }));
            roof_cases.extend(roof_numeric.iter().flat_map(|&field| {
                [
                    json!(u64::MAX),
                    json!(1.8446744073709552e19),
                    json!(0),
                    json!(-0.0),
                ]
                .into_iter()
                .map(move |value| (field, Some(value), false))
            }));
            roof_cases.extend(
                [
                    ("roughness", "rough"),
                    ("roughness", "ROUGH"),
                    ("roughness", " Rough"),
                    ("roughness", "Unknown"),
                    ("moisture_diffusion_calculation_method", "simple"),
                    ("moisture_diffusion_calculation_method", "ADVANCED"),
                    ("moisture_diffusion_calculation_method", " Simple"),
                    ("moisture_diffusion_calculation_method", "Unknown"),
                ]
                .into_iter()
                .map(|(field, token)| (field, Some(json!(token)), false)),
            );
            roof_cases.extend(
                roof_numeric
                    .iter()
                    .copied()
                    .chain([
                        "soil_layer_name",
                        "roughness",
                        "moisture_diffusion_calculation_method",
                    ])
                    .map(|field| (field, None, true)),
            );
            roof_cases.extend(
                [
                    "soil_layer_name",
                    "roughness",
                    "moisture_diffusion_calculation_method",
                ]
                .into_iter()
                .map(|field| (field, Some(json!("")), true)),
            );
            // U09: the sole original-field loop includes90 kinds/75bounds plus strict enums/NoAuto/huge/zero/default cases.
            for (field, value, accepted) in roof_cases {
                let mut value_tree = roof_value.clone();
                if let Some(value) = value {
                    value_tree[ROOF]["\u{a0}"][field] = value;
                } else {
                    value_tree[ROOF]["\u{a0}"]
                        .as_object_mut()
                        .expect("row")
                        .remove(field);
                }
                let raw = ep_raw_model::parse_epjson_str_for_source(&value_tree.to_string())
                    .expect("original Roof field fixture");
                let resolved = crate::ordinary_equipment_input::resolve_source_b(&raw);
                assert!(resolved.is_err() != accepted && (accepted || resolved.is_err_and(|e|e=="Source RoofVegetation requires every supplied known field to remain original-schema-valid")),"Roof original {field}");
            }
            let opaque = roof_model
                .materials
                .iter()
                .find(|row| row.kind() != ep_model::MaterialKind::RoofVegetation)
                .expect("held opaque material");
            // U10: Raw-present/typed-zero, kind/count/global ID, loader/report and both name-to-ID associations.
            for case in 0_usize..12 {
                let mut bad = roof_model.clone();
                match case {
                    0 => bad
                        .materials
                        .retain(|row| row.kind() != ep_model::MaterialKind::RoofVegetation),
                    1 => {
                        let mut extra = bad.materials[roof_a_position].clone();
                        extra.id = ep_model::MaterialId(u32::MAX);
                        extra.name = ep_model::NormalizedName("EXTRA ROOF".into());
                        bad.materials.push(extra);
                    }
                    2 => bad.materials[roof_a_position].id = opaque.id,
                    3 => bad.materials[roof_a_position].definition = opaque.definition,
                    4 => {
                        bad.materials[roof_a_position].name =
                            ep_model::NormalizedName("WRONG ROOF".into())
                    }
                    5 => bad.materials[roof_a_position].id = ep_model::MaterialId(u32::MAX),
                    6 => bad
                        .ctf_report_names
                        .materials
                        .retain(|(id, _)| *id != roof_a),
                    7 => bad
                        .ctf_report_names
                        .materials
                        .push((roof_a, "\u{b}".into())),
                    8 => {
                        let _ = bad.material_names.insert("EXTRA ROOF ALIAS", roof_a);
                    }
                    9 => {
                        bad.ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == roof_a)
                            .expect("A tuple")
                            .1 = "\u{a0}".into();
                        bad.ctf_report_names
                            .materials
                            .iter_mut()
                            .find(|(id, _)| *id == roof_b)
                            .expect("B tuple")
                            .1 = "\u{b}".into();
                    }
                    10 => {
                        bad.material_names = ep_model::NameMap::default();
                        bad.materials.iter().for_each(|row| {
                            let _ = bad.material_names.insert(&row.name.0, row.id);
                        });
                    }
                    _ => {
                        bad.material_names = ep_model::NameMap::source_exact_namespace();
                        bad.materials.iter().for_each(|row| {
                            let id = if row.id == roof_a {
                                roof_b
                            } else if row.id == roof_b {
                                roof_a
                            } else {
                                row.id
                            };
                            let _ = bad.material_names.insert(&row.name.0, id);
                        });
                    }
                }
                assert!(
                    !crate::support::assess_support_with_source_admission(
                        &roof_raw,
                        &roof_compiled.report,
                        Some(&bad),
                        RunMode::Compatibility,
                        PartialRunPolicy::Deny,
                        RunOutputFormat::RustNative,
                        TraceLevel::Normal,
                        Some(&roof_admission)
                    )
                    .allows_rust_runtime(),
                    "Roof custody {case}"
                );
            }
            let construction_index = roof_model
                .constructions
                .iter()
                .position(|row| row.is_ordinary_opaque() && !row.layers.is_empty())
                .expect("real opaque Construction");
            // U11/U12: actual Raw outside/layers2..10 and typed original/effective/nonempty/empty-fallback positions.
            for case in 0_usize..11 {
                assert!(["Construction","Construction:WindowEquivalentLayer","Construction:AirBoundary","Construction:InternalSource"].iter().all(|family| {
                    let mut used=roof_value.clone();let mut fields=serde_json::Map::from_iter([("outside_layer".into(),json!("Opaque"))]);
                    let field=if case==0 || case==10 {"outside_layer".into()}else{format!("layer_{}",case+1)};
                    fields.insert(field,json!(if case.is_multiple_of(2){"\u{b}"}else{"\u{a0}"}));used[*family]["ROOF USED"]=serde_json::Value::Object(fields);
                    crate::ordinary_equipment_input::resolve_source_b(&ep_raw_model::parse_epjson_str_for_source(&used.to_string()).expect("Raw Construction-prefix Roof use"))
                        .is_err_and(|e|e=="Source unused RoofVegetation excludes every Construction layer reference")
                }),"Raw Roof Construction position {case}");
                let mut used = roof_model.clone();
                let construction = &mut used.constructions[construction_index];
                if case == 10 {
                    construction.layers.clear();
                    construction.outside_layer = Some(roof_a);
                } else {
                    construction.layers.resize(case + 1, opaque.id);
                    construction.layers[case] = if case.is_multiple_of(2) {
                        roof_a
                    } else {
                        roof_b
                    };
                }
                let blocked = crate::support::assess_support_with_source_admission(
                    &roof_raw,
                    &roof_compiled.report,
                    Some(&used),
                    RunMode::Compatibility,
                    PartialRunPolicy::Deny,
                    RunOutputFormat::RustNative,
                    TraceLevel::Normal,
                    Some(&roof_admission),
                );
                let mut original_outside = roof_model.clone();
                original_outside.constructions[construction_index].outside_layer = Some(roof_a);
                assert!(
                    !blocked.allows_rust_runtime()
                        && blocked
                            .unsupported_objects
                            .iter()
                            .any(|row| row.object_type == ROOF)
                        && (case != 0
                            || !crate::support::assess_support_with_source_admission(
                                &roof_raw,
                                &roof_compiled.report,
                                Some(&original_outside),
                                RunMode::Compatibility,
                                PartialRunPolicy::Deny,
                                RunOutputFormat::RustNative,
                                TraceLevel::Normal,
                                Some(&roof_admission)
                            )
                            .allows_rust_runtime()),
                    "typed/effective/fallback Roof position {case}"
                );
            }
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
