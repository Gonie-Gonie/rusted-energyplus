//! Selected zone-air environment initialization ownership.
//!
//! The fixed constructor, incoming zone-only reconstruction, selected member,
//! and global guard are distinct operations. Thermostat/day/Space siblings and
//! later temperature/history evolution are outside this bounded owner.

use super::state::ZoneHeatBalanceState;

const SOURCE_ZONE_INITIAL_TEMP_C: f64 = 23.0;

/// Actual selected source fields, including all four slots of six arrays.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneAirInitializationState {
    /// Source MAT; the selected member retains this value.
    pub mat: f64,
    /// Source ZT, distinct from MAT and retained by the selected member.
    pub zt: f64,
    /// Source ZTAV; the selected member retains this value.
    pub ztav: f64,
    /// Source XMPT.
    pub xmpt: f64,
    /// Source XMAT; the solver currently consumes the first three slots.
    pub xmat: [f64; 4],
    /// Source DSXMAT; the solver currently consumes the first three slots.
    pub dsxmat: [f64; 4],
    /// Source TMX.
    pub tmx: f64,
    /// Source TM2.
    pub tm2: f64,
    /// Source current airHumRat, assigned by incoming reconstruction.
    pub air_hum_rat: f64,
    /// Source airHumRatAvg, assigned by incoming reconstruction.
    pub air_hum_rat_avg: f64,
    /// Source ZTM, a separate working buffer from XMAT/DSXMAT.
    pub ztm: [f64; 4],
    /// Source WPrevZoneTS.
    pub w_prev_zone_ts: [f64; 4],
    /// Source DSWPrevZoneTS.
    pub dsw_prev_zone_ts: [f64; 4],
    /// Source WPrevZoneTSTemp.
    pub w_prev_zone_ts_temp: [f64; 4],
    /// Source WTimeMinusP.
    pub w_time_minus_p: f64,
    /// Source W1.
    pub w1: f64,
    /// Source WMX.
    pub wmx: f64,
    /// Source WM2.
    pub wm2: f64,
    /// Source airHumRatTemp; constructor .01, member +0.
    pub air_hum_rat_temp: f64,
    /// Source tempIndLoad, distinct from TempIndCoef.
    pub temp_ind_load: f64,
    /// Source tempDepLoad, distinct from TempDepCoef.
    pub temp_dep_load: f64,
    /// Source airRelHum.
    pub air_rel_hum: f64,
    /// Source AirPowerCap in W/K, distinct from stored capacity in J/K.
    pub air_power_cap: f64,
    /// Source T1.
    pub t1: f64,
}

impl Default for ZoneAirInitializationState {
    fn default() -> Self {
        Self {
            mat: SOURCE_ZONE_INITIAL_TEMP_C,
            zt: SOURCE_ZONE_INITIAL_TEMP_C,
            ztav: SOURCE_ZONE_INITIAL_TEMP_C,
            xmpt: SOURCE_ZONE_INITIAL_TEMP_C,
            xmat: [SOURCE_ZONE_INITIAL_TEMP_C; 4],
            dsxmat: [SOURCE_ZONE_INITIAL_TEMP_C; 4],
            tmx: SOURCE_ZONE_INITIAL_TEMP_C,
            tm2: SOURCE_ZONE_INITIAL_TEMP_C,
            air_hum_rat: 0.01,
            air_hum_rat_avg: 0.01,
            ztm: [0.0; 4],
            w_prev_zone_ts: [0.0; 4],
            dsw_prev_zone_ts: [0.0; 4],
            w_prev_zone_ts_temp: [0.0; 4],
            w_time_minus_p: 0.0,
            w1: 0.0,
            wmx: 0.0,
            wm2: 0.0,
            air_hum_rat_temp: 0.01,
            temp_ind_load: 0.0,
            temp_dep_load: 0.0,
            air_rel_hum: 0.0,
            air_power_cap: 0.0,
            t1: 0.0,
        }
    }
}

