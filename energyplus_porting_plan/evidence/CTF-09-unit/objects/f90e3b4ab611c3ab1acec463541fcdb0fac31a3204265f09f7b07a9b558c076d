//! First selected Gamma owner from existing ordered exponential/inverse returns.

use super::ctf_first_assembly_owner::{ConstructionCtfFirstAssembly, CtfFirstAssemblyUnavailable};
use super::ctf_first_exponential_owner::CtfFirstExponentialUnavailable;
use super::ctf_first_inverse_owner::CtfFirstInverseUnavailable;
use super::ctf_first_matrix_owner::{
    ConstructionCtfFirstMatrixFunctions, PriorExponentialUnavailable,
};
use super::surface_manager::ctf_exponential_matrix::CtfExponentialObservation;
use super::surface_manager::ctf_gamma_matrix::{
    CtfGammaInput, CtfGammaObservation, CtfGammaScopeError, calculate_selected_ctf_gammas,
};
use super::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyObservation, CtfAssemblyRoute, CtfAssemblyUnavailable,
};
use ep_model::ConstructionId;

/// An actual prerequisite stop or inconsistent first-attempt owner association.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfFirstGammasUnavailable {
    /// The original first-assembly owner returned this prerequisite error.
    FirstAssemblyUnavailable(CtfFirstAssemblyUnavailable),
    /// The original assembly retained an unselected source route.
    AssemblyRouteUnavailable(CtfAssemblyUnavailable),
    /// The ordered actual exponential API returned this error.
    ExponentialUnavailable(CtfFirstExponentialUnavailable),
    /// The ordered exponential retained an unselected route, without arrays.
    ExponentialRouteUnavailable(CtfAssemblyUnavailable),
    /// The ordered caller did not invoke inverse after its prior exponential.
    InverseNotInvoked(PriorExponentialUnavailable),
    /// The invoked actual inverse API returned this error.
    InverseUnavailable(CtfFirstInverseUnavailable),
    /// The actual wrapper and result identities do not name one construction.
    ConstructionIdentityMismatch,
    /// This owner consumes only the actual first assembly invocation.
    NotFirstAttempt(usize),
    /// The original assembly context is outside selected 1D/no-source scope.
    AssemblyContext,
    /// A predecessor context or ordered caller context differs, including bits.
    AttemptContextMismatch,
    /// Actual predecessor dimensions do not match the original assembly.
    DimensionMismatch,
    /// The unchanged pure Gamma helper rejected actual scope or buffer shape.
    GammaScope(CtfGammaScopeError),
}

impl std::fmt::Display for CtfFirstGammasUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "first CTF Gamma owner unavailable: {self:?}")
    }
}

impl std::error::Error for CtfFirstGammasUnavailable {}

/// One direct Gamma return, distinct from final coefficients or a retry driver.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfFirstGammas {
    /// Actual original typed construction identity.
    pub construction_id: ConstructionId,
    /// A reached Gamma owner or an explicit prerequisite/association stop.
    pub result: Result<CtfGammaObservation, CtfFirstGammasUnavailable>,
}

/// Invoke Gamma once after borrowing the actual ordered first06/05 returns.
///
/// Construction.cc884/887/890 establishes exponential, inverse, then Gamma.
/// This function does not invoke either predecessor, select a route, generate
/// identity, recalculate timestep or activate cache/driver/retry execution.
/// AInv/AExp are borrowed from their real API returns; actual IdenMatrix/BMat
/// and current context are borrowed from the same first-assembly owner.
pub fn initialize_construction_ctf_first_gammas(
    assembly: &ConstructionCtfFirstAssembly,
    methods: &ConstructionCtfFirstMatrixFunctions,
) -> ConstructionCtfFirstGammas {
    use CtfFirstGammasUnavailable as Unavailable;

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
        if !same_attempt_context(context, exponential.context)
            || !same_attempt_context(context, inverse.context)
            || !methods
                .inverse
                .assembly_context
                .is_some_and(|prior| same_attempt_context(context, prior))
        {
            return Err(Unavailable::AttemptContextMismatch);
        }
        if actual.rcmax != exponential.rcmax || actual.rcmax != inverse.inverse.rcmax {
            return Err(Unavailable::DimensionMismatch);
        }
        calculate_selected_ctf_gammas(
            CtfGammaInput {
                rcmax: actual.rcmax,
                a_inv: &inverse.inverse.a_inv,
                a_exp: &exponential.a_exp,
                iden_matrix: &actual.iden_matrix,
                b_mat: &actual.b_mat,
            },
            context,
        )
        .map_err(Unavailable::GammaScope)
    })();
    ConstructionCtfFirstGammas {
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
#[path = "ctf_first_gamma_owner_tests.rs"]
mod tests;
