use super::*;
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::CtfNormalizedLayer;
use crate::heat_balance::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyInput, CtfAssemblyObservation, CtfAssemblyRoute,
    CtfAssemblyUnavailable, assemble_1d_ctf_state_space,
};

type TestResult = Result<(), String>;

fn flags() -> CtfCallerSharedFlags {
    CtfCallerSharedFlags {
        errors_found: false,
        do_ctf_error_report: false,
    }
}

fn names() -> CtfCallerDiagnosticNames<'static> {
    CtfCallerDiagnosticNames {
        construction_name: "ORIGINAL",
        original_tot_layers: 1,
        original_layer_material_names: &["ORIGINAL MATERIAL"],
    }
}

fn first() -> Result<ConstructionCtfFirstAssembly, String> {
    // Literal source-input fixture from the accepted driver, no output operands.
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
    let result = assemble_1d_ctf_state_space(
        CtfAssemblyInput {
            layers: &layers,
            nodes: &[6],
            dx: &[0.1],
            rcmax: 5,
        },
        context,
    )
    .map_err(|error| format!("fixture first assembly: {error:?}"))?;
    Ok(ConstructionCtfFirstAssembly {
        construction_id: context.construction_id,
        result: Ok(result),
    })
}

#[test]
fn ordinary_source_routes_retain_actual_driver_return_without_stopping_the_next_construction() {
    for initial in [
        CtfInitialUnavailable::UnusedConstruction,
        CtfInitialUnavailable::PreprocessingErrorReturn,
        CtfInitialUnavailable::AllResistiveBranch,
        CtfInitialUnavailable::ReverseConstruction {
            construction_id: ConstructionId(3),
        },
    ] {
        let reason = CtfFirstAssemblyUnavailable::InitialUnavailable(initial);
        let owner = initialize_construction_ctf_retry(
            ConstructionCtfFirstAssembly {
                construction_id: ConstructionId(7),
                result: Err(reason),
            },
            flags(),
            names(),
        );
        assert_eq!(owner.selected_chain_stop, None);
        assert_eq!(owner.flags_on_entry, owner.flags_on_return);
        let actual = owner.result.as_ref().ok();
        assert!(matches!(actual.map(|driver| driver.outcome),
            Some(CtfRetryDriverOutcome::ScopeStopped(CtfRetryDriverStop::FirstAssembly(value))) if value == reason));
        assert!(actual.is_some_and(|driver| driver.attempts.is_empty()));
    }
    for route in [
        CtfAssemblyUnavailable::UnusedConstruction,
        CtfAssemblyUnavailable::LoadingError,
        CtfAssemblyUnavailable::AllResistive,
        CtfAssemblyUnavailable::ReversedConstructionReuse,
    ] {
        let owner = initialize_construction_ctf_retry(
            ConstructionCtfFirstAssembly {
                construction_id: ConstructionId(7),
                result: Ok(CtfAssemblyObservation::Unavailable(route)),
            },
            flags(),
            names(),
        );
        assert_eq!(owner.selected_chain_stop, None);
        assert!(
            matches!(owner.result.as_ref().ok().map(|driver| driver.outcome),
            Some(CtfRetryDriverOutcome::ScopeStopped(CtfRetryDriverStop::FirstAssemblyRoute(value))) if value == route)
        );
    }
}

#[test]
fn actual_domain_stop_is_retained_and_later_initialization_is_explicitly_uninvoked() {
    let reason = CtfFirstAssemblyUnavailable::PostConversionUnavailable;
    let owner = initialize_construction_ctf_retry(
        ConstructionCtfFirstAssembly {
            construction_id: ConstructionId(7),
            result: Err(reason),
        },
        flags(),
        names(),
    );
    let stop = CtfRetryChainStop::ScopeStopped {
        construction_id: ConstructionId(7),
        reason: CtfRetryDriverStop::FirstAssembly(reason),
    };
    assert_eq!(owner.selected_chain_stop, Some(stop));
    let next =
        skip_construction_ctf_retry(ConstructionId(11), owner.flags_on_return, names(), stop);
    assert_eq!(
        next.result,
        Err(CtfRetryInitializationUnavailable::PriorScopeStopped {
            construction_id: ConstructionId(7),
            reason: CtfRetryDriverStop::FirstAssembly(reason),
        })
    );
    assert_eq!(next.flags_on_return, owner.flags_on_return);
    assert_eq!(next.selected_chain_stop, Some(stop));
}

#[test]
fn prior_fatal_is_not_a_fake_driver_return_or_global_diagnostic_backend() {
    let flags = CtfCallerSharedFlags {
        errors_found: true,
        do_ctf_error_report: true,
    };
    let stop = CtfRetryChainStop::Fatal {
        construction_id: ConstructionId(7),
    };
    let next = skip_construction_ctf_retry(ConstructionId(19), flags, names(), stop);
    assert_eq!(next.construction_id, ConstructionId(19));
    assert_eq!(
        next.result,
        Err(CtfRetryInitializationUnavailable::PriorFatal {
            construction_id: ConstructionId(7),
        })
    );
    assert_eq!(next.flags_on_entry, flags);
    assert_eq!(next.flags_on_return, flags);
    assert_eq!(next.construction_name, "ORIGINAL");
    assert_eq!(next.original_layer_material_names, ["ORIGINAL MATERIAL"]);
}

#[test]
fn genuine_first_storage_moves_once_and_first_views_are_actual_driver_visits() -> TestResult {
    let first = first()?;
    let pointers = match &first.result {
        Ok(CtfAssemblyObservation::Assembled(value)) => {
            (value.a_mat.as_ptr(), value.iden_matrix.as_ptr())
        }
        other => return Err(format!("genuine first fixture unavailable: {other:?}")),
    };
    let owner = initialize_construction_ctf_retry(first, flags(), names());
    let actual = owner
        .result
        .as_ref()
        .map_err(|error| format!("initialization: {error:?}"))?;
    let retained = actual
        .retained_assembly
        .as_ref()
        .ok_or("original storage missing")?;
    assert_eq!(
        pointers,
        (retained.a_mat.as_ptr(), retained.iden_matrix.as_ptr())
    );
    assert!(actual.initial_pre_assignment().is_some());
    let visit = actual
        .attempts
        .first()
        .ok_or("actual first method visit missing")?;
    assert_eq!(visit.assembly.context.attempt_ordinal, 1);
    assert_eq!(actual.initial_context, Some(visit.assembly.context));
    assert_eq!(owner.flags_on_return, actual.flags_on_return);
    assert_eq!(
        owner.selected_chain_stop,
        selected_ctf_retry_chain_stop(actual)
    );
    assert_eq!(
        actual.postcheck_context,
        actual
            .attempts
            .last()
            .and_then(|attempt| attempt.caller.as_ref())
            .and_then(|value| value.as_ref().ok())
            .map(|value| value.postcheck_context)
    );
    Ok(())
}
