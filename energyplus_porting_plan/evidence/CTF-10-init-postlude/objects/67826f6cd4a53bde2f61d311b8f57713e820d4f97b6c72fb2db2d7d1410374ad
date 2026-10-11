//! Selected Construction.cc657-981 retry orchestration, excluding SI stores.

use super::ctf_first_assembly_owner::{ConstructionCtfFirstAssembly, CtfFirstAssemblyUnavailable};
use super::surface_manager::ctf_caller_stability::{
    CtfCallerDiagnosticNames, CtfCallerSharedFlags, CtfCallerStabilityDecision,
    CtfCallerStabilityOutcome, CtfCallerStabilityScopeError, decide_selected_ctf_caller_stability,
};
use super::surface_manager::ctf_exponential_matrix::{
    CtfExponentialInput, CtfExponentialObservation, CtfExponentialScopeError,
    calculate_selected_ctf_matrix_exponential,
};
use super::surface_manager::ctf_final_coefficients::{
    CtfFinalCoefficientInput, CtfFinalCoefficientObservation, CtfFinalCoefficientScopeError,
    calculate_selected_ctf_final_coefficients,
};
use super::surface_manager::ctf_gamma_matrix::{
    CtfGammaInput, CtfGammaObservation, CtfGammaScopeError, calculate_selected_ctf_gammas,
};
use super::surface_manager::ctf_inverse_matrix::{
    CtfInverseInput, CtfInverseMatrix, CtfInverseScopeError, invert_selected_ctf_matrix,
};
use super::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyObservation, CtfAssemblyPreAssignment,
    CtfAssemblyReassignmentError, CtfAssemblyRoute, CtfAssemblyUnavailable, CtfStateSpaceAssembly,
    reassign_1d_ctf_state_space,
};
use ep_model::ConstructionId;

/// Actual low-level method whose returned unavailable route stopped invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfRetryMethod {
    /// Construction.cc884.
    Exponential,
    /// Construction.cc890.
    Gammas,
    /// Construction.cc894.
    FinalCoefficients,
}

/// Rust prerequisite/domain stops, never invented Native method outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfRetryDriverStop {
    /// The consumed first owner retains its actual prerequisite error.
    FirstAssembly(CtfFirstAssemblyUnavailable),
    /// No first assembly exists on this actual route.
    FirstAssemblyRoute(CtfAssemblyUnavailable),
    /// Wrapper and original assembly identity differ.
    ConstructionIdentityMismatch,
    /// The consumed owner must be the actual first invocation, not a replay.
    NotInitialAttempt(usize),
    /// Selected original one-dimensional/no-source route is required.
    UnsupportedContext,
    /// A method actually returned its typed unavailable route; later calls stop.
    MethodRoute {
        /// Actual pure method that returned its unavailable route.
        method: CtfRetryMethod,
        /// Typed unavailable route copied from that method result.
        reason: CtfAssemblyUnavailable,
    },
    /// The invoked exponential returned this actual shape/domain error.
    Exponential(CtfExponentialScopeError),
    /// The invoked inverse returned this actual shape error.
    Inverse(CtfInverseScopeError),
    /// The invoked Gamma method returned this actual shape/domain error.
    Gammas(CtfGammaScopeError),
    /// The invoked private final method returned this actual shape/domain error.
    FinalCoefficients(CtfFinalCoefficientScopeError),
    /// The invoked caller decision rejected its actual association/domain.
    Caller(CtfCallerStabilityScopeError),
    /// A next observation ordinal is not representable; no guessed ordinal.
    AttemptOrdinalOverflow,
    /// The next same-storage assignment rejected its exact supplied context.
    Reassignment(CtfAssemblyReassignmentError),
}

/// Terminal selected loop decision, distinct from complete whole-call success.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfRetryDriverOutcome {
    /// No later method or coefficient result is fabricated after this stop.
    ScopeStopped(CtfRetryDriverStop),
    /// Source convergence terminates below the severe threshold.
    ConvergedLoopExit,
    /// Source severe diagnostics/flags precede its literal break.
    SevenHourSevereBreak,
    /// Source would throw before public stores; global backend is uninvoked here.
    FatalNoCtfs,
}

/// Read-only copied actual method inputs, never fed back into the calculation.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfRetryAssemblySnapshot {
    /// Actual pre-method attempt stamp before caller903-977 changes.
    pub context: CtfAssemblyContext,
    /// Unchanged actual source dimension.
    pub rcmax: i32,
    /// Copied original AMat, with i2 contiguous.
    pub a_mat: Vec<f64>,
    /// Copied once-produced IdenMatrix, not an identity reconstruction.
    pub iden_matrix: Vec<f64>,
    /// Copied compressed forcing values after actual B3 reset/assignments.
    pub b_mat: [f64; 3],
    /// Copied original compressed output owner.
    pub c_mat: [f64; 2],
    /// Copied original compressed direct owner.
    pub d_mat: [f64; 2],
}

