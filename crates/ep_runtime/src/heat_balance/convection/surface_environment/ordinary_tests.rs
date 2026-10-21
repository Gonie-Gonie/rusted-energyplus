use super::*;
use crate::heat_balance::error_backend::{SourceErrorLevel, SourceErrorState, SourceErrorTracking};
use std::io::{self, Write};

fn state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: false,
        version_string: "authored weather test".into(),
        idd_version_string: "authored IDD".into(),
        doing_input_processing: false,
        print_console_output: false,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: true,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
            match_counts: [0; 20],
            abort_processing: false,
            total_severe_errors: 7,
            total_severe_errors_during_warmup: 4,
            total_severe_errors_during_sizing: 2,
            last_severe_error: "incoming severe".into(),
            total_warning_errors: 3,
            total_warning_errors_during_warmup: 2,
            total_warning_errors_during_sizing: 1,
        },
    }
}
fn backend<'a>(
    state: &'a mut SourceErrorState,
    error: &'a mut dyn Write,
    console: &'a mut dyn Write,
) -> SourceErrorBackend<'a> {
    SourceErrorBackend {
        state,
        error_stream: Some(error),
        console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    }
}
fn row(height: f64, dry: f64, wet: f64) -> SurfaceOutdoorEnvironmentRow {
    SurfaceOutdoorEnvironmentRow {
        centroid_height_m: height,
        wind_exposed: true,
        dry_bulb_c: dry,
        wet_bulb_c: wet,
        wind_speed_m_per_s: -0.0,
        wind_direction_deg: f64::from_bits(0x7ff8000000000123),
    }
}

