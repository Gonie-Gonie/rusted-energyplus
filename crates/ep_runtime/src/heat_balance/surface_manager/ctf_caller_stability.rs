//! Literal Construction.cc903-977 caller decision, not a retry orchestrator.

use super::ctf_final_coefficients::CtfFinalCoefficients;
use super::ctf_state_space_assembly::{CtfAssemblyContext, CtfAssemblyRoute};

/// Shared flags entering/leaving this selected caller block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CtfCallerSharedFlags {
    /// Shared loading/initialization error state.
    pub errors_found: bool,
    /// Shared construction-report request state.
    pub do_ctf_error_report: bool,
}

/// Original construction name and LayerPoint material names, never merged layers.
#[derive(Clone, Copy, Debug)]
pub struct CtfCallerDiagnosticNames<'a> {
    /// Original Construction.Name.
    pub construction_name: &'a str,
    /// Original TotLayers, not the merged active count.
    pub original_tot_layers: i32,
    /// Actual LayerPoint material names in original outside-to-inside order.
    pub original_layer_material_names: &'a [&'a str],
}

/// Call intent for the explicit selected Source diagnostic calls only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfCallerDiagnosticLevel {
    /// Selected ShowFatalError call boundary.
    Fatal,
    /// Selected ShowSevereError call.
    Severe,
    /// Selected ShowContinueError call.
    Continue,
}

/// One ordered selected diagnostic call intent, not global backend effects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CtfCallerDiagnostic {
    /// Actual source call kind.
    pub level: CtfCallerDiagnosticLevel,
    /// Source literal formatted only with original diagnostic names.
    pub message: String,
}

/// Scope errors are not Native fatal/error outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfCallerStabilityScopeError {
    /// This owner cannot invent a decision for a route with no private result.
    UnavailableRoute,
    /// Selected one-dimensional/no-source actual attempt is required.
    UnsupportedContext,
    /// Private owner and caller must carry the identical attempt context.
    AttemptContextMismatch,
    /// The private history arrays must retain their full actual allocation.
    PrivateOwnerShape,
    /// A reached severe report requires all original LayerPoint names.
    OriginalLayerNames,
    /// Selected signed-i32 increment would be outside defined C++ behavior.
    UndefinedHistoryIncrement,
}

/// Only the two source branches which increment the next timestep/history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfCallerRetryReason {
    /// Actual NumCTFTerms exceeds the caller's literal18 limit.
    ExcessiveTerms,
    /// An actually evaluated strict series-error predicate was true.
    SeriesMismatch,
}

/// The selected Source control-flow decision; no stores or next method run here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfCallerStabilityOutcome {
    /// Source would repeat the calculation loop, without doing so here.
    Retry,
    /// Caller convergence remains true below the seven-hour limit.
    ConvergedLoopExit,
    /// Severe diagnostics and shared flags precede the literal break.
    SevenHourSevereBreak,
    /// Selected noreturn fatal boundary, before severe check or SI stores.
    FatalNoCtfs,
}

/// Only assigned after the actual series-summation branch is entered.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfCallerSeriesSums {
    /// Absolute Xi sum after the actual ordered history accumulation.
    pub sum_xi: f64,
    /// Absolute Yi sum after the actual ordered history accumulation.
    pub sum_yi: f64,
    /// Absolute Zi sum after the actual ordered history accumulation.
    pub sum_zi: f64,
    /// Literal three-argument ObjexxFCL ternary selection result.
    pub biggest_sum: f64,
}

/// One source decision with branch-assigned values and separate timestamps.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfCallerStabilityDecision {
    /// Actual private-method attempt stamp before any caller increment.
    pub input_context: CtfAssemblyContext,
    /// Same attempt identity with actual postcheck dt/history, not a new attempt.
    pub postcheck_context: CtfAssemblyContext,
    /// Actual shared flags on entry.
    pub flags_before: CtfCallerSharedFlags,
    /// Same flags with only reached severe assignments applied.
    pub flags_after: CtfCallerSharedFlags,
    /// Actual private method's term count, unchanged by this block.
    pub num_ctf_terms: i32,
    /// Caller903 variable, independent of the private08 convergence variable.
    pub caller_ctf_converged: bool,
    /// Assigned only by an actual increment branch, including a severe break.
    pub retry_reason: Option<CtfCallerRetryReason>,
    /// Absent when the excessive-term branch skips the summations.
    pub sums: Option<CtfCallerSeriesSums>,
    /// Absent on skipped sums/fatal; the second is also absent on true first OR.
    pub first_relative_error: Option<f64>,
    /// Absent when the source short-circuits the second OR operand.
    pub second_relative_error: Option<f64>,
    /// Exact selected branch outcome, not whole initialization success.
    pub outcome: CtfCallerStabilityOutcome,
    /// Ordered explicit call intents; global summaries/counters are unpaired.
    pub diagnostics: Vec<CtfCallerDiagnostic>,
}

