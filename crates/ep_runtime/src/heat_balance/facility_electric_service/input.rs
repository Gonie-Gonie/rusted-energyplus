//! Literal getPowerManagerInput214-420 zero/default-load-center route.
use super::*;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
impl FacilityElectricService {
    pub(super) fn get_source_power_manager_input<
        H: crate::heat_balance::source_output_registration::SourceOutputFileHost,
        D: crate::heat_balance::source_output_registration::SourceOutputDictionaryHost,
    >(
        &mut self,
        o: &mut SourceFacilityElectricOwners<'_, '_>,
        output: &mut SourceFacilityOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        self.number_of_load_centers = o
            .input
            .get_num_objects_found("ElectricLoadCenter:Distribution");
        if self.number_of_load_centers > 0 {
            return Err(Error::Unavailable(
                "actual ElectPowerLoadCenter constructor224",
            ));
        }
        let mut errors_found = false;
        let lists = o
            .input
            .get_num_objects_found("ElectricLoadCenter:Generators");
        if lists > 0 {
            o.backend.show_severe_error("ElectricLoadCenter:Generators input object requires an ElectricLoadCenterDistribution input object.")?;
            errors_found = true;
        }
        let mut inverters = o
            .input
            .get_num_objects_found("ElectricLoadCenter:Inverter:Simple");
        inverters += o
            .input
            .get_num_objects_found("ElectricLoadCenter:Inverter:FunctionOfPower");
        inverters += o
            .input
            .get_num_objects_found("ElectricLoadCenter:Inverter:LookUpTable");
        if inverters > 0 {
            o.backend.show_severe_error("ElectricLoadCenter:Inverter:* input objects require an ElectricLoadCenter:Distribution input object.")?;
            errors_found = true;
        }
        let mut storage = o
            .input
            .get_num_objects_found("ElectricLoadCenter:Storage:Simple");
        storage += o
            .input
            .get_num_objects_found("ElectricLoadCenter:Storage:Battery");
        if storage > 0 {
            o.backend.show_severe_error("ElectricLoadCenter:Storage:* input objects require an ElectricLoadCenter:Distribution input object.")?;
            errors_found = true;
        }
        let mut generators = o
            .input
            .get_num_objects_found("Generator:InternalCombustionEngine");
        generators += o.input.get_num_objects_found("Generator:CombustionTurbine");
        generators += o.input.get_num_objects_found("Generator:MicroCHP");
        generators += o.input.get_num_objects_found("Generator:FuelCell");
        generators += o.input.get_num_objects_found("Generator:Photovoltaic");
        generators += o.input.get_num_objects_found("Generator:WindTurbine");
        if generators > 0 {
            o.backend.show_severe_error("Electric generator input objects require an ElectricLoadCenter:Distribution input object.")?;
            errors_found = true;
        }
        if errors_found {
            return match o.backend.show_fatal_error(
                "Simulation halted because of missing input objects related to ElectricLoadCenter.",
            ) {
                Ok(never) => match never {},
                Err(error) => Err(Error::Fatal(error)),
            };
        }
        let any_electricity_present = o.registration.source_meter_index("ELECTRICITY:FACILITY")?;
        let plant_profile = o.input.get_num_objects_found("LoadProfile:Plant");
        if any_electricity_present > -1 || plant_profile > 0 {
            self.load_centers
                .push(SourceElectricLoadCenter::new_default(o.shortcut));
            self.number_of_load_centers = 1;
        }
        self.number_of_transformers = o
            .input
            .get_num_objects_found("ElectricLoadCenter:Transformer");
        if self.number_of_transformers > 0 {
            o.shortcut.current_module_object = "ElectricLoadCenter:Transformer".into();
            return Err(Error::Unavailable("actual transformer GetObjectItem283"));
        }
        if self.number_of_load_centers > 0 {
            self.register_source_facility_outputs(o, output)?;
            self.report_source_pv_and_wind_capacity(o)?;
            self.sum_source_storage_devices()?;
            self.check_source_load_centers()?;
        }
        Ok(())
    }
}
