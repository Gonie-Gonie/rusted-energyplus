//! Genuine selected ThermalComfort module/row constructors; no per-step reconstruction.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceThermalComfortData {
    pub(crate) fanger_pmv: f64,                       // Native FangerPMV
    pub(crate) fanger_ppd: f64,                       // Native FangerPPD
    pub(crate) clo_surf_temp: f64,                    // Native CloSurfTemp
    pub(crate) pierce_pmvet: f64,                     // Native PiercePMVET
    pub(crate) pierce_pmvset: f64,                    // Native PiercePMVSET
    pub(crate) pierce_disc: f64,                      // Native PierceDISC
    pub(crate) pierce_tsens: f64,                     // Native PierceTSENS
    pub(crate) pierce_set: f64,                       // Native PierceSET
    pub(crate) ksu_tsv: f64,                          // Native KsuTSV
    pub(crate) thermal_comfort_mrt: f64,              // Native ThermalComfortMRT
    pub(crate) thermal_comfort_op_temp: f64,          // Native ThermalComfortOpTemp
    pub(crate) clothing_value: f64,                   // Native ClothingValue
    pub(crate) thermal_comfort_adaptive_ash5590: i32, // Native ThermalComfortAdaptiveASH5590
    pub(crate) thermal_comfort_adaptive_ash5580: i32, // Native ThermalComfortAdaptiveASH5580
    pub(crate) thermal_comfort_adaptive_cen15251_cat_i: i32, // Native ThermalComfortAdaptiveCEN15251CatI
    pub(crate) thermal_comfort_adaptive_cen15251_cat_ii: i32, // Native ThermalComfortAdaptiveCEN15251CatII
    pub(crate) thermal_comfort_adaptive_cen15251_cat_iii: i32, // Native ThermalComfortAdaptiveCEN15251CatIII
    pub(crate) t_comf_ash55: f64,                              // Native TComfASH55
    pub(crate) t_comf_cen15251: f64,                           // Native TComfCEN15251
    pub(crate) ashrae55_running_mean_outdoor_temp: f64, // Native ASHRAE55RunningMeanOutdoorTemp
    pub(crate) cen15251_running_mean_outdoor_temp: f64, // Native CEN15251RunningMeanOutdoorTemp
    pub(crate) cooling_effect_ash55: f64,               // Native CoolingEffectASH55
    pub(crate) cooling_effect_adjusted_pmvash55: f64,   // Native CoolingEffectAdjustedPMVASH55
    pub(crate) cooling_effect_adjusted_ppdash55: f64,   // Native CoolingEffectAdjustedPPDASH55
    pub(crate) ankle_draft_ppdash55: f64,               // Native AnkleDraftPPDASH55
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceThermalComfortInAsh55 {
    pub(crate) time_not_summer: f64,       // Native timeNotSummer
    pub(crate) time_not_winter: f64,       // Native timeNotWinter
    pub(crate) time_not_either: f64,       // Native timeNotEither
    pub(crate) total_time_not_summer: f64, // Native totalTimeNotSummer
    pub(crate) total_time_not_winter: f64, // Native totalTimeNotWinter
    pub(crate) total_time_not_either: f64, // Native totalTimeNotEither
    pub(crate) zone_is_occupied: bool,     // Native ZoneIsOccupied
    pub(crate) warning_index: i32,         // Native warningIndex
    pub(crate) warning_index2: i32,        // Native warningIndex2
    pub(crate) enable55_warning: bool,     // Native Enable55Warning
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceThermalComfortSetpoint {
    pub(crate) not_met_heating: f64,          // Native notMetHeating
    pub(crate) not_met_cooling: f64,          // Native notMetCooling
    pub(crate) not_met_heating_occupied: f64, // Native notMetHeatingOccupied
    pub(crate) not_met_cooling_occupied: f64, // Native notMetCoolingOccupied
    pub(crate) total_not_met_heating: f64,    // Native totalNotMetHeating
    pub(crate) total_not_met_cooling: f64,    // Native totalNotMetCooling
    pub(crate) total_not_met_heating_occupied: f64, // Native totalNotMetHeatingOccupied
    pub(crate) total_not_met_cooling_occupied: f64, // Native totalNotMetCoolingOccupied
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceAngleFactorData {
    pub(crate) angle_factor: Vec<f64>,
    pub(crate) name: String,
    pub(crate) surface_name: Vec<String>,
    pub(crate) surface_ptr: Vec<i32>,
    pub(crate) total_angle_factor_surfaces: i32,
    pub(crate) enclosure_ptr: i32,
}
/// Actual OutputReportPredefined scalar subset1685–1698, same retained owner.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceComfortPredefinedSummary {
    pub(crate) total_not_met_heating_occupied_for_abups: f64,
    pub(crate) total_not_met_cooling_occupied_for_abups: f64,
    pub(crate) total_not_met_occupied_for_abups: f64,
    pub(crate) total_time_not_simple_ash55_either_for_abups: f64,
    pub(crate) total_any_zone_below_voz_dyn_for_oa: f64,
    pub(crate) total_all_zones_at_voz_dyn_for_oa: f64,
    pub(crate) total_any_zone_above_voz_dyn_for_oa: f64,
    pub(crate) total_any_zone_below_voz_dyn_occ_for_oa: f64,
    pub(crate) total_all_zones_at_voz_dyn_occ_for_oa: f64,
    pub(crate) total_any_zone_above_voz_dyn_occ_for_oa: f64,
    pub(crate) total_any_zone_vent_unocc_for_oa: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceThermalComfort {
    pub(crate) first_time_flag: bool,
    pub(crate) temporary_six_am_temperature: f64,
    pub(crate) people_num: i32,
    pub(crate) zone_num: i32,
    pub(crate) thermal_comfort_data: Option<Vec<SourceThermalComfortData>>,
    pub(crate) thermal_comfort_in_ash55: Option<Vec<SourceThermalComfortInAsh55>>,
    pub(crate) thermal_comfort_setpoint: Option<Vec<SourceThermalComfortSetpoint>>,
    pub(crate) angle_factor_list: Option<Vec<SourceAngleFactorData>>,
    pub(crate) zone_occ_hrs: Option<Vec<f64>>,
    pub(crate) any_zone_time_not_simple_ash55_summer: f64, // Native AnyZoneTimeNotSimpleASH55Summer
    pub(crate) any_zone_time_not_simple_ash55_winter: f64, // Native AnyZoneTimeNotSimpleASH55Winter
    pub(crate) any_zone_time_not_simple_ash55_either: f64, // Native AnyZoneTimeNotSimpleASH55Either
    pub(crate) any_zone_not_met_heating: f64,              // Native AnyZoneNotMetHeating
    pub(crate) any_zone_not_met_cooling: f64,              // Native AnyZoneNotMetCooling
    pub(crate) any_zone_not_met_heating_occupied: f64,     // Native AnyZoneNotMetHeatingOccupied
    pub(crate) any_zone_not_met_cooling_occupied: f64,     // Native AnyZoneNotMetCoolingOccupied
    pub(crate) any_zone_not_met_occupied: f64,             // Native AnyZoneNotMetOccupied
    pub(crate) total_any_zone_time_not_simple_ash55_summer: f64, // Native TotalAnyZoneTimeNotSimpleASH55Summer
    pub(crate) total_any_zone_time_not_simple_ash55_winter: f64, // Native TotalAnyZoneTimeNotSimpleASH55Winter
    pub(crate) total_any_zone_time_not_simple_ash55_either: f64, // Native TotalAnyZoneTimeNotSimpleASH55Either
    pub(crate) total_any_zone_not_met_heating: f64,              // Native TotalAnyZoneNotMetHeating
    pub(crate) total_any_zone_not_met_cooling: f64,              // Native TotalAnyZoneNotMetCooling
    pub(crate) total_any_zone_not_met_heating_occupied: f64, // Native TotalAnyZoneNotMetHeatingOccupied
    pub(crate) total_any_zone_not_met_cooling_occupied: f64, // Native TotalAnyZoneNotMetCoolingOccupied
    pub(crate) total_any_zone_not_met_occupied: f64,         // Native TotalAnyZoneNotMetOccupied
}
impl Default for SourceThermalComfort {
    fn default() -> Self {
        Self {
            first_time_flag: true,
            temporary_six_am_temperature: 0.0,
            people_num: 0,
            zone_num: 0,
            thermal_comfort_data: None,
            thermal_comfort_in_ash55: None,
            thermal_comfort_setpoint: None,
            angle_factor_list: None,
            zone_occ_hrs: None,
            any_zone_time_not_simple_ash55_summer: 0.0,
            any_zone_time_not_simple_ash55_winter: 0.0,
            any_zone_time_not_simple_ash55_either: 0.0,
            any_zone_not_met_heating: 0.0,
            any_zone_not_met_cooling: 0.0,
            any_zone_not_met_heating_occupied: 0.0,
            any_zone_not_met_cooling_occupied: 0.0,
            any_zone_not_met_occupied: 0.0,
            total_any_zone_time_not_simple_ash55_summer: 0.0,
            total_any_zone_time_not_simple_ash55_winter: 0.0,
            total_any_zone_time_not_simple_ash55_either: 0.0,
            total_any_zone_not_met_heating: 0.0,
            total_any_zone_not_met_cooling: 0.0,
            total_any_zone_not_met_heating_occupied: 0.0,
            total_any_zone_not_met_cooling_occupied: 0.0,
            total_any_zone_not_met_occupied: 0.0,
        }
    }
}
