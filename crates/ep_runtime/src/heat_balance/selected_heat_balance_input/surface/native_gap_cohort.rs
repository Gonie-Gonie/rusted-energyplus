//! Primary-input loader inside the real Surface owner; no scientific outputs.
use super::{Error, SurfaceClass, SurfaceGeometryInputState};

impl SurfaceGeometryInputState {
    pub(crate) fn native_gap_opaque_radiation_rows(n: usize) -> Result<Self, Error> {
        let count = i32::try_from(n).map_err(|_| Error::Unavailable("native gap surface count"))?;
        let mut geometry = Self::default();
        geometry.total_surfaces = count;
        let mut rows = vec![super::final_mapping::FinalSurface::default(); n];
        for (i, surface) in rows.iter_mut().enumerate() {
            surface.row.name = format!("Named opaque {}", i + 1);
            surface.row.construction = i32::try_from(i + 1)
                .map_err(|_| Error::Unavailable("native gap construction index"))?;
            surface.row.class = SurfaceClass::Wall;
            surface.original_class = SurfaceClass::Wall;
        }
        geometry.final_surfaces = Some(rows);
        // Only IRfromParentZone is consumed here. Other arrays are genuine
        // allocator defaults and explicitly outside the observer comparison.
        geometry.window_arrays = Some(super::window_arrays::SurfaceWindowArrays::allocate(n, 0)?);
        Ok(geometry)
    }
}

// Declared narrow InitExt primary scalars. No Vertex/Space/geometry-ready
// producer is simulated. The unconsumed bundled fields are NaN canaries.
impl SurfaceGeometryInputState {
    pub(crate) fn native_gap_cold_external_row(
        name: &str,
        wind: bool,
        gross: f64,
        height: f64,
        cos: f64,
        azimuth: f64,
        sky: f64,
        ground: f64,
        model: i32,
    ) -> Result<Self, Error> {
        let mut geometry = Self::default();
        let mut surface = super::final_mapping::FinalSurface::default();
        surface.typed_id = Some(ep_model::SurfaceId(0));
        surface.row.name = name.to_owned();
        surface.row.zone = 1;
        surface.row.base_surface = 1;
        surface.row.class = SurfaceClass::Wall;
        surface.original_class = SurfaceClass::Wall;
        surface.row.exterior_wind = wind;
        surface.row.gross_area = gross;
        surface.row.vertex_progress.height = height;
        surface.row.vertex_progress.view_factor_sky_ir = sky;
        surface.row.vertex_progress.view_factor_ground_ir = ground;
        let q = f64::NAN;
        surface.row.vertex_progress.properties = Some(crate::geometry::GetVerticesProperties {
            // Only these three stored members are consumed by selected InitExt.
            gross_area_m2: gross,
            azimuth_deg: azimuth,
            cos_tilt: cos,
            area_m2: q,
            net_area_shadow_m2: q,
            tilt_deg: q,
            newell_area_vector_m2: [q; 3],
            newell_normal: [q; 3],
            out_norm: [q; 3],
            lcsx: [q; 3],
            lcsy: [q; 3],
            lcsz: [q; 3],
            sin_azimuth: q,
            cos_azimuth: q,
            sin_tilt: q,
        });
        if surface.row.heat_transfer_surface
            || surface.row.space != 0
            || surface.row.vertex.is_some()
        {
            return Err(Error::Unavailable(
                "genuine cold Surface constructor changed",
            ));
        }
        geometry.final_surfaces = Some(vec![surface]);
        geometry.total_surfaces = i32::try_from(
            geometry
                .final_surfaces
                .as_ref()
                .ok_or(Error::Unavailable("cold Surface allocation"))?
                .len(),
        )
        .map_err(|_| Error::Unavailable("cold actual Surface count"))?;
        let mut arrays = super::parallel_arrays::SurfaceParallelArrays::allocate(1);
        arrays.exterior_convection[0].model = model;
        geometry.parallel_arrays = Some(arrays);
        Ok(geometry)
    }
}

impl SurfaceGeometryInputState {
    // Narrow named Sky/opaque consumers set these actual members directly in
    // Native. All unconsumed bundled geometry fields remain NaN canaries.
    pub(crate) fn native_gap_sky_row_inputs(
        &mut self,
        at: usize,
        name: &str,
        normal: [f64; 3],
        tilt: f64,
        sin_tilt: f64,
        sky: f64,
    ) -> Result<(), Error> {
        let surface = self
            .final_surfaces
            .as_mut()
            .and_then(|r| r.get_mut(at))
            .ok_or(Error::Unavailable("actual solar unit row"))?;
        surface.row.name = name.into();
        surface.row.vertex_progress.view_factor_sky = sky;
        let q = f64::NAN;
        surface.row.vertex_progress.properties = Some(crate::geometry::GetVerticesProperties {
            out_norm: normal,
            tilt_deg: tilt,
            sin_tilt,
            area_m2: q,
            gross_area_m2: q,
            net_area_shadow_m2: q,
            azimuth_deg: q,
            cos_tilt: q,
            newell_area_vector_m2: [q; 3],
            newell_normal: [q; 3],
            lcsx: [q; 3],
            lcsy: [q; 3],
            lcsz: [q; 3],
            sin_azimuth: q,
            cos_azimuth: q,
        });
        self.surface_lists
            .all_exterior_solar
            .push(i32::try_from(at + 1).map_err(|_| Error::Unavailable("solar unit ordinal"))?);
        Ok(())
    }
}

impl SurfaceGeometryInputState {
    /// Narrow complete scalar consumer uses the real physical Surface owner.
    /// No vertices, space/geometry readiness or getter producer is asserted.
    pub(crate) fn native_gap_outside_scalar_row(name: &str) -> Result<Self, Error> {
        let mut geometry = Self::native_gap_opaque_radiation_rows(1)?;
        let row = &mut geometry
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("actual outside unit Surface allocation"))?[0];
        row.typed_id = Some(ep_model::SurfaceId(0));
        row.row.name = name.to_owned();
        Ok(geometry)
    }
}
