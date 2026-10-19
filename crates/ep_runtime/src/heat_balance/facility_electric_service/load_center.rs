//! Actual default ElectPowerLoadCenter(state,0), including unwritten scalar mask.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SourceInputProcessorShortCut;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use std::convert::Infallible;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceElectricBusType {
    Invalid = -1,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    Ac = 0,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    DcInverter = 1,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    AcStorage = 2,
    DcInverterDcStorage = 3,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    DcInverterAcStorage = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceGeneratorOperationScheme {
    Invalid = -1,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    BaseLoad = 0,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    DemandLimit = 1,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    TrackElectrical = 2,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    TrackSchedule = 3,
    TrackMeter = 4,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    ThermalFollow = 5,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    ThermalFollowLimitElectrical = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceStorageOperationScheme {
    Invalid = -1,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    FacilityDemandStoreExcessOnSite = 0,
    MeterDemandStoreExcessOnSite = 1,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    ChargeDischargeSchedules = 2,
    #[allow(
        dead_code,
        reason = "ElectricPowerServiceManager.hh retains the complete native bus/operation enum identities; the finite no-generator domain does not construct every alternative."
    )]
    FacilityDemandLeveling = 3,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceElectricLoadCenter {
    /// Actual storageObj.
    pub(crate) storage: Option<Infallible>,
    /// Actual converterObj.
    pub(crate) converter: Option<Infallible>,
    /// Actual transformerObj.
    pub(crate) transformer: Option<Infallible>,
    /// Actual numGenerators.
    pub(crate) number_generators: i32,
    /// Actual elecGenCntrlObj.
    pub(crate) generator_controllers: Vec<Infallible>,
    /// Actual bussType.
    pub(crate) bus_type: SourceElectricBusType,
    /// Actual thermalProd.
    pub(crate) thermal_production: f64,
    /// Actual thermalProdRate.
    pub(crate) thermal_production_rate: f64,
    /// Actual inverterPresent.
    pub(crate) inverter_present: bool,
    /// Actual inverterName.
    pub(crate) inverter_name: String,
    /// Actual inverterObj.
    pub(crate) inverter: Option<Infallible>,
    /// Actual subpanelFeedInRequest.
    pub(crate) subpanel_feed_in_request: f64,
    /// Actual subpanelFeedInRate.
    pub(crate) subpanel_feed_in_rate: f64,
    /// Actual subpanelDrawRate.
    pub(crate) subpanel_draw_rate: f64,
    /// Actual genElectricProd.
    pub(crate) generator_electric_production: f64,
    /// Actual genElectProdRate.
    pub(crate) generator_electric_production_rate: f64,
    /// Actual storOpCVGenRate.
    pub(crate) storage_cv_generator_rate: Option<f64>,
    /// Actual storOpCVDrawRate.
    pub(crate) storage_cv_draw_rate: f64,
    /// Actual storOpCVFeedInRate.
    pub(crate) storage_cv_feed_in_rate: f64,
    /// Actual storOpCVChargeRate.
    pub(crate) storage_cv_charge_rate: f64,
    /// Actual storOpCVDischargeRate.
    pub(crate) storage_cv_discharge_rate: f64,
    /// Actual storOpIsCharging.
    pub(crate) storage_is_charging: bool,
    /// Actual storOpIsDischarging.
    pub(crate) storage_is_discharging: bool,
    /// Actual name_.
    pub(crate) name: String,
    /// Actual generatorListName_.
    pub(crate) generator_list_name: String,
    /// Actual genOperationScheme_.
    pub(crate) generator_operation_scheme: SourceGeneratorOperationScheme,
    /// Actual demandMeterName_.
    pub(crate) demand_meter_name: String,
    /// Actual demandMeterPtr_.
    pub(crate) demand_meter_pointer: i32,
    /// Actual generationMeterName_.
    pub(crate) generation_meter_name: String,
    /// Actual generatorsPresent_.
    pub(crate) generators_present: bool,
    /// Actual myCoGenSetupFlag_.
    pub(crate) my_cogeneration_setup_flag: bool,
    /// Actual demandLimit_.
    pub(crate) demand_limit: f64,
    /// Actual trackSched_.
    pub(crate) track_schedule: Option<SourceScheduleIndex>,
    /// Actual storagePresent_.
    pub(crate) storage_present: bool,
    /// Actual storageName_.
    pub(crate) storage_name: String,
    /// Actual transformerPresent_.
    pub(crate) transformer_present: bool,
    /// Actual transformerName_.
    pub(crate) transformer_name: String,
    /// Actual totalPowerRequest_.
    pub(crate) total_power_request: f64,
    /// Actual totalThermalPowerRequest_.
    pub(crate) total_thermal_power_request: f64,
    /// Actual storageScheme_.
    pub(crate) storage_operation_scheme: SourceStorageOperationScheme,
    /// Actual trackSorageOpMeterName_.
    pub(crate) storage_operation_meter_name: String,
    /// Actual trackStorageOpMeterIndex_.
    pub(crate) storage_operation_meter_index: i32,
    /// Actual converterPresent_.
    pub(crate) converter_present: bool,
    /// Actual converterName_.
    pub(crate) converter_name: String,
    /// Actual maxStorageSOCFraction_.
    pub(crate) maximum_storage_soc_fraction: f64,
    /// Actual minStorageSOCFraction_.
    pub(crate) minimum_storage_soc_fraction: f64,
    /// Actual designStorageChargePower_.
    pub(crate) design_storage_charge_power: f64,
    /// Actual designStorageChargePowerWasSet_.
    pub(crate) design_storage_charge_power_was_set: bool,
    /// Actual designStorageDischargePower_.
    pub(crate) design_storage_discharge_power: f64,
    /// Actual designStorageDischargePowerWasSet_.
    pub(crate) design_storage_discharge_power_was_set: bool,
    /// Actual storageChargeModSched_.
    pub(crate) storage_charge_mod_schedule: Option<SourceScheduleIndex>,
    /// Actual storageDischargeModSched_.
    pub(crate) storage_discharge_mod_schedule: Option<SourceScheduleIndex>,
    /// Actual facilityDemandTarget_.
    pub(crate) facility_demand_target: f64,
    /// Actual facilityDemandTargetModSched_.
    pub(crate) facility_demand_target_mod_schedule: Option<SourceScheduleIndex>,
    /// Actual eMSOverridePelFromStorage_.
    pub(crate) ems_override_power_from_storage: bool,
    /// Actual eMSValuePelFromStorage_.
    pub(crate) ems_power_from_storage: f64,
    /// Actual eMSOverridePelIntoStorage_.
    pub(crate) ems_override_power_into_storage: bool,
    /// Actual eMSValuePelIntoStorage_.
    pub(crate) ems_power_into_storage: f64,
}
impl SourceElectricLoadCenter {
    fn constructor() -> Self {
        Self {
            storage: None,
            converter: None,
            transformer: None,
            number_generators: 0,
            generator_controllers: Vec::new(),
            bus_type: SourceElectricBusType::Invalid,
            thermal_production: 0.0,
            thermal_production_rate: 0.0,
            inverter_present: false,
            inverter_name: String::new(),
            inverter: None,
            subpanel_feed_in_request: 0.0,
            subpanel_feed_in_rate: 0.0,
            subpanel_draw_rate: 0.0,
            generator_electric_production: 0.0,
            generator_electric_production_rate: 0.0,
            storage_cv_generator_rate: None,
            storage_cv_draw_rate: 0.0,
            storage_cv_feed_in_rate: 0.0,
            storage_cv_charge_rate: 0.0,
            storage_cv_discharge_rate: 0.0,
            storage_is_charging: false,
            storage_is_discharging: false,
            name: String::new(),
            generator_list_name: String::new(),
            generator_operation_scheme: SourceGeneratorOperationScheme::Invalid,
            demand_meter_name: String::new(),
            demand_meter_pointer: 0,
            generation_meter_name: String::new(),
            generators_present: false,
            my_cogeneration_setup_flag: true,
            demand_limit: 0.0,
            track_schedule: None,
            storage_present: false,
            storage_name: String::new(),
            transformer_present: false,
            transformer_name: String::new(),
            total_power_request: 0.0,
            total_thermal_power_request: 0.0,
            storage_operation_scheme: SourceStorageOperationScheme::Invalid,
            storage_operation_meter_name: String::new(),
            storage_operation_meter_index: 0,
            converter_present: false,
            converter_name: String::new(),
            maximum_storage_soc_fraction: 1.0,
            minimum_storage_soc_fraction: 0.0,
            design_storage_charge_power: 0.0,
            design_storage_charge_power_was_set: false,
            design_storage_discharge_power: 0.0,
            design_storage_discharge_power_was_set: false,
            storage_charge_mod_schedule: None,
            storage_discharge_mod_schedule: None,
            facility_demand_target: 0.0,
            facility_demand_target_mod_schedule: None,
            ems_override_power_from_storage: false,
            ems_power_from_storage: 0.0,
            ems_override_power_into_storage: false,
            ems_power_into_storage: 0.0,
        }
    }
}
impl SourceElectricLoadCenter {
    pub(super) fn new_default(shortcut: &mut SourceInputProcessorShortCut) -> Self {
        let value = Self::constructor();
        shortcut.current_module_object = "ElectricLoadCenter:Distribution".into();
        // Actual objectNum==0 return902 precedes generator queries and outputs.
        value
    }
    pub(super) fn manage_source(
        &mut self,
        remaining_whole_power_demand: &mut f64,
    ) -> Result<(), Error> {
        self.subpanel_feed_in_request = *remaining_whole_power_demand;
        if self.generators_present {
            return Err(Error::Unavailable("actual positive dispatchGenerators1102"));
        }
        self.update_source_generator_records()?;
        if matches!(
            self.bus_type,
            SourceElectricBusType::DcInverter | SourceElectricBusType::DcInverterAcStorage
        ) {
            return Err(Error::Unavailable(
                "actual positive inverter simulation1107",
            ));
        }
        if self.storage_present {
            return Err(Error::Unavailable(
                "actual positive storage timeCheckAndUpdate1112",
            ));
        }
        if self.bus_type == SourceElectricBusType::DcInverterDcStorage && self.inverter.is_some() {
            return Err(Error::Unavailable(
                "actual positive DC storage inverter simulation1118",
            ));
        }
        if self.converter.is_some() {
            return Err(Error::Unavailable(
                "actual positive converter simulation1123",
            ));
        }
        if self.transformer.is_some()
            && (self.storage_cv_feed_in_rate > 0.0 || self.storage_cv_draw_rate > 0.0)
        {
            return Err(Error::Unavailable(
                "actual positive load-center transformer simulation1128",
            ));
        }
        self.update_source_generator_records()?;
        Ok(())
    }
    fn update_source_generator_records(&mut self) -> Result<(), Error> {
        if self.bus_type != SourceElectricBusType::Invalid {
            return Err(Error::Unavailable(
                "actual positive electrical bus record branch1935",
            ));
        }
        // Native Invalid bus case2026 genuinely does nothing to electrical cells.
        self.thermal_production_rate = 0.0;
        self.thermal_production = 0.0;
        if !self.generator_controllers.is_empty() {
            return Err(Error::Unavailable("actual generator thermal records2036"));
        }
        Ok(())
    }
    pub(super) fn setup_source_meter_indices(
        &mut self,
        registration: &SourceOutputRegistration,
    ) -> Result<(), Error> {
        self.demand_meter_pointer = registration.source_meter_index(&self.demand_meter_name)?;
        if self.demand_meter_pointer == 0
            && self.generator_operation_scheme == SourceGeneratorOperationScheme::TrackMeter
        {
            return Err(Error::Unavailable(
                "actual positive missing-track-meter fatal1850",
            ));
        }
        if self.storage_operation_scheme
            == SourceStorageOperationScheme::MeterDemandStoreExcessOnSite
        {
            self.storage_operation_meter_index =
                registration.source_meter_index(&self.storage_operation_meter_name)?;
            if self.storage_operation_meter_index == 0 {
                return Err(Error::Unavailable(
                    "actual positive missing-storage-meter fatal1861",
                ));
            }
        }
        Ok(())
    }
    pub(super) fn reinitialize_source_environment(&mut self) -> Result<(), Error> {
        self.generator_electric_production = 0.0;
        self.generator_electric_production_rate = 0.0;
        self.thermal_production = 0.0;
        self.thermal_production_rate = 0.0;
        self.total_power_request = 0.0;
        self.total_thermal_power_request = 0.0;
        self.subpanel_feed_in_rate = 0.0;
        self.subpanel_draw_rate = 0.0;
        self.storage_cv_draw_rate = 0.0;
        self.storage_cv_feed_in_rate = 0.0;
        self.storage_cv_charge_rate = 0.0;
        self.storage_cv_discharge_rate = 0.0;
        if self.generators_present
            && self.number_generators > 0
            && !self.generator_controllers.is_empty()
        {
            return Err(Error::Unavailable(
                "actual positive generator environment reset1887",
            ));
        }
        if self.transformer.is_some() {
            return Err(Error::Unavailable(
                "actual load-center transformer environment reset1892",
            ));
        }
        if self.storage.is_some() {
            return Err(Error::Unavailable(
                "actual load-center storage environment reset1896",
            ));
        }
        if self.inverter.is_some() {
            return Err(Error::Unavailable(
                "actual load-center inverter environment reset1900",
            ));
        }
        if self.converter.is_some() {
            return Err(Error::Unavailable(
                "actual load-center converter environment reset1904",
            ));
        }
        Ok(())
    }
    pub(super) fn reinitialize_source_zone_gains(&mut self) -> Result<(), Error> {
        if self.transformer.is_some() {
            return Err(Error::Unavailable(
                "actual load-center transformer zone gains1911",
            ));
        }
        if self.storage.is_some() {
            return Err(Error::Unavailable(
                "actual load-center storage zone gains1915",
            ));
        }
        if self.inverter.is_some() {
            return Err(Error::Unavailable(
                "actual load-center inverter zone gains1919",
            ));
        }
        if self.converter.is_some() {
            return Err(Error::Unavailable(
                "actual load-center converter zone gains1923",
            ));
        }
        Ok(())
    }
}
