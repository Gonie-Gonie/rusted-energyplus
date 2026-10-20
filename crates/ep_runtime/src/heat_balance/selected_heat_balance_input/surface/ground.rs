//! Original SurfaceGeometry4027-4040: true once flag and actual weather inputs.
use super::{Error, SourceErrorBackend};
use crate::weather::day::WeatherEnvironmentState;

/// Wrapper supplies the native true constructor within the larger derived
/// Default owner. It has exactly the one actual NoGroundTempObjWarning bool.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GroundTemperatureWarning {
    pending: bool,
}
impl Default for GroundTemperatureWarning {
    fn default() -> Self {
        Self { pending: true }
    }
}
impl GroundTemperatureWarning {
    pub(super) fn read(
        &mut self,
        environment: &WeatherEnvironmentState,
        name: &str,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.pending {
            if !environment.ground_temp_inputs[0] {
                backend.show_warning_error("GetHTSurfaceData: Surfaces with interface to Ground found but no \"Ground Temperatures\" were input.")?;
                backend.show_continue_error(&format!("Found first in surface={name}"))?;
                backend.show_continue_error(&format!(
                    "Defaults, constant throughout the year of ({}) will be used.",
                    super::super::rounded(environment.ground_temp[0], 1)?
                ))?;
            }
            // Native clears after all optional diagnostics, also with true input.
            self.pending = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
    use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
    use crate::heat_balance::selected_heat_balance_input::surface::final_mapping::FinalSurface;
    use crate::heat_balance::selected_heat_balance_input::surface::{
        SurfaceClass, SurfaceGeometryInputState, SurfaceTmpInput,
    };
    use crate::heat_balance::selected_heat_balance_input::zone::{
        SelectedZoneData, SelectedZoneInputOwners,
    };
    use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
    use std::io::{self, Write};
    fn state() -> SourceErrorState {
        SourceErrorState {
            output_error_header: true,
            version_string: "declared Ground caller".into(),
            idd_version_string: "declared IDD".into(),
            doing_input_processing: false,
            print_console_output: true,
            kickoff_simulation: false,
            developer_flag: false,
            warmup_flag: false,
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
    struct Failing;
    impl Write for Failing {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("declared Ground warning failure"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn ground_warning_uses_actual_input_temperature_and_retains_failed_once_prefix() {
        for provided in [false, true] {
            let mut warning = GroundTemperatureWarning::default();
            let mut environment = WeatherEnvironmentState {
                ground_temp: [18.0, 0.0, 0.0, 0.0],
                ground_temp_inputs: [provided, false, false, false],
                ..Default::default()
            };
            let mut errors = state();
            let mut stream = Vec::new();
            let mut console = Vec::new();
            let mut backend = SourceErrorBackend {
                state: &mut errors,
                error_stream: Some(&mut stream),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            warning
                .read(&environment, "DECLARED FIRST GROUND", &mut backend)
                .unwrap();
            assert!(!warning.pending);
            environment.ground_temp_inputs[0] = false;
            warning
                .read(&environment, "DECLARED SECOND GROUND", &mut backend)
                .unwrap();
            let text = String::from_utf8(stream).unwrap();
            assert_eq!(errors.tracking.total_warning_errors, i32::from(!provided));
            if provided {
                assert!(text.is_empty());
            } else {
                assert_eq!(text.matches("Ground found but no").count(), 1);
                assert!(text.contains("Found first in surface=DECLARED FIRST GROUND"));
                assert!(
                    text.contains("Defaults, constant throughout the year of (18.0) will be used.")
                );
                assert!(!text.contains("DECLARED SECOND GROUND"));
            }
        }
        let mut warning = GroundTemperatureWarning::default();
        let mut errors = state();
        let mut console = Vec::new();
        let mut stream = Failing;
        let environment = WeatherEnvironmentState::default();
        let mut backend = SourceErrorBackend {
            state: &mut errors,
            error_stream: Some(&mut stream),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        assert!(
            warning
                .read(&environment, "DECLARED FAILED GROUND", &mut backend)
                .is_err()
        );
        assert!(warning.pending);
        assert_eq!(errors.tracking.total_warning_errors, 1);
    }
    #[test]
    fn ground_setup_areas_write_real_zone_ground_totals_without_exterior_or_space_aliases() {
        let mut geometry = SurfaceGeometryInputState {
            total_surfaces: 4,
            ..Default::default()
        };
        let rows = [
            (SurfaceClass::Wall, -1, 4.0, 3.5),
            (SurfaceClass::Floor, -1, 6.0, 6.0),
            (SurfaceClass::Roof, -1, 8.0, 8.0),
            (SurfaceClass::Roof, 4, 2.0, 2.0),
        ];
        geometry.final_surfaces = Some(
            rows.into_iter()
                .enumerate()
                .map(|(index, (class, boundary, gross, area))| FinalSurface {
                    row: SurfaceTmpInput {
                        class,
                        exterior_boundary: boundary,
                        base_surface: i32::try_from(index + 1).unwrap(),
                        heat_transfer_surface: true,
                        zone: 1,
                        space: 1,
                        construction: 1,
                        gross_area: gross,
                        area,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .collect(),
        );
        let mut zones = SelectedZoneInputOwners {
            zones: Some(vec![SelectedZoneData {
                multiplier: 2,
                list_multiplier: 3,
                ..Default::default()
            }]),
            spaces: Some(vec![Default::default()]),
            ..Default::default()
        };
        zones.spaces.as_mut().unwrap()[0].zone_num = 1;
        let control = SelectedHeatBalanceInputState {
            construction_usage: vec![Default::default()],
            ..Default::default()
        };
        let input =
            SelectedInputProcessor::from_admitted_input(Vec::new(), Default::default()).unwrap();
        assert!(
            !geometry
                .accumulate_opaque_setup_areas(&input, &mut zones, &control)
                .unwrap()
        );
        let zone = &zones.zones.as_ref().unwrap()[0];
        let space = &zones.spaces.as_ref().unwrap()[0];
        assert_eq!(zone.exterior_total_ground_surface_area, 18.0);
        assert_eq!(zone.exterior_gross_ground_wall_area, 4.0);
        assert_eq!(zone.exterior_gross_ground_wall_area_multiplied, 24.0);
        assert_eq!(
            [zone.total_surface_area, space.total_surface_area],
            [19.5, 19.5]
        );
        assert_eq!(
            [zone.geometric_floor_area, zone.geometric_ceiling_area],
            [6.0, 10.0]
        );
        assert_eq!(
            [
                zone.exterior_total_surface_area,
                space.exterior_total_surface_area,
                zone.exterior_perimeter,
                space.exterior_perimeter
            ],
            [0.0; 4]
        );
    }
}
