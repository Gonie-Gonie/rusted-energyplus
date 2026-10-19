//! UNSEALED genuine PurchasedAir retained constructor records only.
//! No GetPurchasedAir, registration, lifecycle readiness or integration is implemented.
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourcePurchasedAirLimit {
    #[default]
    Invalid = -1,
    None = 0,
    FlowRate = 1,
    Capacity = 2,
    FlowRateAndCapacity = 3,
    Num = 4,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourcePurchasedAirHumidityControl {
    #[default]
    Invalid = -1,
    None = 0,
    ConstantSensibleHeatRatio = 1,
    Humidistat = 2,
    ConstantSupplyHumidityRatio = 3,
    Num = 4,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourcePurchasedAirDcv {
    #[default]
    Invalid = -1,
    None = 0,
    OccupancySchedule = 1,
    CO2SetPoint = 2,
    Num = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourcePurchasedAirEconomizer {
    #[default]
    Invalid = -1,
    NoEconomizer = 0,
    DifferentialDryBulb = 1,
    DifferentialEnthalpy = 2,
    Num = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourcePurchasedAirHeatRecovery {
    #[default]
    Invalid = -1,
    None = 0,
    Sensible = 1,
    Enthalpy = 2,
    Num = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourcePurchasedAirFuel {
    #[default]
    Invalid = -1,
    Electricity = 0,
    NaturalGas = 1,
    Gasoline = 2,
    Diesel = 3,
    Coal = 4,
    Propane = 5,
    FuelOilNo1 = 6,
    FuelOilNo2 = 7,
    OtherFuel1 = 8,
    OtherFuel2 = 9,
    DistrictCooling = 10,
    DistrictHeatingWater = 11,
    DistrictHeatingSteam = 12,
    Water = 13,
    None = 14,
    Num = 15,
}

/// Complete Native PurchasedAirManager.hh132-274 retained unit, constructor277-301.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZonePurchasedAir {
    pub(crate) c_object_name: String, // Native cObjectName
    pub(crate) name: String, // Native Name
    pub(crate) avail_sched: Option<SourceScheduleIndex>, // Native availSched
    pub(crate) zone_supply_air_node_num: i32, // Native ZoneSupplyAirNodeNum
    pub(crate) zone_exhaust_air_node_num: i32, // Native ZoneExhaustAirNodeNum
    pub(crate) plenum_exhaust_air_node_num: i32, // Native PlenumExhaustAirNodeNum
    pub(crate) return_plenum_index: i32, // Native ReturnPlenumIndex
    pub(crate) purch_air_array_index: i32, // Native PurchAirArrayIndex
    pub(crate) return_plenum_name: String, // Native ReturnPlenumName
    pub(crate) zone_recirc_air_node_num: i32, // Native ZoneRecircAirNodeNum
    pub(crate) max_heat_supp_air_temp: f64, // Native MaxHeatSuppAirTemp
    pub(crate) min_cool_supp_air_temp: f64, // Native MinCoolSuppAirTemp
    pub(crate) max_heat_supp_air_hum_rat: f64, // Native MaxHeatSuppAirHumRat
    pub(crate) min_cool_supp_air_hum_rat: f64, // Native MinCoolSuppAirHumRat
    pub(crate) heating_limit: SourcePurchasedAirLimit, // Native HeatingLimit
    pub(crate) max_heat_vol_flow_rate: f64, // Native MaxHeatVolFlowRate
    pub(crate) max_heat_sens_cap: f64, // Native MaxHeatSensCap
    pub(crate) cooling_limit: SourcePurchasedAirLimit, // Native CoolingLimit
    pub(crate) max_cool_vol_flow_rate: f64, // Native MaxCoolVolFlowRate
    pub(crate) max_cool_tot_cap: f64, // Native MaxCoolTotCap
    pub(crate) heat_avail_sched: Option<SourceScheduleIndex>, // Native heatAvailSched
    pub(crate) cool_avail_sched: Option<SourceScheduleIndex>, // Native coolAvailSched
    pub(crate) dehumid_ctrl_type: SourcePurchasedAirHumidityControl, // Native DehumidCtrlType
    pub(crate) cool_shr: f64, // Native CoolSHR
    pub(crate) humid_ctrl_type: SourcePurchasedAirHumidityControl, // Native HumidCtrlType
    pub(crate) oa_requirements_ptr: i32, // Native OARequirementsPtr
    pub(crate) dcv_type: SourcePurchasedAirDcv, // Native DCVType
    pub(crate) economizer_type: SourcePurchasedAirEconomizer, // Native EconomizerType
    pub(crate) outdoor_air: bool, // Native OutdoorAir
    pub(crate) outdoor_air_node_num: i32, // Native OutdoorAirNodeNum
    pub(crate) ht_rec_type: SourcePurchasedAirHeatRecovery, // Native HtRecType
    pub(crate) ht_rec_sen_eff: f64, // Native HtRecSenEff
    pub(crate) ht_rec_lat_eff: f64, // Native HtRecLatEff
    pub(crate) oa_flow_frac_sched: Option<SourceScheduleIndex>, // Native oaFlowFracSched
    pub(crate) max_heat_mass_flow_rate: f64, // Native MaxHeatMassFlowRate
    pub(crate) max_cool_mass_flow_rate: f64, // Native MaxCoolMassFlowRate
    pub(crate) ems_override_mdot_on: bool, // Native EMSOverrideMdotOn
    pub(crate) ems_value_mass_flow_rate: f64, // Native EMSValueMassFlowRate
    pub(crate) ems_override_oa_mdot_on: bool, // Native EMSOverrideOAMdotOn
    pub(crate) ems_value_oa_mass_flow_rate: f64, // Native EMSValueOAMassFlowRate
    pub(crate) ems_override_supply_temp_on: bool, // Native EMSOverrideSupplyTempOn
    pub(crate) ems_value_supply_temp: f64, // Native EMSValueSupplyTemp
    pub(crate) ems_override_supply_hum_rat_on: bool, // Native EMSOverrideSupplyHumRatOn
    pub(crate) ems_value_supply_hum_rat: f64, // Native EMSValueSupplyHumRat
    pub(crate) min_oa_mass_flow_rate: f64, // Native MinOAMassFlowRate
    pub(crate) outdoor_air_mass_flow_rate: f64, // Native OutdoorAirMassFlowRate
    pub(crate) outdoor_air_vol_flow_rate_std_rho: f64, // Native OutdoorAirVolFlowRateStdRho
    pub(crate) supply_air_mass_flow_rate: f64, // Native SupplyAirMassFlowRate
    pub(crate) supply_air_vol_flow_rate_std_rho: f64, // Native SupplyAirVolFlowRateStdRho
    pub(crate) ht_rec_sen_output: f64, // Native HtRecSenOutput
    pub(crate) ht_rec_lat_output: f64, // Native HtRecLatOutput
    pub(crate) oa_sen_output: f64, // Native OASenOutput
    pub(crate) oa_lat_output: f64, // Native OALatOutput
    pub(crate) sen_output_to_zone: f64, // Native SenOutputToZone
    pub(crate) lat_output_to_zone: f64, // Native LatOutputToZone
    pub(crate) sen_coil_load: f64, // Native SenCoilLoad
    pub(crate) lat_coil_load: f64, // Native LatCoilLoad
    pub(crate) oa_flow_max_cool_output_error: i32, // Native OAFlowMaxCoolOutputError
    pub(crate) oa_flow_max_heat_output_error: i32, // Native OAFlowMaxHeatOutputError
    pub(crate) saturation_output_error: i32, // Native SaturationOutputError
    pub(crate) oa_flow_max_cool_output_index: i32, // Native OAFlowMaxCoolOutputIndex
    pub(crate) oa_flow_max_heat_output_index: i32, // Native OAFlowMaxHeatOutputIndex
    pub(crate) saturation_output_index: i32, // Native SaturationOutputIndex
    pub(crate) avail_status: i32, // Native availStatus
    pub(crate) cool_err_index: i32, // Native CoolErrIndex
    pub(crate) heat_err_index: i32, // Native HeatErrIndex
    pub(crate) sen_heat_energy: f64, // Native SenHeatEnergy
    pub(crate) lat_heat_energy: f64, // Native LatHeatEnergy
    pub(crate) tot_heat_energy: f64, // Native TotHeatEnergy
    pub(crate) sen_cool_energy: f64, // Native SenCoolEnergy
    pub(crate) lat_cool_energy: f64, // Native LatCoolEnergy
    pub(crate) tot_cool_energy: f64, // Native TotCoolEnergy
    pub(crate) zone_sen_heat_energy: f64, // Native ZoneSenHeatEnergy
    pub(crate) zone_lat_heat_energy: f64, // Native ZoneLatHeatEnergy
    pub(crate) zone_tot_heat_energy: f64, // Native ZoneTotHeatEnergy
    pub(crate) zone_sen_cool_energy: f64, // Native ZoneSenCoolEnergy
    pub(crate) zone_lat_cool_energy: f64, // Native ZoneLatCoolEnergy
    pub(crate) zone_tot_cool_energy: f64, // Native ZoneTotCoolEnergy
    pub(crate) oa_sen_heat_energy: f64, // Native OASenHeatEnergy
    pub(crate) oa_lat_heat_energy: f64, // Native OALatHeatEnergy
    pub(crate) oa_tot_heat_energy: f64, // Native OATotHeatEnergy
    pub(crate) oa_sen_cool_energy: f64, // Native OASenCoolEnergy
    pub(crate) oa_lat_cool_energy: f64, // Native OALatCoolEnergy
    pub(crate) oa_tot_cool_energy: f64, // Native OATotCoolEnergy
    pub(crate) ht_rec_sen_heat_energy: f64, // Native HtRecSenHeatEnergy
    pub(crate) ht_rec_lat_heat_energy: f64, // Native HtRecLatHeatEnergy
    pub(crate) ht_rec_tot_heat_energy: f64, // Native HtRecTotHeatEnergy
    pub(crate) ht_rec_sen_cool_energy: f64, // Native HtRecSenCoolEnergy
    pub(crate) ht_rec_lat_cool_energy: f64, // Native HtRecLatCoolEnergy
    pub(crate) ht_rec_tot_cool_energy: f64, // Native HtRecTotCoolEnergy
    pub(crate) sen_heat_rate: f64, // Native SenHeatRate
    pub(crate) lat_heat_rate: f64, // Native LatHeatRate
    pub(crate) tot_heat_rate: f64, // Native TotHeatRate
    pub(crate) sen_cool_rate: f64, // Native SenCoolRate
    pub(crate) lat_cool_rate: f64, // Native LatCoolRate
    pub(crate) tot_cool_rate: f64, // Native TotCoolRate
    pub(crate) zone_sen_heat_rate: f64, // Native ZoneSenHeatRate
    pub(crate) zone_lat_heat_rate: f64, // Native ZoneLatHeatRate
    pub(crate) zone_tot_heat_rate: f64, // Native ZoneTotHeatRate
    pub(crate) zone_sen_cool_rate: f64, // Native ZoneSenCoolRate
    pub(crate) zone_lat_cool_rate: f64, // Native ZoneLatCoolRate
    pub(crate) zone_tot_cool_rate: f64, // Native ZoneTotCoolRate
    pub(crate) oa_sen_heat_rate: f64, // Native OASenHeatRate
    pub(crate) oa_lat_heat_rate: f64, // Native OALatHeatRate
    pub(crate) oa_tot_heat_rate: f64, // Native OATotHeatRate
    pub(crate) oa_sen_cool_rate: f64, // Native OASenCoolRate
    pub(crate) oa_lat_cool_rate: f64, // Native OALatCoolRate
    pub(crate) oa_tot_cool_rate: f64, // Native OATotCoolRate
    pub(crate) ht_rec_sen_heat_rate: f64, // Native HtRecSenHeatRate
    pub(crate) ht_rec_lat_heat_rate: f64, // Native HtRecLatHeatRate
    pub(crate) ht_rec_tot_heat_rate: f64, // Native HtRecTotHeatRate
    pub(crate) ht_rec_sen_cool_rate: f64, // Native HtRecSenCoolRate
    pub(crate) ht_rec_lat_cool_rate: f64, // Native HtRecLatCoolRate
    pub(crate) ht_rec_tot_cool_rate: f64, // Native HtRecTotCoolRate
    pub(crate) time_econo_active: f64, // Native TimeEconoActive
    pub(crate) time_ht_rec_active: f64, // Native TimeHtRecActive
    pub(crate) zone_ptr: i32, // Native ZonePtr
    pub(crate) hvac_sizing_index: i32, // Native HVACSizingIndex
    pub(crate) supply_temp: f64, // Native SupplyTemp
    pub(crate) supply_hum_rat: f64, // Native SupplyHumRat
    pub(crate) mixed_air_temp: f64, // Native MixedAirTemp
    pub(crate) mixed_air_hum_rat: f64, // Native MixedAirHumRat
    pub(crate) heat_fuel_eff_sched: Option<SourceScheduleIndex>, // Native heatFuelEffSched
    pub(crate) cool_fuel_eff_sched: Option<SourceScheduleIndex>, // Native coolFuelEffSched
    pub(crate) zone_tot_heat_fuel_rate: f64, // Native ZoneTotHeatFuelRate
    pub(crate) zone_tot_cool_fuel_rate: f64, // Native ZoneTotCoolFuelRate
    pub(crate) zone_tot_heat_fuel_energy: f64, // Native ZoneTotHeatFuelEnergy
    pub(crate) zone_tot_cool_fuel_energy: f64, // Native ZoneTotCoolFuelEnergy
    pub(crate) tot_heat_fuel_rate: f64, // Native TotHeatFuelRate
    pub(crate) tot_cool_fuel_rate: f64, // Native TotCoolFuelRate
    pub(crate) tot_heat_fuel_energy: f64, // Native TotHeatFuelEnergy
    pub(crate) tot_cool_fuel_energy: f64, // Native TotCoolFuelEnergy
    pub(crate) heating_fuel_type: SourcePurchasedAirFuel, // Native heatingFuelType
    pub(crate) cooling_fuel_type: SourcePurchasedAirFuel, // Native coolingFuelType
}
impl Default for SourceZonePurchasedAir {
    fn default() -> Self {
        Self {
            c_object_name: String::new(),
            name: String::new(),
            avail_sched: None,
            zone_supply_air_node_num: 0,
            zone_exhaust_air_node_num: 0,
            plenum_exhaust_air_node_num: 0,
            return_plenum_index: 0,
            purch_air_array_index: 0,
            return_plenum_name: String::new(),
            zone_recirc_air_node_num: 0,
            max_heat_supp_air_temp: 0.0,
            min_cool_supp_air_temp: 0.0,
            max_heat_supp_air_hum_rat: 0.0,
            min_cool_supp_air_hum_rat: 0.0,
            heating_limit: SourcePurchasedAirLimit::Invalid,
            max_heat_vol_flow_rate: 0.0,
            max_heat_sens_cap: 0.0,
            cooling_limit: SourcePurchasedAirLimit::Invalid,
            max_cool_vol_flow_rate: 0.0,
            max_cool_tot_cap: 0.0,
            heat_avail_sched: None,
            cool_avail_sched: None,
            dehumid_ctrl_type: SourcePurchasedAirHumidityControl::Invalid,
            cool_shr: 0.0,
            humid_ctrl_type: SourcePurchasedAirHumidityControl::Invalid,
            oa_requirements_ptr: 0,
            dcv_type: SourcePurchasedAirDcv::Invalid,
            economizer_type: SourcePurchasedAirEconomizer::Invalid,
            outdoor_air: false,
            outdoor_air_node_num: 0,
            ht_rec_type: SourcePurchasedAirHeatRecovery::Invalid,
            ht_rec_sen_eff: 0.0,
            ht_rec_lat_eff: 0.0,
            oa_flow_frac_sched: None,
            max_heat_mass_flow_rate: 0.0,
            max_cool_mass_flow_rate: 0.0,
            ems_override_mdot_on: false,
            ems_value_mass_flow_rate: 0.0,
            ems_override_oa_mdot_on: false,
            ems_value_oa_mass_flow_rate: 0.0,
            ems_override_supply_temp_on: false,
            ems_value_supply_temp: 0.0,
            ems_override_supply_hum_rat_on: false,
            ems_value_supply_hum_rat: 0.0,
            min_oa_mass_flow_rate: 0.0,
            outdoor_air_mass_flow_rate: 0.0,
            outdoor_air_vol_flow_rate_std_rho: 0.0,
            supply_air_mass_flow_rate: 0.0,
            supply_air_vol_flow_rate_std_rho: 0.0,
            ht_rec_sen_output: 0.0,
            ht_rec_lat_output: 0.0,
            oa_sen_output: 0.0,
            oa_lat_output: 0.0,
            sen_output_to_zone: 0.0,
            lat_output_to_zone: 0.0,
            sen_coil_load: 0.0,
            lat_coil_load: 0.0,
            oa_flow_max_cool_output_error: 0,
            oa_flow_max_heat_output_error: 0,
            saturation_output_error: 0,
            oa_flow_max_cool_output_index: 0,
            oa_flow_max_heat_output_index: 0,
            saturation_output_index: 0,
            avail_status: 0,
            cool_err_index: 0,
            heat_err_index: 0,
            sen_heat_energy: 0.0,
            lat_heat_energy: 0.0,
            tot_heat_energy: 0.0,
            sen_cool_energy: 0.0,
            lat_cool_energy: 0.0,
            tot_cool_energy: 0.0,
            zone_sen_heat_energy: 0.0,
            zone_lat_heat_energy: 0.0,
            zone_tot_heat_energy: 0.0,
            zone_sen_cool_energy: 0.0,
            zone_lat_cool_energy: 0.0,
            zone_tot_cool_energy: 0.0,
            oa_sen_heat_energy: 0.0,
            oa_lat_heat_energy: 0.0,
            oa_tot_heat_energy: 0.0,
            oa_sen_cool_energy: 0.0,
            oa_lat_cool_energy: 0.0,
            oa_tot_cool_energy: 0.0,
            ht_rec_sen_heat_energy: 0.0,
            ht_rec_lat_heat_energy: 0.0,
            ht_rec_tot_heat_energy: 0.0,
            ht_rec_sen_cool_energy: 0.0,
            ht_rec_lat_cool_energy: 0.0,
            ht_rec_tot_cool_energy: 0.0,
            sen_heat_rate: 0.0,
            lat_heat_rate: 0.0,
            tot_heat_rate: 0.0,
            sen_cool_rate: 0.0,
            lat_cool_rate: 0.0,
            tot_cool_rate: 0.0,
            zone_sen_heat_rate: 0.0,
            zone_lat_heat_rate: 0.0,
            zone_tot_heat_rate: 0.0,
            zone_sen_cool_rate: 0.0,
            zone_lat_cool_rate: 0.0,
            zone_tot_cool_rate: 0.0,
            oa_sen_heat_rate: 0.0,
            oa_lat_heat_rate: 0.0,
            oa_tot_heat_rate: 0.0,
            oa_sen_cool_rate: 0.0,
            oa_lat_cool_rate: 0.0,
            oa_tot_cool_rate: 0.0,
            ht_rec_sen_heat_rate: 0.0,
            ht_rec_lat_heat_rate: 0.0,
            ht_rec_tot_heat_rate: 0.0,
            ht_rec_sen_cool_rate: 0.0,
            ht_rec_lat_cool_rate: 0.0,
            ht_rec_tot_cool_rate: 0.0,
            time_econo_active: 0.0,
            time_ht_rec_active: 0.0,
            zone_ptr: 0,
            hvac_sizing_index: 0,
            supply_temp: 0.0,
            supply_hum_rat: 0.0,
            mixed_air_temp: 0.0,
            mixed_air_hum_rat: 0.0,
            heat_fuel_eff_sched: None,
            cool_fuel_eff_sched: None,
            zone_tot_heat_fuel_rate: 0.0,
            zone_tot_cool_fuel_rate: 0.0,
            zone_tot_heat_fuel_energy: 0.0,
            zone_tot_cool_fuel_energy: 0.0,
            tot_heat_fuel_rate: 0.0,
            tot_cool_fuel_rate: 0.0,
            tot_heat_fuel_energy: 0.0,
            tot_cool_fuel_energy: 0.0,
            heating_fuel_type: SourcePurchasedAirFuel::DistrictHeatingWater,
            cooling_fuel_type: SourcePurchasedAirFuel::DistrictCooling,
        }
    }
}

/// Native PurchAirNumericFieldData contains the actual Array1D_string owner.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourcePurchasedAirNumericFieldData {
    pub(crate) field_names: Option<Vec<String>>,
}

/// Native PurchAirPlenumArrayData constructor324: counts0, both arrays unallocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourcePurchasedAirPlenumArrayData {
    pub(crate) number_purchased_air: i32,
    pub(crate) return_plenum_index: i32,
    pub(crate) purchased_air: Option<Vec<i32>>,
    pub(crate) is_simulated: Option<Vec<bool>>,
}

/// Complete actual PurchasedAirManagerData.hh391-404 constructor state.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourcePurchasedAirManager {
    pub(crate) number_purchased_air: i32, // Native NumPurchAir
    pub(crate) number_plenum_arrays: i32, // Native NumPlenumArrays
    pub(crate) get_purchased_air_input: bool, // Native GetPurchAirInputFlag
    pub(crate) check_equipment_name: Option<Vec<bool>>, // Native CheckEquipName
    pub(crate) units: Option<Vec<SourceZonePurchasedAir>>, // Native PurchAir
    pub(crate) numeric_fields: Option<Vec<SourcePurchasedAirNumericFieldData>>, // Native PurchAirNumericFields
    pub(crate) plenum_arrays: Option<Vec<SourcePurchasedAirPlenumArrayData>>, // Native PurchAirPlenumArrays
    pub(crate) initialize_once: bool, // Native InitPurchasedAirMyOneTimeFlag
    pub(crate) zone_equipment_list_checked: bool, // Native InitPurchasedAirZoneEquipmentListChecked
    pub(crate) environment_flags: Option<Vec<bool>>, // Native InitPurchasedAirMyEnvrnFlag
    pub(crate) sizing_flags: Option<Vec<bool>>, // Native InitPurchasedAirMySizeFlag
    pub(crate) unit_initializations_done: Option<Vec<bool>>, // Native InitPurchasedAirOneTimeUnitInitsDone
    pub(crate) temporary_plenum_arrays: Option<Vec<SourcePurchasedAirPlenumArrayData>>, // Native TempPurchAirPlenumArrays
}
impl Default for SourcePurchasedAirManager {
    fn default() -> Self {
        Self {
            number_purchased_air: 0,
            number_plenum_arrays: 0,
            get_purchased_air_input: true,
            check_equipment_name: None,
            units: None,
            numeric_fields: None,
            plenum_arrays: None,
            initialize_once: true,
            zone_equipment_list_checked: false,
            environment_flags: None,
            sizing_flags: None,
            unit_initializations_done: None,
            temporary_plenum_arrays: None,
        }
    }
}
