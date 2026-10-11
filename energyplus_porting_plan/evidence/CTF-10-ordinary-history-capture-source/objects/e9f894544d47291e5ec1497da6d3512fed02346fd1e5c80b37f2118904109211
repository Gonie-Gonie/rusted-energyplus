// Rust integration tests proposed after source review; no Native answer rows are used.
// The real SUR-04/SUR-07 Native observation obligations remain separate.

#[test]
fn ctf02_ordinary_initialization_carries_timestep_and_fresh_surface_history()
-> Result<(), Box<dyn std::error::Error>> {
    for steps in [1, 3, 4] {
        let mut typed = cube_model();
        typed.timestep.number_of_timesteps_per_hour = steps;
        let model = SimulationModel::from_typed(typed);
        // This public initializer supplies an empty coefficient override slice.
        let mut first = initialize_heat_balance_state(&model, 17.25)?;
        let surface = first
            .surfaces
            .first()
            .ok_or_else(|| std::io::Error::other("ordinary surface owner absent"))?;
        let owner = first
            .construction_ctf02_coefficients
            .iter()
            .find(|owner| owner.construction_id == surface.construction_id)
            .ok_or_else(|| std::io::Error::other("actual construction owner absent"))?;
        let generated = owner
            .result
            .map_err(|error| std::io::Error::other(format!("selected CTF-02 owner: {error:?}")))?;
        assert_eq!(
            generated.ctf_time_step.to_bits(),
            (1.0 / f64::from(steps)).to_bits()
        );
        assert_eq!((generated.num_histories, generated.num_ctf_terms), (1, 1));
        assert_eq!(
            surface.ctf.outside_0_w_per_m2_k.to_bits(),
            generated.outside[0].to_bits()
        );
        assert_eq!(
            surface.ctf.cross_0_w_per_m2_k.to_bits(),
            generated.cross[0].to_bits()
        );
        assert_eq!(
            surface.ctf.inside_0_w_per_m2_k.to_bits(),
            generated.inside[0].to_bits()
        );
        assert_eq!(
            surface.ctf.flux_0.map(f64::to_bits),
            Some(generated.flux[0].to_bits())
        );
        assert_eq!(
            surface
                .ctf
                .outside_history_w_per_m2_k
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            generated.outside[1..=generated.num_ctf_terms]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            surface
                .ctf
                .cross_history_w_per_m2_k
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            generated.cross[1..=generated.num_ctf_terms]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            surface
                .ctf
                .inside_history_w_per_m2_k
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            generated.inside[1..=generated.num_ctf_terms]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            surface
                .ctf
                .flux_history
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            generated.flux[1..=generated.num_ctf_terms]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        // Construction allocation precedes BeginEnvrn; the requested air
        // temperature is a separate owner and does not seed surface histories.
        assert_eq!(first.zones[0].mean_air_temperature_c, 17.25);
        assert_eq!(surface.ctf.inside_temperature_history_c, vec![0.0]);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![0.0]);
        assert_eq!(surface.ctf.inside_flux_history_w_per_m2, vec![0.0]);
        assert_eq!(surface.ctf.outside_flux_history_w_per_m2, vec![0.0]);
        // A local stale-state canary must not enter another ordinary initialization.
        first.surfaces[0].ctf.inside_temperature_history_c[0] = 99.0;
        let second = initialize_heat_balance_state(&model, 23.5)?;
        assert_eq!(second.zones[0].mean_air_temperature_c, 23.5);
        assert_eq!(
            second.surfaces[0].ctf.inside_temperature_history_c,
            vec![0.0]
        );
        assert_eq!(
            second.surfaces[0].ctf.outside_temperature_history_c,
            vec![0.0]
        );
        assert_eq!(
            second.surfaces[0].ctf.inside_flux_history_w_per_m2,
            vec![0.0]
        );
        assert_eq!(
            second.construction_ctf02_coefficients,
            first.construction_ctf02_coefficients
        );
    }
    Ok(())
}