/// One real Rust invocation sequence with explicit later non-invocation.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfRetryAttempt {
    /// Copy of the retained assembly inputs at this visit.
    pub assembly: CtfRetryAssemblySnapshot,
    /// Every retained attempt really invokes exponential first.
    pub exponential: Result<CtfExponentialObservation, CtfExponentialScopeError>,
    /// None means not invoked after the preceding actual stop.
    pub inverse: Option<Result<CtfInverseMatrix, CtfInverseScopeError>>,
    /// Fresh real Gamma result; never the prior attempt's mutated Gamma1.
    pub gammas: Option<Result<CtfGammaObservation, CtfGammaScopeError>>,
    /// Actual private s0/full s/e return or scope error, not public19 stores.
    pub final_coefficients:
        Option<Result<CtfFinalCoefficientObservation, CtfFinalCoefficientScopeError>>,
    /// Actual caller decision; unavailable when a preceding method did not return.
    pub caller: Option<Result<CtfCallerStabilityDecision, CtfCallerStabilityScopeError>>,
}

/// One actual same-storage reassignment invocation after a real Retry decision.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfRetryReassignment {
    /// Copied persistent storage before the next band's assignments.
    pub before: CtfRetryAssemblySnapshot,
    /// Actual preceding private result, not reset or inferred from history.
    pub preceding_num_ctf_terms: i32,
    /// Exact postcheck stamp with only the checked observation ordinal advanced.
    pub supplied_context: CtfAssemblyContext,
    /// Actual reassignment return; no method attempt follows a returned error.
    pub result: Result<(), CtfAssemblyReassignmentError>,
    /// Actual retained owner stamp after that invocation, even on scope error.
    pub context_after: CtfAssemblyContext,
}

/// Selected driver owner, not cache activation or a complete CTF09 certificate.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfRetryDriver {
    /// Original typed construction identity.
    pub construction_id: ConstructionId,
    /// Actual shared flags on entry.
    pub flags_on_entry: CtfCallerSharedFlags,
    /// Flags changed only by actually reached caller severe assignments.
    pub flags_on_return: CtfCallerSharedFlags,
    /// The actual first assembly stamp, absent if no assembly owner existed.
    pub initial_context: Option<CtfAssemblyContext>,
    /// Last caller postcheck stamp, if a caller decision actually returned.
    pub postcheck_context: Option<CtfAssemblyContext>,
    /// The exact moved original allocation, retaining the last method stamp.
    pub retained_assembly: Option<Box<CtfStateSpaceAssembly>>,
    /// All actual Rust attempts in executed order, without a declared cap.
    pub attempts: Vec<CtfRetryAttempt>,
    /// Actual reassignment calls; the first assembly is consumed, not replayed.
    pub reassignments: Vec<CtfRetryReassignment>,
    /// Exact selected terminal decision or typed scope stop.
    pub outcome: CtfRetryDriverOutcome,
}

impl ConstructionCtfRetryDriver {
    /// Borrow the genuine once-captured initial producer checkpoint, if present.
    ///
    /// This is the retained first owner's optional copy, not a reconstruction
    /// from the later band, dimension, or advanced caller/attempt timestamp.
    #[must_use]
    pub fn initial_pre_assignment(&self) -> Option<&CtfAssemblyPreAssignment> {
        self.retained_assembly
            .as_ref()?
            .initial_pre_assignment
            .as_ref()
    }
}

#[derive(Clone, Copy)]
enum AttemptControl {
    Stop(CtfRetryDriverStop),
    Caller {
        outcome: CtfCallerStabilityOutcome,
        postcheck_context: CtfAssemblyContext,
        flags_after: CtfCallerSharedFlags,
        num_ctf_terms: i32,
    },
}

fn next_retry_context(
    postcheck: CtfAssemblyContext,
) -> Result<CtfAssemblyContext, CtfRetryDriverStop> {
    let ordinal = postcheck
        .attempt_ordinal
        .checked_add(1)
        .ok_or(CtfRetryDriverStop::AttemptOrdinalOverflow)?;
    Ok(CtfAssemblyContext {
        attempt_ordinal: ordinal,
        ..postcheck
    })
}

fn copy_assembly(assembly: &CtfStateSpaceAssembly) -> CtfRetryAssemblySnapshot {
    CtfRetryAssemblySnapshot {
        context: assembly.context,
        rcmax: assembly.rcmax,
        a_mat: assembly.a_mat.clone(),
        iden_matrix: assembly.iden_matrix.clone(),
        b_mat: assembly.b_mat,
        c_mat: assembly.c_mat,
        d_mat: assembly.d_mat,
    }
}

