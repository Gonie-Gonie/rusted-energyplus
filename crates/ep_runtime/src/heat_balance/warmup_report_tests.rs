use super::*;

fn owner(names: &[&str], detailed: bool) -> Result<WarmupReportState, WarmupReportError> {
    WarmupReportState::new(
        names.iter().map(|s| (*s).to_owned()).collect(),
        1,
        WarmupReportConfiguration {
            doing_sizing: false,
            do_pure_load_calc: false,
            run_period_environment: true,
            environment_name: "INPUT ENVIRONMENT".into(),
            detailed,
        },
    )
}

fn clock(day: u32, time_step: u32) -> WarmupRecordClock {
    WarmupRecordClock {
        day,
        hour: 1,
        time_step,
    }
}

#[test]
fn allocation_availability_and_zero_zone_report() -> Result<(), WarmupReportError> {
    let mut report = owner(&[], true)?;
    assert_eq!(report.count, 0);
    assert_eq!(report.temperature.len(), 24);
    assert!(report.temperature.iter().all(Vec::is_empty));
    assert_eq!(report.temperature_stddev, vec![None; 24]);
    let mut eio = Vec::new();
    report.report(true, &[], &mut eio)?;
    assert_eq!(report.temperature_stddev, vec![None; 24]);
    report.report(false, &[], &mut eio)?;
    assert_eq!(report.temperature_stddev, vec![Some(0.0); 24]);
    assert_eq!(report.load_stddev, vec![Some(0.0); 24]);
    assert!(report.first_summary_write && report.first_detailed_write && eio.is_empty());
    Ok(())
}

#[test]
fn actual_record_count_tail_last_zone_scratch_and_repeated_report() -> Result<(), WarmupReportError>
{
    let mut report = owner(&["A", "B"], true)?;
    let mut zones = [
        HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0)),
        HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(1)),
    ];
    let temperatures = [[0.0, 0.0], [2.0, 3.0], [6.0, 12.0], [14.0, 30.0]];
    let loads = [[0.0, 0.0], [2.0, 4.0], [4.0, 8.0], [8.0, 16.0]];
    let mut eio = Vec::new();
    for (step, (temperature, load)) in temperatures.iter().zip(loads).enumerate() {
        for (index, zone) in zones.iter_mut().enumerate() {
            zone.record(temperature[index], load[index], 0.0);
            report.record_zone(index, zone, step < 2, clock(1, 1), &mut eio)?;
        }
    }
    assert_eq!(report.count, 2);
    assert_eq!(&report.temperature[..2], &[vec![2.0, 3.0], vec![4.0, 9.0]]);
    assert!(
        report.temperature[2..]
            .iter()
            .flatten()
            .all(|v| v.to_bits() == 0)
    );
    assert!(report.temperature_stddev.iter().all(Option::is_none));
    report.report(false, &zones, &mut eio)?;
    assert_eq!(report.load[0], vec![0.5, 0.5]);
    assert_eq!(report.load[1], vec![0.25, 0.25]);
    assert_eq!(report.last_local_summaries[0].mean_temperature, 3.0);
    assert_eq!(report.last_local_summaries[0].stddev_temperature, 1.0);
    assert_eq!(report.last_local_summaries[1].mean_temperature, 6.0);
    assert_eq!(report.last_local_summaries[1].stddev_temperature, 3.0);
    assert_eq!(&report.temperature_stddev[..2], &[Some(9.0), Some(9.0)]);
    assert!(
        report.temperature_stddev[2..]
            .iter()
            .all(|v| *v == Some(0.0))
    );
    report.report(false, &zones, &mut eio)?;
    assert_eq!(report.load[0], vec![0.125, 0.0625]);
    assert_eq!(report.load[1], vec![0.03125, 0.015625]);
    let text =
        String::from_utf8(eio).map_err(|_| WarmupReportError::Unavailable("test output UTF-8"))?;
    assert_eq!(text.matches(DETAILED_HEADER).count(), 1);
    assert_eq!(text.matches(SUMMARY_HEADER).count(), 1);
    assert_eq!(text.lines().count(), 10);
    Ok(())
}

#[test]
fn actual_sizing_day_and_load_threshold_guards() -> Result<(), WarmupReportError> {
    let mut report = owner(&["A"], false)?;
    let zone = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0));
    let mut eio = Vec::new();
    report.record_zone(0, &zone, false, clock(2, 1), &mut eio)?;
    report.configuration.doing_sizing = true;
    report.record_zone(0, &zone, false, clock(1, 1), &mut eio)?;
    assert_eq!(report.count, 0);
    report.configuration.do_pure_load_calc = true;
    report.record_zone(0, &zone, false, clock(1, 1), &mut eio)?;
    report.record_zone(0, &zone, false, clock(1, 1), &mut eio)?;
    report.load[0][0] = 7.0;
    report.maximum_load[0][0] = 1.0e-4;
    report.load[1][0] = 1.0;
    report.maximum_load[1][0] = f64::from_bits(1.0e-4_f64.to_bits() + 1);
    report.report(false, &[zone], &mut eio)?;
    assert_eq!(report.load[0][0].to_bits(), 0);
    assert_eq!(
        report.load[1][0].to_bits(),
        (1.0 / report.maximum_load[1][0]).to_bits()
    );
    assert!(report.first_detailed_write);
    Ok(())
}

struct FailingWriter {
    bytes: Vec<u8>,
    remaining: usize,
}
impl Write for FailingWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::other("actual test sink stop"));
        }
        let length = self.remaining.min(bytes.len());
        self.bytes.extend_from_slice(&bytes[..length]);
        self.remaining -= length;
        Ok(length)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn detailed_sink_failure_retains_current_zone_writes_prefix_and_error_source()
-> Result<(), WarmupReportError> {
    let mut report = owner(&["A", "B"], true)?;
    let mut zone = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0));
    zone.record(23.0, 8.0, 0.0);
    let mut sink = FailingWriter {
        bytes: Vec::new(),
        remaining: 11,
    };
    let error = match report.record_zone(0, &zone, false, clock(1, 1), &mut sink) {
        Ok(()) => {
            return Err(WarmupReportError::Unavailable(
                "test sink unexpectedly returned",
            ));
        }
        Err(error) => error,
    };
    assert!(matches!(&error, WarmupReportError::Io(_)));
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(sink.bytes, &DETAILED_HEADER.as_bytes()[..11]);
    assert_eq!(report.count, 1);
    assert_eq!(report.maximum_load[0], vec![8.0, 0.0]);
    assert!(report.first_detailed_write);
    Ok(())
}

#[test]
fn report_sink_failure_retains_normalization_and_complete_header() -> Result<(), WarmupReportError>
{
    let mut report = owner(&["A"], false)?;
    report.count = 1;
    report.load[0][0] = 2.0;
    report.maximum_load[0][0] = 8.0;
    let zone = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0));
    let mut sink = FailingWriter {
        bytes: Vec::new(),
        remaining: SUMMARY_HEADER.len(),
    };
    assert!(matches!(
        report.report(false, &[zone], &mut sink),
        Err(WarmupReportError::Io(_))
    ));
    assert_eq!(sink.bytes, SUMMARY_HEADER.as_bytes());
    assert!(!report.first_summary_write);
    assert_eq!(report.load[0][0], 0.25);
    assert_eq!(report.last_local_summaries.len(), 1);
    assert_eq!(report.temperature_stddev, vec![Some(0.0); 24]);
    Ok(())
}