#[test]
fn ctf02_generated_surface_owner_drives_inside_boundary_and_damping_operands()
-> Result<(), Box<dyn std::error::Error>> {
    let model = SimulationModel::from_typed(cube_model());
    let state = initialize_heat_balance_state(&model, 20.0)?;
    let mut surface = state
        .surfaces
        .first()
        .ok_or_else(|| std::io::Error::other("ordinary surface owner absent"))?
        .clone();
    assert_eq!(surface.ctf.outside_history_w_per_m2_k.len(), 1);
    surface.inside_additional_heat_source_w_per_m2 = 3.0;
    let input = CtfInsideFaceBalanceInput {
        reference_air_temperature_c: 21.0,
        inside_convection_coefficient_w_per_m2_k: 2.0,
        previous_inside_face_temperature_c: 19.0,
        zone_timestep_seconds: 3600.0,
    };
    surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;
    surface.outside_face_temperature_c = 5.0;
    let outdoor_cold = energyplus_ctf_inside_face_temperature_c(&surface, input);
    surface.outside_face_temperature_c = 30.0;
    let outdoor_warm = energyplus_ctf_inside_face_temperature_c(&surface, input);
    assert!(outdoor_cold.is_finite() && outdoor_warm.is_finite());
    assert!(outdoor_warm > outdoor_cold);
    surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
    let adiabatic_warm = energyplus_ctf_inside_face_temperature_c(&surface, input);
    surface.outside_face_temperature_c = 5.0;
    let adiabatic_cold = energyplus_ctf_inside_face_temperature_c(&surface, input);
    assert_eq!(adiabatic_warm.to_bits(), adiabatic_cold.to_bits());
    let changed_previous = energyplus_ctf_inside_face_temperature_c(
        &surface,
        CtfInsideFaceBalanceInput {
            previous_inside_face_temperature_c: 24.0,
            ..input
        },
    );
    assert!(changed_previous > adiabatic_cold);
    Ok(())
}

#[test]
fn ctf02_generated_surface_owner_retains_report_signs_phase_and_history_commit()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::heat_balance::ctf::{
        surface_inside_conduction_rate_w_for_report, surface_outside_conduction_rate_w_for_report,
    };
    let model = SimulationModel::from_typed(cube_model());
    let mut state = initialize_heat_balance_state(&model, 20.0)?;
    let surface = state
        .surfaces
        .first_mut()
        .ok_or_else(|| std::io::Error::other("ordinary surface owner absent"))?;
    surface.inside_face_temperature_c = 22.0;
    surface.outside_face_temperature_c = 8.0;
    surface.inside_ctf_outside_temperature_c = surface.outside_face_temperature_c;
    let coefficients_before = surface.ctf.clone();
    update_surface_ctf_history_constants(surface);
    let inside_flux = surface_inside_conduction_flux_w_per_m2(surface);
    let outside_internal_flux = surface_outside_conduction_flux_w_per_m2(surface);
    let inside_rate = surface_inside_conduction_rate_w(surface);
    let outside_report_rate = surface_outside_conduction_rate_w(surface);
    assert!(inside_rate < 0.0 && outside_report_rate > 0.0);
    assert_eq!(inside_rate, surface.area_m2 * inside_flux);
    assert_eq!(
        outside_report_rate,
        -surface.area_m2 * outside_internal_flux
    );
    assert_eq!(
        surface_heat_storage_rate_w(inside_rate, outside_report_rate),
        -(inside_rate + outside_report_rate)
    );
    // A later report-face value must not replace the retained inside-update operand.
    surface.outside_face_temperature_c = 30.0;
    assert_eq!(
        surface_inside_conduction_rate_w_for_report(surface, true),
        inside_rate
    );
    assert_eq!(
        surface_outside_conduction_rate_w_for_report(surface, true),
        outside_report_rate
    );
    assert_ne!(
        surface_inside_conduction_rate_w_for_report(surface, false),
        inside_rate
    );
    let committed_outside_temperature_c = surface.inside_ctf_outside_temperature_c;
    advance_surface_ctf_histories_with_outside_temperature_override(
        surface,
        Some(committed_outside_temperature_c),
    );
    assert_eq!(surface.ctf.inside_temperature_history_c, vec![22.0]);
    assert_eq!(surface.ctf.outside_temperature_history_c, vec![8.0]);
    assert_eq!(surface.ctf.inside_flux_history_w_per_m2, vec![inside_flux]);
    assert_eq!(
        surface.ctf.outside_flux_history_w_per_m2,
        vec![outside_internal_flux]
    );
    assert_eq!(
        surface.ctf.outside_0_w_per_m2_k.to_bits(),
        coefficients_before.outside_0_w_per_m2_k.to_bits()
    );
    assert_eq!(
        surface.ctf.outside_history_w_per_m2_k,
        coefficients_before.outside_history_w_per_m2_k
    );
    assert_eq!(
        surface.ctf.cross_history_w_per_m2_k,
        coefficients_before.cross_history_w_per_m2_k
    );
    assert_eq!(
        surface.ctf.inside_history_w_per_m2_k,
        coefficients_before.inside_history_w_per_m2_k
    );
    assert_eq!(surface.ctf.flux_history, coefficients_before.flux_history);
    Ok(())
}

