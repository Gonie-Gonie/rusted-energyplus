//! Genuine startup setup; unconditional three output children are mandatory evenzero.
use super::*;
impl SourcePollutionModule {
    pub(crate) fn setup_source_pollution_meter_reporting<H, D>(
        &mut self,
        clock: &WeatherGlobalState,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        outputs: &mut SourcePollutionOutputContext<'_, H, D>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>
    where
        H: crate::heat_balance::source_output_registration::SourceOutputFileHost,
        D: crate::heat_balance::source_output_registration::SourceOutputDictionaryHost,
    {
        if self.get_input_flag {
            self.get_source_pollution_factor_input(input, shortcut, backend)?;
            self.get_input_flag = false;
        }
        for &fuel in &self.fuel_factor_list {
            let coefficient = self
                .coefficients
                .get(fuel)
                .ok_or(Error::Unavailable("actual Pollution fuel enum436"))?;
            if !coefficient.used {
                continue;
            }
            return Err(Error::Unavailable(
                "actual positive fuel source-energy output registration464",
            ));
        }
        for field in [
            SourceCarbonEquivalentField::N2o,
            SourceCarbonEquivalentField::Ch4,
            SourceCarbonEquivalentField::Co2,
        ] {
            outputs.declare(self, field, clock, input, backend, eio)?;
        }
        for (i, name) in [
            "ELECTRICITY:FACILITY",
            "NATURALGAS:FACILITY",
            "FUELOILNO1:FACILITY",
            "FUELOILNO2:FACILITY",
            "COAL:FACILITY",
            "GASOLINE:FACILITY",
            "PROPANE:FACILITY",
            "DIESEL:FACILITY",
            "OTHERFUEL1:FACILITY",
            "OTHERFUEL2:FACILITY",
            "ELECTRICITYSURPLUSSOLD:FACILITY",
            "ELECTRICITYPURCHASED:FACILITY",
            "ELECTRICITYPRODUCED:FACILITY",
            "DISTRICTHEATINGSTEAM:FACILITY",
            "DISTRICTHEATINGWATER:FACILITY",
            "DISTRICTCOOLING:FACILITY",
        ]
        .iter()
        .enumerate()
        {
            self.facility_meter_numbers[i] = outputs.registration.source_meter_index(name)?;
        }
        Ok(())
    }
    pub(crate) fn setup_source_pollution_calculations(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Output:EnvironmentalImpactFactors".into();
        let count = input.get_num_objects_found(&shortcut.current_module_object);
        self.report_setup = true;
        if count > 0 {
            return Err(Error::Unavailable(
                "actual OutputEnvironmentalImpactFactors GetObjectItem156 before InitPollutionMeterReporting",
            ));
        }
        Ok(())
    }
    pub(crate) fn check_source_pollution_meter_reporting(
        &self,
        registration: &SourceOutputRegistration,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.number_fuel_factors == 0 || self.number_environmental_impact_factors == 0 {
            let mut requested = false;
            for name in [
                "Environmental Impact Total N2O Emissions Carbon Equivalent Mass",
                "Environmental Impact Total CH4 Emissions Carbon Equivalent Mass",
                "Environmental Impact Total CO2 Emissions Carbon Equivalent Mass",
                "Carbon Equivalent:Facility",
                "CarbonEquivalentEmissions:Carbon Equivalent",
            ] {
                if registration
                    .reporting_source_variable(name)
                    .map_err(Error::Output)?
                {
                    requested = true;
                    break;
                }
            }
            if requested {
                backend.show_warning_error("GetPollutionFactorInput: Requested reporting for Carbon Equivalent Pollution, but insufficient information is entered.")?;
                backend.show_continue_error("Both \"FuelFactors\" and \"EnvironmentalImpactFactors\" must be entered or the displayed carbon pollution will all be zero.")?;
            }
        }
        Ok(())
    }
}
