//! Genuine EcoRoofManager five-procedure owner/caller port, Native26.1.
//! All persistent numerical fields remain in the sole session EcoRoof compartment.
//! The CTF source-history read stays unavailable when its actual owner is absent.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    reason = "Retain Native local scalar names and expression association for direct review."
)]

use super::error_backend::SourceDiagnosticFlags;
use super::inside_convection_dispatcher::source::{
    SourceExternalConvectionCall, SourceExternalConvectionOwners,
};
use super::ordinary_hvac_manager::water_manager::{
    SourceIrrigationMode, SourceRainfallMode, SourceWaterManager,
};
use super::ordinary_surface_initialization::{
    OrdinarySurfaceInitialization, outside_driver::SourceOutsideCtfOwners,
};
use super::radiation::ordinary::{OrdinaryRadiationError, OrdinaryRadiationUnavailable};
use super::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatTransferModel,
};
use super::selected_material_data::SelectedMaterialData;
use super::source_initialization::SourceRecurringErrorState;
use super::source_output_prescan::SourceOutputPreScan;
use super::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputOwners, SourceRealOutputTarget, SourceRealRegistrationOptions,
    SourceReportVariableEnvironment, SourceReportVariableInput, TimeStepKind,
};
use crate::error::{RuntimeError, WarmupOutputError};
use crate::weather::day::source_current_weather::SourceEcoRoofData;
use ep_model::MaterialId;
use ep_platform_math::MathProviderError;
use std::{io::Write, sync::OnceLock};

/// Actual18 field identities, never copied report values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(
    clippy::upper_case_acronyms,
    reason = "Retain Native EcoRoof output-member identity names."
)]
pub(crate) enum SourceEcoRoofOutputField {
    Tg,
    Tf,
    MeanRootMoisture,
    Moisture,
    Sensibleg,
    Sensiblef,
    Vfluxf,
    Vfluxg,
    Lf,
    Lg,
    CumPrecip,
    CumIrrigation,
    CumRunoff,
    CumET,
    CurrentPrecipitation,
    CurrentIrrigation,
    CurrentRunoff,
    CurrentET,
}

impl SourceRealOutputOwners for SourceEcoRoofData {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::EcoRoof { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            SourceEcoRoofOutputField::Tg => &self.tg,
            SourceEcoRoofOutputField::Tf => &self.tf,
            SourceEcoRoofOutputField::MeanRootMoisture => &self.mean_root_moisture,
            SourceEcoRoofOutputField::Moisture => &self.moisture,
            SourceEcoRoofOutputField::Sensibleg => &self.sensibleg,
            SourceEcoRoofOutputField::Sensiblef => &self.sensiblef,
            SourceEcoRoofOutputField::Vfluxf => &self.vfluxf,
            SourceEcoRoofOutputField::Vfluxg => &self.vfluxg,
            SourceEcoRoofOutputField::Lf => &self.lf,
            SourceEcoRoofOutputField::Lg => &self.lg,
            SourceEcoRoofOutputField::CumPrecip => &self.cum_precip,
            SourceEcoRoofOutputField::CumIrrigation => &self.cum_irrigation,
            SourceEcoRoofOutputField::CumRunoff => &self.cum_runoff,
            SourceEcoRoofOutputField::CumET => &self.cum_et,
            SourceEcoRoofOutputField::CurrentPrecipitation => &self.current_precipitation,
            SourceEcoRoofOutputField::CurrentIrrigation => &self.current_irrigation,
            SourceEcoRoofOutputField::CurrentRunoff => &self.current_runoff,
            SourceEcoRoofOutputField::CurrentET => &self.current_et,
        })
    }
}

/// Actual outside child, borrowing only the existing surface and persistent owners.
pub(crate) trait SourceEcoRoofOutsideChild {
    fn calculate(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        o: &mut SourceOutsideCtfOwners<'_, '_>,
        number: i32,
        space: i32,
        construction: i32,
    ) -> Result<f64, Error>;
}
/// No cloned Material/water/physics state; this is a temporary disjoint view.
pub(crate) struct SourceEcoRoofOutsideOwners<'a, H, D> {
    pub(crate) eco_roof: &'a mut SourceEcoRoofData,
    pub(crate) materials: &'a mut SelectedMaterialData,
    pub(crate) water: &'a mut SourceWaterManager,
    pub(crate) recurring_errors: &'a mut SourceRecurringErrorState,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) developer: &'a bool,
    pub(crate) output_host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("EcoRoof actual positive Source ordinal"))
}
fn value(values: &Option<Vec<f64>>, i: usize, name: &'static str) -> Result<f64, Error> {
    values
        .as_ref()
        .and_then(|v| v.get(i))
        .copied()
        .ok_or(Error::Unavailable(name))
}
fn history_index(o: &SourceOutsideCtfOwners<'_, '_>, number: i32) -> Result<usize, Error> {
    let id = o
        .geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(index(number).ok()?))
        .and_then(|row| row.typed_id)
        .ok_or(Error::Unavailable(
            "EcoRoof same actual final Surface identity",
        ))?;
    o.history
        .surface_ids
        .iter()
        .position(|&v| v == id)
        .ok_or(Error::Unavailable(
            "EcoRoof same actual outside-history owner",
        ))
}
fn material_identity(
    materials: &SelectedMaterialData,
    o: &SourceOutsideCtfOwners<'_, '_>,
    construction: i32,
) -> Result<(i32, MaterialId), Error> {
    let usage = o
        .control
        .construction_usage
        .get(index(construction)?)
        .ok_or(Error::Unavailable("EcoRoof actual Construction"))?;
    let number = *usage
        .layer_points
        .first()
        .ok_or(Error::Unavailable("EcoRoof actual outside LayerPoint"))?;
    let id = *materials
        .materials
        .get(index(number)?)
        .ok_or(Error::Unavailable("EcoRoof actual outside MaterialNum"))?;
    Ok((number, id))
}
fn math(operation: &'static str, result: Result<f64, MathProviderError>) -> Result<f64, Error> {
    result.map_err(|error| {
        Error::SourceInput(Box::new(RuntimeError::WarmupOutput(Box::new(
            WarmupOutputError::Unavailable(format!("actual EcoRoof {operation}: {error}")),
        ))))
    })
}
fn pow_2(value: f64) -> f64 {
    value * value
}
fn pow_3(value: f64) -> f64 {
    (value * value) * value
}
fn pow_4(value: f64) -> f64 {
    (value * value) * (value * value)
}
fn cpp_min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceEcoRoofOutsideChild
    for SourceEcoRoofOutsideOwners<'_, H, D>
{
    fn calculate(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        o: &mut SourceOutsideCtfOwners<'_, '_>,
        number: i32,
        _space: i32,
        construction: i32,
    ) -> Result<f64, Error> {
        self.calc_eco_roof(surface, o, number, construction)
    }
}

impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceEcoRoofOutsideOwners<'_, H, D> {
    /// EcoRoofManager516-683; the first-time flag clears before the actual fatal branch.
    pub(crate) fn init_eco_roof_first_time(
        &mut self,
        o: &mut SourceOutsideCtfOwners<'_, '_>,
        number: i32,
        construction: i32,
    ) -> Result<(), Error> {
        let (material_number, id) = material_identity(self.materials, o, construction)?;
        let mat = self
            .materials
            .source_roof_vegetation_material(material_number, id)?;
        self.eco_roof.eco_roof_begin_flag = false;
        let i = index(number)?;
        let algorithm = o
            .geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(i))
            .ok_or(Error::Unavailable(
                "initEcoRoofFirstTime actual Surface algorithm",
            ))?
            .row
            .heat_transfer_algorithm;
        if algorithm != SelectedHeatTransferModel::Ctf {
            let flags = SourceDiagnosticFlags {
                kickoff: &o.globals.kickoff_simulation,
                developer: self.developer,
                warmup: &o.clock.warmup_flag,
                doing_sizing: &o.globals.doing_sizing,
            };
            o.backend.show_source_severe_error(
                "initEcoRoofFirstTime: EcoRoof simulation but HeatBalanceAlgorithm is not ConductionTransferFunction(CTF). EcoRoof model currently works only with CTF heat balance solution algorithm.",
                &flags,
            )?;
            let name = &o
                .geometry
                .final_surfaces
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable(
                    "initEcoRoofFirstTime actual Surface Name",
                ))?
                .row
                .name;
            o.backend
                .show_continue_error(&format!("Occurs for surface named {name}"))?;
            o.backend.show_continue_error("Check input syntax for HeatBalanceAlgorithm, SurfaceProperty:HeatTransferAlgorithm,")?;
            o.backend.show_continue_error("SurfaceProperty:HeatTransferAlgorithm:MultipleSurface, and SurfaceProperty:HeatTransferAlgorithm:SurfaceList ")?;
            o.backend.show_continue_error("to verify that the solution method is set to CTF for the surface that is an EcoRoof.")?;
            return match o.backend.show_source_fatal_error(
                "initEcoRoofFirstTime: Program terminates due to preceding conditions.",
                &flags,
            ) {
                Ok(never) => match never {},
                Err(error) => Err(error),
            };
        }
        self.eco_roof.zf = mat.height_of_plants;
        self.eco_roof.lai = mat.leaf_area_index;
        self.eco_roof.alphag = 1.0 - mat.solar_absorptance;
        self.eco_roof.alphaf = mat.leaf_reflectivity;
        self.eco_roof.epsilonf = mat.leaf_emissivity;
        self.eco_roof.stomatal_resistance_min = mat.minimum_stomatal_resistance;
        self.eco_roof.epsilong = mat.thermal_absorptance;
        self.eco_roof.moisture_max = mat.porosity;
        self.eco_roof.moisture_residual = mat.minimum_moisture;
        self.eco_roof.moisture = mat.initial_moisture;
        self.eco_roof.mean_root_moisture = self.eco_roof.moisture;
        self.eco_roof.soil_thickness = mat.thickness;
        self.eco_roof.first_eco_surf = number;
        for (name, field, units, store_type) in [
            (
                "Green Roof Soil Temperature",
                SourceEcoRoofOutputField::Tg,
                SourceOutputUnits::C,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Vegetation Temperature",
                SourceEcoRoofOutputField::Tf,
                SourceOutputUnits::C,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Soil Root Moisture Ratio",
                SourceEcoRoofOutputField::MeanRootMoisture,
                SourceOutputUnits::Dimensionless,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Soil Near Surface Moisture Ratio",
                SourceEcoRoofOutputField::Moisture,
                SourceOutputUnits::Dimensionless,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Soil Sensible Heat Transfer Rate per Area",
                SourceEcoRoofOutputField::Sensibleg,
                SourceOutputUnits::WPerM2,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Vegetation Sensible Heat Transfer Rate per Area",
                SourceEcoRoofOutputField::Sensiblef,
                SourceOutputUnits::WPerM2,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Vegetation Moisture Transfer Rate",
                SourceEcoRoofOutputField::Vfluxf,
                SourceOutputUnits::MPerS,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Soil Moisture Transfer Rate",
                SourceEcoRoofOutputField::Vfluxg,
                SourceOutputUnits::MPerS,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Vegetation Latent Heat Transfer Rate per Area",
                SourceEcoRoofOutputField::Lf,
                SourceOutputUnits::WPerM2,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Soil Latent Heat Transfer Rate per Area",
                SourceEcoRoofOutputField::Lg,
                SourceOutputUnits::WPerM2,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Cumulative Precipitation Depth",
                SourceEcoRoofOutputField::CumPrecip,
                SourceOutputUnits::M,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Cumulative Irrigation Depth",
                SourceEcoRoofOutputField::CumIrrigation,
                SourceOutputUnits::M,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Cumulative Runoff Depth",
                SourceEcoRoofOutputField::CumRunoff,
                SourceOutputUnits::M,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Cumulative Evapotranspiration Depth",
                SourceEcoRoofOutputField::CumET,
                SourceOutputUnits::M,
                SourceOutputStoreType::Average,
            ),
            (
                "Green Roof Current Precipitation Depth",
                SourceEcoRoofOutputField::CurrentPrecipitation,
                SourceOutputUnits::M,
                SourceOutputStoreType::Sum,
            ),
            (
                "Green Roof Current Irrigation Depth",
                SourceEcoRoofOutputField::CurrentIrrigation,
                SourceOutputUnits::M,
                SourceOutputStoreType::Sum,
            ),
            (
                "Green Roof Current Runoff Depth",
                SourceEcoRoofOutputField::CurrentRunoff,
                SourceOutputUnits::M,
                SourceOutputStoreType::Sum,
            ),
            (
                "Green Roof Current Evapotranspiration Depth",
                SourceEcoRoofOutputField::CurrentET,
                SourceOutputUnits::M,
                SourceOutputStoreType::Sum,
            ),
        ] {
            let binding = SourceRealOutputBinding::new(
                SourceRealOutputTarget::EcoRoof { field },
                &*self.eco_roof,
            )
            .map_err(Error::Output)?;
            let mut input = SourceReportVariableInput::new(
                o.reports,
                &mut *o.input,
                SourceReportVariableEnvironment::from_owners(
                    self.minimum_report_frequency,
                    o.globals,
                    self.developer,
                    Some(o.clock),
                ),
                o.backend,
            );
            self.registration
                .setup_bound_real(
                    &mut input,
                    self.prescan,
                    SourceRealOutputDeclaration {
                        name,
                        key: "Environment",
                        units,
                        store_type,
                    },
                    o.clock.minutes_in_time_step,
                    self.output_host,
                    self.eio,
                    binding,
                    self.dictionary_host,
                    SourceRealRegistrationOptions::unmetered(TimeStepKind::Zone),
                )
                .map_err(Error::Output)?;
        }
        Ok(())
    }

    /// EcoRoofManager685-723 on every active surface call, including warmup's later surfaces.
    pub(crate) fn init_eco_roof(
        &mut self,
        o: &mut SourceOutsideCtfOwners<'_, '_>,
        number: i32,
        construction: i32,
    ) -> Result<(), Error> {
        let (material_number, id) = material_identity(self.materials, o, construction)?;
        let mat = self
            .materials
            .source_roof_vegetation_material(material_number, id)?;
        let actual_surface = o
            .geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable("initEcoRoof actual Surface"))?;
        if o.clock.begin_envrn_flag || o.clock.warmup_flag {
            self.eco_roof.moisture = mat.initial_moisture;
            self.eco_roof.mean_root_moisture = self.eco_roof.moisture;
            self.eco_roof.alphag = 1.0 - mat.solar_absorptance;
        }
        if o.clock.begin_envrn_flag && self.eco_roof.calc_eco_roof_my_envrn_flag {
            let flags = SourceDiagnosticFlags {
                kickoff: &o.globals.kickoff_simulation,
                developer: self.developer,
                warmup: &o.clock.warmup_flag,
                doing_sizing: &o.globals.doing_sizing,
            };
            self.eco_roof.tgold = o
                .environment
                .outdoor_environment_parameters
                .point_dry_bulb_temperature_at_height(
                    o.environment.current_weather.out_dry_bulb_temp,
                    actual_surface.centroid[2],
                    o.backend,
                    &flags,
                )?;
            self.eco_roof.tfold = o
                .environment
                .outdoor_environment_parameters
                .point_dry_bulb_temperature_at_height(
                    o.environment.current_weather.out_dry_bulb_temp,
                    actual_surface.centroid[2],
                    o.backend,
                    &flags,
                )?;
            self.eco_roof.tg = 10.0;
            self.eco_roof.tf = 10.0;
            self.eco_roof.vfluxf = 0.0;
            self.eco_roof.vfluxg = 0.0;
            self.eco_roof.cum_runoff = 0.0;
            self.eco_roof.cum_et = 0.0;
            self.eco_roof.cum_precip = 0.0;
            self.eco_roof.cum_irrigation = 0.0;
            self.eco_roof.current_runoff = 0.0;
            self.eco_roof.current_et = 0.0;
            self.eco_roof.current_precipitation = 0.0;
            self.eco_roof.current_irrigation = 0.0;
            self.eco_roof.calc_eco_roof_my_envrn_flag = false;
        }
        if !o.clock.begin_envrn_flag {
            self.eco_roof.calc_eco_roof_my_envrn_flag = true;
        }
        Ok(())
    }

    /// EcoRoofManager188-513. First surface solves; every active surface writes shared Tgold.
    #[expect(
        clippy::needless_late_init,
        clippy::neg_multiply,
        reason = "Preserve Native EcoRoofManager188-513 branch assignments and multiplication order."
    )]
    pub(crate) fn calc_eco_roof(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        o: &mut SourceOutsideCtfOwners<'_, '_>,
        number: i32,
        construction: i32,
    ) -> Result<f64, Error> {
        const Kv: f64 = 0.4;
        const rch: f64 = 0.63;
        const rche: f64 = 0.71;
        const Rair: f64 = 0.286e3;
        const g1: f64 = 9.81;
        const Sigma: f64 = 5.6697e-08;
        const Cpa: f64 = 1005.6;
        const Kelvin: f64 = 273.15;
        let i = index(number)?;
        let height = o
            .geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(i))
            .ok_or(Error::Unavailable("CalcEcoRoof actual Surface centroid"))?
            .centroid[2];
        let mut Ws = math(
            "WindSpeedAt pow",
            o.environment
                .outdoor_environment_parameters
                .point_wind_speed_at_height(o.environment.current_weather.wind_speed, height),
        )?;
        if Ws < 2.0 {
            Ws = 2.0;
        }
        let (material_number, id) = material_identity(self.materials, o, construction)?;
        let mat = self
            .materials
            .source_roof_vegetation_material(material_number, id)?;
        let RoughSurf = mat.roughness.ok_or(Error::Unavailable(
            "CalcEcoRoof actual outside Material roughness",
        ))?;
        let AbsThermSurf = mat.thermal_absorptance;
        let HMovInsul = 0.0;
        let wind_exposed = o
            .geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(i))
            .ok_or(Error::Unavailable("CalcEcoRoof actual Surface ExtWind"))?
            .row
            .exterior_wind;
        if wind_exposed {
            let h = history_index(o, number)?;
            let temperature =
                *o.history.normal.outside_temperature[0]
                    .get(h)
                    .ok_or(Error::Unavailable(
                        "CalcEcoRoof actual outside history before convection",
                    ))?;
            o.convection.initialize_external(
                SourceExternalConvectionCall {
                    surface: number,
                    h_movable_insulation: HMovInsul,
                    roughness: RoughSurf,
                    absorptance: AbsThermSurf,
                    outside_temperature: temperature,
                },
                &mut SourceExternalConvectionOwners {
                    geometry: o.geometry,
                    zones: o.zones,
                    control: o.control,
                    arrays: &mut surface.arrays,
                    physical: o.physical,
                    environment: o.environment,
                    input: &*o.input,
                    reports: o.reports,
                    scan: o.scan,
                    backend: o.backend,
                },
            )?;
        }
        let actual = o
            .geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(i))
            .ok_or(Error::Unavailable(
                "CalcEcoRoof actual Surface view factors",
            ))?;
        let Latm = Sigma * actual.row.view_factor_ground * pow_4(o.environment.ground_temp_kelvin)
            + Sigma
                * actual.row.vertex_progress.view_factor_sky
                * pow_4(o.environment.current_weather.sky_temp_kelvin);
        if self.eco_roof.eco_roof_begin_flag {
            self.init_eco_roof_first_time(o, number, construction)?;
        }
        self.init_eco_roof(o, number, construction)?;
        if number == self.eco_roof.first_eco_surf {
            self.update_soil_props(o, construction)?;
            let flags = SourceDiagnosticFlags {
                kickoff: &o.globals.kickoff_simulation,
                developer: self.developer,
                warmup: &o.clock.warmup_flag,
                doing_sizing: &o.globals.doing_sizing,
            };
            let height = o
                .geometry
                .final_surfaces
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable(
                    "CalcEcoRoof actual Surface centroid before dry getter",
                ))?
                .centroid[2];
            let Ta = o
                .environment
                .outdoor_environment_parameters
                .point_dry_bulb_temperature_at_height(
                    o.environment.current_weather.out_dry_bulb_temp,
                    height,
                    o.backend,
                    &flags,
                )?;
            let e = &mut *self.eco_roof;
            e.tg = e.tgold;
            e.tf = e.tfold;
            let usage = o
                .control
                .construction_usage
                .get(index(construction)?)
                .ok_or(Error::Unavailable(
                    "CalcEcoRoof actual Construction coefficient identity",
                ))?;
            let id = usage
                .construction_id
                .ok_or(Error::Unavailable("CalcEcoRoof genuine ConstructionId"))?;
            let c = &o
                .public
                .constructions
                .iter()
                .find(|row| row.construction_id == id)
                .ok_or(Error::Unavailable(
                    "CalcEcoRoof same actual public CTF Construction",
                ))?
                .result
                .as_ref()
                .map_err(|_| Error::Unavailable("CalcEcoRoof actual returned public CTF"))?
                .storage;
            let F1temp: f64;
            let Qsoilpart1: f64;
            if c.cross[0] > 0.01 {
                e.quick_conduction_surf = true;
                let spaceNum = o
                    .geometry
                    .final_surfaces
                    .as_ref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("CalcEcoRoof actual Surface spaceNum"))?
                    .row
                    .space;
                F1temp = c.cross[0]
                    / (c.inside[0]
                        + value(
                            &surface.arrays.surf_h_conv_int,
                            i,
                            "CalcEcoRoof SurfHConvInt",
                        )?);
                Qsoilpart1 = -value(
                    &surface.arrays.surf_ctf_const_out_part,
                    i,
                    "CalcEcoRoof SurfCTFConstOutPart",
                )? + F1temp
                    * (value(
                        &surface.arrays.surf_ctf_const_in_part,
                        i,
                        "CalcEcoRoof SurfCTFConstInPart",
                    )? + value(
                        &surface.arrays.surf_opaq_q_rad_sw_in_abs,
                        i,
                        "CalcEcoRoof SurfOpaqQRadSWInAbs",
                    )? + value(
                        &surface.arrays.surf_qdot_rad_int_gains_in_per_area,
                        i,
                        "CalcEcoRoof SurfQdotRadIntGainsInPerArea",
                    )? + c.source_in[0]
                        * surface
                            .arrays
                            .surf_qsrc_hist
                            .as_ref()
                            .and_then(|rows| rows.get(i))
                            .and_then(|row| row.first())
                            .copied()
                            .ok_or(Error::Unavailable(
                                "CalcEcoRoof actual SurfQsrcHist(SurfNum,1); no zero substitute",
                            ))?
                        + value(
                            &surface.arrays.surf_h_conv_int,
                            i,
                            "CalcEcoRoof SurfHConvInt",
                        )? * *o.air.source_space_mat(spaceNum)?
                        + value(
                            &surface.arrays.surf_qdot_rad_net_lw_in_per_area,
                            i,
                            "CalcEcoRoof SurfQdotRadNetLWInPerArea",
                        )?);
            } else {
                Qsoilpart1 = -value(
                    &surface.arrays.surf_ctf_const_out_part,
                    i,
                    "CalcEcoRoof SurfCTFConstOutPart",
                )? + c.cross[0]
                    * value(&surface.arrays.surf_temp_in, i, "CalcEcoRoof SurfTempIn")?;
                F1temp = 0.0;
            }
            let Qsoilpart2 = c.outside[0] - F1temp * c.cross[0];
            e.pa = o.environment.std_baro_press;
            let Tgk = e.tg + Kelvin;
            let Tak = Ta + Kelvin;

            let sigmaf = 0.9 - 0.7 * math("CalcEcoRoof exp", ep_platform_math::exp(-0.75 * e.lai))?;

            let EpsilonOne = e.epsilonf + e.epsilong - e.epsilong * e.epsilonf;
            e.rh = o.environment.current_weather.out_rel_hum;
            let eair = (e.rh / 100.0)
                * 611.2
                * math(
                    "CalcEcoRoof exp",
                    ep_platform_math::exp(17.67 * Ta / (Tak - 29.65)),
                )?;
            let qa = (0.622 * eair) / (e.pa - 1.000 * eair);
            let Rhoa = e.pa / (Rair * Tak);
            let Tif = e.tf;

            let Tafk =
                (1.0 - sigmaf) * Tak + sigmaf * (0.3 * Tak + 0.6 * (Tif + Kelvin) + 0.1 * Tgk);

            let Taf = Tafk - Kelvin;
            let Rhof = e.pa / (Rair * Tafk);
            let Rhoaf = (Rhoa + Rhof) / 2.0;
            let Zd = 0.701 * math("CalcEcoRoof pow", ep_platform_math::pow(e.zf, 0.979))?;
            let mut Zo = 0.131 * math("CalcEcoRoof pow", ep_platform_math::pow(e.zf, 0.997))?;
            if Zo < 0.02 {
                Zo = 0.02;
            }

            let Cfhn =
                pow_2(Kv / math("CalcEcoRoof log", ep_platform_math::log((e.za - Zd) / Zo))?);
            let Waf = 0.83 * math("CalcEcoRoof sqrt", ep_platform_math::sqrt(Cfhn))? * sigmaf * Ws
                + (1.0 - sigmaf) * Ws;
            let Cf = 0.01 * (1.0 + 0.3 / Waf);
            e.sheatf = e.e0 + 1.1 * e.lai * Rhoaf * Cpa * Cf * Waf;
            e.sensiblef = e.sheatf * (Taf - e.tf);

            let esf = 611.2
                * math(
                    "CalcEcoRoof exp",
                    ep_platform_math::exp(17.67 * Tif / (Tif + Kelvin - 29.65)),
                )?;

            let qsf = 0.622 * esf / (e.pa - 1.000 * esf);

            let ra = 1.0 / (Cf * Waf);

            let (RS, f1) = calculate_eco_roof_solar(o, number)?;
            let f2inv: f64;
            if e.moisture_max == e.moisture_residual {
                f2inv = 1.0e10;
            } else {
                f2inv = (e.mean_root_moisture - e.moisture_residual)
                    / (e.moisture_max - e.moisture_residual);
            }

            let f2 = 1.0 / f2inv;

            e.f3 = 1.0
                / (math(
                    "CalcEcoRoof exp",
                    ep_platform_math::exp(-0.0 * (esf - eair)),
                )?);

            let r_s = e.stomatal_resistance_min * f1 * f2 * e.f3 / e.lai;
            let rn = ra / (ra + r_s);

            let Mg = e.moisture / e.moisture_max;
            let dOne = 1.0 - sigmaf * (0.6 * (1.0 - rn) + 0.1 * (1.0 - Mg));

            let mut Lef = 1.91846e6 * pow_2((Tif + Kelvin) / (Tif + Kelvin - 33.91));

            if e.tfold < 0.0 {
                Lef = 2.838e6;
            }

            let Desf = 611.2
                * math(
                    "CalcEcoRoof exp",
                    ep_platform_math::exp(17.67 * (e.tf / (e.tf + Kelvin - 29.65))),
                )?
                * (17.67
                    * e.tf
                    * (-1.0)
                    * math(
                        "CalcEcoRoof pow",
                        ep_platform_math::pow(e.tf + Kelvin - 29.65, -2.0),
                    )?
                    + 17.67 / (Kelvin - 29.65 + e.tf));
            let dqf = ((0.622 * e.pa) / pow_2(e.pa - esf)) * Desf;
            let esg = 611.2
                * math(
                    "CalcEcoRoof exp",
                    ep_platform_math::exp(17.67 * (e.tg / ((e.tg + Kelvin) - 29.65))),
                )?;

            let qsg = 0.622 * esg / (e.pa - esg);

            let mut Leg = 1.91846e6 * pow_2(Tgk / (Tgk - 33.91));

            if e.tgold < 0.0 {
                Leg = 2.838e6;
            }

            let Desg = 611.2
                * math(
                    "CalcEcoRoof exp",
                    ep_platform_math::exp(17.67 * (e.tg / (e.tg + Kelvin - 29.65))),
                )?
                * (17.67
                    * e.tg
                    * (-1.0)
                    * math(
                        "CalcEcoRoof pow",
                        ep_platform_math::pow(e.tg + Kelvin - 29.65, -2.0),
                    )?
                    + 17.67 / (Kelvin - 29.65 + e.tg));
            let dqg = (0.622 * e.pa / pow_2(e.pa - esg)) * Desg;

            let Rhog = e.pa / (Rair * Tgk);

            let Rhoag = (Rhoa + Rhog) / 2.0;
            let mut Rib = 2.0 * g1 * e.za * (Taf - e.tg) / ((Tafk + Tgk) * pow_2(Waf));

            let Gammah: f64;
            if Rib < 0.0 {
                Gammah = math(
                    "CalcEcoRoof pow",
                    ep_platform_math::pow(1.0 - 16.0 * Rib, -0.5),
                )?;
            } else {
                if Rib >= 0.19 {
                    Rib = 0.19;
                }
                Gammah = math(
                    "CalcEcoRoof pow",
                    ep_platform_math::pow(1.0 - 5.0 * Rib, -0.5),
                )?;
            }

            e.zog = match RoughSurf {
                ep_model::MaterialSurfaceRoughness::VeryRough => 0.005,
                ep_model::MaterialSurfaceRoughness::Rough => 0.0030,
                ep_model::MaterialSurfaceRoughness::MediumRough => 0.0020,
                ep_model::MaterialSurfaceRoughness::MediumSmooth => 0.0015,
                ep_model::MaterialSurfaceRoughness::Smooth => 0.0010,
                ep_model::MaterialSurfaceRoughness::VerySmooth => 0.0008,
            };

            let Chng =
                pow_2(Kv / math("CalcEcoRoof log", ep_platform_math::log(e.za / e.zog))?) / rch;
            let Chg = Gammah * ((1.0 - sigmaf) * Chng + sigmaf * Cfhn);
            e.sheatg = e.e0 + Rhoag * Cpa * Chg * Waf;
            e.sensibleg = e.sheatg * (Taf - e.tg);

            let Chne =
                pow_2(Kv / math("CalcEcoRoof log", ep_platform_math::log(e.za / e.zog))?) / rche;
            let Ce = Gammah * ((1.0 - sigmaf) * Chne + sigmaf * Cfhn);

            let qaf = ((1.0 - sigmaf) * qa + sigmaf * (0.3 * qa + 0.6 * qsf * rn + 0.1 * qsg * Mg))
                / (1.0 - sigmaf * (0.6 * (1.0 - rn) + 0.1 * (1.0 - Mg)));
            let qg = Mg * qsg + (1.0 - Mg) * qaf;

            e.lf = Lef * e.lai * Rhoaf * Cf * Waf * rn * (qaf - qsf);
            e.lg = Ce * Leg * Waf * Rhoag * (qaf - qg) * Mg;

            e.vfluxf = -1.0 * e.lf / Lef / 990.0;
            e.vfluxg = -1.0 * e.lg / Leg / 990.0;
            if e.vfluxf < 0.0 {
                e.vfluxf = 0.0;
            }
            if e.vfluxg < 0.0 {
                e.vfluxg = 0.0;
            }

            let mut LeafTK = e.tf + Kelvin;
            let mut SoilTK = e.tg + Kelvin;

            for _EcoLoop in 1..=3 {
                let P1 = sigmaf * (RS * (1.0 - e.alphaf) + e.epsilonf * Latm)
                    - 3.0 * sigmaf * e.epsilonf * e.epsilong * Sigma * pow_4(SoilTK) / EpsilonOne
                    - 3.0
                        * (-sigmaf * e.epsilonf * Sigma
                            - sigmaf * e.epsilonf * e.epsilong * Sigma / EpsilonOne)
                        * pow_4(LeafTK)
                    + e.sheatf * (1.0 - 0.7 * sigmaf) * (Ta + Kelvin)
                    + e.lai * Rhoaf * Cf * Lef * Waf * rn * ((1.0 - 0.7 * sigmaf) / dOne) * qa
                    + e.lai
                        * Rhoaf
                        * Cf
                        * Lef
                        * Waf
                        * rn
                        * (((0.6 * sigmaf * rn) / dOne) - 1.0)
                        * (qsf - LeafTK * dqf)
                    + e.lai
                        * Rhoaf
                        * Cf
                        * Lef
                        * Waf
                        * rn
                        * ((0.1 * sigmaf * Mg) / dOne)
                        * (qsg - SoilTK * dqg);
                let P2 = 4.0 * (sigmaf * e.epsilonf * e.epsilong * Sigma) * pow_3(SoilTK)
                    / EpsilonOne
                    + 0.1 * sigmaf * e.sheatf
                    + e.lai * Rhoaf * Cf * Lef * Waf * rn * (0.1 * sigmaf * Mg) / dOne * dqg;
                let P3 = 4.0
                    * (-sigmaf * e.epsilonf * Sigma
                        - (sigmaf * e.epsilonf * Sigma * e.epsilong) / EpsilonOne)
                    * pow_3(LeafTK)
                    + (0.6 * sigmaf - 1.0) * e.sheatf
                    + e.lai
                        * Rhoaf
                        * Cf
                        * Lef
                        * Waf
                        * rn
                        * (((0.6 * sigmaf * rn) / dOne) - 1.0)
                        * dqf;

                let T1G = (1.0 - sigmaf) * (RS * (1.0 - e.alphag) + e.epsilong * Latm)
                    - (3.0 * (sigmaf * e.epsilonf * e.epsilong * Sigma) / EpsilonOne)
                        * pow_4(LeafTK)
                    - 3.0
                        * (-(1.0 - sigmaf) * e.epsilong * Sigma
                            - sigmaf * e.epsilonf * e.epsilong * Sigma / EpsilonOne)
                        * pow_4(SoilTK)
                    + e.sheatg * (1.0 - 0.7 * sigmaf) * (Ta + Kelvin)
                    + Rhoag * Ce * Leg * Waf * Mg * ((1.0 - 0.7 * sigmaf) / dOne) * qa
                    + Rhoag
                        * Ce
                        * Leg
                        * Waf
                        * Mg
                        * (0.1 * sigmaf * Mg / dOne - Mg)
                        * (qsg - SoilTK * dqg)
                    + Rhoag
                        * Ce
                        * Leg
                        * Waf
                        * Mg
                        * (0.6 * sigmaf * rn / dOne)
                        * (qsf - LeafTK * dqf)
                    + Qsoilpart1
                    + Qsoilpart2 * (Kelvin);

                let T2G = 4.0
                    * (-(1.0 - sigmaf) * e.epsilong * Sigma
                        - sigmaf * e.epsilonf * e.epsilong * Sigma / EpsilonOne)
                    * pow_3(SoilTK)
                    + (0.1 * sigmaf - 1.0) * e.sheatg
                    + Rhoag * Ce * Leg * Waf * Mg * (0.1 * sigmaf * Mg / dOne - Mg) * dqg
                    - Qsoilpart2;

                let T3G = (4.0 * (sigmaf * e.epsilong * e.epsilonf * Sigma) / EpsilonOne)
                    * pow_3(LeafTK)
                    + 0.6 * sigmaf * e.sheatg
                    + Rhoag * Ce * Leg * Waf * Mg * (0.6 * sigmaf * rn / dOne) * dqf;

                LeafTK = 0.5 * (LeafTK + (P1 * T2G - P2 * T1G) / (-P3 * T2G + T3G * P2));
                SoilTK = 0.5 * (SoilTK + (P1 * T3G - P3 * T1G) / (-P2 * T3G + P3 * T2G));
            }
            e.qsoil = -1.0 * (Qsoilpart1 - Qsoilpart2 * (SoilTK - Kelvin));
            e.tfold = LeafTK - Kelvin;
            e.tgold = SoilTK - Kelvin;
        }
        let temperature = self.eco_roof.tgold;
        let h = history_index(o, number)?;
        *o.history.normal.outside_temperature[0]
            .get_mut(h)
            .ok_or(Error::Unavailable(
                "CalcEcoRoof actual SurfOutsideTempHist(1) write",
            ))? = temperature;
        Ok(temperature)
    }

    /// EcoRoofManager725-1194; the same Material K/rho/Cp are the only wet property writes.
    #[expect(
        clippy::manual_clamp,
        clippy::if_same_then_else,
        reason = "Preserve Native EcoRoofManager725-1194 sequential bounds and irrigation branch writes."
    )]
    pub(crate) fn update_soil_props(
        &mut self,
        o: &mut SourceOutsideCtfOwners<'_, '_>,
        construction: i32,
    ) -> Result<(), Error> {
        // Native const-value formal arguments are captured at this real call.
        let MoistureMax = self.eco_roof.moisture_max;
        let MoistureResidual = self.eco_roof.moisture_residual;
        let SoilThickness = self.eco_roof.soil_thickness;
        let Vfluxf = self.eco_roof.vfluxf;
        let Vfluxg = self.eco_roof.vfluxg;
        static DEPTH_FACTOR: OnceLock<Result<f64, MathProviderError>> = OnceLock::new();
        let depth_fac = math(
            "UpdateSoilProps depth_fac pow",
            *DEPTH_FACTOR.get_or_init(|| {
                ep_platform_math::pow(2.0, -2.3).map(|power| (161240.0 * power) / 60.0)
            }),
        )?;
        const alpha: f64 = 23.0;
        const n: f64 = 1.27;
        const lambda: f64 = 0.5;
        const SoilConductivitySaturation: f64 = 5.157e-7;
        let minutes = o.clock.minutes_in_time_step;
        let minutes_float = f64::from(minutes);
        let RatioMax = 1.0 + 0.20 * minutes_float / 15.0;
        let RatioMin = 1.0 - 0.20 * minutes_float / 15.0;
        let (material_number, id) = material_identity(self.materials, o, construction)?;
        let mat = self
            .materials
            .source_roof_vegetation_material_mut(material_number, id)?;
        let e = &mut *self.eco_roof;
        let water = &mut self.water.data;
        if e.update_begin_flag {
            e.dry_cond = mat.conductivity;
            e.dry_dens = mat.density;
            e.dry_absorp = mat.solar_absorptance;
            e.dry_spec_heat = mat.specific_heat;
            if SoilThickness > 0.12 {
                e.top_depth = 0.06;
            } else {
                e.top_depth = 0.5 * SoilThickness;
            }
            if mat.calc_method == 1 {
                // RootDepth is deliberately its old value here; ctor0 on first entry.
                let depth_limit = depth_fac
                    * math(
                        "UpdateSoilProps depth_limit pow",
                        ep_platform_math::pow(e.top_depth + e.root_depth, 2.07),
                    )?;
                let mut index1 = 21_i32;
                for candidate in 1_i32..=20 {
                    // Preserve C++ int division BEFORE conversion to double.
                    if f64::from(minutes / candidate) <= depth_limit {
                        index1 = candidate;
                        break;
                    }
                }
                if index1 > 1 {
                    let flags = SourceDiagnosticFlags {
                        kickoff: &o.globals.kickoff_simulation,
                        developer: self.developer,
                        warmup: &o.clock.warmup_flag,
                        doing_sizing: &o.globals.doing_sizing,
                    };
                    o.backend.show_source_warning_error(
                        "CalcEcoRoof: Too few time steps per hour for stability.",
                        &flags,
                    )?;
                    // C++ integer expression inside ceil, retained before formatting.
                    let required = 60_i32
                        .checked_mul(index1)
                        .and_then(|v| v.checked_div(minutes))
                        .ok_or(Error::Unavailable(
                            "UpdateSoilProps defined integer stability division",
                        ))?;
                    if f64::from(required).ceil() <= 60.0 {
                        o.backend.show_continue_error(&format!(
                            "...Entered Timesteps per hour=[{}], Change to some value greater than or equal to [{}] for assured stability.",
                            o.clock.time_steps_in_hour, required,
                        ))?;
                        o.backend.show_continue_error(
                            "...Note that EnergyPlus has a maximum of 60 timesteps per hour",
                        )?;
                        o.backend.show_continue_error("...The program will continue, but if the simulation fails due to too low/high temperatures, instability here could be the reason.")?;
                    } else {
                        o.backend.show_continue_error(&format!(
                            "...Entered Timesteps per hour=[{}], however the required frequency for stability [{}] is over the EnergyPlus maximum of 60.",
                            o.clock.time_steps_in_hour, required,
                        ))?;
                        o.backend.show_continue_error("...Consider using the simple moisture diffusion calculation method for this application")?;
                        o.backend.show_continue_error("...The program will continue, but if the simulation fails due to too low/high temperatures, instability here could be the reason.")?;
                    }
                }
            }
            e.root_depth = SoilThickness - e.top_depth;
            e.time_step_zone_sec = minutes_float * 60.0;
            e.update_begin_flag = false;
        }
        e.current_runoff = 0.0;

        e.moisture -= (Vfluxg) * minutes_float * 60.0 / e.top_depth;
        e.mean_root_moisture -= (Vfluxf) * minutes_float * 60.0 / e.root_depth;

        e.current_et = (Vfluxg + Vfluxf) * minutes_float * 60.0;
        if !o.clock.warmup_flag {
            e.cum_et += e.current_et;
        }

        e.moisture += e.current_precipitation / e.top_depth;

        if !o.clock.warmup_flag {
            e.cum_precip += e.current_precipitation;
        }

        e.current_irrigation = 0.0;
        water.irrigation.actual_amount = 0.0;
        if water.irrigation.mode == SourceIrrigationMode::SchedDesign {
            e.current_irrigation = water.irrigation.scheduled_amount;
            water.irrigation.actual_amount = e.current_irrigation;
        } else if water.irrigation.mode == SourceIrrigationMode::SmartSched
            && e.moisture < water.irrigation.threshold * MoistureMax
        {
            e.current_irrigation = water.irrigation.scheduled_amount;
            water.irrigation.actual_amount = e.current_irrigation;
        } else {
            if water.rainfall.mode == SourceRainfallMode::EpwPrecipitation {
                e.current_irrigation = 0.0;
                water.irrigation.actual_amount = e.current_irrigation;
            }
        }

        e.moisture += e.current_irrigation / e.top_depth;

        if (o.environment.run_period_environment) && (!o.clock.warmup_flag) {
            e.cum_irrigation += e.current_irrigation;

            let month = index(o.environment.month)?;
            *e.monthly_irrigation
                .get_mut(month)
                .ok_or(Error::Unavailable(
                    "UpdateSoilProps actual MonthlyIrrigation month",
                ))? += water.irrigation.actual_amount * 1000.0;
        }

        if e.current_irrigation + e.current_precipitation > 0.5 * 0.0254 * minutes_float / 60.0 {
            e.current_runoff = e.current_irrigation + e.current_precipitation
                - (0.5 * 0.0254 * minutes_float / 60.0);

            e.moisture -= e.current_runoff / e.top_depth;
        }

        if e.moisture > MoistureMax {
            e.current_runoff += (e.moisture - MoistureMax) * e.top_depth;
            e.moisture = MoistureMax;
        }

        if mat.calc_method == 0 {
            if e.moisture > e.mean_root_moisture {
                let mut MoistureDiffusion = cpp_min(
                    (MoistureMax - e.mean_root_moisture) * e.root_depth,
                    (e.moisture - e.mean_root_moisture) * e.top_depth,
                );
                MoistureDiffusion = cpp_max(0.0, MoistureDiffusion);

                MoistureDiffusion *= 0.00005 * minutes_float * 60.0;
                e.moisture -= MoistureDiffusion / e.top_depth;
                e.mean_root_moisture += MoistureDiffusion / e.root_depth;
            } else if e.mean_root_moisture > e.moisture {
                let mut MoistureDiffusion = cpp_min(
                    (MoistureMax - e.moisture) * e.top_depth,
                    (e.mean_root_moisture - e.moisture) * e.root_depth,
                );
                MoistureDiffusion = cpp_max(0.0, MoistureDiffusion);

                MoistureDiffusion *= 0.00001 * minutes_float * 60.0;
                e.moisture += MoistureDiffusion / e.top_depth;
                e.mean_root_moisture -= MoistureDiffusion / e.root_depth;
            }
        } else {
            e.relative_soil_saturation_top =
                (e.moisture - MoistureResidual) / (MoistureMax - MoistureResidual);
            if e.relative_soil_saturation_top < 0.0001 {
                if e.err_index == 0 {
                    let printed =
                        super::source_round_format::round(e.relative_soil_saturation_top, 5)
                            .map_err(|error| {
                                Error::Format(format!(
                                    "EcoRoof UpdateSoilProps Native .5R: {error:?}"
                                ))
                            })?;
                    o.backend.show_warning_message(&format!(
                        "EcoRoof: UpdateSoilProps: Relative Soil Saturation Top Moisture <= 0.0001, Value=[{printed}]."
                    ))?;
                    o.backend
                        .show_continue_error("Value is set to 0.0001 and simulation continues.")?;
                    o.backend.show_continue_error("You may wish to increase the number of timesteps to attempt to alleviate the problem.")?;
                }
                let relative = e.relative_soil_saturation_top;
                o.backend.show_source_recurring_warning_at_end(
                    "EcoRoof: UpdateSoilProps: Relative Soil Saturation Top Moisture < 0. continues",
                    &mut e.err_index, self.recurring_errors, o.clock, o.globals,
                    Some(relative), Some(relative), None, "", "", "",
                )?;
                e.relative_soil_saturation_top = 0.0001;
            }
            e.soil_hydro_conductivity_top = SoilConductivitySaturation
                * math(
                    "UpdateSoilProps pow",
                    ep_platform_math::pow(e.relative_soil_saturation_top, lambda),
                )?
                * pow_2(
                    1.0 - math(
                        "UpdateSoilProps pow",
                        ep_platform_math::pow(
                            1.0 - math(
                                "UpdateSoilProps pow",
                                ep_platform_math::pow(
                                    e.relative_soil_saturation_top,
                                    n / (n - 1.0),
                                ),
                            )?,
                            (n - 1.0) / n,
                        ),
                    )?,
                );
            e.capillary_potential_top = (-1.0 / alpha)
                * math(
                    "UpdateSoilProps pow",
                    ep_platform_math::pow(
                        math(
                            "UpdateSoilProps pow",
                            ep_platform_math::pow(
                                1.0 / e.relative_soil_saturation_top,
                                n / (n - 1.0),
                            ),
                        )? - 1.0,
                        1.0 / n,
                    ),
                )?;

            e.relative_soil_saturation_root =
                (e.mean_root_moisture - MoistureResidual) / (MoistureMax - MoistureResidual);
            e.soil_hydro_conductivity_root = SoilConductivitySaturation
                * math(
                    "UpdateSoilProps pow",
                    ep_platform_math::pow(e.relative_soil_saturation_root, lambda),
                )?
                * pow_2(
                    1.0 - math(
                        "UpdateSoilProps pow",
                        ep_platform_math::pow(
                            1.0 - math(
                                "UpdateSoilProps pow",
                                ep_platform_math::pow(
                                    e.relative_soil_saturation_root,
                                    n / (n - 1.0),
                                ),
                            )?,
                            (n - 1.0) / n,
                        ),
                    )?,
                );
            e.capillary_potential_root = (-1.0 / alpha)
                * math(
                    "UpdateSoilProps pow",
                    ep_platform_math::pow(
                        math(
                            "UpdateSoilProps pow",
                            ep_platform_math::pow(
                                1.0 / e.relative_soil_saturation_root,
                                n / (n - 1.0),
                            ),
                        )? - 1.0,
                        1.0 / n,
                    ),
                )?;

            e.soil_conductivity_ave_top =
                (e.soil_hydro_conductivity_top + e.soil_hydro_conductivity_root) * 0.5;
            e.moisture += (e.time_step_zone_sec / e.top_depth)
                * ((e.soil_conductivity_ave_top
                    * (e.capillary_potential_top - e.capillary_potential_root)
                    / e.top_depth)
                    - e.soil_conductivity_ave_top);

            if e.moisture >= MoistureMax {
                e.moisture = 0.9999 * MoistureMax;
                e.current_runoff += (e.moisture - MoistureMax * 0.9999) * e.top_depth;
            }

            if e.moisture <= (1.01 * MoistureResidual) {
                e.moisture = 1.01 * MoistureResidual;
            }

            e.soil_conductivity_ave_root = e.soil_hydro_conductivity_root;

            if (e.soil_conductivity_ave_root * 3600.0) <= (2.33e-7) {
                e.soil_conductivity_ave_root = 0.0;
            }

            e.test_moisture = e.mean_root_moisture;
            e.mean_root_moisture += (e.time_step_zone_sec / e.root_depth)
                * ((e.soil_conductivity_ave_top
                    * (e.capillary_potential_top - e.capillary_potential_root)
                    / e.root_depth)
                    + e.soil_conductivity_ave_top
                    - e.soil_conductivity_ave_root);

            if e.mean_root_moisture >= MoistureMax {
                e.mean_root_moisture = 0.9999 * MoistureMax;
                e.current_runoff += (e.moisture - MoistureMax * 0.9999) * e.root_depth;
            }

            if e.mean_root_moisture <= (1.01 * MoistureResidual) {
                e.mean_root_moisture = 1.01 * MoistureResidual;
            }

            e.current_runoff += e.soil_conductivity_ave_root * e.time_step_zone_sec;
        }

        if !o.clock.warmup_flag {
            e.cum_runoff += e.current_runoff;
        }

        if e.mean_root_moisture <= MoistureResidual * 1.00001 {
            e.moisture -=
                (MoistureResidual * 1.00001 - e.mean_root_moisture) * e.root_depth / e.top_depth;

            if e.moisture < MoistureResidual * 1.00001 {
                e.moisture = MoistureResidual * 1.00001;
            }
            e.mean_root_moisture = MoistureResidual * 1.00001;
        }

        let mut SoilAbsorpSolar = e.dry_absorp
            + (0.92 - e.dry_absorp) * (e.moisture - MoistureResidual)
                / (MoistureMax - MoistureResidual);

        if SoilAbsorpSolar > 0.95 {
            SoilAbsorpSolar = 0.95;
        }

        if SoilAbsorpSolar < 0.20 {
            SoilAbsorpSolar = 0.20;
        }

        let mut TestRatio = (1.0 - SoilAbsorpSolar) / e.alphag;
        if TestRatio > RatioMax {
            TestRatio = RatioMax;
        }
        if TestRatio < RatioMin {
            TestRatio = RatioMin;
        }
        e.alphag *= TestRatio;

        let AvgMoisture =
            (e.root_depth * e.mean_root_moisture + e.top_depth * e.moisture) / SoilThickness;
        let mut SoilDensity = e.dry_dens + (AvgMoisture - MoistureResidual) * 990.0;

        let mut SoilSpecHeat = e.dry_spec_heat + 1900.0 * AvgMoisture;

        let SatRatio = (AvgMoisture - MoistureResidual) / (MoistureMax - MoistureResidual);
        let mut SoilConductivity = (e.dry_cond / 1.15)
            * (1.45
                * math(
                    "UpdateSoilProps exp",
                    ep_platform_math::exp(4.411 * SatRatio),
                )?)
            / (1.0
                + 0.45
                    * math(
                        "UpdateSoilProps exp",
                        ep_platform_math::exp(4.411 * SatRatio),
                    )?);

        TestRatio = SoilConductivity / mat.conductivity;
        if TestRatio > RatioMax {
            TestRatio = RatioMax;
        }
        if TestRatio < RatioMin {
            TestRatio = RatioMin;
        }
        mat.conductivity *= TestRatio;
        SoilConductivity = mat.conductivity;

        TestRatio = SoilDensity / mat.density;
        if TestRatio > RatioMax {
            TestRatio = RatioMax;
        }
        if TestRatio < RatioMin {
            TestRatio = RatioMin;
        }
        mat.density *= TestRatio;
        SoilDensity = mat.density;

        TestRatio = SoilSpecHeat / mat.specific_heat;
        if TestRatio > RatioMax {
            TestRatio = RatioMax;
        }
        if TestRatio < RatioMin {
            TestRatio = RatioMin;
        }
        mat.specific_heat *= TestRatio;
        SoilSpecHeat = mat.specific_heat;

        // Retain the three final Native local reads used only by its commented diagnostics.
        let _ = (SoilConductivity, SoilDensity, SoilSpecHeat);
        // Native1183 InitConductionTransferFunctions call is commented: no rebuild.
        Ok(())
    }
}

/// EcoRoofManager1196-1209 intentionally assumes the one shared roof is horizontal.
pub(crate) fn calculate_eco_roof_solar(
    o: &SourceOutsideCtfOwners<'_, '_>,
    number: i32,
) -> Result<(f64, f64), Error> {
    let ordinal = usize::try_from(number)
        .ok()
        .filter(|&n| n > 0)
        .ok_or(Error::Unavailable(
            "CalculateEcoRoofSolar actual Source Surface ordinal",
        ))?;
    let RS = cpp_max(
        o.environment.current_weather.solar_direction_cosines[2],
        0.0,
    ) * o.environment.current_weather.beam_solar_rad
        + o.radiation
            .solar_timestep
            .shading
            .surf_aniso_sky_mult
            .read([ordinal])?
            * o.environment.current_weather.dif_solar_rad;
    let f1inv = cpp_min(1.0, (0.004 * RS + 0.005) / (0.81 * (0.004 * RS + 1.0)));
    let f1 = 1.0 / f1inv;
    Ok((RS, f1))
}
