//! Direct DataSurfaces surface-environment setters, with caller-owned rows.
//!
//! Direct setters and the bounded ordinary A caller share these exact bodies.
//! Full local-environment/EMS surface-manager branches remain outside this unit.

use ep_model::Terrain;
mod initialization;
pub use initialization::SurfaceOutdoorInitializationError;
pub(crate) mod ordinary;
pub(crate) mod source_rows;
pub use ordinary::{
    SurfaceOutdoorEnvironmentError, SurfaceOutdoorEnvironmentState,
    check_surface_out_bulb_temperatures,
};

const EARTH_RADIUS_M: f64 = 6_356_000.0;
const ATMOSPHERIC_TEMPERATURE_GRADIENT_K_PER_M: f64 = 0.0065;

/// Actual environment operands read by the three setters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceOutdoorEnvironmentParameters {
    /// Actual SiteTempGradient.
    pub site_temperature_gradient_k_per_m: f64,
    /// Actual WeatherFileTempModCoeff after the station producer.
    pub weather_file_temperature_modifier_c: f64,
    /// Actual SiteWindExp.
    pub site_wind_exponent: f64,
    /// Actual SiteWindBLHeight.
    pub site_wind_boundary_layer_height_m: f64,
    /// Actual WeatherFileWindModCoeff.
    pub weather_file_wind_modifier: f64,
}

impl SurfaceOutdoorEnvironmentParameters {
    /// Source default producer for absent Site:WeatherStation/HeightVariation.
    /// Terrain is the actual compiled Building value. Explicit overrides remain
    /// outside the ordinary CON01 scope; the direct API can receive real operands.
    #[must_use]
    pub fn default_weather_station(terrain: Terrain) -> Self {
        let mut parameters = Self::for_building_terrain(terrain);
        initialization::assign_default_weather_station_coefficients(&mut parameters);
        parameters
    }

    fn temperature_from_base(self, base_temperature_c: f64, height_m: f64) -> f64 {
        if height_m <= 0.0 {
            base_temperature_c
        } else {
            let gradient_dividend =
                self.site_temperature_gradient_k_per_m * EARTH_RADIUS_M * height_m;
            let gradient_divisor = EARTH_RADIUS_M + height_m;
            base_temperature_c - gradient_dividend / gradient_divisor
        }
    }

    /// DataEnvironment::OutDryBulbTempAt71-116 on the actual point operands.
    pub(crate) fn point_dry_bulb_temperature_at_height(
        self,
        outdoor_temperature_c: f64,
        height_m: f64,
        backend: &mut crate::heat_balance::error_backend::SourceErrorBackend<'_>,
        flags: &crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
    ) -> Result<f64, crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError> {
        use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
        let base_temperature_c = outdoor_temperature_c + self.weather_file_temperature_modifier_c;
        let local_temperature_c = if self.site_temperature_gradient_k_per_m == 0.0 {
            outdoor_temperature_c
        } else if height_m <= 0.0 {
            base_temperature_c
        } else {
            base_temperature_c
                - self.site_temperature_gradient_k_per_m * EARTH_RADIUS_M * height_m
                    / (EARTH_RADIUS_M + height_m)
        };
        if local_temperature_c < -100.0 {
            backend.show_source_severe_error(
                "OutDryBulbTempAt: outdoor drybulb temperature < -100 C",
                flags,
            )?;
            let height =
                crate::heat_balance::source_round_format::round(height_m, 0).map_err(|error| {
                    Error::Format(format!("OutDryBulbTempAt Native .0R: {error:?}"))
                })?;
            backend.show_continue_error(&format!("...check heights, this height=[{height}]."))?;
            return match backend
                .show_source_fatal_error("Program terminates due to preceding condition(s).", flags)
            {
                Ok(never) => match never {},
                Err(error) => Err(error),
            };
        }
        Ok(local_temperature_c)
    }

    /// The selected setter's temperature operand, not DataEnvironment's distinct
    /// point getter (whose zero-gradient branch differs in its base handling).
    #[must_use]
    pub fn temperature_at_height(self, outdoor_temperature_c: f64, height_m: f64) -> f64 {
        if self.site_temperature_gradient_k_per_m == 0.0 {
            outdoor_temperature_c
        } else {
            let base_temperature_c =
                outdoor_temperature_c + self.weather_file_temperature_modifier_c;
            self.temperature_from_base(base_temperature_c, height_m)
        }
    }

