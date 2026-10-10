//! Literal Construction.cc1591-1906, selected 1D/no-source method only.

use super::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyRoute, CtfAssemblyUnavailable,
};

/// Same-attempt actual owners after the Gamma method has returned.
#[derive(Clone, Copy, Debug)]
pub struct CtfFinalCoefficientInput<'a> {
    /// Actual positive source matrix dimension.
    pub rcmax: i32,
    /// Actual AExp and source-produced IdenMatrix, i2 contiguous.
    pub a_exp: &'a [f64],
    /// Original source-produced identity owner in the same square layout.
    pub iden_matrix: &'a [f64],
    /// Actual Gamma(j,i), three rows with i contiguous.
    pub gamma1: &'a [f64],
    /// Unchanged actual Gamma2 in the same three-row layout.
    pub gamma2: &'a [f64],
    /// Original compressed CMat(1..2) and DMat(1..2).
    pub c_mat: &'a [f64],
    /// Actual compressed direct boundary owner with exactly two elements.
    pub d_mat: &'a [f64],
}

/// Scope/shape rejection is distinct from any actual Native method outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfFinalCoefficientScopeError {
    /// A reached matrix must have positive rcmax in the source i32 domain.
    Dimension(i32),
    /// Each actual square owner must contain rcmax squared elements.
    MatrixShape,
    /// Gamma and history dimensions must fit their declared owner shapes.
    GammaShape,
    /// Actual compressed CMat and DMat must each have two elements.
    BoundaryShape,
    /// The perpendicular branch is outside this selected port.
    UnsupportedDimensions(i32),
    /// Internal-source equations are outside this selected port.
    InternalSource,
    /// The selected no-source owner must have both selectors reset to zero.
    SourceNodeState,
    /// An actual enclosing invocation cannot have ordinal zero.
    AttemptOrdinal,
}

/// Private method state before its three workspace deallocations.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfFinalCoefficients {
    /// Unchanged same-attempt metadata; no retry state is generated here.
    pub context: CtfAssemblyContext,
    /// Actual source matrix dimension retained unchanged.
    pub rcmax: i32,
    /// Actual mutated Gamma1=original Gamma1-Gamma2, not the original07 owner.
    pub gamma1_minus_gamma2: Vec<f64>,
    /// Actual s0(j,k), (j-1)*4+(k-1), all3-by-4 slots retained.
    pub s0: [f64; 12],
    /// Actual s(j,k,term), ((j-1)*4+(k-1))*rcmax+(term-1).
    /// Every source-allocated slot is retained, including initialized zeros.
    pub s: Vec<f64>,
    /// Actual e(term), one-through-rcmax in contiguous storage.
    pub e: Vec<f64>,
    /// NumCTFTerms assigned by this method, before caller stability checks.
    pub num_ctf_terms: i32,
    /// This method's local CTFConvrg, not caller retry/stability convergence.
    pub history_loop_converged: bool,
    /// This method's inum after its exact increment/termination sequence.
    pub next_history_term: i32,
    /// Actual workspaces immediately before Construction.cc1903-1905.
    pub phi_r0: Vec<f64>,
    /// Actual current R matrix, with i2 contiguous.
    pub rnew: Vec<f64>,
    /// Actual previous R matrix, with i2 contiguous.
    pub rold: Vec<f64>,
}

/// A real unavailable route never publishes invented coefficient workspaces.
#[derive(Clone, Debug, PartialEq)]
pub enum CtfFinalCoefficientObservation {
    /// The enclosing source route did not invoke this selected method.
    Unavailable(CtfAssemblyUnavailable),
    /// Selected method state from one actual invocation.
    Coefficients(Box<CtfFinalCoefficients>),
}

