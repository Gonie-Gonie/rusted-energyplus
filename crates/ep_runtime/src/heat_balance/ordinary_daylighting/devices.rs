//! Genuine earlier HB InitDaylightingDevices zero-TDD/shelf input children.
use super::*;
impl SourceDaylighting {
    /// DaylightingDevices173-429. Source has no one-time flag here; the real
    /// HB BeginSim caller controls this call. Do not invent a successful latch.
    pub(crate) fn initialize_daylighting_devices(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        modules: &mut SourceGainModules,
        control: &SelectedHeatBalanceInputState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let _stored: Option<Vec<Infallible>> = None; // actual unallocated local TDDPipeStored
        shortcut.current_module_object = "DaylightingDevice:Tubular".into();
        let pipes = input.get_num_objects_found(&shortcut.current_module_object);
        if pipes > 0 {
            return Err(Error::Unavailable(
                "GetTDDInput actual positive TDDPipe constructor allocation451",
            ));
        }
        // Real count0 does NOT allocate(0) or clear the existing device array.
        if !empty(&modules.tdd_pipe) {
            backend.display_string("Initializing Tubular Daylighting Devices")?;
            return Err(Error::Unavailable(
                "InitDaylightingDevices positive COSAngle setup209",
            ));
        }
        shortcut.current_module_object = "DaylightingDevice:Shelf".into();
        let shelves = input.get_num_objects_found(&shortcut.current_module_object);
        if shelves > 0 {
            return Err(Error::Unavailable(
                "GetShelfInput actual positive Shelf constructor allocation811",
            ));
        }
        if !empty(&modules.shelf) {
            backend.display_string("Initializing Light Shelf Daylighting Devices")?;
        }
        if !empty(&modules.shelf) {
            return Err(Error::Unavailable(
                "InitDaylightingDevices actual positive Shelf.Window370",
            ));
        }
        if control.calculate_solar_reflections
            && (!empty(&modules.tdd_pipe) || !empty(&modules.shelf))
        {
            backend.show_warning_error("InitDaylightingDevices: Solar Distribution Model includes Solar Reflection calculations;")?;
            backend.show_continue_error(
                "the resulting reflected solar values will not be used in the",
            )?;
            backend.show_continue_error(
                "DaylightingDevice:Shelf or DaylightingDevice:Tubular calculations.",
            )?;
        }
        Ok(())
    }
}
