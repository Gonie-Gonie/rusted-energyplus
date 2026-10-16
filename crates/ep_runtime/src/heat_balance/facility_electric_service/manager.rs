//! Literal zero/default-load-center electric facility manager over the same owner.
use super::*;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::source_output_registration::{
    SourceOutputRegistration, SourceRealOutputOwners, TimeStepKind,
};
use crate::weather::day::WeatherGlobalState;
/// Disjoint actual live owners; no private metering snapshot or reconstructed facility.
pub(crate) struct SourceFacilityElectricOwners<'a, 'backend> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    /// Actual disjoint TimeStepSysSec field, beside separately borrowed SimElec.
    pub(crate) time_step_sys_seconds: &'a f64,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) predefined_registry: &'a crate::heat_balance::predefined_tables::PredefinedRegistry,
    pub(crate) predefined_table: &'a mut crate::heat_balance::ctf_reports::PredefinedStringTable,
    pub(crate) real_output_owners: &'a dyn SourceRealOutputOwners,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
impl FacilityElectricService {
    pub(crate) fn manage_source_electric_power_service<
        H: crate::heat_balance::source_output_registration::SourceOutputFileHost,
        D: crate::heat_balance::source_output_registration::SourceOutputDictionaryHost,
    >(
        &mut self,
        first_hvac_iteration: bool,
        sim_electric_circuits: &mut bool,
        update_meters_only: bool,
        o: &mut SourceFacilityElectricOwners<'_, '_>,
        output: &mut SourceFacilityOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_power_manager_input(o, output)?;
            self.get_input_flag = false;
        }
        if o.globals.meters_have_been_initialized && self.setup_meter_index_flag {
            self.setup_source_meter_indices(o)?;
            self.setup_meter_index_flag = false;
        }
        if o.clock.begin_envrn_flag && self.new_environment_flag {
            self.reinitialize_source_environment()?;
            self.new_environment_flag = false;
        }
        if !o.clock.begin_envrn_flag {
            self.new_environment_flag = true;
        }
        self.read_source_meter_rates(o)?;
        self.whole_building_remaining_load = self.total_electric_demand;
        if update_meters_only {
            if self.facility_power_in_transformer_present {
                return Err(Error::Unavailable("actual utility transformer manage159"));
            }
            self.update_source_whole_building_records(o)?;
            return Ok(());
        }
        for load_center in &mut self.load_centers {
            load_center.manage_source(&mut self.whole_building_remaining_load)?;
        }
        self.update_source_whole_building_records(o)?;
        if self.facility_power_in_transformer_present {
            return Err(Error::Unavailable("actual utility transformer manage177"));
        }
        self.update_source_whole_building_records(o)?;
        if self.power_out_transformer.is_some() {
            return Err(Error::Unavailable("actual power-out transformer manage182"));
        }
        *sim_electric_circuits = first_hvac_iteration;
        Ok(())
    }
    fn setup_source_meter_indices(
        &mut self,
        o: &SourceFacilityElectricOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.electricity_facility_meter_index =
            o.registration.source_meter_index("ELECTRICITY:FACILITY")?;
        self.electricity_produced_cogeneration_meter_index = o
            .registration
            .source_meter_index("COGENERATION:ELECTRICITYPRODUCED")?;
        self.electricity_produced_pv_meter_index = o
            .registration
            .source_meter_index("PHOTOVOLTAIC:ELECTRICITYPRODUCED")?;
        self.electricity_produced_wind_meter_index = o
            .registration
            .source_meter_index("WINDTURBINE:ELECTRICITYPRODUCED")?;
        self.electricity_produced_storage_meter_index = o
            .registration
            .source_meter_index("ELECTRICSTORAGE:ELECTRICITYPRODUCED")?;
        self.electricity_produced_conversion_meter_index = o
            .registration
            .source_meter_index("POWERCONVERSION:ELECTRICITYPRODUCED")?;
        if self.number_of_load_centers > 0 {
            for load_center in &mut self.load_centers {
                load_center.setup_source_meter_indices(o.registration)?;
            }
        }
        if self.facility_power_in_transformer_present {
            return Err(Error::Unavailable(
                "actual utility transformer setupMeterIndices437",
            ));
        }
        Ok(())
    }
    fn reinitialize_source_environment(&mut self) -> Result<(), Error> {
        self.whole_building_remaining_load = 0.0;
        self.electricity_produced = 0.0;
        self.electricity_produced_rate = 0.0;
        self.electricity_purchased = 0.0;
        self.electricity_purchased_rate = 0.0;
        self.electricity_surplus_rate = 0.0;
        self.electricity_surplus = 0.0;
        self.electricity_net_rate = 0.0;
        self.electricity_net = 0.0;
        self.total_building_electric_demand = 0.0;
        self.total_hvac_electric_demand = 0.0;
        self.total_electric_demand = 0.0;
        self.electricity_produced_pv_rate = 0.0;
        self.electricity_produced_wind_rate = 0.0;
        self.electricity_produced_storage_rate = 0.0;
        self.electricity_produced_cogeneration_rate = 0.0;
        // Native deliberately does NOT reset elecProducedPowerConversionRate_.
        if self.number_of_load_centers > 0 {
            for load_center in &mut self.load_centers {
                load_center.reinitialize_source_environment()?;
            }
        }
        if self.facility_power_in_transformer_present {
            return Err(Error::Unavailable(
                "actual utility transformer environment reset466",
            ));
        }
        if self.power_out_transformer.is_some() {
            return Err(Error::Unavailable(
                "actual power-out transformer environment reset469",
            ));
        }
        Ok(())
    }
    fn read_source_meter_rates(
        &mut self,
        o: &SourceFacilityElectricOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.total_building_electric_demand = instant_meter_value(
            self,
            o,
            self.electricity_facility_meter_index,
            TimeStepKind::Zone,
        )? / o.clock.time_step_zone_sec;
        self.total_hvac_electric_demand = instant_meter_value(
            self,
            o,
            self.electricity_facility_meter_index,
            TimeStepKind::System,
        )? / *o.time_step_sys_seconds;
        self.total_electric_demand =
            self.total_building_electric_demand + self.total_hvac_electric_demand;
        self.electricity_produced_pv_rate = instant_meter_value(
            self,
            o,
            self.electricity_produced_pv_meter_index,
            TimeStepKind::System,
        )? / *o.time_step_sys_seconds;
        self.electricity_produced_wind_rate = instant_meter_value(
            self,
            o,
            self.electricity_produced_wind_meter_index,
            TimeStepKind::System,
        )? / *o.time_step_sys_seconds;
        self.electricity_produced_storage_rate = instant_meter_value(
            self,
            o,
            self.electricity_produced_storage_meter_index,
            TimeStepKind::System,
        )? / *o.time_step_sys_seconds;
        self.electricity_produced_cogeneration_rate = instant_meter_value(
            self,
            o,
            self.electricity_produced_cogeneration_meter_index,
            TimeStepKind::System,
        )? / *o.time_step_sys_seconds;
        self.electricity_produced_conversion_rate = instant_meter_value(
            self,
            o,
            self.electricity_produced_conversion_meter_index,
            TimeStepKind::System,
        )? / *o.time_step_sys_seconds;
        Ok(())
    }
    fn update_source_whole_building_records(
        &mut self,
        o: &SourceFacilityElectricOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.read_source_meter_rates(o)?;
        self.electricity_produced_rate = self.electricity_produced_cogeneration_rate
            + self.electricity_produced_pv_rate
            + self.electricity_produced_wind_rate
            + self.electricity_produced_storage_rate
            + self.electricity_produced_conversion_rate;
        self.electricity_produced = self.electricity_produced_rate * *o.time_step_sys_seconds;
        self.electricity_purchased_rate =
            self.total_electric_demand - self.electricity_produced_rate;
        if self.electricity_purchased_rate.abs() < 0.0001 {
            self.electricity_purchased_rate = 0.0;
        }
        if self.electricity_purchased_rate < 0.0 {
            self.electricity_purchased_rate = 0.0;
        }
        self.electricity_purchased = self.electricity_purchased_rate * *o.time_step_sys_seconds;
        self.electricity_surplus_rate = self.electricity_produced_rate - self.total_electric_demand;
        if self.electricity_surplus_rate.abs() < 0.0001 {
            self.electricity_surplus_rate = 0.0;
        }
        if self.electricity_surplus_rate < 0.0 {
            self.electricity_surplus_rate = 0.0;
        }
        self.electricity_surplus = self.electricity_surplus_rate * *o.time_step_sys_seconds;
        self.electricity_net_rate = self.total_electric_demand - self.electricity_produced_rate;
        self.electricity_net = self.electricity_net_rate * *o.time_step_sys_seconds;
        Ok(())
    }
}
fn instant_meter_value(
    facility: &FacilityElectricService,
    o: &SourceFacilityElectricOwners<'_, '_>,
    index: i32,
    time_step: TimeStepKind,
) -> Result<f64, Error> {
    let owners = SourceFacilityAndOtherOutputOwners {
        facility,
        other: o.real_output_owners,
    };
    o.registration
        .instant_source_meter_value(index, time_step, &owners)
}
impl FacilityElectricService {
    pub(crate) fn reinitialize_source_zone_gains(&mut self) -> Result<(), Error> {
        if self.facility_power_in_transformer_present {
            return Err(Error::Unavailable(
                "actual power-in transformer zone-gain reinit196",
            ));
        }
        if self.power_out_transformer.is_some() {
            return Err(Error::Unavailable(
                "actual power-out transformer zone-gain reinit199",
            ));
        }
        if self.number_of_load_centers > 0 {
            for load_center in &mut self.load_centers {
                load_center.reinitialize_source_zone_gains()?;
            }
        }
        Ok(())
    }
}
