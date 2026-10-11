use super::*;

fn context() -> ApproximateViewFactorContext {
    ApproximateViewFactorContext {
        enclosure_index: 7,
        zone_id: ZoneId(4),
    }
}

fn surface(
    id: u32,
    kind: SurfaceType,
    area: f64,
    azimuth: f64,
    tilt: f64,
) -> ApproximateViewFactorSurface {
    ApproximateViewFactorSurface {
        surface_id: SurfaceId(id),
        surface_name: format!("surface-{id}"),
        zone_id: ZoneId(4),
        zone_name: "selected-zone".into(),
        surface_type: kind,
        area_m2: area,
        azimuth_deg: azimuth,
        tilt_deg: tilt,
    }
}

#[test]
fn asymmetric_areas_preserve_f_destination_row_source_column()
-> Result<(), ApproximateViewFactorScopeError> {
    let rows = [
        surface(90, SurfaceType::Wall, 2.0, 0.0, 90.0),
        surface(3, SurfaceType::Wall, 4.0, 180.0, 90.0),
        surface(41, SurfaceType::Floor, 8.0, 0.0, 180.0),
    ];
    let actual = calculate_selected_approximate_view_factors(context(), &rows)?;
    assert_eq!(actual.context, context());
    assert_eq!(
        actual.ordered_surface_ids,
        vec![SurfaceId(90), SurfaceId(3), SurfaceId(41)]
    );
    assert_eq!(actual.zone_area_seen_m2, vec![12.0, 10.0, 6.0]);
    assert_eq!(actual.view_factors.len(), 9);
    assert_eq!(actual.view_factors[3].to_bits(), (4.0_f64 / 12.0).to_bits());
    assert_eq!(actual.view_factors[1].to_bits(), (2.0_f64 / 10.0).to_bits());
    assert_eq!(actual.view_factors[6].to_bits(), (8.0_f64 / 12.0).to_bits());
    assert_eq!(actual.view_factors[2].to_bits(), (2.0_f64 / 6.0).to_bits());
    for diagonal in [0, 4, 8] {
        assert_eq!(actual.view_factors[diagonal].to_bits(), 0);
    }
    assert!(actual.warnings.is_empty());
    Ok(())
}

#[test]
fn strict_angle_limits_and_tilt_branch_match_source_boundaries()
-> Result<(), ApproximateViewFactorScopeError> {
    for (azimuth, tilt, sees) in [
        (10.0, 90.0, false),
        (10.0001, 90.0, true),
        (350.0, 90.0, false),
        (349.9999, 90.0, true),
        (0.0, 100.0, false),
        (0.0, 100.0001, true),
        (360.0, 90.0, false),
    ] {
        let rows = [
            surface(9, SurfaceType::Wall, 2.0, 0.0, 90.0),
            surface(2, SurfaceType::Wall, 3.0, azimuth, tilt),
        ];
        let actual = calculate_selected_approximate_view_factors(context(), &rows)?;
        assert_eq!(actual.view_factors[2], if sees { 1.0 } else { 0.0 });
        assert_eq!(actual.warnings.len(), if sees { 0 } else { 2 });
    }
    Ok(())
}

#[test]
fn floors_do_not_see_each_other_but_see_same_orientation_roof()
-> Result<(), ApproximateViewFactorScopeError> {
    let rows = [
        surface(12, SurfaceType::Floor, 2.0, 0.0, 90.0),
        surface(1, SurfaceType::Floor, 4.0, 0.0, 90.0),
        surface(8, SurfaceType::Roof, 8.0, 0.0, 90.0),
    ];
    let actual = calculate_selected_approximate_view_factors(context(), &rows)?;
    assert_eq!(actual.view_factors[3].to_bits(), 0);
    assert_eq!(actual.view_factors[1].to_bits(), 0);
    assert_eq!(actual.view_factors[6], 1.0);
    assert_eq!(actual.view_factors[7], 1.0);
    assert_eq!(actual.zone_area_seen_m2, vec![8.0, 8.0, 6.0]);
    assert!(actual.warnings.is_empty());
    Ok(())
}

