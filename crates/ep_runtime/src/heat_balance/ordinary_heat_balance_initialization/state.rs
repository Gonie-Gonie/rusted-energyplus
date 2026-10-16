use super::*;
/// Source Array2D shape is retained even when the second extent is zero.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceReportMatrix<T> {
    pub(crate) rows: usize,
    pub(crate) columns: usize,
    pub(crate) cells: Vec<T>,
}
impl<T: Clone + Default> SourceReportMatrix<T> {
    pub(super) fn allocate(rows: usize, columns: usize) -> Result<Self, Error> {
        let size = rows
            .checked_mul(columns)
            .ok_or(Error::Unavailable("actual resilience array extent"))?;
        Ok(Self {
            rows,
            columns,
            cells: vec![T::default(); size],
        })
    }
}
/// The existing selected control owner retains the only actual contaminant flags.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceContaminantArrays {
    pub(crate) outdoor_co2: f64,
    pub(crate) outdoor_gc: f64,
    pub(crate) zone_air_co2: Option<Vec<f64>>,
    pub(crate) zone_air_co2_temp: Option<Vec<f64>>,
    pub(crate) zone_air_co2_avg: Option<Vec<f64>>,
    pub(crate) zone_air_gc: Option<Vec<f64>>,
    pub(crate) zone_air_gc_temp: Option<Vec<f64>>,
    pub(crate) zone_air_gc_avg: Option<Vec<f64>>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceWarmupConvergence {
    pub(crate) pass_flags: [i32; 4],
    pub(crate) test_values: [f64; 4],
}
impl Default for SourceWarmupConvergence {
    fn default() -> Self {
        Self {
            pass_flags: [2; 4],
            test_values: [0.0; 4],
        }
    }
}
/// Genuine disjoint HeatBalanceMgr arrays. The report fields remain solely in
/// the existing WarmupReportState; bundled legacy current/rolling rows are not
/// allocated or used by this route. No mutable copyback adapter is provided.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceHeatBalanceManager {
    /// Actual HeatBalanceMgrData.hh187 ctortrue; only genuine first-call suffix clears it.
    pub(crate) manage_heat_balance_get_input_flag: bool,
    pub(crate) change_set: bool,
    pub(crate) report: WarmupReportState,
    pub(crate) convergence: Option<Vec<SourceWarmupConvergence>>,
    pub(crate) contaminants: SourceContaminantArrays,
    pub(crate) max_temp_prev_day: Option<Vec<f64>>,
    pub(crate) min_temp_prev_day: Option<Vec<f64>>,
    pub(crate) max_heat_load_prev_day: Option<Vec<f64>>,
    pub(crate) max_cool_load_prev_day: Option<Vec<f64>>,
    pub(crate) max_heat_load_zone: Option<Vec<f64>>,
    pub(crate) max_cool_load_zone: Option<Vec<f64>>,
    pub(crate) max_temp_zone: Option<Vec<f64>>,
    pub(crate) min_temp_zone: Option<Vec<f64>>,
    pub(crate) temp_zone_prev_day: Option<Vec<f64>>,
    pub(crate) load_zone_prev_day: Option<Vec<f64>>,
    pub(crate) temp_zone_sec_prev_day: Option<Vec<f64>>,
    pub(crate) load_zone_sec_prev_day: Option<Vec<f64>>,
    pub(crate) temp_zone: Option<Vec<f64>>,
    pub(crate) load_zone: Option<Vec<f64>>,
    pub(crate) crossed_cold_thresh_rep_period: Option<SourceReportMatrix<Option<bool>>>,
    pub(crate) crossed_heat_thresh_rep_period: Option<SourceReportMatrix<Option<bool>>>,
    pub(crate) zone_heat_index_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_heat_index_occupied_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_heat_index_occu_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_humidex_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_humidex_occupied_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_humidex_occu_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_cold_hour_of_safety_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_heat_hour_of_safety_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_unmet_degree_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_discomfort_wt_exceed_occu_hour_bins_rep_period:
        Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_discomfort_wt_exceed_occupied_hour_bins_rep_period:
        Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_c_o2_level_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_c_o2_level_occu_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_c_o2_level_occupied_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_lighting_level_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_lighting_level_occu_hour_bins_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_lighting_level_occupied_hour_bins_rep_period:
        Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_low_s_e_t_hours_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) zone_high_s_e_t_hours_rep_period: Option<SourceReportMatrix<Vec<f64>>>,
    pub(crate) low_s_e_t_longest_hours_rep_period: Option<SourceReportMatrix<Option<f64>>>,
    pub(crate) high_s_e_t_longest_hours_rep_period: Option<SourceReportMatrix<Option<f64>>>,
    pub(crate) low_s_e_t_longest_start_rep_period: Option<SourceReportMatrix<Option<i32>>>,
    pub(crate) high_s_e_t_longest_start_rep_period: Option<SourceReportMatrix<Option<i32>>>,
}
impl SourceHeatBalanceManager {
    pub(crate) fn new(report: WarmupReportState) -> Result<Self, Error> {
        if report.source_allocations != Some([false; 7]) {
            return Err(Error::Unavailable(
                "Source report constructor must be unallocated; no legacy array copy",
            ));
        }
        Ok(Self {
            manage_heat_balance_get_input_flag: true,
            change_set: true,
            report,
            convergence: None,
            contaminants: SourceContaminantArrays::default(),
            max_temp_prev_day: None,
            min_temp_prev_day: None,
            max_heat_load_prev_day: None,
            max_cool_load_prev_day: None,
            max_heat_load_zone: None,
            max_cool_load_zone: None,
            max_temp_zone: None,
            min_temp_zone: None,
            temp_zone_prev_day: None,
            load_zone_prev_day: None,
            temp_zone_sec_prev_day: None,
            load_zone_sec_prev_day: None,
            temp_zone: None,
            load_zone: None,
            crossed_cold_thresh_rep_period: None,
            crossed_heat_thresh_rep_period: None,
            zone_heat_index_hour_bins_rep_period: None,
            zone_heat_index_occupied_hour_bins_rep_period: None,
            zone_heat_index_occu_hour_bins_rep_period: None,
            zone_humidex_hour_bins_rep_period: None,
            zone_humidex_occupied_hour_bins_rep_period: None,
            zone_humidex_occu_hour_bins_rep_period: None,
            zone_cold_hour_of_safety_bins_rep_period: None,
            zone_heat_hour_of_safety_bins_rep_period: None,
            zone_unmet_degree_hour_bins_rep_period: None,
            zone_discomfort_wt_exceed_occu_hour_bins_rep_period: None,
            zone_discomfort_wt_exceed_occupied_hour_bins_rep_period: None,
            zone_c_o2_level_hour_bins_rep_period: None,
            zone_c_o2_level_occu_hour_bins_rep_period: None,
            zone_c_o2_level_occupied_hour_bins_rep_period: None,
            zone_lighting_level_hour_bins_rep_period: None,
            zone_lighting_level_occu_hour_bins_rep_period: None,
            zone_lighting_level_occupied_hour_bins_rep_period: None,
            zone_low_s_e_t_hours_rep_period: None,
            zone_high_s_e_t_hours_rep_period: None,
            low_s_e_t_longest_hours_rep_period: None,
            high_s_e_t_longest_hours_rep_period: None,
            low_s_e_t_longest_start_rep_period: None,
            high_s_e_t_longest_start_rep_period: None,
        })
    }
    pub(super) fn begin_environment(&mut self) -> Result<(), Error> {
        fill(&mut self.max_heat_load_prev_day, 0.0)?;
        fill(&mut self.max_cool_load_prev_day, 0.0)?;
        fill(&mut self.max_temp_prev_day, 0.0)?;
        fill(&mut self.min_temp_prev_day, 0.0)?;
        fill(&mut self.max_heat_load_zone, -9999.0)?;
        fill(&mut self.max_cool_load_zone, -9999.0)?;
        fill(&mut self.max_temp_zone, -9999.0)?;
        fill(&mut self.min_temp_zone, 1000.0)?;
        fill(&mut self.temp_zone, -9999.0)?;
        fill(&mut self.load_zone, -9999.0)?;
        fill(&mut self.temp_zone_prev_day, 1000.0)?;
        fill(&mut self.load_zone_prev_day, -9999.0)?;
        fill(&mut self.temp_zone_sec_prev_day, -9999.0)?;
        self.report.temperature_difference.fill(0.0);
        self.report.load_difference.fill(0.0);
        for row in &mut self.report.temperature {
            row.fill(0.0);
        }
        for row in &mut self.report.load {
            row.fill(0.0);
        }
        for row in &mut self.report.maximum_load {
            row.fill(0.0);
        }
        self.report.count = 0;
        Ok(())
    }
}
