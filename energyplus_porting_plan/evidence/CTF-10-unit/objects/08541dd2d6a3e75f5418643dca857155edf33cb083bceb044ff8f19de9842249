//! Ordered first exponential, inverse, then Gamma; no retry/coefficient driver.

use super::ctf_first_assembly_owner::ConstructionCtfFirstAssembly;
use super::ctf_first_gamma_owner::{
    ConstructionCtfFirstGammas, initialize_construction_ctf_first_gammas,
};
use super::ctf_first_matrix_owner::{
    ConstructionCtfFirstMatrixFunctions, initialize_construction_ctf_first_matrix_functions,
};

/// Separate actual predecessor outcomes and their direct first Gamma return.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfFirstMatrixWithGammas {
    /// Actual ordered06 then05 outcomes, including inverse non-invocation.
    pub methods: ConstructionCtfFirstMatrixFunctions,
    /// Actual public07 return; unavailable prerequisites contain no Gamma arrays.
    pub gammas: ConstructionCtfFirstGammas,
}

/// Call the accepted combined06 owner, then public07 once on its actual return.
///
/// Native Construction.cc884,887,890 orders exponential, inverse, then Gamma.
/// Both calls borrow the same original assembly. Public07 borrows the actual
/// returned predecessor owners, including any error/non-invocation ledger.
/// Public07 alone checks whether Gamma arithmetic can run; this wrapper neither
/// replays a predecessor nor constructs a substitute available Gamma owner.
/// It derives no matrix, identity, timestep, source route or retry state.
pub fn initialize_construction_ctf_first_matrix_with_gammas(
    assembly: &ConstructionCtfFirstAssembly,
) -> ConstructionCtfFirstMatrixWithGammas {
    let methods = initialize_construction_ctf_first_matrix_functions(assembly);
    let gammas = initialize_construction_ctf_first_gammas(assembly, &methods);
    ConstructionCtfFirstMatrixWithGammas { methods, gammas }
}

#[cfg(test)]
#[path = "ctf_first_gamma_ingress_tests.rs"]
mod tests;
