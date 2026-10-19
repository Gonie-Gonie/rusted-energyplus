//! Primary-input loader inside the real Surface owner; no scientific outputs.
use super::{SurfaceGeometryInputState, SurfaceClass, Error};

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
