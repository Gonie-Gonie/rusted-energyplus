//! Genuine InitExtConvCoeff with same shared getter and live output cells.
use super::SourceInsideConvection;
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::convection::tarp_inside::natural_coefficient_w_per_m2_k;
use crate::heat_balance::convection::{
    energyplus_roughness_multiplier, energyplus_surface_is_windward,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::WeatherEnvironmentState;
use ep_model::{
    MaterialSurfaceRoughness as Roughness, OutsideSurfaceConvectionAlgorithm as Outside,
    ReportInputs,
};

/// Native const-value arguments evaluated by the actual outside caller.
pub(crate) struct SourceExternalConvectionCall {
    pub(crate) surface: i32,
    pub(crate) h_movable_insulation: f64,
    pub(crate) roughness: Roughness,
    pub(crate) absorptance: f64,
    pub(crate) outside_temperature: f64,
}
/// Disjoint borrows of the original persistent owners, not a scientific cache.
pub(crate) struct SourceExternalConvectionOwners<'a, 'backend> {
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) arrays: &'a mut SourceSurfaceHeatBalanceArrays,
    pub(crate) physical: &'a [SourceSurfaceOutdoorState],
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) input: &'a SelectedInputProcessor,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
fn index(n: i32) -> Result<usize, Error> {
    n.checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable(
            "actual external-convection positive ordinal",
        ))
}
fn cell(values: &Option<Vec<f64>>, i: usize) -> Result<&f64, Error> {
    values
        .as_deref()
        .and_then(|v| v.get(i))
        .ok_or(Error::Unavailable("actual external-convection live scalar"))
}
fn set(values: &mut Option<Vec<f64>>, i: usize, value: f64) -> Result<(), Error> {
    *values
        .as_deref_mut()
        .and_then(|v| v.get_mut(i))
        .ok_or(Error::Unavailable("actual external-convection live output"))? = value;
    Ok(())
}
fn roughness_index(roughness: Roughness) -> usize {
    match roughness {
        Roughness::VeryRough => 0,
        Roughness::Rough => 1,
        Roughness::MediumRough => 2,
        Roughness::MediumSmooth => 3,
        Roughness::Smooth => 4,
        Roughness::VerySmooth => 5,
    }
}
fn simple(roughness: Roughness, wind: f64) -> f64 {
    let i = roughness_index(roughness);
    let d = [11.58, 12.49, 10.79, 8.23, 10.22, 8.23];
    let e = [5.894, 4.065, 4.192, 4.00, 3.100, 3.33];
    let f = [0.0, 0.028, 0.0, -0.057, 0.0, -0.036];
    d[i] + e[i] * wind + f[i] * (wind * wind)
}
fn forced_smooth(windward: bool, wind: f64) -> f64 {
    if windward {
        3.26 * wind.powf(0.89)
    } else {
        3.55 * wind.powf(0.617)
    }
}
fn doe2_forced(
    surface: f64,
    air: f64,
    cos: f64,
    windward: bool,
    wind: f64,
    roughness: Roughness,
) -> f64 {
    // Source6114: forced smooth is computed BEFORE CalcDOE2Forced natural.
    let smooth = forced_smooth(windward, wind);
    let natural = natural_coefficient_w_per_m2_k(surface, air, cos);
    let combined = (natural * natural + smooth * smooth).sqrt();
    energyplus_roughness_multiplier(roughness) * (combined - natural)
}
fn pow4(value: f64) -> f64 {
    let square = value * value;
    square * square
}

