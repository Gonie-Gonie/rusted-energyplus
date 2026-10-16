//! Actual no-glazing traversal of InitGlassOpticalCalculations339-1636.
use super::*;
impl SourceWindowOpticalInitialization {
    pub(super) fn init_glass_optical_calculations(
        &mut self,
        owners: &mut SourceWindowOpticalOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Declared local scratch153-337 has no reader or externally observable
        // effect before the first represented positive-glazing boundary374.
        // In particular no persistent window arrays are fabricated from it.
        self.w5_init_glass_parameters(owners)?; //339
        if owners.materials.num_blinds > 0 {
            return Err(Error::Unavailable(
                "InitGlass positive CalcWindowBlindProperties343",
            ));
        }
        if owners.materials.num_screens > 0 {
            return Err(Error::Unavailable(
                "InitGlass positive CalcWindowScreenProperties348",
            ));
        }
        for number in 0..owners.control.total_constructions {
            let row = owners
                .control
                .construction_usage
                .get(number)
                .ok_or(Error::Unavailable("InitGlass actual Construct ordinal355"))?;
            if !row.type_is_window {
                continue;
            }
            if row.window_type_bsdf {
                continue;
            }
            if row.window_type_eql {
                continue;
            }
            return Err(Error::Unavailable("InitGlass positive glazing TotLay374"));
        }
        let count = usize::try_from(owners.geometry.total_surfaces)
            .map_err(|_| Error::Unavailable("InitGlass actual TotSurfaces"))?;
        for number in 0..count {
            let surface = owners
                .geometry
                .final_surfaces
                .as_deref()
                .and_then(|rows| rows.get(number))
                .ok_or(Error::Unavailable("InitGlass actual Surface1521"))?;
            if !surface.row.heat_transfer_surface {
                continue;
            }
            let construction = source_surface_index(surface.row.construction)?;
            if !owners
                .control
                .construction_usage
                .get(construction)
                .ok_or(Error::Unavailable("InitGlass Surface Construction1525"))?
                .type_is_window
            {
                continue;
            }
            return Err(Error::Unavailable("InitGlass positive window model1528"));
        }
        for number in 0..count {
            let surface = owners
                .geometry
                .final_surfaces
                .as_deref()
                .and_then(|rows| rows.get(number))
                .ok_or(Error::Unavailable("InitGlass actual Surface1581"))?;
            if surface.row.construction <= 0 {
                continue;
            }
            let construction = source_surface_index(surface.row.construction)?;
            if !owners
                .control
                .construction_usage
                .get(construction)
                .ok_or(Error::Unavailable("InitGlass Surface Construction1585"))?
                .type_is_window
            {
                continue;
            }
            return Err(Error::Unavailable(
                "InitGlass positive window glazing thickness1591",
            ));
        }
        self.report_glass(owners) //1636
    }
}
