use super::*;
use crate::heat_balance::ctf_first_assembly_owner::initialize_construction_ctf_first_assembly;
use crate::heat_balance::ctf_initial_owner::initialize_construction_ctf_initial;
use crate::heat_balance::ctf_retry_initialization::initialize_construction_ctf_retry;
use crate::heat_balance::surface_manager::ctf_all_resistive::{
    ConstructionCtfAllResistive, generate_all_resistive_ctf,
};
use crate::heat_balance::surface_manager::ctf_caller_stability::CtfCallerDiagnosticNames;
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::{
    ConstructionCtfLayerPreprocessing, CtfLayerContext, material_layer_inputs,
    preprocess_ctf_layers,
};
use ep_model::{
    AutoOrNumber, Construction, ConstructionKind, Material, NoMassMaterial, NormalizedName,
    OpaqueSurfaceProperties, SpaceId, SunExposure, Surface, SurfaceType, WindExposure, ZoneId,
};

type TestResult<T = ()> = Result<T, String>;

fn context() -> CtfSurfaceGenerationContext {
    CtfSurfaceGenerationContext {
        producer_revision: CtfSurfaceProducerRevision::Explicit([1; 32]),
        time_step_zone_hours: 0.25,
        initial_flags: CtfCallerSharedFlags {
            errors_found: false,
            do_ctf_error_report: false,
        },
        initial_simple_ctf_only: true,
        initial_max_ctf_terms: 0,
        any_ems_in_model: false,
        any_construction_overrides: false,
    }
}

fn construction(id: u32, layers: &[u32]) -> Construction {
    Construction {
        id: ConstructionId(id),
        name: NormalizedName(format!("C{id}")),
        kind: ConstructionKind::Opaque,
        outside_layer: layers.first().copied().map(MaterialId),
        layers: layers.iter().copied().map(MaterialId).collect(),
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    }
}

fn surface(id: u32, construction: u32) -> Surface {
    Surface {
        id: SurfaceId(id),
        name: NormalizedName(format!("S{id}")),
        surface_type: SurfaceType::Wall,
        construction: ConstructionId(construction),
        zone: ZoneId(9),
        space: SpaceId(3),
        outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::SunExposed,
        wind_exposure: WindExposure::WindExposed,
        view_factor_to_ground: AutoOrNumber::AutoCalculate,
        vertices: Vec::new(),
        computed_geometry: None,
    }
}

fn fixture() -> TestResult<(TypedModel, OrderedCtfPublicInitialization)> {
    let materials = [2_u32, 91]
        .into_iter()
        .map(|id| Material {
            id: MaterialId(id),
            name: NormalizedName(format!("M{id}")),
            definition: MaterialDefinition::NoMass(NoMassMaterial {
                roughness: MaterialSurfaceRoughness::Smooth,
                thermal_resistance_m2_k_per_w: 0.25,
                surface: OpaqueSurfaceProperties::default(),
            }),
        })
        .collect();
    let model = TypedModel {
        timestep: ep_model::TimestepConfig {
            number_of_timesteps_per_hour: 4,
        },
        materials,
        constructions: vec![construction(40, &[2, 91]), construction(8, &[91, 2])],
        surfaces: vec![surface(500, 8), surface(3, 40)],
        ..TypedModel::default()
    };
    let used = BTreeSet::from([40_u32, 8]);
    let mut flags = context().initial_flags;
    let mut public = OrderedCtfPublicInitialization::begin(true, 0);
    for (index, current) in model.constructions.iter().enumerate() {
        let actual_materials = current
            .effective_layers()
            .iter()
            .map(|id| {
                model
                    .materials
                    .iter()
                    .find(|value| value.id == *id)
                    .ok_or_else(|| format!("missing {id:?}"))
            })
            .collect::<TestResult<Vec<_>>>()?;
        let inputs = material_layer_inputs(&actual_materials)
            .map_err(|value| format!("inputs {value:?}"))?;
        let layer_context = CtfLayerContext {
            is_used_ctf: true,
            errors_found: flags.errors_found,
            source_sink_present: false,
            solution_dimensions: 1,
        };
        let phases = preprocess_ctf_layers(&inputs, layer_context)
            .map_err(|value| format!("phases {value:?}"))?;
        flags.errors_found = phases.errors_found;
        let all_resistive = ConstructionCtfAllResistive {
            construction_id: current.id,
            result: generate_all_resistive_ctf(&phases, context().time_step_zone_hours),
        };
        let preprocessing = ConstructionCtfLayerPreprocessing {
            construction_id: current.id,
            is_used_ctf: true,
            result: Ok(phases),
        };
        let initial = initialize_construction_ctf_initial(
            current,
            &model.constructions[..index],
            &used,
            &preprocessing,
            layer_context,
            context().time_step_zone_hours,
        );
        let first = initialize_construction_ctf_first_assembly(&initial, &preprocessing);
        let names = actual_materials
            .iter()
            .map(|value| value.name.0.as_str())
            .collect::<Vec<_>>();
        let retry = initialize_construction_ctf_retry(
            first,
            flags,
            CtfCallerDiagnosticNames {
                construction_name: current.name.0.as_str(),
                original_tot_layers: i32::try_from(names.len())
                    .map_err(|value| value.to_string())?,
                original_layer_material_names: &names,
            },
        );
        flags = retry.flags_on_return;
        public.record_ordinary(
            index,
            current,
            &preprocessing,
            Some(&initial),
            &all_resistive,
            &retry,
        );
    }
    public.finish_declared_loop(model.constructions.len());
    Ok((model, public))
}

