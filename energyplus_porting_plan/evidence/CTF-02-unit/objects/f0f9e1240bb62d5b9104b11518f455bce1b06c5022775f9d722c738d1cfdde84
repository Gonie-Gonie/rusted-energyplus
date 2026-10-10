use super::*;
use crate::heat_balance::ctf::all_resistive_surface_ctf_state;
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::{
    CtfLayerContext, CtfLayerInput, preprocess_ctf_layers,
};
use ep_model::MaterialId;

fn context() -> CtfLayerContext {
    CtfLayerContext {
        is_used_ctf: true,
        errors_found: false,
        source_sink_present: false,
        solution_dimensions: 1,
    }
}

fn no_mass(id: u32, resistance: f64) -> CtfLayerInput {
    CtfLayerInput {
        material_id: MaterialId(id),
        thickness: 0.0,
        conductivity: 0.0,
        density: 0.0,
        specific_heat: 0.0,
        resistance,
        resistance_only: true,
    }
}

#[test]
fn selected_storage_keeps_one_history_term_and_signed_zero() -> Result<(), String> {
    let prefix = preprocess_ctf_layers(&[no_mass(1, 2.0)], context())
        .map_err(|error| format!("prefix: {error:?}"))?;
    let result = generate_all_resistive_ctf(&prefix, 0.25)
        .map_err(|error| format!("all-resistive: {error:?}"))?;
    assert_eq!((result.num_histories, result.num_ctf_terms), (1, 1));
    assert_eq!(result.ctf_time_step.to_bits(), 0.25_f64.to_bits());
    assert_eq!(result.outside[1].to_bits(), 0.0_f64.to_bits());
    assert_eq!(result.cross[1].to_bits(), 0.0_f64.to_bits());
    assert_eq!(result.inside[1].to_bits(), (-0.0_f64).to_bits());
    assert_eq!(result.flux[1].to_bits(), (-0.0_f64).to_bits());
    assert_eq!(result.flux[0].to_bits(), 0.0_f64.to_bits());
    for array in [&result.outside, &result.cross, &result.inside, &result.flux] {
        assert!(array[2..].iter().all(|value| value.to_bits() == 0));
    }
    assert_eq!(result.outside[0].to_bits(), result.cross[0].to_bits());
    assert_eq!(result.outside[0].to_bits(), result.inside[0].to_bits());
    assert_eq!(result.outside[0].to_bits(), result.u_value.to_bits());
    Ok(())
}

#[test]
fn branch_uses_computed_classification_instead_of_material_ronly() -> Result<(), String> {
    let mut regular = no_mass(1, 2.0);
    regular.resistance_only = false;
    let prefix = preprocess_ctf_layers(&[regular], context())
        .map_err(|error| format!("classified regular: {error:?}"))?;
    let converted = prefix
        .after_conversion
        .as_ref()
        .ok_or_else(|| "converted classified regular absent".to_string())?;
    assert_eq!(
        converted.active.num_res_layers,
        converted.active.layers.len()
    );
    assert!(generate_all_resistive_ctf(&prefix, 0.25).is_ok());

    let massive = CtfLayerInput {
        thickness: 0.2,
        conductivity: 1.0,
        density: 1000.0,
        specific_heat: 1000.0,
        resistance: 0.2,
        resistance_only: false,
        ..regular
    };
    let prefix = preprocess_ctf_layers(&[massive], context())
        .map_err(|error| format!("massive prefix: {error:?}"))?;
    assert_eq!(
        generate_all_resistive_ctf(&prefix, 0.25),
        Err(CtfAllResistiveUnavailable::MassiveBranchNotSelected)
    );
    Ok(())
}

