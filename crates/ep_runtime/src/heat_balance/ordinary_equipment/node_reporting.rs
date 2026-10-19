//! Genuine CalcMoreNodeInfo955-1206 on the sole Node/MoreNodeInfo owners.
use super::{OrdinaryEquipment, source_nodes::SourceNodeFluidType};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_schedule_manager::{
    SelectedScheduleManager, SourceScheduleIndex,
};
use crate::heat_balance::selected_simulation_startup::SelectedRuntimeLanguage;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use crate::psychrometrics::{
    energyplus_cpcw, energyplus_psy_cp_air_fn_w, energyplus_psy_h_fn_tdb_w,
    energyplus_psy_rho_air_fn_pb_tdb_w,
};
use crate::weather::day::WeatherEnvironmentState;
fn fill_allocated<T: Clone>(array: &mut Option<Vec<T>>, value: T) -> Result<(), Error> {
    array
        .as_mut()
        .ok_or(Error::Unavailable("same reached Node reporting allocation"))?
        .fill(value);
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceNodeReporting {
    once: bool,
    rho_air_standard_initial: Option<f64>,
    rho_water_standard_initial: Option<f64>,
    wet_bulb_requested: Option<Vec<Option<bool>>>,
    wet_bulb_schedules: Option<Vec<Option<Option<SourceScheduleIndex>>>>,
    relative_humidity_requested: Option<Vec<Option<bool>>>,
    relative_humidity_schedules: Option<Vec<Option<Option<SourceScheduleIndex>>>>,
    dew_point_requested: Option<Vec<Option<bool>>>,
    dew_point_schedules: Option<Vec<Option<Option<SourceScheduleIndex>>>>,
    specific_heat_requested: Option<Vec<Option<bool>>>,
    specific_heat_schedules: Option<Vec<Option<Option<SourceScheduleIndex>>>>,
    reporting_strings: Vec<String>,
    // A real nullptr is represented by None. Positive glycol pointers are
    // unrepresented and stop at their actual first same-owner lookup.
    fluids: Vec<Option<std::convert::Infallible>>,
}
impl Default for SourceNodeReporting {
    fn default() -> Self {
        Self {
            once: true,
            rho_air_standard_initial: None,
            rho_water_standard_initial: None,
            wet_bulb_requested: None,
            wet_bulb_schedules: None,
            relative_humidity_requested: None,
            relative_humidity_schedules: None,
            dew_point_requested: None,
            dew_point_schedules: None,
            specific_heat_requested: None,
            specific_heat_schedules: None,
            reporting_strings: Vec::new(),
            fluids: Vec::new(),
        }
    }
}
fn initialized<T: Copy>(array: &Option<Vec<Option<T>>>, index: usize) -> Result<T, Error> {
    array
        .as_ref()
        .and_then(|rows| rows.get(index))
        .copied()
        .flatten()
        .ok_or(Error::Unavailable(
            "actual initialized Node reporting array cell",
        ))
}
fn write_cell<T>(array: &mut Option<Vec<Option<T>>>, index: usize, value: T) -> Result<(), Error> {
    *array
        .as_mut()
        .and_then(|rows| rows.get_mut(index))
        .ok_or(Error::Unavailable(
            "actual allocated Node reporting array cell",
        ))? = Some(value);
    Ok(())
}
fn sensed_by_ems(
    runtime_language: &SelectedRuntimeLanguage,
    _node_number: usize,
    _variable_name: &str,
) -> Result<bool, Error> {
    // Actual EMSManager1619 loops NumSensors, independently of AnyEMS.
    // With positive count, Sensor(1).UniqueKeyName is the first missing owner.
    if runtime_language.number_of_sensors > 0 {
        return Err(Error::Unavailable(
            "actual CheckIfNodeMoreInfoSensedByEMS Sensor(1).UniqueKeyName1620",
        ));
    }
    Ok(false)
}
fn requested_now(
    requested: &Option<Vec<Option<bool>>>,
    reporting_schedule: &Option<Vec<Option<Option<SourceScheduleIndex>>>>,
    index: usize,
    schedules: &SelectedScheduleManager,
) -> Result<bool, Error> {
    if initialized(requested, index)? {
        if let Some(schedule) = initialized(reporting_schedule, index)? {
            Ok(schedules.source_current_schedule_value(schedule)? > 0.0)
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

impl OrdinaryEquipment {
    pub(crate) fn calculate_source_more_node_info(
        &mut self,
        environment: &WeatherEnvironmentState,
        registration: &SourceOutputRegistration,
        schedules: &SelectedScheduleManager,
        runtime_language: &SelectedRuntimeLanguage,
    ) -> Result<(), Error> {
        let number = self.nodes.number_of_nodes;
        let report = &mut self.nodes.reporting;
        if report.once {
            report.rho_air_standard_initial = Some(environment.std_rho_air);
            report.rho_water_standard_initial =
                Some(crate::psychrometrics::energyplus_rho_h2o(5.05));
            report.wet_bulb_requested = Some(vec![None; number]);
            report.wet_bulb_schedules = Some(vec![None; number]);
            report.relative_humidity_requested = Some(vec![None; number]);
            report.relative_humidity_schedules = Some(vec![None; number]);
            report.dew_point_requested = Some(vec![None; number]);
            report.dew_point_schedules = Some(vec![None; number]);
            report.specific_heat_requested = Some(vec![None; number]);
            report.specific_heat_schedules = Some(vec![None; number]);
            report.reporting_strings.reserve(number);
            report.fluids.reserve(number);
            fill_allocated(&mut report.wet_bulb_requested, Some(false))?;
            fill_allocated(&mut report.wet_bulb_schedules, Some(None))?;
            fill_allocated(&mut report.relative_humidity_requested, Some(false))?;
            fill_allocated(&mut report.relative_humidity_schedules, Some(None))?;
            fill_allocated(&mut report.dew_point_requested, Some(false))?;
            fill_allocated(&mut report.dew_point_schedules, Some(None))?;
            fill_allocated(&mut report.specific_heat_requested, Some(false))?;
            fill_allocated(&mut report.specific_heat_schedules, Some(None))?;
            for i in 0..number {
                let node_id = self
                    .nodes
                    .node_ids
                    .as_ref()
                    .and_then(|rows| rows.get(i + 1))
                    .ok_or(Error::Unavailable("actual CalcMoreNodeInfo NodeID1026"))?;
                report
                    .reporting_strings
                    .push(format!("NodeReportingCalc:{node_id}"));
                let node = self
                    .nodes
                    .physical
                    .as_ref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable(
                        "actual CalcMoreNodeInfo Node.FluidIndex1028",
                    ))?;
                if node.fluid_index != 0 {
                    return Err(Error::Unavailable(
                        "actual CalcMoreNodeInfo dataFluid.glycols(FluidIndex)1028",
                    ));
                }
                report.fluids.push(None);
                // Literal full reqVars order, including unused/frequency entries.
                // Every matching assignment is retained; the last schedule wins.
                for request in &registration.requested_variables {
                    if request.key.eq_ignore_ascii_case(node_id) || request.key.is_empty() {
                        if request
                            .name
                            .eq_ignore_ascii_case("System Node Wetbulb Temperature")
                        {
                            write_cell(&mut report.wet_bulb_requested, i, true)?;
                            write_cell(
                                &mut report.wet_bulb_schedules,
                                i,
                                request.reporting_schedule,
                            )?;
                        } else if request
                            .name
                            .eq_ignore_ascii_case("System Node Relative Humidity")
                        {
                            write_cell(&mut report.relative_humidity_requested, i, true)?;
                            write_cell(
                                &mut report.relative_humidity_schedules,
                                i,
                                request.reporting_schedule,
                            )?;
                        } else if request
                            .name
                            .eq_ignore_ascii_case("System Node Dewpoint Temperature")
                        {
                            write_cell(&mut report.dew_point_requested, i, true)?;
                            write_cell(
                                &mut report.dew_point_schedules,
                                i,
                                request.reporting_schedule,
                            )?;
                        } else if request
                            .name
                            .eq_ignore_ascii_case("System Node Specific Heat")
                        {
                            write_cell(&mut report.specific_heat_requested, i, true)?;
                            write_cell(
                                &mut report.specific_heat_schedules,
                                i,
                                request.reporting_schedule,
                            )?;
                        }
                    }
                }
                if sensed_by_ems(runtime_language, i + 1, "System Node Wetbulb Temperature")? {
                    write_cell(&mut report.wet_bulb_requested, i, true)?;
                    write_cell(&mut report.wet_bulb_schedules, i, None)?;
                }
                if sensed_by_ems(runtime_language, i + 1, "System Node Relative Humidity")? {
                    write_cell(&mut report.relative_humidity_requested, i, true)?;
                    write_cell(&mut report.relative_humidity_schedules, i, None)?;
                }
                if sensed_by_ems(runtime_language, i + 1, "System Node Dewpoint Temperature")? {
                    write_cell(&mut report.dew_point_requested, i, true)?;
                    write_cell(&mut report.dew_point_schedules, i, None)?;
                }
                if sensed_by_ems(runtime_language, i + 1, "System Node Specific Heat")? {
                    write_cell(&mut report.specific_heat_requested, i, true)?;
                    write_cell(&mut report.specific_heat_schedules, i, None)?;
                }
            }
            report.once = false;
        }
        for i in 0..number {
            // The SPM flag is considered only after both requested cases fail.
            let report_wet_bulb = if initialized(&report.wet_bulb_requested, i)? {
                if let Some(schedule) = initialized(&report.wet_bulb_schedules, i)? {
                    schedules.source_current_schedule_value(schedule)? > 0.0
                } else {
                    true
                }
            } else {
                self.nodes
                    .physical
                    .as_ref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable(
                        "actual CalcMoreNodeInfo Node.SPMNodeWetBulbRepReq1076",
                    ))?
                    .setpoint_manager_wet_bulb_reporting_required
            };
            let report_relative_humidity = requested_now(
                &report.relative_humidity_requested,
                &report.relative_humidity_schedules,
                i,
                schedules,
            )?;
            let report_dew_point = requested_now(
                &report.dew_point_requested,
                &report.dew_point_schedules,
                i,
                schedules,
            )?;
            let report_specific_heat = requested_now(
                &report.specific_heat_requested,
                &report.specific_heat_schedules,
                i,
                schedules,
            )?;
            let node = self
                .nodes
                .physical
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable(
                    "actual CalcMoreNodeInfo Node fluid branch1095",
                ))?;
            let more = self
                .nodes
                .more_node_info
                .as_mut()
                .and_then(|rows| rows.get_mut(i))
                .ok_or(Error::Unavailable(
                    "actual CalcMoreNodeInfo MoreNodeInfo allocation1096",
                ))?;
            if node.fluid_type == SourceNodeFluidType::Air {
                more.volume_flow_rate_standard_density = node.mass_flow_rate
                    / report
                        .rho_air_standard_initial
                        .ok_or(Error::Unavailable("actual RhoAirStdInit1096"))?;
                let rho = energyplus_psy_rho_air_fn_pb_tdb_w(
                    environment.current_weather.out_baro_press,
                    node.temperature,
                    node.humidity_ratio,
                );
                if rho < 0.0 {
                    return Err(Error::Unavailable(
                        "actual PsyRhoAirFnPbTdbW negative-density diagnostic Psychrometrics.hh543",
                    ));
                }
                more.density = rho;
                if rho != 0.0 {
                    more.volume_flow_rate_current_density = node.mass_flow_rate / rho;
                }
                more.report_enthalpy =
                    energyplus_psy_h_fn_tdb_w(node.temperature, node.humidity_ratio);
                if report_wet_bulb {
                    let _called_from = report
                        .reporting_strings
                        .get(i)
                        .ok_or(Error::Unavailable("actual nodeReportingStrings1112"))?;
                    return Err(Error::Unavailable(
                        "actual CalcMoreNodeInfo PsyTwbFnTdbWPb1108 Source CalledFrom/diagnostic boundary",
                    ));
                } else {
                    more.wet_bulb_temperature = 0.0;
                }
                if report_dew_point {
                    return Err(Error::Unavailable(
                        "actual CalcMoreNodeInfo PsyTdpFnWPb1118 cached TsatPb/diagnostic Source boundary",
                    ));
                } else {
                    more.air_dew_point_temperature = 0.0;
                }
                if report_relative_humidity {
                    let _called_from = report
                        .reporting_strings
                        .get(i)
                        .ok_or(Error::Unavailable("actual nodeReportingStrings1129"))?;
                    return Err(Error::Unavailable(
                        "actual CalcMoreNodeInfo PsyRhFnTdbWPb1125 cached Psat/diagnostic Source boundary",
                    ));
                } else {
                    more.relative_humidity = 0.0;
                }
                more.specific_heat = if report_specific_heat {
                    energyplus_psy_cp_air_fn_w(node.humidity_ratio)
                } else {
                    0.0
                };
            } else if node.fluid_type == SourceNodeFluidType::Water {
                // FluidIndex<=0 makes Native's left && operand false. A positive
                // current index needs the actual glycols array size and pointer.
                if node.fluid_index > 0 {
                    return Err(Error::Unavailable(
                        "actual CalcMoreNodeInfo dataFluid.glycols.isize1141",
                    ));
                }
                let rho = report
                    .rho_water_standard_initial
                    .ok_or(Error::Unavailable("actual RhoWaterStdInit1142"))?;
                let rho_standard = rho;
                let cp = energyplus_cpcw(node.temperature);
                more.volume_flow_rate_standard_density = node.mass_flow_rate / rho_standard;
                more.volume_flow_rate_current_density = node.mass_flow_rate / rho;
                more.density = rho;
                more.report_enthalpy = cp * node.temperature;
                more.specific_heat = cp;
                more.wet_bulb_temperature = 0.0;
                more.relative_humidity = 100.0;
            } else if node.fluid_type == SourceNodeFluidType::Steam {
                if node.quality == 1.0 {
                    return Err(Error::Unavailable(
                        "actual CalcMoreNodeInfo Fluid::GetSteam1160",
                    ));
                } else if node.quality == 0.0 {
                    more.volume_flow_rate_standard_density = node.mass_flow_rate
                        / report
                            .rho_water_standard_initial
                            .ok_or(Error::Unavailable("actual RhoWaterStdInit1170"))?;
                    more.report_enthalpy = energyplus_cpcw(node.temperature) * node.temperature;
                    more.wet_bulb_temperature = 0.0;
                    more.relative_humidity = 0.0;
                }
            } else if node.fluid_type == SourceNodeFluidType::Electric {
                more.volume_flow_rate_standard_density = 0.0;
                more.report_enthalpy = 0.0;
                more.wet_bulb_temperature = 0.0;
                more.relative_humidity = 0.0;
                more.specific_heat = 0.0;
            } else {
                more.volume_flow_rate_standard_density = node.mass_flow_rate
                    / report
                        .rho_air_standard_initial
                        .ok_or(Error::Unavailable("actual RhoAirStdInit1183"))?;
                if node.humidity_ratio > 0.0 {
                    more.report_enthalpy =
                        energyplus_psy_h_fn_tdb_w(node.temperature, node.humidity_ratio);
                    if report_wet_bulb {
                        return Err(Error::Unavailable(
                            "actual CalcMoreNodeInfo PsyTwbFnTdbWPb1188 StdBaroPress Source boundary",
                        ));
                    } else {
                        more.wet_bulb_temperature = 0.0;
                    }
                    more.specific_heat = if report_specific_heat {
                        energyplus_psy_cp_air_fn_w(node.humidity_ratio)
                    } else {
                        0.0
                    };
                } else {
                    more.report_enthalpy = energyplus_cpcw(node.temperature) * node.temperature;
                    more.wet_bulb_temperature = 0.0;
                    more.specific_heat = 0.0;
                }
            }
        }
        Ok(())
    }
}