impl SourceInsideConvection {
    pub(crate) fn initialize_external(
        &mut self,
        call: SourceExternalConvectionCall,
        owners: &mut SourceExternalConvectionOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.prepare_external_convection(
            owners.geometry,
            owners.zones,
            owners.control,
            owners.input,
            owners.reports,
            owners.scan,
            owners.backend,
        )?;
        let i = index(call.surface)?;
        let surfaces = owners
            .geometry
            .final_surfaces
            .as_deref()
            .ok_or(Error::Unavailable("actual InitExt Surface"))?;
        let surface = surfaces
            .get(i)
            .ok_or(Error::Unavailable("actual InitExt Surface ordinal"))?;
        let id = surface
            .typed_id
            .ok_or(Error::Unavailable("actual InitExt SurfaceId"))?;
        let physical = owners
            .physical
            .iter()
            .find(|row| row.surface_id == id)
            .ok_or(Error::Unavailable(
                "same sole InitExt physical outdoor owner",
            ))?;
        let air = physical.outdoor_environment.row.dry_bulb_c + 273.15;
        let mut temperature = call.outside_temperature + 273.15;
        let sky = owners.environment.current_weather.sky_temp_kelvin;
        let ground = air;
        set(&mut owners.arrays.surf_h_srd_surf_ext, i, 0.0)?; //445
        if surface.has_surrounding_surface_property {
            return Err(Error::Unavailable(
                "actual InitExt positive surrounding schedule/radiation child448",
            ));
        }
        if surface.use_ground_surface_temperature {
            return Err(Error::Unavailable(
                "actual InitExt positive GroundSurfsProperty temperature457",
            ));
        }
        let base = surface.row.base_surface;
        let wind_direction = physical.outdoor_environment.row.wind_direction_deg;
        let wind = if !surface.row.exterior_wind {
            0.0
        } else if surface.row.class == SurfaceClass::Window {
            let flag = *owners
                .geometry
                .window_arrays
                .as_ref()
                .and_then(|a| a.surf_win_shading_flag.get(i))
                .ok_or(Error::Unavailable("actual InitExt window shading flag"))?;
            if flag == 4 {
                0.0
            } else {
                physical.outdoor_environment.row.wind_speed_m_per_s
            }
        } else {
            physical.outdoor_environment.row.wind_speed_m_per_s
        };
        let parallel = owners
            .geometry
            .parallel_arrays
            .as_ref()
            .ok_or(Error::Unavailable("actual InitExt parallel metadata"))?;
        let mut algorithm = parallel
            .exterior_convection
            .get(i)
            .ok_or(Error::Unavailable("actual surfExtConv ordinal"))?
            .model;
        if algorithm == 2 {
            algorithm = match owners
                .zones
                .zones
                .as_deref()
                .and_then(|rows| index(surface.row.zone).ok().and_then(|i| rows.get(i)))
                .ok_or(Error::Unavailable("actual InitExt Surface Zone"))?
                .outside_algorithm
            {
                Outside::SimpleCombined => 3,
                Outside::Tarp => 4,
                Outside::MoWitt => 6,
                Outside::Doe2 => 7,
                Outside::AdaptiveConvectionAlgorithm => 9,
            };
        }
        match algorithm {
            0|1|13=>return Err(Error::Unavailable("actual InitExt SetExtConvCoeff positive user child")),
            3=>{
                if surface.row.exterior_boundary==-6 {
                    return Err(Error::Unavailable("actual InitExt Kiva simple convection maps489"));
                }
                set(&mut owners.arrays.surf_h_conv_ext,i,simple(call.roughness,wind))?;
            },
            4|5|8=>{
                if surface.row.exterior_boundary==-6 {
                    return Err(Error::Unavailable("actual InitExt Kiva Sparrow convection maps519"));
                }
                let base=surfaces.get(index(base)?).ok_or(Error::Unavailable("actual InitExt base Surface"))?;
                let properties=base.row.vertex_progress.properties.ok_or(Error::Unavailable("actual InitExt base GrossArea"))?;
                let forced=if properties.gross_area_m2!=0.0 && base.row.vertex_progress.height!=0.0 {
                    let perimeter=2.0*(properties.gross_area_m2/base.row.vertex_progress.height+base.row.vertex_progress.height);
                    let properties_surface=surface.row.vertex_progress.properties.ok_or(Error::Unavailable("actual InitExt CosTilt/Azimuth"))?;
                    let windward=energyplus_surface_is_windward(properties_surface.cos_tilt,properties_surface.azimuth_deg,wind_direction);
                    let value=2.537*energyplus_roughness_multiplier(call.roughness)*(perimeter*wind/properties.gross_area_m2).sqrt();
                    if windward {value} else {0.5*value}
                } else {0.0};
                if call.h_movable_insulation>0.0 {
                    temperature=(call.h_movable_insulation*temperature+forced*air)/(call.h_movable_insulation+forced);
                }
                let cos=surface.row.vertex_progress.properties.ok_or(Error::Unavailable("actual InitExt natural CosTilt"))?.cos_tilt;
                let natural=natural_coefficient_w_per_m2_k(temperature,air,cos);
                set(&mut owners.arrays.surf_h_conv_ext,i,natural+forced)?;
            },
            6=>{
                if surface.row.exterior_boundary==-6 {
                    return Err(Error::Unavailable("actual InitExt Kiva MoWITT convection maps575"));
                }
                let properties=surface.row.vertex_progress.properties.ok_or(Error::Unavailable("actual InitExt MoWITT geometry"))?;
                let windward=energyplus_surface_is_windward(properties.cos_tilt,properties.azimuth_deg,wind_direction);
                let natural=0.84*(air-temperature).abs().powf(1.0/3.0);
                let forced=forced_smooth(windward,wind);
                set(&mut owners.arrays.surf_h_conv_ext,i,(natural*natural+forced*forced).sqrt())?;
            },
            7=>{
                if surface.row.exterior_boundary==-6 {
                    return Err(Error::Unavailable("actual InitExt Kiva DOE2 convection maps603"));
                }
                let properties=surface.row.vertex_progress.properties.ok_or(Error::Unavailable("actual InitExt DOE2 geometry"))?;
                let windward=energyplus_surface_is_windward(properties.cos_tilt,properties.azimuth_deg,wind_direction);
                let forced=doe2_forced(temperature,air,properties.cos_tilt,windward,wind,call.roughness);
                if call.h_movable_insulation>0.0 {
                    temperature=(call.h_movable_insulation*temperature+forced*air)/(call.h_movable_insulation+forced);
                }
                let natural=natural_coefficient_w_per_m2_k(temperature,air,properties.cos_tilt);
                set(&mut owners.arrays.surf_h_conv_ext,i,natural+forced)?;
            },
            9=>return Err(Error::Unavailable("actual InitExt positive ManageExtAdaptiveConvAlgo")),
            _=>return match owners.backend.show_fatal_error(&format!("InitExtConvection Coefficients: invalid parameter -- outside convection type, Surface={}",surface.row.name)) {
                Err(error)=>Err(Error::Fatal(error)),Ok(never)=>match never {},
            },
        }
        if *parallel
            .surf_ems_override_ext_conv_coef
            .get(i)
            .ok_or(Error::Unavailable("actual InitExt EMS flag"))?
        {
            let value = *parallel
                .surf_ems_value_for_ext_conv_coef
                .get(i)
                .ok_or(Error::Unavailable("actual InitExt EMS value"))?;
            set(&mut owners.arrays.surf_h_conv_ext, i, value)?;
            if surface.row.exterior_boundary == -6 {
                return Err(Error::Unavailable(
                    "actual InitExt Kiva EMS convection maps653",
                ));
            }
        }
        let adjusted = *cell(&owners.arrays.surf_h_conv_ext, i)?
            * *cell(&owners.arrays.surf_win_coeff_adj_ratio, i)?;
        set(&mut owners.arrays.surf_h_conv_ext, i, adjusted)?; //659
        if temperature == sky || algorithm == 3 {
            set(&mut owners.arrays.surf_h_sky_ext, i, 0.0)?;
        } else {
            let view = surface.row.vertex_progress.view_factor_sky_ir;
            let split = *parallel
                .surf_air_sky_rad_split
                .get(i)
                .ok_or(Error::Unavailable("actual InitExt sky split"))?;
            let value =
                5.6697e-8 * call.absorptance * view * split * (pow4(temperature) - pow4(sky))
                    / (temperature - sky);
            set(&mut owners.arrays.surf_h_sky_ext, i, value)?;
        }
        if temperature == air || algorithm == 3 {
            set(&mut owners.arrays.surf_h_grd_ext, i, 0.0)?;
            set(&mut owners.arrays.surf_h_air_ext, i, 0.0)?;
        } else {
            let ground_view = surface.row.vertex_progress.view_factor_ground_ir;
            let value =
                5.6697e-8 * call.absorptance * ground_view * (pow4(temperature) - pow4(ground))
                    / (temperature - ground);
            set(&mut owners.arrays.surf_h_grd_ext, i, value)?;
            let sky_view = surface.row.vertex_progress.view_factor_sky_ir;
            let split = *parallel
                .surf_air_sky_rad_split
                .get(i)
                .ok_or(Error::Unavailable("actual InitExt air split"))?;
            let value = 5.6697e-8
                * call.absorptance
                * sky_view
                * (1.0 - split)
                * (pow4(temperature) - pow4(air))
                / (temperature - air);
            set(&mut owners.arrays.surf_h_air_ext, i, value)?;
        }
        Ok(())
    }
}
