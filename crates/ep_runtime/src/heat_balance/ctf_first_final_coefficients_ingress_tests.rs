use super::*;
use crate::heat_balance::ctf_first_assembly_owner::CtfFirstAssemblyUnavailable;
use crate::heat_balance::ctf_first_exponential_owner::CtfFirstExponentialUnavailable;
use crate::heat_balance::ctf_first_final_coefficients_owner::CtfFirstFinalCoefficientsUnavailable;
use crate::heat_balance::ctf_first_gamma_owner::CtfFirstGammasUnavailable;
use crate::heat_balance::ctf_first_matrix_owner::PriorExponentialUnavailable;
use crate::heat_balance::ctf_initial_owner::CtfInitialUnavailable;
use crate::heat_balance::surface_manager::ctf_exponential_matrix::{
    CtfExponentialObservation, CtfExponentialScopeError,
};
use crate::heat_balance::surface_manager::ctf_final_coefficients::CtfFinalCoefficientObservation;
use crate::heat_balance::surface_manager::ctf_gamma_matrix::CtfGammaObservation;
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::CtfNormalizedLayer;
use crate::heat_balance::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyInput, CtfAssemblyObservation, CtfAssemblyRoute,
    CtfStateSpaceAssembly, assemble_1d_ctf_state_space,
};
use ep_model::ConstructionId;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn first_assembly() -> TestResult<ConstructionCtfFirstAssembly> {
    // Literal source-derived unit operands; no observed Native answers.
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
    Ok(ConstructionCtfFirstAssembly {
        construction_id: context.construction_id,
        result: Ok(observed),
    })
}

fn assembled(owner: &mut ConstructionCtfFirstAssembly) -> TestResult<&mut CtfStateSpaceAssembly> {
    match owner.result.as_mut().map_err(|reason| *reason)? {
        CtfAssemblyObservation::Assembled(actual) => Ok(actual),
        CtfAssemblyObservation::Unavailable(_) => {
            Err(std::io::Error::other("fixture assembly unavailable").into())
        }
    }
}

#[test]
fn reached_chain_retains_all_first_owners_and_private_result_separately() -> TestResult {
    let assembly = first_assembly()?;
    let before = assembly.clone();
    let result = initialize_construction_ctf_first_matrix_with_final_coefficients(&assembly);
    let CtfExponentialObservation::Exponential(exponential) = result.methods.exponential.result?
    else {
        return Err(std::io::Error::other("fixture exponential unavailable").into());
    };
    let inverse_call = result.methods.inverse.result?;
    let inverse = inverse_call.result?;
    let CtfGammaObservation::Gamma(gammas) = result.gammas.result? else {
        return Err(std::io::Error::other("fixture Gamma unavailable").into());
    };
    let CtfFinalCoefficientObservation::Coefficients(private) = result.final_coefficients.result?
    else {
        return Err(std::io::Error::other("fixture private coefficients unavailable").into());
    };
    for id in [
        result.methods.exponential.construction_id,
        inverse_call.construction_id,
        result.gammas.construction_id,
        result.final_coefficients.construction_id,
    ] {
        assert_eq!(id, assembly.construction_id);
    }
    assert_eq!(exponential.context, inverse.context);
    assert_eq!(gammas.context, inverse.context);
    assert_eq!(private.context, gammas.context);
    assert_eq!(
        result.methods.inverse.assembly_context,
        Some(private.context)
    );
    assert_eq!(private.context.attempt_ordinal, 1);
    assert_ne!(
        private.context.time_step_zone.to_bits(),
        private.context.ctf_time_step.to_bits()
    );
    assert_eq!(gammas.gamma1.len(), 15);
    assert_eq!(gammas.gamma2.len(), 15);
    assert_eq!(private.s0.len(), 12);
    assert_eq!(private.s.len(), 60);
    assert_eq!(private.e.len(), 5);
    assert_eq!(private.gamma1_minus_gamma2.len(), 15);
    assert_eq!(assembly, before);
    Ok(())
}

