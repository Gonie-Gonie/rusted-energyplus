//! Sole reached OutputProcessor time bindings, before any variable registration.
//! Future requested-variable/dictionary storage extends this same owner.
use super::error_backend::SourceErrorBackend;
use super::selected_simulation_startup::{SelectedHvacGlobals, StartupError};
use crate::weather::day::WeatherGlobalState;
fn fatal(backend: &mut SourceErrorBackend<'_>, message: &str) -> Result<(), StartupError> {
    match backend.show_fatal_error(message) {
        Ok(never) => match never {},
        Err(error) => Err(StartupError::Fatal(error)),
    }
}
/// Original OutputProcessor Zone/System timestep discriminator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeStepKind {
    /// The actual DataGlobal TimeStepZone field.
    Zone,
    /// The actual DataHVACGlobals TimeStepSys field.
    System,
}

/// A concrete field identity resolved against the sole live owners on every read.
/// This represents the pointer target without unsafe addresses or cached values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimeStepTarget {
    GlobalZoneTimeStep,
    HvacSystemTimeStep,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedOutputTimeValue {
    pub(crate) target: Option<TimeStepTarget>,
    pub(crate) current_minute: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedOutputTimePointers {
    pub(crate) zone: SelectedOutputTimeValue,
    pub(crate) system: SelectedOutputTimeValue,
}

/// Root's operations boundary must borrow its same retained session/context pair.
/// The reference lifetime prevents retaining a scalar copy across later updates.
pub(crate) struct LiveTimeStepOwners<'a> {
    pub(crate) global: &'a WeatherGlobalState,
    pub(crate) hvac: &'a SelectedHvacGlobals,
}
impl SelectedOutputTimePointers {
    pub(crate) fn setup_time_pointer(
        &mut self,
        kind: TimeStepKind,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), StartupError> {
        let (slot, target, name) = match kind {
            TimeStepKind::Zone => (&mut self.zone, TimeStepTarget::GlobalZoneTimeStep, "Zone"),
            TimeStepKind::System => (
                &mut self.system,
                TimeStepTarget::HvacSystemTimeStep,
                "System",
            ),
        };
        if slot.target.is_some() {
            return fatal(
                backend,
                &format!("SetupTimePointers was already called for {name}"),
            );
        }
        slot.target = Some(target);
        Ok(())
    }

    pub(crate) fn time_step<'a>(
        &self,
        kind: TimeStepKind,
        owners: &LiveTimeStepOwners<'a>,
    ) -> Result<&'a f64, StartupError> {
        let target = match kind {
            TimeStepKind::Zone => self.zone.target,
            TimeStepKind::System => self.system.target,
        };
        match target {
            Some(TimeStepTarget::GlobalZoneTimeStep) if kind == TimeStepKind::Zone => {
                Ok(&owners.global.time_step_zone)
            }
            Some(TimeStepTarget::HvacSystemTimeStep) if kind == TimeStepKind::System => {
                Ok(&owners.hvac.time_step_sys)
            }
            _ => Err(StartupError::Unavailable(
                "OutputProcessor timestep target association",
            )),
        }
    }
}

/// Sole selected OutputProcessor storage; expanded by real later registrations.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceOutputRegistration {
    pub(crate) time_pointers: SelectedOutputTimePointers,
}
