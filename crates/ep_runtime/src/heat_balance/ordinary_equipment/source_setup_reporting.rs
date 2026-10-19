//! Genuine SetupNodeVarsForReporting188-514 on the sole full NodeOwner.
use super::{OrdinaryEquipment, source_nodes::SourceNodeFluidType};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::{OrdinaryRadiationError, OrdinaryRadiationUnavailable};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{HeatBalanceInputError as Error, SelectedHeatBalanceInputState};
use crate::heat_balance::source_output_registration::{
    SourceNodeOutputField as Field, SourceNodeReportingOutputContext,
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputStoreType,
    SourceOutputUnits as Units, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputOwners, SourceRealOutputTarget as Target,
    SourceRealRegistrationOptions, SourceReportVariableInput, TimeStepKind,
};
use std::io::Write;

/// Actual DataLoopNode.hh502–513 constructor record. The zero allocation has no rows.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceMoreNodeData {
    pub(super) relative_humidity: f64,
    pub(super) report_enthalpy: f64,
    pub(super) volume_flow_rate_standard_density: f64,
    pub(super) volume_flow_rate_current_density: f64,
    pub(super) wet_bulb_temperature: f64,
    pub(super) density: f64,
    pub(super) air_dew_point_temperature: f64,
    pub(super) specific_heat: f64,
}

impl SourceMoreNodeData {
    /// Native HVACManager2186-2194 deliberately leaves dewpoint/specific heat.
    pub(super) fn reset_source_node_data(&mut self, default_temperature: f64, default_enthalpy: f64) {
        self.wet_bulb_temperature = default_temperature;
        self.relative_humidity = 0.0;
        self.report_enthalpy = default_enthalpy;
        self.volume_flow_rate_standard_density = 0.0;
        self.volume_flow_rate_current_density = 0.0;
        self.density = 0.0;
    }
}