// Authored IEEE counterexamples for the pinned Source expression, not Native answers.
fn sur04_inside_expression_surface()
-> Result<crate::heat_balance::state::SurfaceHeatBalanceState, Box<dyn std::error::Error>> {
    let model = SimulationModel::from_typed(cube_model());
    let state = initialize_heat_balance_state(&model, 20.0)?;
    let mut surface = state
        .surfaces
        .first()
        .ok_or_else(|| std::io::Error::other("ordinary surface owner absent"))?
        .clone();
    surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;
    surface.outside_face_temperature_c = 0.0;
    surface.inside_face_temperature_c = 19.0;
    surface.ctf.inside_0_w_per_m2_k = 2.0;
    surface.ctf.cross_0_w_per_m2_k = 0.0;
    surface.ctf.const_in_part_w_per_m2 = 5.0;
    surface.inside_radiant_internal_gain_w_per_m2 = 0.0;
    surface.inside_shortwave_absorbed_w_per_m2 = 0.0;
    surface.inside_additional_heat_source_w_per_m2 = 0.0;
    surface.inside_radiant_hvac_w_per_m2 = 0.0;
    surface.inside_net_longwave_w_per_m2 = 0.0;
    Ok(surface)
}

fn sur04_inside_expression_input() -> CtfInsideFaceBalanceInput {
    CtfInsideFaceBalanceInput {
        reference_air_temperature_c: 0.0,
        inside_convection_coefficient_w_per_m2_k: 0.0,
        previous_inside_face_temperature_c: 0.0,
        zone_timestep_seconds: 3600.0,
    }
}

#[test]
fn sur04_inside_expression_multiplies_the_stored_reciprocal()
-> Result<(), Box<dyn std::error::Error>> {
    let surface = sur04_inside_expression_surface()?;
    let result = energyplus_ctf_inside_face_temperature_c(&surface, sur04_inside_expression_input());
    // 5 * binary64(1 / 7) is one ULP below binary64(5 / 7).
    assert_eq!(result.to_bits(), 0x3fe6_db6d_b6db_6db6);
    assert_ne!(result.to_bits(), (5.0_f64 / 7.0).to_bits());
    Ok(())
}

#[test]
fn sur04_inside_expression_keeps_gain_order_and_adds_net_longwave_after_temp_term()
-> Result<(), Box<dyn std::error::Error>> {
    let mut surface = sur04_inside_expression_surface()?;
    surface.ctf.const_in_part_w_per_m2 = 9_007_199_254_740_992.0;
    surface.inside_radiant_internal_gain_w_per_m2 = 1.0;
    surface.inside_shortwave_absorbed_w_per_m2 = -9_007_199_254_740_992.0;
    surface.inside_net_longwave_w_per_m2 = 1.0;
    let input = sur04_inside_expression_input();
    let result = energyplus_ctf_inside_face_temperature_c(&surface, input);
    // (2^53 + 1) rounds before the shortwave cancellation; net LW remains active.
    assert_eq!(result.to_bits(), 0x3fc2_4924_9249_2492);
    surface.inside_net_longwave_w_per_m2 = 2.0;
    let changed = energyplus_ctf_inside_face_temperature_c(&surface, input);
    assert_eq!(changed.to_bits(), 0x3fd2_4924_9249_2492);
    Ok(())
}

#[test]
fn sur04_inside_expression_does_not_replace_a_zero_denominator_with_previous_temperature()
-> Result<(), Box<dyn std::error::Error>> {
    let mut surface = sur04_inside_expression_surface()?;
    surface.ctf.inside_0_w_per_m2_k = -5.0;
    let result = energyplus_ctf_inside_face_temperature_c(&surface, sur04_inside_expression_input());
    assert!(result.is_infinite() && result.is_sign_positive());
    assert_ne!(result.to_bits(), surface.inside_face_temperature_c.to_bits());
    Ok(())
}

#[test]
fn sur04_inside_expression_retains_the_zone_seconds_operand_even_with_no_afn()
-> Result<(), Box<dyn std::error::Error>> {
    let surface = sur04_inside_expression_surface()?;
    let result = energyplus_ctf_inside_face_temperature_c(
        &surface,
        CtfInsideFaceBalanceInput {
            zone_timestep_seconds: 0.0,
            ..sur04_inside_expression_input()
        },
    );
    // This authored invalid-clock boundary is outside ordinary admission; 0 / 0 stays NaN.
    assert!(result.is_nan());
    Ok(())
}
