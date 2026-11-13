use super::{compiled_projection, runtime_projection};
use ep_compiler::compile_raw_model;
use ep_model::{SimulationModel, TypedModel};
use ep_raw_model::{RawModel, parse_epjson_str};
use serde_json::json;

fn fixture() -> (RawModel, TypedModel) {
    let raw = parse_epjson_str(r#"{
        "Building":{"Building":{"north_axis":765}},
        "GlobalGeometryRules":{"Rules":{
            "starting_vertex_position":"UpperLeftCorner",
            "vertex_entry_direction":"Counterclockwise","coordinate_system":"World"}},
        "Zone":{"Zone":{"volume":10}},
        "Material:NoMass":{"Material":{"roughness":"Rough","thermal_resistance":1}},
        "Construction":{"Construction":{"outside_layer":"Material"}},
        "BuildingSurface:Detailed":{"Wall":{"surface_type":"Wall","construction_name":"Construction",
            "zone_name":"Zone","outside_boundary_condition":"Outdoors","vertices":[
                {"vertex_x_coordinate":-0.0,"vertex_y_coordinate":-0.0,"vertex_z_coordinate":3},
                {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                {"vertex_x_coordinate":2,"vertex_y_coordinate":0,"vertex_z_coordinate":0}]}}
    }"#).expect("geometry input parses");
    let compiled = compile_raw_model(&raw);
    assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
    (raw, compiled.model.expect("geometry input compiles"))
}

#[test]
fn compile_projection_retains_actual_input_zero_bits_and_copies_stored_vertices() {
    let (raw, mut model) = fixture();
    // Deliberately distinct stored coordinates prove the observer does not
    // reconstruct the output from the incoming vertices or rerun a transform.
    model.surfaces[0].vertices[0].x_m = 1234.5;
    let hashes = json!({"algorithm":"fnv-1a-64","source":{"path":"source.idf","hash":"a"},
        "staged_original":{"path":"original.idf","hash":"b"},
        "converted_epjson":{"path":"converted.epJSON","hash":"c"}});
    let observed = compiled_projection(&raw, &model, hashes, true).expect("projection succeeds");
    assert_eq!(observed["schema"], "compiled-geometry.v1");
    assert_eq!(observed["physics_executed"], false);
    assert_eq!(observed["observer_supplies_inputs"], false);
    assert_eq!(observed["inputs"]["hash_algorithm"], "fnv-1a-64");
    assert_eq!(
        observed["inputs"]["source_assisted_lexical_conversion"],
        true
    );
    // The observer copies the actual RawModel format, independently of the
    // deliberately true caller-extension flag above.
    assert_eq!(observed["parsed_input"]["raw_source_is_epjson"], true);
    assert_eq!(
        observed["parsed_input"]["scheduled_idf_declaration_overlay"],
        false
    );
    let scheduled = r#"{"Zone":{"Z":{}},"SetpointManager:Scheduled":{
        "Alpha Manager":{"control_variable":"MassFlowRate","schedule_name":"On","setpoint_node_or_nodelist_name":"Supply"},
        "Zulu Manager":{"control_variable":"Temperature","schedule_name":"On","setpoint_node_or_nodelist_name":"Supply"}
    }}"#;
    let direct = parse_epjson_str(scheduled).expect("direct Scheduled raw input");
    let direct_observed = super::parsed_input(&direct).expect("direct raw observation");
    assert_eq!(direct_observed["raw_source_is_epjson"], true);
    assert_eq!(direct_observed["scheduled_idf_declaration_overlay"], false);
    let staged = ep_raw_model::parse_epjson_str_with_idf_order(scheduled,
        "Zone,Z;SetpointManager:Scheduled,Zulu Manager,Temperature,On,Supply;SetpointManager:Scheduled,Alpha Manager,MassFlowRate,On,Supply;")
        .expect("genuine Scheduled declaration writer");
    let staged_observed = super::parsed_input(&staged).expect("actual staged raw observation");
    assert_eq!(staged_observed["raw_source_is_epjson"], false);
    assert_eq!(staged_observed["scheduled_idf_declaration_overlay"], true);
    for (name, ordinal) in [("Zulu Manager", 2), ("Alpha Manager", 3)] {
        let actual = &staged_observed["objects"]["SetpointManager:Scheduled"][name];
        assert_eq!(actual["idf_order"], f64::from(ordinal));
        assert_eq!(actual["idf_max_fields"], 4.0);
        assert_eq!(actual["idf_max_extensible_fields"], 0.0);
        assert_eq!(actual["schedule_name"], "On");
        assert_eq!(actual["setpoint_node_or_nodelist_name"], "Supply");
        assert_eq!(
            actual["control_variable"],
            direct_observed["objects"]["SetpointManager:Scheduled"][name]["control_variable"]
        );
    }
    assert_eq!(observed["settings"]["building_raw_north_axis_deg"], 765.0);
    assert_eq!(
        observed["settings"]["building_effective_north_axis_deg"],
        model.building.as_ref().expect("building").north_axis_deg
    );
    assert_eq!(observed["surfaces"][0]["world_vertices_m"][0][0], 1234.5);
    assert_eq!(
        observed["surfaces"][0]["world_vertex_bits"][0][0],
        format!("{:016x}", 1234.5_f64.to_bits())
    );
    let input_bits = observed["parsed_input"]["numeric_bits"]
        .as_array()
        .expect("numeric observations");
    let x = input_bits
        .iter()
        .find(|row| row["path"] == "/BuildingSurface:Detailed/Wall/vertices/0/vertex_x_coordinate")
        .expect("actual input x");
    assert_eq!(x["bits"], "8000000000000000");
    assert_eq!(
        observed["surfaces"][0]["zone_id"],
        observed["zones"][0]["id"]
    );
    assert!(
        observed["native_only_unpaired_state"]
            .as_array()
            .expect("exclusions")
            .iter()
            .any(|row| row == "MaxVerticesPerSurface")
    );
}