    /// DataEnvironment::WindSpeedAt point getter, distinct from the surface setter.
    /// Height is tested before the zero-exponent branch; the nonzero path uses
    /// one platform pow on the height/boundary-layer ratio in original association.
    pub fn point_wind_speed_at_height(
        self,
        wind_speed_m_per_s: f64,
        height_m: f64,
    ) -> Result<f64, ep_platform_math::MathProviderError> {
        if height_m <= 0.0 {
            return Ok(0.0);
        }
        if self.site_wind_exponent == 0.0 {
            return Ok(wind_speed_m_per_s);
        }
        Ok(wind_speed_m_per_s
            * self.weather_file_wind_modifier
            * ep_platform_math::pow(
                height_m / self.site_wind_boundary_layer_height_m,
                self.site_wind_exponent,
            )?)
    }

    /// Wind-exposed row of SetSurfaceWindSpeedAt, preserving its multiplication
    /// and pow order. No clipping of the passed wind operand is introduced.
    #[must_use]
    pub fn wind_speed_at_height(self, wind_speed_m_per_s: f64, height_m: f64) -> f64 {
        let factor = wind_speed_m_per_s
            * self.weather_file_wind_modifier
            * self
                .site_wind_boundary_layer_height_m
                .powf(-self.site_wind_exponent);
        if self.site_wind_exponent == 0.0 {
            wind_speed_m_per_s
        } else if height_m <= 0.0 {
            0.0
        } else {
            factor * height_m.powf(self.site_wind_exponent)
        }
    }
}

/// Actual global weather values at the selected setter call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceOutdoorWeather {
    /// Actual OutDryBulbTemp.
    pub dry_bulb_c: f64,
    /// Actual OutWetBulbTemp.
    pub wet_bulb_c: f64,
    /// Actual WindSpeed.
    pub wind_speed_m_per_s: f64,
    /// Actual WindDir.
    pub wind_direction_deg: f64,
}

/// One ordered allocated surface row. The caller supplies actual geometry and
/// exposure plus all four incoming owners, including retained NoWind values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceOutdoorEnvironmentRow {
    /// Actual stored Surface.Centroid.z; never derived from vertices here.
    pub centroid_height_m: f64,
    /// Actual Surface.ExtWind.
    pub wind_exposed: bool,
    /// Actual incoming/current SurfOutDryBulbTemp row.
    pub dry_bulb_c: f64,
    /// Actual incoming/current SurfOutWetBulbTemp row.
    pub wet_bulb_c: f64,
    /// Actual incoming/current SurfOutWindSpeed row.
    pub wind_speed_m_per_s: f64,
    /// Actual incoming/current SurfOutWindDir row.
    pub wind_direction_deg: f64,
}

// Both routes use the same numerical body. Source rows borrow actual live
// centroid/ExtWind fields; legacy/direct rows retain their established carrier.
trait SurfaceOutdoorSetterRow {
    fn centroid_height_m(&self) -> f64;
    fn wind_exposed(&self) -> bool;
    fn fields(&mut self) -> &mut SurfaceOutdoorEnvironmentRow;
}
impl SurfaceOutdoorSetterRow for &mut SurfaceOutdoorEnvironmentRow {
    fn centroid_height_m(&self) -> f64 {
        self.centroid_height_m
    }
    fn wind_exposed(&self) -> bool {
        self.wind_exposed
    }
    fn fields(&mut self) -> &mut SurfaceOutdoorEnvironmentRow {
        self
    }
}

/// SetSurfaceOutBulbTempAt: all TotSurfaces rows, dry then wet writes.
pub fn set_surface_out_bulb_temperatures(
    parameters: SurfaceOutdoorEnvironmentParameters,
    weather: SurfaceOutdoorWeather,
    rows: &mut [SurfaceOutdoorEnvironmentRow],
) {
    set_surface_out_bulb_temperature_rows(parameters, weather, rows.iter_mut());
}

