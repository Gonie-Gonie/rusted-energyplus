//! Complete genuine ZoneSpaceHeatBalanceData constructor and retained air owners.
//! Allocation2832/2834 and HBSM reconstruction2231-2250 remain separate calls.
//! This owner has no copied physical ZoneHeatBalanceState or Native seed.
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainLatentOwners;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

/// Original ZoneTempPredictorCorrector.hh102-209, all scalar/array constructors.
/// Source MAT/latent/history members persist in this one record across calls.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZoneSpaceHeatBalanceData {
    /// Source MAT.
    pub(crate) mat: f64,
    /// Source MRT.
    pub(crate) mrt: f64,
    /// Source ZTAV.
    pub(crate) ztav: f64,
    /// Source ZT.
    pub(crate) zt: f64,
    /// Source ZTAVComf.
    pub(crate) ztav_comf: f64,
    /// Source XMPT.
    pub(crate) xmpt: f64,
    /// Source XMAT.
    pub(crate) xmat: [f64; 4],
    /// Source DSXMAT.
    pub(crate) dsxmat: [f64; 4],
    /// Source TMX.
    pub(crate) tmx: f64,
    /// Source TM2.
    pub(crate) tm2: f64,
    /// Source T1.
    pub(crate) t1: f64,
    /// Source airHumRat.
    pub(crate) air_hum_rat: f64,
    /// Source airHumRatAvg.
    pub(crate) air_hum_rat_avg: f64,
    /// Source airHumRatTemp.
    pub(crate) air_hum_rat_temp: f64,
    /// Source airHumRatAvgComf.
    pub(crate) air_hum_rat_avg_comf: f64,
    /// Source WPrevZoneTS.
    pub(crate) w_prev_zone_ts: [f64; 4],
    /// Source DSWPrevZoneTS.
    pub(crate) dsw_prev_zone_ts: [f64; 4],
    /// Source WTimeMinusP.
    pub(crate) w_time_minus_p: f64,
    /// Source WMX.
    pub(crate) wmx: f64,
    /// Source WM2.
    pub(crate) wm2: f64,
    /// Source W1.
    pub(crate) w1: f64,
    /// Source ZTM.
    pub(crate) ztm: [f64; 4],
    /// Source WPrevZoneTSTemp.
    pub(crate) w_prev_zone_ts_temp: [f64; 4],
    /// Source SumIntGain.
    pub(crate) sum_int_gain: f64,
    /// Source SumHA.
    pub(crate) sum_ha: f64,
    /// Source SumHATsurf.
    pub(crate) sum_ha_tsurf: f64,
    /// Source SumHATref.
    pub(crate) sum_ha_tref: f64,
    /// Source SumMCp.
    pub(crate) sum_m_cp: f64,
    /// Source SumMCpT.
    pub(crate) sum_m_cp_t: f64,
    /// Source SumSysMCp.
    pub(crate) sum_sys_m_cp: f64,
    /// Source SumSysMCpT.
    pub(crate) sum_sys_m_cp_t: f64,
    /// Source SumIntGainExceptPeople.
    pub(crate) sum_int_gain_except_people: f64,
    /// Source SumHmAW.
    pub(crate) sum_hm_aw: f64,
    /// Source SumHmARa.
    pub(crate) sum_hm_a_ra: f64,
    /// Source SumHmARaW.
    pub(crate) sum_hm_a_ra_w: f64,
    /// Source SumHmARaZ.
    pub(crate) sum_hm_a_ra_z: f64,
    /// Source TempDepCoef.
    pub(crate) temp_dep_coef: f64,
    /// Source TempIndCoef.
    pub(crate) temp_ind_coef: f64,
    /// Source TempHistoryTerm.
    pub(crate) temp_history_term: f64,
    /// Source MCPI.
    pub(crate) mcpi: f64,
    /// Source MCPTI.
    pub(crate) mcpti: f64,
    /// Source MCPV.
    pub(crate) mcpv: f64,
    /// Source MCPTV.
    pub(crate) mcptv: f64,
    /// Source MCPM.
    pub(crate) mcpm: f64,
    /// Source MCPTM.
    pub(crate) mcptm: f64,
    /// Source MCPE.
    pub(crate) mcpe: f64,
    /// Source EAMFL.
    pub(crate) eamfl: f64,
    /// Source EAMFLxHumRat.
    pub(crate) eamf_lx_hum_rat: f64,
    /// Source MCPTE.
    pub(crate) mcpte: f64,
    /// Source MCPC.
    pub(crate) mcpc: f64,
    /// Source CTMFL.
    pub(crate) ctmfl: f64,
    /// Source MCPTC.
    pub(crate) mcptc: f64,
    /// Source ThermChimAMFL.
    pub(crate) therm_chim_amfl: f64,
    /// Source MCPTThermChim.
    pub(crate) mcpt_therm_chim: f64,
    /// Source MCPThermChim.
    pub(crate) mcp_therm_chim: f64,
    /// Source latentGain.
    pub(crate) latent_gain: f64,
    /// Source latentGainExceptPeople.
    pub(crate) latent_gain_except_people: f64,
    /// Source OAMFL.
    pub(crate) oamfl: f64,
    /// Source VAMFL.
    pub(crate) vamfl: f64,
    /// Source NonAirSystemResponse.
    pub(crate) non_air_system_response: f64,
    /// Source SysDepZoneLoads.
    pub(crate) sys_dep_zone_loads: f64,
    /// Source SysDepZoneLoadsLagged.
    pub(crate) sys_dep_zone_loads_lagged: f64,
    /// Source MDotCPOA.
    pub(crate) m_dot_cpoa: f64,
    /// Source MDotOA.
    pub(crate) m_dot_oa: f64,
    /// Source MixingMAT.
    pub(crate) mixing_mat: f64,
    /// Source MixingHumRat.
    pub(crate) mixing_hum_rat: f64,
    /// Source MixingMassFlowZone.
    pub(crate) mixing_mass_flow_zone: f64,
    /// Source MixingMassFlowXHumRat.
    pub(crate) mixing_mass_flow_x_hum_rat: f64,
    /// Source setPointLast.
    pub(crate) set_point_last: f64,
    /// Source tempIndLoad.
    pub(crate) temp_ind_load: f64,
    /// Source tempDepLoad.
    pub(crate) temp_dep_load: f64,
    /// Source airRelHum.
    pub(crate) air_rel_hum: f64,
    /// Source AirPowerCap.
    pub(crate) air_power_cap: f64,
    /// Source hmThermalMassMultErrIndex.
    pub(crate) hm_thermal_mass_mult_err_index: i32,
}
impl Default for SourceZoneSpaceHeatBalanceData {
    fn default() -> Self {
        Self {
            mat: 23.0,
            mrt: 23.0,
            ztav: 23.0,
            zt: 23.0,
            ztav_comf: 23.0,
            xmpt: 23.0,
            xmat: [23.0; 4],
            dsxmat: [23.0; 4],
            tmx: 23.0,
            tm2: 23.0,
            t1: 0.0,
            air_hum_rat: 0.01,
            air_hum_rat_avg: 0.01,
            air_hum_rat_temp: 0.01,
            air_hum_rat_avg_comf: 0.01,
            w_prev_zone_ts: [0.0; 4],
            dsw_prev_zone_ts: [0.0; 4],
            w_time_minus_p: 0.0,
            wmx: 0.0,
            wm2: 0.0,
            w1: 0.0,
            ztm: [0.0; 4],
            w_prev_zone_ts_temp: [0.0; 4],
            sum_int_gain: 0.0,
            sum_ha: 0.0,
            sum_ha_tsurf: 0.0,
            sum_ha_tref: 0.0,
            sum_m_cp: 0.0,
            sum_m_cp_t: 0.0,
            sum_sys_m_cp: 0.0,
            sum_sys_m_cp_t: 0.0,
            sum_int_gain_except_people: 0.0,
            sum_hm_aw: 0.0,
            sum_hm_a_ra: 0.0,
            sum_hm_a_ra_w: 0.0,
            sum_hm_a_ra_z: 0.0,
            temp_dep_coef: 0.0,
            temp_ind_coef: 0.0,
            temp_history_term: 0.0,
            mcpi: 0.0,
            mcpti: 0.0,
            mcpv: 0.0,
            mcptv: 0.0,
            mcpm: 0.0,
            mcptm: 0.0,
            mcpe: 0.0,
            eamfl: 0.0,
            eamf_lx_hum_rat: 0.0,
            mcpte: 0.0,
            mcpc: 0.0,
            ctmfl: 0.0,
            mcptc: 0.0,
            therm_chim_amfl: 0.0,
            mcpt_therm_chim: 0.0,
            mcp_therm_chim: 0.0,
            latent_gain: 0.0,
            latent_gain_except_people: 0.0,
            oamfl: 0.0,
            vamfl: 0.0,
            non_air_system_response: 0.0,
            sys_dep_zone_loads: 0.0,
            sys_dep_zone_loads_lagged: 0.0,
            m_dot_cpoa: 0.0,
            m_dot_oa: 0.0,
            mixing_mat: 23.0,
            mixing_hum_rat: 0.01,
            mixing_mass_flow_zone: 0.0,
            mixing_mass_flow_x_hum_rat: 0.0,
            set_point_last: 0.0,
            temp_ind_load: 0.0,
            temp_dep_load: 0.0,
            air_rel_hum: 0.0,
            air_power_cap: 0.0,
            hm_thermal_mass_mult_err_index: 0,
        }
    }
}

