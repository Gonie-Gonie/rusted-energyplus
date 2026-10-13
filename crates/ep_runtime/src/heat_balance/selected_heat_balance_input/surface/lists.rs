//! Actual selected SurfaceGeometry2601-2791 list/obstruction/filter effects.
use super::super::{SelectedConstructionUsage, SelectedHeatBalanceInputState};
use super::{
    Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners,
};

// Native enum Invalid=-1 is not an array index; Num=11 is its length.
#[derive(Clone, Copy)]
#[repr(usize)]
enum Filter {
    ExteriorSurfaces = 0,
    ExteriorWindows,
    ExteriorWalls,
    ExteriorRoofs,
    ExteriorFloors,
    InteriorSurfaces,
    InteriorWindows,
    InteriorWalls,
    InteriorRoofs,
    InteriorCeilings,
    InteriorFloors,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceLists {
    pub(crate) all_ht: Vec<i32>,
    pub(crate) all_exterior_solar: Vec<i32>,
    pub(crate) all_shadow_possible_obstructions: Vec<i32>,
    pub(crate) all_interzone: Vec<i32>,
    pub(crate) all_ht_non_window: Vec<i32>,
    pub(crate) all_ht_window: Vec<i32>,
    pub(crate) all_exterior_solar_window: Vec<i32>,
    pub(crate) all_exterior_solar_window_with_frame: Vec<i32>,
    pub(crate) all_ht_kiva: Vec<i32>,
    pub(crate) filters: [Vec<i32>; 11],
}

fn index(value: i32) -> Result<usize, Error> {
    value
        .checked_sub(1)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Error::Unavailable("positive final Source list index"))
}
fn construction(
    control: &SelectedHeatBalanceInputState,
    number: i32,
) -> Result<&SelectedConstructionUsage, Error> {
    control
        .construction_usage
        .get(index(number)?)
        .ok_or(Error::Unavailable("same Construction type/list owner"))
}
fn reserve(list: &mut Vec<i32>, requested: usize) {
    // C++ reserve(n) is absolute; Rust reserve is additional to current length.
    // Capacity/address/allocator/OOM equivalence is not claimed.
    if requested > list.len() {
        list.reserve(requested - list.len());
    }
}
impl SurfaceLists {
    fn reserve_source_lists(&mut self, total: usize, windows: usize) -> Result<(), Error> {
        reserve(&mut self.all_ht, total);
        reserve(&mut self.all_exterior_solar, total);
        reserve(&mut self.all_shadow_possible_obstructions, total);
        reserve(&mut self.all_interzone, total);
        let opaque = total
            .checked_sub(windows)
            .ok_or(Error::Unavailable("Source TotSurfaces minus TotWindows"))?;
        reserve(&mut self.all_ht_non_window, opaque);
        reserve(&mut self.all_ht_window, windows);
        reserve(&mut self.all_exterior_solar_window, windows);
        reserve(&mut self.all_exterior_solar_window_with_frame, windows);
        reserve(&mut self.all_ht_kiva, total);
        Ok(())
    }
    fn push_filter(&mut self, filter: Filter, surface: i32) {
        self.filters[filter as usize].push(surface);
    }
}

