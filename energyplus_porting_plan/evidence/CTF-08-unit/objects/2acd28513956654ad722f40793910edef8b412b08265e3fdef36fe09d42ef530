use super::*;
use crate::heat_balance::ctf_first_gamma_owner::initialize_construction_ctf_first_gammas;
use crate::heat_balance::ctf_first_matrix_owner::initialize_construction_ctf_first_matrix_functions;
use crate::heat_balance::ctf_initial_owner::CtfInitialUnavailable;
use crate::heat_balance::surface_manager::ctf_exponential_matrix::CtfExponentialMatrix;
use crate::heat_balance::surface_manager::ctf_gamma_matrix::CtfGammaMatrix;
use crate::heat_balance::surface_manager::ctf_inverse_matrix::CtfInverseScopeError;
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::CtfNormalizedLayer;
use crate::heat_balance::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyInput, CtfStateSpaceAssembly, assemble_1d_ctf_state_space,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
type Owners = (
    ConstructionCtfFirstAssembly,
    ConstructionCtfFirstMatrixFunctions,
    ConstructionCtfFirstGammas,
);

fn actual_owners() -> TestResult<Owners> {
    // Source-derived literal unit operands, never observed Native answers.
    let layers = [CtfNormalizedLayer {
        dl: 0.5,
        rk: 0.25,
        rho: 1.0,
        cp: 1.0,
        lr: 2.0,
        res_layer: false,
    }];
    let context = CtfAssemblyContext {
        construction_id: ConstructionId(8),
        route: CtfAssemblyRoute::Assemble,
        solution_dimensions: 1,
        source_sink_present: false,
        node_source: 0,
        node_user_temp: 0,
        attempt_ordinal: 1,
        time_step_zone: 0.25,
        ctf_time_step: 0.5,
        num_histories: 2,
    };
    let observed = assemble_1d_ctf_state_space(
        CtfAssemblyInput {
            layers: &layers,
            nodes: &[6],
            dx: &[0.1],
            rcmax: 5,
        },
        context,
    )
    .map_err(|error| std::io::Error::other(format!("{error:?}")))?;
    let assembly = ConstructionCtfFirstAssembly {
        construction_id: context.construction_id,
        result: Ok(observed),
    };
    let methods = initialize_construction_ctf_first_matrix_functions(&assembly);
    let gammas = initialize_construction_ctf_first_gammas(&assembly, &methods);
    Ok((assembly, methods, gammas))
}

fn assembled(owner: &mut ConstructionCtfFirstAssembly) -> TestResult<&mut CtfStateSpaceAssembly> {
    match owner.result.as_mut().map_err(|reason| *reason)? {
        CtfAssemblyObservation::Assembled(actual) => Ok(actual),
        CtfAssemblyObservation::Unavailable(_) => {
            Err(std::io::Error::other("test assembly unavailable").into())
        }
    }
}
fn exponential(
    owner: &mut ConstructionCtfFirstMatrixFunctions,
) -> TestResult<&mut CtfExponentialMatrix> {
    match owner
        .exponential
        .result
        .as_mut()
        .map_err(|reason| *reason)?
    {
        CtfExponentialObservation::Exponential(actual) => Ok(actual),
        CtfExponentialObservation::Unavailable(_) => {
            Err(std::io::Error::other("test exponential unavailable").into())
        }
    }
}
fn gamma(owner: &mut ConstructionCtfFirstGammas) -> TestResult<&mut CtfGammaMatrix> {
    match owner.result.as_mut().map_err(|reason| *reason)? {
        CtfGammaObservation::Gamma(actual) => Ok(actual),
        CtfGammaObservation::Unavailable(_) => {
            Err(std::io::Error::other("test Gamma unavailable").into())
        }
    }
}

#[test]
fn real_ordered_predecessors_feed_private_owner_without_mutation() -> TestResult {
    let (assembly, methods, gammas) = actual_owners()?;
    let before = (assembly.clone(), methods.clone(), gammas.clone());
    let output = initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas);
    let CtfFinalCoefficientObservation::Coefficients(actual) = output.result? else {
        return Err(std::io::Error::other("test final coefficients unavailable").into());
    };
    assert_eq!(output.construction_id, ConstructionId(8));
    assert_eq!(actual.context.attempt_ordinal, 1);
    assert_eq!(actual.rcmax, 5);
    assert_eq!(actual.s0.len(), 12);
    assert_eq!(actual.s.len(), 60);
    assert_eq!(actual.e.len(), 5);
    assert_eq!(actual.gamma1_minus_gamma2.len(), 15);
    assert_eq!(actual.phi_r0.len(), 25);
    assert_eq!(actual.rnew.len(), 25);
    assert_eq!(actual.rold.len(), 25);
    assert!((1..=5).contains(&actual.num_ctf_terms));
    assert_ne!(
        actual.context.ctf_time_step.to_bits(),
        actual.context.time_step_zone.to_bits()
    );
    assert_eq!((assembly, methods, gammas), before);
    Ok(())
}

