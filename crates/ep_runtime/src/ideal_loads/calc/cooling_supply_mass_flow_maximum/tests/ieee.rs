use super::{Operand, release_case, run};

#[test]
fn strict_less_tree_preserves_every_unique_greatest_operand() {
    for (label, outdoor_air, cool, dehumidification, humidification, winner, expected) in [
        ("outdoor air", 4.0, 1.0, 2.0, 3.0, Operand::OutdoorAir, 4.0),
        ("cooling", 1.0, 4.0, 2.0, 3.0, Operand::Cooling, 4.0),
        (
            "dehumidification",
            1.0,
            2.0,
            4.0,
            3.0,
            Operand::Dehumidification,
            4.0,
        ),
        (
            "humidification",
            1.0,
            2.0,
            3.0,
            4.0,
            Operand::Humidification,
            4.0,
        ),
    ] {
        assert_case(
            label,
            outdoor_air,
            cool,
            dehumidification,
            humidification,
            winner,
            expected,
        );
    }
}

#[test]
fn unordered_inputs_follow_strict_less_left_wins_at_each_tree_node() {
    for (label, outdoor_air, cool, dehumidification, humidification, winner, expected) in [
        (
            "outdoor air NaN",
            f64::NAN,
            -1.0,
            -2.0,
            -3.0,
            Operand::PositiveZeroFloor,
            0.0,
        ),
        (
            "cooling NaN",
            -1.0,
            f64::NAN,
            5.0,
            -2.0,
            Operand::PositiveZeroFloor,
            0.0,
        ),
        (
            "dehumidification NaN",
            -1.0,
            5.0,
            f64::NAN,
            -2.0,
            Operand::Cooling,
            5.0,
        ),
        (
            "humidification NaN",
            -1.0,
            5.0,
            -2.0,
            f64::NAN,
            Operand::Cooling,
            5.0,
        ),
        (
            "all dynamic operands NaN",
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            Operand::PositiveZeroFloor,
            0.0,
        ),
    ] {
        assert_case(
            label,
            outdoor_air,
            cool,
            dehumidification,
            humidification,
            winner,
            expected,
        );
    }
}

#[test]
fn infinities_and_ties_retain_the_source_earlier_tree_winner() {
    for (label, outdoor_air, cool, dehumidification, humidification, winner) in [
        (
            "outdoor air infinity",
            f64::INFINITY,
            1.0,
            2.0,
            3.0,
            Operand::OutdoorAir,
        ),
        (
            "cooling infinity",
            1.0,
            f64::INFINITY,
            2.0,
            3.0,
            Operand::Cooling,
        ),
        (
            "dehumidification infinity",
            1.0,
            2.0,
            f64::INFINITY,
            3.0,
            Operand::Dehumidification,
        ),
        (
            "humidification infinity",
            1.0,
            2.0,
            3.0,
            f64::INFINITY,
            Operand::Humidification,
        ),
        (
            "outdoor air and cooling infinity tie",
            f64::INFINITY,
            f64::INFINITY,
            1.0,
            2.0,
            Operand::OutdoorAir,
        ),
        (
            "cooling and dehumidification infinity tie",
            0.0,
            f64::INFINITY,
            f64::INFINITY,
            2.0,
            Operand::Cooling,
        ),
        (
            "leading and humidification infinity tie",
            f64::INFINITY,
            1.0,
            2.0,
            f64::INFINITY,
            Operand::OutdoorAir,
        ),
    ] {
        assert_case(
            label,
            outdoor_air,
            cool,
            dehumidification,
            humidification,
            winner,
            f64::INFINITY,
        );
    }
}

#[test]
fn positive_zero_floor_and_positive_ties_preserve_source_shape() {
    for (label, outdoor_air, cool, dehumidification, humidification, winner, expected) in [
        (
            "all negative infinity",
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
            Operand::PositiveZeroFloor,
            0.0,
        ),
        (
            "negative and signed zero",
            -1.0,
            -0.0,
            -2.0,
            -0.0,
            Operand::PositiveZeroFloor,
            0.0,
        ),
        (
            "all positive tie",
            5.0,
            5.0,
            5.0,
            5.0,
            Operand::OutdoorAir,
            5.0,
        ),
        (
            "cooling dehumidification positive tie",
            1.0,
            5.0,
            5.0,
            2.0,
            Operand::Cooling,
            5.0,
        ),
    ] {
        assert_case(
            label,
            outdoor_air,
            cool,
            dehumidification,
            humidification,
            winner,
            expected,
        );
    }
}

fn assert_case(
    label: &str,
    outdoor_air: f64,
    cool: f64,
    dehumidification: f64,
    humidification: f64,
    expected_winner: Operand,
    expected_value: f64,
) {
    let (_, _, mut predecessor) = release_case(-1_000.0);
    predecessor.resulting_supply_mass_flow_rate_for_cool_kg_per_s = Some(cool);
    predecessor.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s =
        Some(dehumidification);
    predecessor.resulting_supply_mass_flow_rate_for_humidification_kg_per_s = Some(humidification);
    let snapshot = run(predecessor, outdoor_air);
    assert_eq!(snapshot.final_winner, Some(expected_winner), "{label}");
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_kg_per_s
            .expect("tree result")
            .to_bits(),
        expected_value.to_bits(),
        "{label}"
    );
}
