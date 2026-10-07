//! Independent analytic and branch regressions for the source geometry owner.
use super::*;

fn p(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
    Point3 { x_m, y_m, z_m }
}

#[test]
fn quad_centroid_is_area_weighted_and_not_vertex_mean() {
    // A 4 by 2 trapezoid with its top edge shortened to 2 has area 6;
    // its analytic centroid is (14/9,8/9), unlike the vertex mean (1.5,1).
    let value =
        surface_geometry_properties(&[p(0., 0., 2.), p(4., 0., 2.), p(2., 2., 2.), p(0., 2., 2.)])
            .unwrap();
    assert_eq!(value.area_m2, 6.0);
    assert!((value.centroid_m.x_m - 14.0 / 9.0).abs() < 1e-14);
    assert!((value.centroid_m.y_m - 8.0 / 9.0).abs() < 1e-14);
    assert_eq!(value.centroid_m.z_m, 2.0);
    assert_ne!(value.centroid_m.x_m, 1.5);
    assert_ne!(value.centroid_m.y_m, 1.0);
}

#[test]
fn concave_quad_uses_signed_area_and_alternate_centroid_triangles() {
    // The first triangle overlaps the polygon. Summing triangle magnitudes
    // gives 5.25; the ordered Newell vector gives its actual area, 3.75.
    let value = surface_geometry_properties(&[
        p(0., 0., 2.),
        p(0.5, 0.5, 2.),
        p(3., 0., 2.),
        p(0., 3., 2.),
    ])
    .unwrap();
    assert_eq!(value.area_m2, 3.75);
    assert_eq!(value.newell_area_vector_m2, [0., 0., 3.75]);
    assert!((value.centroid_m.x_m - 29.0 / 30.0).abs() < 1e-14);
    assert!((value.centroid_m.y_m - 7.0 / 6.0).abs() < 1e-14);
}

#[test]
fn tilt_uses_winding_and_near_horizontal_geometry_without_class_override() {
    let vertices = [
        p(0., 0., 2.),
        p(3., 0., 2.00000003),
        p(3., 2., 2.00000003),
        p(0., 2., 2.),
    ];
    let value = surface_geometry_properties(&vertices).unwrap();
    assert!(value.tilt_deg > 0.0 && value.tilt_deg < 1e-5);
    assert_eq!(value.azimuth_deg, 90.0);
    let mut reversed = vertices;
    reversed.reverse();
    let reversed = surface_geometry_properties(&reversed).unwrap();
    assert!(reversed.tilt_deg > 179.99);
    // OutNorm is independently snapped; the raw near-horizontal normal remains.
    assert_ne!(
        value.newell_normal[0].to_bits(),
        value.out_norm[0].to_bits()
    );
    assert_eq!(value.out_norm, [0., 0., 1.]);
}

#[test]
fn unsafe_geometry_cannot_initialize_a_bundle() {
    let collinear = [p(0., 0., 0.), p(1., 0., 0.), p(2., 0., 0.), p(3., 0., 0.)];
    assert_eq!(
        surface_geometry_properties(&collinear),
        Err(SurfaceGeometryError::Degenerate)
    );
    assert_eq!(
        surface_geometry_properties(&[p(0., 0., 0.); 4]),
        Err(SurfaceGeometryError::Degenerate)
    );
    assert_eq!(
        surface_geometry_properties(&collinear[..2]),
        Err(SurfaceGeometryError::TooFewVertices)
    );
    let mut invalid = collinear;
    invalid[1].x_m = f64::INFINITY;
    assert_eq!(
        surface_geometry_properties(&invalid),
        Err(SurfaceGeometryError::Nonfinite)
    );
}

#[test]
fn triangle_centroid_keeps_the_source_ordered_sum_and_signed_zero() {
    let left_loss = source_triangle_centroid(p(1e16, 0., 0.), p(1., 0., 0.), p(-1e16, 0., 0.));
    let left_exact = source_triangle_centroid(p(1e16, 0., 0.), p(-1e16, 0., 0.), p(1., 0., 0.));
    assert_eq!(left_loss.x_m, 0.0);
    assert!(left_exact.x_m > 0.0);
    let zero = source_triangle_centroid(p(-0., 0., 0.), p(-0., 0., 0.), p(-0., 0., 0.));
    assert_eq!(zero.x_m.to_bits(), (-0.0f64).to_bits());
}
