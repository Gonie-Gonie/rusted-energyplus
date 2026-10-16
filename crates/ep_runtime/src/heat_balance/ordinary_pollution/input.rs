//! Actual GetPollutionFactorInput188-409 and TAB accessor734-817 zero-input closure.
use super::*;
impl SourcePollutionModule {
    pub(crate) fn get_source_pollution_factor_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if !self.get_input_flag {
            return Ok(());
        }
        self.get_input_flag = false;
        shortcut.current_module_object = "EnvironmentalImpactFactors".into();
        self.number_environmental_impact_factors =
            input.get_num_objects_found(&shortcut.current_module_object);
        if self.number_environmental_impact_factors > 0 {
            return Err(Error::Unavailable(
                "actual EnvironmentalImpactFactors GetObjectItem222",
            ));
        } else if self.report_setup {
            backend.show_warning_error(&format!(
                "{}: not entered.  Values will be defaulted.",
                shortcut.current_module_object
            ))?;
        }
        self.purchased_heat_efficiency = 0.3;
        self.purchased_cooling_cop = 3.0;
        self.steam_conversion_efficiency = 0.25;
        self.carbon_equivalent_n2o = 0.0;
        self.carbon_equivalent_ch4 = 0.0;
        self.carbon_equivalent_co2 = 0.0;
        shortcut.current_module_object = "FuelFactors".into();
        self.number_fuel_factors = input.get_num_objects_found(&shortcut.current_module_object);
        if self.number_fuel_factors > 0 {
            return Err(Error::Unavailable(
                "actual FuelFactors GetObjectItem271 before coefficient/schedule writes",
            ));
        }
        let mut errors_found = false;
        if self.report_setup {
            for (coefficient, meters, name) in [
                (0, &[0, 12, 15][..], "ELECTRICITY"),
                (1, &[1, 14, 13][..], "NATURAL GAS"),
                (3, &[3][..], "FUEL OIL #2"),
                (2, &[2][..], "FUEL OIL #1"),
                (4, &[4][..], "COAL"),
                (5, &[5][..], "GASOLINE"),
                (6, &[6][..], "PROPANE"),
                (7, &[7][..], "DIESEL"),
                (8, &[8][..], "OTHERFUEL1"),
                (9, &[9][..], "OTHERFUEL2"),
            ] {
                if !self.coefficients[coefficient].used
                    && meters.iter().any(|&i| self.facility_meter_numbers[i] > 0)
                {
                    backend.show_severe_error(&format!(
                        "{} Not Found or Fuel not specified For Pollution Calculation for {name}",
                        shortcut.current_module_object
                    ))?;
                    errors_found = true;
                }
            }
        }
        if errors_found {
            return match backend
                .show_fatal_error("Errors found in getting Pollution Calculation Reporting Input")
            {
                Ok(never) => match never {},
                Err(error) => Err(Error::Fatal(error)),
            };
        }
        Ok(())
    }
    pub(crate) fn get_source_fuel_factor_info(
        &mut self,
        fuel: SourcePollutionFuel,
        used: &mut bool,
        factor: &mut f64,
        schedule_used: &mut bool,
        schedule: &mut Option<SourceScheduleIndex>,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_pollution_factor_input(input, shortcut, backend)?;
            self.get_input_flag = false;
        }
        *used = false;
        *factor = 0.0;
        *schedule_used = false;
        *schedule = None;
        let index = fuel.coefficient();
        let coefficient = &self.coefficients[index];
        if coefficient.used {
            *used = true;
            *factor = coefficient.source_coefficient;
            if coefficient.source_schedule.is_none() {
                *schedule_used = false;
            } else {
                *schedule_used = true;
                *schedule = coefficient.source_schedule;
            }
        } else {
            *factor = [3.167, 1.084, 1.05, 1.05, 1.05, 1.05, 1.05, 1.05, 1.0, 1.0][index];
        }
        if fuel == SourcePollutionFuel::DistrictHeatingWater {
            *factor /= self.purchased_heat_efficiency;
        } else if fuel == SourcePollutionFuel::DistrictCooling {
            *factor /= self.purchased_cooling_cop;
        } else if fuel == SourcePollutionFuel::DistrictHeatingSteam {
            *factor = 0.3 / self.steam_conversion_efficiency;
        }
        Ok(())
    }
    pub(crate) fn get_source_environmental_impact_factor_info(
        &mut self,
        heating: &mut f64,
        cooling: &mut f64,
        steam: &mut f64,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_pollution_factor_input(input, shortcut, backend)?;
            self.get_input_flag = false;
        }
        if self.number_environmental_impact_factors > 0 {
            *heating = self.purchased_heat_efficiency;
            *steam = self.steam_conversion_efficiency;
            *cooling = self.purchased_cooling_cop;
        }
        Ok(())
    }
}