#[test]
fn exponential_error_keeps_inverse_noninvocation_and_both_later_stops() -> TestResult {
    let mut assembly = first_assembly()?;
    assembled(&mut assembly)?.a_mat.fill(f64::INFINITY);
    let before = assembly.clone();
    let result = initialize_construction_ctf_first_matrix_with_final_coefficients(&assembly);
    let reason = CtfFirstExponentialUnavailable::ExponentialScope(
        CtfExponentialScopeError::UndefinedScalingExponent,
    );
    assert_eq!(result.methods.exponential.result, Err(reason));
    assert_eq!(
        result.methods.inverse.result,
        Err(PriorExponentialUnavailable::Error(reason))
    );
    assert_eq!(
        result.gammas.result,
        Err(CtfFirstGammasUnavailable::ExponentialUnavailable(reason))
    );
    assert_eq!(
        result.final_coefficients.result,
        Err(CtfFirstFinalCoefficientsUnavailable::ExponentialUnavailable(reason))
    );
    assert_eq!(assembly, before);
    Ok(())
}

#[test]
fn unavailable_original_constructions_keep_order_ids_and_no_private_arrays() {
    let reasons = [
        CtfInitialUnavailable::UnusedConstruction,
        CtfInitialUnavailable::PreprocessingErrorReturn,
        CtfInitialUnavailable::AllResistiveBranch,
        CtfInitialUnavailable::ReverseConstruction {
            construction_id: ConstructionId(3),
        },
    ];
    let owners = [10_u32, 11, 12, 13]
        .into_iter()
        .zip(reasons)
        .map(|(id, reason)| ConstructionCtfFirstAssembly {
            construction_id: ConstructionId(id),
            result: Err(CtfFirstAssemblyUnavailable::InitialUnavailable(reason)),
        })
        .collect::<Vec<_>>();
    let results = owners
        .iter()
        .map(initialize_construction_ctf_first_matrix_with_final_coefficients)
        .collect::<Vec<_>>();
    for ((id, reason), result) in [10_u32, 11, 12, 13].into_iter().zip(reasons).zip(results) {
        let first_error = CtfFirstAssemblyUnavailable::InitialUnavailable(reason);
        let prior = CtfFirstExponentialUnavailable::FirstAssemblyUnavailable(first_error);
        assert_eq!(
            result.methods.exponential.construction_id,
            ConstructionId(id)
        );
        assert_eq!(result.methods.exponential.result, Err(prior));
        assert_eq!(result.methods.inverse.construction_id, ConstructionId(id));
        assert_eq!(result.methods.inverse.assembly_context, None);
        assert_eq!(
            result.methods.inverse.result,
            Err(PriorExponentialUnavailable::Error(prior))
        );
        assert_eq!(result.gammas.construction_id, ConstructionId(id));
        assert_eq!(
            result.gammas.result,
            Err(CtfFirstGammasUnavailable::FirstAssemblyUnavailable(
                first_error
            ))
        );
        assert_eq!(
            result.final_coefficients.construction_id,
            ConstructionId(id)
        );
        assert_eq!(
            result.final_coefficients.result,
            Err(CtfFirstFinalCoefficientsUnavailable::FirstAssemblyUnavailable(first_error))
        );
    }
}

#[test]
fn later_assembly_never_becomes_a_private_first_coefficient_result() -> TestResult {
    let mut assembly = first_assembly()?;
    assembled(&mut assembly)?.context.attempt_ordinal = 2;
    let before = assembly.clone();
    let result = initialize_construction_ctf_first_matrix_with_final_coefficients(&assembly);
    assert_eq!(
        result.final_coefficients.result,
        Err(CtfFirstFinalCoefficientsUnavailable::NotFirstAttempt(2))
    );
    assert_eq!(assembly, before);
    Ok(())
}