/// Actual derived Zone class adds no fields; its distinct sums method remains
/// an operation on this same base record, not a second copied numeric owner.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneHeatBalanceData {
    pub(crate) air: SourceZoneSpaceHeatBalanceData,
}
/// Actual derived Space class likewise adds no constructor fields.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSpaceHeatBalanceData {
    pub(crate) air: SourceZoneSpaceHeatBalanceData,
}

/// Sole DataZoneTempPredictorCorrector allocated arrays; None is unallocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceAirHeatBalance {
    pub(crate) zones: Option<Vec<SourceZoneHeatBalanceData>>,
    pub(crate) spaces: Option<Vec<SourceSpaceHeatBalanceData>>,
}

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or(Error::Unavailable("actual positive Source air ordinal"))
}

impl SourceAirHeatBalance {
    /// Exact AllocateZoneHeatBalArrays2832-2834. The actual caller performs
    /// conditional AllocateIntGains first and eight enclosure allocations next.
    pub(crate) fn allocate_source_arrays(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        let zones = usize::try_from(globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual NumOfZones for AirHB allocation"))?;
        if self.zones.is_some() {
            return Err(Error::Unavailable(
                "actual zoneHeatBalance allocate requires unallocated owner",
            ));
        }
        self.zones = Some(vec![SourceZoneHeatBalanceData::default(); zones]);
        let spaces = usize::try_from(globals.num_spaces)
            .map_err(|_| Error::Unavailable("actual numSpaces for AirHB allocation"))?;
        if self.spaces.is_some() {
            return Err(Error::Unavailable(
                "actual spaceHeatBalance allocate requires unallocated owner",
            ));
        }
        // Always allocated independently, including when doSpaceHeatBalance=false.
        self.spaces = Some(vec![SourceSpaceHeatBalanceData::default(); spaces]);
        Ok(())
    }

    /// HBSM2231-2239 zone placement reconstruction. The genuine caller invokes
    /// the real enclosure MRT assignment2241-2243 before reconstructing Spaces.
    pub(crate) fn reconstruct_source_zone_environment(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        out_humidity_ratio: f64,
        actual_thermostat_air: &mut [f64],
    ) -> Result<(), Error> {
        let zones = self
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("actual zoneHeatBalance allocation"))?;
        for number in 1..=globals.num_of_zones {
            let i = index(number)?;
            let row = zones
                .get_mut(i)
                .ok_or(Error::Unavailable("actual Source Zone AirHB ordinal"))?;
            *row = SourceZoneHeatBalanceData::default();
            row.air.air_hum_rat_avg = out_humidity_ratio;
            row.air.air_hum_rat = out_humidity_ratio;
            *actual_thermostat_air
                .get_mut(i)
                .ok_or(Error::Unavailable("actual TempTstatAir allocation"))? = 23.0;
        }
        Ok(())
    }

