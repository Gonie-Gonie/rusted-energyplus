//! Genuine local END open/write and CloseMiscOpenFiles retained handles.
use super::{SourceInitializationFiles, native_text_bytes};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_output_control::SourceOutputControl;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use std::io::{Seek, Write};
impl SourceInitializationFiles {
    /// Persist the genuine backend's logical stream at its own physical ERR host.
    /// Structured CLI diagnostics have a separate artifact and are not input here.
    /// # Errors
    /// Propagates the actual filesystem write failure.
    pub fn persist_source_error(&self, logical: &[u8]) -> std::io::Result<()> {
        std::fs::write(&self.error, native_text_bytes(logical).as_ref())
    }
    pub(crate) fn close_source_misc_files(&mut self, debug_output: bool) -> Result<(), Error> {
        // CloseDFSFile always resets the same optional os; a missing handle stays missing.
        self.dfs.close_source_output();
        let keep = if debug_output {
            true
        } else if self.debug.file.is_some() && !self.debug.failed {
            self.debug
                .file
                .as_mut()
                .ok_or(Error::Unavailable("actual same debug file"))?
                .stream_position()?
                > 0
        } else {
            false
        };
        if keep {
            self.debug.close_source_output();
        } else {
            self.debug.delete_source_output()?;
        }
        Ok(())
    }
    pub(crate) fn write_source_program_end(
        &self,
        control: &SourceOutputControl,
        row: &str,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        if !control.end {
            return Err(Error::Unavailable(
                "actual END try_open disabled null-buffer stream574",
            ));
        }
        let opened = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.end);
        let mut file = match opened {
            Ok(v) => v,
            Err(e) => {
                backend.display_source_string(
                    &format!(
                        "EndEnergyPlus: Could not open file {} for output (write).",
                        self.end.display()
                    ),
                    flags,
                )?;
                return Err(e.into());
            }
        };
        let logical = format!("{row}\n");
        let bytes = native_text_bytes(logical.as_bytes());
        file.write_all(bytes.as_ref())?;
        file.flush()?;
        Ok(())
    }
    pub(crate) fn close_source_program_audit(&mut self) {
        self.audit.close_source_output();
    }
}