#[test]
fn unavailable_prefix_states_do_not_produce_zero_coefficients() -> Result<(), String> {
    let unused = preprocess_ctf_layers(
        &[],
        CtfLayerContext {
            is_used_ctf: false,
            errors_found: true,
            ..context()
        },
    )
    .map_err(|error| format!("unused prefix: {error:?}"))?;
    assert_eq!(
        generate_all_resistive_ctf(&unused, 0.25),
        Err(CtfAllResistiveUnavailable::UnusedConstruction)
    );
    let failed = preprocess_ctf_layers(&[no_mass(1, 0.0001)], context())
        .map_err(|error| format!("source error prefix: {error:?}"))?;
    assert_eq!(
        generate_all_resistive_ctf(&failed, 0.25),
        Err(CtfAllResistiveUnavailable::PreprocessingErrorReturn)
    );
    let mut unavailable = preprocess_ctf_layers(&[no_mass(1, 2.0)], context())
        .map_err(|error| format!("available prefix: {error:?}"))?;
    unavailable.after_conversion = None;
    assert_eq!(
        generate_all_resistive_ctf(&unavailable, 0.25),
        Err(CtfAllResistiveUnavailable::PostConversionUnavailable)
    );
    Ok(())
}

#[test]
fn assigned_cnd_is_consumed_without_resumming_the_converted_layers() -> Result<(), String> {
    let mut prefix = preprocess_ctf_layers(
        &[no_mass(1, 1.0e16), no_mass(2, 1.0), no_mass(3, 1.0)],
        context(),
    )
    .map_err(|error| format!("ordered CFU prefix: {error:?}"))?;
    let before = generate_all_resistive_ctf(&prefix, 0.25)
        .map_err(|error| format!("assigned cnd result: {error:?}"))?;
    // Dependency canary only: the source has already assigned cnd at401.
    // Altering a later copy of lr must not cause this branch to recalculate it.
    let converted = prefix
        .after_conversion
        .as_mut()
        .ok_or_else(|| "converted owner absent".to_string())?;
    for layer in &mut converted.active.layers {
        layer.lr = 0.0;
    }
    let after = generate_all_resistive_ctf(&prefix, 0.25)
        .map_err(|error| format!("cnd dependency canary: {error:?}"))?;
    assert_eq!(
        before.outside.map(f64::to_bits),
        after.outside.map(f64::to_bits)
    );
    assert_eq!(before.u_value.to_bits(), after.u_value.to_bits());
    Ok(())
}

#[test]
fn fresh_generation_and_actual_surface_copy_preserve_shape_and_bits() -> Result<(), String> {
    let prefix = preprocess_ctf_layers(&[no_mass(1, 2.0)], context())
        .map_err(|error| format!("prefix: {error:?}"))?;
    let mut old = generate_all_resistive_ctf(&prefix, 0.25)
        .map_err(|error| format!("first result: {error:?}"))?;
    old.outside[18] = 42.0;
    let current = generate_all_resistive_ctf(&prefix, 1.0 / 3.0)
        .map_err(|error| format!("fresh result: {error:?}"))?;
    assert_ne!(old.outside[18].to_bits(), current.outside[18].to_bits());
    assert_eq!(current.ctf_time_step.to_bits(), (1.0_f64 / 3.0).to_bits());
    let surface = all_resistive_surface_ctf_state(&current, 20.0);
    assert_eq!(
        surface.outside_0_w_per_m2_k.to_bits(),
        current.outside[0].to_bits()
    );
    assert_eq!(
        surface.cross_0_w_per_m2_k.to_bits(),
        current.cross[0].to_bits()
    );
    assert_eq!(
        surface.inside_0_w_per_m2_k.to_bits(),
        current.inside[0].to_bits()
    );
    assert_eq!(
        surface.flux_0.map(f64::to_bits),
        Some(current.flux[0].to_bits())
    );
    assert_eq!(surface.outside_history_w_per_m2_k.len(), 1);
    assert_eq!(surface.cross_history_w_per_m2_k.len(), 1);
    assert_eq!(
        surface.inside_history_w_per_m2_k[0].to_bits(),
        current.inside[1].to_bits()
    );
    assert_eq!(surface.flux_history[0].to_bits(), current.flux[1].to_bits());
    assert_eq!(surface.inside_temperature_history_c, vec![20.0]);
    Ok(())
}