    /// HBSM2245-2250 visits the actual entire allocated Space owner even when
    /// doSpaceHeatBalance is false. Humidity writes preserve raw signed zero.
    pub(crate) fn reconstruct_source_space_environment(
        &mut self,
        out_humidity_ratio: f64,
    ) -> Result<(), Error> {
        for row in self
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("actual spaceHeatBalance allocation"))?
        {
            *row = SourceSpaceHeatBalanceData::default();
            row.air.air_hum_rat_avg = out_humidity_ratio;
            row.air.air_hum_rat = out_humidity_ratio;
        }
        Ok(())
    }

    /// Direct immutable actual Space MAT reference for Surface::getInsideAirTemperature.
    pub(crate) fn source_space_mat(&self, number: i32) -> Result<&f64, Error> {
        Ok(&self
            .spaces
            .as_ref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable("same live actual Space AirHB MAT owner"))?
            .air
            .mat)
    }
}

impl SourceGainLatentOwners for SourceAirHeatBalance {
    fn space_latent_gain_mut(&mut self, number: i32) -> Result<&mut f64, Error> {
        Ok(&mut self
            .spaces
            .as_mut()
            .and_then(|rows| rows.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "same live actual Space AirHB latentGain owner",
            ))?
            .air
            .latent_gain)
    }
    fn zone_latent_gain_mut(&mut self, number: i32) -> Result<&mut f64, Error> {
        Ok(&mut self
            .zones
            .as_mut()
            .and_then(|rows| rows.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "same live actual Zone AirHB latentGain owner",
            ))?
            .air
            .latent_gain)
    }
}

impl SourceZoneSpaceHeatBalanceData {
    /// The distinct genuine beginEnvironmentInit2818-2836 member body. Whole
    /// placement reconstruction does not imply that the later member was called.
    pub(crate) fn begin_source_environment(&mut self, out_humidity_ratio: f64) {
        for i in 0..4 {
            self.ztm[i] = 0.0;
            self.w_prev_zone_ts[i] = out_humidity_ratio;
            self.dsw_prev_zone_ts[i] = out_humidity_ratio;
            self.w_prev_zone_ts_temp[i] = 0.0;
        }
        self.w_time_minus_p = out_humidity_ratio;
        self.w1 = out_humidity_ratio;
        self.wmx = out_humidity_ratio;
        self.wm2 = out_humidity_ratio;
        self.air_hum_rat_temp = 0.0;
        self.temp_ind_load = 0.0;
        self.temp_dep_load = 0.0;
        self.air_rel_hum = 0.0;
        self.air_power_cap = 0.0;
        self.t1 = 0.0;
    }
}

mod convection;
pub(crate) use convection::SourceSpaceAirView;

pub(crate) mod environment;
pub(crate) mod provider_host;

pub(crate) mod output_registration;
pub(crate) mod predictor_corrector;
