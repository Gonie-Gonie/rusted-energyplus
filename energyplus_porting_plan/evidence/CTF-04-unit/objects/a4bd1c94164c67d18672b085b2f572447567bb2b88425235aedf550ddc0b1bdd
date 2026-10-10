use super::*;
use crate::heat_balance::surface_manager::ctf_initial_discretization::{
    CtfInitialDiscretizationScopeError, discretize_selected_ctf_1d_initial,
};
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::{
    CtfConvertedLayers, CtfLayerCheckpoint, CtfLayerContext, CtfLayerPreprocessing,
    CtfNormalizedLayer,
};

fn owners() -> Result<
    (
        ConstructionCtfInitialDiscretization,
        ConstructionCtfLayerPreprocessing,
    ),
    CtfInitialDiscretizationScopeError,
> {
    // Literal converted-owner fixture; neither Native outputs nor a second
    // preprocessing call supplies the first assembly's actual operands.
    let converted = CtfConvertedLayers {
        active: CtfLayerCheckpoint {
            layers: vec![CtfNormalizedLayer {
                dl: 1.0,
                rk: 1.0,
                rho: 1000.0,
                cp: 1.0,
                lr: 1.0,
                res_layer: false,
            }],
            num_res_layers: 0,
        },
        dyn_spacing: 0.0,
        total_resistance: 1.0,
        conductance: 1.0,
    };
    let caller = 1.0;
    let nodal = discretize_selected_ctf_1d_initial(&converted, caller)?;
    Ok((
        ConstructionCtfInitialDiscretization {
            construction_id: ConstructionId(7),
            context: CtfLayerContext {
                is_used_ctf: true,
                errors_found: false,
                source_sink_present: false,
                solution_dimensions: 1,
            },
            time_step_zone_hours: caller,
            result: Ok(nodal),
        },
        ConstructionCtfLayerPreprocessing {
            construction_id: ConstructionId(7),
            is_used_ctf: true,
            result: Ok(CtfLayerPreprocessing {
                skipped_unused: false,
                errors_found: false,
                issues: Vec::new(),
                after_load: None,
                after_merge: None,
                after_conversion: Some(converted),
            }),
        },
    ))
}

#[test]
fn first_assembly_borrows_the_same_initial_and_converted_owners()
-> Result<(), Box<dyn std::error::Error>> {
    let (initial, preprocessing) = owners()?;
    let before_initial = initial.clone();
    let before_preprocessing = preprocessing.clone();
    let nodal = initial
        .result
        .as_ref()
        .map_err(|error| format!("{error:?}"))?;
    assert_ne!(
        nodal.initial_ctf_time_step_hours.to_bits(),
        initial.time_step_zone_hours.to_bits()
    );
    let copied = initialize_construction_ctf_first_assembly(&initial, &preprocessing);
    assert_eq!(copied.construction_id, initial.construction_id);
    let CtfAssemblyObservation::Assembled(actual) = copied.result? else {
        return Err("selected actual first assembly missing".into());
    };
    assert_eq!(actual.context.attempt_ordinal, 1);
    assert_eq!(
        actual.context.time_step_zone.to_bits(),
        initial.time_step_zone_hours.to_bits()
    );
    assert_eq!(
        actual.context.ctf_time_step.to_bits(),
        nodal.initial_ctf_time_step_hours.to_bits()
    );
    assert_eq!(actual.context.num_histories, nodal.initial_num_histories);
    assert_eq!(actual.rcmax, nodal.rcmax);
    assert_eq!(
        actual.nodes,
        nodal.active.iter().map(|row| row.nodes).collect::<Vec<_>>()
    );
    assert_eq!(
        actual.dx.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        nodal
            .active
            .iter()
            .map(|row| row.dx.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(initial, before_initial);
    assert_eq!(preprocessing, before_preprocessing);
    Ok(())
}

#[test]
fn unavailable_initial_routes_remain_distinct_without_an_assembly_owner()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut initial, preprocessing) = owners()?;
    for reason in [
        CtfInitialUnavailable::UnusedConstruction,
        CtfInitialUnavailable::PreprocessingErrorReturn,
        CtfInitialUnavailable::PostConversionUnavailable,
        CtfInitialUnavailable::AllResistiveBranch,
        CtfInitialUnavailable::ReverseConstruction {
            construction_id: ConstructionId(2),
        },
        CtfInitialUnavailable::PreprocessingScope(CtfLayerScopeError::InternalSource),
        CtfInitialUnavailable::InitialScope(
            CtfInitialDiscretizationScopeError::UnsupportedZoneTimeStep,
        ),
    ] {
        initial.result = Err(reason);
        assert_eq!(
            initialize_construction_ctf_first_assembly(&initial, &preprocessing).result,
            Err(CtfFirstAssemblyUnavailable::InitialUnavailable(reason))
        );
    }
    Ok(())
}

#[test]
fn stale_identity_and_caller_bits_cannot_bind_an_unrelated_prefix()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut initial, mut preprocessing) = owners()?;
    preprocessing.construction_id = ConstructionId(8);
    assert_eq!(
        initialize_construction_ctf_first_assembly(&initial, &preprocessing).result,
        Err(CtfFirstAssemblyUnavailable::ConstructionIdentityMismatch)
    );
    preprocessing.construction_id = initial.construction_id;
    initial.time_step_zone_hours = 0.25;
    assert_eq!(
        initialize_construction_ctf_first_assembly(&initial, &preprocessing).result,
        Err(CtfFirstAssemblyUnavailable::CallerTimeStepMismatch)
    );
    Ok(())
}

#[test]
fn absent_or_error_converted_owners_are_not_default_matrices()
-> Result<(), Box<dyn std::error::Error>> {
    let (initial, mut preprocessing) = owners()?;
    preprocessing.result = Err(CtfLayerScopeError::InternalSource);
    assert_eq!(
        initialize_construction_ctf_first_assembly(&initial, &preprocessing).result,
        Err(CtfFirstAssemblyUnavailable::PreprocessingScope(
            CtfLayerScopeError::InternalSource
        ))
    );
    let (_, mut preprocessing) = owners()?;
    let phases = preprocessing
        .result
        .as_mut()
        .map_err(|error| format!("{error:?}"))?;
    phases.after_conversion = None;
    assert_eq!(
        initialize_construction_ctf_first_assembly(&initial, &preprocessing).result,
        Err(CtfFirstAssemblyUnavailable::PostConversionUnavailable)
    );
    let phases = preprocessing
        .result
        .as_mut()
        .map_err(|error| format!("{error:?}"))?;
    phases.errors_found = true;
    assert_eq!(
        initialize_construction_ctf_first_assembly(&initial, &preprocessing).result,
        Err(CtfFirstAssemblyUnavailable::PreprocessingRouteMismatch)
    );
    Ok(())
}

#[test]
fn actual_initial_shape_error_is_retained_without_saturating_or_replacing_it()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut initial, preprocessing) = owners()?;
    let nodal = initial
        .result
        .as_mut()
        .map_err(|error| format!("{error:?}"))?;
    nodal.rcmax += 1;
    assert_eq!(
        initialize_construction_ctf_first_assembly(&initial, &preprocessing).result,
        Err(CtfFirstAssemblyUnavailable::AssemblyScope(
            CtfAssemblyScopeError::MatrixShape
        ))
    );
    Ok(())
}
