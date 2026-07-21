use super::{point, set_geometry, surface};
use crate::compiler::surface_computed_geometry::source_rectangle_right_angle_is_admitted;
use ep_model::{Point3, SurfaceId, SurfaceProjectionAxis, TypedModel};

fn diagonal_tolerance_fixture(difference_m: f64) -> Vec<Point3> {
    let diagonal_2 = std::f64::consts::SQRT_2 + difference_m;
    let fourth_y = (diagonal_2 * diagonal_2 - 1.0).sqrt();
    vec![
        point(0.0, 0.0, 0.0),
        point(1.0, 0.0, 0.0),
        point(1.0, 1.0, 0.0),
        point(0.0, fourth_y, 0.0),
    ]
}

#[test]
fn rectangle_diagonal_tolerance_is_strictly_less_than_point_zero_two_meters() {
    let mut model = TypedModel::default();
    model
        .surfaces
        .push(surface(0, diagonal_tolerance_fixture(0.020 - 1.0e-8)));
    model
        .surfaces
        .push(surface(1, diagonal_tolerance_fixture(0.020)));

    set_geometry(&mut model);

    assert!(model.surfaces[0].computed_geometry.is_some());
    assert!(model.surfaces[1].computed_geometry.is_none());
}

fn angle_tolerance_fixture(angle_degrees: f64) -> Vec<Point3> {
    let angle = angle_degrees.to_radians();
    let side = 0.5;
    let cosine = angle.cos();
    let offset_x = side * cosine;
    let offset_y = side * (1.0 - cosine * cosine).sqrt();
    vec![
        point(0.0, 0.0, 0.0),
        point(side, 0.0, 0.0),
        point(side + offset_x, offset_y, 0.0),
        point(offset_x, offset_y, 0.0),
    ]
}

#[test]
fn right_angle_comparison_is_inclusive_and_surface_fixture_respects_tolerance() {
    let cosine_89_degrees = 89.0_f64.to_radians().cos();
    assert!(source_rectangle_right_angle_is_admitted(cosine_89_degrees));
    assert!(source_rectangle_right_angle_is_admitted(-cosine_89_degrees));
    assert!(!source_rectangle_right_angle_is_admitted(
        cosine_89_degrees.next_up()
    ));

    let mut model = TypedModel::default();
    model
        .surfaces
        .push(surface(0, angle_tolerance_fixture(89.000_001)));
    model
        .surfaces
        .push(surface(1, angle_tolerance_fixture(88.9)));

    set_geometry(&mut model);

    assert!(model.surfaces[0].computed_geometry.is_some());
    assert!(model.surfaces[1].computed_geometry.is_none());
}

#[test]
fn derived_state_follows_dense_surface_order_and_omits_unsupported_or_unsafe_geometry() {
    let mut model = TypedModel {
        surfaces: vec![
            surface(
                0,
                vec![
                    point(0.0, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(0.0, 1.0, 0.0),
                ],
            ),
            surface(
                1,
                vec![
                    point(0.0, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(0.5, 1.5, 0.0),
                    point(0.0, 1.0, 0.0),
                ],
            ),
            surface(
                2,
                vec![
                    point(0.0, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(2.0, 0.0, 0.0),
                ],
            ),
            surface(
                3,
                vec![
                    point(0.0, 0.0, 0.0),
                    point(0.0, 2.0, 0.0),
                    point(4.0, 2.0, 0.0),
                    point(4.0, 0.0, 0.1),
                ],
            ),
            surface(
                4,
                vec![
                    point(f64::NAN, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(0.0, 1.0, 0.0),
                ],
            ),
            surface(
                5,
                vec![
                    point(0.0, 0.0, 1.0),
                    point(0.0, 2.0, 1.0),
                    point(4.0, 2.0, 1.0),
                    point(4.0, 0.0, 1.0),
                ],
            ),
            surface(6, vec![point(0.0, 0.0, 0.0), point(1.0, 0.0, 0.0)]),
        ],
        ..TypedModel::default()
    };

    set_geometry(&mut model);

    assert_eq!(
        model
            .surfaces
            .iter()
            .map(|surface| surface.computed_geometry.is_some())
            .collect::<Vec<_>>(),
        [true, false, false, false, false, true, false]
    );
    assert_eq!(
        model
            .surfaces
            .iter()
            .map(|surface| surface.id)
            .collect::<Vec<_>>(),
        (0..7).map(SurfaceId).collect::<Vec<_>>()
    );
}

#[test]
fn projected_bounds_preserve_source_first_operand_signed_zero_ties() {
    let mut model = TypedModel::default();
    model.surfaces.push(surface(
        0,
        vec![
            point(0.0, 0.0, 0.0),
            point(1.0, -0.0, 0.0),
            point(-0.0, 1.0, 0.0),
        ],
    ));
    model.surfaces.push(surface(
        1,
        vec![
            point(-0.0, 0.0, 0.0),
            point(-1.0, 1.0, 0.0),
            point(0.0, 2.0, 0.0),
        ],
    ));

    set_geometry(&mut model);

    let positive_zero_lower = model.surfaces[0]
        .computed_geometry
        .as_ref()
        .expect("positive-zero triangle should be admitted");
    assert_eq!(
        positive_zero_lower.projection_axis,
        SurfaceProjectionAxis::Z
    );
    assert_eq!(
        positive_zero_lower.projected_lower_bound.x_m.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        positive_zero_lower.projected_lower_bound.y_m.to_bits(),
        0.0_f64.to_bits()
    );

    let negative_zero_upper = model.surfaces[1]
        .computed_geometry
        .as_ref()
        .expect("negative-zero triangle should be admitted");
    assert_eq!(
        negative_zero_upper.projection_axis,
        SurfaceProjectionAxis::Z
    );
    assert_eq!(
        negative_zero_upper.projected_upper_bound.x_m.to_bits(),
        (-0.0_f64).to_bits()
    );
}
