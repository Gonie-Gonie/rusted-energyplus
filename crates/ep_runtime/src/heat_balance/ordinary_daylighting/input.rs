//! Actual first-call input/check/association prefix and true RefPt.size0 return.
use super::*;
impl SourceDaylighting {
    /// DaylightingManager398-420; the real three children precede flag clear.
    pub(crate) fn calculate_daylighting_coefficients(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        zones: &SelectedZoneInputOwners,
        radiation: &mut OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        modules: &SourceGainModules,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.calc_coefficients_first_time {
            self.get_daylighting_parameters(input, shortcut, zones, radiation, geometry, control)?;
            // Source4983 child reads the actual sole device arrays, not raw
            // object guesses or a default-success provider.
            if !empty(&modules.tdd_pipe) {
                return Err(Error::Unavailable(
                    "CheckTDDsAndLightShelves actual positive pipe.Diffuser owner",
                ));
            }
            if !empty(&modules.shelf) {
                return Err(Error::Unavailable(
                    "CheckTDDsAndLightShelves actual positive shelf.Window owner",
                ));
            }
            // Selected GetWindowShadingControl input stops at positive count
            // before creating rows. Its actual retained0 proves this array's
            // Source iteration empty; no independent numeric registry is made.
            if geometry.total_window_shading_control != 0 {
                return Err(Error::Unavailable(
                    "AssociateWindowShadingControl actual WindowShadingControl array",
                ));
            }
            self.calc_coefficients_first_time = false;
        }
        if empty(&self.daylight_reference_points) {
            return Ok(());
        }
        let _ = backend;
        Err(Error::Unavailable(
            "CalcDayltgCoefficients positive actual DaylRefPt suffix",
        ))
    }

