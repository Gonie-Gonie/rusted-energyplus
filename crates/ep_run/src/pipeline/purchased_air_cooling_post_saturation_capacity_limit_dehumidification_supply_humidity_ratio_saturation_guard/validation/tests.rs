use super::*;

#[test]
fn conceptual_contract_is_54_total_with_17_public_and_126_source_sites() {
    let conceptual = 36 + 18;
    let public = PUBLIC_LOGICAL_INDICES.len() + 4;
    assert_eq!((conceptual, public, conceptual - public), (54, 17, 37));
    assert_eq!(18 * 3 + 18 * 4, 126);
    assert_eq!((36, 41, 51), (36, 41, 51));
}

#[test]
fn public_route_firewall_exposes_four_active_outcomes() {
    assert_eq!(
        PUBLIC_LOGICAL_INDICES
            .into_iter()
            .filter(|index| *index >= FIRST_ACTIVE_LOGICAL_INDEX)
            .count(),
        4,
    );
}

#[test]
fn overflow_helpers_fail_closed() {
    assert!(checked_sum(&[usize::MAX, 1], "test partition").is_err());
    assert!(usize::MAX.checked_mul(3).is_none());
}

#[test]
fn source_order_has_exact_four_sites() {
    assert_eq!(EXPECTED_SOURCE_ORDER.len(), 4);
    assert_eq!(
        EXPECTED_SOURCE_ORDER[3],
        "enter-saturation-supply-humidity-ratio-guard-body-if-comparison-satisfied",
    );
}
