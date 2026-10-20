use super::*;
use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
use crate::psychrometrics::{
    EnergyPlusPsychrometricCacheSlot as Slot, EnergyPlusPsychrometricFunction as Function,
    EnergyPlusPsychrometricsState, with_fresh_psychrometric_state,
};
use std::cell::{Cell, RefCell};
use std::io::{self, Write};

fn error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: false,
        version_string: "declared property test".into(),
        idd_version_string: "declared IDD".into(),
        doing_input_processing: false,
        print_console_output: true,
        kickoff_simulation: false,
        developer_flag: false,
        // Deliberately distinct: read the actual supplied phase owners.
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
struct Caller {
    clock: WeatherGlobalState,
    globals: SelectedHeatBalanceGlobals,
    environment: WeatherEnvironmentState,
    hvac: SelectedHvacGlobals,
    recurring: SourceRecurringErrorState,
    errors: SourceErrorState,
    stream: Vec<u8>,
    console: Vec<u8>,
}
impl Caller {
    fn new() -> Self {
        let clock = WeatherGlobalState {
            current_time: 1.0,
            time_step_zone: 0.25,
            ..WeatherGlobalState::default()
        };
        let environment = WeatherEnvironmentState {
            environment_name: "DECLARED PROPERTY ENVIRONMENT".into(),
            current_month_day: "01/01".into(),
            ..WeatherEnvironmentState::default()
        };
        Self {
            clock,
            globals: SelectedHeatBalanceGlobals::default(),
            environment,
            hvac: SelectedHvacGlobals::default(),
            recurring: SourceRecurringErrorState::default(),
            errors: error_state(),
            stream: Vec::new(),
            console: Vec::new(),
        }
    }
    fn with_owners<R>(
        &mut self,
        execute: impl FnOnce(&mut SourcePsychrometricOwners<'_, '_>) -> R,
    ) -> R {
        let mut backend = SourceErrorBackend {
            state: &mut self.errors,
            error_stream: Some(&mut self.stream),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        execute(&mut SourcePsychrometricOwners {
            clock: &self.clock,
            globals: &self.globals,
            environment: &self.environment,
            hvac: &self.hvac,
            recurring: &mut self.recurring,
            backend: &mut backend,
        })
    }
}
fn psat_slot(temperature: f64) -> (i64, f64) {
    let index = ((temperature.to_bits() as i64) >> 28) as usize
        & (crate::psychrometrics::ENERGYPLUS_PSY_CACHE_SIZE - 1);
    with_state(|state| {
        state
            .final_caches()
            .psat
            .into_iter()
            .find_map(|slot| match slot {
                Slot::Psat {
                    index: found,
                    i_tdb,
                    value,
                } if found == index => Some((i_tdb, value)),
                _ => None,
            })
            .expect("same actual Psat cell")
    })
}

#[test]
fn source_dew_point_uses_exact_two_argument_domain_and_shared_cache_without_rh_floors() {
    with_fresh_psychrometric_state(|| {
        let mut caller = Caller::new();
        let mut access = SourcePsychrometricAccess;
        for temperature in [-80.0, 20.125, 90.0] {
            let mut reference = EnergyPlusPsychrometricsState::default();
            let dew = reference
                .evaluate(Function::Psat, &[temperature])
                .unwrap()
                .result;
            for pressure in [101_325.0, dew + 500.0] {
                let expected = reference
                    .evaluate(Function::WFromDewPoint, &[temperature, pressure])
                    .unwrap()
                    .result;
                let actual = caller
                    .with_owners(|owners| {
                        access.source_dew_point_humidity_ratio(
                            temperature,
                            pressure,
                            "Declared property caller",
                            owners,
                        )
                    })
                    .unwrap();
                assert_eq!(actual.to_bits(), expected.to_bits());
                assert_eq!(
                    actual.to_bits(),
                    (dew * 0.62198 / (pressure - dew)).to_bits()
                );
                if temperature == -80.0 && pressure == 101_325.0 {
                    assert!(actual < 1.0e-5);
                }
                if pressure == dew + 500.0 {
                    let rh = reference
                        .evaluate(Function::WFromRh, &[temperature, 1.0, pressure])
                        .unwrap()
                        .result;
                    assert_ne!(actual.to_bits(), rh.to_bits());
                }
            }
        }
        assert!(caller.stream.is_empty());
        assert_eq!(caller.recurring.number, 0);
        assert!(with_state(|state| state
            .source_error_indices
            .iter()
            .all(|index| *index == 0)));
        assert!(
            EnergyPlusPsychrometricsState::default()
                .evaluate(Function::WFromDewPoint, &[20.0, 1.0, 101_325.0])
                .is_err()
        );
    });
}

#[test]
fn source_dew_point_whole_degree_retries_keep_actual_warmup_and_recurring_owners() {
    with_fresh_psychrometric_state(|| {
        let mut caller = Caller::new();
        let mut access = SourcePsychrometricAccess;
        let mut reference = EnergyPlusPsychrometricsState::default();
        let expected = reference
            .evaluate(Function::WFromDewPoint, &[100.0, 80_000.0])
            .unwrap()
            .result;
        caller.clock.warmup_flag = true;
        let result = caller
            .with_owners(|owners| {
                access.source_dew_point_humidity_ratio(100.0, 80_000.0, "", owners)
            })
            .unwrap();
        assert_eq!(result.to_bits(), expected.to_bits());
        assert_eq!(
            with_state(|state| state.final_caches()),
            reference.final_caches()
        );
        assert!(caller.stream.is_empty());
        assert!(with_state(|state| state.source_string.is_empty()));
        caller.clock.warmup_flag = false;
        caller.globals.doing_sizing = true;
        for _ in 0..2 {
            let result = caller
                .with_owners(|owners| {
                    access.source_dew_point_humidity_ratio(100.0, 80_000.0, "", owners)
                })
                .unwrap();
            assert_eq!(result.to_bits(), expected.to_bits());
        }
        let text = String::from_utf8(caller.stream.clone()).unwrap();
        assert_eq!(text.matches("Calculated partial vapor pressure").count(), 1);
        assert!(text.contains(
            " Routine=Unknown, During Sizing, Environment=DECLARED PROPERTY ENVIRONMENT"
        ));
        assert!(text.contains("Simulation time=01/01 00:45 - 01:00"));
        assert!(text.contains("degree less) = "));
        assert!(with_state(|state| state
            .source_string
            .starts_with("Instead, calculated Humidity Ratio")));
        assert_eq!(
            with_state(|state| state.source_error_indices[W_FROM_DEW_POINT]),
            1
        );
        assert_eq!(caller.recurring.number, 1);
        let row = &caller.recurring.rows.as_ref().unwrap()[0];
        assert_eq!((row.count, row.warmup_count, row.sizing_count), (2, 0, 2));
        assert_eq!(
            (row.max_units.as_str(), row.min_units.as_str()),
            ("[]", "[]")
        );
        assert_eq!(row.max_value.to_bits(), expected.to_bits());
        assert_eq!(caller.errors.tracking.total_warning_errors, 2);
    });
}

struct Failing;
impl Write for Failing {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("declared diagnostic failure"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn source_psat_keeps_miss_failure_prefix_and_same_cell_callback_reentry() {
    with_fresh_psychrometric_state(|| {
        let mut caller = Caller::new();
        let mut access = SourcePsychrometricAccess;
        let old = access.evaluate(Function::Psat, &[75.0]).unwrap().result;
        let mut failing = Failing;
        let mut backend = SourceErrorBackend {
            state: &mut caller.errors,
            error_stream: Some(&mut failing),
            console: &mut caller.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let failure = access.source_saturation_pressure(
            300.0,
            "Actual caller",
            &mut SourcePsychrometricOwners {
                clock: &caller.clock,
                globals: &caller.globals,
                environment: &caller.environment,
                hvac: &caller.hvac,
                recurring: &mut caller.recurring,
                backend: &mut backend,
            },
        );
        assert!(matches!(failure, Err(Error::Backend(_))));
        let (tag, value) = psat_slot(300.0);
        assert_eq!(tag, (300.0_f64.to_bits() as i64) >> 28);
        assert_eq!(value.to_bits(), old.to_bits());
        assert_eq!(with_state(|state| state.source_error_indices[PSAT]), 0);
        assert_eq!(caller.recurring.number, 0);
        let hit = caller
            .with_owners(|owners| access.source_saturation_pressure(300.0, "Actual caller", owners))
            .unwrap();
        assert_eq!(hit.to_bits(), old.to_bits());
        assert!(caller.stream.is_empty());
    });
    with_fresh_psychrometric_state(|| {
        let mut caller = Caller::new();
        let called = Cell::new(false);
        let events = RefCell::new(Vec::new());
        let mut callback = |_: &mut SourceErrorState, level, message: &str| {
            events.borrow_mut().push((level, message.to_owned()));
            if !called.replace(true) {
                //75 and300 share the direct-map cell, with different tags.
                SourcePsychrometricAccess
                    .evaluate(Function::Psat, &[75.0])
                    .unwrap();
            }
        };
        let mut backend = SourceErrorBackend {
            state: &mut caller.errors,
            error_stream: Some(&mut caller.stream),
            console: &mut caller.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: Some(&mut callback),
            sqlite: None,
        };
        let value = SourcePsychrometricAccess
            .source_saturation_pressure(
                300.0,
                "A deliberately long actual caller identity that exceeds fifty characters",
                &mut SourcePsychrometricOwners {
                    clock: &caller.clock,
                    globals: &caller.globals,
                    environment: &caller.environment,
                    hvac: &caller.hvac,
                    recurring: &mut caller.recurring,
                    backend: &mut backend,
                },
            )
            .unwrap();
        assert_eq!(value.to_bits(), 1_555_073.745_636_215_f64.to_bits());
        let (tag, stored) = psat_slot(75.0);
        assert_eq!(
            tag,
            (75.0_f64.to_bits() as i64) >> 28,
            "outer finish preserves callback tag"
        );
        assert_eq!(stored.to_bits(), value.to_bits());
        assert_eq!(with_state(|state| state.source_error_indices[PSAT]), 1);
        assert_eq!(caller.recurring.rows.as_ref().unwrap()[0].count, 1);
        let events = events.borrow();
        assert_eq!(events.len(), 4); //Warning, long timestamp2Continues, input temperatureContinue.
        use crate::heat_balance::error_backend::SourceErrorLevel;
        assert_eq!(events[0].0, SourceErrorLevel::Warning);
        assert!(events[1].1.starts_with(" Routine=A deliberately long"));
        assert!(
            events[2]
                .1
                .starts_with(" Environment=DECLARED PROPERTY ENVIRONMENT")
        );
        assert_eq!(events[3].1, " Input Temperature=300.00");
        assert!(
            events[1..]
                .iter()
                .all(|event| event.0 == SourceErrorLevel::Continue)
        );
        let count = caller.errors.tracking.total_warning_errors;
        assert_eq!(
            caller
                .with_owners(
                    |owners| SourcePsychrometricAccess.source_saturation_pressure(75.0, "", owners)
                )
                .unwrap()
                .to_bits(),
            value.to_bits()
        );
        assert_eq!(caller.errors.tracking.total_warning_errors, count);
    });
    for (value, precision, expected) in [
        (12.3499, 2, "12.34"),
        (-12.34, 2, "-12.33"),
        (0.012345678, 4, "1.2345E-002"),
        (-0.0, 2, "0.00"),
    ] {
        assert_eq!(trimmed(value, precision).unwrap(), expected);
    }
}

#[test]
fn source_air_density_reaches_real_fatal_and_retains_diagnostic_failure_prefix() {
    with_fresh_psychrometric_state(|| {
        let mut caller = Caller::new();
        let mut access = SourcePsychrometricAccess;
        for (pressure, temperature, humidity) in [
            (101325.0, 23.0, 0.008),
            (-0.0, 23.0, 0.008),
            (101325.0, 23.0, -1.0),
        ] {
            let expected =
                super::super::energyplus_psy_rho_air_fn_pb_tdb_w(pressure, temperature, humidity);
            let result = caller
                .with_owners(|owners| {
                    access.source_air_density(
                        pressure,
                        temperature,
                        humidity,
                        "Declared density caller",
                        &false,
                        owners,
                    )
                })
                .unwrap();
            assert_eq!(result.to_bits(), expected.to_bits());
        }
        assert!(caller.stream.is_empty());
        caller.clock.warmup_flag = false;
        caller.globals.doing_sizing = true;
        let failure = caller.with_owners(|owners| {
            access.source_air_density(
                -101325.0,
                23.0,
                0.008,
                "Declared density caller",
                &false,
                owners,
            )
        });
        assert!(matches!(
            failure,
            Err(Error::Fatal(
                crate::heat_balance::error_backend::SourceFatalStop::Fatal(_)
            ))
        ));
        assert_eq!(caller.errors.tracking.total_severe_errors, 1);
        assert_eq!(caller.errors.tracking.total_severe_errors_during_warmup, 0);
        assert_eq!(caller.errors.tracking.total_severe_errors_during_sizing, 1);
        let text = String::from_utf8(caller.stream.clone()).unwrap();
        let ordered = [
            " ** Severe  ** PsyRhoAirFnPbTdbW",
            "pb =[-101325.00]",
            " Routine=Declared density caller During Sizing, Environment=DECLARED PROPERTY ENVIRONMENT",
            " **  Fatal  ** Program terminates due to preceding condition.",
            " ...Summary of Errors that led to program termination:",
            " ..... Reference severe error count=1",
            " ..... Last severe error=PsyRhoAirFnPbTdbW",
        ];
        let mut cursor = 0;
        for expected in ordered {
            cursor += text[cursor..].find(expected).expect(expected) + expected.len();
        }
        assert!(
            String::from_utf8(caller.console.clone())
                .unwrap()
                .contains("**FATAL:Program terminates due to preceding condition.")
        );
        assert_eq!(caller.recurring.number, 0);
        assert!(with_state(|state| state
            .source_error_indices
            .iter()
            .all(|index| *index == 0)));
    });
    let mut caller = Caller::new();
    caller.clock.warmup_flag = false;
    let mut failing = Failing;
    let mut backend = SourceErrorBackend {
        state: &mut caller.errors,
        error_stream: Some(&mut failing),
        console: &mut caller.console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let failure = SourcePsychrometricAccess.source_air_density(
        -101325.0,
        23.0,
        0.008,
        "",
        &false,
        &mut SourcePsychrometricOwners {
            clock: &caller.clock,
            globals: &caller.globals,
            environment: &caller.environment,
            hvac: &caller.hvac,
            recurring: &mut caller.recurring,
            backend: &mut backend,
        },
    );
    assert!(matches!(failure, Err(Error::Backend(_))));
    assert_eq!(caller.errors.tracking.total_severe_errors, 1);
    assert!(caller.console.is_empty());
    assert!(
        caller.errors.tracking.last_severe_error.is_empty(),
        "write failure precedes last-severe assignment"
    );
}
