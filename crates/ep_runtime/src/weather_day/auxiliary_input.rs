//! Genuine optional GroundReflectance/SnowModifier/WaterMains getters.
//! No positive getObjectItem or missing schedule/correlation owner is fabricated.
use super::{WeatherDayError, WeatherEnvironmentState, WeatherOwnerState, WeatherSession};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use std::io::Write;

fn fixed(value: f64, width: usize, precision: usize) -> Result<String, Error> {
    if !value.is_finite() {
        return Err(Error::Format(
            "actual auxiliary-weather fixed F nonfinite domain".into(),
        ));
    }
    Ok(format!("{value:width$.precision$}"))
}
/// Actual max(min(value,1),0), with selected ObjexxFCL double overloads.
fn snow_reflectance(value: f64) -> f64 {
    let value = if value < 1.0 { value } else { 1.0 };
    if value < 0.0 { 0.0 } else { value }
}
impl WeatherOwnerState {
    fn get_source_ground_reflectances(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Site:GroundReflectance".into();
        let objects = input.get_num_objects_found(&shortcut.current_module_object);
        if objects != 0 {
            if objects == 1 {
                return Err(Error::Unavailable(
                    "GetGroundReflectances actual getObjectItem6956",
                ));
            } else {
                backend.show_severe_error(&format!(
                    "{}: Too many objects entered. Only one allowed.",
                    shortcut.current_module_object
                ))?;
                *errors_found = true;
            }
        }
        writeln!(
            eio,
            "! <Site:GroundReflectance>,Jan{{dimensionless}},Feb{{dimensionless}},Mar{{dimensionless}},Apr{{dimensionless}},May{{dimensionless}},Jun{{dimensionless}},Jul{{dimensionless}},Aug{{dimensionless}},Sep{{dimensionless}},Oct{{dimensionless}},Nov{{dimensionless}},Dec{{dimensionless}}"
        )?;
        write!(eio, " Site:GroundReflectance")?;
        for month in 1..=12 {
            write!(
                eio,
                ", {}",
                fixed(self.ground_reflectances[month - 1], 5, 2)?
            )?;
        }
        writeln!(eio)?;
        Ok(())
    }
    fn get_source_snow_ground_reflectance_modifiers(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Site:GroundReflectance:SnowModifier".into();
        let objects = input.get_num_objects_found(&shortcut.current_module_object);
        if objects != 0 {
            if objects == 1 {
                return Err(Error::Unavailable(
                    "GetSnowGroundRefModifiers actual getObjectItem7010",
                ));
            } else {
                backend.show_severe_error(&format!(
                    "{}: Too many objects entered. Only one allowed.",
                    shortcut.current_module_object
                ))?;
                *errors_found = true;
            }
        }
        writeln!(
            eio,
            "! <Site:GroundReflectance:SnowModifier>, Normal, Daylighting {{dimensionless}}"
        )?;
        writeln!(
            eio,
            " Site:GroundReflectance:SnowModifier, {}, {}",
            fixed(self.snow_ground_reflectance_modifier, 7, 3)?,
            fixed(self.snow_ground_reflectance_for_daylighting_modifier, 7, 3)?
        )?;
        writeln!(
            eio,
            "! <Site:GroundReflectance:Snow>,Jan{{dimensionless}},Feb{{dimensionless}},Mar{{dimensionless}},Apr{{dimensionless}},May{{dimensionless}},Jun{{dimensionless}},Jul{{dimensionless}},Aug{{dimensionless}},Sep{{dimensionless}},Oct{{dimensionless}},Nov{{dimensionless}},Dec{{dimensionless}}"
        )?;
        write!(eio, " Site:GroundReflectance:Snow")?;
        for month in 1..=12 {
            write!(
                eio,
                ", {}",
                fixed(
                    snow_reflectance(
                        self.ground_reflectances[month - 1] * self.snow_ground_reflectance_modifier
                    ),
                    5,
                    2
                )?
            )?;
        }
        writeln!(eio)?;
        writeln!(
            eio,
            "! <Site:GroundReflectance:Snow:Daylighting>,Jan{{dimensionless}},Feb{{dimensionless}},Mar{{dimensionless}},Apr{{dimensionless}},May{{dimensionless}},Jun{{dimensionless}},Jul{{dimensionless}},Aug{{dimensionless}},Sep{{dimensionless}},Oct{{dimensionless}},Nov{{dimensionless}},Dec{{dimensionless}}"
        )?;
        write!(eio, " Site:GroundReflectance:Snow:Daylighting")?;
        for month in 1..=12 {
            write!(
                eio,
                ", {}",
                fixed(
                    snow_reflectance(
                        self.ground_reflectances[month - 1]
                            * self.snow_ground_reflectance_for_daylighting_modifier
                    ),
                    5,
                    2
                )?
            )?;
        }
        writeln!(eio)?;
        Ok(())
    }
    fn get_source_water_mains_temperatures(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Site:WaterMainsTemperature".into();
        let objects = input.get_num_objects_found(&shortcut.current_module_object);
        if objects == 1 {
            return Err(Error::Unavailable(
                "GetWaterMainsTemperatures actual getObjectItem7076",
            ));
        } else if objects > 1 {
            backend.show_severe_error(&format!(
                "{}: Too many objects entered. Only one allowed.",
                shortcut.current_module_object
            ))?;
            *errors_found = true;
        }
        Ok(())
    }
}
impl WeatherSession {
    /// Actual4822,4824,4826 after Ground4820 and before Station4828. This does
    /// not certify the earlier/later full ReadUserWeatherInput orchestration.
    pub(crate) fn initialize_source_weather_auxiliary_inputs(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        self.state.weather.get_source_ground_reflectances(
            input,
            shortcut,
            backend,
            eio,
            errors_found,
        )?;
        self.state
            .weather
            .get_source_snow_ground_reflectance_modifiers(
                input,
                shortcut,
                backend,
                eio,
                errors_found,
            )?;
        self.state.weather.get_source_water_mains_temperatures(
            input,
            shortcut,
            backend,
            errors_found,
        )?;
        Ok(())
    }
}
/// Actual CalcWaterMainsTemp7136-7180 at SetCurrentWeather2105. FixedDefault=3
/// reaches the original default arm; construction itself never injects10.
pub(super) fn calculate_water_mains_temperature(
    weather: &WeatherOwnerState,
    environment: &mut WeatherEnvironmentState,
) -> Result<(), WeatherDayError> {
    match weather.water_mains_temperature_method {
        0 => {
            return Err(WeatherDayError::admission(
                "CalcWaterMainsTemp actual Schedule getCurrentVal unavailable",
            ));
        }
        1 => {
            return Err(WeatherDayError::admission(
                "CalcWaterMainsTemp actual WaterMainsTempFromCorrelation unavailable",
            ));
        }
        2 => {
            return Err(WeatherDayError::admission(
                "CalcWaterMainsTemp actual OADryBulbAverage owner unavailable",
            ));
        }
        _ => environment.water_mains_temperature = 10.0,
    }
    Ok(())
}
