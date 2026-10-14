use super::*;
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking,
};
use std::io::{self, Write};

fn state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "authored caller".into(),
        idd_version_string: "authored input".into(),
        doing_input_processing: false,
        print_console_output: true,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: true,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            match_counts: [0; 20],
            abort_processing: false,
            total_severe_errors: 0,
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}
fn owner() -> Result<HeatBalanceWarmupConvergenceState, RuntimeError> {
    let mut a = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(0));
    let mut b = HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(1));
    a.record(23.0, 200.0, 0.0);
    b.record(10.0, 0.0, 0.0);
    Ok(HeatBalanceWarmupConvergenceState {
        zones: vec![a, b],
        checked_day: 0,
        convergence_checks_failed: false,
        warmup_flag: true,
        maximum_day_forced_exit: false,
        unavailable_reason: None,
        report: Some(WarmupReportState::new(
            vec!["A".into(), "B".into()],
            1,
            WarmupReportConfiguration {
                doing_sizing: false,
                do_pure_load_calc: false,
                run_period_environment: true,
                environment_name: "ACTUAL INPUT ENV".into(),
                detailed: false,
            },
        )?),
    })
}
fn current() -> ProductionWarmupReportContext {
    ProductionWarmupReportContext {
        phase: WeatherDayPhase::Warmup { day: 1 },
        begin_environment: false,
        warmup_flag: true,
        end_day: true,
        day: 1,
        hour: 24,
        time_step: 1,
        time_steps_in_hour: 1,
        run_period_environment: true,
        environment_name: "ACTUAL INPUT ENV".into(),
    }
}
fn options() -> HeatBalanceWarmupOptions {
    HeatBalanceWarmupOptions {
        enabled: true,
        minimum_days: 1,
        maximum_days: 1,
        temperature_convergence_tolerance_delta_c: 0.0,
        loads_convergence_tolerance_w: 0.0,
    }
}

#[test]
fn maximum_day_diagnostics_once_flag_zone_order_and_reset_before_global_severe()
-> Result<(), Box<dyn std::error::Error>> {
    let mut owner = owner()?;
    let (mut reports, mut state, mut eio, mut error, mut console) = (
        CtfInitializationReports::default(),
        state(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    {
        let mut output = CtfInitializationContext {
            reports: &mut reports,
            eio: &mut eio,
            backend: SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        };
        check_day_with_context(&mut owner, 1, options(), &current(), &mut output)?;
    }
    let text = std::str::from_utf8(&error)?;
    let a = text.find("Zone=\"A\"").ok_or("missing A severe")?;
    let b = text.find("Zone=\"B\"").ok_or("missing B severe")?;
    let final_severe = text
        .find("User supplied maximum")
        .ok_or("missing global severe")?;
    assert!(a < b && b < final_severe);
    assert_eq!(
        text.matches("See Warmup Convergence Information").count(),
        1
    );
    assert_eq!(state.tracking.total_severe_errors, 3);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 3);
    assert!(
        owner
            .report
            .as_ref()
            .ok_or("missing report")?
            .warmup_convergence_warning
    );
    assert!(!owner.warmup_flag && owner.maximum_day_forced_exit);
    assert_eq!(owner.zones[0].previous, [200.0, 0.0, 23.0, 23.0]);
    assert_eq!(owner.zones[1].current, [-9999.0, -9999.0, -9999.0, 1000.0]);
    Ok(())
}

struct Stop {
    bytes: Vec<u8>,
    remaining: usize,
}
impl Write for Stop {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.remaining == 0 {
            return Err(io::Error::other("authored actual diagnostic sink stop"));
        }
        let n = self.remaining.min(bytes.len());
        self.bytes.extend_from_slice(&bytes[..n]);
        self.remaining -= n;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn reached_diagnostic_sink_failure_stops_before_zone_reset_and_preserves_cause()
-> Result<(), Box<dyn std::error::Error>> {
    let mut owner = owner()?;
    let (mut reports, mut state, mut eio, mut console) = (
        CtfInitializationReports::default(),
        state(),
        Vec::new(),
        Vec::new(),
    );
    let mut sink = Stop {
        bytes: Vec::new(),
        remaining: 9,
    };
    let result = {
        let mut output = CtfInitializationContext {
            reports: &mut reports,
            eio: &mut eio,
            backend: SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut sink),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        };
        check_day_with_context(&mut owner, 1, options(), &current(), &mut output)
    };
    let error = match result {
        Ok(()) => return Err("actual sink unexpectedly succeeded".into()),
        Err(error) => error,
    };
    assert!(matches!(&error, RuntimeError::WarmupOutput(_)));
    let first = std::error::Error::source(&error).ok_or("missing warmup cause")?;
    let backend = first.source().ok_or("missing backend cause")?;
    assert!(
        backend
            .source()
            .and_then(|source| source.downcast_ref::<io::Error>())
            .is_some()
    );
    assert_eq!(sink.bytes.len(), 9);
    assert_eq!(state.tracking.total_severe_errors, 1);
    assert_eq!(owner.zones[0].current[0], 200.0);
    assert_eq!(owner.zones[0].previous[2], 0.0);
    assert_eq!(owner.zones[1].pass_flags, [2; 4]);
    assert_eq!(owner.checked_day, 0);
    assert!(owner.warmup_flag);
    Ok(())
}

#[test]
fn begin_environment_preserves_header_warning_and_scratch_owners()
-> Result<(), Box<dyn std::error::Error>> {
    let mut owner = owner()?;
    let report = owner.report.as_mut().ok_or("missing report")?;
    report.first_detailed_write = false;
    report.first_summary_write = false;
    report.warmup_convergence_warning = true;
    report.count = 2;
    report.temperature[0][0] = 7.0;
    report.temperature_stddev[0] = Some(3.0);
    report.begin_environment()?;
    assert_eq!(report.count, 0);
    assert_eq!(report.temperature[0][0].to_bits(), 0);
    assert_eq!(report.temperature_stddev[0], Some(3.0));
    assert!(
        !report.first_detailed_write
            && !report.first_summary_write
            && report.warmup_convergence_warning
    );
    Ok(())
}
