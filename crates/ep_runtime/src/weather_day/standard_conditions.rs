//! Genuine successful ResolveLocationInformation4476-4495 standard-condition child.
//! The resolved actual LocationTitle is borrowed; TimeZoneNumber is the same live field.
use super::{WeatherDayError, WeatherEnvironmentState};
impl WeatherEnvironmentState {
    pub(crate) fn resolve_source_standard_conditions(
        &mut self,
        location_title: &str,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), WeatherDayError> {
        self.std_baro_press = 101325.0 * (1.0 - 2.25577e-5 * self.elevation).powf(5.2559);
        let density = crate::psychrometrics::energyplus_psy_rho_air_fn_pb_tdb_w(
            self.std_baro_press,
            20.0,
            0.0,
        );
        if density < 0.0 {
            return Err(WeatherDayError::admission(
                "actual PsyRhoAirFnPbTdbW negative-density diagnostic/fatal child before StdRhoAir4477 assignment",
            ));
        }
        self.std_rho_air = density;
        eio.write_all(b"! <Site:Location>, Location Name, Latitude {N+/S- Deg}, Longitude {E+/W- Deg},  Time Zone Number {GMT+/-}, Elevation {m},  Standard Pressure at Elevation {Pa}, Standard RhoAir at Elevation\n")
            .map_err(|e| WeatherDayError::admission(format!("actual Site:Location header EIO: {e}")))?;
        let round = |value, precision| {
            crate::heat_balance::source_round_format::round(value, precision).map_err(|e| {
                WeatherDayError::admission(format!("actual Site:Location .{precision}R: {e:?}"))
            })
        };
        let row = format!(
            "Site:Location,{},{},{},{},{},{},{}\n",
            location_title,
            round(self.latitude, 2)?,
            round(self.longitude, 2)?,
            round(self.time_zone_number, 2)?,
            round(self.elevation, 2)?,
            round(self.std_baro_press, 0)?,
            round(self.std_rho_air, 4)?
        );
        eio.write_all(row.as_bytes()).map_err(|e| {
            WeatherDayError::admission(format!("actual Site:Location row EIO: {e}"))
        })?;
        Ok(())
    }
}
