//! Setup455-547: actual Space traversal, reused height math and warning order.
use super::final_mapping::FinalSurface;
use super::{Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState};
use crate::geometry::{ZoneGeometryProperties, ZoneVolumeFace, calculated_zone_height};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use ep_model::{Point3, SurfaceType};

fn index(number: i32) -> Result<usize, Error> {
    usize::try_from(number)
        .ok()
        .and_then(|value| value.checked_sub(1))
        .ok_or(Error::Unavailable("positive Source height association"))
}

fn retained_height_face(surface: &FinalSurface) -> Result<ZoneVolumeFace, Error> {
    let row = &surface.row;
    let properties = row
        .vertex_progress
        .properties
        .ok_or(Error::Unavailable("actual retained height face properties"))?;
    let vertices = row
        .vertex
        .as_ref()
        .ok_or(Error::Unavailable("actual height Vertex allocation"))?;
    if vertices.len() < 3 || usize::try_from(row.sides).ok() != Some(vertices.len()) {
        // The legacy numerical helper silently skips empty faces. The actual
        // selected Source caller requires its complete written polygon instead.
        return Err(Error::Unavailable("complete Source height face bounds"));
    }
    Ok(ZoneVolumeFace {
        surface_id: surface
            .typed_id
            .ok_or(Error::Unavailable("actual bound height SurfaceId"))?,
        name: row.name.clone(),
        surface_type: match row.class {
            SurfaceClass::Wall => SurfaceType::Wall,
            SurfaceClass::Floor => SurfaceType::Floor,
            SurfaceClass::Roof => SurfaceType::Roof,
            _ => return Err(Error::Unavailable("selected opaque height face class")),
        },
        vertices: vertices
            .iter()
            .map(|value| {
                let [x_m, y_m, z_m] =
                    value.ok_or(Error::Unavailable("actual written height Vertex"))?;
                Ok(Point3 { x_m, y_m, z_m })
            })
            .collect::<Result<_, Error>>()?,
        area_m2: row.area,
        gross_area_m2: row.gross_area,
        newell_area_vector_m2: properties.newell_area_vector_m2,
        tilt_deg: properties.tilt_deg,
    })
}

impl SurfaceGeometryInputState {
    pub(crate) fn prepare_opaque_setup_heights(
        &self,
        owners: &mut SelectedZoneInputOwners,
        display_extra_warnings: bool,
        detailed_wwr: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let surfaces = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("same final Source height Surface owner"))?;
        let zones = owners
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same Source height Zone owner"))?;
        let spaces = owners
            .spaces
            .as_ref()
            .ok_or(Error::Unavailable("same Source height Space owner"))?;
        for (zone_index, zone) in zones.iter_mut().enumerate() {
            if detailed_wwr {
                return Err(Error::Unavailable(
                    "positive DetailedWWR Zone debug OutputFile",
                ));
            }
            let zone_number = super::signed(zone_index + 1)?;
            let mut faces = Vec::new();
            for &space_number in &zone.space_indexes {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or(Error::Unavailable("actual height Zone.spaceIndexes"))?;
                if space.zone_num != zone_number {
                    return Err(Error::Unavailable("same height Space Zone association"));
                }
                // Source AllSurface ranges include air boundaries; there is no
                // HT-only filter, typed class sort or vertex/property recompute.
                for number in space.all_surface_first..=space.all_surface_last {
                    let surface = surfaces
                        .get(index(number)?)
                        .ok_or(Error::Unavailable("actual height Space AllSurface range"))?;
                    if surface.row.zone != zone_number || surface.row.space != space_number {
                        return Err(Error::Unavailable(
                            "same height Surface Zone/Space association",
                        ));
                    }
                    faces.push(retained_height_face(surface)?);
                }
            }
            if faces.is_empty() {
                return Err(Error::Unavailable(
                    "nonempty selected Source height traversal",
                ));
            }
            let numeric = ZoneGeometryProperties {
                volume_m3: zone.volume,
                ceiling_height_m: zone.ceiling_height,
                floor_area_m2: zone.floor_area,
                user_entered_floor_area_m2: zone.user_entered_floor_area,
                geometric_floor_area_m2: zone.geometric_floor_area,
                ceiling_area_m2: zone.ceiling_area,
                geometric_ceiling_area_m2: zone.geometric_ceiling_area,
                faces,
                ..Default::default()
            };
            let average = calculated_zone_height(&numeric)
                .map_err(|_| Error::Unavailable("selected Source height numerical domain"))?;
            if zone.ceiling_height > 0.0 {
                zone.ceiling_height_entered = true;
                if average > 0.0
                    && (average - zone.ceiling_height).abs() / zone.ceiling_height > 0.05
                {
                    // Literal ErrCount read: this routine does not increment it.
                    if self.err_count == 1 && !display_extra_warnings {
                        backend.show_warning_error("SetUpZoneGeometry: Entered Ceiling Height for some zone(s) significantly different from calculated Ceiling Height")?;
                        backend.show_continue_error("...use Output:Diagnostics,DisplayExtraWarnings; to show more details on each max iteration exceeded.")?;
                    }
                    if display_extra_warnings {
                        backend.show_warning_error(&format!("SetUpZoneGeometry: Entered Ceiling Height for Zone=\"{}\" significantly different from calculated Ceiling Height", zone.name))?;
                        if !average.is_finite() {
                            return Err(Error::Unavailable("nonfinite Source height .2F branch"));
                        }
                        // IOFiles.hh .F falls directly through to fmt fixed
                        // output; do not substitute the .R nextafter nudges.
                        let entered = format!("{:.2}", zone.ceiling_height);
                        let calculated = format!("{average:.2}");
                        backend.show_continue_error(&format!("SetUpZoneGeometry: Entered Ceiling Height={entered}, Calculated Ceiling Height={calculated}, entered height will be used in calculations."))?;
                    }
                }
            }
            if zone.ceiling_height <= 0.0 && average > 0.0 {
                zone.ceiling_height = average;
            }
            if !zone.ceiling_height.is_finite() {
                return Err(Error::Unavailable("nonfinite calculated Source height"));
            }
        }
        Ok(()) // The separate548 CalculateZoneVolume closure is still qualified.
    }
}
