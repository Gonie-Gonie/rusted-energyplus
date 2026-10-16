//! Genuine CalcMoreNodeInfo955-1214 constructor/zero-node path on same NodeOwner.
use super::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
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
    fluids: Vec<std::convert::Infallible>,
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
impl OrdinaryEquipment {
    pub(crate) fn calculate_source_more_node_info(
        &mut self,
        environment: &WeatherEnvironmentState,
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
            if number > 0 {
                return Err(Error::Unavailable(
                    "actual CalcMoreNodeInfo first NodeID1012 before request/EMS scan",
                ));
            }
            report.once = false;
        }
        if number > 0 {
            return Err(Error::Unavailable(
                "actual CalcMoreNodeInfo NodeWetBulbRepReq1071 before physical-node computation",
            ));
        }
        Ok(())
    }
}
