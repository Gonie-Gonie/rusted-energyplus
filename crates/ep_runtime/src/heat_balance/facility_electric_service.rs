//! Actual SimulationManager222 selected electric-facility constructor storage.
//! The same value moves to the later gain provider; no reset or second owner.
mod input;
mod load_center;
mod output;
use load_center::SourceElectricLoadCenter;
pub(crate) use output::{SourceFacilityAndOtherOutputOwners, SourceFacilityOutputContext};
mod manager;
pub(crate) use manager::SourceFacilityElectricOwners;
/// Same real facility object shared by gain initialization and the electric manager.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FacilityElectricService {
    pub(crate) new_environment_internal_gains_flag: bool,
    pub(crate) get_input_flag: bool,
    pub(crate) facility_power_in_transformer_present: bool,
    pub(crate) power_out_transformer: Option<()>,
    pub(crate) number_of_load_centers: usize,
    pub(crate) load_centers: Vec<SourceElectricLoadCenter>,
    // Actual ElectricPowerServiceManager.hh764-773 constructor and private members.
    pub(crate) new_environment_flag: bool,
    pub(crate) setup_meter_index_flag: bool,
    pub(crate) number_of_transformers: usize,
    pub(crate) number_of_electric_storage_devices: i32,
    pub(crate) number_of_power_out_transformers: i32,
    pub(crate) name: String,
    pub(crate) facility_power_in_transformer_name: String,
    pub(crate) power_out_transformer_name: String,
    pub(crate) power_in_transformer: Option<()>,
    pub(crate) electricity_facility_meter_index: i32,
    pub(crate) electricity_produced_cogeneration_meter_index: i32,
    pub(crate) electricity_produced_pv_meter_index: i32,
    pub(crate) electricity_produced_wind_meter_index: i32,
    pub(crate) electricity_produced_storage_meter_index: i32,
    pub(crate) electricity_produced_conversion_meter_index: i32,
    pub(crate) whole_building_remaining_load: f64,
    pub(crate) electricity_produced: f64,
    pub(crate) electricity_produced_rate: f64,
    pub(crate) electricity_purchased: f64,
    pub(crate) electricity_purchased_rate: f64,
    pub(crate) electricity_surplus_rate: f64,
    pub(crate) electricity_surplus: f64,
    pub(crate) electricity_net_rate: f64,
    pub(crate) electricity_net: f64,
    pub(crate) total_building_electric_demand: f64,
    pub(crate) total_hvac_electric_demand: f64,
    pub(crate) total_electric_demand: f64,
    pub(crate) electricity_produced_pv_rate: f64,
    pub(crate) electricity_produced_wind_rate: f64,
    pub(crate) electricity_produced_storage_rate: f64,
    pub(crate) electricity_produced_conversion_rate: f64,
    pub(crate) electricity_produced_cogeneration_rate: f64,
    pub(crate) pv_total_capacity: f64,
    pub(crate) wind_total_capacity: f64,
}
impl FacilityElectricService {
    pub(crate) fn new() -> Self {
        Self {
            new_environment_internal_gains_flag: true,
            get_input_flag: true,
            facility_power_in_transformer_present: false,
            power_out_transformer: None,
            number_of_load_centers: 0,
            load_centers: Vec::new(),
            new_environment_flag: true,
            setup_meter_index_flag: true,
            number_of_transformers: 0,
            number_of_electric_storage_devices: 0,
            number_of_power_out_transformers: 0,
            name: "Whole Building".into(),
            facility_power_in_transformer_name: String::new(),
            power_out_transformer_name: String::new(),
            power_in_transformer: None,
            electricity_facility_meter_index: -1,
            electricity_produced_cogeneration_meter_index: -1,
            electricity_produced_pv_meter_index: -1,
            electricity_produced_wind_meter_index: -1,
            electricity_produced_storage_meter_index: -1,
            electricity_produced_conversion_meter_index: -1,
            whole_building_remaining_load: 0.0,
            electricity_produced: 0.0,
            electricity_produced_rate: 0.0,
            electricity_purchased: 0.0,
            electricity_purchased_rate: 0.0,
            electricity_surplus_rate: 0.0,
            electricity_surplus: 0.0,
            electricity_net_rate: 0.0,
            electricity_net: 0.0,
            total_building_electric_demand: 0.0,
            total_hvac_electric_demand: 0.0,
            total_electric_demand: 0.0,
            electricity_produced_pv_rate: 0.0,
            electricity_produced_wind_rate: 0.0,
            electricity_produced_storage_rate: 0.0,
            electricity_produced_conversion_rate: 0.0,
            electricity_produced_cogeneration_rate: 0.0,
            pv_total_capacity: 0.0,
            wind_total_capacity: 0.0,
        }
    }
}

impl FacilityElectricService {
    /// Native verifyCustomMetersElecPowerMgr473-478 traverses this actual vector.
    /// Reuses the same load-center setup child on every call without a once flag.
    pub(crate) fn verify_source_custom_meters(
        &mut self,
        registration: &crate::heat_balance::source_output_registration::SourceOutputRegistration,
    ) -> Result<(), crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError> {
        for load_center in &mut self.load_centers {
            load_center.setup_source_meter_indices(registration)?;
        }
        Ok(())
    }
}
