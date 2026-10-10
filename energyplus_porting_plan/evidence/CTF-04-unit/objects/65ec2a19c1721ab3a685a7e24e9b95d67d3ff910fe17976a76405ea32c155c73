//! Construction.cc 638-729: selected 1D, no-source state-space assembly.

use ep_model::ConstructionId;

use super::ctf_layer_preprocessing::CtfNormalizedLayer;

/// An actual whole-call route that never reaches the selected assembly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfAssemblyUnavailable {
    /// Construction.cc 163-165 returns before layer or matrix work.
    UnusedConstruction,
    /// Construction.cc 313-315 returns with the shared loading error set.
    LoadingError,
    /// Construction.cc 985-1018 takes the all-resistive coefficient branch.
    AllResistive,
    /// Construction.cc 404-486 reuses an earlier reversed construction.
    ReversedConstructionReuse,
}

/// The actual branch selected by the enclosing construction owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfAssemblyRoute {
    /// Nodal placement and timestep selection reached the selected branch.
    Assemble,
    /// No initialized matrix buffers are claimed for an unexecuted branch.
    Unavailable(CtfAssemblyUnavailable),
}

/// Caller-owned metadata for one real assembly attempt, not a retry driver.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfAssemblyContext {
    /// Original construction identity, independent of merged layer positions.
    pub construction_id: ConstructionId,
    /// Actual enclosing source route.
    pub route: CtfAssemblyRoute,
    /// Actual SolutionDimensions; this port accepts only one.
    pub solution_dimensions: i32,
    /// Actual source/sink declaration; true is outside this selected port.
    pub source_sink_present: bool,
    /// Actual helper-owned NodeSource, reset to zero on the no-source path.
    pub node_source: i32,
    /// Actual helper-owned NodeUserTemp, reset to zero on the no-source path.
    pub node_user_temp: i32,
    /// One-based invocation ordinal supplied by the actual enclosing loop.
    pub attempt_ordinal: usize,
    /// Actual caller TimeStepZone in hours; no duration is derived here.
    pub time_step_zone: f64,
    /// Actual current-attempt CTFTimeStep, distinct from later returned state.
    pub ctf_time_step: f64,
    /// Actual current-attempt history count, copied without recalculation.
    pub num_histories: i32,
}

/// Actual CTF-03 nodal inputs; no material or node placement is recomputed.
#[derive(Clone, Copy, Debug)]
pub struct CtfAssemblyInput<'a> {
    /// Converted English-unit active prefix in its actual merged order.
    pub layers: &'a [CtfNormalizedLayer],
    /// Actual Nodes for the same active prefix.
    pub nodes: &'a [i32],
    /// Actual dx in feet for the same active prefix.
    pub dx: &'a [f64],
    /// Actual source-owned rcmax; checked against the nodal owner shape.
    pub rcmax: i32,
}

/// Invalid scope or owner shape; never a synthetic Native source outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfAssemblyScopeError {
    /// The perpendicular branch is outside CTF-04.
    UnsupportedDimensions(i32),
    /// Internal-source equations are outside CTF-04.
    InternalSource,
    /// The real no-source helper must have reset both node selectors.
    SourceNodeState,
    /// Only the source's one-through-eleven active layers are admitted.
    LayerCount(usize),
    /// Nodes, dx and layers must refer to the same actual active prefix.
    ActivePrefixShape,
    /// CTF-03 source bounds are six-through-nineteen, or one for interior R.
    NodeCount(usize),
    /// rcmax must be the actual sum of Nodes minus one.
    MatrixShape,
    /// An assembly attempt cannot have the pre-invocation ordinal zero.
    AttemptOrdinal,
}

/// Owned initialized compressed state at the pre-exponential call boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfStateSpaceAssembly {
    /// Metadata copied for this attempt; later retries create separate owners.
    pub context: CtfAssemblyContext,
    /// The unchanged actual converted layer inputs, without invented IDs.
    pub layers: Vec<CtfNormalizedLayer>,
    /// The unchanged actual CTF-03 node counts.
    pub nodes: Vec<i32>,
    /// The unchanged actual CTF-03 spacings.
    pub dx: Vec<f64>,
    /// Both source matrix dimensions and one-based upper bounds.
    pub rcmax: i32,
    /// AMat(i1,i2), with i2 contiguous: (i1-1)*rcmax+(i2-1).
    pub a_mat: Vec<f64>,
    /// Actual source-initialized IdenMatrix, with the same two-index layout.
    pub iden_matrix: Vec<f64>,
    /// Actual compressed BMat(1..3), not an expanded dense input matrix.
    pub b_mat: [f64; 3],
    /// Actual compressed CMat(1..2).
    pub c_mat: [f64; 2],
    /// Actual compressed DMat(1..2).
    pub d_mat: [f64; 2],
}

impl CtfStateSpaceAssembly {
    /// Read one source one-based element without transposing the owner.
    #[must_use]
    pub fn a(&self, i1: usize, i2: usize) -> Option<f64> {
        let rcmax = self.rcmax as usize;
        if i1 == 0 || i2 == 0 || i1 > rcmax || i2 > rcmax {
            return None;
        }
        self.a_mat.get((i1 - 1) * rcmax + i2 - 1).copied()
    }
}

/// A reached owner or an explicit unavailable route, never a zero fallback.
#[derive(Clone, Debug, PartialEq)]
pub enum CtfAssemblyObservation {
    /// No selected arithmetic or initialized matrix was observed.
    Unavailable(CtfAssemblyUnavailable),
    /// Complete selected arithmetic for exactly one invocation.
    Assembled(Box<CtfStateSpaceAssembly>),
}

