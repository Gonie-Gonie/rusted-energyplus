//! SetupZoneGeometry370-454: retained opaque area and exterior perimeter totals.
use super::{Error, SelectedInputProcessor, SurfaceClass, SurfaceGeometryInputState};
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

fn index(number: i32) -> Result<usize, Error> {
    usize::try_from(number)
        .ok()
        .and_then(|value| value.checked_sub(1))
        .ok_or(Error::Unavailable("positive Source area association"))
}

impl SurfaceGeometryInputState {
    pub(crate) fn accumulate_opaque_setup_areas(
        &self,
        input: &SelectedInputProcessor,
        owners: &mut SelectedZoneInputOwners,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<bool, Error> {
        // The genuine debug header precedes every area write. Its positive
        // OutputFile branch is unavailable; it is never replaced by a sink.
        let detailed_wwr = input
            .get_num_sections_found("DETAILEDWWR_DEBUG")
            .map_err(|_| Error::Unavailable("actual DetailedWWR section count"))?
            > 0;
        if detailed_wwr {
            return Err(Error::Unavailable("positive DetailedWWR debug OutputFile"));
        }
        let surfaces = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("same final Source area Surface owner"))?;
        if usize::try_from(self.total_surfaces).ok() != Some(surfaces.len()) {
            return Err(Error::Unavailable("actual Source area Surface bounds"));
        }
        let zones = owners
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same Source area Zone owner"))?;
        let spaces = owners
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("same Source area Space owner"))?;
        // Final Source order, with no reset of the constructor/current totals.
        for (surface_index, surface) in surfaces.iter().enumerate() {
            let row = &surface.row;
            if !row.heat_transfer_surface && !row.is_air_boundary_surface {
                continue;
            }
            let zone = zones
                .get_mut(index(row.zone)?)
                .ok_or(Error::Unavailable("actual area Surface Zone association"))?;
            let space = spaces
                .get_mut(index(row.space)?)
                .ok_or(Error::Unavailable("actual area Surface Space association"))?;
            if space.zone_num != row.zone {
                return Err(Error::Unavailable("same area Zone/Space association"));
            }
            let area = row
                .vertex_progress
                .properties
                .ok_or(Error::Unavailable("actual retained Surface.Area"))?
                .area_m2;
            zone.total_surface_area += area;
            space.total_surface_area += area;
            let gross_area = row
                .vertex_progress
                .normal_area
                .ok_or(Error::Unavailable("actual retained Surface.GrossArea"))?
                .gross_area_m2;
            if row.class == SurfaceClass::Roof {
                zone.geometric_ceiling_area += gross_area;
            } else if row.class == SurfaceClass::Floor {
                zone.geometric_floor_area += gross_area;
            }
            // Original392 reaches Construction only after the preceding writes.
            let construction = control
                .construction_usage
                .get(index(row.construction)?)
                .ok_or(Error::Unavailable("actual area Construction association"))?;
            if construction.type_is_window {
                return Err(Error::Unavailable("positive area window/frame branch"));
            }
            if row.exterior_boundary == 0 {
                zone.exterior_total_surface_area += gross_area;
                space.exterior_total_surface_area += gross_area;
                if row.class == SurfaceClass::Wall {
                    zone.exterior_net_wall_area += area;
                    zone.exterior_gross_wall_area += gross_area;
                    space.exterior_gross_wall_area += gross_area;
                    zone.exterior_gross_wall_area_multiplied +=
                        gross_area * f64::from(zone.multiplier) * f64::from(zone.list_multiplier);
                    zone.exterior_perimeter += row.vertex_progress.width;
                    space.exterior_perimeter += row.vertex_progress.width;
                }
            } else {
                let source_number = super::signed(surface_index + 1)?;
                if row.exterior_boundary != source_number || row.base_surface != source_number {
                    // Ground/OSCM/other positive getters remain excluded. This
                    // reached boundary retains total/geometric writes above.
                    return Err(Error::Unavailable(
                        "non-Outdoors/self-Adiabatic area boundary",
                    ));
                }
            }
        }
        // DetailedWWR is false, so its450-453 footer has no side effect.
        Ok(detailed_wwr) // Carry the same370 local into the455 height traversal.
    }
}