#[test]
fn actual_assembly_stops_never_publish_private_coefficients() -> TestResult {
    let (mut assembly, methods, gammas) = actual_owners()?;
    let reason =
        CtfFirstAssemblyUnavailable::InitialUnavailable(CtfInitialUnavailable::UnusedConstruction);
    assembly.result = Err(reason);
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::FirstAssemblyUnavailable(reason))
    );
    for reason in [
        CtfAssemblyUnavailable::UnusedConstruction,
        CtfAssemblyUnavailable::LoadingError,
        CtfAssemblyUnavailable::AllResistive,
        CtfAssemblyUnavailable::ReversedConstructionReuse,
    ] {
        assembly.result = Ok(CtfAssemblyObservation::Unavailable(reason));
        assert_eq!(
            initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas)
                .result,
            Err(CtfFirstFinalCoefficientsUnavailable::AssemblyRouteUnavailable(reason))
        );
    }
    Ok(())
}

#[test]
fn exponential_stop_inverse_noninvocation_and_inverse_error_stay_distinct() -> TestResult {
    let (assembly, mut methods, gammas) = actual_owners()?;
    let original = methods.clone();
    let error = CtfFirstExponentialUnavailable::NotFirstAttempt(2);
    methods.exponential.result = Err(error);
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::ExponentialUnavailable(error))
    );
    methods = original.clone();
    let route = CtfAssemblyUnavailable::AllResistive;
    methods.exponential.result = Ok(CtfExponentialObservation::Unavailable(route));
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::ExponentialRouteUnavailable(route))
    );
    methods = original.clone();
    let prior = PriorExponentialUnavailable::Error(error);
    methods.inverse.result = Err(prior);
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::InverseNotInvoked(
            prior
        ))
    );
    methods = original;
    let error = CtfFirstInverseUnavailable::InverseScope(CtfInverseScopeError::MatrixShape);
    methods
        .inverse
        .result
        .as_mut()
        .map_err(|reason| *reason)?
        .result = Err(error);
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::InverseUnavailable(
            error
        ))
    );
    Ok(())
}

#[test]
fn gamma_error_and_unselected_route_do_not_fall_back_to_other_arrays() -> TestResult {
    let (assembly, methods, mut gammas) = actual_owners()?;
    let error = CtfFirstGammasUnavailable::NotFirstAttempt(2);
    gammas.result = Err(error);
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::GammasUnavailable(
            error
        ))
    );
    let route = CtfAssemblyUnavailable::ReversedConstructionReuse;
    gammas.result = Ok(CtfGammaObservation::Unavailable(route));
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::GammasRouteUnavailable(route))
    );
    Ok(())
}

#[test]
fn actual_wrapper_and_invoked_owner_ids_must_match() -> TestResult {
    let owners = actual_owners()?;
    for selected in 0..5 {
        let (mut assembly, mut methods, mut gammas) = owners.clone();
        match selected {
            0 => assembly.construction_id = ConstructionId(9),
            1 => methods.exponential.construction_id = ConstructionId(9),
            2 => methods.inverse.construction_id = ConstructionId(9),
            3 => gammas.construction_id = ConstructionId(9),
            _ => {
                methods
                    .inverse
                    .result
                    .as_mut()
                    .map_err(|reason| *reason)?
                    .construction_id = ConstructionId(9)
            }
        }
        assert_eq!(
            initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas)
                .result,
            Err(CtfFirstFinalCoefficientsUnavailable::ConstructionIdentityMismatch)
        );
    }
    Ok(())
}

