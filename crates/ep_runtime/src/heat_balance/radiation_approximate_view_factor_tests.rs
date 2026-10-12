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

// Genuine Native factual report ae64c773: all four actual calls admitted.
// The complete actual cases fixture is copied unchanged; no sampling or projection.
#[test]
fn actual_native_scriptf_lu_calls_replay_complete_working_and_inverse_bits()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/ordinary_radiation_lu_native_cases.json"
    ))?;
    assert_eq!(
        fixture["schema"].as_str(),
        Some("src06-actual-native-radiation-lu-cases.v1")
    );
    assert_eq!(fixture["all_actual_calls_preserved"].as_bool(), Some(true));
    assert_eq!(
        fixture["comparison_policy"]["sha256"].as_str(),
        Some("6319883465b7a3d6d7006f9bbedfb7e5780de65bb516aa57a1ff9fa3389b958f")
    );
    let cases = fixture["cases"]
        .as_array()
        .ok_or("actual LU cases absent")?;
    let expected_calls: usize = 4;
    let expected_admitted: usize = 4;
    assert!(expected_calls > 0 && expected_admitted > 0);
    assert_eq!(cases.len(), expected_calls);
    let admitted = cases
        .iter()
        .filter(|case| case["numerical_admitted"].as_bool() == Some(true))
        .count();
    assert_eq!(admitted, expected_admitted);
    assert_eq!(cases.len() - admitted, expected_calls - expected_admitted);
    // An excluded actual call stays in the table and prevents complete-table
    // numerical success. No nonfinite/nonpositive call is silently skipped.
    assert_eq!(
        expected_admitted, expected_calls,
        "actual factual report contains numerical exclusions"
    );

    let decode = |matrix: &serde_json::Value,
                  dimension: usize|
     -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let extent = dimension
            .checked_mul(dimension)
            .ok_or("actual LU extent overflow")?;
        let rows = matrix["flat_values"]
            .as_array()
            .ok_or("actual initialized LU matrix absent")?;
        assert_eq!(rows.len(), extent);
        assert_eq!(matrix["size"].as_u64(), Some(u64::try_from(extent)?));
        assert_eq!(
            matrix["contents_initialized_by_actual_producer"].as_bool(),
            Some(true)
        );
        rows.iter()
            .enumerate()
            .map(|(offset, row)| {
                assert_eq!(row["offset"].as_u64(), Some(u64::try_from(offset)?));
                let bits = row["value"]["value_bits"]
                    .as_str()
                    .ok_or("actual LU binary64 bits absent")?;
                assert_eq!(bits.len(), 16);
                let value = f64::from_bits(u64::from_str_radix(bits, 16)?);
                assert!(
                    value.is_finite(),
                    "actual LU matrix is numerically excluded"
                );
                Ok(value)
            })
            .collect()
    };
    for (call_index, case) in cases.iter().enumerate() {
        assert_eq!(case["operation"].as_str(), Some("radiation_lu_inverse"));
        assert_eq!(case["numerical_admitted"].as_bool(), Some(true));
        assert!(
            case["numerical_exclusions"]
                .as_array()
                .ok_or("actual LU exclusions ledger absent")?
                .is_empty()
        );
        let dimension = usize::try_from(
            case["dimension"]
                .as_u64()
                .ok_or("actual LU dimension absent")?,
        )?;
        assert!(dimension > 0);
        assert!(case["inverse_entry"]["matrices"]["I"]["flat_values"].is_null());
        let mut working = decode(&case["inverse_entry"]["matrices"]["A"], dimension)?;
        let expected_working = decode(&case["inverse_return"]["matrices"]["A"], dimension)?;
        let expected_inverse = decode(&case["inverse_return"]["matrices"]["I"], dimension)?;
        // Inputs come only from the actual entry A. Native returned A/I remain
        // test expectations and are never supplied to the real helper.
        let inverse = invert_square_matrix(&mut working, dimension)
            .ok_or("real Rust inverse unavailable for admitted actual Native input")?;
        assert_eq!(working.len(), expected_working.len());
        assert_eq!(inverse.len(), expected_inverse.len());
        for (offset, (actual, expected)) in working.iter().zip(&expected_working).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "actual Native call {call_index}, reduced A offset {offset}"
            );
        }
        for (offset, (actual, expected)) in inverse.iter().zip(&expected_inverse).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "actual Native call {call_index}, inverse I offset {offset}"
            );
        }
    }
    Ok(())
}