#[test]
fn real_cache_ingress_uses_typed_timestep_and_retains_eio_precedence() -> Result<(), String> {
    use crate::heat_balance::ctf::ConstructionCtfCoefficientOverride;
    use crate::heat_balance::surface_manager::{
        ConstructionCtfCoefficientSource, ConstructionThermalDataCache,
    };
    use ep_model::{
        AutoOrNumber, Construction, ConstructionId, ConstructionKind, Material, MaterialDefinition,
        MaterialSurfaceRoughness, NoMassMaterial, NormalizedName, OpaqueSurfaceProperties,
        OutsideBoundaryCondition, SpaceId, SunExposure, Surface, SurfaceId, SurfaceType,
        TypedModel, WindExposure, ZoneId,
    };
    use std::collections::BTreeMap;

    // Only the actual cache ingress is exercised; no geometry/zone solve is claimed.
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = 3;
    model.materials.push(Material {
        id: MaterialId(0),
        name: NormalizedName::new("R"),
        definition: MaterialDefinition::NoMass(NoMassMaterial {
            roughness: MaterialSurfaceRoughness::MediumRough,
            thermal_resistance_m2_k_per_w: 2.0,
            surface: OpaqueSurfaceProperties::default(),
        }),
    });
    model.constructions.push(Construction {
        id: ConstructionId(0),
        name: NormalizedName::new("C"),
        kind: ConstructionKind::Opaque,
        outside_layer: Some(MaterialId(0)),
        layers: vec![MaterialId(0)],
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    });
    model.surfaces.push(Surface {
        id: SurfaceId(0),
        name: NormalizedName::new("S"),
        surface_type: SurfaceType::Wall,
        construction: ConstructionId(0),
        zone: ZoneId(0),
        space: SpaceId(0),
        outside_boundary_condition: OutsideBoundaryCondition::Adiabatic,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::NoSun,
        wind_exposure: WindExposure::NoWind,
        view_factor_to_ground: AutoOrNumber::Value(0.0),
        vertices: Vec::new(),
        computed_geometry: None,
    });
    let cache = ConstructionThermalDataCache::build(&model, &BTreeMap::new())
        .map_err(|error| format!("actual cache ingress: {error:?}"))?;
    let owners = cache.all_resistive_coefficients();
    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].construction_id, ConstructionId(0));
    let generated = owners[0]
        .result
        .map_err(|error| format!("cache result: {error:?}"))?;
    assert_eq!(generated.ctf_time_step.to_bits(), (1.0_f64 / 3.0).to_bits());
    let entry = cache
        .data_for_surface(&model.surfaces[0])
        .map_err(|error| format!("surface cache lookup: {error:?}"))?;
    assert_eq!(
        entry.ctf_coefficient_source,
        ConstructionCtfCoefficientSource::RustGeneratedAllResistive
    );

    let row = ConstructionCtfCoefficientOverride {
        construction_name: "C".to_string(),
        time_index: 0,
        outside_w_per_m2_k: 7.0,
        cross_w_per_m2_k: 8.0,
        inside_w_per_m2_k: 9.0,
        flux: None,
    };
    let overrides = BTreeMap::from([("C".to_string(), vec![&row])]);
    let cache = ConstructionThermalDataCache::build(&model, &overrides)
        .map_err(|error| format!("diagnostic cache ingress: {error:?}"))?;
    let entry = cache
        .data_for_surface(&model.surfaces[0])
        .map_err(|error| format!("diagnostic surface lookup: {error:?}"))?;
    assert_eq!(
        entry.ctf_coefficient_source,
        ConstructionCtfCoefficientSource::EnergyPlusEioSeeded
    );
    assert_eq!(entry.ctf_coefficients, vec![row]);
    Ok(())
}
