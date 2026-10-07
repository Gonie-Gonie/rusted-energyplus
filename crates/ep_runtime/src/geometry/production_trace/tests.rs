use super::*;
use crate::psychrometrics::production_trace as execution;

fn supplied_height(surface: u32, operand: GeometryOperand) {
    record(
        GeometryConsumer::OutdoorAirTemperature,
        SurfaceId(surface),
        ZoneId(3),
        operand,
    );
}

#[test]
fn exact_supplied_bits_context_identity_and_ordered_prefix_are_retained() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let operand = GeometryOperand::CentroidHeight {
        centroid_m: [-0.0, nan, 7.0],
        height_m: 11.0,
    };
    let (((), geometry), _) = execution::capture(true, || {
        capture_with_limits(10, 2, || {
            {
                let _zone = execution::zone_step(4, 1, 4, 900.0);
                supplied_height(8, operand);
                supplied_height(8, operand);
            }
            {
                let _zone = execution::zone_step(4, 2, 4, 900.0);
                supplied_height(8, operand);
                // A third dictionary identity exhausts the bound. A later
                // repetition must stay omitted rather than create a gap.
                supplied_height(9, operand);
                supplied_height(8, operand);
            }
        })
    });
    let trace = geometry.expect("geometry capture");
    assert_eq!(trace.ordered_ids, [0, 0, 1]);
    assert_eq!(trace.total_call_count, 5);
    assert_eq!(trace.retained_call_count, 3);
    assert_eq!(trace.omitted_call_count, 2);
    assert_eq!(trace.truncation_reason, Some("unique_tuple_limit"));
    assert_eq!(trace.dictionary.len(), 2);
    assert_eq!(trace.dictionary[0].sequence, 1);
    assert_eq!(trace.dictionary[1].sequence, 3);
    assert_eq!(
        trace.consumer_counts[&GeometryConsumer::OutdoorAirTemperature],
        5
    );
    let row = &trace.dictionary[0];
    assert_eq!(row.surface_id, SurfaceId(8));
    assert_eq!(row.zone_id, ZoneId(3));
    assert_eq!(
        row.operand_bits,
        GeometryOperandBits::CentroidHeight([
            (-0.0_f64).to_bits(),
            nan.to_bits(),
            7.0_f64.to_bits(),
            11.0_f64.to_bits(),
        ])
    );
    assert!(matches!(
        row.operand,
        GeometryOperand::CentroidHeight { .. }
    ));
    if let GeometryOperand::CentroidHeight {
        centroid_m,
        height_m,
    } = row.operand
    {
        assert_eq!(centroid_m[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(centroid_m[1].to_bits(), nan.to_bits());
        assert_eq!(height_m.to_bits(), 11.0_f64.to_bits());
    }
    let first = row
        .context
        .expect("actual zone scope")
        .zone_timestep
        .expect("zone");
    let second = trace.dictionary[1]
        .context
        .expect("actual zone scope")
        .zone_timestep
        .expect("zone");
    assert_eq!(first.hour_index, 4);
    assert_eq!(first.zone_timestep, 1);
    assert_eq!(second.zone_timestep, 2);
    assert_eq!(first.timestep_seconds_bits, 900.0_f64.to_bits());
}

#[test]
fn disabled_and_nested_capture_preserve_execution_and_predecessor() {
    let operand = GeometryOperand::Area { area_m2: -0.0 };
    let (value, disabled) = capture(false, || {
        record(
            GeometryConsumer::SurfaceHeatTransfer,
            SurfaceId(1),
            ZoneId(3),
            operand,
        );
        (-0.0_f64).to_bits()
    });
    assert_eq!(value, (-0.0_f64).to_bits());
    assert!(disabled.is_none());
    assert!(!ACTIVE_GEOMETRY.with_borrow(Option::is_some));
    let ((value, inner), outer) = capture(true, || {
        record(
            GeometryConsumer::SurfaceHeatTransfer,
            SurfaceId(1),
            ZoneId(3),
            operand,
        );
        let inner = capture(true, || {
            record(
                GeometryConsumer::SurfaceHeatTransfer,
                SurfaceId(2),
                ZoneId(3),
                operand,
            );
            17
        });
        record(
            GeometryConsumer::SurfaceHeatTransfer,
            SurfaceId(1),
            ZoneId(3),
            operand,
        );
        inner
    });
    assert_eq!(value, 17);
    let outer = outer.expect("outer capture");
    assert_eq!(outer.total_call_count, 2);
    assert!(
        outer
            .dictionary
            .iter()
            .all(|row| row.surface_id == SurfaceId(1))
    );
    let inner = inner.expect("inner capture");
    assert_eq!(inner.total_call_count, 1);
    assert_eq!(inner.dictionary[0].surface_id, SurfaceId(2));
    assert!(!ACTIVE_GEOMETRY.with_borrow(Option::is_some));
}

#[test]
#[allow(clippy::panic)] // Deliberate unwind verifies observer-only predecessor restoration.
fn suspension_restores_predecessor_after_nested_scopes_and_unwinding() {
    let operand = GeometryOperand::Area { area_m2: 3.0 };
    let (_, trace) = capture(true, || {
        record(
            GeometryConsumer::SurfaceHeatTransfer,
            SurfaceId(1),
            ZoneId(2),
            operand,
        );
        let result = std::panic::catch_unwind(|| {
            let _outer = suspend_observations();
            record(
                GeometryConsumer::SurfaceHeatTransfer,
                SurfaceId(1),
                ZoneId(2),
                operand,
            );
            {
                let _inner = suspend_observations();
                record(
                    GeometryConsumer::SurfaceHeatTransfer,
                    SurfaceId(1),
                    ZoneId(2),
                    operand,
                );
            }
            panic!("intentional observer suspension unwind");
        });
        assert!(result.is_err());
        record(
            GeometryConsumer::SurfaceHeatTransfer,
            SurfaceId(1),
            ZoneId(2),
            operand,
        );
    });
    let trace = trace.expect("outer capture restores");
    assert_eq!(trace.total_call_count, 2);
    assert_eq!(trace.ordered_ids.len(), 2);
    assert_eq!(trace.omitted_call_count, 0);
}
