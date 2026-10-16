//! Sole selected DataHeatBalFanSys Zone allocations at HBManager2864-2878.
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneFanHeatBalanceArrays {
    pub(crate) thermostat: crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceAirThermostatArrays,
    pub(crate) sum_conv_ht_rad_sys: Option<Vec<f64>>,
    pub(crate) sum_latent_ht_rad_sys: Option<Vec<f64>>,
    pub(crate) sum_conv_pool: Option<Vec<f64>>,
    pub(crate) sum_latent_pool: Option<Vec<f64>>,
    pub(crate) zone_qdot_rad_hvac_to_person: Option<Vec<f64>>,
    pub(crate) zone_q_ht_rad_sys_to_person: Option<Vec<f64>>,
    pub(crate) zone_q_hw_baseboard_to_person: Option<Vec<f64>>,
    pub(crate) zone_q_steam_baseboard_to_person: Option<Vec<f64>>,
    pub(crate) zone_q_elec_baseboard_to_person: Option<Vec<f64>>,
    pub(crate) zone_q_cooling_panel_to_person: Option<Vec<f64>>,
    pub(crate) zone_reorder: Option<Vec<Option<i32>>>,
    pub(crate) zone_mass_balance_flag: Option<Vec<bool>>,
    pub(crate) zone_infiltration_flag: Option<Vec<bool>>,
    pub(crate) temp_tstat_air: Option<Vec<f64>>,
}
impl SourceZoneFanHeatBalanceArrays {
    /// Called at2864 after actual AirHB + eight enclosure allocations return.
    /// Later contaminant/warmup/report allocations are independent Source code.
    pub(crate) fn allocate_source_fan_zone_arrays(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        let n = usize::try_from(globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual fan NumOfZones extent"))?;
        self.sum_conv_ht_rad_sys = Some(vec![0.0; n]);
        self.sum_latent_ht_rad_sys = Some(vec![0.0; n]);
        self.sum_conv_pool = Some(vec![0.0; n]);
        self.sum_latent_pool = Some(vec![0.0; n]);
        self.zone_qdot_rad_hvac_to_person = Some(vec![0.0; n]);
        self.zone_q_ht_rad_sys_to_person = Some(vec![0.0; n]);
        self.zone_q_hw_baseboard_to_person = Some(vec![0.0; n]);
        self.zone_q_steam_baseboard_to_person = Some(vec![0.0; n]);
        self.zone_q_elec_baseboard_to_person = Some(vec![0.0; n]);
        self.zone_q_cooling_panel_to_person = Some(vec![0.0; n]);
        self.zone_reorder = Some(vec![None; n]);
        self.zone_mass_balance_flag = Some(vec![false; n]);
        self.zone_infiltration_flag = Some(vec![false; n]);
        self.zone_reorder
            .as_mut()
            .ok_or(Error::Unavailable("actual ZoneReOrder allocated owner"))?
            .fill(Some(0));
        self.temp_tstat_air = Some(vec![23.0; n]);
        Ok(())
    }
}
