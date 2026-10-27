//! Actual thirty-nine GetNextEnvironment weather output declarations, in order.
use super::output_bindings::SourceWeatherOutputOwners;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceIntegerOutputBinding, SourceIntegerOutputDeclaration, SourceIntegerOutputTarget,
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceOutputStoreType as Store, SourceOutputUnits as Units, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputTarget, SourceReportVariableInput,
    SourceWeatherIntegerOutputField as Integer, SourceWeatherRealOutputField as Real,
};
use std::io::Write;
pub(crate) struct SourceWeatherOutputContext<'a, 'request, 'backend, H, D> {
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) input: &'a mut SourceReportVariableInput<'request, 'backend>,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
}
enum Target {
    Real(Real),
    Integer(Integer),
}
pub(crate) fn register_source_weather_outputs<
    H: SourceOutputFileHost,
    D: SourceOutputDictionaryHost,
>(
    owners: &SourceWeatherOutputOwners<'_>,
    globals: &SelectedHeatBalanceGlobals,
    c: &mut SourceWeatherOutputContext<'_, '_, '_, H, D>,
) -> Result<(), Error> {
    for (target, name, units, store) in [
        (
            Target::Real(Real::OutDryBulbTemp),
            "Site Outdoor Air Drybulb Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::OutDewPointTemp),
            "Site Outdoor Air Dewpoint Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::OutWetBulbTemp),
            "Site Outdoor Air Wetbulb Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::OutHumRat),
            "Site Outdoor Air Humidity Ratio",
            Units::KgWaterPerKgDryAir,
            Store::Average,
        ),
        (
            Target::Real(Real::OutRelHum),
            "Site Outdoor Air Relative Humidity",
            Units::Percentage,
            Store::Average,
        ),
        (
            Target::Real(Real::OutBaroPress),
            "Site Outdoor Air Barometric Pressure",
            Units::Pa,
            Store::Average,
        ),
        (
            Target::Real(Real::WindSpeed),
            "Site Wind Speed",
            Units::MPerS,
            Store::Average,
        ),
        (
            Target::Real(Real::WindDir),
            "Site Wind Direction",
            Units::Deg,
            Store::Average,
        ),
        (
            Target::Real(Real::SkyTemp),
            "Site Sky Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::HorizIRSky),
            "Site Horizontal Infrared Radiation Rate per Area",
            Units::WPerM2,
            Store::Average,
        ),
        (
            Target::Real(Real::DifSolarRad),
            "Site Diffuse Solar Radiation Rate per Area",
            Units::WPerM2,
            Store::Average,
        ),
        (
            Target::Real(Real::BeamSolarRad),
            "Site Direct Solar Radiation Rate per Area",
            Units::WPerM2,
            Store::Average,
        ),
        (
            Target::Real(Real::LiquidPrecipitation),
            "Liquid Precipitation Depth",
            Units::M,
            Store::Sum,
        ),
        (
            Target::Real(Real::RainfallCurrentRate),
            "Site Precipitation Rate",
            Units::MPerS,
            Store::Average,
        ),
        (
            Target::Real(Real::RainfallCurrentAmount),
            "Site Precipitation Depth",
            Units::M,
            Store::Sum,
        ),
        (
            Target::Real(Real::GndSolarRad),
            "Site Ground Reflected Solar Radiation Rate per Area",
            Units::WPerM2,
            Store::Average,
        ),
        (
            Target::Real(Real::GroundTemperatureBuildingSurface),
            "Site Ground Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::GroundTemperatureShallow),
            "Site Surface Ground Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::GroundTemperatureDeep),
            "Site Deep Ground Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::GroundTemperatureFCFactorMethod),
            "Site Simple Factor Model Ground Temperature",
            Units::C,
            Store::Average,
        ),
        (
            Target::Real(Real::TotalCloudCover),
            "Site Total Sky Cover",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Real(Real::OpaqueCloudCover),
            "Site Opaque Sky Cover",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Real(Real::OutEnthalpy),
            "Site Outdoor Air Enthalpy",
            Units::JPerKg,
            Store::Average,
        ),
        (
            Target::Real(Real::OutAirDensity),
            "Site Outdoor Air Density",
            Units::KgPerM3,
            Store::Average,
        ),
        (
            Target::Real(Real::SolarAzimuthAngle),
            "Site Solar Azimuth Angle",
            Units::Deg,
            Store::Average,
        ),
        (
            Target::Real(Real::SolarAltitudeAngle),
            "Site Solar Altitude Angle",
            Units::Deg,
            Store::Average,
        ),
        (
            Target::Real(Real::HrAngle),
            "Site Solar Hour Angle",
            Units::Deg,
            Store::Average,
        ),
        (
            Target::Integer(Integer::RainStatus),
            "Site Rain Status",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Integer(Integer::SnowStatus),
            "Site Snow on Ground Status",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Real(Real::HorizontalSkyIlluminance),
            "Site Exterior Horizontal Sky Illuminance",
            Units::Lux,
            Store::Average,
        ),
        (
            Target::Real(Real::HorizontalBeamIlluminance),
            "Site Exterior Horizontal Beam Illuminance",
            Units::Lux,
            Store::Average,
        ),
        (
            Target::Real(Real::BeamNormalIlluminance),
            "Site Exterior Beam Normal Illuminance",
            Units::Lux,
            Store::Average,
        ),
        (
            Target::Real(Real::DiffuseLuminousEfficacy),
            "Site Sky Diffuse Solar Radiation Luminous Efficacy",
            Units::LumPerW,
            Store::Average,
        ),
        (
            Target::Real(Real::DirectLuminousEfficacy),
            "Site Beam Solar Radiation Luminous Efficacy",
            Units::LumPerW,
            Store::Average,
        ),
        (
            Target::Real(Real::SkyClearness),
            "Site Daylighting Model Sky Clearness",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Real(Real::SkyBrightness),
            "Site Daylighting Model Sky Brightness",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Integer(Integer::DstIndicator),
            "Site Daylight Saving Time Status",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Integer(Integer::DayTypeIndex),
            "Site Day Type Index",
            Units::Dimensionless,
            Store::Average,
        ),
        (
            Target::Real(Real::WaterMainsTemperature),
            "Site Mains Water Temperature",
            Units::C,
            Store::Average,
        ),
    ] {
        match target {
            Target::Real(field) => {
                let binding =
                    SourceRealOutputBinding::new(SourceRealOutputTarget::Weather { field }, owners)
                        .map_err(Error::Output)?;
                c.registration
                    .setup_bound_unmetered_zone_real(
                        c.input,
                        c.prescan,
                        SourceRealOutputDeclaration {
                            name,
                            key: "Environment",
                            units,
                            store_type: store,
                        },
                        c.minutes_in_time_step,
                        c.host,
                        c.eio,
                        binding,
                        c.dictionary_host,
                    )
                    .map_err(Error::Output)?;
            }
            Target::Integer(field) => {
                let binding = SourceIntegerOutputBinding::new(
                    SourceIntegerOutputTarget::Weather { field },
                    owners,
                )
                .map_err(Error::Output)?;
                c.registration
                    .setup_bound_zone_integer(
                        c.input,
                        c.prescan,
                        SourceIntegerOutputDeclaration {
                            name,
                            key: "Environment",
                            units,
                            store_type: store,
                        },
                        c.minutes_in_time_step,
                        c.host,
                        c.eio,
                        binding,
                        c.dictionary_host,
                    )
                    .map_err(Error::Output)?;
            }
        }
    }
    if globals.any_energy_management_system_in_model {
        return Err(Error::Unavailable(
            "actual Weather SetupEMSActuator650 first Weather Data/Outdoor Dry Bulb",
        ));
    }
    Ok(())
}
