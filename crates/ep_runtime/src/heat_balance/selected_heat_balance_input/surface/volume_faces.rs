//! Retained actual final faces for the existing bounded height/volume kernel.
//! No property bundle or Source CreateNewellAreaVector invocation is repeated.
use super::{Error, SurfaceClass, SurfaceGeometryInputState};
use crate::geometry::ZoneVolumeFace;
use ep_model::{Point3, SurfaceType};

impl SurfaceGeometryInputState {
    pub(crate) fn retained_zone_volume_faces(
        &self,
        zone_number: i32,
    ) -> Result<Vec<ZoneVolumeFace>, Error> {
        let rows = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("final Source Surface volume faces"))?;
        let mut faces = Vec::new();
        // Final actual AllSurface storage order, not a typed class re-sort.
        for surface in rows
            .iter()
            .filter(|surface| surface.row.zone == zone_number)
        {
            let row = &surface.row;
            let surface_type = match row.class {
                SurfaceClass::Wall => SurfaceType::Wall,
                SurfaceClass::Floor => SurfaceType::Floor,
                SurfaceClass::Roof => SurfaceType::Roof,
                _ => return Err(Error::Unavailable("selected opaque final volume class")),
            };
            let quantities = row
                .vertex_progress
                .properties
                .ok_or(Error::Unavailable("retained volume face quantities"))?;
            let points = row
                .vertex
                .as_ref()
                .ok_or(Error::Unavailable("retained volume Vertex allocation"))?
                .iter()
                .map(|value| {
                    let [x_m, y_m, z_m] =
                        value.ok_or(Error::Unavailable("retained written volume Vertex"))?;
                    Ok(Point3 { x_m, y_m, z_m })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            faces.push(ZoneVolumeFace {
                surface_id: surface
                    .typed_id
                    .ok_or(Error::Unavailable("bound physical volume SurfaceId"))?,
                name: row.name.clone(),
                surface_type,
                vertices: points,
                area_m2: quantities.area_m2,
                gross_area_m2: quantities.gross_area_m2,
                newell_area_vector_m2: quantities.newell_area_vector_m2,
                tilt_deg: quantities.tilt_deg,
            });
        }
        Ok(faces)
    }
}
