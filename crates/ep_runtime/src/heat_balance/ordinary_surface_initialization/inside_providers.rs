//! Concrete live inside convection and actual guarded temperature diagnostics.
use super::{SourceInsideCtfProviders, SourceSurfaceHeatBalanceArrays};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::inside_convection_dispatcher::{
    InsideSurfaceTemperatureOwner, source::SourceInsideConvection,
};
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
    surface::SurfaceGeometryInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::WeatherGlobalState;
use ep_model::ReportInputs;

/// The same controller used by InitSurface473 is borrowed after that call.
/// Constructor limits/clock/globals remain in the caller's actual live owners.
pub(crate) struct SourceOrdinaryInsideCtfHost<'a> {
    pub(crate) convection: &'a mut SourceInsideConvection,
    pub(crate) equipment: &'a mut OrdinaryEquipment,
    pub(crate) constructions: &'a SelectedHeatBalanceInputState,
    pub(crate) input: &'a SelectedInputProcessor,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a crate::weather::day::WeatherEnvironmentState,
    pub(crate) hvac: &'a crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
    pub(crate) afn_control_type: &'a i32,
    pub(crate) psychrometrics: &'a mut crate::psychrometrics::SourcePsychrometricAccess,
    pub(crate) developer: &'a bool,
    pub(crate) recurring_errors:
        &'a mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual Source inside diagnostic ordinal",
        ))
}
impl SourceInsideCtfProviders for SourceOrdinaryInsideCtfHost<'_> {
    fn initialize_inside_convection(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        arrays: &mut SourceSurfaceHeatBalanceArrays,
        zones: &mut SelectedZoneInputOwners,
        air: &SourceAirHeatBalance,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        self.convection.initialize(
            InsideSurfaceTemperatureOwner::SurfTempIn,
            geometry,
            arrays,
            zones,
            air,
            self.constructions,
            globals,
            clock,
            self.equipment,
            self.input,
            self.reports,
            self.scan,
            crate::heat_balance::inside_convection_dispatcher::source::SourceInsideConvectionPropertyOwners {
                environment: self.environment, hvac: self.hvac,
                psychrometrics: self.psychrometrics, recurring: self.recurring_errors,
                developer: self.developer,
            },
            backend,
        )
    }

    fn test_surface_temperature(
        &mut self,
        temperature: f64,
        surface: i32,
        zone: i32,
        geometry: &mut SurfaceGeometryInputState,
        zones: &mut SelectedZoneInputOwners,
        warmup_error_count: &mut i32,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        source_test_surface_temperature(
            temperature,
            surface,
            zone,
            geometry,
            zones,
            *warmup_error_count,
            SourceSurfaceTemperatureDiagnosticOwners {
                constructions: self.constructions,
                globals: self.globals,
                clock: self.clock,
                environment: self.environment,
                hvac: self.hvac,
                afn_control_type: self.afn_control_type,
                recurring_errors: self.recurring_errors,
            },
            backend,
        )
    }

    fn warn_nonconvergence(
        &mut self,
        maximum_difference: f64,
        maximum_allowed_cond_fd: f64,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        source_inside_nonconvergence_warning(
            maximum_difference,
            maximum_allowed_cond_fd,
            self.clock,
            self.globals,
            self.environment,
            self.hvac,
            backend,
        )
    }

    fn recurring_nonconvergence(
        &mut self,
        maximum_difference: f64,
        error_pointer: &mut i32,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        source_inside_nonconvergence_recurring(
            maximum_difference,
            error_pointer,
            self.recurring_errors,
            self.clock,
            self.globals,
            backend,
        )
    }
}

/// Per-call references to the real persistent diagnostic, clock and input owners.
/// No temperature, AFN state, phase flag or recurring row is reconstructed here.
struct SourceSurfaceTemperatureDiagnosticOwners<'a> {
    constructions: &'a SelectedHeatBalanceInputState,
    globals: &'a SelectedHeatBalanceGlobals,
    clock: &'a WeatherGlobalState,
    environment: &'a crate::weather::day::WeatherEnvironmentState,
    hvac: &'a crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
    afn_control_type: &'a i32,
    recurring_errors: &'a mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
}

