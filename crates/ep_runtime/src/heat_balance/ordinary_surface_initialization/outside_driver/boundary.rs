//! Exact per-surface outside boundary dispatch on the same live history/coefficient cells.
use super::{
    Error, OrdinarySurfaceInitialization, SourceOutsideCtfOwners, index, outdoor, outside_history,
    set, value, write_outside_history,
};
use crate::heat_balance::inside_convection_dispatcher::source::{
    SourceExternalConvectionCall, SourceExternalConvectionOwners,
};
use crate::heat_balance::selected_heat_balance_input::SelectedHeatTransferModel;

pub(super) fn calculate(
    surface: &mut OrdinarySurfaceInitialization,
    o: &mut SourceOutsideCtfOwners<'_, '_>,
    number: i32,
    space: i32,
    construction: i32,
) -> Result<(), Error> {
    let i = index(number)?;
    let actual = o
        .geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(i))
        .ok_or(Error::Unavailable("outside boundary actual Surface"))?;
    let boundary = actual.row.exterior_boundary;
    let algorithm = actual.row.heat_transfer_algorithm;
    let source = o
        .control
        .construction_usage
        .get(index(construction)?)
        .ok_or(Error::Unavailable(
            "outside boundary actual active Construction",
        ))?
        .source_sink_present;
    match boundary {
        -1 | -5 => {
            // Actual -1 BuildingSurface and -5 FCFactorMethod retain their
            // distinct real WeatherEnvironment slots. Kiva -6 is separate.
            let temperature = if boundary == -1 {
                o.environment.ground_temp[0]
            } else {
                o.environment.ground_temp[3]
            };
            write_outside_history(o, number, temperature)?;
            if source {
                return Err(Error::Unavailable(
                    "outside Ground actual RadSysToHBConstCoef7102",
                ));
            }
            if algorithm != SelectedHeatTransferModel::Ctf {
                return Err(Error::Unavailable(
                    "outside Ground positive moisture/non-CTF boundary7110",
                ));
            }
        }
        -2 => {
            return Err(Error::Unavailable(
                "outside actual OtherSideCoefNoCalcExt OSCPtr7214",
            ));
        }
        -3 => {
            return Err(Error::Unavailable(
                "outside actual OtherSideCoefCalcExt OSCPtr7301",
            ));
        }
        -4 => {
            return Err(Error::Unavailable(
                "outside actual OtherSideCondModeledExt OSCMPtr7373",
            ));
        }
        0 => {
            let eco = *o
                .geometry
                .parallel_arrays
                .as_ref()
                .and_then(|a| a.surf_ext_eco_roof.get(i))
                .ok_or(Error::Unavailable("outside actual SurfExtEcoRoof"))?;
            if eco {
                return Err(Error::Unavailable(
                    "outside actual positive CalcEcoRoof7439",
                ));
            }
            let roughness = surface
                .arrays
                .surf_roughness_ext
                .as_ref()
                .and_then(|a| a.get(i))
                .copied()
                .flatten()
                .ok_or(Error::Unavailable(
                    "outside actual written SurfRoughnessExt",
                ))?;
            let absorptance = value(&surface.arrays.surf_abs_thermal_ext, i)?;
            let mut h_movable = 0.0;
            if o.geometry.any_movable_insulation {
                let movable = o
                    .geometry
                    .parallel_arrays
                    .as_ref()
                    .and_then(|a| a.exterior_movable_insulation.get(i))
                    .ok_or(Error::Unavailable("outside actual extMovInsuls"))?;
                if movable.present {
                    h_movable = movable.h;
                }
            }
            let wind_exposed = o
                .geometry
                .final_surfaces
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable("outside actual ExtWind"))?
                .row
                .exterior_wind;
            // Both actual wind/no-wind branches call the same Source helper;
            // that child uses this same ExtWind owner to select actual speed0.
            let call = SourceExternalConvectionCall {
                surface: number,
                h_movable_insulation: h_movable,
                roughness,
                absorptance,
                outside_temperature: outside_history(o, number)?,
            };
            o.convection.initialize_external(
                call,
                &mut SourceExternalConvectionOwners {
                    geometry: o.geometry,
                    zones: o.zones,
                    control: o.control,
                    arrays: &mut surface.arrays,
                    physical: o.physical,
                    environment: o.environment,
                    input: o.input,
                    reports: o.reports,
                    scan: o.scan,
                    backend: o.backend,
                },
            )?;
            let temperature = if wind_exposed && o.environment.current_weather.is_rain {
                let user = *o
                    .geometry
                    .parallel_arrays
                    .as_ref()
                    .and_then(|a| a.exterior_convection.get(i))
                    .map(|row| &row.user_model_num)
                    .ok_or(Error::Unavailable("outside wet actual userModelNum"))?;
                if user == 0 {
                    set(&mut surface.arrays.surf_h_conv_ext, i, 1000.0)?;
                } else {
                    return Err(Error::Unavailable("outside wet actual SetExtConvCoeff7479"));
                }
                outdoor(o, number)?.wet_bulb_c
            } else {
                outdoor(o, number)?.dry_bulb_c
            };
            if algorithm != SelectedHeatTransferModel::Ctf {
                return Err(Error::Unavailable(
                    "outside actual positive moisture/non-CTF external branch7493",
                ));
            }
            if o.geometry
                .final_surfaces
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable(
                    "outside actual surrounding property flag",
                ))?
                .has_surrounding_surface_property
            {
                return Err(Error::Unavailable(
                    "outside actual positive surrounding LW schedule7650",
                ));
            }
            super::scalar::calculate(
                surface,
                o,
                number,
                space,
                construction,
                h_movable,
                temperature,
            )?;
            // Actual MovInsulErrorFlag remains false on the represented no-source
            // branch; no fatal outcome is fabricated or suppressed.
        }
        -6 => {
            return Err(Error::Unavailable(
                "outside actual positive Kiva material/convection7680",
            ));
        }
        _ => {
            let temperature =
                if boundary == number {
                    value(&surface.arrays.surf_temp_in, i)?
                } else {
                    let adjacent = super::history_index(o.geometry, o.history, boundary)?;
                    *o.history.normal.inside_temperature[0].get(adjacent).ok_or(
                        Error::Unavailable("outside actual adjacent SurfInsideTempHist(1)"),
                    )?
                };
            write_outside_history(o, number, temperature)?;
            if algorithm != SelectedHeatTransferModel::Ctf {
                return Err(Error::Unavailable(
                    "outside actual positive moisture/non-CTF interzone branch7700",
                ));
            }
        }
    }
    Ok(())
}
