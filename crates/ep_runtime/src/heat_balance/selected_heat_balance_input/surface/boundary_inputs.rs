//! Reached zero vented/exposed/HT-override inputs at GetSurfaceData2850-2854.
//! Stops before2857 enclosures and does not complete the surface getter.
use super::super::{SelectedHeatTransferModel, rounded};
use super::{Error, SourceErrorBackend, SurfaceGeometryInputState, SurfaceGetterOwners, signed};
use std::io::Write;

fn require_empty(owners: &mut SurfaceGetterOwners<'_>, family: &'static str) -> Result<(), Error> {
    owners.shortcut.current_module_object = family.into();
    if owners.input.get_num_objects_found(family) != 0 {
        return Err(Error::Unavailable(family));
    }
    Ok(())
}

impl SurfaceGeometryInputState {
    fn read_empty_exterior_vented_cavities(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        let family = "SurfaceProperty:ExteriorNaturalVentedCavity";
        owners.shortcut.current_module_object = family.into();
        let counts = owners
            .input
            .get_object_def_max_args(family)
            .map_err(|_| Error::Unavailable("actual vented-cavity definition query"))?;
        if counts.numerics != 8 {
            backend.show_severe_error(&format!(
                "{family}: Object Definition indicates not = 8 Number Objects, Number Indicated={}",
                counts.numerics,
            ))?;
            *errors_found = true;
        }
        self.total_ext_vented_cavities = signed(owners.input.get_num_objects_found(family))?;
        if self.total_ext_vented_cavities != 0 {
            // The positive array constructor/getObjectItem/output producers
            // remain outside the selected closure. Preserve the real count.
            return Err(Error::Unavailable(
                "positive exterior vented-cavity allocation",
            ));
        }
        self.ext_vented_cavities = Some(Vec::new()); // Original7471 allocate(0).
        Ok(())
    }

    fn read_empty_heat_transfer_overrides(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        require_empty(owners, "SurfaceProperty:HeatBalanceSourceTerm")?;
        for surface in self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("actual final Surface algorithm owner"))?
        {
            surface.row.heat_transfer_algorithm = owners.control.overall_heat_transfer_solution;
        }
        require_empty(owners, "SurfaceProperty:HeatTransferAlgorithm")?;
        // Literal8459 repeats this assignment before the empty item loop.
        owners.shortcut.current_module_object = "SurfaceProperty:HeatTransferAlgorithm".into();
        require_empty(
            owners,
            "SurfaceProperty:HeatTransferAlgorithm:MultipleSurface",
        )?;
        require_empty(owners, "SurfaceProperty:HeatTransferAlgorithm:SurfaceList")?;
        require_empty(owners, "SurfaceProperty:HeatTransferAlgorithm:Construction")?;
        for surface in self.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "actual Kiva Surface algorithm traversal",
        ))? {
            if surface.row.exterior_boundary == -6 {
                return Err(Error::Unavailable("positive Kiva algorithm assignment"));
            }
        }
        // Literal8893-8908: actual IP queries, including both PC families and
        // all six HAMT families, before evaluating the material warning tests.
        let mut material_counts = [0_i32; 10];
        for (count, family) in material_counts.iter_mut().zip([
            "MaterialProperty:MoisturePenetrationDepth:Settings",
            "MaterialProperty:PhaseChange",
            "MaterialProperty:PhaseChangeHysteresis",
            "MaterialProperty:VariableThermalConductivity",
            "MaterialProperty:HeatAndMoistureTransfer:Settings",
            "MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm",
            "MaterialProperty:HeatAndMoistureTransfer:Suction",
            "MaterialProperty:HeatAndMoistureTransfer:Redistribution",
            "MaterialProperty:HeatAndMoistureTransfer:Diffusion",
            "MaterialProperty:HeatAndMoistureTransfer:ThermalConductivity",
        ]) {
            *count = signed(owners.input.get_num_objects_found(family))?;
        }
        if material_counts.iter().any(|&count| count != 0) {
            return Err(Error::Unavailable(
                "positive HT material-property warning branches",
            ));
        }
        if owners.control.any_empd || owners.control.any_cond_fd || owners.control.any_hamt {
            return Err(Error::Unavailable(
                "non-CTF heat-transfer algorithm report branches",
            ));
        }
        writeln!(
            eio,
            "! <Surface Heat Transfer Algorithm>, Value {{CTF - ConductionTransferFunction | EMPD - MoisturePenetrationDepthConductionTransferFunction | CondFD - ConductionFiniteDifference | HAMT - CombinedHeatAndMoistureFiniteElement}} - Description,Inside Surface Max Temperature Limit{{C}}, Surface Convection Coefficient Lower Limit {{W/m2-K}}, Surface Convection Coefficient Upper Limit {{W/m2-K}}"
        )?;
        let mut number_of_heat_transfer_algorithms_used = 0;
        if owners.control.any_ctf {
            number_of_heat_transfer_algorithms_used += 1;
            writeln!(
                eio,
                "Surface Heat Transfer Algorithm, CTF - ConductionTransferFunction,{},{},{}",
                rounded(owners.control.max_surface_temperature_limit, 0)?,
                rounded(owners.control.low_hconv_limit, 2)?,
                rounded(owners.control.high_hconv_limit, 1)?,
            )?;
        }
        if self.any_kiva {
            return Err(Error::Unavailable("positive Kiva algorithm EIO report"));
        }
        if number_of_heat_transfer_algorithms_used > 1 {
            return Err(Error::Unavailable(
                "multiple interior Surface algorithm comparison",
            ));
        }
        // Selected final rows are actual opaque SurfaceData. No window,
        // shade or TDD producer is admitted; their class setters stay unported.
        for surface in self.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "actual final Surface construction-use traversal",
        ))? {
            if !matches!(
                surface.row.class,
                super::SurfaceClass::Wall | super::SurfaceClass::Floor | super::SurfaceClass::Roof
            ) {
                return Err(Error::Unavailable(
                    "nonopaque final algorithm class assignment",
                ));
            }
            if surface.row.heat_transfer_algorithm == SelectedHeatTransferModel::Ctf {
                let index = surface
                    .row
                    .construction
                    .checked_sub(1)
                    .and_then(|value| usize::try_from(value).ok())
                    .ok_or(Error::Unavailable("actual Construction algorithm index"))?;
                owners
                    .control
                    .construction_usage
                    .get_mut(index)
                    .ok_or(Error::Unavailable("same Construction IsUsedCTF owner"))?
                    .is_used_ctf = true;
            }
        }
        Ok(())
    }

    pub(crate) fn read_empty_surface_boundary_inputs(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        self.read_empty_exterior_vented_cavities(owners, backend, errors_found)?;
        // The empty ExposedFoundationPerimeter loop writes no map entry and
        // does not reset or replace the existing sole map.
        require_empty(owners, "SurfaceProperty:ExposedFoundationPerimeter")?;
        self.read_empty_heat_transfer_overrides(owners, eio)
    }
}
