//! Actual EMSManager1983-2006 endrun traversal, without an AnyEMS proxy guard.
use super::{Error, SelectedRuntimeLanguage};

impl SelectedRuntimeLanguage {
    pub(crate) fn check_source_unused_actuators_at_end(&self) -> Result<(), Error> {
        if self.number_of_actuators_used > 0 {
            return Err(Error::Unavailable(
                "actual checkForUnusedActuatorsAtEnd EMSActuatorUsed/ErlVariable read1988",
            ));
        }
        Ok(())
    }
}
