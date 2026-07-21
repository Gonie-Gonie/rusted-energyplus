mod bounds;
use super::super::{Compiler, compile_raw_model};
use ep_model::{
    AutoOrNumber, ConstructionId, ModelGraph, NormalizedName, OutsideBoundaryCondition, Point3,
    SpaceId, SunExposure, Surface, SurfaceId, SurfaceProjectedPoint, SurfaceProjectionAxis,
    SurfaceShapeCategory, SurfaceType, TypedModel, WindExposure, ZoneId,
};
use ep_raw_model::parse_epjson_str;

fn point(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
    Point3 { x_m, y_m, z_m }
}

fn projected_point(x_m: f64, y_m: f64) -> SurfaceProjectedPoint {
    SurfaceProjectedPoint { x_m, y_m }
}

fn surface(id: u32, vertices: Vec<Point3>) -> Surface {
    Surface {
        id: SurfaceId(id),
        name: NormalizedName::new(&format!("Surface {id}")),
        surface_type: SurfaceType::Wall,
        construction: ConstructionId(0),
        zone: ZoneId(0),
        space: SpaceId(0),
        outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::SunExposed,
        wind_exposure: WindExposure::WindExposed,
        view_factor_to_ground: AutoOrNumber::AutoCalculate,
        vertices,
        computed_geometry: None,
    }
}

fn set_geometry(model: &mut TypedModel) {
    let raw = parse_epjson_str("{}").expect("empty epJSON should parse");
    let mut compiler = Compiler::new(&raw, None);
    compiler.set_bounded_surface_computed_geometry(model);
    assert!(compiler.diagnostics.is_empty());
}

#[test]
fn newell_planes_match_source_coefficients_and_select_largest_normal_axis() {
    let mut model = TypedModel::default();
    model.surfaces.push(surface(
        0,
        vec![
            point(1.0, 1.0, 1.0),
            point(-1.0, 1.0, 0.0),
            point(2.0, 0.0, 3.0),
        ],
    ));
    model.surfaces.push(surface(
        1,
        vec![
            point(2.0, 1.0, -1.0),
            point(0.0, -2.0, 0.0),
            point(1.0, -1.0, 2.0),
        ],
    ));

    set_geometry(&mut model);

    let first = model.surfaces[0]
        .computed_geometry
        .as_ref()
        .expect("first triangle should be admitted");
    assert_eq!(first.shape_category, SurfaceShapeCategory::Triangular);
    assert_eq!(first.plane, [-1.0, 3.0, 2.0, -4.0]);
    assert_eq!(first.projection_axis, SurfaceProjectionAxis::Y);
    assert_eq!(
        first.projected_vertices,
        [
            projected_point(1.0, 1.0),
            projected_point(2.0, 3.0),
            projected_point(-1.0, 0.0),
        ]
    );
    let second = model.surfaces[1]
        .computed_geometry
        .as_ref()
        .expect("second triangle should be admitted");
    assert_eq!(second.plane, [-7.0, 5.0, 1.0, 10.0]);
    assert_eq!(second.projection_axis, SurfaceProjectionAxis::X);
}

#[test]
fn clockwise_projection_reverses_only_the_tail_and_builds_wraparound_edges() {
    let original_vertices = vec![
        point(2.0, 0.0, 0.0),
        point(2.0, 0.0, 2.0),
        point(2.0, 3.0, 0.0),
    ];
    let mut model = TypedModel::default();
    model.surfaces.push(surface(0, original_vertices.clone()));

    set_geometry(&mut model);

    let geometry = model.surfaces[0]
        .computed_geometry
        .as_ref()
        .expect("triangle should be admitted");
    assert_eq!(model.surfaces[0].vertices, original_vertices);
    assert_eq!(geometry.plane, [-6.0, 0.0, 0.0, 12.0]);
    assert_eq!(geometry.projection_axis, SurfaceProjectionAxis::X);
    assert_eq!(
        geometry.projected_vertices,
        vec![
            projected_point(0.0, 0.0),
            projected_point(3.0, 0.0),
            projected_point(0.0, 2.0),
        ]
    );
    assert_eq!(
        geometry.projected_edges,
        vec![
            projected_point(3.0, 0.0),
            projected_point(-3.0, 2.0),
            projected_point(0.0, -2.0),
        ]
    );
    assert_eq!(geometry.projected_lower_bound, projected_point(0.0, 0.0));
    assert_eq!(geometry.projected_upper_bound, projected_point(3.0, 2.0));
    assert_eq!(geometry.rectangle_side_1_squared_m2, 0.0);
    assert_eq!(geometry.rectangle_side_3_squared_m2, 0.0);
}