#[test]
fn three_operand_minimum_retains_objexx_nan_and_tie_routes() {
    let nan = f64::from_bits(0x7ff8000000000345);
    assert_eq!(minimum_three(0.0, -101.0, nan).to_bits(), nan.to_bits());
    assert_eq!(minimum_three(0.0, nan, -101.0), -101.0);
    assert_eq!(
        minimum_three(0.0, 0.0, -0.0).to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(minimum_three(0.0, -0.0, 0.0).to_bits(), 0.0_f64.to_bits());

    use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
    let mut zones = [SelectedZoneData {
        name: "actual Zone owner".into(),
        centroid_m: Some([0.0, 0.0, 10.0]),
        ..Default::default()
    }];
    let mut parameters = SurfaceOutdoorEnvironmentParameters {
        site_temperature_gradient_k_per_m: 0.0,
        weather_file_temperature_modifier_c: 123.0,
        site_wind_exponent: 0.0,
        site_wind_boundary_layer_height_m: 77.0,
        weather_file_wind_modifier: 19.0,
    };
    let weather = SurfaceOutdoorWeather {
        dry_bulb_c: 21.0,
        wet_bulb_c: 13.0,
        wind_speed_m_per_s: 4.0,
        wind_direction_deg: -0.0,
    };
    let (mut error_state, mut error, mut console) = (state(), Vec::new(), Vec::new());
    assert!(
        source::update_zones(
            &mut zones,
            parameters,
            weather,
            &mut backend(&mut error_state, &mut error, &mut console)
        )
        .is_ok()
    );
    assert_eq!(
        [zones[0].dry_bulb, zones[0].wet_bulb, zones[0].wind_speed],
        [21.0, 13.0, 4.0]
    );
    assert_eq!(zones[0].wind_direction.to_bits(), (-0.0_f64).to_bits());
    parameters.site_temperature_gradient_k_per_m = 0.0065;
    parameters.weather_file_temperature_modifier_c = 0.75;
    parameters.site_wind_exponent = 0.22;
    parameters.weather_file_wind_modifier = 1.25;
    assert!(
        source::update_zones(
            &mut zones,
            parameters,
            weather,
            &mut backend(&mut error_state, &mut error, &mut console)
        )
        .is_ok()
    );
    assert_eq!(
        zones[0].dry_bulb.to_bits(),
        (21.0_f64 + 0.75 - 0.0065 * 6_356_000.0 * 10.0 / (6_356_000.0 + 10.0)).to_bits()
    );
    assert_eq!(
        zones[0].wind_speed.to_bits(),
        (4.0_f64 * 1.25 * 77.0_f64.powf(-0.22) * 10.0_f64.powf(0.22)).to_bits()
    );
    zones[0].centroid_m = Some([0.0, 0.0, -1.0]);
    assert!(
        source::update_zones(
            &mut zones,
            parameters,
            weather,
            &mut backend(&mut error_state, &mut error, &mut console)
        )
        .is_ok()
    );
    assert_eq!(zones[0].dry_bulb, 21.75);
    assert_eq!(zones[0].wet_bulb, 13.75);
    assert_eq!(zones[0].wind_speed.to_bits(), 0.0_f64.to_bits());
    zones[0].centroid_m = None;
    let before = zones.clone();
    assert!(matches!(
        source::update_zones(
            &mut zones,
            parameters,
            weather,
            &mut backend(&mut error_state, &mut error, &mut console)
        ),
        Err(SurfaceOutdoorEnvironmentError::Unavailable(_))
    ));
    assert_eq!(zones, before);
    assert!(error.is_empty());
}

#[test]
fn guard_uses_first_reached_cold_surface_and_actual_severity_state()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut state, mut error, mut console) = (state(), Vec::new(), Vec::new());
    let rows = [
        row(3.0, -100.0, -100.0),
        row(20000.0, -101.0, -99.0),
        row(50000.0, -999.0, -999.0),
    ];
    let names = ["threshold", "actual cold name", "must not be visited"];
    let mut calls = Vec::new();
    let mut callback = |_: &mut SourceErrorState, level: SourceErrorLevel, text: &[u8]| {
        let text = std::str::from_utf8(text).expect("existing cold-surface UTF-8 diagnostic");
        calls.push((level, text.to_owned()));
    };
    let result = {
        let mut backend = backend(&mut state, &mut error, &mut console);
        backend.error_callback = Some(&mut callback);
        check_surface_out_bulb_temperatures(rows.iter().zip(names), &mut backend)
    };
    assert!(matches!(
        result,
        Err(SurfaceOutdoorEnvironmentError::Fatal(
            SourceFatalStop::Fatal(_)
        ))
    ));
    assert_eq!(
        calls.iter().map(|(level, _)| *level).collect::<Vec<_>>(),
        [
            SourceErrorLevel::Severe,
            SourceErrorLevel::Continue,
            SourceErrorLevel::Continue,
            SourceErrorLevel::Continue,
            SourceErrorLevel::Fatal
        ]
    );
    assert_eq!(
        calls[1].1,
        "...check Surface Heights - Maximum Surface Height=[20000]."
    );
    assert_eq!(calls[3].1, "...look at Surface Name= actual cold name");
    assert_eq!(state.tracking.total_severe_errors, 8);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 5);
    assert_eq!(state.tracking.total_severe_errors_during_sizing, 2);
    assert!(!state.tracking.abort_processing);
    let text = std::str::from_utf8(&error)?;
    assert!(text.contains("Reference severe error count=8"));
    assert!(!text.contains("must not be visited"));

    use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
    let mut zones = [
        SelectedZoneData {
            name: "first cold Zone".into(),
            centroid_m: Some([0.0, 0.0, 20000.0]),
            wind_speed: -0.0,
            wind_direction: 321.0,
            ..Default::default()
        },
        SelectedZoneData {
            name: "later Zone".into(),
            centroid_m: Some([0.0, 0.0, 1.0]),
            wind_speed: 987.0,
            wind_direction: -0.0,
            ..Default::default()
        },
    ];
    let incoming = zones.clone();
    let parameters = SurfaceOutdoorEnvironmentParameters {
        site_temperature_gradient_k_per_m: 0.0,
        weather_file_temperature_modifier_c: 99.0,
        site_wind_exponent: 0.22,
        site_wind_boundary_layer_height_m: 370.0,
        weather_file_wind_modifier: 1.0,
    };
    let weather = SurfaceOutdoorWeather {
        dry_bulb_c: -101.0,
        wet_bulb_c: -99.0,
        wind_speed_m_per_s: 4.0,
        wind_direction_deg: 123.0,
    };
    let mut zone_error = Vec::new();
    let result = source::update_zones(
        &mut zones,
        parameters,
        weather,
        &mut backend(&mut state, &mut zone_error, &mut console),
    );
    assert!(matches!(
        result,
        Err(SurfaceOutdoorEnvironmentError::Fatal(_))
    ));
    for (zone, before) in zones.iter().zip(&incoming) {
        assert_eq!([zone.dry_bulb, zone.wet_bulb], [-101.0, -99.0]);
        assert_eq!(zone.wind_speed.to_bits(), before.wind_speed.to_bits());
        assert_eq!(
            zone.wind_direction.to_bits(),
            before.wind_direction.to_bits()
        );
    }
    let text = std::str::from_utf8(&zone_error)?;
    assert!(text.contains("SetOutBulbTempAt: Zone Outdoor Temperatures < -100 C"));
    assert!(text.contains("...check Zone Heights - Maximum Zone Height=[20000]."));
    assert!(text.contains("...look at Zone Name= first cold Zone"));
    assert!(!text.contains("later Zone"));
    assert_eq!(state.tracking.total_severe_errors, 9);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 6);
    Ok(())
}

