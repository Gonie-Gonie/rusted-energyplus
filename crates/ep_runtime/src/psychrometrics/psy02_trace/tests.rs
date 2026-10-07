use super::*;
use crate::psychrometrics::{
    energyplus_psy_tsat_fn_h_pb, energyplus_psy_tsat_fn_pb_raw, energyplus_psy_twb_fn_tdb_w_pb,
    energyplus_psy_w_fn_tdb_h, with_fresh_psychrometric_state,
};

#[test]
fn actual_external_raw_call_is_retained_for_saved_pair_and_nested_cache_replay() {
    with_fresh_psychrometric_state(|| {
        let (_, trace) = capture(true, || energyplus_psy_tsat_fn_pb_raw(101325.0));
        let trace = trace.unwrap();
        assert_eq!(trace.total_root_count, 1);
        assert_eq!(trace.dictionary[0].routine, "PsyTsatFnPb_raw");
        assert!(trace.dictionary[0].operation.cache_before.is_none());
        assert_eq!(trace.final_state.press_save, 101325.0);
        assert!(!trace.final_caches.psat.is_empty());
        assert!(trace.final_caches.tsat_pb.is_empty());
    });
}

#[test]
fn top_level_inverse_records_one_root_with_all_nested_mutations() {
    with_fresh_psychrometric_state(|| {
        let (result, trace) = capture(true, || {
            energyplus_psy_twb_fn_tdb_w_pb(30.0, 0.008, 101325.0)
        });
        let trace = trace.unwrap();
        assert_eq!(trace.total_root_count, 1);
        assert_eq!(trace.ordered_ids, [0]);
        assert_eq!(trace.dictionary[0].routine, "PsyTwbFnTdbWPb");
        assert_eq!(
            trace.dictionary[0].operation.result.to_bits(),
            result.to_bits()
        );
        assert!(!trace.final_caches.psat.is_empty());
        assert!(!trace.final_caches.tsat_pb.is_empty());
        assert_eq!(trace.omitted_root_count, 0);
    });
}

#[test]
fn nested_humidity_inverse_never_reborrows_the_active_owner() {
    with_fresh_psychrometric_state(|| {
        let (_, trace) = capture(true, || {
            let _ = energyplus_psy_tsat_fn_h_pb(50000.0, 101325.0);
            let _ = energyplus_psy_w_fn_tdb_h(22.0, 50000.0);
        });
        let trace = trace.unwrap();
        assert_eq!(trace.total_root_count, 2);
        assert_eq!(trace.dictionary[0].routine, "PsyTsatFnHPb");
        assert_eq!(trace.dictionary[1].routine, "PsyWFnTdbH");
        assert!(trace.dictionary[1].operation.cache_before.is_none());
    });
}

#[test]
fn complete_exact_repeat_order_and_contiguous_truncation_are_explicit() {
    with_fresh_psychrometric_state(|| {
        let (_, trace) = capture_with_limits(3, 10, || {
            for _ in 0..5 {
                let _ = energyplus_psy_w_fn_tdb_h(22.0, 50000.0);
            }
        });
        let trace = trace.unwrap();
        assert_eq!(trace.total_root_count, 5);
        assert_eq!(trace.omitted_root_count, 2);
        assert_eq!(trace.dictionary.len(), 1);
        assert_eq!(trace.ordered_ids, [0, 0, 0]);
        assert_eq!(trace.truncation_reason, Some("event_limit"));
        assert_eq!(trace.routine_counts["PsyWFnTdbH"], 5);
    });
}

#[test]
#[allow(clippy::panic)] // Test-only deliberate panic checks nested restoration.
fn disabled_and_nested_unwinding_capture_preserve_predecessor() {
    with_fresh_psychrometric_state(|| {
        let (_, trace) = capture(true, || {
            assert!(
                capture(false, || energyplus_psy_w_fn_tdb_h(22.0, 50000.0))
                    .1
                    .is_none()
            );
            assert!(
                std::panic::catch_unwind(|| capture(true, || {
                    let _ = energyplus_psy_w_fn_tdb_h(23.0, 50000.0);
                    panic!("intentional trace unwind");
                }))
                .is_err()
            );
            let _ = energyplus_psy_w_fn_tdb_h(24.0, 50000.0);
        });
        let trace = trace.unwrap();
        assert_eq!(trace.total_root_count, 2);
        assert_eq!(trace.dictionary[0].input_bits[0], 22.0_f64.to_bits());
        assert_eq!(trace.dictionary[1].input_bits[0], 24.0_f64.to_bits());
    });
}