/// One invocation from actual same-attempt07/06/04 owners; no retry or cache.
///
/// This ports the private s0/s/e calculation, not caller SI coefficient stores,
/// term-count retry limits, convergence checks, EIO, or a whole CTF solution.
/// No finite/zero conditioning guard changes the source's arithmetic.
#[allow(clippy::collapsible_if)] // Preserve the source's nested epsilon guards.
pub fn calculate_selected_ctf_final_coefficients(
    input: CtfFinalCoefficientInput<'_>,
    context: CtfAssemblyContext,
) -> Result<CtfFinalCoefficientObservation, CtfFinalCoefficientScopeError> {
    use CtfFinalCoefficientScopeError as Error;
    if let CtfAssemblyRoute::Unavailable(reason) = context.route {
        return Ok(CtfFinalCoefficientObservation::Unavailable(reason));
    }
    if context.source_sink_present {
        return Err(Error::InternalSource);
    }
    if context.solution_dimensions != 1 {
        return Err(Error::UnsupportedDimensions(context.solution_dimensions));
    }
    if context.node_source != 0 || context.node_user_temp != 0 {
        return Err(Error::SourceNodeState);
    }
    if context.attempt_ordinal == 0 {
        return Err(Error::AttemptOrdinal);
    }
    let n = usize::try_from(input.rcmax)
        .ok()
        .filter(|&n| n > 0)
        .ok_or(Error::Dimension(input.rcmax))?;
    let square = n.checked_mul(n).ok_or(Error::MatrixShape)?;
    let gamma_size = n.checked_mul(3).ok_or(Error::GammaShape)?;
    let history_size = n.checked_mul(12).ok_or(Error::GammaShape)?;
    if input.a_exp.len() != square || input.iden_matrix.len() != square {
        return Err(Error::MatrixShape);
    }
    if input.gamma1.len() != gamma_size || input.gamma2.len() != gamma_size {
        return Err(Error::GammaShape);
    }
    if input.c_mat.len() != 2 || input.d_mat.len() != 2 {
        return Err(Error::BoundaryShape);
    }
    let m = |i1: usize, i2: usize| (i1 - 1) * n + i2 - 1;
    let g = |j: usize, i: usize| (j - 1) * n + i - 1;
    let z = |j: usize, k: usize| (j - 1) * 4 + k - 1;
    let h = |j: usize, k: usize, term: usize| ((j - 1) * 4 + k - 1) * n + term - 1;
    // Source initializes these exact owners and copies actual R0/IdenMatrix.
    let mut phi_r0 = vec![0.0; square];
    let mut rold = vec![0.0; square];
    let mut rnew = input.iden_matrix.to_vec();
    let mut s0 = [0.0; 12];
    let mut s = vec![0.0; history_size];
    let mut e = vec![0.0; n];
    let mut gamma1 = input.gamma1.to_vec();
    for i in 1..=n {
        for j in 1..=3 {
            gamma1[g(j, i)] -= input.gamma2[g(j, i)];
        }
    }
    s0[z(1, 1)] = input.c_mat[0] * input.gamma2[g(1, 1)] + input.d_mat[0];
    s0[z(2, 1)] = input.c_mat[0] * input.gamma2[g(2, 1)];
    s0[z(3, 1)] = input.c_mat[0] * input.gamma2[g(3, 1)];
    s0[z(1, 2)] = input.c_mat[1] * input.gamma2[g(1, n)];
    s0[z(2, 2)] = input.c_mat[1] * input.gamma2[g(2, n)] + input.d_mat[1];
    s0[z(3, 2)] = input.c_mat[1] * input.gamma2[g(3, n)];
    if s0[z(2, 1)].abs() != s0[z(1, 2)].abs() {
        let avg = (s0[z(2, 1)].abs() + s0[z(1, 2)].abs()) * 0.5;
        s0[z(2, 1)] *= avg / s0[z(2, 1)].abs();
        s0[z(1, 2)] *= avg / s0[z(1, 2)].abs();
    }
    let mut inum: i32 = 1;
    let mut converged = false;
    let mut num_ctf_terms = 0;
    while !converged && inum < input.rcmax {
        let term = inum as usize;
        let mut trace = 0.0;
        for ir in 1..=n {
            for ic in 1..=n {
                phi_r0[m(ic, ir)] = 0.0;
                for is in 1..=n {
                    // Native rTinyValue is double epsilon, not MIN_POSITIVE.
                    if rnew[m(ic, is)].abs() > f64::EPSILON {
                        if input.a_exp[m(is, ir)].abs() > (f64::EPSILON / rnew[m(ic, is)]).abs() {
                            phi_r0[m(ic, ir)] += input.a_exp[m(is, ir)] * rnew[m(ic, is)];
                        }
                    }
                }
            }
            trace += phi_r0[m(ir, ir)];
        }
        e[term - 1] = -trace / f64::from(inum);
        for ir in 1..=n {
            for ic in 1..=n {
                rold[m(ic, ir)] = rnew[m(ic, ir)];
                rnew[m(ic, ir)] = phi_r0[m(ic, ir)];
            }
            rnew[m(ir, ir)] += e[term - 1];
        }
        for j in 1..=3 {
            for is2 in 1..=n {
                s[h(j, 1, term)] += input.c_mat[0]
                    * (rold[m(is2, 1)] * gamma1[g(j, is2)]
                        + rnew[m(is2, 1)] * input.gamma2[g(j, is2)]);
                s[h(j, 2, term)] += input.c_mat[1]
                    * (rold[m(is2, n)] * gamma1[g(j, is2)]
                        + rnew[m(is2, n)] * input.gamma2[g(j, is2)]);
            }
            if j != 3 {
                s[h(j, j, term)] += e[term - 1] * input.d_mat[j - 1];
            }
        }
        if s[h(2, 1, term)].abs() != s[h(1, 2, term)].abs() {
            let avg = (s[h(2, 1, term)].abs() + s[h(1, 2, term)].abs()) * 0.5;
            s[h(2, 1, term)] *= avg / s[h(2, 1, term)].abs();
            s[h(1, 2, term)] *= avg / s[h(1, 2, term)].abs();
        }
        if e[0] == 0.0 {
            num_ctf_terms = 1;
            converged = true;
        } else {
            let rat = (e[term - 1] / e[0]).abs();
            if rat < 1.0e-13 {
                num_ctf_terms = inum;
                converged = true;
            }
        }
        inum += 1;
    }
    if !converged {
        let mut trace = 0.0;
        for ir in 1..=n {
            for is in 1..=n {
                trace += input.a_exp[m(is, ir)] * rnew[m(ir, is)];
            }
        }
        e[n - 1] = -trace / f64::from(input.rcmax);
        for j in 1..=3 {
            for is2 in 1..=n {
                s[h(j, 1, n)] += input.c_mat[0] * rnew[m(is2, 1)] * gamma1[g(j, is2)];
                s[h(j, 2, n)] += input.c_mat[1] * rnew[m(is2, n)] * gamma1[g(j, is2)];
            }
        }
        s[h(1, 1, n)] += e[n - 1] * input.d_mat[0];
        s[h(2, 2, n)] += e[n - 1] * input.d_mat[1];
        num_ctf_terms = input.rcmax;
        if s[h(2, 1, n)].abs() != s[h(1, 2, n)].abs() {
            let avg = (s[h(2, 1, n)].abs() + s[h(1, 2, n)].abs()) * 0.5;
            s[h(2, 1, n)] *= avg / s[h(2, 1, n)].abs();
            s[h(1, 2, n)] *= avg / s[h(1, 2, n)].abs();
        }
    }
    Ok(CtfFinalCoefficientObservation::Coefficients(Box::new(
        CtfFinalCoefficients {
            context,
            rcmax: input.rcmax,
            gamma1_minus_gamma2: gamma1,
            s0,
            s,
            e,
            num_ctf_terms,
            history_loop_converged: converged,
            next_history_term: inum,
            phi_r0,
            rnew,
            rold,
        },
    )))
}

#[cfg(test)]
#[path = "ctf_final_coefficients_tests.rs"]
mod tests;
