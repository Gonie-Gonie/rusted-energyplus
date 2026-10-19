//! Actual monthly/annual/bin gather prefixes on the same retained TAB owner.
//! Positive report records remain explicit missing-operation boundaries.
use super::*;
impl SourceTabularReports {
    // The Original negated comparison deliberately retains its unordered/NaN branch.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    pub(crate) fn gather_source_monthly_results(
        &mut self,
        time_step: TimeStepKind,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        let _time_step_sys_seconds = o.hvac.time_step_sys_seconds;
        if !o.clock.do_weath_sim {
            return Ok(());
        }
        if !(o.clock.time_step_zone_sec > 0.0) {
            return Err(Error::Unavailable(
                "actual monthly TimeStepZoneSec assertion3542",
            ));
        }
        let s = &mut self.input_state;
        if s.gather_monthly_once {
            let columns = s.monthly_columns.as_ref().map_or(0, Vec::len);
            s.monthly_columns_type_of_var = Some(vec![None; columns]);
            s.monthly_columns_step_type = Some(vec![None; columns]);
            s.monthly_columns_aggregation_type = Some(vec![None; columns]);
            s.monthly_columns_variable_number = Some(vec![None; columns]);
            if columns > 0 {
                return Err(Error::Unavailable(
                    "actual MonthlyColumns.typeOfVar3554 before cache cell writes",
                ));
            }
            let tables = s.monthly_tables.as_ref().map_or(0, Vec::len);
            s.monthly_tables_number_columns = Some(vec![None; tables]);
            if tables > 0 {
                return Err(Error::Unavailable(
                    "actual MonthlyTables.numColumns3562 before cache cell write",
                ));
            }
            s.gather_monthly_once = false;
        }
        let _elapsed_time = match time_step {
            TimeStepKind::System => o.hvac.time_step_sys,
            TimeStepKind::Zone => o.clock.time_step_zone,
        };
        let month = o
            .environment
            .month
            .checked_sub(1)
            .and_then(|v| usize::try_from(v).ok())
            .and_then(|i| s.is_month_gathered.get_mut(i))
            .ok_or(Error::Unavailable("actual IsMonthGathered Month3573"))?;
        *month = true;
        if s.monthly_tables_count > 0 {
            return Err(Error::Unavailable(
                "actual MonthlyTables.firstColumn3577 before curFirstColumn write",
            ));
        }
        Ok(())
    }

    pub(crate) fn gather_source_annual_results(&self) -> Result<(), Error> {
        if !self.input_state.annual_tables.is_empty() {
            return Err(Error::Unavailable(
                "actual AnnualTable.gatherForTimestep327 before elapsed-time reads",
            ));
        }
        Ok(())
    }

    pub(crate) fn gather_source_bin_results(
        &mut self,
        time_step: TimeStepKind,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !o.clock.do_weath_sim {
            return Ok(());
        }
        let _elapsed_time = match time_step {
            TimeStepKind::System => o.hvac.time_step_sys,
            TimeStepKind::Zone => o.clock.time_step_zone,
        };
        // Native3453 uses the actual System timestep for BOTH call types.
        self.input_state.time_in_year += o.hvac.time_step_sys;
        if self.input_state.output_table_binned_count > 0 {
            return Err(Error::Unavailable(
                "actual OutputTableBinned.sched3458 before schedule read",
            ));
        }
        Ok(())
    }
}