/// Explicit existing caller temperature inputs, separate from source defaults.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneAirCallerTemperatureInputs {
    /// Actual existing current temperature.
    pub mat: f64,
    /// Actual existing zone-average temperature.
    pub ztav: f64,
    /// Actual existing first-three XMAT values.
    pub xmat: [f64; 3],
    /// Actual existing first-three DSXMAT values.
    pub dsxmat: [f64; 3],
}

impl ZoneAirCallerTemperatureInputs {
    /// Read inputs from the existing initialized state without a temperature test.
    #[must_use]
    pub fn from_legacy_zone(zone: &ZoneHeatBalanceState) -> Self {
        Self {
            mat: zone.mean_air_temperature_c,
            ztav: zone.zone_timestep_average_air_temperature_c,
            xmat: zone.previous_mean_air_temperatures_c,
            dsxmat: zone.previous_system_mean_air_temperatures_c,
        }
    }
}

impl ZoneAirInitializationState {
    /// Selected incoming zone reconstruction from HBSurfaceManager2231-2239.
    /// TempTstatAir and other owners are not fields of this selected bundle.
    pub fn bulk_reconstruct_and_current_w_seed(&mut self, out_hum_rat: f64) {
        *self = Self::default();
        self.air_hum_rat_avg = out_hum_rat;
        self.air_hum_rat = out_hum_rat;
    }

    /// Source beginEnvironmentInit2818-2836, preserving its assignment order.
    /// External humidity is copied directly, including signed zero/subfloor W.
    pub fn begin_environment_init(&mut self, out_hum_rat: f64) {
        for i in 0..4 {
            self.ztm[i] = 0.0;
            self.w_prev_zone_ts[i] = out_hum_rat;
            self.dsw_prev_zone_ts[i] = out_hum_rat;
            self.w_prev_zone_ts_temp[i] = 0.0;
        }
        self.w_time_minus_p = out_hum_rat;
        self.w1 = out_hum_rat;
        self.wmx = out_hum_rat;
        self.wm2 = out_hum_rat;
        self.air_hum_rat_temp = 0.0;
        self.temp_ind_load = 0.0;
        self.temp_dep_load = 0.0;
        self.air_rel_hum = 0.0;
        self.air_power_cap = 0.0;
        self.t1 = 0.0;
    }

    /// Prepare actual caller temperatures after reconstruction and before begin.
    /// ZT/other retained defaults and the fourth temperature slots are unchanged.
    pub fn prepare_caller_temperature_inputs(&mut self, input: &ZoneAirCallerTemperatureInputs) {
        self.mat = input.mat;
        self.ztav = input.ztav;
        self.xmat[..3].copy_from_slice(&input.xmat);
        self.dsxmat[..3].copy_from_slice(&input.dsxmat);
    }

    /// Store consumed projections from this actual mutated owner.
    /// Unprojected fourth slots/working fields remain returned owner state only.
    pub fn project_to_legacy_zone(&self, zone: &mut ZoneHeatBalanceState) {
        zone.mean_air_temperature_c = self.mat;
        zone.zone_timestep_average_air_temperature_c = self.ztav;
        zone.air_humidity_ratio = self.air_hum_rat;
        zone.zone_timestep_average_air_humidity_ratio = self.air_hum_rat_avg;
        zone.previous_mean_air_temperatures_c = first_three(self.xmat);
        zone.previous_system_mean_air_temperatures_c = first_three(self.dsxmat);
        zone.previous_air_humidity_ratios = first_three(self.w_prev_zone_ts);
        zone.previous_system_air_humidity_ratios = first_three(self.dsw_prev_zone_ts);
        zone.zone_air_temperature_coefficients
            .third_order_temp_independent_load_w = self.temp_ind_load;
        zone.zone_air_temperature_coefficients
            .third_order_temp_dependent_load_w_per_k = self.temp_dep_load;
        zone.zone_air_temperature_coefficients.air_power_cap_w_per_k = self.air_power_cap;
    }
}

const fn first_three(values: [f64; 4]) -> [f64; 3] {
    [values[0], values[1], values[2]]
}