fn same_context(a: CtfAssemblyContext, b: CtfAssemblyContext) -> bool {
    a.construction_id == b.construction_id
        && a.route == b.route
        && a.solution_dimensions == b.solution_dimensions
        && a.source_sink_present == b.source_sink_present
        && a.node_source == b.node_source
        && a.node_user_temp == b.node_user_temp
        && a.attempt_ordinal == b.attempt_ordinal
        && a.time_step_zone.to_bits() == b.time_step_zone.to_bits()
        && a.ctf_time_step.to_bits() == b.ctf_time_step.to_bits()
        && a.num_histories == b.num_histories
}

// ObjexxFCL Fmath.hh479-481: preserve ties and NaN selection, not f64::max.
fn source_max3(a: f64, b: f64, c: f64) -> f64 {
    if a < b {
        if b < c { c } else { b }
    } else if a < c {
        c
    } else {
        a
    }
}

fn increment_history(context: &mut CtfAssemblyContext) -> Result<(), CtfCallerStabilityScopeError> {
    // C++ signed overflow has no defined Source result; do not wrap/saturate.
    context.num_histories = context
        .num_histories
        .checked_add(1)
        .ok_or(CtfCallerStabilityScopeError::UndefinedHistoryIncrement)?;
    context.ctf_time_step += context.time_step_zone;
    Ok(())
}

fn emit(
    records: &mut Vec<CtfCallerDiagnostic>,
    level: CtfCallerDiagnosticLevel,
    message: impl Into<String>,
) {
    records.push(CtfCallerDiagnostic {
        level,
        message: message.into(),
    });
}

