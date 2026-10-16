//! Actual CalculatePollution98/ReadEnergyMeters673/CalcPollution581 same-owner writes.
use super::*;
impl SourcePollutionModule {
    pub(crate) fn calculate_source_pollution(
        &mut self,
        fraction_zone_step: &f64,
        registration: &SourceOutputRegistration,
        owners: &dyn SourceRealOutputOwners,
        schedules: &SelectedScheduleManager,
    ) -> Result<(), Error> {
        if !self.report_setup {
            return Ok(());
        }
        self.read_source_energy_meters(fraction_zone_step, registration, owners)?;
        self.calculate_source_pollutants(schedules)
    }
    fn read_source_energy_meters(
        &mut self,
        fraction_zone_step: &f64,
        registration: &SourceOutputRegistration,
        owners: &dyn SourceRealOutputOwners,
    ) -> Result<(), Error> {
        // Native takes this actual scalar by value before traversing its meters.
        let fraction_zone_step = *fraction_zone_step;
        for i in 0..16 {
            self.facility_meter_values[i] = registration.instant_source_meter_value(
                self.facility_meter_numbers[i],
                TimeStepKind::Zone,
                owners,
            )? * fraction_zone_step
                + registration.instant_source_meter_value(
                    self.facility_meter_numbers[i],
                    TimeStepKind::System,
                    owners,
                )?;
        }
        self.fuel_component_values[0] = self.facility_meter_values[0]
            - self.facility_meter_values[12]
            + self.facility_meter_values[15] / self.purchased_cooling_cop;
        if self.fuel_component_values[0] < 0.0 {
            self.fuel_component_values[0] = 0.0;
        }
        self.fuel_component_values[1] = self.facility_meter_values[1]
            + self.facility_meter_values[14] / self.purchased_heat_efficiency
            + self.facility_meter_values[13] / self.steam_conversion_efficiency;
        self.fuel_component_values[2] = self.facility_meter_values[2];
        self.fuel_component_values[3] = self.facility_meter_values[3];
        self.fuel_component_values[5] = self.facility_meter_values[5];
        self.fuel_component_values[6] = self.facility_meter_values[6];
        self.fuel_component_values[4] = self.facility_meter_values[4];
        self.fuel_component_values[7] = self.facility_meter_values[7];
        self.fuel_component_values[8] = self.facility_meter_values[8];
        self.fuel_component_values[9] = self.facility_meter_values[9];
        self.fuel_component_values[11] = self.facility_meter_values[11];
        self.fuel_component_values[10] = self.facility_meter_values[10];
        Ok(())
    }
    fn calculate_source_pollutants(
        &mut self,
        schedules: &SelectedScheduleManager,
    ) -> Result<(), Error> {
        for pollutant in 0..16 {
            self.pollutant_values[pollutant] = 0.0;
            for fuel in 0..10 {
                let coefficient = &self.coefficients[fuel];
                let component = &mut self.components[fuel];
                if coefficient.used {
                    component.pollutant_values[pollutant] = 0.0;
                    let mut value = coefficient.pollutant_coefficients[pollutant];
                    if pollutant != 13 && pollutant != 15 {
                        value *= 0.001;
                    }
                    if let Some(schedule) = coefficient.pollutant_schedules[pollutant] {
                        value *= current_schedule(schedule, schedules)?;
                    }
                    component.pollutant_values[pollutant] =
                        self.fuel_component_values[fuel] * 1.0e-6 * value;
                }
                self.pollutant_values[pollutant] += component.pollutant_values[pollutant];
            }
        }
        self.total_carbon_equivalent_n2o = self.pollutant_values[4] * self.carbon_equivalent_n2o;
        self.total_carbon_equivalent_ch4 = self.pollutant_values[2] * self.carbon_equivalent_ch4;
        self.total_carbon_equivalent_co2 = self.pollutant_values[0] * self.carbon_equivalent_co2;
        let electric = &self.coefficients[0];
        self.components[0].source_value =
            self.fuel_component_values[0] * electric.source_coefficient;
        self.components[11].source_value =
            self.fuel_component_values[11] * electric.source_coefficient;
        self.components[10].source_value =
            self.fuel_component_values[10] * electric.source_coefficient;
        if let Some(schedule) = electric.source_schedule {
            let value = current_schedule(schedule, schedules)?;
            self.components[0].source_value *= value;
            self.components[11].source_value *= value;
            self.components[10].source_value *= value;
        }
        let gas = &self.coefficients[1];
        self.components[1].source_value = self.facility_meter_values[1] * gas.source_coefficient;
        if let Some(schedule) = gas.source_schedule {
            self.components[1].source_value *= current_schedule(schedule, schedules)?;
        }
        for fuel in [2, 3, 7, 5, 6, 4, 8, 9] {
            let coefficient = &self.coefficients[fuel];
            self.components[fuel].source_value =
                self.fuel_component_values[fuel] * coefficient.source_coefficient;
            if let Some(schedule) = coefficient.source_schedule {
                self.components[fuel].source_value *= current_schedule(schedule, schedules)?;
            }
        }
        Ok(())
    }
}
