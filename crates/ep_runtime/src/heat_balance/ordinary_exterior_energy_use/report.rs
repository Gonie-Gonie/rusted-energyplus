//! Native ReportExteriorEnergyUse427-509 expression, schedule-read and write order.
use super::*;
fn current(
    schedules: &SelectedScheduleManager,
    pointer: Option<SourceScheduleIndex>,
) -> Result<f64, Error> {
    schedules
        .schedules
        .get(
            pointer
                .ok_or(Error::Unavailable("actual Exterior Schedule nullptr use"))?
                .0,
        )
        .map(|row| {
            if row.ems_actuated_on {
                row.ems_value
            } else {
                row.current_value
            }
        })
        .ok_or(Error::Unavailable("actual same Exterior Schedule pointer"))
}
impl SourceExteriorEnergyUse {
    pub(crate) fn report_source_exterior_energy_use(
        &mut self,
        schedules: &SelectedScheduleManager,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
    ) -> Result<(), Error> {
        for number in 1..=self.number_lights {
            let light = self.light_mut(number)?;
            match light.control {
                SourceLightControlType::ScheduleOnly => {
                    light.power = light.design_level * current(schedules, light.schedule)?;
                    light.current_use = light.power * clock.time_step_zone_sec;
                }
                SourceLightControlType::AstroClockOverride => {
                    if environment.current_weather.sun_is_up {
                        light.power = 0.0;
                        light.current_use = 0.0;
                    } else {
                        light.power = light.design_level * current(schedules, light.schedule)?;
                        light.current_use = light.power * clock.time_step_zone_sec;
                    }
                }
                SourceLightControlType::Invalid | SourceLightControlType::Num => {}
            }
            if light.manage_demand && light.power > light.demand_limit {
                light.power = light.demand_limit;
                light.current_use = light.power * clock.time_step_zone_sec;
            }
            if light.power_actuator_on {
                light.power = light.power_actuator_value;
            }
            light.current_use = light.power * clock.time_step_zone_sec;
            if !clock.warmup_flag && clock.do_output_reporting && clock.kind_of_sim == 3 {
                light.sum_consumption += light.current_use;
                if light.current_use > 0.01 {
                    light.sum_time_not_zero_consumption += clock.time_step_zone;
                }
            }
        }
        for number in 1..=self.number_equipment {
            let equipment = self
                .equipment
                .as_mut()
                .and_then(|v| v.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual ExteriorEquipment allocated row"))?;
            equipment.power = equipment.design_level * current(schedules, equipment.schedule)?;
            equipment.current_use = equipment.power * clock.time_step_zone_sec;
        }
        Ok(())
    }
}
