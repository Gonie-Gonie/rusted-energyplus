//! Ordered first exponential then inverse; no retry or coefficient driver.

use super::ctf_first_assembly_owner::ConstructionCtfFirstAssembly;
use super::ctf_first_exponential_owner::{
    ConstructionCtfFirstExponential, CtfFirstExponentialUnavailable,
    initialize_construction_ctf_first_exponential,
};
use super::ctf_first_inverse_owner::{
    ConstructionCtfFirstInverse, initialize_construction_ctf_first_inverse,
};
use super::surface_manager::ctf_exponential_matrix::CtfExponentialObservation;
use super::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyObservation, CtfAssemblyUnavailable,
};
use ep_model::ConstructionId;

/// A genuine prior exponential outcome prevented an inverse invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PriorExponentialUnavailable {
    /// The real exponential API returned this exact prerequisite/scope error.
    Error(CtfFirstExponentialUnavailable),
    /// The real exponential API retained this unselected route.
    Route(CtfAssemblyUnavailable),
}

impl std::fmt::Display for PriorExponentialUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "inverse not invoked after first exponential: {self:?}"
        )
    }
}

impl std::error::Error for PriorExponentialUnavailable {}

/// The inverse call ledger after its actual first exponential prerequisite.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfFirstInverseAfterExponential {
    /// Original typed construction identity, in unchanged initialization order.
    pub construction_id: ConstructionId,
    /// Copy of the real assembled context, if that prerequisite owner exists.
    /// An unavailable assembly has no fabricated context or default timestep.
    pub assembly_context: Option<CtfAssemblyContext>,
    /// Ok means the public inverse API was invoked; its inner Result is retained.
    /// Err means it was not invoked, and contains the actual prior reason.
    pub result: Result<ConstructionCtfFirstInverse, PriorExponentialUnavailable>,
}

/// Two distinct ordered outcomes from one borrowed original first assembly.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfFirstMatrixFunctions {
    /// Actual public06 return, including its error or unavailable route.
    pub exponential: ConstructionCtfFirstExponential,
    /// Actual public05 invocation or explicit non-invocation after public06.
    pub inverse: ConstructionCtfFirstInverseAfterExponential,
}

/// Invoke real06 before real05 on the same original AMat and IdenMatrix.
///
/// This selected first-only call follows Native Construction.cc884 then887.
/// Exponential errors stop the inverse call; no public05 owner is fabricated.
/// On success inverse borrows `assembly`, never the exponential result matrix.
/// Existing public05/06 functions own scope/association/shape checks unchanged.
/// No retry, source branch, timestep, identity or coefficient state is generated.
pub fn initialize_construction_ctf_first_matrix_functions(
    assembly: &ConstructionCtfFirstAssembly,
) -> ConstructionCtfFirstMatrixFunctions {
    let exponential = initialize_construction_ctf_first_exponential(assembly);
    let inverse_result = match &exponential.result {
        Err(reason) => Err(PriorExponentialUnavailable::Error(*reason)),
        Ok(CtfExponentialObservation::Unavailable(reason)) => {
            Err(PriorExponentialUnavailable::Route(*reason))
        }
        Ok(CtfExponentialObservation::Exponential(_)) => {
            Ok(initialize_construction_ctf_first_inverse(assembly))
        }
    };
    let assembly_context = match &assembly.result {
        Ok(CtfAssemblyObservation::Assembled(actual)) => Some(actual.context),
        _ => None,
    };
    ConstructionCtfFirstMatrixFunctions {
        exponential,
        inverse: ConstructionCtfFirstInverseAfterExponential {
            construction_id: assembly.construction_id,
            assembly_context,
            result: inverse_result,
        },
    }
}

#[cfg(test)]
#[path = "ctf_first_matrix_owner_tests.rs"]
mod tests;