fn capture<'a>(
    model: &'a TypedModel,
    public: &'a OrderedCtfPublicInitialization,
) -> TestResult<CtfPublicSurfaceGeneration<'a>> {
    capture_public_ctf_surface_generation(model, public, context())
        .map_err(|value| format!("capture {value:?}"))
}

#[test]
fn actual_non_dense_surface_and_construction_ids_select_the_borrowed_owner() -> TestResult {
    let (model, public) = fixture()?;
    let generation = capture(&model, &public)?;
    let bound = generation
        .bind_surface(SurfaceId(500))
        .map_err(|value| format!("bind {value:?}"))?;
    let actual = &public.constructions[1]
        .result
        .as_ref()
        .map_err(|value| format!("return {value:?}"))?
        .storage;
    assert_eq!(bound.construction_id, ConstructionId(8));
    assert_eq!(bound.declared_construction_index, 1);
    assert!(std::ptr::eq(bound.coefficients, actual));
    assert_eq!(bound.history.fixed_history_slots, 19);
    assert_eq!(bound.history.active_coefficient_slots, 2);
    assert!(bound.history.simple_history_shift_selected);
    assert!(!bound.history.master_histories_required);
    Ok(())
}

#[test]
fn all_twelve_fixed_tails_signed_zero_and_nan_payloads_are_exact_key_inputs() -> TestResult {
    let (model, mut public) = fixture()?;
    let before = capture(&model, &public)?.key().clone();
    for selected in 0..12 {
        let mut changed = public.clone();
        let storage = &mut changed.constructions[1]
            .result
            .as_mut()
            .map_err(|value| format!("return {value:?}"))?
            .storage;
        let arrays = [
            &mut storage.outside,
            &mut storage.cross,
            &mut storage.inside,
            &mut storage.flux,
            &mut storage.source_in,
            &mut storage.source_out,
            &mut storage.temperature_source_out,
            &mut storage.temperature_source_in,
            &mut storage.temperature_source_q,
            &mut storage.temperature_user_out,
            &mut storage.temperature_user_in,
            &mut storage.temperature_user_source,
        ];
        arrays[selected][18] = -0.0;
        assert_ne!(before, *capture(&model, &changed)?.key());
    }
    let storage = &mut public.constructions[1]
        .result
        .as_mut()
        .map_err(|value| format!("return {value:?}"))?
        .storage;
    storage.temperature_user_source[18] = -0.0;
    storage.outside[18] = f64::from_bits(0x7ff8_0000_0000_0042);
    let generation = capture(&model, &public)?;
    assert_ne!(before, *generation.key());
    let bound = generation
        .bind_surface(SurfaceId(500))
        .map_err(|value| format!("bind {value:?}"))?;
    assert_eq!(
        bound.coefficients.temperature_user_source[18].to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(
        bound.coefficients.outside[18].to_bits(),
        0x7ff8_0000_0000_0042
    );
    assert_eq!(generation.key(), capture(&model, &public)?.key());
    Ok(())
}

#[test]
fn stored_regular_members_order_timestep_caller_and_revision_invalidate_exactly() -> TestResult {
    let (mut model, public) = fixture()?;
    let original = capture(&model, &public)?.key().clone();
    // Contract-only input mutation; capture never regenerates or authenticates coefficients.
    model.materials[0].definition = MaterialDefinition::Regular(ep_model::RegularMaterial {
        roughness: MaterialSurfaceRoughness::Smooth,
        thickness_m: 0.2,
        conductivity_w_per_m_k: 0.8,
        density_kg_per_m3: 800.0,
        specific_heat_j_per_kg_k: 900.0,
        thermal_resistance_m2_k_per_w: 0.25,
        surface: OpaqueSurfaceProperties::default(),
    });
    let regular = capture(&model, &public)?.key().clone();
    assert_ne!(original, regular);
    if let MaterialDefinition::Regular(value) = &mut model.materials[0].definition {
        value.thermal_resistance_m2_k_per_w =
            f64::from_bits(value.thermal_resistance_m2_k_per_w.to_bits() + 1);
    }
    assert_ne!(regular, *capture(&model, &public)?.key());
    let before_order = capture(&model, &public)?.key().clone();
    model.materials.swap(0, 1);
    assert_ne!(before_order, *capture(&model, &public)?.key());
    let before_count = capture(&model, &public)?.key().clone();
    model.timestep.number_of_timesteps_per_hour = 6;
    assert_ne!(before_count, *capture(&model, &public)?.key());
    for changed in [
        CtfSurfaceGenerationContext {
            producer_revision: CtfSurfaceProducerRevision::Explicit([2; 32]),
            ..context()
        },
        CtfSurfaceGenerationContext {
            time_step_zone_hours: -0.0,
            ..context()
        },
        CtfSurfaceGenerationContext {
            initial_max_ctf_terms: 2,
            ..context()
        },
        CtfSurfaceGenerationContext {
            initial_flags: CtfCallerSharedFlags {
                errors_found: false,
                do_ctf_error_report: true,
            },
            ..context()
        },
    ] {
        assert_ne!(
            capture(&model, &public)?.key(),
            capture_public_ctf_surface_generation(&model, &public, changed)
                .map_err(|value| format!("changed {value:?}"))?
                .key()
        );
    }
    Ok(())
}

#[test]
fn original_reverse_provider_custody_is_checked_without_numeric_reselection() -> TestResult {
    let (model, mut public) = fixture()?;
    public.constructions[1]
        .result
        .as_mut()
        .map_err(|value| format!("return {value:?}"))?
        .route = CtfPublicReturnRoute::Reversed {
        construction_id: ConstructionId(40),
        declared_index: 0,
    };
    assert!(
        capture(&model, &public)?
            .bind_surface(SurfaceId(500))
            .is_ok()
    );
    public.constructions[1]
        .result
        .as_mut()
        .map_err(|value| format!("return {value:?}"))?
        .route = CtfPublicReturnRoute::Reversed {
        construction_id: ConstructionId(40),
        declared_index: 1,
    };
    assert_eq!(
        capture(&model, &public)?.bind_surface(SurfaceId(500)).err(),
        Some(CtfSurfaceBindingUnavailable::ReverseProvider(
            ConstructionId(40)
        ))
    );
    Ok(())
}

#[test]
fn actual_timestep_history_count_and_global_master_branch_remain_separate() -> TestResult {
    let (model, mut public) = fixture()?;
    public.aggregate_after_returned_calls.simple_ctf_only = false;
    public.aggregate_after_returned_calls.max_ctf_terms = 18;
    let storage = &mut public.constructions[1]
        .result
        .as_mut()
        .map_err(|value| format!("return {value:?}"))?
        .storage;
    storage.num_ctf_terms = 18;
    storage.num_histories = 3;
    storage.ctf_time_step = 0.75;
    let generation = capture(&model, &public)?;
    let bound = generation
        .bind_surface(SurfaceId(500))
        .map_err(|value| format!("bind {value:?}"))?;
    assert_eq!(bound.history.active_coefficient_slots, 19);
    assert_eq!(bound.history.num_histories, 3);
    assert_eq!(
        bound.history.ctf_time_step_hours.to_bits(),
        0.75_f64.to_bits()
    );
    assert_eq!(
        bound.history.time_step_zone_hours.to_bits(),
        0.25_f64.to_bits()
    );
    assert!(bound.history.master_histories_required);
    assert!(!bound.history.simple_history_shift_selected);
    Ok(())
}

#[test]
fn incomplete_error_severe_capacity_and_unsupported_paths_never_bind() -> TestResult {
    let (mut model, mut public) = fixture()?;
    public.selected_declared_loop_completed = false;
    assert_eq!(
        capture(&model, &public)?.bind_surface(SurfaceId(500)).err(),
        Some(CtfSurfaceBindingUnavailable::IncompleteDeclaredLoop)
    );
    public.selected_declared_loop_completed = true;
    public.constructions[0]
        .retry_flags_on_return
        .as_mut()
        .ok_or("missing flags")?
        .errors_found = true;
    assert_eq!(
        capture(&model, &public)?.bind_surface(SurfaceId(500)).err(),
        Some(CtfSurfaceBindingUnavailable::SharedErrors)
    );
    public.constructions[0]
        .retry_flags_on_return
        .as_mut()
        .ok_or("missing flags")?
        .errors_found = false;
    public.constructions[1]
        .result
        .as_mut()
        .map_err(|value| format!("return {value:?}"))?
        .route = CtfPublicReturnRoute::MassiveSevereBreak;
    assert_eq!(
        capture(&model, &public)?.bind_surface(SurfaceId(500)).err(),
        Some(CtfSurfaceBindingUnavailable::SevereReturn)
    );
    public.constructions[1]
        .result
        .as_mut()
        .map_err(|value| format!("return {value:?}"))?
        .route = CtfPublicReturnRoute::AllResistive;
    public.constructions[1]
        .result
        .as_mut()
        .map_err(|value| format!("return {value:?}"))?
        .storage
        .num_ctf_terms = 19;
    assert_eq!(
        capture(&model, &public)?.bind_surface(SurfaceId(500)).err(),
        Some(CtfSurfaceBindingUnavailable::HistoryShape)
    );
    model.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Ground;
    assert_eq!(
        capture_public_ctf_surface_generation(&model, &public, context()).err(),
        Some(CtfSurfaceBindingUnavailable::UnsupportedBoundary(
            SurfaceId(500)
        ))
    );
    model.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Outdoors;
    let dynamic = CtfSurfaceGenerationContext {
        any_construction_overrides: true,
        ..context()
    };
    assert_eq!(
        capture_public_ctf_surface_generation(&model, &public, dynamic).err(),
        Some(CtfSurfaceBindingUnavailable::DynamicControls)
    );
    Ok(())
}

#[test]
fn duplicate_ids_and_changed_original_association_are_explicit_errors() -> TestResult {
    let (mut model, mut public) = fixture()?;
    model.surfaces.push(model.surfaces[0].clone());
    assert_eq!(
        capture_public_ctf_surface_generation(&model, &public, context()).err(),
        Some(CtfSurfaceBindingUnavailable::DuplicateSurface(SurfaceId(
            500
        )))
    );
    model.surfaces.pop();
    public.constructions[1].original_material_ids.reverse();
    assert_eq!(
        capture_public_ctf_surface_generation(&model, &public, context()).err(),
        Some(CtfSurfaceBindingUnavailable::Association)
    );
    Ok(())
}

fn unread_glazing_material(id: MaterialId, name: &str) -> Material {
    Material {
        id,
        name: NormalizedName::new(name),
        definition: ep_model::MaterialDefinition::WindowGlazingSpectralAverage(
            ep_model::WindowGlazingSpectralAverageMaterial {
                thickness_m: 0.006,
                solar_transmittance_at_normal_incidence: 0.775,
                front_side_solar_reflectance_at_normal_incidence: 0.071,
                back_side_solar_reflectance_at_normal_incidence: 0.071,
                visible_transmittance_at_normal_incidence: 0.881,
                front_side_visible_reflectance_at_normal_incidence: 0.08,
                back_side_visible_reflectance_at_normal_incidence: 0.08,
                infrared_transmittance_at_normal_incidence: 0.0,
                front_side_infrared_hemispherical_emissivity: 0.84,
                back_side_infrared_hemispherical_emissivity: 0.84,
                conductivity_w_per_m_k: 0.9,
                dirt_correction_factor_for_solar_and_visible_transmittance: 1.0,
                solar_diffusing: false,
                youngs_modulus_pa: 72.0e9,
                poissons_ratio: 0.22,
            },
        ),
    }
}

#[test]
fn unread_unsupported_material_does_not_replace_used_ordinary_owner() -> TestResult {
    let (mut model, public) = fixture()?;
    let original = capture(&model, &public)?.key().clone();
    model
        .materials
        .push(unread_glazing_material(MaterialId(700), "Unused glass"));
    let generation = capture(&model, &public)?;
    assert_ne!(original, *generation.key());
    assert_eq!(generation.key(), capture(&model, &public)?.key());
    let bound = generation
        .bind_surface(SurfaceId(500))
        .map_err(|value| format!("used bind {value:?}"))?;
    let actual = &public.constructions[1]
        .result
        .as_ref()
        .map_err(|value| format!("used return {value:?}"))?
        .storage;
    assert!(std::ptr::eq(bound.coefficients, actual));
    Ok(())
}

#[test]
fn unread_excluded_declarations_keep_identity_order_and_variant_without_faking_return() -> TestResult
{
    let (mut model, mut public) = fixture()?;
    model
        .materials
        .push(unread_glazing_material(MaterialId(700), "Unused glass"));
    let mut unused = construction(701, &[700]);
    unused.kind = ConstructionKind::Fenestration;
    model.constructions.push(unused);
    public.record_excluded(2, &model.constructions[2], false);
    public.finish_declared_loop(model.constructions.len());
    let original = capture(&model, &public)?.key().clone();
    assert_eq!(
        capture(&model, &public)?.bind_surface(SurfaceId(500)).err(),
        Some(CtfSurfaceBindingUnavailable::SelectedStop(
            CtfPublicInitStop::Scope {
                construction_id: ConstructionId(701),
            }
        ))
    );
    assert!(!public.selected_declared_loop_completed);
    assert!(!public.constructions[2].public_initializer_invoked);
    assert_eq!(
        public.constructions[2].result,
        Err(CtfPublicInitializationUnavailable::ExcludedConstruction)
    );
    model.materials[2].definition =
        MaterialDefinition::InfraredTransparent(ep_model::InfraredTransparentMaterial);
    assert_ne!(original, *capture(&model, &public)?.key());
    let before_name = capture(&model, &public)?.key().clone();
    model.materials[2].name = NormalizedName::new("Renamed unread material");
    assert_ne!(before_name, *capture(&model, &public)?.key());
    let before_order = capture(&model, &public)?.key().clone();
    model.materials.swap(0, 2);
    assert_ne!(before_order, *capture(&model, &public)?.key());
    let before_kind = capture(&model, &public)?.key().clone();
    model.constructions[2].kind = ConstructionKind::ComplexFenestration;
    assert_ne!(before_kind, *capture(&model, &public)?.key());
    Ok(())
}

#[test]
fn real_used_membership_never_admits_unsupported_material_or_construction() -> TestResult {
    let (model, public) = fixture()?;
    let mut changed = model.clone();
    changed.materials[0].definition =
        MaterialDefinition::InfraredTransparent(ep_model::InfraredTransparentMaterial);
    assert_eq!(
        capture_public_ctf_surface_generation(&changed, &public, context()).err(),
        Some(CtfSurfaceBindingUnavailable::UnsupportedMaterial(
            MaterialId(2)
        ))
    );
    changed = model.clone();
    changed.constructions[1].kind = ConstructionKind::Fenestration;
    assert_eq!(
        capture_public_ctf_surface_generation(&changed, &public, context()).err(),
        Some(CtfSurfaceBindingUnavailable::UnsupportedConstruction(
            ConstructionId(8)
        ))
    );
    changed = model;
    changed
        .materials
        .push(unread_glazing_material(MaterialId(700), "Unused glass"));
    let mut unused = construction(701, &[700]);
    unused.kind = ConstructionKind::Fenestration;
    changed.constructions.push(unused);
    let mut actual_public = public;
    actual_public.record_excluded(2, &changed.constructions[2], false);
    actual_public.finish_declared_loop(changed.constructions.len());
    assert!(capture(&changed, &actual_public).is_ok());
    changed.surfaces[0].construction = ConstructionId(701);
    assert_eq!(
        capture_public_ctf_surface_generation(&changed, &actual_public, context()).err(),
        Some(CtfSurfaceBindingUnavailable::UnsupportedMaterial(
            MaterialId(700)
        ))
    );
    changed.surfaces[0].construction = ConstructionId(8);
    actual_public.constructions[2].is_used_ctf = true;
    assert_eq!(
        capture_public_ctf_surface_generation(&changed, &actual_public, context()).err(),
        Some(CtfSurfaceBindingUnavailable::Association)
    );
    Ok(())
}

#[test]
fn genuine_unused_reset_binds_only_exact_storage_and_separate_caller_flags() -> TestResult {
    let (mut model, mut public) = fixture()?;
    model
        .materials
        .push(unread_glazing_material(MaterialId(700), "Unused glass"));
    let mut unused = construction(701, &[700]);
    unused.kind = ConstructionKind::Fenestration;
    model.constructions.push(unused);
    public.record_unused_reset(2, &model.constructions[2], context().initial_flags);
    public.finish_declared_loop(model.constructions.len());
    let generation = capture(&model, &public)?;
    let actual = &public.constructions[1]
        .result
        .as_ref()
        .map_err(|value| format!("used return {value:?}"))?
        .storage;
    let bound = generation
        .bind_surface(SurfaceId(500))
        .map_err(|value| format!("bind after actual reset {value:?}"))?;
    assert!(std::ptr::eq(bound.coefficients, actual));
    let original = generation.key().clone();
    let mut changed = public.clone();
    changed.constructions[2]
        .unused_reset_flags
        .as_mut()
        .ok_or("unused flags missing")?
        .do_ctf_error_report = true;
    assert_ne!(original, *capture(&model, &changed)?.key());
    assert!(
        capture(&model, &changed)?
            .bind_surface(SurfaceId(500))
            .is_ok()
    );
    changed.constructions[2]
        .unused_reset_flags
        .as_mut()
        .ok_or("unused flags missing")?
        .errors_found = true;
    assert_eq!(
        capture(&model, &changed)?
            .bind_surface(SurfaceId(500))
            .err(),
        Some(CtfSurfaceBindingUnavailable::SharedErrors)
    );
    changed = public.clone();
    changed.constructions[2].unused_reset_flags = None;
    assert_eq!(
        capture(&model, &changed)?
            .bind_surface(SurfaceId(500))
            .err(),
        Some(CtfSurfaceBindingUnavailable::SharedErrors)
    );
    changed = public.clone();
    changed.constructions[2].retry_flags_on_entry = Some(context().initial_flags);
    assert_eq!(
        capture(&model, &changed)?
            .bind_surface(SurfaceId(500))
            .err(),
        Some(CtfSurfaceBindingUnavailable::Association)
    );
    changed = public;
    changed.constructions[2]
        .result
        .as_mut()
        .map_err(|value| format!("unused return {value:?}"))?
        .storage
        .outside[18] = -0.0;
    assert_eq!(
        capture(&model, &changed)?
            .bind_surface(SurfaceId(500))
            .err(),
        Some(CtfSurfaceBindingUnavailable::Association)
    );
    Ok(())
}
