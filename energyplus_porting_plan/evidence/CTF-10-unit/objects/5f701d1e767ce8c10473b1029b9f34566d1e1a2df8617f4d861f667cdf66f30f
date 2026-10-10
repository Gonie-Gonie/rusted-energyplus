//! First private final-coefficient owner from actual ordered first predecessors.

use super::ctf_first_assembly_owner::{ConstructionCtfFirstAssembly, CtfFirstAssemblyUnavailable};
use super::ctf_first_exponential_owner::CtfFirstExponentialUnavailable;
use super::ctf_first_gamma_owner::{ConstructionCtfFirstGammas, CtfFirstGammasUnavailable};
use super::ctf_first_inverse_owner::CtfFirstInverseUnavailable;
use super::ctf_first_matrix_owner::{
    ConstructionCtfFirstMatrixFunctions, PriorExponentialUnavailable,
};
use super::surface_manager::ctf_exponential_matrix::CtfExponentialObservation;
use super::surface_manager::ctf_final_coefficients::{
    CtfFinalCoefficientInput, CtfFinalCoefficientObservation, CtfFinalCoefficientScopeError,
    calculate_selected_ctf_final_coefficients,
};
use super::surface_manager::ctf_gamma_matrix::CtfGammaObservation;
use super::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyObservation, CtfAssemblyRoute, CtfAssemblyUnavailable,
};
use ep_model::ConstructionId;

/// A real prerequisite stop or inconsistent first-attempt owner association.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfFirstFinalCoefficientsUnavailable {
    /// The original first-assembly owner retained this error.
    FirstAssemblyUnavailable(CtfFirstAssemblyUnavailable),
    /// The original assembly retained this unselected route, without arrays.
    AssemblyRouteUnavailable(CtfAssemblyUnavailable),
    /// The ordered actual exponential API returned this error.
    ExponentialUnavailable(CtfFirstExponentialUnavailable),
    /// The exponential retained this unselected route, without arrays.
    ExponentialRouteUnavailable(CtfAssemblyUnavailable),
    /// The ordered caller did not invoke inverse after its actual exponential.
    InverseNotInvoked(PriorExponentialUnavailable),
    /// The invoked actual inverse API returned this error.
    InverseUnavailable(CtfFirstInverseUnavailable),
    /// The actual public Gamma API retained this prerequisite or scope error.
    GammasUnavailable(CtfFirstGammasUnavailable),
    /// The actual Gamma return retained an unselected route, without arrays.
    GammasRouteUnavailable(CtfAssemblyUnavailable),
    /// Actual wrapper and result identities do not name one construction.
    ConstructionIdentityMismatch,
    /// This owner consumes only the actual first assembly invocation.
    NotFirstAttempt(usize),
    /// The original assembly is outside selected 1D/no-source scope.
    AssemblyContext,
    /// A predecessor or ordered caller context differs, including float bits.
    AttemptContextMismatch,
    /// Actual precursor dimensions differ from the original assembly.
    DimensionMismatch,
    /// The unchanged pure final-coefficient method rejected actual shape/scope.
    FinalCoefficientScope(CtfFinalCoefficientScopeError),
}

impl std::fmt::Display for CtfFirstFinalCoefficientsUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "first private CTF final coefficients unavailable: {self:?}"
        )
    }
}

impl std::error::Error for CtfFirstFinalCoefficientsUnavailable {}

/// One private s0/s/e return, distinct from public SI stores or a retry driver.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfFirstFinalCoefficients {
    /// Actual original typed construction identity.
    pub construction_id: ConstructionId,
    /// A reached private method owner or an explicit prerequisite/association stop.
    pub result: Result<CtfFinalCoefficientObservation, CtfFirstFinalCoefficientsUnavailable>,
}