/// Consume one actually returned private08 owner and the same caller stamp.
///
/// This invokes no matrix method, retry, global diagnostic backend or SI store.
/// A typed FatalNoCtfs represents the selected noreturn call boundary; its global
/// summary/counters/throw are not fabricated here. Full orchestration is separate.
pub fn decide_selected_ctf_caller_stability(
    coefficients: &CtfFinalCoefficients,
    caller_context: CtfAssemblyContext,
    flags: CtfCallerSharedFlags,
    names: CtfCallerDiagnosticNames<'_>,
) -> Result<CtfCallerStabilityDecision, CtfCallerStabilityScopeError> {
    use CtfCallerDiagnosticLevel::{Continue, Fatal, Severe};
    use CtfCallerStabilityScopeError as Error;
    if let CtfAssemblyRoute::Unavailable(_) = caller_context.route {
        return Err(Error::UnavailableRoute);
    }
    if caller_context.solution_dimensions != 1
        || caller_context.source_sink_present
        || caller_context.node_source != 0
        || caller_context.node_user_temp != 0
        || caller_context.attempt_ordinal == 0
    {
        return Err(Error::UnsupportedContext);
    }
    if !same_context(coefficients.context, caller_context) {
        return Err(Error::AttemptContextMismatch);
    }
    let n = usize::try_from(coefficients.rcmax)
        .ok()
        .filter(|&n| n > 0)
        .ok_or(Error::PrivateOwnerShape)?;
    let history_size = n.checked_mul(12).ok_or(Error::PrivateOwnerShape)?;
    let terms =
        usize::try_from(coefficients.num_ctf_terms).map_err(|_| Error::PrivateOwnerShape)?;
    if coefficients.s.len() != history_size || coefficients.e.len() != n || terms > n {
        return Err(Error::PrivateOwnerShape);
    }
    let mut result = CtfCallerStabilityDecision {
        input_context: caller_context,
        postcheck_context: caller_context,
        flags_before: flags,
        flags_after: flags,
        num_ctf_terms: coefficients.num_ctf_terms,
        caller_ctf_converged: true,
        retry_reason: None,
        sums: None,
        first_relative_error: None,
        second_relative_error: None,
        outcome: CtfCallerStabilityOutcome::ConvergedLoopExit,
        diagnostics: Vec::new(),
    };
    if coefficients.num_ctf_terms > (19 - 1) {
        increment_history(&mut result.postcheck_context)?;
        result.caller_ctf_converged = false;
        result.retry_reason = Some(CtfCallerRetryReason::ExcessiveTerms);
        result.outcome = CtfCallerStabilityOutcome::Retry;
    }
    if result.caller_ctf_converged {
        let mut sum_xi = coefficients.s0[5]; // Source s0(2,2).
        let mut sum_yi = coefficients.s0[1]; // Source s0(1,2).
        let mut sum_zi = coefficients.s0[0]; // Source s0(1,1).
        let at = |j: usize, k: usize, term: usize| ((j - 1) * 4 + (k - 1)) * n + term - 1;
        for hist_term in 1..=coefficients.num_ctf_terms {
            let term = hist_term as usize;
            sum_xi += coefficients.s[at(2, 2, term)];
            sum_yi += coefficients.s[at(1, 2, term)];
            sum_zi += coefficients.s[at(1, 1, term)];
        }
        sum_xi = sum_xi.abs();
        sum_yi = sum_yi.abs();
        sum_zi = sum_zi.abs();
        let biggest_sum = source_max3(sum_xi, sum_yi, sum_zi);
        result.sums = Some(CtfCallerSeriesSums {
            sum_xi,
            sum_yi,
            sum_zi,
            biggest_sum,
        });
        if biggest_sum > 0.0 {
            let first = (sum_xi - sum_yi).abs() / biggest_sum;
            result.first_relative_error = Some(first);
            let mismatch = if first > 0.01 {
                true
            } else {
                let second = (sum_zi - sum_yi).abs() / biggest_sum;
                result.second_relative_error = Some(second);
                second > 0.01
            };
            if mismatch {
                increment_history(&mut result.postcheck_context)?;
                result.caller_ctf_converged = false;
                result.retry_reason = Some(CtfCallerRetryReason::SeriesMismatch);
                result.outcome = CtfCallerStabilityOutcome::Retry;
            }
        } else {
            emit(
                &mut result.diagnostics,
                Fatal,
                format!(
                    "Illegal construction definition, no CTFs calculated for {}",
                    names.construction_name
                ),
            );
            result.outcome = CtfCallerStabilityOutcome::FatalNoCtfs;
            return Ok(result);
        }
    }
    if result.postcheck_context.ctf_time_step >= 7.0 {
        let count = usize::try_from(names.original_tot_layers)
            .ok()
            .filter(|&count| count > 0)
            .ok_or(Error::OriginalLayerNames)?;
        if names.original_layer_material_names.len() != count {
            return Err(Error::OriginalLayerNames);
        }
        emit(
            &mut result.diagnostics,
            Severe,
            format!(
                "CTF calculation convergence problem for Construction=\"{}\".",
                names.construction_name
            ),
        );
        emit(
            &mut result.diagnostics,
            Continue,
            "...with Materials (outside layer to inside)",
        );
        emit(
            &mut result.diagnostics,
            Continue,
            format!("(outside)=\"{}\"", names.original_layer_material_names[0]),
        );
        for layer in 2..=count {
            if layer != count {
                emit(
                    &mut result.diagnostics,
                    Continue,
                    format!(
                        "(next)=\"{}\"",
                        names.original_layer_material_names[layer - 1]
                    ),
                );
            } else {
                emit(
                    &mut result.diagnostics,
                    Continue,
                    format!(
                        "(inside)=\"{}\"",
                        names.original_layer_material_names[layer - 1]
                    ),
                );
            }
        }
        for message in [
            "The Construction report will be produced. This will show more details on Constructions and their materials.",
            "Attempts will be made to complete the CTF process but the report may be incomplete.",
            "Constructs reported after this construction may appear to have all 0 CTFs.",
            "The potential causes of this problem are related to the input for the construction",
            "listed in the severe error above.  The CTF calculate routine is unable to come up",
            "with a series of CTF terms that have a reasonable time step and this indicates an",
            "error.  Check the definition of this construction and the materials that make up",
            "the construction.  Very thin, highly conductive materials may cause problems.",
            "This may be avoided by ignoring the presence of those materials since they probably",
            "do not effect the heat transfer characteristics of the construction.  Highly",
            "conductive or highly resistive layers that are alternated with high mass layers",
            "may also result in problems.  After confirming that the input is correct and",
            "realistic, the user should contact the EnergyPlus support team.",
        ] {
            emit(&mut result.diagnostics, Continue, message);
        }
        result.flags_after.do_ctf_error_report = true;
        result.flags_after.errors_found = true;
        result.outcome = CtfCallerStabilityOutcome::SevenHourSevereBreak;
    }
    Ok(result)
}

#[cfg(test)]
#[path = "ctf_caller_stability_tests.rs"]
mod tests;