fn temperature_diagnostic_real(value: f64, precision: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, precision).map_err(|error| {
        Error::Format(format!(
            "surface temperature diagnostic R-format unavailable: {error:?}"
        ))
    })
}

fn source_temperature_zone_details(
    zone: &crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData,
    afn_control_type: &i32,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    backend.show_continue_error(&format!("Zone=\"{}\", Diagnostic Details:", zone.name))?;
    if zone.floor_area > 0.0 {
        backend.show_continue_error(&format!(
            "...Internal Heat Gain [{}] W/m2",
            temperature_diagnostic_real(zone.internal_heat_gains / zone.floor_area, 3)?
        ))?;
    } else {
        backend.show_continue_error(&format!(
            "...Internal Heat Gain (no floor) [{}] W",
            temperature_diagnostic_real(zone.internal_heat_gains, 3)?
        ))?;
    }
    if *afn_control_type == 0 {
        backend.show_continue_error(&format!(
            "...Infiltration/Ventilation [{}] m3/s",
            temperature_diagnostic_real(zone.nominal_infiltration_ventilation, 3)?
        ))?;
        backend.show_continue_error(&format!(
            "...Mixing/Cross Mixing [{}] m3/s",
            temperature_diagnostic_real(zone.nominal_mixing, 3)?
        ))?;
    } else {
        backend.show_continue_error(
            "...Airflow Network Simulation: Nominal Infiltration/Ventilation/Mixing not available.",
        )?;
    }
    backend.show_continue_error(if zone.is_controlled {
        "...Zone is part of HVAC controlled system."
    } else {
        "...Zone is not part of HVAC controlled system."
    })?;
    Ok(())
}

/// HBSM9287-9468. The continuing diagnostics change only their actual owners;
/// the inside solver has already written its temperature and history cells.
#[allow(clippy::too_many_arguments, clippy::manual_range_contains)]
fn source_test_surface_temperature(
    temperature: f64,
    surface: i32,
    zone: i32,
    geometry: &mut SurfaceGeometryInputState,
    zones: &mut SelectedZoneInputOwners,
    warmup_error_count: i32,
    o: SourceSurfaceTemperatureDiagnosticOwners<'_>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    // Actual9289 copies the same final Surface name before either predicate.
    let surface_index = index(surface)?;
    let surface_name = geometry
        .final_surfaces
        .as_deref()
        .and_then(|rows| rows.get(surface_index))
        .ok_or(Error::Unavailable("actual diagnostic Surface Name"))?
        .row
        .name
        .clone();
    let zone = zones
        .zones
        .as_mut()
        .and_then(|rows| index(zone).ok().and_then(|i| rows.get_mut(i)))
        .ok_or(Error::Unavailable("actual diagnostic Zone reference"))?;
    // Original takes this argument by value. No caller counter write occurs.
    let mut local_warmup_count = warmup_error_count;
    if temperature > o.constructions.max_surface_temperature_limit || temperature < -100.0 {
        if o.clock.warmup_flag {
            local_warmup_count = local_warmup_count.checked_add(1).ok_or(Error::Unavailable(
                "undefined Source local WarmupSurfTemp increment",
            ))?;
        }
        if !o.clock.warmup_flag || local_warmup_count > 10 || o.globals.display_extra_warnings {
            let low = temperature < -100.0;
            let parallel = geometry.parallel_arrays.as_mut().ok_or(Error::Unavailable(
                "actual diagnostic Surface recurring arrays",
            ))?;
            let message_index = (if low {
                &mut parallel.surf_low_temp_err_count
            } else {
                &mut parallel.surf_high_temp_err_count
            })
            .get_mut(surface_index)
            .ok_or(Error::Unavailable(
                "actual diagnostic Surface recurring index",
            ))?;
            let direction = if low { "low" } else { "high" };
            if *message_index == 0 {
                let temperature_text = temperature_diagnostic_real(temperature, 2)?;
                // Preserve the Original initial high message's unmatched '(' token.
                let opening = if low { '[' } else { '(' };
                backend.show_severe_message(&format!(
                    "Temperature ({direction}) out of bounds {opening}{temperature_text}] for zone=\"{}\", for surface=\"{surface_name}\"",
                    zone.name
                ))?;
                backend.show_source_continue_error_time_stamp(
                    "",
                    o.clock,
                    o.globals,
                    o.environment,
                    o.hvac,
                )?;
                if !zone.temp_out_of_bounds_reported {
                    source_temperature_zone_details(zone, o.afn_control_type, backend)?;
                    zone.temp_out_of_bounds_reported = true;
                }
            }
            backend.show_source_recurring_severe_at_end(
                &format!(
                    "Temperature ({direction}) out of bounds for zone={} for surface={surface_name}",
                    zone.name
                ),
                message_index,
                o.recurring_errors,
                o.clock,
                o.globals,
                Some(temperature),
                Some(temperature),
                None,
                "C",
                "C",
                "",
            )?;
            // Fatal children remain unavailable at their first unported call,
            // after the genuine preceding continuing diagnostic effects above.
            if zone.enforced_reciprocity {
                if local_warmup_count > 3 {
                    return Err(Error::Unavailable(
                        "actual TestSurfTemp enforced-reciprocity fatal child9389",
                    ));
                }
            } else if local_warmup_count > 10 {
                return Err(Error::Unavailable(
                    "actual TestSurfTemp warmup fatal child9395",
                ));
            }
        }
    }
    if temperature > o.constructions.max_surface_temperature_before_fatal || temperature < -250.0 {
        if !o.clock.warmup_flag {
            return Err(Error::Unavailable(
                "actual positive TestSurfTemp fatal diagnostic9420",
            ));
        } else if temperature < -10000.0 || temperature > 10000.0 {
            return Err(Error::Unavailable(
                "actual extreme warmup TestSurfTemp diagnostic9459",
            ));
        }
    }
    Ok(())
}