/// Preserve the source's assignments and floating-point operation order.
///
/// This function does not allocate the downstream solver workspaces, call the
/// exponential/inverse/Gamma/coefficient helpers, choose a route or run retries.
#[allow(clippy::neg_multiply)] // Retain the original interface multiplication order.
pub fn assemble_1d_ctf_state_space(
    input: CtfAssemblyInput<'_>,
    context: CtfAssemblyContext,
) -> Result<CtfAssemblyObservation, CtfAssemblyScopeError> {
    if let CtfAssemblyRoute::Unavailable(reason) = context.route {
        return Ok(CtfAssemblyObservation::Unavailable(reason));
    }
    if context.source_sink_present {
        return Err(CtfAssemblyScopeError::InternalSource);
    }
    if context.solution_dimensions != 1 {
        return Err(CtfAssemblyScopeError::UnsupportedDimensions(
            context.solution_dimensions,
        ));
    }
    if context.node_source != 0 || context.node_user_temp != 0 {
        return Err(CtfAssemblyScopeError::SourceNodeState);
    }
    let count = input.layers.len();
    if count == 0 || count > 11 {
        return Err(CtfAssemblyScopeError::LayerCount(count));
    }
    if input.nodes.len() != count || input.dx.len() != count {
        return Err(CtfAssemblyScopeError::ActivePrefixShape);
    }
    for (index, &nodes) in input.nodes.iter().enumerate() {
        let interior_resistance = index > 0 && index + 1 < count && input.layers[index].res_layer;
        let valid = if interior_resistance {
            nodes == 1
        } else {
            (6..=19).contains(&nodes)
        };
        if !valid {
            return Err(CtfAssemblyScopeError::NodeCount(index + 1));
        }
    }
    let source_rcmax = input.nodes.iter().sum::<i32>() - 1;
    if input.rcmax != source_rcmax {
        return Err(CtfAssemblyScopeError::MatrixShape);
    }
    if context.attempt_ordinal == 0 {
        return Err(CtfAssemblyScopeError::AttemptOrdinal);
    }
    // Checked positive source bounds above make this integer index conversion exact.
    let rcmax = source_rcmax as usize;
    let layers = input.layers;
    let dx = input.dx;
    let mut a_mat = vec![0.0; rcmax * rcmax];
    // Construction.cc644-648: source producer, not observer reconstruction.
    let mut iden_matrix = vec![0.0; rcmax * rcmax];
    for ir in 1..=rcmax {
        write_a(&mut iden_matrix, rcmax, ir, ir, 1.0);
    }
    let mut b_mat = [0.0; 3];

    let mut cap = layers[0].rho * layers[0].cp * dx[0];
    cap *= 1.5;
    let mut dxtmp = 1.0 / dx[0] / cap;
    write_a(&mut a_mat, rcmax, 1, 1, -2.0 * layers[0].rk * dxtmp);
    write_a(&mut a_mat, rcmax, 2, 1, layers[0].rk * dxtmp);
    b_mat[0] = layers[0].rk * dxtmp;

    let mut layer = 0;
    let mut node_in_layer = 2;
    for node in 2..rcmax {
        if node_in_layer == input.nodes[layer] && count != 1 {
            cap = (layers[layer].rho * layers[layer].cp * dx[layer]
                + layers[layer + 1].rho * layers[layer + 1].cp * dx[layer + 1])
                * 0.5;
            write_a(
                &mut a_mat,
                rcmax,
                node - 1,
                node,
                layers[layer].rk / dx[layer] / cap,
            );
            write_a(
                &mut a_mat,
                rcmax,
                node,
                node,
                -1.0 * (layers[layer].rk / dx[layer] + layers[layer + 1].rk / dx[layer + 1]) / cap,
            );
            write_a(
                &mut a_mat,
                rcmax,
                node + 1,
                node,
                layers[layer + 1].rk / dx[layer + 1] / cap,
            );
            node_in_layer = 0;
            layer += 1;
        } else {
            cap = layers[layer].rho * layers[layer].cp * dx[layer];
            dxtmp = 1.0 / dx[layer] / cap;
            write_a(&mut a_mat, rcmax, node - 1, node, layers[layer].rk * dxtmp);
            write_a(
                &mut a_mat,
                rcmax,
                node,
                node,
                -2.0 * layers[layer].rk * dxtmp,
            );
            write_a(&mut a_mat, rcmax, node + 1, node, layers[layer].rk * dxtmp);
        }
        node_in_layer += 1;
    }

    let last = count - 1;
    cap = layers[last].rho * layers[last].cp * dx[last];
    cap *= 1.5;
    dxtmp = 1.0 / dx[last] / cap;
    write_a(
        &mut a_mat,
        rcmax,
        rcmax,
        rcmax,
        -2.0 * layers[last].rk * dxtmp,
    );
    write_a(&mut a_mat, rcmax, rcmax - 1, rcmax, layers[last].rk * dxtmp);
    b_mat[1] = layers[last].rk * dxtmp;
    let c_mat = [-layers[0].rk / dx[0], layers[last].rk / dx[last]];
    let d_mat = [layers[0].rk / dx[0], -layers[last].rk / dx[last]];
    Ok(CtfAssemblyObservation::Assembled(Box::new(
        CtfStateSpaceAssembly {
            context,
            layers: layers.to_vec(),
            nodes: input.nodes.to_vec(),
            dx: dx.to_vec(),
            rcmax: source_rcmax,
            a_mat,
            iden_matrix,
            b_mat,
            c_mat,
            d_mat,
        },
    )))
}

fn write_a(values: &mut [f64], rcmax: usize, i1: usize, i2: usize, value: f64) {
    values[(i1 - 1) * rcmax + i2 - 1] = value;
}

#[cfg(test)]
mod tests;