fn invoke_attempt(
    assembly: &CtfStateSpaceAssembly,
    flags: CtfCallerSharedFlags,
    names: CtfCallerDiagnosticNames<'_>,
) -> (CtfRetryAttempt, AttemptControl) {
    use CtfRetryDriverStop as Stop;
    let context = assembly.context;
    let mut record = CtfRetryAttempt {
        assembly: copy_assembly(assembly),
        exponential: calculate_selected_ctf_matrix_exponential(
            CtfExponentialInput {
                rcmax: assembly.rcmax,
                a_mat: &assembly.a_mat,
                iden_matrix: &assembly.iden_matrix,
            },
            context,
        ),
        inverse: None,
        gammas: None,
        final_coefficients: None,
        caller: None,
    };
    let exponential = match &record.exponential {
        Err(error) => {
            let stop = Stop::Exponential(*error);
            return (record, AttemptControl::Stop(stop));
        }
        Ok(CtfExponentialObservation::Unavailable(reason)) => {
            let stop = Stop::MethodRoute {
                method: CtfRetryMethod::Exponential,
                reason: *reason,
            };
            return (record, AttemptControl::Stop(stop));
        }
        Ok(CtfExponentialObservation::Exponential(actual)) => actual,
    };
    // Native887 consumes original AMat/Iden, never AExp or final AMat1.
    let inverse_result = invert_selected_ctf_matrix(CtfInverseInput {
        rcmax: assembly.rcmax,
        a_mat: &assembly.a_mat,
        iden_matrix: &assembly.iden_matrix,
    });
    let inverse = match &inverse_result {
        Err(error) => {
            let stop = Stop::Inverse(*error);
            record.inverse = Some(inverse_result);
            return (record, AttemptControl::Stop(stop));
        }
        Ok(actual) => actual,
    };
    // A fresh Gamma method call resets its own real Gamma1/Gamma2 each visit.
    let gamma_result = calculate_selected_ctf_gammas(
        CtfGammaInput {
            rcmax: assembly.rcmax,
            a_inv: &inverse.a_inv,
            a_exp: &exponential.a_exp,
            iden_matrix: &assembly.iden_matrix,
            b_mat: &assembly.b_mat,
        },
        context,
    );
    record.inverse = Some(inverse_result);
    let gamma = match &gamma_result {
        Err(error) => {
            let stop = Stop::Gammas(*error);
            record.gammas = Some(gamma_result);
            return (record, AttemptControl::Stop(stop));
        }
        Ok(CtfGammaObservation::Unavailable(reason)) => {
            let stop = Stop::MethodRoute {
                method: CtfRetryMethod::Gammas,
                reason: *reason,
            };
            record.gammas = Some(gamma_result);
            return (record, AttemptControl::Stop(stop));
        }
        Ok(CtfGammaObservation::Gamma(actual)) => actual,
    };
    let coefficient_result = calculate_selected_ctf_final_coefficients(
        CtfFinalCoefficientInput {
            rcmax: assembly.rcmax,
            a_exp: &exponential.a_exp,
            iden_matrix: &assembly.iden_matrix,
            gamma1: &gamma.gamma1,
            gamma2: &gamma.gamma2,
            c_mat: &assembly.c_mat,
            d_mat: &assembly.d_mat,
        },
        context,
    );
    record.gammas = Some(gamma_result);
    let coefficients = match &coefficient_result {
        Err(error) => {
            let stop = Stop::FinalCoefficients(*error);
            record.final_coefficients = Some(coefficient_result);
            return (record, AttemptControl::Stop(stop));
        }
        Ok(CtfFinalCoefficientObservation::Unavailable(reason)) => {
            let stop = Stop::MethodRoute {
                method: CtfRetryMethod::FinalCoefficients,
                reason: *reason,
            };
            record.final_coefficients = Some(coefficient_result);
            return (record, AttemptControl::Stop(stop));
        }
        Ok(CtfFinalCoefficientObservation::Coefficients(actual)) => actual,
    };
    let caller_result = decide_selected_ctf_caller_stability(coefficients, context, flags, names);
    record.final_coefficients = Some(coefficient_result);
    let control = match &caller_result {
        Err(error) => AttemptControl::Stop(Stop::Caller(*error)),
        Ok(actual) => AttemptControl::Caller {
            outcome: actual.outcome,
            postcheck_context: actual.postcheck_context,
            flags_after: actual.flags_after,
            num_ctf_terms: actual.num_ctf_terms,
        },
    };
    record.caller = Some(caller_result);
    (record, control)
}

