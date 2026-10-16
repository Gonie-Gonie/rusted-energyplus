//! Actual Monthly/Annual/TimeBins input order and zero-object storage effects.
use super::*;
use std::convert::Infallible;
fn extent(value: i32) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| Error::Unavailable("actual TAB array extent"))
}
impl SourceTabularReports {
    pub(super) fn get_source_tabular_monthly(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        const FAMILY: &str = "Output:Table:Monthly";
        if !o.files.source_write_tabular(o.results) {
            self.input_state.write_tabular_files = false;
            return Ok(());
        }
        self.input_state.monthly_input_count = i32::try_from(o.input.get_num_objects_found(FAMILY))
            .map_err(|_| Error::Unavailable("actual MonthlyInputCount353"))?;
        if self.input_state.monthly_input_count > 0 {
            self.input_state.write_tabular_files = true;
            if !o.clock.do_weath_sim {
                o.backend.show_warning_error(&format!("{FAMILY} requested with SimulationControl Run Simulation for Weather File Run Periods set to No so {FAMILY} will not be generated"))?;
                return Ok(());
            }
        }
        let counts = o
            .input
            .get_object_def_max_args(FAMILY)
            .map_err(|_| Error::Unavailable("actual Monthly definition375"))?;
        let _alphas = vec![String::new(); counts.alphas];
        let _numbers = vec![0.0; counts.numerics];
        if self.input_state.monthly_input_count > 0 {
            return Err(Error::Unavailable(
                "actual Monthly379 positive getObjectItem/AddMonthlyReport",
            ));
        }
        Ok(())
    }
    pub(super) fn get_source_tabular_annual(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        const FAMILY: &str = "Output:Table:Annual";
        let objects = o.input.get_num_objects_found(FAMILY);
        if objects > 0 {
            self.input_state.write_tabular_files = true;
            if !o.clock.do_weath_sim {
                o.backend.show_warning_error(&format!("{FAMILY} requested with SimulationControl Run Simulation for Weather File Run Periods set to No so {FAMILY} will not be generated"))?;
                return Ok(());
            }
        }
        let counts = o
            .input
            .get_object_def_max_args(FAMILY)
            .map_err(|_| Error::Unavailable("actual Annual definition119"))?;
        let _alphas = vec![String::new(); counts.alphas];
        let _numbers = vec![0.0; counts.numerics];
        if objects > 0 {
            return Err(Error::Unavailable(
                "actual Annual123 positive getObjectItem/AnnualTable constructor",
            ));
        }
        Ok(())
    }
    pub(super) fn check_source_annual_aggregation(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !o.clock.do_weath_sim {
            return Ok(());
        }
        if !self.input_state.annual_tables.is_empty() {
            return Err(Error::Unavailable(
                "actual Annual257 invalidAggregationOrder child",
            ));
        }
        Ok(())
    }
    pub(super) fn get_source_tabular_time_bins(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        const FAMILY: &str = "Output:Table:TimeBins";
        if !o.files.source_write_tabular(o.results) {
            self.input_state.write_tabular_files = false;
            return Ok(());
        }
        let counts = o
            .input
            .get_object_def_max_args(FAMILY)
            .map_err(|_| Error::Unavailable("actual TimeBins definition1069"))?;
        let _alphas = vec![String::new(); counts.alphas];
        let _numbers = vec![0.0; counts.numerics];
        self.input_state.time_in_year = 0.0;
        self.input_state.output_table_binned_count =
            i32::try_from(o.input.get_num_objects_found(FAMILY))
                .map_err(|_| Error::Unavailable("actual OutputTableBinnedCount1075"))?;
        if self.input_state.output_table_binned.is_some() {
            return Err(Error::Unavailable(
                "native repeated OutputTableBinned allocate1076",
            ));
        }
        self.input_state.output_table_binned =
            Some(vec![
                state::SourceOutputTableBinned::default();
                extent(self.input_state.output_table_binned_count)?
            ]);
        if self.input_state.output_table_binned_count > 0 {
            self.input_state.write_tabular_files = true;
            if !o.clock.do_weath_sim {
                o.backend.show_warning_error(&format!("{FAMILY} requested with SimulationControl Run Simulation for Weather File Run Periods set to No so {FAMILY} will not be generated"))?;
                return Ok(());
            }
        }
        self.input_state.bin_results_interval_count = 0;
        self.input_state.bin_results_table_count = 0;
        if self.input_state.output_table_binned_count > 0 {
            return Err(Error::Unavailable(
                "actual TimeBins1094 positive getObjectItem/variable-key lookup",
            ));
        }
        self.input_state.bin_results.dimension(
            [
                extent(self.input_state.bin_results_interval_count)?,
                extent(self.input_state.bin_results_table_count)?,
            ],
            Some(state::SourceBinResults::default()),
        )?;
        self.input_state.bin_results_below.dimension(
            [extent(self.input_state.bin_results_table_count)?],
            Some(state::SourceBinResults::default()),
        )?;
        self.input_state.bin_results_above.dimension(
            [extent(self.input_state.bin_results_table_count)?],
            Some(state::SourceBinResults::default()),
        )?;
        self.input_state.bin_statistics.dimension(
            [extent(self.input_state.bin_results_table_count)?],
            Some(state::SourceBinStatistics::default()),
        )?;
        self.input_state.bin_obj_var_id = Some(Vec::<Infallible>::new());
        for value in self.input_state.bin_results.values.iter_mut().flatten() {
            value.monthly.fill(0.0);
            value.hourly.fill(0.0);
        }
        for value in self
            .input_state
            .bin_results_below
            .values
            .iter_mut()
            .flatten()
        {
            value.monthly.fill(0.0);
            value.hourly.fill(0.0);
        }
        for value in self
            .input_state
            .bin_results_above
            .values
            .iter_mut()
            .flatten()
        {
            value.monthly.fill(0.0);
            value.hourly.fill(0.0);
        }
        for value in self.input_state.bin_statistics.values.iter_mut().flatten() {
            value.minimum = f64::MAX;
            value.maximum = -f64::MAX;
            value.count = 0;
            value.sum = 0.0;
            value.sum2 = 0.0;
        }
        Ok(())
    }
}
