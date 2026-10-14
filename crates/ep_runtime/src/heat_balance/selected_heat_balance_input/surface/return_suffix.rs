//! Genuine selected GetSurfaceData2859-2915 suffix, after radiant enclosures.
//! Positive Ground/Surrounding/Local objects and representative producers remain
//! outside admission. The zero loops retain genuine EPVector allocation state.
use super::super::super::selected_input_processor::SourceInputFamily;
use super::{Error, SourceErrorBackend, SurfaceGeometryInputState, SurfaceGetterOwners, signed};

impl SurfaceGeometryInputState {
    fn read_empty_ground_surface_properties(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        let family = "SurfaceProperty:GroundSurfaces";
        owners.shortcut.current_module_object = family.into();
        self.total_surface_ground_properties = signed(owners.input.get_num_objects_found(family))?;
        // Literal8254: count does not insert null. An earlier definition query
        // may have done so; the direct exact-case find still occurs either way.
        let Some(instances) = owners.input.find_exact_family(family) else {
            if self.total_surface_ground_properties > 0 {
                *errors = true;
            }
            return Ok(()); // Actual8259, before iteration/report branches.
        };
        match instances {
            SourceInputFamily::Null => {} // Null begin==end, not absent-find return.
            SourceInputFamily::Objects(objects) if objects.is_empty() => {}
            SourceInputFamily::Objects(_) => {
                return Err(Error::Unavailable("positive GroundSurfaces instance body"));
            }
        }
        // Actual8263 has no instances, and8319's positive report predicate is false.
        // No GroundSurfsProperty.push_back or allocation is manufactured.
        Ok(())
    }

    fn read_empty_surrounding_surface_properties(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
    ) -> Result<(), Error> {
        let family = "SurfaceProperty:SurroundingSurfaces";
        owners.shortcut.current_module_object = family.into();
        let total_surrounding_properties = signed(owners.input.get_num_objects_found(family))?;
        if total_surrounding_properties > 0 {
            return Err(Error::Unavailable(
                "positive SurroundingSurfaces allocation/items",
            ));
        }
        // Actual8154 skips the allocate/getObjectItem loop, leaving the same owner.
        Ok(())
    }

    fn read_empty_surface_local_environments(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
    ) -> Result<(), Error> {
        let family = "SurfaceProperty:LocalEnvironment";
        owners.shortcut.current_module_object = family.into();
        self.total_surface_local_environments = signed(owners.input.get_num_objects_found(family))?;
        if self.total_surface_local_environments > 0 {
            // Stop before unsupported AnyLocalEnvironmentsInModel/allocate/item writes.
            return Err(Error::Unavailable(
                "positive LocalEnvironment global/allocation/items",
            ));
        }
        // Actual7974 skips the complete positive body, including its final Surface loop.
        Ok(())
    }

    fn selected_surface_fatal_if_errors(
        &self,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        if self.surface_error || *errors {
            *errors = true; // Actual2866/2912 occurs before the backend/fatal call.
            return match backend
                .show_fatal_error("GetSurfaceData: Errors discovered, program terminates.")
            {
                Err(error) => Err(Error::Fatal(error)),
                Ok(never) => match never {},
            };
        }
        Ok(())
    }

    pub(crate) fn finish_selected_surface_input(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        self.read_empty_ground_surface_properties(owners, errors)?; // Actual2859.
        self.read_empty_surrounding_surface_properties(owners)?; // Actual2861.
        self.read_empty_surface_local_environments(owners)?; // Actual2863.
        self.selected_surface_fatal_if_errors(backend, errors)?; // Actual2865.

        // Actual2870 uses these nine retained locals, not a second IP query.
        let c = self.source_surface_counts;
        let total_shading_surfaces = c[0]
            .checked_add(c[1])
            .and_then(|v| v.checked_add(c[2]))
            .and_then(|v| v.checked_add(c[3]))
            .and_then(|v| v.checked_add(c[9]))
            .and_then(|v| v.checked_add(c[10]))
            .and_then(|v| v.checked_add(c[11]))
            .and_then(|v| v.checked_add(c[12]))
            .and_then(|v| v.checked_add(c[13]))
            .ok_or(Error::Unavailable("Original signed TotShadSurf sum"))?;
        // Unconditional2872, including the genuine zero-shading selected route.
        let _num_delight_complex_fenestration = signed(
            owners
                .input
                .get_num_objects_found("Daylighting:DElight:ComplexFenestration"),
        )?;
        if total_shading_surfaces > 0 {
            // Positive shadow getters are excluded earlier. The actual left &&
            // operand is false in this closure, so no daylight helper is called.
            return Err(Error::Unavailable(
                "positive shading DElight warning predicate",
            ));
        }

        for number in 1..=self.total_surfaces {
            // Actual2878 order and same owners.
            let index = usize::try_from(number - 1)
                .map_err(|_| Error::Unavailable("actual Surface loop index"))?;
            let surface = self
                .final_surfaces
                .as_mut()
                .and_then(|rows| rows.get_mut(index))
                .ok_or(Error::Unavailable("actual final Surface runtime owner"))?;
            let active = self
                .window_arrays
                .as_mut()
                .and_then(|arrays| arrays.surf_active_construction.get_mut(index))
                .ok_or(Error::Unavailable(
                    "same allocated SurfActiveConstruction owner",
                ))?;
            *active = surface.row.construction; // Actual2881.
            surface.representative_calc_surface_num = number; // Actual2882.
        }
        if self.use_representative_surface_calculations {
            // The earlier PerformancePrecisionTradeoffs true writer is outside
            // admission. Stop at the unsupported true branch before its extra
            // count/helper; false short-circuits the genuine2886 && entirely.
            return Err(Error::Unavailable(
                "positive representative Surface calculations",
            ));
        }
        self.selected_surface_fatal_if_errors(backend, errors)?; // Actual2911.
        self.selected_get_surface_data_returned = true; // Bounded body return only.
        Ok(())
    }
}