/// Shared actual production warning/formatter/timestamp operations. No full
/// convection host is constructed by the specified suffix unit.
pub(crate) fn source_inside_nonconvergence_warning(
    maximum_difference: f64,
    maximum_allowed_cond_fd: f64,
    clock: &WeatherGlobalState,
    globals: &SelectedHeatBalanceGlobals,
    environment: &crate::weather::day::WeatherEnvironmentState,
    hvac: &crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let difference = crate::heat_balance::source_round_format::round(maximum_difference, 3)
        .map_err(|error| {
            Error::Format(format!(
                "inside maximum difference .3R unavailable: {error:?}"
            ))
        })?;
    let allowed = crate::heat_balance::source_round_format::round(maximum_allowed_cond_fd, 6)
        .map_err(|error| {
            Error::Format(format!(
                "inside allowed difference .6R unavailable: {error:?}"
            ))
        })?;
    backend.show_inside_warning_error(&format!(
            "Inside surface heat balance did not converge with Max Temp Difference [C] ={difference} vs Max Allowed Temp Diff [C] ={allowed}"
        ), clock, globals)?;
    backend.show_source_continue_error_time_stamp("", clock, globals, environment, hvac)
}
/// Same original recurring message/optional operands; shared by the live host
/// and the explicitly selected suffix unit, on their real persistent rows.
pub(crate) fn source_inside_nonconvergence_recurring(
    maximum_difference: f64,
    error_pointer: &mut i32,
    recurring_errors: &mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
    clock: &WeatherGlobalState,
    globals: &SelectedHeatBalanceGlobals,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    backend.show_source_recurring_warning_at_end(
        "Inside surface heat balance convergence problem continues",
        error_pointer,
        recurring_errors,
        clock,
        globals,
        Some(maximum_difference),
        Some(maximum_difference),
        None,
        "[C]",
        "[C]",
        "",
    )
}

#[cfg(test)]
mod temperature_diagnostic_tests {
    use super::*;
    use crate::heat_balance::error_backend::{
        SourceErrorLevel, SourceErrorSqlite, SourceErrorState, SourceErrorTracking,
    };
    use crate::heat_balance::selected_heat_balance_input::surface::SurfaceClass;
    use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
    use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
    use crate::heat_balance::source_initialization::SourceRecurringErrorState;
    use crate::weather::day::WeatherEnvironmentState;
    type TestResult = Result<(), Box<dyn std::error::Error>>;

