//! Actual InitializeTabularMonthly582–955, genuine empty retained input route.
use super::*;
impl SourceTabularReports {
    pub(super) fn initialize_source_tabular_monthly(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !o.clock.do_weath_sim {
            return Ok(());
        }
        self.input_state.max_unique_key_count = 1500;
        let count = usize::try_from(self.input_state.max_unique_key_count)
            .map_err(|_| Error::Unavailable("actual monthly UniqueKeyNames allocate count"))?;
        // Array1D_string.allocate constructs 1500 empty strings at actual587;
        // this local remains live until the routine's own scope ends955.
        let unique_key_names = vec![String::new(); count];
        self.input_state.monthly_columns_count = 0;
        self.input_state.monthly_tables_count = 0;
        // Read the retained count after genuine AddMonthlyReports, not raw IP count.
        if self.input_state.monthly_input_count > 0 {
            return Err(Error::Unavailable(
                "actual InitializeTabularMonthly608 MonthlyInput(Tab).numFieldSet first read",
            ));
        }
        if self.input_state.monthly_tables.is_some() {
            return Err(Error::Unavailable(
                "actual MonthlyTables allocate on already allocated owner708",
            ));
        }
        let tables = usize::try_from(self.input_state.monthly_tables_count)
            .map_err(|_| Error::Unavailable("actual monthly table allocation count"))?;
        if tables > 0 {
            return Err(Error::Unavailable(
                "actual MonthlyTables positive record constructor708",
            ));
        }
        self.input_state.monthly_tables = Some(Vec::new());
        if self.input_state.monthly_columns.is_some() {
            return Err(Error::Unavailable(
                "actual MonthlyColumns allocate on already allocated owner709",
            ));
        }
        let columns = usize::try_from(self.input_state.monthly_columns_count)
            .map_err(|_| Error::Unavailable("actual monthly column allocation count"))?;
        if columns > 0 {
            return Err(Error::Unavailable(
                "actual MonthlyColumns positive record constructor709",
            ));
        }
        self.input_state.monthly_columns = Some(Vec::new());
        // Native table/column initialization traverses these actual empty arrays;
        // no element constructor or reset writes occur in this reached route.
        let mut _issue_warnings = false;
        if o.clock.do_weath_sim && self.input_state.monthly_invalid_variable_count > 0 {
            o.backend.show_warning_error(
                "Processing Monthly Tabular Reports: Variable names not valid for this simulation",
            )?;
            if !o.globals.display_extra_warnings {
                o.backend.show_continue_error("...use Output:Diagnostics,DisplayExtraWarnings; to show more details on individual variables.")?;
            } else {
                o.backend.show_continue_error("..Variables not valid for this simulation will have \"[Invalid/Undefined]\" in the Units Column of the Table Report.")?;
                _issue_warnings = true;
            }
        }
        let _columns_recount = 0_i32;
        let _tables_recount = 0_i32;
        // The second actual MonthlyInput traversal also has zero retained rows.
        // Native local string-array destruction follows that loop at scope end.
        drop(unique_key_names);
        Ok(())
    }
}