fn set_surface_out_bulb_temperature_rows<R: SurfaceOutdoorSetterRow>(
    parameters: SurfaceOutdoorEnvironmentParameters,
    weather: SurfaceOutdoorWeather,
    rows: impl Iterator<Item = R>,
) {
    if parameters.site_temperature_gradient_k_per_m == 0.0 {
        for mut row in rows {
            row.fields().dry_bulb_c = weather.dry_bulb_c;
            row.fields().wet_bulb_c = weather.wet_bulb_c;
        }
    } else {
        let base_dry_temperature_c =
            weather.dry_bulb_c + parameters.weather_file_temperature_modifier_c;
        let base_wet_temperature_c =
            weather.wet_bulb_c + parameters.weather_file_temperature_modifier_c;
        for mut row in rows {
            let height_m = row.centroid_height_m();
            if height_m <= 0.0 {
                row.fields().dry_bulb_c = base_dry_temperature_c;
                row.fields().wet_bulb_c = base_wet_temperature_c;
            } else {
                let gradient_dividend =
                    parameters.site_temperature_gradient_k_per_m * EARTH_RADIUS_M * height_m;
                let gradient_divisor = EARTH_RADIUS_M + height_m;
                row.fields().dry_bulb_c =
                    base_dry_temperature_c - gradient_dividend / gradient_divisor;
                row.fields().wet_bulb_c =
                    base_wet_temperature_c - gradient_dividend / gradient_divisor;
            }
        }
    }
}

/// SetSurfaceWindSpeedAt: zero exponent writes every row; otherwise NoWind rows
/// retain their actual incoming value. Row order and allocation are unchanged.
pub fn set_surface_wind_speeds(
    parameters: SurfaceOutdoorEnvironmentParameters,
    wind_speed_m_per_s: f64,
    rows: &mut [SurfaceOutdoorEnvironmentRow],
) {
    set_surface_wind_speed_rows(parameters, wind_speed_m_per_s, rows.iter_mut());
}

fn set_surface_wind_speed_rows<R: SurfaceOutdoorSetterRow>(
    parameters: SurfaceOutdoorEnvironmentParameters,
    wind_speed_m_per_s: f64,
    rows: impl Iterator<Item = R>,
) {
    match set_surface_wind_speed_rows_with_pow(
        parameters,
        wind_speed_m_per_s,
        rows,
        |base, exponent| Ok::<f64, std::convert::Infallible>(base.powf(exponent)),
    ) {
        Ok(()) => (),
        Err(never) => match never {},
    }
}

fn set_surface_wind_speed_rows_with_pow<R: SurfaceOutdoorSetterRow, E>(
    parameters: SurfaceOutdoorEnvironmentParameters,
    wind_speed_m_per_s: f64,
    rows: impl Iterator<Item = R>,
    mut pow: impl FnMut(f64, f64) -> Result<f64, E>,
) -> Result<(), E> {
    let factor = wind_speed_m_per_s
        * parameters.weather_file_wind_modifier
        * pow(
            parameters.site_wind_boundary_layer_height_m,
            -parameters.site_wind_exponent,
        )?;
    if parameters.site_wind_exponent == 0.0 {
        for mut row in rows {
            row.fields().wind_speed_m_per_s = wind_speed_m_per_s;
        }
    } else {
        for mut row in rows {
            if !row.wind_exposed() {
                continue;
            }
            let height_m = row.centroid_height_m();
            if height_m <= 0.0 {
                row.fields().wind_speed_m_per_s = 0.0;
            } else {
                row.fields().wind_speed_m_per_s =
                    factor * pow(height_m, parameters.site_wind_exponent)?;
            }
        }
    }
    Ok(())
}

/// SetSurfaceWindDirAt: no normalization or exposure filtering.
pub fn set_surface_wind_directions(
    wind_direction_deg: f64,
    rows: &mut [SurfaceOutdoorEnvironmentRow],
) {
    set_surface_wind_direction_rows(wind_direction_deg, rows.iter_mut());
}

