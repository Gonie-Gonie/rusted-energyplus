//! Actual selected Setup292/300-311 effects on the same Surface/trig owners.
use super::{
    Error, SelectedInputProcessor, SourceInputProcessorShortCut, SurfaceClass,
    SurfaceGeometryInputState, signed,
};

impl SurfaceGeometryInputState {
    pub(crate) fn release_setup_coordinates_if_errors(&mut self, errors: bool) -> bool {
        if !errors {
            return false;
        }
        self.coordinate.cos_zone_relative = None; // Actual293.
        self.coordinate.sin_zone_relative = None; // Actual294.
        true // Native295 return only; no parent HB readiness flag.
    }

    pub(crate) fn finish_zero_window_setup(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        errors: bool,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "WindowProperty:AirflowControl".into();
        let total_airflow_controls =
            signed(input.get_num_objects_found(&shortcut.current_module_object))?;
        if total_airflow_controls != 0 {
            return Err(Error::Unavailable("positive window airflow control body"));
        } // Actual10560-10562 zero early return, with no AirflowWindows/array writes.
        shortcut.current_module_object = "WindowProperty:StormWindow".into();
        self.total_storm_windows =
            signed(input.get_num_objects_found(&shortcut.current_module_object))?;
        if self.total_storm_windows != 0 {
            return Err(Error::Unavailable("positive storm window allocation/items"));
        } // Actual10352 zero early return; same StormWindow remains unallocated.
        if !errors && self.total_storm_windows > 0 {
            return Err(Error::Unavailable(
                "positive CreateStormWindowConstructions",
            ));
        }
        for surface in self.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "same final Surface shade-flag traversal",
        ))? {
            if surface.row.class != SurfaceClass::Window {
                continue;
            }
            return Err(Error::Unavailable(
                "positive Window shade/blind Construction traversal",
            ));
        }
        self.coordinate.cos_zone_relative = None; // Actual310.
        self.coordinate.sin_zone_relative = None; // Actual311.
        Ok(())
    }
}
