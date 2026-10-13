//! Actual zero-object getter sequence at GetSurfaceData1169-1213.
//! Called after the real GetHTSurfaceData row/definition loop, before1215.
use super::{
    Error, FAMILIES, SelectedInputProcessor, SourceErrorBackend, SourceInputProcessorShortCut,
    SurfaceGeometryInputState, signed,
};

fn require_empty(count: i32, family: &'static str) -> Result<(), Error> {
    if count != 0 {
        return Err(Error::Unavailable(family));
    }
    Ok(())
}

impl SurfaceGeometryInputState {
    // Actual GetHTSubSurfaceData prefix5034-5053; the real item loop is empty.
    fn get_empty_ht_subsurface_data(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        // GetHTSubSurfaceData1185 first reaches WindowShadingControl9707-9711.
        shortcut.current_module_object = "WindowShadingControl".into();
        self.total_window_shading_control =
            signed(input.get_num_objects_found(&shortcut.current_module_object))?;
        require_empty(
            self.total_window_shading_control,
            "positive WindowShadingControl getter",
        )?;
        // The genuine zero branch returns before WindowShadingControl.allocate.
        shortcut.current_module_object = "FenestrationSurface:Detailed".into();
        let counts = input
            .get_object_def_max_args(&shortcut.current_module_object)
            .map_err(|_| Error::Unavailable("actual FenestrationSurface definition"))?;
        if counts.alphas != 6 {
            backend.show_severe_error(&format!(
                "{}: Object Definition indicates not = 6 Alpha Objects, Number Indicated={}",
                shortcut.current_module_object, counts.alphas,
            ))?;
            *errors = true;
        }
        if counts.numerics != 15 {
            // Original5046 tests !=15, says >15, and prints NumAlpha literally.
            backend.show_severe_error(&format!(
                "{}: Object Definition indicates > 15 Numeric Objects, Number Indicated={}",
                shortcut.current_module_object, counts.alphas,
            ))?;
            *errors = true;
        }
        self.need_to_add_subsurfaces = Some(0); // actual5053 out-argument write
        require_empty(
            self.source_surface_counts[8],
            "positive FenestrationSurface getter",
        )?;
        Ok(())
    }

    // Actual GetAttShdSurfaceData6373-6385, with its item loop empty.
    fn get_empty_attached_shading_data(
        &self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "Shading:Zone:Detailed".into();
        let counts = input
            .get_object_def_max_args(&shortcut.current_module_object)
            .map_err(|_| Error::Unavailable("actual attached shading definition"))?;
        if counts.alphas != 3 {
            backend.show_severe_error(&format!(
                "{}: Object Definition indicates not = 3 Alpha Objects, Number Indicated={}",
                shortcut.current_module_object, counts.alphas,
            ))?;
            *errors = true;
        }
        require_empty(
            self.source_surface_counts[9],
            "positive attached shading getter",
        )?;
        Ok(())
    }

    /// Preserve Source scratch/query/error effects, without completing the
    /// subsequent GetSurfaceData geometry/enclosure/report lifecycle.
    pub(crate) fn finish_opaque_input_getters(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        if self.pending_vertices.is_some()
            || signed(self.next_object)? != self.source_surface_counts[4]
        {
            return Err(Error::Unavailable(
                "complete actual GetHTSurfaceData item loop",
            ));
        }
        // GetRectSurfaces1169 /4430-4789: all ten outer assignments run even
        // when each passed ItemsToGet is0. No fresh object count/schema query.
        for (index, &(family, _)) in FAMILIES.iter().enumerate().skip(20) {
            shortcut.current_module_object = family.into();
            require_empty(self.source_surface_counts[index], family)?;
        }
        self.get_empty_ht_subsurface_data(input, shortcut, backend, errors)?;
        // GetRectSubSurfaces1194 /5430-5744, passed zero counts in source order.
        for (index, &(family, _)) in FAMILIES.iter().enumerate().take(20).skip(14) {
            shortcut.current_module_object = family.into();
            require_empty(self.source_surface_counts[index], family)?;
        }
        self.get_empty_attached_shading_data(input, shortcut, backend, errors)?;

        // GetSimpleShdSurfaceData1209 /6575-6941: zero total does not warn;
        // the four outer current-module writes still occur, no argument reset.
        for (index, &(family, _)) in FAMILIES.iter().enumerate().take(14).skip(10) {
            shortcut.current_module_object = family.into();
            require_empty(self.source_surface_counts[index], family)?;
        }

        // GetIntMassSurfaceData1211 /6987-6991 performs its second real query.
        shortcut.current_module_object = "InternalMass".into();
        require_empty(
            signed(input.get_num_objects_found(&shortcut.current_module_object))?,
            "positive InternalMass getter",
        )?;
        // No IntMassObjects allocation, unused erase or getObjectItem at count0.
        // Original1213 final total precedes the actual1215 error/fatal boundary.
        let base = signed(self.surfaces_read)?;
        let add_base = self
            .need_to_add_surfaces
            .ok_or(Error::Unavailable("actual GetHT need-to-add reset"))?;
        let add_sub = self
            .need_to_add_subsurfaces
            .ok_or(Error::Unavailable("actual GetHTSub need-to-add reset"))?;
        self.total_surfaces = base
            .checked_add(self.added_subsurfaces)
            .and_then(|value| value.checked_add(add_base))
            .and_then(|value| value.checked_add(add_sub))
            .ok_or(Error::Unavailable("Source signed final surface count"))?;
        Ok(())
    }
}
