//! Actual output options/nullable SQL prerequisite; no alternative report owner.
use super::{CtfInitializationReports, SourceInitializationError, SourceInitializationFiles};
use crate::heat_balance::ordinary_output_control::SourceOutputControl;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{SelectedInputProcessor, SourceInputFamily};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourceSqliteState {
    /// SQLiteProceduresData actual std::unique_ptr constructor-null.
    database: Option<std::convert::Infallible>,
}
impl SourceSqliteState {
    pub(crate) fn present(&self) -> bool {
        self.database.is_some()
    }
    fn create_if_absent(
        &mut self,
        files: &mut SourceOutputControl,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), Error> {
        if self.database.is_some() {
            return Ok(());
        }
        if !files.sqlite {
            return Ok(());
        }
        // Native ParseSQLiteInput uses epJSON.find, not a count/getter query.
        let first = match input.find_exact_family("Output:SQLite") {
            None => None,
            Some(SourceInputFamily::Objects(rows)) => Some(
                rows.first()
                    .ok_or(Error::Unavailable(
                        "actual SQLite JSON first-object iterator domain",
                    ))?
                    .object_name
                    .clone(),
            ),
            Some(SourceInputFamily::Null) => {
                return Err(Error::Unavailable(
                    "actual SQLite null-object iterator domain",
                ));
            }
        };
        if let Some(name) = first {
            input.mark_object_as_used("Output:SQLite", &name);
            return Err(Error::Unavailable(
                "actual SQLite option_type find_input/default before database construction",
            ));
        }
        files.sqlite = false;
        self.database = None; // actual caller assignment of returned nullptr
        Ok(())
    }
}
impl CtfInitializationReports {
    /// Real ManageSimulation183?191 prefix. The parent must call this after IP
    /// admission, before actual PostIPProcessing/BeginSim writes. Reentry is not
    /// hidden behind a fabricated once flag; no whole ManageSimulation claim.
    ///
    /// # Errors
    /// Preserves actual IP/options/file prefixes and typed errors; positive SQL
    /// construction and debug null-stream/fatal formatting stay unavailable.
    pub fn prepare_source_reporting_prefix(
        &mut self,
        files: &mut SourceInitializationFiles,
    ) -> Result<(), SourceInitializationError> {
        let input = self.selected_input_processor.as_mut().ok_or(
            SourceInitializationError::Association("actual output-prefix IP"),
        )?;
        let apply = |source: Error| SourceInitializationError::Handler {
            handler: "ManageSimulation output prefix",
            source: Box::new(source),
        };
        self.source_output_control
            .get_source_output_control_input(input)
            .map_err(apply)?;
        self.source_results_framework
            .setup_source_output_options(&self.source_output_control, input)
            .map_err(apply)?;
        if !self.source_output_control.dbg {
            return Err(SourceInitializationError::Association(
                "actual debug null-stream open unavailable",
            ));
        }
        // A real open failure retains its actual OS cause. Native fmt::filesystem
        // fatal message is an explicit unpaired failure leaf, never synthesized.
        files
            .ensure_debug_open()
            .map_err(|source| SourceInitializationError::Handler {
                handler: "actual debug open; Native fatal filename formatter unavailable",
                source: Box::new(source),
            })?;
        self.source_sqlite
            .create_if_absent(&mut self.source_output_control, input)
            .map_err(apply)?;
        Ok(())
    }
}

impl SourceSqliteState {
    pub(crate) fn reset_source_pointer(&mut self) {
        self.database = None;
    }
}
