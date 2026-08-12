use crate::ideal_loads::DirectZonePurchasedAirScheduledCouplingOutput;

// Match the production lazy owner gate: inactive routes must not read CP329.
#[allow(dead_code, clippy::unnecessary_lazy_evaluations)]
pub(super) fn cp419_owner_operand(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
) -> Option<f64> {
    output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment
        .post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed
        .then(|| {
            output
                .calculation_cooling_mixed_air_call
                .mixed_air_humidity_ratio
        })
        .flatten()
}

#[test]
fn cp419_fixture_is_owner_gated() {
    let source = include_str!(
        "../coupled_runtime/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_validation.rs"
    );
    assert!(source.contains("then(|| owner.mixed_air_humidity_ratio)"));
    assert!(!source.contains("then_some(owner.mixed_air_humidity_ratio)"));
    assert!(source.contains("snapshot_has_exact_cp418_prefix_and_local_assignment"));
    assert!(!source.contains("private_characterization"));
}
