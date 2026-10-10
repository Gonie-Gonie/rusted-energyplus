use super::*;
use crate::heat_balance::ctf_first_inverse_owner::CtfFirstInverseUnavailable;
use crate::heat_balance::ctf_first_matrix_owner::initialize_construction_ctf_first_matrix_functions;
use crate::heat_balance::ctf_initial_owner::CtfInitialUnavailable;
use crate::heat_balance::surface_manager::ctf_inverse_matrix::CtfInverseScopeError;
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::CtfNormalizedLayer;
use crate::heat_balance::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyInput, CtfStateSpaceAssembly, assemble_1d_ctf_state_space,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn actual_owners() -> TestResult<(
    ConstructionCtfFirstAssembly,
    ConstructionCtfFirstMatrixFunctions,
)> {
    let layers = [CtfNormalizedLayer {
        dl: 0.5,
        rk: 0.25,
        rho: 1.0,
        cp: 1.0,
        lr: 2.0,
        res_layer: false,
    }];
    let context = CtfAssemblyContext {
        construction_id: ConstructionId(7),
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
    Ok((assembly, methods))
}

fn assembled(owner: &mut ConstructionCtfFirstAssembly) -> TestResult<&mut CtfStateSpaceAssembly> {
    match owner.result.as_mut().map_err(|reason| *reason)? {
        CtfAssemblyObservation::Assembled(actual) => Ok(actual),
        CtfAssemblyObservation::Unavailable(_) => {
            Err(std::io::Error::other("test assembly unavailable").into())
        }
    }
}

#[test]
fn actual_ordered_returns_feed_gamma_without_mutating_prerequisites() -> TestResult {
    let (assembly, methods) = actual_owners()?;
    let before = (assembly.clone(), methods.clone());
    let output = initialize_construction_ctf_first_gammas(&assembly, &methods);
    let CtfGammaObservation::Gamma(gamma) = output.result? else {
        return Err(std::io::Error::other("test Gamma unavailable").into());
    };
    assert_eq!(output.construction_id, ConstructionId(7));
    assert_eq!(gamma.context.attempt_ordinal, 1);
    assert_eq!(gamma.rcmax, 5);
    assert_eq!(gamma.a_temp.len(), 25);
    assert_eq!(gamma.gamma1.len(), 15);
    assert_eq!(gamma.gamma2.len(), 15);
    assert_ne!(
        gamma.context.ctf_time_step.to_bits(),
        gamma.context.time_step_zone.to_bits()
    );
    assert_eq!((assembly, methods), before);
    Ok(())
}

#[test]
fn actual_prerequisite_stop_has_no_gamma_owner() -> TestResult {
    let (mut assembly, methods) = actual_owners()?;
    let reason =
        CtfFirstAssemblyUnavailable::InitialUnavailable(CtfInitialUnavailable::UnusedConstruction);
    assembly.result = Err(reason);
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::FirstAssemblyUnavailable(reason))
    );
    for reason in [
        CtfAssemblyUnavailable::UnusedConstruction,
        CtfAssemblyUnavailable::LoadingError,
        CtfAssemblyUnavailable::AllResistive,
        CtfAssemblyUnavailable::ReversedConstructionReuse,
    ] {
        assembly.result = Ok(CtfAssemblyObservation::Unavailable(reason));
        assert_eq!(
            initialize_construction_ctf_first_gammas(&assembly, &methods).result,
            Err(CtfFirstGammasUnavailable::AssemblyRouteUnavailable(reason))
        );
    }
    Ok(())
}

#[test]
fn predecessor_errors_and_inverse_noninvocation_stay_distinct() -> TestResult {
    let (assembly, mut methods) = actual_owners()?;
    let original = methods.clone();
    let prior = CtfFirstExponentialUnavailable::NotFirstAttempt(2);
    methods.exponential.result = Err(prior);
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::ExponentialUnavailable(prior))
    );
    methods = original.clone();
    let prior = PriorExponentialUnavailable::Error(prior);
    methods.inverse.result = Err(prior);
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::InverseNotInvoked(prior))
    );
    methods = original;
    let invoked = methods.inverse.result.as_mut().map_err(|reason| *reason)?;
    let reason = CtfFirstInverseUnavailable::InverseScope(CtfInverseScopeError::MatrixShape);
    invoked.result = Err(reason);
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::InverseUnavailable(reason))
    );
    Ok(())
}

#[test]
fn stale_construction_later_attempt_and_context_bits_are_rejected() -> TestResult {
    let (mut assembly, mut methods) = actual_owners()?;
    methods.exponential.construction_id = ConstructionId(8);
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::ConstructionIdentityMismatch)
    );
    methods.exponential.construction_id = ConstructionId(7);
    assembled(&mut assembly)?.context.attempt_ordinal = 2;
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::NotFirstAttempt(2))
    );
    assembled(&mut assembly)?.context.attempt_ordinal = 1;
    let CtfExponentialObservation::Exponential(exponential) = methods
        .exponential
        .result
        .as_mut()
        .map_err(|reason| *reason)?
    else {
        return Err(std::io::Error::other("test exponential unavailable").into());
    };
    exponential.context.time_step_zone = 0.0;
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
        .ok_or_else(|| std::io::Error::other("test inverse context unavailable"))?
        .time_step_zone = -0.0;
    assembled(&mut assembly)?.context.time_step_zone = 0.0;
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::AttemptContextMismatch)
    );
    Ok(())
}

#[test]
fn unsupported_source_or_perpendicular_context_has_no_gamma_arrays() -> TestResult {
    let (mut assembly, methods) = actual_owners()?;
    assembled(&mut assembly)?.context.source_sink_present = true;
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::AssemblyContext)
    );
    assembled(&mut assembly)?.context.source_sink_present = false;
    assembled(&mut assembly)?.context.solution_dimensions = 2;
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::AssemblyContext)
    );
    Ok(())
}

#[test]
fn actual_dimensions_and_borrowed_shapes_are_not_repaired() -> TestResult {
    let (assembly, mut methods) = actual_owners()?;
    let original = methods.clone();
    methods
        .inverse
        .result
        .as_mut()
        .map_err(|reason| *reason)?
        .result
        .as_mut()
        .map_err(|reason| *reason)?
        .inverse
        .rcmax = 4;
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::DimensionMismatch)
    );
    methods = original;
    methods
        .inverse
        .result
        .as_mut()
        .map_err(|reason| *reason)?
        .result
        .as_mut()
        .map_err(|reason| *reason)?
        .inverse
        .a_inv
        .pop();
    assert_eq!(
        initialize_construction_ctf_first_gammas(&assembly, &methods).result,
        Err(CtfFirstGammasUnavailable::GammaScope(
            CtfGammaScopeError::MatrixShape
        ))
    );
    Ok(())
}
