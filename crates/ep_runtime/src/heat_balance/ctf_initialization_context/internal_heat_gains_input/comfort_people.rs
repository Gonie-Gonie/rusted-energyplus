//! Sole DataHeatBalance.People row constructor750-821; positive People input is unported.
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
/// Native EPVector default std::vector empty and m_allocated=false, retained separately.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourcePeopleArray {
    pub(crate) allocated: bool,
    pub(crate) rows: Vec<SourcePeopleComfortData>,
}
impl SourcePeopleArray {
    pub(crate) fn get(&self, at: usize) -> Option<&SourcePeopleComfortData> {
        self.rows.get(at)
    }
    pub(crate) fn first(&self) -> Option<&SourcePeopleComfortData> {
        self.rows.first()
    }
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, SourcePeopleComfortData> {
        self.rows.iter()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourcePeopleComfortData {
    pub(crate) name: String,                                       // Native Name
    pub(crate) zone_ptr: i32,                                      // Native ZonePtr
    pub(crate) space_index: i32,                                   // Native spaceIndex
    pub(crate) number_of_people: f64,                              // Native NumberOfPeople
    pub(crate) sched: Option<SourceScheduleIndex>,                 // Native sched
    pub(crate) ems_people_on: bool,                                // Native EMSPeopleOn
    pub(crate) ems_number_of_people: f64,                          // Native EMSNumberOfPeople
    pub(crate) activity_level_sched: Option<SourceScheduleIndex>,  // Native activityLevelSched
    pub(crate) fraction_radiant: f64,                              // Native FractionRadiant
    pub(crate) fraction_convected: f64,                            // Native FractionConvected
    pub(crate) nom_min_number_people: f64,                         // Native NomMinNumberPeople
    pub(crate) nom_max_number_people: f64,                         // Native NomMaxNumberPeople
    pub(crate) work_eff_sched: Option<SourceScheduleIndex>,        // Native workEffSched
    pub(crate) clothing_sched: Option<SourceScheduleIndex>,        // Native clothingSched
    pub(crate) clothing_method_sched: Option<SourceScheduleIndex>, // Native clothingMethodSched
    pub(crate) clothing_type: i32,                                 // Native clothingType
    pub(crate) air_velocity_sched: Option<SourceScheduleIndex>,    // Native airVelocitySched
    pub(crate) ankle_air_velocity_sched: Option<SourceScheduleIndex>, // Native ankleAirVelocitySched
    pub(crate) fanger: bool,                                          // Native Fanger
    pub(crate) pierce: bool,                                          // Native Pierce
    pub(crate) ksu: bool,                                             // Native KSU
    pub(crate) adaptive_ash55: bool,                                  // Native AdaptiveASH55
    pub(crate) adaptive_cen15251: bool,                               // Native AdaptiveCEN15251
    pub(crate) cooling_effect_ash55: bool,                            // Native CoolingEffectASH55
    pub(crate) ankle_draft_ash55: bool,                               // Native AnkleDraftASH55
    pub(crate) mrt_calc_type: i32,                                    // Native MRTCalcType
    pub(crate) surface_ptr: i32,                                      // Native SurfacePtr
    pub(crate) angle_factor_list_name: String,                        // Native AngleFactorListName
    pub(crate) angle_factor_list_ptr: i32,                            // Native AngleFactorListPtr
    pub(crate) user_spec_sens_frac: f64,                              // Native UserSpecSensFrac
    pub(crate) show55_warning: bool,                                  // Native Show55Warning
    pub(crate) co2_rate_factor: f64,                                  // Native CO2RateFactor
    pub(crate) num_occ: f64,                                          // Native NumOcc
    pub(crate) temperature_in_zone: f64,                              // Native TemperatureInZone
    pub(crate) cold_stress_temp_thresh: f64,                          // Native ColdStressTempThresh
    pub(crate) heat_stress_temp_thresh: f64,                          // Native HeatStressTempThresh
    pub(crate) relative_humidity_in_zone: f64, // Native RelativeHumidityInZone
    pub(crate) rad_gain_rate: f64,             // Native RadGainRate
    pub(crate) con_gain_rate: f64,             // Native ConGainRate
    pub(crate) sen_gain_rate: f64,             // Native SenGainRate
    pub(crate) lat_gain_rate: f64,             // Native LatGainRate
    pub(crate) tot_gain_rate: f64,             // Native TotGainRate
    pub(crate) co2_gain_rate: f64,             // Native CO2GainRate
    pub(crate) rad_gain_energy: f64,           // Native RadGainEnergy
    pub(crate) con_gain_energy: f64,           // Native ConGainEnergy
    pub(crate) sen_gain_energy: f64,           // Native SenGainEnergy
    pub(crate) lat_gain_energy: f64,           // Native LatGainEnergy
    pub(crate) tot_gain_energy: f64,           // Native TotGainEnergy
    pub(crate) air_vel_err_index: i32,         // Native AirVelErrIndex
    pub(crate) time_not_met_ash5580: f64,      // Native TimeNotMetASH5580
    pub(crate) time_not_met_ash5590: f64,      // Native TimeNotMetASH5590
    pub(crate) time_not_met_cen15251_cat_i: f64, // Native TimeNotMetCEN15251CatI
    pub(crate) time_not_met_cen15251_cat_ii: f64, // Native TimeNotMetCEN15251CatII
    pub(crate) time_not_met_cen15251_cat_iii: f64, // Native TimeNotMetCEN15251CatIII
}
impl Default for SourcePeopleComfortData {
    fn default() -> Self {
        Self {
            name: String::new(),
            zone_ptr: 0,
            space_index: 0,
            number_of_people: 0.0,
            sched: None,
            ems_people_on: false,
            ems_number_of_people: 0.0,
            activity_level_sched: None,
            fraction_radiant: 0.0,
            fraction_convected: 0.0,
            nom_min_number_people: 0.0,
            nom_max_number_people: 0.0,
            work_eff_sched: None,
            clothing_sched: None,
            clothing_method_sched: None,
            clothing_type: -1,
            air_velocity_sched: None,
            ankle_air_velocity_sched: None,
            fanger: false,
            pierce: false,
            ksu: false,
            adaptive_ash55: false,
            adaptive_cen15251: false,
            cooling_effect_ash55: false,
            ankle_draft_ash55: false,
            mrt_calc_type: -1,
            surface_ptr: -1,
            angle_factor_list_name: String::new(),
            angle_factor_list_ptr: -1,
            user_spec_sens_frac: 0.0,
            show55_warning: false,
            co2_rate_factor: 0.0,
            num_occ: 0.0,
            temperature_in_zone: 0.0,
            cold_stress_temp_thresh: 15.56,
            heat_stress_temp_thresh: 30.0,
            relative_humidity_in_zone: 0.0,
            rad_gain_rate: 0.0,
            con_gain_rate: 0.0,
            sen_gain_rate: 0.0,
            lat_gain_rate: 0.0,
            tot_gain_rate: 0.0,
            co2_gain_rate: 0.0,
            rad_gain_energy: 0.0,
            con_gain_energy: 0.0,
            sen_gain_energy: 0.0,
            lat_gain_energy: 0.0,
            tot_gain_energy: 0.0,
            air_vel_err_index: 0,
            time_not_met_ash5580: 0.0,
            time_not_met_ash5590: 0.0,
            time_not_met_cen15251_cat_i: 0.0,
            time_not_met_cen15251_cat_ii: 0.0,
            time_not_met_cen15251_cat_iii: 0.0,
        }
    }
}