#[test]
fn rectangle_projection_matches_source_plane_order_bounds_and_side_squares() {
    let vertices = vec![
        point(0.0, 0.0, 1.0),
        point(0.0, 2.0, 1.0),
        point(4.0, 2.0, 1.0),
        point(4.0, 0.0, 1.0),
    ];
    let mut model = TypedModel::default();
    model.surfaces.push(surface(0, vertices));

    set_geometry(&mut model);

    let geometry = model.surfaces[0]
        .computed_geometry
        .as_ref()
        .expect("rectangle should be admitted");
    assert_eq!(geometry.shape_category, SurfaceShapeCategory::Rectangular);
    assert_eq!(geometry.plane, [0.0, 0.0, -16.0, 16.0]);
    assert_eq!(geometry.projection_axis, SurfaceProjectionAxis::Z);
    assert_eq!(
        geometry.projected_vertices,
        vec![
            projected_point(0.0, 0.0),
            projected_point(4.0, 0.0),
            projected_point(4.0, 2.0),
            projected_point(0.0, 2.0),
        ]
    );
    assert_eq!(geometry.projected_lower_bound, projected_point(0.0, 0.0));
    assert_eq!(geometry.projected_upper_bound, projected_point(4.0, 2.0));
    assert_eq!(geometry.rectangle_side_1_squared_m2, 16.0);
    assert_eq!(geometry.rectangle_side_3_squared_m2, 4.0);
}

#[test]
fn projection_axis_ties_prefer_x_then_y_then_z() {
    let mut model = TypedModel::default();
    model.surfaces.push(surface(
        0,
        vec![
            point(1.0, 0.0, 0.0),
            point(0.0, 1.0, 0.0),
            point(0.0, 0.0, 1.0),
        ],
    ));
    model.surfaces.push(surface(
        1,
        vec![
            point(0.0, 0.0, 0.0),
            point(1.0, 0.0, 0.0),
            point(0.0, 1.0, -1.0),
        ],
    ));
    model.surfaces.push(surface(
        2,
        vec![
            point(0.0, 0.0, 0.0),
            point(1.0, 0.0, 0.0),
            point(0.0, 1.0, 0.0),
        ],
    ));

    set_geometry(&mut model);

    let axes = model
        .surfaces
        .iter()
        .map(|surface| {
            surface
                .computed_geometry
                .as_ref()
                .expect("tie fixture should be admitted")
                .projection_axis
        })
        .collect::<Vec<_>>();
    assert_eq!(
        axes,
        [
            SurfaceProjectionAxis::X,
            SurfaceProjectionAxis::Y,
            SurfaceProjectionAxis::Z,
        ]
    );
}

#[test]
fn pre_existing_error_suppresses_the_whole_projection_and_empty_input_is_a_no_op() {
    let raw = parse_epjson_str(
        r#"{"Timestep":{"Broken":{"number_of_timesteps_per_hour":"not a number"}}}"#,
    )
    .expect("diagnostic fixture should parse");
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    model.surfaces.push(surface(
        0,
        vec![
            point(0.0, 0.0, 0.0),
            point(1.0, 0.0, 0.0),
            point(0.0, 1.0, 0.0),
        ],
    ));
    compiler.parse_timestep(&mut model);
    assert!(
        compiler
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.severity == super::super::DiagnosticSeverity::Error })
    );

    compiler.set_bounded_surface_computed_geometry(&mut model);

    assert!(model.surfaces[0].computed_geometry.is_none());

    let empty_raw = parse_epjson_str("{}").expect("empty epJSON should parse");
    let mut empty_compiler = Compiler::new(&empty_raw, None);
    let mut empty_model = TypedModel::default();
    let before = empty_model.clone();
    empty_compiler.set_bounded_surface_computed_geometry(&mut empty_model);
    assert_eq!(empty_model, before);
    assert!(empty_compiler.diagnostics.is_empty());
}

#[test]
fn derived_attachment_changes_neither_object_count_nor_model_graph() {
    let mut model = TypedModel::default();
    model.surfaces.push(surface(
        0,
        vec![
            point(0.0, 0.0, 0.0),
            point(1.0, 0.0, 0.0),
            point(0.0, 1.0, 0.0),
        ],
    ));
    let object_count_before = model.object_count();
    let graph_before = ModelGraph::from_typed(&model);

    set_geometry(&mut model);

    assert!(model.surfaces[0].computed_geometry.is_some());
    assert_eq!(model.object_count(), object_count_before);
    assert_eq!(ModelGraph::from_typed(&model), graph_before);
}

#[test]
fn full_compiler_invokes_projection_after_surface_and_internal_gain_input() {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass":{"Layer":{"roughness":"Rough","thermal_resistance":1.0}},
            "Construction":{"Wall":{"outside_layer":"Layer"}},
            "Zone":{"Zone One":{}},
            "BuildingSurface:Detailed":{
                "Triangle":{
                    "surface_type":"Wall",
                    "construction_name":"Wall",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":1,"vertex_z_coordinate":0}
                    ]
                }
            }
        }"#,
    )
    .expect("full compiler fixture should parse");

    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result.model.expect("fixture should compile");
    assert_eq!(model.surfaces.len(), 1);
    assert!(model.surfaces[0].computed_geometry.is_some());
    assert_eq!(result.report.typed_object_count, model.object_count());
}
