use super::*;

fn layer(rk: f64, rho: f64, cp: f64, res_layer: bool) -> CtfNormalizedLayer {
    CtfNormalizedLayer {
        dl: 12.0,
        rk,
        rho,
        cp,
        lr: 4.0,
        res_layer,
    }
}

fn context() -> CtfAssemblyContext {
    CtfAssemblyContext {
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
    }
}

fn assembled(
    layers: &[CtfNormalizedLayer],
    nodes: &[i32],
    dx: &[f64],
    rcmax: i32,
    context: CtfAssemblyContext,
) -> CtfStateSpaceAssembly {
    let observation = assemble_1d_ctf_state_space(
        CtfAssemblyInput {
            layers,
            nodes,
            dx,
            rcmax,
        },
        context,
    )
    .expect("literal selected nodal state");
    let value = match observation {
        CtfAssemblyObservation::Assembled(value) => Some(*value),
        CtfAssemblyObservation::Unavailable(_) => None,
    };
    value.expect("selected arithmetic was unavailable")
}

#[test]
fn single_layer_keeps_source_orientation_boundary_mass_and_compressed_shapes() {
    let layers = [layer(3.0, 2.0, 2.0, false)];
    let value = assembled(&layers, &[6], &[2.0], 5, context());
    // Independently specified binary-exact entries. In particular a(2,1)
    // differs from a(1,2), so a transposed or uniformly lumped owner fails.
    assert_eq!(
        value.a_mat,
        vec![
            -0.25, 0.1875, 0.0, 0.0, 0.0, 0.125, -0.375, 0.1875, 0.0, 0.0, 0.0, 0.1875, -0.375,
            0.1875, 0.0, 0.0, 0.0, 0.1875, -0.375, 0.125, 0.0, 0.0, 0.0, 0.1875, -0.25,
        ]
    );
    assert_eq!(value.b_mat, [0.125, 0.125, 0.0]);
    assert_eq!(value.c_mat, [-1.5, 1.5]);
    assert_eq!(value.d_mat, [1.5, -1.5]);
    assert_eq!(value.a(2, 1), Some(0.125));
    assert_eq!(value.a(1, 2), Some(0.1875));
    assert_eq!(value.a(0, 1), None);
    assert_eq!(value.a(1, 6), None);
    assert_eq!(value.a_mat[2].to_bits(), 0.0_f64.to_bits());
    assert_eq!(value.b_mat[2].to_bits(), 0.0_f64.to_bits());
    assert_eq!(value.layers, layers);
    assert_eq!(value.nodes, [6]);
    assert_eq!(value.dx, [2.0]);
}

#[test]
fn two_mass_layers_use_half_capacitances_and_keep_literal_stack_direction() {
    let outside = layer(3.0, 2.0, 2.0, false);
    let inside = layer(6.0, 4.0, 2.0, false);
    let forward = assembled(&[outside, inside], &[6, 6], &[2.0, 2.0], 11, context());
    assert_eq!(forward.a(5, 6), Some(0.125));
    assert_eq!(forward.a(6, 6), Some(-0.375));
    assert_eq!(forward.a(7, 6), Some(0.25));
    assert_eq!(forward.a(7, 7), Some(-0.375));
    assert_eq!(forward.c_mat, [-1.5, 3.0]);
    assert_eq!(forward.d_mat, [1.5, -3.0]);

    // A fresh explicit assembly route, not the whole-method reverse shortcut.
    let reverse = assembled(&[inside, outside], &[6, 6], &[2.0, 2.0], 11, context());
    assert_eq!(reverse.a(5, 6), Some(0.25));
    assert_eq!(reverse.a(7, 6), Some(0.125));
    assert_eq!(reverse.c_mat, [-3.0, 1.5]);
    for i1 in 1..=11 {
        for i2 in 1..=11 {
            assert_eq!(forward.a(i1, i2), reverse.a(12 - i1, 12 - i2));
        }
    }
}