fn index(number: i32) -> Result<usize, OrdinaryRadiationError> {
    number.checked_sub(1).and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
}
impl SourceRealOutputOwners for OrdinaryEquipment {
    fn real_output(&self, target: Target) -> Result<&f64, OrdinaryRadiationError> {
        if let Target::PurchasedAir { field, index: number } = target {
            let row = self.purchased_air.units.as_ref().and_then(|rows| rows.get(index(number).ok()?))
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
            return Ok(match field {
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SenHeatEnergy => &row.sen_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::LatHeatEnergy => &row.lat_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotHeatEnergy => &row.tot_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SenCoolEnergy => &row.sen_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::LatCoolEnergy => &row.lat_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotCoolEnergy => &row.tot_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotHeatFuelEnergy => &row.tot_heat_fuel_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotCoolFuelEnergy => &row.tot_cool_fuel_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneSenHeatEnergy => &row.zone_sen_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneLatHeatEnergy => &row.zone_lat_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotHeatEnergy => &row.zone_tot_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneSenCoolEnergy => &row.zone_sen_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneLatCoolEnergy => &row.zone_lat_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotCoolEnergy => &row.zone_tot_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OASenHeatEnergy => &row.oa_sen_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OALatHeatEnergy => &row.oa_lat_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OATotHeatEnergy => &row.oa_tot_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OASenCoolEnergy => &row.oa_sen_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OALatCoolEnergy => &row.oa_lat_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OATotCoolEnergy => &row.oa_tot_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecSenHeatEnergy => &row.ht_rec_sen_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecLatHeatEnergy => &row.ht_rec_lat_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecTotHeatEnergy => &row.ht_rec_tot_heat_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecSenCoolEnergy => &row.ht_rec_sen_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecLatCoolEnergy => &row.ht_rec_lat_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecTotCoolEnergy => &row.ht_rec_tot_cool_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotHeatFuelEnergy => &row.zone_tot_heat_fuel_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotCoolFuelEnergy => &row.zone_tot_cool_fuel_energy,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SenHeatRate => &row.sen_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::LatHeatRate => &row.lat_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotHeatRate => &row.tot_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SenCoolRate => &row.sen_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::LatCoolRate => &row.lat_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotCoolRate => &row.tot_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotHeatFuelRate => &row.tot_heat_fuel_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TotCoolFuelRate => &row.tot_cool_fuel_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneSenHeatRate => &row.zone_sen_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneLatHeatRate => &row.zone_lat_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotHeatRate => &row.zone_tot_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneSenCoolRate => &row.zone_sen_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneLatCoolRate => &row.zone_lat_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotCoolRate => &row.zone_tot_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OASenHeatRate => &row.oa_sen_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OALatHeatRate => &row.oa_lat_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OATotHeatRate => &row.oa_tot_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OASenCoolRate => &row.oa_sen_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OALatCoolRate => &row.oa_lat_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OATotCoolRate => &row.oa_tot_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecSenHeatRate => &row.ht_rec_sen_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecLatHeatRate => &row.ht_rec_lat_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecTotHeatRate => &row.ht_rec_tot_heat_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecSenCoolRate => &row.ht_rec_sen_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecLatCoolRate => &row.ht_rec_lat_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::HtRecTotCoolRate => &row.ht_rec_tot_cool_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotHeatFuelRate => &row.zone_tot_heat_fuel_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::ZoneTotCoolFuelRate => &row.zone_tot_cool_fuel_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TimeEconoActive => &row.time_econo_active,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::TimeHtRecActive => &row.time_ht_rec_active,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OutdoorAirMassFlowRate => &row.outdoor_air_mass_flow_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::OutdoorAirVolFlowRateStdRho => &row.outdoor_air_vol_flow_rate_std_rho,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SupplyAirMassFlowRate => &row.supply_air_mass_flow_rate,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SupplyAirVolFlowRateStdRho => &row.supply_air_vol_flow_rate_std_rho,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SupplyTemp => &row.supply_temp,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::SupplyHumRat => &row.supply_hum_rat,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::MixedAirTemp => &row.mixed_air_temp,
                crate::heat_balance::source_output_registration::SourcePurchasedAirOutputField::MixedAirHumRat => &row.mixed_air_hum_rat,
            });
        }
        let Target::Node {field, index: number} = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        let i = index(number)?;
        match field {
            Field::Temperature |
            Field::MassFlowRate |
            Field::HumidityRatio |
            Field::TemperatureSetpoint |
            Field::TemperatureSetpointHigh |
            Field::TemperatureSetpointLow |
            Field::HumidityRatioSetpoint |
            Field::HumidityRatioMin |
            Field::HumidityRatioMax |
            Field::Pressure |
            Field::OutdoorAirWindSpeed |
            Field::OutdoorAirWindDirection |
            Field::Quality |
            Field::Height |
            Field::TemperatureMin |
            Field::TemperatureMax |
            Field::MassFlowRateMin |
            Field::MassFlowRateMax |
            Field::MassFlowRateMinAvailable |
            Field::MassFlowRateMaxAvailable |
            Field::MassFlowRateSetpoint |
            Field::MassFlowRateRequest |
            Field::TemperatureLastTimestep |
            Field::EnthalpyLastTimestep |
            Field::CarbonDioxide |
            Field::GenericContaminant => {
                let row = self.nodes.physical.as_ref().and_then(|rows| rows.get(i))
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                Ok(match field {
                    Field::Temperature => &row.temperature,
                    Field::MassFlowRate => &row.mass_flow_rate,
                    Field::HumidityRatio => &row.humidity_ratio,
                    Field::TemperatureSetpoint => &row.temperature_setpoint,
                    Field::TemperatureSetpointHigh => &row.temperature_setpoint_high,
                    Field::TemperatureSetpointLow => &row.temperature_setpoint_low,
                    Field::HumidityRatioSetpoint => &row.humidity_ratio_setpoint,
                    Field::HumidityRatioMin => &row.humidity_ratio_min,
                    Field::HumidityRatioMax => &row.humidity_ratio_max,
                    Field::Pressure => &row.pressure,
                    Field::OutdoorAirWindSpeed => &row.outdoor_air_wind_speed,
                    Field::OutdoorAirWindDirection => &row.outdoor_air_wind_direction,
                    Field::Quality => &row.quality,
                    Field::Height => &row.height,
                    Field::TemperatureMin => &row.temperature_min,
                    Field::TemperatureMax => &row.temperature_max,
                    Field::MassFlowRateMin => &row.mass_flow_rate_min,
                    Field::MassFlowRateMax => &row.mass_flow_rate_max,
                    Field::MassFlowRateMinAvailable => &row.mass_flow_rate_min_available,
                    Field::MassFlowRateMaxAvailable => &row.mass_flow_rate_max_available,
                    Field::MassFlowRateSetpoint => &row.mass_flow_rate_setpoint,
                    Field::MassFlowRateRequest => &row.mass_flow_rate_request,
                    Field::TemperatureLastTimestep => &row.temperature_last_timestep,
                    Field::EnthalpyLastTimestep => &row.enthalpy_last_timestep,
                    Field::CarbonDioxide => &row.carbon_dioxide,
                    Field::GenericContaminant => &row.generic_contaminant,
                    _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
                })
            }
            Field::RelativeHumidity |
            Field::VolumeFlowRateStandardDensity |
            Field::VolumeFlowRateCurrentDensity |
            Field::Density |
            Field::SpecificHeat |
            Field::ReportEnthalpy |
            Field::WetBulbTemperature |
            Field::AirDewPointTemperature => {
                let row = self.nodes.more_node_info.as_ref().and_then(|rows| rows.get(i))
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                Ok(match field {
                    Field::RelativeHumidity => &row.relative_humidity,
                    Field::VolumeFlowRateStandardDensity => &row.volume_flow_rate_standard_density,
                    Field::VolumeFlowRateCurrentDensity => &row.volume_flow_rate_current_density,
                    Field::Density => &row.density,
                    Field::SpecificHeat => &row.specific_heat,
                    Field::ReportEnthalpy => &row.report_enthalpy,
                    Field::WetBulbTemperature => &row.wet_bulb_temperature,
                    Field::AirDewPointTemperature => &row.air_dew_point_temperature,
                    _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
                })
            }
        }
    }
}