/// Invoke the held private coefficient method once after borrowing actual Gamma.
///
/// Construction.cc884/887/890/894 establishes exponential, inverse, Gamma, then
/// final coefficients. The three arguments must be the actual same first owners.
/// This function invokes no predecessor and reconstructs no identity or context.
/// Actual Gamma1/Gamma2 and AExp are borrowed from their returned owners; actual
/// IdenMatrix/CMat/DMat and context come from the unchanged original assembly.
/// There is no retry, cache activation, public 19-slot SI store or SUR handoff.
pub fn initialize_construction_ctf_first_final_coefficients(
    assembly: &ConstructionCtfFirstAssembly,
    methods: &ConstructionCtfFirstMatrixFunctions,
    gammas: &ConstructionCtfFirstGammas,
) -> ConstructionCtfFirstFinalCoefficients {
    use CtfFirstFinalCoefficientsUnavailable as Unavailable;

    let result = (|| {
        let observed = assembly
            .result
            .as_ref()
            .map_err(|reason| Unavailable::FirstAssemblyUnavailable(*reason))?;
        let actual = match observed {
            CtfAssemblyObservation::Unavailable(reason) => {
                return Err(Unavailable::AssemblyRouteUnavailable(*reason));
            }
            CtfAssemblyObservation::Assembled(actual) => actual,
        };
        let context = actual.context;
        if assembly.construction_id != context.construction_id
            || methods.exponential.construction_id != assembly.construction_id
            || methods.inverse.construction_id != assembly.construction_id
            || gammas.construction_id != assembly.construction_id
        {
            return Err(Unavailable::ConstructionIdentityMismatch);
        }
        if context.attempt_ordinal != 1 {
            return Err(Unavailable::NotFirstAttempt(context.attempt_ordinal));
        }
        if context.route != CtfAssemblyRoute::Assemble
            || context.solution_dimensions != 1
            || context.source_sink_present
            || context.node_source != 0
            || context.node_user_temp != 0
        {
            return Err(Unavailable::AssemblyContext);
        }
        let exponential = methods
            .exponential
            .result
            .as_ref()
            .map_err(|reason| Unavailable::ExponentialUnavailable(*reason))?;
        let exponential = match exponential {
            CtfExponentialObservation::Unavailable(reason) => {
                return Err(Unavailable::ExponentialRouteUnavailable(*reason));
            }
            CtfExponentialObservation::Exponential(actual) => actual,
        };
        let invoked_inverse = methods
            .inverse
            .result
            .as_ref()
            .map_err(|reason| Unavailable::InverseNotInvoked(*reason))?;
        if invoked_inverse.construction_id != assembly.construction_id {
            return Err(Unavailable::ConstructionIdentityMismatch);
        }
        let inverse = invoked_inverse
            .result
            .as_ref()
            .map_err(|reason| Unavailable::InverseUnavailable(*reason))?;
        let gamma = gammas
            .result
            .as_ref()
            .map_err(|reason| Unavailable::GammasUnavailable(*reason))?;
        let gamma = match gamma {
            CtfGammaObservation::Unavailable(reason) => {
                return Err(Unavailable::GammasRouteUnavailable(*reason));
            }
            CtfGammaObservation::Gamma(actual) => actual,
        };
        if !same_attempt_context(context, exponential.context)
            || !same_attempt_context(context, inverse.context)
            || !same_attempt_context(context, gamma.context)
            || !methods
                .inverse
                .assembly_context
                .is_some_and(|prior| same_attempt_context(context, prior))
        {
            return Err(Unavailable::AttemptContextMismatch);
        }
        if actual.rcmax != exponential.rcmax
            || actual.rcmax != inverse.inverse.rcmax
            || actual.rcmax != gamma.rcmax
        {
            return Err(Unavailable::DimensionMismatch);
        }
        calculate_selected_ctf_final_coefficients(
            CtfFinalCoefficientInput {
                rcmax: actual.rcmax,
                a_exp: &exponential.a_exp,
                iden_matrix: &actual.iden_matrix,
                gamma1: &gamma.gamma1,
                gamma2: &gamma.gamma2,
                c_mat: &actual.c_mat,
                d_mat: &actual.d_mat,
            },
            context,
        )
        .map_err(Unavailable::FinalCoefficientScope)
    })();
    ConstructionCtfFirstFinalCoefficients {
        construction_id: assembly.construction_id,
        result,
    }
}

fn same_attempt_context(left: CtfAssemblyContext, right: CtfAssemblyContext) -> bool {
    left.construction_id == right.construction_id
        && left.route == right.route
        && left.solution_dimensions == right.solution_dimensions
        && left.source_sink_present == right.source_sink_present
        && left.node_source == right.node_source
        && left.node_user_temp == right.node_user_temp
        && left.attempt_ordinal == right.attempt_ordinal
        && left.time_step_zone.to_bits() == right.time_step_zone.to_bits()
        && left.ctf_time_step.to_bits() == right.ctf_time_step.to_bits()
        && left.num_histories == right.num_histories
}

#[cfg(test)]
#[path = "ctf_first_final_coefficients_owner_tests.rs"]
mod tests;