fn stopped_before_methods(
    construction_id: ConstructionId,
    flags: CtfCallerSharedFlags,
    assembly: Option<Box<CtfStateSpaceAssembly>>,
    reason: CtfRetryDriverStop,
) -> ConstructionCtfRetryDriver {
    let initial_context = assembly.as_ref().map(|actual| actual.context);
    ConstructionCtfRetryDriver {
        construction_id,
        flags_on_entry: flags,
        flags_on_return: flags,
        initial_context,
        postcheck_context: None,
        retained_assembly: assembly,
        attempts: Vec::new(),
        reassignments: Vec::new(),
        outcome: CtfRetryDriverOutcome::ScopeStopped(reason),
    }
}

/// Continue from the genuine first assembly using its original storage once.
///
/// The first owner already performed source638-728; it is consumed, not cloned
/// or replayed. Later visits reuse the literal shared assignment body before
/// actual06->05->07->08->caller09 calls. First-only wrappers remain unchanged.
/// No Native results, dynamic timestep answers, identity defaults or fake flags
/// enter this API. There is no iteration cap, SI store or global fatal backend.
pub fn run_selected_construction_ctf_retry_driver(
    first: ConstructionCtfFirstAssembly,
    flags: CtfCallerSharedFlags,
    names: CtfCallerDiagnosticNames<'_>,
) -> ConstructionCtfRetryDriver {
    use CtfRetryDriverOutcome as Outcome;
    use CtfRetryDriverStop as Stop;
    let id = first.construction_id;
    let mut assembly = match first.result {
        Err(reason) => {
            return stopped_before_methods(id, flags, None, Stop::FirstAssembly(reason));
        }
        Ok(CtfAssemblyObservation::Unavailable(reason)) => {
            return stopped_before_methods(id, flags, None, Stop::FirstAssemblyRoute(reason));
        }
        Ok(CtfAssemblyObservation::Assembled(actual)) => actual,
    };
    let initial = assembly.context;
    let stop = if initial.construction_id != id {
        Some(Stop::ConstructionIdentityMismatch)
    } else if initial.attempt_ordinal != 1 {
        Some(Stop::NotInitialAttempt(initial.attempt_ordinal))
    } else if initial.route != CtfAssemblyRoute::Assemble
        || initial.solution_dimensions != 1
        || initial.source_sink_present
        || initial.node_source != 0
        || initial.node_user_temp != 0
    {
        Some(Stop::UnsupportedContext)
    } else {
        None
    };
    if let Some(stop) = stop {
        return stopped_before_methods(id, flags, Some(assembly), stop);
    }
    let mut attempts = Vec::new();
    let mut reassignments = Vec::new();
    let mut flags_on_return = flags;
    let mut last_postcheck_context = None;
    let outcome = loop {
        let (attempt, control) = invoke_attempt(&assembly, flags_on_return, names);
        attempts.push(attempt);
        match control {
            AttemptControl::Stop(reason) => {
                break Outcome::ScopeStopped(reason);
            }
            AttemptControl::Caller {
                outcome,
                postcheck_context,
                flags_after,
                num_ctf_terms,
            } => {
                last_postcheck_context = Some(postcheck_context);
                flags_on_return = flags_after;
                match outcome {
                    CtfCallerStabilityOutcome::Retry => {
                        let next = match next_retry_context(postcheck_context) {
                            Ok(actual) => actual,
                            Err(reason) => {
                                break Outcome::ScopeStopped(reason);
                            }
                        };
                        let before = copy_assembly(&assembly);
                        let result = reassign_1d_ctf_state_space(&mut assembly, next);
                        reassignments.push(CtfRetryReassignment {
                            before,
                            preceding_num_ctf_terms: num_ctf_terms,
                            supplied_context: next,
                            result,
                            context_after: assembly.context,
                        });
                        if let Err(error) = result {
                            break Outcome::ScopeStopped(Stop::Reassignment(error));
                        }
                    }
                    CtfCallerStabilityOutcome::ConvergedLoopExit => {
                        break Outcome::ConvergedLoopExit;
                    }
                    CtfCallerStabilityOutcome::SevenHourSevereBreak => {
                        break Outcome::SevenHourSevereBreak;
                    }
                    CtfCallerStabilityOutcome::FatalNoCtfs => {
                        break Outcome::FatalNoCtfs;
                    }
                }
            }
        }
    };
    ConstructionCtfRetryDriver {
        construction_id: id,
        flags_on_entry: flags,
        flags_on_return,
        initial_context: Some(initial),
        postcheck_context: last_postcheck_context,
        retained_assembly: Some(assembly),
        attempts,
        reassignments,
        outcome,
    }
}

#[cfg(test)]
#[path = "ctf_retry_driver_tests.rs"]
mod tests;