    // Authored function-entry unit data over the real constructors/arrays only.
    // No full Source manager execution, input admission or Native parity claim.
    struct Fixture {
        geometry: SurfaceGeometryInputState,
        zones: SelectedZoneInputOwners,
        control: SelectedHeatBalanceInputState,
        globals: SelectedHeatBalanceGlobals,
        clock: WeatherGlobalState,
        environment: WeatherEnvironmentState,
        hvac: SelectedHvacGlobals,
        afn_control_type: i32,
        recurring: SourceRecurringErrorState,
        warmup_errors: i32,
        state: SourceErrorState,
        error: Vec<u8>,
        console: Vec<u8>,
    }
    impl Fixture {
        fn new() -> Self {
            let mut geometry = SurfaceGeometryInputState::default();
            geometry.restore_native_cohort_opaque_surfaces(&[
                (1.0, SurfaceClass::Wall, 0),
                (1.0, SurfaceClass::Roof, 0),
            ]);
            if let Some(rows) = geometry.final_surfaces.as_mut() {
                rows[0].row.name = "WALL ONE".into();
                rows[1].row.name = "ROOF".into();
            }
            Self {
                geometry,
                zones: SelectedZoneInputOwners {
                    zones: Some(vec![SelectedZoneData {
                        name: "ZONE ONE".into(),
                        floor_area: 2.0,
                        internal_heat_gains: 100.0,
                        nominal_infiltration_ventilation: 0.5,
                        nominal_mixing: 0.25,
                        is_controlled: true,
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                control: Default::default(),
                globals: Default::default(),
                clock: WeatherGlobalState {
                    time_step_zone: 0.25,
                    current_time: 10.0,
                    ..Default::default()
                },
                environment: WeatherEnvironmentState {
                    environment_name: "UNIT ENVIRONMENT".into(),
                    current_month_day: "01/01".into(),
                    ..Default::default()
                },
                hvac: Default::default(),
                afn_control_type: 0,
                recurring: Default::default(),
                warmup_errors: 0,
                state: SourceErrorState {
                    output_error_header: false,
                    version_string: "authored version".into(),
                    idd_version_string: "authored IDD".into(),
                    doing_input_processing: false,
                    print_console_output: false,
                    kickoff_simulation: false,
                    developer_flag: false,
                    warmup_flag: false,
                    doing_sizing: false,
                    tracking: SourceErrorTracking {
                        ask_for_plant_check_on_abort: false,
                        exit_during_simulations: false,
                        match_counts: [0; 20],
                        abort_processing: false,
                        ask_for_connections_report: false,
                        ask_for_surfaces_report: false,
                        total_multiplied_windows: 0,
                        total_coincident_vertices: 0,
                        total_degenerate_surfaces: 0,
                        total_receiving_non_convex_surfaces: 0,
                        total_casting_non_convex_surfaces: 0,
                        total_severe_errors: 0,
                        total_severe_errors_during_warmup: 0,
                        total_severe_errors_during_sizing: 0,
                        last_severe_error: String::new(),
                        total_warning_errors: 0,
                        total_warning_errors_during_warmup: 0,
                        total_warning_errors_during_sizing: 0,
                    },
                },
                error: Vec::new(),
                console: Vec::new(),
            }
        }
        fn run(&mut self, temperature: f64, surface: i32) -> Result<(), Error> {
            source_test_surface_temperature(
                temperature,
                surface,
                1,
                &mut self.geometry,
                &mut self.zones,
                self.warmup_errors,
                SourceSurfaceTemperatureDiagnosticOwners {
                    constructions: &self.control,
                    globals: &self.globals,
                    clock: &self.clock,
                    environment: &self.environment,
                    hvac: &self.hvac,
                    afn_control_type: &self.afn_control_type,
                    recurring_errors: &mut self.recurring,
                },
                &mut backend(&mut self.state, &mut self.error, &mut self.console),
            )
        }
    }
    fn backend<'a>(
        state: &'a mut SourceErrorState,
        error: &'a mut Vec<u8>,
        console: &'a mut Vec<u8>,
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

    #[test]
    fn surface_temperature_strict_bounds_and_warmup_fatal_gate() -> TestResult {
        // T, warmup, extra warnings, incoming by-value count, reciprocity, returns, recurring count.
        for (temperature, warmup, extra, count, reciprocity, returns, events) in [
            (-100.0, false, false, 0, false, true, 0),
            (200.0, false, false, 0, false, true, 0),
            (-100.000001, false, false, 0, false, true, 1),
            (200.000001, false, false, 0, false, true, 1),
            (-250.0, false, false, 0, false, true, 1),
            (500.0, false, false, 0, false, true, 1),
            (-250.000001, false, false, 0, false, false, 1),
            (500.000001, false, false, 0, false, false, 1),
            (201.0, true, false, 0, false, true, 0),
            (201.0, true, false, 9, false, true, 0),
            (201.0, true, false, 10, false, false, 1),
            (201.0, true, true, 2, true, true, 1),
            (201.0, true, true, 3, true, false, 1),
            (201.0, true, false, 3, true, true, 0),
            (-10000.0, true, false, 0, false, true, 0),
            (10000.0, true, false, 0, false, true, 0),
            (-10000.000001, true, false, 0, false, false, 0),
            (10000.000001, true, false, 0, false, false, 0),
            (f64::NAN, false, false, 0, false, true, 0),
        ] {
            let mut f = Fixture::new();
            f.clock.warmup_flag = warmup;
            f.globals.display_extra_warnings = extra;
            f.warmup_errors = count;
            f.zones.zones.as_mut().ok_or("unit Zone allocation")?[0].enforced_reciprocity =
                reciprocity;
            let result = f.run(temperature, 1);
            assert_eq!(
                result.is_ok(),
                returns,
                "T={temperature}, warmup={warmup}, extra={extra}, count={count}"
            );
            if !returns {
                assert!(matches!(result, Err(Error::Unavailable(_))));
            }
            assert_eq!(
                f.warmup_errors, count,
                "by-value local must not write the caller"
            );
            assert_eq!(f.state.tracking.total_severe_errors, events);
            assert_eq!(f.recurring.number, i32::from(events > 0));
        }
        Ok(())
    }

    #[test]
    fn surface_temperature_first_repeated_shared_zone_and_exact_summary() -> TestResult {
        let mut f = Fixture::new();
        for (temperature, surface) in [(201.0, 1), (205.0, 1), (202.0, 2), (-101.0, 1), (-110.0, 1)]
        {
            f.run(temperature, surface)?;
        }
        let rows = f
            .recurring
            .rows
            .as_ref()
            .ok_or("unit recurring allocation")?;
        assert_eq!(f.recurring.number, 3);
        assert_eq!(
            rows.iter()
                .map(|r| (r.count, r.max_value, r.min_value))
                .collect::<Vec<_>>(),
            [(2, 205.0, 201.0), (1, 202.0, 202.0), (2, -101.0, -110.0)]
        );
        let parallel = f
            .geometry
            .parallel_arrays
            .as_ref()
            .ok_or("unit Surface arrays")?;
        assert_eq!(parallel.surf_high_temp_err_count, [1, 2]);
        assert_eq!(parallel.surf_low_temp_err_count, [3, 0]);
        assert!(
            f.zones.zones.as_ref().ok_or("unit Zone allocation")?[0].temp_out_of_bounds_reported
        );
        assert_eq!(f.state.tracking.total_severe_errors, 5);
        assert_eq!(f.state.tracking.match_counts[15], 3);
        assert_eq!(f.state.tracking.match_counts[16], 5);
        let text = std::str::from_utf8(&f.error)?;
        assert_eq!(text.matches("Diagnostic Details:").count(), 1);
        assert_eq!(
            text.matches("Temperature (high) out of bounds (").count(),
            2
        );
        assert_eq!(text.matches("Temperature (low) out of bounds [").count(), 1);
        assert!(text.contains("...Internal Heat Gain [50.000] W/m2"));
        assert!(
            text.contains("Environment=UNIT ENVIRONMENT, at Simulation time=01/01 09:45 - 10:00")
        );
        let prefix_len = f.error.len();
        backend(&mut f.state, &mut f.error, &mut f.console)
            .show_source_recurring_errors(&f.recurring)?;
        assert_eq!(
            std::str::from_utf8(&f.error[prefix_len..])?,
            concat!(
                "   *************\n",
                "   ************* ===== Recurring Error Summary =====\n",
                "   ************* The following recurring error messages occurred.\n",
                "   *************\n",
                "   *************  ** Severe  ** Temperature (high) out of bounds for zone=ZONE ONE for surface=WALL ONE\n",
                "   *************  **   ~~~   **   This error occurred 2 total times;\n",
                "   *************  **   ~~~   **   during Warmup 0 times;\n",
                "   *************  **   ~~~   **   during Sizing 0 times.\n",
                "   *************  **   ~~~   **   Max=205.000000 C  Min=201.000000 C\n",
                "   *************\n",
                "   *************  ** Severe  ** Temperature (high) out of bounds for zone=ZONE ONE for surface=ROOF\n",
                "   *************  **   ~~~   **   This error occurred 1 total times;\n",
                "   *************  **   ~~~   **   during Warmup 0 times;\n",
                "   *************  **   ~~~   **   during Sizing 0 times.\n",
                "   *************  **   ~~~   **   Max=202.000000 C  Min=202.000000 C\n",
                "   *************\n",
                "   *************  ** Severe  ** Temperature (low) out of bounds for zone=ZONE ONE for surface=WALL ONE\n",
                "   *************  **   ~~~   **   This error occurred 2 total times;\n",
                "   *************  **   ~~~   **   during Warmup 0 times;\n",
                "   *************  **   ~~~   **   during Sizing 0 times.\n",
                "   *************  **   ~~~   **   Max=-101.000000 C  Min=-110.000000 C\n",
                "   *************\n",
            )
        );
        assert_eq!(
            f.state.tracking.total_severe_errors, 5,
            "serialization must not recount errors"
        );
        assert_eq!(
            (
                f.state.tracking.match_counts[15],
                f.state.tracking.match_counts[16]
            ),
            (3, 5)
        );
        Ok(())
    }

    // Explicit borrowed-interface test double; it does not represent a database.
    #[derive(Default)]
    struct SqliteCalls(Vec<String>);
    impl SourceErrorSqlite for SqliteCalls {
        fn create_error_record(
            &mut self,
            simulation: i32,
            kind: i32,
            message: &str,
            count: i32,
        ) -> std::io::Result<()> {
            self.0
                .push(format!("create:{simulation}:{kind}:{message}:{count}"));
            Ok(())
        }
        fn update_error_record(&mut self, message: &str) -> std::io::Result<()> {
            self.0.push(format!("update:{message}"));
            Ok(())
        }
        fn within_transaction(&mut self) -> std::io::Result<bool> {
            Ok(false)
        }
        fn commit(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn surface_temperature_live_phase_counts_and_recurring_optional_effects() -> TestResult {
        let mut f = Fixture::new();
        f.clock.warmup_flag = true;
        f.globals.display_extra_warnings = true;
        f.globals.doing_sizing = true;
        f.run(201.0, 1)?;
        f.run(205.0, 1)?;
        let row = &f
            .recurring
            .rows
            .as_ref()
            .ok_or("unit recurring allocation")?[0];
        assert_eq!((row.count, row.warmup_count, row.sizing_count), (2, 2, 2));
        assert_eq!(f.warmup_errors, 0);
        assert_eq!(f.state.tracking.total_severe_errors, 2);
        assert_eq!(f.state.tracking.total_severe_errors_during_warmup, 0);
        assert_eq!(f.state.tracking.total_severe_errors_during_sizing, 0);
        assert!(
            std::str::from_utf8(&f.error)?
                .contains("During Warmup & Sizing, Environment=UNIT ENVIRONMENT")
        );
        let mut sqlite = SqliteCalls::default();
        let mut callbacks = Vec::new();
        {
            let mut callback =
                |_: &mut SourceErrorState, level: SourceErrorLevel, message: &str| {
                    callbacks.push((level, message.to_owned()));
                };
            let mut b = backend(&mut f.state, &mut f.error, &mut f.console);
            b.sqlite = Some(&mut sqlite);
            b.error_callback = Some(&mut callback);
            b.show_source_recurring_errors(&f.recurring)?;
        }
        assert!(sqlite.0.contains(&"create:1:1:Temperature (high) out of bounds for zone=ZONE ONE for surface=WALL ONE:2".into()));
        assert!(callbacks.windows(2).any(|pair| pair[0] == (SourceErrorLevel::Severe, " ** Severe  ** Temperature (high) out of bounds for zone=ZONE ONE for surface=WALL ONE".into())
            && pair[1] == (SourceErrorLevel::Continue, String::new())));
        assert_eq!(f.state.tracking.total_severe_errors, 2);
        Ok(())
    }
}
