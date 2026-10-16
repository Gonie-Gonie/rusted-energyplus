//! Sole DataHeatBalance::AllocateIntGains1042-1048, shared by both real callers.
use super::{E, SelectedInternalHeatGains, unavailable};
impl SelectedInternalHeatGains {
    pub(crate) fn allocate_source_internal_gains_if_unallocated(
        &mut self,
        zones: usize,
        spaces: usize,
    ) {
        if self.allocations.zone_int_gain.is_none() {
            self.allocations.zone_int_gain = Some(vec![[0.0; 2]; zones]);
            self.allocations.space_int_gain = Some(vec![[0.0; 2]; spaces]);
            self.allocations.space_devices = Some(vec![Default::default(); spaces]);
            self.allocations.space_power_reduction_factor = Some(vec![1.0; spaces]);
        }
    }
    /// Rust association/constructor readback immediately after real equipment input.
    /// This is not a new Source operation or a later timestep readiness flag.
    pub(crate) fn validate_source_initial_gain_allocations(
        &self,
        zones: usize,
        spaces: usize,
    ) -> Result<(), E> {
        for (values, extent) in [
            (&self.allocations.zone_int_gain, zones),
            (&self.allocations.space_int_gain, spaces),
        ] {
            if values
                .as_ref()
                .is_none_or(|v| v.len() != extent || v.iter().any(|row| *row != [0.0; 2]))
            {
                return Err(unavailable(
                    "same actual initial Zone/SpaceIntGain allocation",
                ));
            }
        }
        let devices = self
            .allocations
            .space_devices
            .as_ref()
            .ok_or_else(|| unavailable("same initial actual SpaceIntGainDevices"))?;
        if devices.len() != spaces
            || devices.iter().any(|d| {
                d.number_of_devices != 0 || d.max_number_of_devices != 0 || d.devices.is_some()
            })
        {
            return Err(unavailable("same initial actual gain device constructors"));
        }
        if self
            .allocations
            .space_power_reduction_factor
            .as_ref()
            .is_none_or(|v| v.len() != spaces || v.iter().any(|value| *value != 1.0))
        {
            return Err(unavailable("same initial actual spacePowerReductionFactor"));
        }
        Ok(())
    }
}
