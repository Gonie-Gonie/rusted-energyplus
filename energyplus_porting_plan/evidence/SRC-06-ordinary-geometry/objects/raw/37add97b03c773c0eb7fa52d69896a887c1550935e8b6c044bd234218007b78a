#[test]
fn ordinary_initializer_retains_its_real_zone_return_and_selected_geometry() -> Result<(), Box<dyn std::error::Error>> {
    let model = SimulationModel::from_typed(ctf_ordinary_output_test_model());
    let state = initialize_heat_balance_state(&model, 23.0)?;
    let owners = &state.initialized_geometry;
    assert_eq!(owners.zone_returns.len(), state.zones.len());
    let retained = owners.zone_returns.first().ok_or_else(|| std::io::Error::other("missing actual retained zone geometry"))?;
    assert_eq!(retained.zone_id, state.zones[0].zone_id);
    assert_eq!(retained.returned.volume_m3.to_bits(), state.zones[0].volume_m3.to_bits());
    assert_eq!(retained.returned.volume_calculation_count, 1);
    assert_eq!(retained.returned.faces.len(), state.surfaces.len());
    let setup = owners.opaque_setup.as_ref().map_err(|error| std::io::Error::other(format!("selected geometry unavailable: {error:?}")))?;
    assert_eq!(setup.surface_dimensions.len(), state.surfaces.len());
    assert_eq!(setup.zones[0].bounds_m, [0.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
    assert_eq!(setup.zones[0].exterior_gross_wall_area_m2, 4.0);
    assert_eq!(setup.zones[0].exterior_window_area_m2.to_bits(), 0.0_f64.to_bits());
    Ok(())
}

#[test]
fn missing_surface_source_output_is_unavailable_without_recalculation() -> Result<(), Box<dyn std::error::Error>> {
    let model = SimulationModel::from_typed(ctf_ordinary_output_test_model());
    let state = initialize_heat_balance_state(&model, 23.0)?;
    let mut actual = state.surfaces.iter().map(|s| (s.surface_id, s.geometry)).collect::<Vec<_>>();
    actual.pop();
    assert_eq!(crate::geometry::opaque_geometry_owners(&model.typed, &actual), Err(crate::geometry::GeometryOwnerUnavailable::Association));
    Ok(())
}

#[test]
fn empty_first_space_retains_constructor_bound_before_real_surface_reductions() -> Result<(), Box<dyn std::error::Error>> {
    let mut typed = ctf_ordinary_output_test_model();
    typed.spaces.push(fixture_default_space(7, ZoneId(0), "Empty first"));
    typed.zones[0].spaces.insert(0, ep_model::SpaceId(7));
    for surface in &mut typed.surfaces {
        for vertex in &mut surface.vertices {
            vertex.x_m += 10.0;
            vertex.y_m += 20.0;
            vertex.z_m += 30.0;
        }
    }
    let model = SimulationModel::from_typed(typed);
    let state = initialize_heat_balance_state(&model, 23.0)?;
    let setup = state.initialized_geometry.opaque_setup.as_ref()
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    assert_eq!(setup.zones[0].bounds_m, [0.0, 11.0, 0.0, 21.0, 0.0, 31.0]);
    Ok(())
}
