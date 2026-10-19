//! Same IOFiles audit ensure_open for actual TAB5278 and CloseOutputFiles1797.
use super::SourceInitializationFiles;
use crate::heat_balance::ordinary_output_control::SourceOutputControl;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use std::io::Write;

impl SourceInitializationFiles {
    pub(crate) fn ensure_source_reporting_audit(
        &mut self,
        control: &SourceOutputControl,
    ) -> Result<&mut dyn Write, Error> {
        // Native ensure_open first tests this same pointer and good state. An
        // already opened good stream is retained, irrespective of the option.
        if self.audit.file.is_none() || self.audit.failed {
            if !control.audit {
                return Err(Error::Unavailable(
                    "actual audit ensure_open disabled null-buffer stream allocation",
                ));
            }
            self.audit.ensure_open()?;
        }
        Ok(&mut self.audit)
    }
}