#[test]
fn positive_seen_area_below_epsilon_is_divided_in_both_interfaces()
-> Result<(), ApproximateViewFactorScopeError> {
    let tiny = f64::EPSILON / 2.0;
    let rows = [
        surface(5, SurfaceType::Wall, tiny, 0.0, 90.0),
        surface(6, SurfaceType::Wall, tiny, 180.0, 90.0),
    ];
    let actual = calculate_selected_approximate_view_factors(context(), &rows)?;
    assert_eq!(actual.zone_area_seen_m2[0].to_bits(), tiny.to_bits());
    assert_eq!(actual.view_factors[2], 1.0);
    assert_eq!(actual.view_factors[1], 1.0);
    assert!(actual.warnings.is_empty());
    let legacy = rows
        .iter()
        .map(|row| InteriorLongwaveSurfaceSnapshot {
            zone_id: row.zone_id,
            surface_type: row.surface_type,
            area_m2: row.area_m2,
            azimuth_deg: row.azimuth_deg,
            tilt_deg: row.tilt_deg,
            temperature_k4: 0.0,
            thermal_absorptance: 0.0,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        energyplus_approximate_view_factors(&legacy),
        actual.view_factors
    );
    Ok(())
}

#[test]
fn nonpositive_seen_area_keeps_zero_matrix_and_warning_identity_order()
-> Result<(), ApproximateViewFactorScopeError> {
    // Direct-helper diagnostic inputs, not fixed-CON production geometry.
    let rows = [
        surface(99, SurfaceType::Wall, -1.0, 0.0, 90.0),
        surface(2, SurfaceType::Wall, 0.0, 180.0, 90.0),
    ];
    let actual = calculate_selected_approximate_view_factors(context(), &rows)?;
    assert_eq!(actual.zone_area_seen_m2, vec![0.0, -1.0]);
    assert!(actual.view_factors.iter().all(|value| value.to_bits() == 0));
    assert_eq!(actual.warnings.len(), 2);
    for (index, warning) in actual.warnings.iter().enumerate() {
        assert_eq!(warning.surface_index, index);
        assert_eq!(warning.surface_id, rows[index].surface_id);
        assert_eq!(warning.surface_name, rows[index].surface_name);
        assert_eq!(warning.zone_id, rows[index].zone_id);
        assert_eq!(warning.zone_name, rows[index].zone_name);
    }
    Ok(())
}

#[test]
fn seen_area_accumulation_uses_actual_pointer_order_without_sorting()
-> Result<(), ApproximateViewFactorScopeError> {
    let rows = [
        surface(40, SurfaceType::Wall, 5.0, 0.0, 90.0),
        surface(8, SurfaceType::Wall, 1.0e16, 180.0, 90.0),
        surface(2, SurfaceType::Wall, 1.0, 180.0, 90.0),
        surface(1, SurfaceType::Wall, 1.0, 180.0, 90.0),
    ];
    let reordered = [
        rows[0].clone(),
        rows[2].clone(),
        rows[3].clone(),
        rows[1].clone(),
    ];
    let left = calculate_selected_approximate_view_factors(context(), &rows)?;
    let right = calculate_selected_approximate_view_factors(context(), &reordered)?;
    assert_eq!(left.zone_area_seen_m2[0].to_bits(), 1.0e16_f64.to_bits());
    assert_eq!(
        right.zone_area_seen_m2[0].to_bits(),
        (1.0e16_f64 + 2.0).to_bits()
    );
    assert_ne!(
        left.zone_area_seen_m2[0].to_bits(),
        right.zone_area_seen_m2[0].to_bits()
    );
    assert_eq!(left.ordered_surface_ids[1], SurfaceId(8));
    assert_eq!(right.ordered_surface_ids[1], SurfaceId(2));
    Ok(())
}

#[test]
fn association_errors_are_unavailable_and_empty_direct_helper_is_empty()
-> Result<(), ApproximateViewFactorScopeError> {
    let mut wrong_zone = surface(4, SurfaceType::Ceiling, 1.0, 0.0, 0.0);
    wrong_zone.zone_id = ZoneId(5);
    assert_eq!(
        calculate_selected_approximate_view_factors(context(), &[wrong_zone]),
        Err(ApproximateViewFactorScopeError::ZoneMismatch { surface_index: 0 })
    );
    let duplicate = surface(4, SurfaceType::Ceiling, 1.0, 0.0, 0.0);
    assert_eq!(
        calculate_selected_approximate_view_factors(context(), &[duplicate.clone(), duplicate]),
        Err(ApproximateViewFactorScopeError::DuplicateSurfaceId {
            first_index: 0,
            duplicate_index: 1,
        })
    );
    let empty = calculate_selected_approximate_view_factors(context(), &[])?;
    assert!(empty.ordered_surface_ids.is_empty());
    assert!(empty.zone_area_seen_m2.is_empty());
    assert!(empty.view_factors.is_empty());
    assert!(empty.warnings.is_empty());
    Ok(())
}