#[test]
fn undefined_height_format_keeps_completed_severe_prefix_and_incoming_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut state, mut error, mut console) = (state(), Vec::new(), Vec::new());
    let rows = [row(f64::INFINITY, -101.0, -102.0)];
    let before = rows[0];
    let result = check_surface_out_bulb_temperatures(
        rows.iter().map(|row| (row, "infinite height")),
        &mut backend(&mut state, &mut error, &mut console),
    );
    assert!(matches!(
        result,
        Err(SurfaceOutdoorEnvironmentError::Format(_))
    ));
    assert_eq!(state.tracking.total_severe_errors, 8);
    assert_eq!(
        state.tracking.last_severe_error,
        "SetOutBulbTempAt: Surface Outdoor Temperatures < -100 C"
    );
    assert_eq!(
        rows[0].wind_speed_m_per_s.to_bits(),
        before.wind_speed_m_per_s.to_bits()
    );
    assert_eq!(
        rows[0].wind_direction_deg.to_bits(),
        before.wind_direction_deg.to_bits()
    );
    let text = std::str::from_utf8(&error)?;
    assert!(text.contains("** Severe  **"));
    assert!(!text.contains("**   ~~~   **"));
    assert!(!text.contains("**  Fatal  **"));
    Ok(())
}

#[test]
fn real_sink_failure_retains_counter_prefix_and_error_source()
-> Result<(), Box<dyn std::error::Error>> {
    struct FailingSink(Vec<u8>);
    impl Write for FailingSink {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.0.is_empty() {
                self.0.extend_from_slice(&bytes[..bytes.len().min(5)]);
                Ok(bytes.len().min(5))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "authored surface sink failure",
                ))
            }
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let (mut state, mut sink, mut console) = (state(), FailingSink(Vec::new()), Vec::new());
    let rows = [row(1.0, -101.0, -101.0)];
    let result = check_surface_out_bulb_temperatures(
        rows.iter().map(|row| (row, "sink")),
        &mut backend(&mut state, &mut sink, &mut console),
    );
    let error = result.err().ok_or("cold guard returned")?;
    assert!(matches!(error, SurfaceOutdoorEnvironmentError::Backend(_)));
    assert_eq!(state.tracking.total_severe_errors, 8);
    assert_eq!(state.tracking.last_severe_error, "incoming severe");
    assert!(!sink.0.is_empty());
    let mut source = std::error::Error::source(&error);
    let mut actual_io = None;
    while let Some(error) = source {
        if let Some(error) = error.downcast_ref::<io::Error>() {
            actual_io = Some(error);
            break;
        }
        source = error.source();
    }
    assert_eq!(
        actual_io.map(io::Error::kind),
        Some(io::ErrorKind::BrokenPipe)
    );
    Ok(())
}