#[test]
fn later_attempt_and_unsupported_assembly_context_have_no_coefficient_owner() -> TestResult {
    let owners = actual_owners()?;
    for ordinal in [0, 2] {
        let (mut assembly, methods, gammas) = owners.clone();
        assembled(&mut assembly)?.context.attempt_ordinal = ordinal;
        assert_eq!(
            initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas)
                .result,
            Err(CtfFirstFinalCoefficientsUnavailable::NotFirstAttempt(
                ordinal
            ))
        );
    }
    for selected in 0..5 {
        let (mut assembly, methods, gammas) = owners.clone();
        let context = &mut assembled(&mut assembly)?.context;
        match selected {
            0 => context.source_sink_present = true,
            1 => context.solution_dimensions = 2,
            2 => context.node_source = 1,
            3 => context.node_user_temp = 1,
            _ => {
                context.route = CtfAssemblyRoute::Unavailable(CtfAssemblyUnavailable::AllResistive)
            }
        }
        assert_eq!(
            initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas)
                .result,
            Err(CtfFirstFinalCoefficientsUnavailable::AssemblyContext)
        );
    }
    Ok(())
}

#[test]
fn precursor_context_bits_and_absent_ordered_context_are_not_repaired() -> TestResult {
    let owners = actual_owners()?;
    for selected in 0..5 {
        let (assembly, mut methods, mut gammas) = owners.clone();
        match selected {
            0 => exponential(&mut methods)?.context.num_histories += 1,
            1 => {
                methods
                    .inverse
                    .result
                    .as_mut()
                    .map_err(|reason| *reason)?
                    .result
                    .as_mut()
                    .map_err(|reason| *reason)?
                    .context
                    .ctf_time_step = 0.25
            }
            2 => gamma(&mut gammas)?.context.construction_id = ConstructionId(9),
            3 => {
                methods
                    .inverse
                    .assembly_context
                    .as_mut()
                    .ok_or_else(|| std::io::Error::other("test inverse context absent"))?
                    .attempt_ordinal = 2
            }
            _ => methods.inverse.assembly_context = None,
        }
        assert_eq!(
            initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas)
                .result,
            Err(CtfFirstFinalCoefficientsUnavailable::AttemptContextMismatch)
        );
    }
    let (mut assembly, mut methods, mut gammas) = owners;
    assembled(&mut assembly)?.context.time_step_zone = 0.0;
    exponential(&mut methods)?.context.time_step_zone = 0.0;
    methods
        .inverse
        .result
        .as_mut()
        .map_err(|reason| *reason)?
        .result
        .as_mut()
        .map_err(|reason| *reason)?
        .context
        .time_step_zone = 0.0;
    methods
        .inverse
        .assembly_context
        .as_mut()
        .ok_or_else(|| std::io::Error::other("test inverse context absent"))?
        .time_step_zone = 0.0;
    gamma(&mut gammas)?.context.time_step_zone = -0.0;
    assert_eq!(
        initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas).result,
        Err(CtfFirstFinalCoefficientsUnavailable::AttemptContextMismatch)
    );
    Ok(())
}

#[test]
fn actual_precursor_dimensions_must_match_first_assembly() -> TestResult {
    let owners = actual_owners()?;
    for selected in 0..3 {
        let (assembly, mut methods, mut gammas) = owners.clone();
        match selected {
            0 => exponential(&mut methods)?.rcmax = 4,
            1 => {
                methods
                    .inverse
                    .result
                    .as_mut()
                    .map_err(|reason| *reason)?
                    .result
                    .as_mut()
                    .map_err(|reason| *reason)?
                    .inverse
                    .rcmax = 4
            }
            _ => gamma(&mut gammas)?.rcmax = 4,
        }
        assert_eq!(
            initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas)
                .result,
            Err(CtfFirstFinalCoefficientsUnavailable::DimensionMismatch)
        );
    }
    Ok(())
}

#[test]
fn missing_borrowed_input_shapes_are_rejected_without_reconstruction() -> TestResult {
    let owners = actual_owners()?;
    for selected in 0..4 {
        let (mut assembly, mut methods, mut gammas) = owners.clone();
        let expected = match selected {
            0 => {
                exponential(&mut methods)?.a_exp.pop();
                CtfFinalCoefficientScopeError::MatrixShape
            }
            1 => {
                assembled(&mut assembly)?.iden_matrix.pop();
                CtfFinalCoefficientScopeError::MatrixShape
            }
            2 => {
                gamma(&mut gammas)?.gamma1.pop();
                CtfFinalCoefficientScopeError::GammaShape
            }
            _ => {
                gamma(&mut gammas)?.gamma2.pop();
                CtfFinalCoefficientScopeError::GammaShape
            }
        };
        assert_eq!(
            initialize_construction_ctf_first_final_coefficients(&assembly, &methods, &gammas)
                .result,
            Err(CtfFirstFinalCoefficientsUnavailable::FinalCoefficientScope(
                expected
            ))
        );
    }
    Ok(())
}
