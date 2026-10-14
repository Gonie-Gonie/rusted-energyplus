//! Actual CalculateZoneVolume FacePoints/Newell copy in Source AllSurface order.
use super::{Error, SurfaceClass, SurfaceGeometryInputState};
use crate::geometry::{ZoneVolumeFace, source_newell_area_vector, source_vector_length};
use ep_model::{Point3, SurfaceType};

impl SurfaceGeometryInputState {
    pub(crate) fn copy_source_zone_volume_faces(
        &self,
        zone_number: i32,
        first: i32,
        last: i32,
    ) -> Result<(Vec<ZoneVolumeFace>, f64), Error> {
        let rows = self.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "actual Source volume Surface allocation",
        ))?;
        let mut faces = Vec::new();
        let mut sum_areas = 0.0;
        for number in first..=last {
            let index = usize::try_from(number)
                .ok()
                .and_then(|n| n.checked_sub(1))
                .ok_or(Error::Unavailable("positive Source volume Surface index"))?;
            let surface = rows
                .get(index)
                .ok_or(Error::Unavailable("actual Zone volume AllSurface range"))?;
            let row = &surface.row;
            if row.zone != zone_number {
                return Err(Error::Unavailable("same volume Surface Zone association"));
            }
            let surface_type = match row.class {
                SurfaceClass::Wall => SurfaceType::Wall,
                SurfaceClass::Floor => SurfaceType::Floor,
                SurfaceClass::Roof => SurfaceType::Roof,
                _ => {
                    return Err(Error::Unavailable(
                        "nonbase Source volume notused/debug branch",
                    ));
                }
            };
            let vertices = row
                .vertex
                .as_ref()
                .ok_or(Error::Unavailable("actual volume FacePoints source"))?;
            if vertices.len() != 4 || row.sides != 4 {
                return Err(Error::Unavailable("selected six-quad volume admission"));
            }
            let points = vertices
                .iter()
                .map(|value| {
                    let [x_m, y_m, z_m] =
                        value.ok_or(Error::Unavailable("actual written volume FacePoint"))?;
                    Ok(Point3 { x_m, y_m, z_m })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            // Reached12079/12080, independently of the earlier GetVertices call.
            // Binary64 scaling by exact0.5 retains the existing finite divide2 core.
            let newell_area_vector_m2 = source_newell_area_vector(&points);
            sum_areas += source_vector_length(newell_area_vector_m2);
            let properties = row
                .vertex_progress
                .properties
                .ok_or(Error::Unavailable("actual volume Surface tilt/area"))?;
            faces.push(ZoneVolumeFace {
                surface_id: surface
                    .typed_id
                    .ok_or(Error::Unavailable("actual bound volume SurfaceId"))?,
                name: row.name.clone(),
                surface_type,
                vertices: points,
                area_m2: properties.area_m2,
                gross_area_m2: properties.gross_area_m2,
                newell_area_vector_m2,
                tilt_deg: properties.tilt_deg,
            });
        }
        Ok((faces, sum_areas))
    }
}