impl SurfaceGeometryInputState {
    fn set_surface_eco_roof_flags(
        &mut self,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        let rows = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("final list Surface allocation"))?;
        let array = &mut self
            .parallel_arrays
            .as_mut()
            .ok_or(Error::Unavailable("actual allocated SurfExtEcoRoof"))?
            .surf_ext_eco_roof;
        let mut outside_layer = 0;
        for (i, surface) in rows.iter().enumerate() {
            let row = &surface.row;
            let flag = array
                .get_mut(i)
                .ok_or(Error::Unavailable("SurfExtEcoRoof actual bounds"))?;
            if row.construction > 0 {
                *flag = construction(control, row.construction)?.type_is_eco_roof;
            }
            if !*flag {
                continue;
            }
            let layer = construction(control, row.construction)?.layer_points[0];
            if outside_layer == 0 {
                outside_layer = layer;
                continue;
            }
            if outside_layer != layer {
                // Positive EcoRoof producers are excluded. Retain prior flags
                // and stop before the unported real material-name diagnostics.
                return Err(Error::Unavailable(
                    "positive conflicting EcoRoof material diagnostics",
                ));
            }
        }
        Ok(())
    }

    fn populate_opaque_surface_lists(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        errors_found: bool,
    ) -> Result<(), Error> {
        let rows = self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("final list Surface allocation"))?;
        let zones = owners
            .zones
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same Zone list allocation"))?;
        for i in 0..rows.len() {
            let number = i32::try_from(i)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(Error::Unavailable("Source surface list int capacity"))?;
            rows[i].row.is_shadow_possible_obstruction = false;
            let row = &rows[i].row;
            if row.exterior_solar {
                self.surface_lists.all_exterior_solar.push(number);
            }
            if row.heat_transfer_surface {
                self.surface_lists.all_ht.push(number);
                let zone_index = index(row.zone)?;
                let zone = zones
                    .get_mut(zone_index)
                    .ok_or(Error::Unavailable("actual Zone list bounds"))?;
                zone.ht_surface_list.push(number);
                if row.class == SurfaceClass::Window {
                    self.surface_lists.all_ht_window.push(number);
                    zone.ht_window_surface_list.push(number);
                    if row.exterior_solar {
                        self.surface_lists.all_exterior_solar_window.push(number);
                        if row.frame_divider > 0 {
                            self.surface_lists
                                .all_exterior_solar_window_with_frame
                                .push(number);
                        }
                    }
                } else {
                    self.surface_lists.all_ht_non_window.push(number);
                    zone.ht_non_window_surface_list.push(number);
                }
                if row.exterior_boundary > 0 && row.exterior_boundary != number {
                    self.surface_lists.all_interzone.push(number);
                    zone.iz_surface_list.push(number);
                    let adjacent = rows
                        .get(index(row.exterior_boundary)?)
                        .ok_or(Error::Unavailable("actual adjacent Surface list bounds"))?;
                    let adjacent_zone = zones
                        .get_mut(index(adjacent.row.zone)?)
                        .ok_or(Error::Unavailable("actual adjacent Zone list bounds"))?;
                    adjacent_zone.ht_surface_list.push(number);
                    adjacent_zone.iz_surface_list.push(number);
                    if row.class == SurfaceClass::Window {
                        adjacent_zone.ht_window_surface_list.push(number);
                    } else {
                        adjacent_zone.ht_non_window_surface_list.push(number);
                    }
                }
            }
            let row = &rows[i].row;
            if row.heat_transfer_surface && row.exterior_boundary > 0 {
                continue;
            }
            if row.heat_transfer_surface && row.exterior_boundary == -1 {
                continue;
            } // Ground
            if row.heat_transfer_surface && row.exterior_boundary == -6 {
                // KivaFoundation
                self.surface_lists.all_ht_kiva.push(number);
                if !errors_found {
                    return Err(Error::Unavailable(
                        "positive Kiva foundation surface association",
                    ));
                }
                continue;
            }
            if row.heat_transfer_surface && row.exterior_boundary == -2 {
                continue;
            }
            if row.heat_transfer_surface && row.exterior_boundary == -3 {
                continue;
            }
            if matches!(row.class, SurfaceClass::Window | SurfaceClass::Door) {
                continue;
            }
            if row.mirrored_surface {
                continue;
            }
            if row.is_air_boundary_surface {
                continue;
            }
            rows[i].row.is_shadow_possible_obstruction = true;
            self.surface_lists
                .all_shadow_possible_obstructions
                .push(number);
        }
        Ok(())
    }

    fn check_surface_irt_uses(
        &self,
        owners: &SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if !owners
            .control
            .construction_usage
            .iter()
            .any(|row| row.type_is_irt)
        {
            return Ok(());
        }
        let rows = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("IRT final Surface allocation"))?;
        let mut count = 0i32;
        for (i, surface) in rows.iter().enumerate() {
            let row = &surface.row;
            if !row.heat_transfer_surface {
                continue;
            }
            let number = i32::try_from(i)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(Error::Unavailable("Source IRT surface int capacity"))?;
            if row.exterior_boundary > 0 && row.exterior_boundary != number {
                continue;
            }
            if !construction(owners.control, row.construction)?.type_is_irt {
                continue;
            }
            if !owners.globals.display_extra_warnings {
                count = count
                    .checked_add(1)
                    .ok_or(Error::Unavailable("Source IRT count int capacity"))?;
            } else {
                backend.show_warning_error(&format!("GetSurfaceData: Surface=\"{}\" uses InfraredTransparent construction in a non-interzone surface. (illegal use)", row.name))?;
            }
        }
        if count > 0 {
            backend.show_warning_error(&format!("GetSurfaceData: Surfaces use InfraredTransparent constructions {count} in non-interzone surfaces. (illegal use)"))?;
            backend.show_continue_error(
                "For explicit details on each use, use Output:Diagnostics,DisplayExtraWarnings;",
            )?;
        }
        Ok(())
    }

    fn populate_source_surface_filters(
        &mut self,
        control: &SelectedHeatBalanceInputState,
        total: usize,
    ) -> Result<(), Error> {
        // Source starts at1, so AllExteriorSurfaces index0 is NOT reserved here.
        for filter in self.surface_lists.filters.iter_mut().skip(1) {
            reserve(filter, total);
        }
        let rows = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("filter final Surface allocation"))?;
        for (i, surface) in rows.iter().enumerate() {
            let row = &surface.row;
            if !row.heat_transfer_surface {
                continue;
            }
            let number = i32::try_from(i)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(Error::Unavailable("Source filter surface int capacity"))?;
            let interior = row.exterior_boundary > 0; // includes adiabatic self
            self.surface_lists.push_filter(
                if interior {
                    Filter::InteriorSurfaces
                } else {
                    Filter::ExteriorSurfaces
                },
                number,
            );
            if construction(control, row.construction)?.type_is_window {
                self.surface_lists.push_filter(
                    if interior {
                        Filter::InteriorWindows
                    } else {
                        Filter::ExteriorWindows
                    },
                    number,
                );
            } else {
                match row.class {
                    SurfaceClass::Wall => self.surface_lists.push_filter(
                        if interior {
                            Filter::InteriorWalls
                        } else {
                            Filter::ExteriorWalls
                        },
                        number,
                    ),
                    SurfaceClass::Floor => self.surface_lists.push_filter(
                        if interior {
                            Filter::InteriorFloors
                        } else {
                            Filter::ExteriorFloors
                        },
                        number,
                    ),
                    SurfaceClass::Roof => {
                        self.surface_lists.push_filter(
                            if interior {
                                Filter::InteriorRoofs
                            } else {
                                Filter::ExteriorRoofs
                            },
                            number,
                        );
                        // Literal2788 uses AllInteriorCeilings even for exterior Roof.
                        self.surface_lists
                            .push_filter(Filter::InteriorCeilings, number);
                    }
                    SurfaceClass::None
                    | SurfaceClass::Invalid
                    | SurfaceClass::Window
                    | SurfaceClass::Door => {}
                }
            }
        }
        Ok(())
    }

    pub(crate) fn build_opaque_surface_lists(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: bool,
    ) -> Result<(), Error> {
        self.set_surface_eco_roof_flags(owners.control)?;
        let total = usize::try_from(self.total_surfaces)
            .map_err(|_| Error::Unavailable("Source TotSurfaces list reserve"))?;
        let windows = usize::try_from(self.total_windows)
            .map_err(|_| Error::Unavailable("Source TotWindows list reserve"))?;
        self.surface_lists.reserve_source_lists(total, windows)?;
        self.populate_opaque_surface_lists(owners, errors_found)?;
        self.check_surface_irt_uses(owners, backend)?;
        self.populate_source_surface_filters(owners.control, total)
    }
}
