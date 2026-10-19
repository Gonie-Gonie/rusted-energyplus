//! Genuine ZoneEnergyDemand/HeatBalance AirReport allocated owners.
// Sequenced EPVector arrays preserve None allocation vs Some allocation.
// Their real allocate() performs value-initialization and an explicit T{} fill.
use super::*;
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceAirReport {
    pub(crate) mean_air_temp: f64,
    pub(crate) operative_temp: f64,
    pub(crate) wetbulb_globe_temp: f64,
    pub(crate) mean_air_hum_rat: f64,
    pub(crate) mean_air_dew_point_temp: f64,
    pub(crate) therm_operative_temp: f64,
    pub(crate) infil_heat_gain: f64,
    pub(crate) infil_heat_loss: f64,
    pub(crate) infil_latent_gain: f64,
    pub(crate) infil_latent_loss: f64,
    pub(crate) infil_total_gain: f64,
    pub(crate) infil_total_loss: f64,
    pub(crate) infil_volume_cur_density: f64,
    pub(crate) infil_volume_std_density: f64,
    pub(crate) infil_vdot_cur_density: f64,
    pub(crate) infil_vdot_std_density: f64,
    pub(crate) infil_vdot_out_density: f64,
    pub(crate) infil_mass: f64,
    pub(crate) infil_mdot: f64,
    pub(crate) infil_air_change_rate_cur_density: f64,
    pub(crate) infil_air_change_rate_std_density: f64,
    pub(crate) infil_air_change_rate_out_density: f64,
    pub(crate) ventil_heat_loss: f64,
    pub(crate) ventil_heat_gain: f64,
    pub(crate) ventil_latent_loss: f64,
    pub(crate) ventil_latent_gain: f64,
    pub(crate) ventil_total_loss: f64,
    pub(crate) ventil_total_gain: f64,
    pub(crate) ventil_volume_cur_density: f64,
    pub(crate) ventil_volume_std_density: f64,
    pub(crate) ventil_vdot_cur_density: f64,
    pub(crate) ventil_vdot_std_density: f64,
    pub(crate) ventil_vdot_out_density: f64,
    pub(crate) ventil_mass: f64,
    pub(crate) ventil_mdot: f64,
    pub(crate) ventil_air_change_rate_cur_density: f64,
    pub(crate) ventil_air_change_rate_std_density: f64,
    pub(crate) ventil_air_change_rate_out_density: f64,
    pub(crate) ventil_fan_elec: f64,
    pub(crate) ventil_air_temp: f64,
    pub(crate) mix_volume: f64,
    pub(crate) mix_vdot_cur_density: f64,
    pub(crate) mix_vdot_std_density: f64,
    pub(crate) mix_mass: f64,
    pub(crate) mix_mdot: f64,
    pub(crate) mix_sen_load: f64,
    pub(crate) mix_lat_load: f64,
    pub(crate) mix_heat_loss: f64,
    pub(crate) mix_heat_gain: f64,
    pub(crate) mix_latent_loss: f64,
    pub(crate) mix_latent_gain: f64,
    pub(crate) mix_total_loss: f64,
    pub(crate) mix_total_gain: f64,
    pub(crate) sys_inlet_mass: f64,
    pub(crate) sys_outlet_mass: f64,
    pub(crate) exfil_mass: f64,
    pub(crate) exfil_total_loss: f64,
    pub(crate) exfil_sensi_loss: f64,
    pub(crate) exfil_latent_loss: f64,
    pub(crate) exh_total_loss: f64,
    pub(crate) exh_sensi_loss: f64,
    pub(crate) exh_latent_loss: f64,
    pub(crate) sum_int_gains: f64,
    pub(crate) sum_had_tsurfs: f64,
    pub(crate) sum_m_cp_d_tzones: f64,
    pub(crate) sum_m_cp_dt_infil: f64,
    pub(crate) sum_m_cp_d_tsystem: f64,
    pub(crate) sum_non_air_system: f64,
    pub(crate) czd_tdt: f64,
    pub(crate) im_balance: f64,
    pub(crate) oa_balance_heat_loss: f64,
    pub(crate) oa_balance_heat_gain: f64,
    pub(crate) oa_balance_latent_loss: f64,
    pub(crate) oa_balance_latent_gain: f64,
    pub(crate) oa_balance_total_loss: f64,
    pub(crate) oa_balance_total_gain: f64,
    pub(crate) oa_balance_volume_cur_density: f64,
    pub(crate) oa_balance_volume_std_density: f64,
    pub(crate) oa_balance_vdot_cur_density: f64,
    pub(crate) oa_balance_vdot_std_density: f64,
    pub(crate) oa_balance_mass: f64,
    pub(crate) oa_balance_mdot: f64,
    pub(crate) oa_balance_air_change_rate: f64,
    pub(crate) oa_balance_fan_elec: f64,
    pub(crate) sum_enthalpy_m: f64,
    pub(crate) sum_enthalpy_h: f64,
    pub(crate) report_wbgt: bool,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneAirReports {
    pub(crate) zones: Option<Vec<SourceAirReport>>,
    pub(crate) spaces: Option<Vec<SourceAirReport>>,
}
impl SourceZoneAirReports {
    /// Actual GetSimpleAirModelInputs292; Space allocation294 is separate Source guard.
    pub(crate) fn allocate_source_reports(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        space_controls: &SelectedHeatBalanceSpaceControls,
    ) -> Result<(), Error> {
        let zones = usize::try_from(globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual ZnAirRpt NumOfZones292"))?;
        if self.zones.is_some() {
            return Err(Error::Unavailable(
                "repeated native ZnAirRpt allocate292 state not yet represented",
            ));
        }
        self.zones = Some(vec![SourceAirReport::default(); zones]);
        if space_controls.sizing || space_controls.simulation {
            let spaces = usize::try_from(globals.num_spaces)
                .map_err(|_| Error::Unavailable("actual spaceAirRpt numSpaces294"))?;
            if self.spaces.is_some() {
                return Err(Error::Unavailable(
                    "repeated native spaceAirRpt allocate294 state not yet represented",
                ));
            }
            self.spaces = Some(vec![SourceAirReport::default(); spaces]);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSystemDemandBase {
    pub(crate) remaining_output_required: f64,
    pub(crate) unadjusted_remaining_output_required: f64,
    pub(crate) total_output_required: f64,
    pub(crate) num_zone_equipment: i32,
    pub(crate) supply_air_adjust_factor: f64,
    pub(crate) stage_num: i32,
}
impl Default for SourceSystemDemandBase {
    fn default() -> Self {
        Self {
            remaining_output_required: 0.0,
            unadjusted_remaining_output_required: 0.0,
            total_output_required: 0.0,
            num_zone_equipment: 0,
            supply_air_adjust_factor: 1.0,
            stage_num: 0,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSystemSensibleDemand {
    pub(crate) base: SourceSystemDemandBase,
    pub(crate) output_required_to_heating_sp: f64,
    pub(crate) output_required_to_cooling_sp: f64,
    pub(crate) remaining_output_to_heat_sp: f64,
    pub(crate) remaining_output_to_cool_sp: f64,
    pub(crate) unadjusted_remaining_output_to_heat_sp: f64,
    pub(crate) unadjusted_remaining_output_to_cool_sp: f64,
    pub(crate) sequenced_output_required: Option<Vec<f64>>,
    pub(crate) sequenced_output_to_heating_sp: Option<Vec<f64>>,
    pub(crate) sequenced_output_to_cooling_sp: Option<Vec<f64>>,
    pub(crate) predicted_rate: f64,
    pub(crate) predicted_heating_sp_rate: f64,
    pub(crate) predicted_cooling_sp_rate: f64,
    pub(crate) air_sys_heat_rate: f64,
    pub(crate) air_sys_cool_rate: f64,
    pub(crate) air_sys_heat_energy: f64,
    pub(crate) air_sys_cool_energy: f64,
}
impl SourceSystemSensibleDemand {
    pub(super) fn report_source_sensible_load(&mut self, load: f64, seconds: f64) {
        self.air_sys_heat_rate = cpp_max(load, 0.0);
        self.air_sys_cool_rate = cpp_min(load, 0.0).abs();
        self.air_sys_heat_energy = self.air_sys_heat_rate * seconds;
        self.air_sys_cool_energy = self.air_sys_cool_rate * seconds;
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSystemMoistureDemand {
    pub(crate) base: SourceSystemDemandBase,
    pub(crate) output_required_to_humidifying_sp: f64,
    pub(crate) output_required_to_dehumidifying_sp: f64,
    pub(crate) remaining_output_to_humid_sp: f64,
    pub(crate) remaining_output_to_dehumid_sp: f64,
    pub(crate) unadjusted_remaining_output_to_humid_sp: f64,
    pub(crate) unadjusted_remaining_output_to_dehumid_sp: f64,
    pub(crate) sequenced_output_required: Option<Vec<f64>>,
    pub(crate) sequenced_output_to_humid_sp: Option<Vec<f64>>,
    pub(crate) sequenced_output_to_dehumid_sp: Option<Vec<f64>>,
    pub(crate) predicted_rate: f64,
    pub(crate) predicted_humid_sp_rate: f64,
    pub(crate) predicted_dehumid_sp_rate: f64,
    pub(crate) air_sys_heat_rate: f64,
    pub(crate) air_sys_cool_rate: f64,
    pub(crate) air_sys_heat_energy: f64,
    pub(crate) air_sys_cool_energy: f64,
    pub(crate) air_sys_sensible_heat_ratio: f64,
    pub(crate) vapor_pressure_difference: f64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneAirDemand {
    pub(crate) setback: Option<Vec<bool>>,
    pub(crate) deadband_or_setback: Option<Vec<bool>>,
    pub(crate) current_deadband_or_setback: Option<Vec<bool>>,
    pub(crate) zones: Option<Vec<SourceSystemSensibleDemand>>,
    pub(crate) zone_moisture: Option<Vec<SourceSystemMoistureDemand>>,
    pub(crate) spaces: Option<Vec<SourceSystemSensibleDemand>>,
    pub(crate) space_moisture: Option<Vec<SourceSystemMoistureDemand>>,
}
impl SourceZoneAirDemand {
    /// Actual InitZoneAirSetPoints2412-2416. Caller owns all earlier/later allocations.
    pub(crate) fn allocate_source_demands(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        space_controls: &SelectedHeatBalanceSpaceControls,
    ) -> Result<(), Error> {
        let zones = usize::try_from(globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual ZoneDemand NumOfZones2412"))?;
        let rows = self.zones.get_or_insert_with(Vec::new);
        rows.resize(zones, SourceSystemSensibleDemand::default());
        rows.fill(SourceSystemSensibleDemand::default());
        let rows = self.zone_moisture.get_or_insert_with(Vec::new);
        rows.resize(zones, SourceSystemMoistureDemand::default());
        rows.fill(SourceSystemMoistureDemand::default());
        if space_controls.simulation || space_controls.sizing {
            let spaces = usize::try_from(globals.num_spaces)
                .map_err(|_| Error::Unavailable("actual SpaceDemand numSpaces2415"))?;
            let rows = self.spaces.get_or_insert_with(Vec::new);
            rows.resize(spaces, SourceSystemSensibleDemand::default());
            rows.fill(SourceSystemSensibleDemand::default());
            let rows = self.space_moisture.get_or_insert_with(Vec::new);
            rows.resize(spaces, SourceSystemMoistureDemand::default());
            rows.fill(SourceSystemMoistureDemand::default());
        }
        Ok(())
    }
}