fn set_surface_wind_direction_rows<R: SurfaceOutdoorSetterRow>(
    wind_direction_deg: f64,
    rows: impl Iterator<Item = R>,
) {
    for mut row in rows {
        row.fields().wind_direction_deg = wind_direction_deg;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(height: f64, exposed: bool, wind: f64) -> SurfaceOutdoorEnvironmentRow {
        SurfaceOutdoorEnvironmentRow {
            centroid_height_m: height,
            wind_exposed: exposed,
            dry_bulb_c: 7.0,
            wet_bulb_c: 6.0,
            wind_speed_m_per_s: wind,
            wind_direction_deg: 123.0,
        }
    }

    #[test]
    fn no_wind_retains_owner_bits_across_multiple_calls() {
        let parameters =
            SurfaceOutdoorEnvironmentParameters::default_weather_station(Terrain::Suburbs);
        let retained = f64::from_bits(0x7ff8_0000_0000_1234);
        let mut rows = [row(3.0, false, retained), row(0.0, true, 9.0)];
        for wind in [4.0, -2.0, 0.0] {
            set_surface_wind_speeds(parameters, wind, &mut rows);
            assert_eq!(rows[0].wind_speed_m_per_s.to_bits(), retained.to_bits());
            assert_eq!(rows[1].wind_speed_m_per_s.to_bits(), 0.0_f64.to_bits());
            assert_eq!(rows[0].dry_bulb_c.to_bits(), 7.0_f64.to_bits());
            assert_eq!(rows[0].wind_direction_deg.to_bits(), 123.0_f64.to_bits());
        }
    }

    #[test]
    fn zero_exponent_overwrites_no_wind_and_nonpositive_heights() {
        let parameters = SurfaceOutdoorEnvironmentParameters {
            site_wind_exponent: 0.0,
            ..SurfaceOutdoorEnvironmentParameters::default_weather_station(Terrain::Country)
        };
        let mut rows = [
            row(-1.0, false, 9.0),
            row(0.0, true, 8.0),
            row(3.0, false, 7.0),
        ];
        set_surface_wind_speeds(parameters, -0.0, &mut rows);
        for value in rows {
            assert_eq!(value.wind_speed_m_per_s.to_bits(), (-0.0_f64).to_bits());
        }

        // Point getters test height first and preserve the exact zero-exponent
        // return. A nontrivial ratio exercises the real platform pow ABI.
        let point_parameters = SurfaceOutdoorEnvironmentParameters {
            site_wind_boundary_layer_height_m: f64::NAN,
            weather_file_wind_modifier: f64::NAN,
            ..parameters
        };
        let nonzero_point_parameters = SurfaceOutdoorEnvironmentParameters {
            site_wind_exponent: 0.5,
            site_wind_boundary_layer_height_m: 16.0,
            weather_file_wind_modifier: 2.0,
            ..parameters
        };
        assert!(
            [(-1.0, 0.0_f64), (-0.0, 0.0), (3.0, -0.0)]
                .iter()
                .all(|(height, expected)| point_parameters
                    .point_wind_speed_at_height(-0.0, *height)
                    .is_ok_and(|value| value.to_bits() == expected.to_bits()))
                && nonzero_point_parameters
                    .point_wind_speed_at_height(4.0, 4.0)
                    .is_ok_and(|value| value.to_bits() == 4.0_f64.to_bits())
        );

        assert_eq!(
            ep_platform_math::log(1.0).expect("platform log").to_bits(),
            0.0_f64.to_bits()
        );
        for (base, exponent, expected) in [
            (2.0, 3.0, 8.0_f64),
            (3.0, 2.0, 9.0),
            (4.0, 0.5, 2.0),
            (2.0, -2.0, 0.25),
        ] {
            assert_eq!(
                ep_platform_math::pow(base, exponent)
                    .expect("platform pow")
                    .to_bits(),
                expected.to_bits()
            );
        }
    }

    #[test]
    fn zero_gradient_copies_bulbs_without_station_modifier() {
        let parameters = SurfaceOutdoorEnvironmentParameters {
            site_temperature_gradient_k_per_m: 0.0,
            weather_file_temperature_modifier_c: 50.0,
            ..SurfaceOutdoorEnvironmentParameters::default_weather_station(Terrain::Ocean)
        };
        let weather = SurfaceOutdoorWeather {
            dry_bulb_c: -0.0,
            wet_bulb_c: 3.0,
            wind_speed_m_per_s: 9.0,
            wind_direction_deg: -12.0,
        };
        let mut rows = [row(-5.0, true, 8.0), row(9.0, false, 7.0)];
        set_surface_out_bulb_temperatures(parameters, weather, &mut rows);
        for value in rows {
            assert_eq!(value.dry_bulb_c.to_bits(), weather.dry_bulb_c.to_bits());
            assert_eq!(value.wet_bulb_c.to_bits(), weather.wet_bulb_c.to_bits());
            assert_eq!(value.wind_direction_deg.to_bits(), 123.0_f64.to_bits());
        }
        let mut state = crate::heat_balance::error_backend::SourceErrorState {
            output_error_header: true,
            version_string: "focused point getter".into(),
            idd_version_string: "declared test IDD".into(),
            doing_input_processing: false,
            print_console_output: false,
            kickoff_simulation: false,
            developer_flag: false,
            warmup_flag: false,
            doing_sizing: false,
            tracking: crate::heat_balance::error_backend::SourceErrorTracking {
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
        };
        let phase = false;
        let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
            kickoff: &phase,
            developer: &phase,
            warmup: &phase,
            doing_sizing: &phase,
        };
        let mut error_text = Vec::new();
        let mut console = Vec::new();
        let exact_boundary = {
            let mut backend = crate::heat_balance::error_backend::SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error_text),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            assert!(
                parameters
                    .point_dry_bulb_temperature_at_height(
                        weather.dry_bulb_c,
                        9.0,
                        &mut backend,
                        &flags,
                    )
                    .is_ok_and(|value| value.to_bits() == (-0.0_f64).to_bits())
            );
            let gradient_parameters = SurfaceOutdoorEnvironmentParameters {
                site_temperature_gradient_k_per_m: 0.0065,
                ..parameters
            };
            assert!(
                gradient_parameters
                    .point_dry_bulb_temperature_at_height(-6.0, -5.0, &mut backend, &flags,)
                    .is_ok_and(|value| value.to_bits() == 44.0_f64.to_bits())
            );
            let analytic_parameters = SurfaceOutdoorEnvironmentParameters {
                site_temperature_gradient_k_per_m: 0.00000095367431640625,
                weather_file_temperature_modifier_c: 2.0,
                ..parameters
            };
            assert!(
                analytic_parameters
                    .point_dry_bulb_temperature_at_height(20.0, 6_356_000.0, &mut backend, &flags,)
                    .is_ok_and(|value| value.to_bits() == 0x4032_f81f_0000_0000_u64)
            );
            gradient_parameters.point_dry_bulb_temperature_at_height(
                -150.0,
                0.0,
                &mut backend,
                &flags,
            )
        };
        assert!(
            exact_boundary.is_ok_and(|value| value.to_bits() == (-100.0_f64).to_bits())
                && state.tracking.total_severe_errors == 0
                && state.output_error_header
                && error_text.is_empty()
                && console.is_empty()
        );
    }

    #[test]
    fn nonpositive_heights_share_base_and_direction_copies_raw_signed_zero() {
        let parameters =
            SurfaceOutdoorEnvironmentParameters::default_weather_station(Terrain::City);
        let weather = SurfaceOutdoorWeather {
            dry_bulb_c: -6.0,
            wet_bulb_c: -7.0,
            wind_speed_m_per_s: 4.0,
            wind_direction_deg: -0.0,
        };
        let mut rows = [row(-5.0, true, 8.0), row(0.0, false, 7.0)];
        set_surface_out_bulb_temperatures(parameters, weather, &mut rows);
        assert_eq!(rows[0].dry_bulb_c.to_bits(), rows[1].dry_bulb_c.to_bits());
        assert_eq!(rows[0].wet_bulb_c.to_bits(), rows[1].wet_bulb_c.to_bits());
        set_surface_wind_directions(weather.wind_direction_deg, &mut rows);
        for value in rows {
            assert_eq!(
                value.wind_direction_deg.to_bits(),
                weather.wind_direction_deg.to_bits()
            );
        }
    }
}

#[cfg(test)]
#[path = "surface_environment_selected_reference.rs"]
mod selected_reference;