#[test]
fn runtime_projection_copies_mutated_final_consumer_fields_without_recomputation() {
    let (_, model) = fixture();
    let simulation_model = SimulationModel::from_typed(model);
    let mut state =
        ep_runtime::heat_balance::initialize_heat_balance_state(&simulation_model, 20.0)
            .expect("fixture initializes");
    // These are stored-field observations, independently varied from the
    // input geometry to reject any alternate calculation in the serializer.
    state.surfaces[0].area_m2 = 12345.125;
    state.surfaces[0].azimuth_deg = -77.25;
    state.surfaces[0].tilt_deg = 44.75;
    state.zones[0].volume_m3 = 2468.125;
    state.surfaces[0].geometry.centroid_m.z_m = -123.5;
    state.surfaces[0].geometry.out_norm = [-0.0, 0.25, 0.75];
    let observed = runtime_projection(&simulation_model.typed, &state);
    assert_eq!(observed["phase"], "runtime_final_state_geometry");
    assert_eq!(observed["physics_executed"], true);
    let surface = &observed["surfaces"][0]["rust_consumer"];
    assert_eq!(surface["area_m2"], 12345.125);
    assert_eq!(surface["azimuth_deg"], -77.25);
    assert_eq!(surface["tilt_deg"], 44.75);
    assert_eq!(
        surface["area_bits"],
        format!("{:016x}", 12345.125_f64.to_bits())
    );
    assert_eq!(observed["zones"][0]["rust_consumer"]["volume_m3"], 2468.125);
    assert_eq!(
        observed["zones"][0]["rust_consumer"]["volume_bits"],
        format!("{:016x}", 2468.125_f64.to_bits())
    );
    let canonical = &observed["geo02_geometry_snapshot"]["surfaces"][0]["geometry"];
    assert_eq!(canonical["centroid_m"][2]["value"], -123.5);
    assert_eq!(canonical["out_norm"][0]["value_bits"], "8000000000000000");
    assert_eq!(canonical["out_norm"][1]["value"], 0.25);
    assert_eq!(canonical["out_norm"][2]["value"], 0.75);
    assert_eq!(
        observed["geo02_geometry_snapshot"]["surfaces"][0]["legacy_geometry"]["area_m2"]["value"],
        12345.125,
    );
}