#[test]
fn one_node_interior_resistance_advances_two_consecutive_interfaces() {
    let layers = [
        layer(3.0, 2.0, 2.0, false),
        layer(1.0, 0.0, 0.0, true),
        layer(6.0, 4.0, 2.0, false),
    ];
    let value = assembled(&layers, &[6, 1, 6], &[2.0, 1.0, 2.0], 12, context());
    assert_eq!(value.a(5, 6), Some(0.375));
    assert_eq!(value.a(6, 6), Some(-0.625));
    assert_eq!(value.a(7, 6), Some(0.25));
    assert_eq!(value.a(6, 7), Some(0.125));
    assert_eq!(value.a(7, 7), Some(-0.5));
    assert_eq!(value.a(8, 7), Some(0.375));
    assert_eq!(value.a(8, 8), Some(-0.375));
    assert!(value.a_mat.iter().all(|value| value.is_finite()));
    assert_eq!(value.layers[1].rho.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn retry_metadata_never_replaces_or_recomputes_the_nodal_owner() {
    let layers = [layer(3.0, 2.0, 2.0, false)];
    let initial = assembled(&layers, &[6], &[2.0], 5, context());
    let next_context = CtfAssemblyContext {
        attempt_ordinal: 2,
        ctf_time_step: 0.75,
        num_histories: 3,
        ..context()
    };
    let next = assembled(&layers, &[6], &[2.0], 5, next_context);
    assert_eq!(initial.context.ctf_time_step, 0.5);
    assert_eq!(initial.context.num_histories, 2);
    assert_eq!(next.context, next_context);
    assert_eq!(next.nodes, initial.nodes);
    assert_eq!(next.dx, initial.dx);
    assert_eq!(next.a_mat, initial.a_mat);
    assert_eq!(next.b_mat, initial.b_mat);
    assert_eq!(next.c_mat, initial.c_mat);
    assert_eq!(next.d_mat, initial.d_mat);
}

#[test]
fn unavailable_routes_have_no_synthetic_matrix_or_input_access() {
    for reason in [
        CtfAssemblyUnavailable::UnusedConstruction,
        CtfAssemblyUnavailable::LoadingError,
        CtfAssemblyUnavailable::AllResistive,
        CtfAssemblyUnavailable::ReversedConstructionReuse,
    ] {
        let result = assemble_1d_ctf_state_space(
            CtfAssemblyInput {
                layers: &[],
                nodes: &[],
                dx: &[],
                rcmax: 0,
            },
            CtfAssemblyContext {
                route: CtfAssemblyRoute::Unavailable(reason),
                ..context()
            },
        );
        assert_eq!(result, Ok(CtfAssemblyObservation::Unavailable(reason)));
    }
}

#[test]
fn out_of_scope_and_misaligned_owners_fail_without_a_matrix() {
    let layers = [layer(3.0, 2.0, 2.0, false)];
    let valid = CtfAssemblyInput {
        layers: &layers,
        nodes: &[6],
        dx: &[2.0],
        rcmax: 5,
    };
    for (invalid, expected) in [
        (
            CtfAssemblyContext {
                source_sink_present: true,
                ..context()
            },
            CtfAssemblyScopeError::InternalSource,
        ),
        (
            CtfAssemblyContext {
                solution_dimensions: 2,
                ..context()
            },
            CtfAssemblyScopeError::UnsupportedDimensions(2),
        ),
        (
            CtfAssemblyContext {
                node_source: 1,
                ..context()
            },
            CtfAssemblyScopeError::SourceNodeState,
        ),
        (
            CtfAssemblyContext {
                node_user_temp: 1,
                ..context()
            },
            CtfAssemblyScopeError::SourceNodeState,
        ),
        (
            CtfAssemblyContext {
                attempt_ordinal: 0,
                ..context()
            },
            CtfAssemblyScopeError::AttemptOrdinal,
        ),
    ] {
        assert_eq!(assemble_1d_ctf_state_space(valid, invalid), Err(expected));
    }
    assert_eq!(
        assemble_1d_ctf_state_space(CtfAssemblyInput { rcmax: 6, ..valid }, context()),
        Err(CtfAssemblyScopeError::MatrixShape),
    );
    assert_eq!(
        assemble_1d_ctf_state_space(
            CtfAssemblyInput {
                nodes: &[1],
                rcmax: 0,
                ..valid
            },
            context()
        ),
        Err(CtfAssemblyScopeError::NodeCount(1)),
    );
    assert_eq!(
        assemble_1d_ctf_state_space(CtfAssemblyInput { dx: &[], ..valid }, context()),
        Err(CtfAssemblyScopeError::ActivePrefixShape),
    );
}

#[test]
fn ieee_arithmetic_class_is_returned_in_the_actual_owner() {
    // Pure-source behavior only: this does not propose a Native full-call case.
    let layers = [layer(3.0, 0.0, 2.0, false)];
    let value = assembled(&layers, &[6], &[2.0], 5, context());
    assert!(value.a(1, 1).unwrap().is_infinite());
    assert!(value.a(1, 1).unwrap().is_sign_negative());
    assert!(value.b_mat[0].is_infinite());
    assert_eq!(value.b_mat[2].to_bits(), 0.0_f64.to_bits());
}
