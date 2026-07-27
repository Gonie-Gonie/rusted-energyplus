use super::*;

#[test]
fn model_multipliers_are_forwarded_and_removed_once_from_feedback() {
    let (model, cache) = fixture(|typed| {
        typed.zones[0].multiplier = 2;
        typed.zones[0].list_multiplier = 3;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("multiplied binding");
    let mut state = zone_state_for_temp_independent_load(0.0);

    let output = couple(&binding, &cache, &mut state, 0).expect("multiplied coupling");

    assert_eq!(binding.zone_multiplier, 2);
    assert_eq!(binding.zone_list_multiplier, 3);
    assert_eq!(output.coupling.feedback.multiplier_product, 6.0);
    assert_eq!(
        output.coupling.feedback.zone_supply_mass_flow_rate_kg_per_s,
        output
            .coupling
            .feedback
            .multiplied_supply_mass_flow_rate_kg_per_s
            / 6.0
    );
}