fn fluid_name(fluid: SourceNodeFluidType) -> Result<&'static str, Error> {
    match fluid {
        SourceNodeFluidType::Blank => Ok("blank"), SourceNodeFluidType::Air => Ok("Air"),
        SourceNodeFluidType::Water => Ok("Water"), SourceNodeFluidType::Steam => Ok("Steam"),
        SourceNodeFluidType::Electric => Ok("Electric"),
        _ => Err(Error::Unavailable("actual BND Node FluidTypeNames index")),
    }
}
impl OrdinaryEquipment {
    pub(crate) fn setup_source_node_variables_for_reporting<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &mut self, globals: &SelectedHeatBalanceGlobals,
        control: &SelectedHeatBalanceInputState, backend: &mut SourceErrorBackend<'_>,
        output: &mut SourceNodeReportingOutputContext<'_, H, D>, bnd: &mut dyn Write,
    ) -> Result<(), Error> {
        if !self.nodes.node_variables_setup {
            if !backend.state.tracking.abort_processing {
                self.nodes.more_node_info = Some(vec![SourceMoreNodeData::default(); self.nodes.number_of_unique_node_names]);
                for number in 1..=self.nodes.number_of_unique_node_names {
                    let node = self.nodes.physical.as_ref().and_then(|rows| rows.get(number - 1))
                        .ok_or(Error::Unavailable("actual SetupNodeVarsForReporting Node204"))?;
                    let node_id = self.nodes.node_ids.as_ref().and_then(|rows| rows.get(number))
                        .ok_or(Error::Unavailable("actual SetupNodeVarsForReporting NodeID205"))?;
                    let ordinal = i32::try_from(number).map_err(|_| Error::Unavailable("actual Node report signed ordinal"))?;
                    let mut register = |field, name, units| -> Result<(), Error> {
                        let binding = SourceRealOutputBinding::new(Target::Node {field, index: ordinal}, self)
                            .map_err(Error::Output)?;
                        let mut requests = SourceReportVariableInput::new(
                            output.reports, &mut *output.input, output.minimum_report_frequency,
                        );
                        output.registration.setup_bound_real(
                            &mut requests, output.prescan,
                            SourceRealOutputDeclaration {name, key: node_id, units, store_type: SourceOutputStoreType::Average},
                            output.minutes_in_time_step, &mut *output.host, &mut *output.eio,
                            binding, &mut *output.dictionary_host,
                            SourceRealRegistrationOptions::unmetered(TimeStepKind::System), backend,
                        ).map_err(Error::Output)
                    };
                    for (field, name, units) in [
                        (Field::Temperature, "System Node Temperature", Units::C),
                        (Field::MassFlowRate, "System Node Mass Flow Rate", Units::KgPerSecond),
                        (Field::HumidityRatio, "System Node Humidity Ratio", Units::KgWaterPerKgDryAir),
                        (Field::TemperatureSetpoint, "System Node Setpoint Temperature", Units::C),
                        (Field::TemperatureSetpointHigh, "System Node Setpoint High Temperature", Units::C),
                        (Field::TemperatureSetpointLow, "System Node Setpoint Low Temperature", Units::C),
                        (Field::HumidityRatioSetpoint, "System Node Setpoint Humidity Ratio", Units::KgWaterPerKgDryAir),
                        (Field::HumidityRatioMin, "System Node Setpoint Minimum Humidity Ratio", Units::KgWaterPerKgDryAir),
                        (Field::HumidityRatioMax, "System Node Setpoint Maximum Humidity Ratio", Units::KgWaterPerKgDryAir),
                        (Field::RelativeHumidity, "System Node Relative Humidity", Units::Percentage),
                        (Field::Pressure, "System Node Pressure", Units::Pa),
                        (Field::VolumeFlowRateStandardDensity, "System Node Standard Density Volume Flow Rate", Units::M3PerSecond),
                    ] { register(field, name, units)?; }
                    if matches!(node.fluid_type, SourceNodeFluidType::Air | SourceNodeFluidType::Water) {
                        for (field, name, units) in [
                            (Field::VolumeFlowRateCurrentDensity, "System Node Current Density Volume Flow Rate", Units::M3PerSecond),
                            (Field::Density, "System Node Current Density", Units::KgPerM3),
                            (Field::SpecificHeat, "System Node Specific Heat", Units::JPerKgK),
                        ] { register(field, name, units)?; }
                    }
                    for (field, name, units) in [
                        (Field::ReportEnthalpy, "System Node Enthalpy", Units::JPerKg),
                        (Field::WetBulbTemperature, "System Node Wetbulb Temperature", Units::C),
                        (Field::AirDewPointTemperature, "System Node Dewpoint Temperature", Units::C),
                        (Field::OutdoorAirWindSpeed, "System Node Wind Speed", Units::MPerS),
                        (Field::OutdoorAirWindDirection, "System Node Wind Direction", Units::Deg),
                        (Field::Quality, "System Node Quality", Units::Dimensionless),
                        (Field::Height, "System Node Height", Units::M),
                    ] { register(field, name, units)?; }
                    if globals.display_advanced_report_variables {
                        for (field, name, units) in [
                            (Field::TemperatureMin, "System Node Minimum Temperature", Units::C),
                            (Field::TemperatureMax, "System Node Maximum Temperature", Units::C),
                            (Field::MassFlowRateMin, "System Node Minimum Limit Mass Flow Rate", Units::KgPerSecond),
                            (Field::MassFlowRateMax, "System Node Maximum Limit Mass Flow Rate", Units::KgPerSecond),
                            (Field::MassFlowRateMinAvailable, "System Node Minimum Available Mass Flow Rate", Units::KgPerSecond),
                            (Field::MassFlowRateMaxAvailable, "System Node Maximum Available Mass Flow Rate", Units::KgPerSecond),
                            (Field::MassFlowRateSetpoint, "System Node Setpoint Mass Flow Rate", Units::KgPerSecond),
                            (Field::MassFlowRateRequest, "System Node Requested Mass Flow Rate", Units::KgPerSecond),
                            (Field::TemperatureLastTimestep, "System Node Last Timestep Temperature", Units::C),
                            (Field::EnthalpyLastTimestep, "System Node Last Timestep Enthalpy", Units::JPerKg),
                        ] { register(field, name, units)?; }
                    }
                    if control.carbon_dioxide_simulation {
                        for (field, name, units) in [
                            (Field::CarbonDioxide, "System Node CO2 Concentration", Units::Ppm),
                        ] { register(field, name, units)?; }
                    }
                    if control.generic_contaminant_simulation {
                        for (field, name, units) in [
                            (Field::GenericContaminant, "System Node Generic Air Contaminant Concentration", Units::Ppm),
                        ] { register(field, name, units)?; }
                    }
                }
            }
            self.nodes.node_variables_setup = true;
            bnd.write_all(b"! This file shows details about the branches, nodes, and other\n")?;
            bnd.write_all(b"! elements of the flow connections.\n")?;
            bnd.write_all(
                b"! This file is intended for use in \"debugging\" potential problems\n",
            )?;
            bnd.write_all(b"! that may also be detected by the program, but may be more easily\n")?;
            bnd.write_all(b"! identified by \"eye\".\n")?;
            bnd.write_all(b"! This file is also intended to support software which draws a\n")?;
            bnd.write_all(b"! schematic diagram of the HVAC system.\n")?;
            bnd.write_all(b"! ===============================================================\n")?;
            bnd.write_all(b"! #Nodes,<Number of Unique Nodes>\n")?;
            write!(bnd, " #Nodes,{}\n", self.nodes.number_of_unique_node_names)?;
            if self.nodes.number_of_unique_node_names > 0 {
                bnd.write_all(b"! <Node>,<NodeNumber>,<Node Name>,<Node Fluid Type>,<# Times Node Referenced After Definition>\n")?;
            }
            let mut count_zero: i32 = 0;
            for number in 1..=self.nodes.number_of_unique_node_names {
                let node = self.nodes.physical.as_ref().and_then(|rows| rows.get(number - 1))
                    .ok_or(Error::Unavailable("actual BND Node479"))?;
                let node_id = self.nodes.node_ids.as_ref().and_then(|rows| rows.get(number))
                    .ok_or(Error::Unavailable("actual BND NodeID480"))?;
                let reference = self.nodes.node_references.as_ref().and_then(|rows| rows.get(number - 1))
                    .ok_or(Error::Unavailable("actual BND NodeRef487"))?;
                writeln!(bnd, " Node,{number},{node_id},{},{}", fluid_name(node.fluid_type)?, reference)?;
                if *reference == 0 {
                    count_zero = count_zero.checked_add(1)
                        .ok_or(Error::Unavailable("actual BND Count0 signed increment"))?;
                }
            }
            if count_zero > 0 {
                bnd.write_all(b"! ===============================================================\n")?;
                bnd.write_all(b"! Suspicious nodes have 0 references.  It is normal for some nodes, however.\n")?;
                bnd.write_all(b"! Listing nodes with 0 references (culled from previous list):\n")?;
                bnd.write_all(b"! <Suspicious Node>,<NodeNumber>,<Node Name>,<Node Fluid Type>,<# Times Node Referenced After Definition>\n")?;
                for number in 1..=self.nodes.number_of_unique_node_names {
                    let node = self.nodes.physical.as_ref().and_then(|rows| rows.get(number - 1))
                        .ok_or(Error::Unavailable("actual suspicious BND Node500"))?;
                    let node_id = self.nodes.node_ids.as_ref().and_then(|rows| rows.get(number))
                        .ok_or(Error::Unavailable("actual suspicious BND NodeID501"))?;
                    let reference = self.nodes.node_references.as_ref().and_then(|rows| rows.get(number - 1))
                        .ok_or(Error::Unavailable("actual suspicious BND NodeRef502"))?;
                    if *reference > 0 { continue; }
                    writeln!(bnd, " Suspicious Node,{number},{node_id},{},{}", fluid_name(node.fluid_type)?, reference)?;
                }
            }
        }
        Ok(())
    }
}

/// Actual enum overload binds the live underlying i32, not a real conversion.
impl crate::heat_balance::source_output_registration::SourceIntegerOutputOwners for OrdinaryEquipment {
    fn integer_output(&self, target: crate::heat_balance::source_output_registration::SourceIntegerOutputTarget)
        -> Result<&i32, OrdinaryRadiationError> {
        let crate::heat_balance::source_output_registration::SourceIntegerOutputTarget::PurchasedAirAvailabilityStatus { index: number } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        let row = self.purchased_air.units.as_ref().and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        Ok(&row.avail_status)
    }
}