/// One persistent global environment guard, rather than one flag per zone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneAirEnvironmentGuard {
    /// Source MyEnvrnFlag, initially true and rearmed when BeginEnvrn is false.
    pub my_environment_flag: bool,
}

impl Default for ZoneAirEnvironmentGuard {
    fn default() -> Self {
        Self {
            my_environment_flag: true,
        }
    }
}

/// Actual bounded Rust guard invocation; counts are not native member counters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneAirEnvironmentInvocation {
    /// Actual caller-provided BeginEnvrn input.
    pub begin_environment: bool,
    /// Actual persistent guard before this invocation.
    pub my_environment_before: bool,
    /// Source predicate evaluated against the actual before flags.
    pub eligible_before: bool,
    /// Actual persistent guard after the invocation.
    pub my_environment_after: bool,
    /// Actual Rust selected-owner method calls in this invocation.
    pub initializer_invocations: usize,
}

impl ZoneAirEnvironmentGuard {
    /// Selected zone loop and flag transitions from InitZoneAirSetPoints2622-2669.
    /// Thermostat/day/demand/Space/hybrid siblings are explicitly excluded.
    pub fn apply(
        &mut self,
        begin_environment: bool,
        out_hum_rat: f64,
        owners: &mut [ZoneAirInitializationState],
    ) -> ZoneAirEnvironmentInvocation {
        let my_environment_before = self.my_environment_flag;
        let eligible_before = self.my_environment_flag && begin_environment;
        let mut initializer_invocations = 0;
        if eligible_before {
            for owner in owners {
                owner.begin_environment_init(out_hum_rat);
                initializer_invocations += 1;
            }
            self.my_environment_flag = false;
        }
        if !begin_environment {
            self.my_environment_flag = true;
        }
        ZoneAirEnvironmentInvocation {
            begin_environment,
            my_environment_before,
            eligible_before,
            my_environment_after: self.my_environment_flag,
            initializer_invocations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistent_guard_skips_then_rearms_without_overwriting_retained_inputs() {
        let mut guard = ZoneAirEnvironmentGuard::default();
        let mut owners = [ZoneAirInitializationState {
            mat: 41.0,
            ..Default::default()
        }];
        let first = guard.apply(true, 0.008, &mut owners);
        assert_eq!(first.initializer_invocations, 1);
        owners[0].w_prev_zone_ts = [0.025; 4];
        let before_skip = owners[0];
        let skipped = guard.apply(true, 0.02, &mut owners);
        assert_eq!(skipped.initializer_invocations, 0);
        assert_eq!(owners[0], before_skip);
        let rearmed = guard.apply(false, 0.03, &mut owners);
        assert!(rearmed.my_environment_after);
        assert_eq!(owners[0], before_skip);
        let next = guard.apply(true, 0.012, &mut owners);
        assert_eq!(next.initializer_invocations, 1);
        assert_eq!(owners[0].mat, 41.0);
        assert_eq!(owners[0].w_prev_zone_ts, [0.012; 4]);
    }

    #[test]
    fn custom_temperature_inputs_remain_distinct_from_working_and_fourth_slots() {
        let mut owner = ZoneAirInitializationState::default();
        owner.bulk_reconstruct_and_current_w_seed(0.02);
        owner.prepare_caller_temperature_inputs(&ZoneAirCallerTemperatureInputs {
            mat: 18.5,
            ztav: 19.25,
            xmat: [18.0, 17.0, 16.0],
            dsxmat: [20.0, 21.0, 22.0],
        });
        owner.begin_environment_init(0.02);
        assert_eq!(owner.mat, 18.5);
        assert_eq!(owner.ztav, 19.25);
        assert_eq!(owner.zt, SOURCE_ZONE_INITIAL_TEMP_C);
        assert_eq!(owner.xmat, [18.0, 17.0, 16.0, SOURCE_ZONE_INITIAL_TEMP_C]);
        assert_eq!(owner.dsxmat, [20.0, 21.0, 22.0, SOURCE_ZONE_INITIAL_TEMP_C]);
        assert_eq!(owner.ztm, [0.0; 4]);
    }
}