    fn get_daylighting_parameters(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        zones: &SelectedZoneInputOwners,
        radiation: &mut OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        if !self.get_parameters_input_flag {
            return Ok(());
        }
        self.get_parameters_input_flag = false;
        shortcut.current_module_object = "Daylighting:Controls".into();
        let controls = input.get_num_objects_found(&shortcut.current_module_object);
        if controls > 0 {
            let count = usize::try_from(radiation.num_solar_enclosures())
                .map_err(|_| Error::Unavailable("Source Dayltg enclosure allocation bounds"))?;
            self.enclosure_daylight = Some(vec![SourceEnclosureDaylight::default(); count]);
            return Err(Error::Unavailable(
                "GetInputDayliteRefPt actual positive Controls getter child3820",
            ));
        }
        self.max_num_ref_pt_in_any_daylight_control = 0;
        self.max_num_ref_pt_in_any_enclosure = 0;
        if !empty(&self.daylight_controls) {
            return Err(Error::Unavailable(
                "GetParameters actual positive control.TotalDaylRefPoints",
            ));
        }
        for number in 1..=radiation.num_solar_enclosures() {
            let count = solar(radiation, number)?.total_enclosure_daylight_reference_points;
            self.max_num_ref_pt_in_any_enclosure = count.max(self.max_num_ref_pt_in_any_enclosure);
        }
        // Objexx member maxval seeds empty arrays with numeric_limits<int>::lowest,
        // not0. Native reads the first member then traverses begin..end again.
        let zone_rows = zones
            .zones
            .as_deref()
            .ok_or(Error::Unavailable("same Zone allocation for Dayltg maxval"))?;
        let mut zone_max = zone_rows.first().map_or(i32::MIN, |z| z.num_sub_surfaces);
        for zone in zone_rows {
            zone_max = zone_max.max(zone.num_sub_surfaces);
        }
        let rows = self.enclosure_daylight.as_deref().unwrap_or_default();
        let mut enclosure_max = rows
            .first()
            .map_or(i32::MIN, |z| z.number_daylighting_exterior_windows);
        for row in rows {
            enclosure_max = enclosure_max.max(row.number_daylighting_exterior_windows);
        }
        self.max_enclosure_sub_surfaces = zone_max.max(enclosure_max);
        for &number in &geometry.surface_lists.all_ht_window {
            let row = surface_row(geometry, number)?;
            let count =
                solar(radiation, row.solar_encl_index)?.total_enclosure_daylight_reference_points;
            let surface_index = index(number)?;
            let _window = geometry
                .surface_windows
                .as_ref()
                .and_then(|rows| rows.get(surface_index))
                .ok_or(Error::Unavailable(
                    "same Source SurfaceWindow in GetParameters3849",
                ))?;
            if count > 0 {
                let initialized = *geometry
                    .window_arrays
                    .as_ref()
                    .and_then(|arrays| arrays.surf_win_surf_day_light_init.get(surface_index))
                    .ok_or(Error::Unavailable("actual SurfWinSurfDayLightInit at3851"))?;
                if !initialized {
                    return Err(Error::Unavailable(
                        "GetParameters actual positive SurfaceWindow.refPts allocation3852",
                    ));
                }
            } else if row.exterior_boundary > 0 {
                let adjacent = surface_row(geometry, row.exterior_boundary)?;
                if solar(radiation, adjacent.solar_encl_index)?
                    .total_enclosure_daylight_reference_points
                    > 0
                {
                    let initialized = *geometry
                        .window_arrays
                        .as_ref()
                        .and_then(|arrays| arrays.surf_win_surf_day_light_init.get(surface_index))
                        .ok_or(Error::Unavailable(
                            "actual adjacent SurfWinSurfDayLightInit at3866",
                        ))?;
                    if !initialized {
                        return Err(Error::Unavailable(
                            "GetParameters actual adjacent SurfaceWindow.refPts allocation3867",
                        ));
                    }
                }
            }
            if row.exterior_boundary != 0 {
                continue;
            }
            if !row.has_shade_control {
                continue;
            }
            return Err(Error::Unavailable(
                "GetParameters actual positive WindowShadingControl glare owner3885",
            ));
        }
        if !control.any_air_boundary {
            for number in 1..=geometry.total_surfaces {
                let row = surface_row(geometry, number)?;
                if row.class != SurfaceClass::Window || !row.exterior_solar {
                    continue;
                }
                let enclosure = solar(radiation, row.solar_encl_index)?;
                if enclosure.total_enclosure_daylight_reference_points == 0
                    || enclosure.has_interzone_window
                {
                    continue;
                }
                let daylight = self
                    .enclosure_daylight
                    .as_deref()
                    .and_then(|rows| index(row.solar_encl_index).ok().and_then(|i| rows.get(i)))
                    .ok_or(Error::Unavailable(
                        "actual Dayltg enclosure for output registration",
                    ))?;
                if !daylight.has_split_flux_daylighting {
                    continue;
                }
                return Err(Error::Unavailable(
                    "GetParameters actual positive daylight ref output registration3958",
                ));
            }
        } else {
            for number in 1..=radiation.num_solar_enclosures() {
                let enclosure = solar(radiation, number)?;
                for &surface in enclosure
                    .source_surface_ptr
                    .as_deref()
                    .ok_or(Error::Unavailable("actual Source Solar SurfacePtr"))?
                {
                    let i = index(surface)?;
                    let row = surface_row(geometry, surface)?;
                    let _window = geometry
                        .surface_windows
                        .as_ref()
                        .and_then(|rows| rows.get(i))
                        .ok_or(Error::Unavailable(
                            "actual same SurfaceWindow in AirBoundary traversal",
                        ))?;
                    if row.class != SurfaceClass::Window || !row.exterior_solar {
                        continue;
                    }
                    if enclosure.total_enclosure_daylight_reference_points == 0
                        || enclosure.has_interzone_window
                    {
                        continue;
                    }
                    let daylight = self
                        .enclosure_daylight
                        .as_deref()
                        .and_then(|rows| rows.get(index(number).ok()?))
                        .ok_or(Error::Unavailable(
                            "actual EnclDaylight in AirBoundary traversal",
                        ))?;
                    if !daylight.has_split_flux_daylighting {
                        continue;
                    }
                    return Err(Error::Unavailable(
                        "GetParameters actual positive AirBoundary daylight output3987",
                    ));
                }
            }
        }
        // doesDayLightingUseDElight iterates this actual control array. Any
        // positive row requires the actual DaylightMethod before a branch.
        if !empty(&self.daylight_controls) {
            return Err(Error::Unavailable(
                "doesDayLightingUseDElight actual positive DaylightMethod",
            ));
        }
        shortcut.current_module_object = "Output:DaylightFactors".into();
        let reports = input.get_num_objects_found(&shortcut.current_module_object);
        if reports > 0 {
            return Err(Error::Unavailable(
                "Output:DaylightFactors actual getObjectItem4088",
            ));
        }
        // ErrorsFound is the actual local false and none of these zero loops
        // can write true; no fatal/diagnostic/output was suppressed.
        Ok(())
    }
}
